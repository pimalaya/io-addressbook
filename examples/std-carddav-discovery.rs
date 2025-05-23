use std::{
    env,
    io::{stdin, stdout, Read, Write},
    net::TcpStream,
    sync::Arc,
};

use http::Version;
use io_addressbook::carddav::{
    config::Authentication,
    coroutines::{AddressbookHomeSet, CurrentUserPrincipal, ListAddressbooks, ListCards},
    Config,
};
use io_stream::{runtimes::std::handle, Io};
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use rustls_platform_verifier::ConfigVerifierExt;

fn main() {
    env_logger::init();

    let host = match env::var("HOST") {
        Ok(host) => host,
        Err(_) => prompt("Host?").parse().unwrap(),
    };

    let port: u16 = match env::var("PORT") {
        Ok(port) => port.parse().unwrap(),
        Err(_) => prompt("Port?").parse().unwrap(),
    };

    let scheme = match env::var("SCHEME") {
        Ok(scheme) => scheme,
        Err(_) => prompt("Scheme?"),
    };

    let user = match env::var("USER") {
        Ok(user) => user,
        Err(_) => prompt("User?").parse().unwrap(),
    };

    let pass = match env::var("PASS") {
        Ok(pass) => pass.into(),
        Err(_) => prompt("Password?").into(),
    };

    let http_version = match env::var("HTTP_VERSION") {
        Ok(v) if v == "1.0" => Version::HTTP_10,
        Ok(v) if v == "1.1" => Version::HTTP_11,
        Ok(v) => panic!("invalid HTTP version {v}, expect 1.0 or 1.1"),
        Err(_) => Version::default(),
    };

    println!("connect to {host}:{port}");
    let mut stream = connect(&host, port, &scheme);

    let mut config = Config {
        host,
        port,
        http_version,
        home_uri: "/".into(),
        authentication: Authentication::Basic(user, pass),
    };

    let mut arg = None;
    let mut http = CurrentUserPrincipal::new(&config, &config.home_uri);

    config.home_uri = loop {
        match http.resume(arg.take()) {
            Ok(None) => break config.home_uri.clone(),
            Ok(Some(path)) => break path,
            Err(Io::Error(err)) => panic!("{err}"),
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    println!("current user principal: {:?}", config.home_uri);

    let mut arg = None;
    let mut http = AddressbookHomeSet::new(&config, &config.home_uri);

    config.home_uri = loop {
        match http.resume(arg.take()) {
            Ok(None) => break config.home_uri.clone(),
            Ok(Some(path)) => break path,
            Err(Io::Error(err)) => panic!("{err}"),
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    println!("addressbook home set: {:?}", config.home_uri);

    let mut arg = None;
    let mut http = ListAddressbooks::new(&config);

    let addressbooks = loop {
        match http.resume(arg.take()) {
            Ok(addressbooks) => break addressbooks,
            Err(Io::Error(err)) => panic!("{err}"),
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    println!("addressbooks: {addressbooks:#?}");

    let mut arg = None;
    let mut http = ListCards::new(&config, addressbooks.into_iter().next().unwrap().id);

    let cards = loop {
        match http.resume(arg.take()) {
            Ok(cards) => break cards,
            Err(Io::Error(err)) => panic!("{err}"),
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    println!("cards: {cards:#?}");
}

fn prompt(message: &str) -> String {
    print!("{message} ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim().to_owned()
}

trait StreamExt: Read + Write {}
impl<T: Read + Write> StreamExt for T {}

fn connect(host: &str, port: u16, scheme: &str) -> Box<dyn StreamExt> {
    if scheme.eq_ignore_ascii_case("https") {
        let config = ClientConfig::with_platform_verifier();
        let server_name = host.to_string().try_into().unwrap();
        let conn = ClientConnection::new(Arc::new(config), server_name).unwrap();
        let tcp = TcpStream::connect((host.to_string(), port)).unwrap();
        let tls = StreamOwned::new(conn, tcp);
        Box::new(tls)
    } else {
        let tcp = TcpStream::connect((host.to_string(), port)).unwrap();
        Box::new(tcp)
    }
}
