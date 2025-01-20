use std::{env, io::stderr, process::Command};

use addressbook::{
    carddav::Client,
    contact::HttpVersion,
    tcp::{sans_io::Io as TcpIo, std::Connector},
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .init();

    let host = env::var("HOST").unwrap_or(String::from("localhost"));
    println!("using host: {host:?}");

    let port = env::var("PORT").unwrap_or(String::from("8001"));
    let port = port.parse::<u16>().expect("should be an integer");
    println!("using port: {port:?}");

    let version = env::var("VERSION").unwrap_or(String::from("1.1"));
    let version = if version == "1.1" {
        HttpVersion::Http1_1
    } else {
        HttpVersion::Http1_0
    };
    println!("using HTTP version: {version:?}");

    let user = env::var("USER").unwrap_or(String::from("test"));
    println!("using user: {user:?}");

    let pass = env::var("PASSWORD_COMMAND").expect("missing env PASSWORD_COMMAND");
    println!("using password command: {pass:?}");
    let mut args = pass.split_whitespace();
    let program = args.next().unwrap();
    let pass = Command::new(program).args(args).output().unwrap().stdout;
    let pass = String::from_utf8_lossy(pass.trim_ascii()).to_string();

    // Define CardDAV client

    let client = Client::new().with_basic_authentication(user, pass);

    // Current user principal

    // NOTE: ideally, this should be needed once in order to re-use
    // the connection. It depends on the HTTP protocol returned by the
    // server.
    let mut tcp = Connector::connect(&host, port).unwrap();
    let mut flow = client.current_user_principal();

    while let Some(io) = flow.next() {
        match io {
            TcpIo::Read => {
                tcp.read(&mut flow).unwrap();
            }
            TcpIo::Write => {
                tcp.write(&mut flow).unwrap();
            }
        }
    }

    let url = flow.output().unwrap();

    println!("current user principal: {url:?}");

    // // Addressbook home set

    // let mut tcp = Connector::connect(&host, port).unwrap();
    // let mut flow =
    //     AddressbookHomeSetFlow::new(current_user_principal_url, &version, &user, &password);

    // while let Some(io) = flow.next() {
    //     match io {
    //         TcpIo::Read => {
    //             tcp.read(&mut flow).unwrap();
    //         }
    //         TcpIo::Write => {
    //             tcp.write(&mut flow).unwrap();
    //         }
    //     }
    // }

    // let output = flow.output().unwrap();

    // println!("addressbook home set output: {output:#?}");

    // let addressbook_home_set_url = output
    //     .responses
    //     .into_iter()
    //     .next()
    //     .unwrap()
    //     .propstats
    //     .into_iter()
    //     .next()
    //     .unwrap()
    //     .prop
    //     .addressbook_home_set
    //     .href
    //     .value;

    // println!("addressbook home set: {addressbook_home_set_url:?}");

    // // Addressbooks

    // let mut tcp = Connector::connect(&host, port).unwrap();
    // let mut flow = ListAddressbooksFlow::new(addressbook_home_set_url, &version, &user, &password);

    // while let Some(io) = flow.next() {
    //     match io {
    //         TcpIo::Read => {
    //             tcp.read(&mut flow).unwrap();
    //         }
    //         TcpIo::Write => {
    //             tcp.write(&mut flow).unwrap();
    //         }
    //     }
    // }

    // let output = flow.output().unwrap();

    // println!("addressbooks output: {output:#?}");

    // let addressbook_hrefs = output.get_addressbook_hrefs().collect::<Vec<_>>();

    // println!(
    //     "found {} addressbooks: {addressbook_hrefs:#?}",
    //     addressbook_hrefs.len()
    // );

    // // List CardDAV contacts

    // let addressbook_href = addressbook_hrefs.into_iter().next().unwrap();

    // let mut tcp = Connector::connect(&host, port).unwrap();
    // let mut flow = ListContactsFlow::new(addressbook_href, &version, &user, &password);

    // while let Some(io) = flow.next() {
    //     match io {
    //         TcpIo::Read => {
    //             tcp.read(&mut flow).unwrap();
    //         }
    //         TcpIo::Write => {
    //             tcp.write(&mut flow).unwrap();
    //         }
    //     }
    // }

    // let output = flow.output().unwrap();

    // println!("contacts output: {output:#?}");
}
