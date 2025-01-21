use std::io::stderr;

use addressbook::{carddav::Client, tcp};
use addressbook_carddav_rustls::{Connector, CryptoProvider};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

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

    let client = Client::new_from_envs();
    let mut tls = Connector::connect(&client.config.hostname, client.config.port, &crypto).unwrap();
    let mut flow = client.list_addressbooks();

    while let Some(io) = flow.next() {
        match io {
            tcp::Io::Read => {
                tls.read(&mut flow).unwrap();
            }
            tcp::Io::Write => {
                tls.write(&mut flow).unwrap();
            }
        }
    }

    let addressbooks = flow.output().unwrap();
    println!();
    println!("{addressbooks:#?}");
}
