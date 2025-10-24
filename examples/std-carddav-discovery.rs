#![cfg(feature = "carddav")]

use std::{borrow::Cow, env};

use io_addressbook::carddav::{
    config::{CarddavAuth, CarddavConfig},
    coroutines::{
        current_user_principal::CurrentUserPrincipal, follow_redirects::FollowRedirectsResult,
    },
};
use io_stream::runtimes::std::handle;
use pimalaya_toolbox::stream::{Stream, Tls};
use secrecy::SecretString;

fn main() {
    env_logger::init();

    let uri = env::var("URI").expect("URI env var").parse().unwrap();

    let username = Cow::Owned(env::var("USERNAME").expect("USERNAME env var"));
    let password = Cow::Owned(SecretString::from(
        env::var("PASSWORD").expect("PASSWORD env var"),
    ));

    println!("connecting to {uri}…");
    let mut stream = Stream::connect(&uri, &Tls::RustlsAws).unwrap();

    let config = CarddavConfig {
        uri: Cow::Borrowed(&uri),
        auth: CarddavAuth::Basic { username, password },
    };

    let mut arg = None;
    let mut http = CurrentUserPrincipal::new(&config);

    let uri = loop {
        match http.resume(arg.take()) {
            FollowRedirectsResult::Ok(res) => {
                break match res.body {
                    Some(uri) => uri,
                    None => uri,
                }
            }
            FollowRedirectsResult::Err(err) => panic!("{err}"),
            FollowRedirectsResult::Io(io) => arg = Some(handle(&mut stream, io).unwrap()),
            FollowRedirectsResult::Reset(uri) => {
                stream = Stream::connect(&uri, &Tls::RustlsAws).unwrap()
            }
        }
    };

    println!("current user principal: {:?}", uri);

    // let mut arg = None;
    // let mut http = AddressbookHomeSet::new(&config);

    // config.home_uri = loop {
    //     match http.resume(arg.take()) {
    //         Ok(None) => break config.home_uri.clone(),
    //         Ok(Some(path)) => break path,
    //         Err(Io::Error(err)) => panic!("{err}"),
    //         Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
    //     }
    // };

    // println!("addressbook home set: {:?}", config.home_uri);

    // let mut arg = None;
    // let mut http = ListAddressbooks::new(&config);

    // let addressbooks = loop {
    //     match http.resume(arg.take()) {
    //         Ok(addressbooks) => break addressbooks,
    //         Err(Io::Error(err)) => panic!("{err}"),
    //         Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
    //     }
    // };

    // println!("addressbooks: {addressbooks:#?}");

    // let mut arg = None;
    // let mut http = ListCards::new(&config, addressbooks.into_iter().next().unwrap().id);

    // let cards = loop {
    //     match http.resume(arg.take()) {
    //         Ok(cards) => break cards,
    //         Err(Io::Error(err)) => panic!("{err}"),
    //         Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
    //     }
    // };

    // println!("cards: {cards:#?}");
}
