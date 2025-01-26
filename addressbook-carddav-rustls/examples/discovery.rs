use std::io::stderr;

use addressbook::carddav::Client;
use addressbook_carddav_rustls::{Connector, CryptoProvider};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new_from_envs();

    let crypto =
        std::env::var("CRYPTO").expect("CRYPTO env var should be either `aws-lc` or `ring`");
    let crypto = match crypto.as_str() {
        #[cfg(feature = "aws-lc")]
        "aws-lc" => CryptoProvider::AwsLc,
        #[cfg(not(feature = "aws-lc"))]
        "aws-lc" => panic!("missing feature `aws-lc`"),
        #[cfg(feature = "ring")]
        "ring" => CryptoProvider::Ring,
        #[cfg(not(feature = "ring"))]
        "ring" => panic!("missing feature `ring`"),
        unknown => panic!("unknown crypto provider {unknown} (valid: aws-lc, ring)"),
    };

    // Current user principal

    // NOTE: ideally, this should be needed once in order to re-use
    // the connection. It depends on the HTTP protocol returned by the
    // server.
    let mut tls = Connector::connect(&client.config.hostname, client.config.port, &crypto).unwrap();
    let mut flow = client.current_user_principal();
    while let Some(io) = flow.next() {
        tls.execute(&mut flow, io).unwrap()
    }

    let current_user_principal = flow.output().unwrap();
    println!();
    println!("current user principal: {current_user_principal:?}");

    let current_user_principal = current_user_principal.unwrap_or(String::from("/"));

    // Addressbook home set

    tls = Connector::connect(&client.config.hostname, client.config.port, &crypto).unwrap();
    let mut flow = client.addressbook_home_set(current_user_principal);
    while let Some(io) = flow.next() {
        tls.execute(&mut flow, io).unwrap()
    }

    let addressbook_home_set = flow.output().unwrap();
    println!("addressbook home set: {addressbook_home_set:?}");
}
