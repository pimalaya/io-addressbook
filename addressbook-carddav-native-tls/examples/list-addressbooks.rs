use std::io::stderr;

use addressbook::{carddav::Client, tcp};
use addressbook_carddav_native_tls::Connector;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new_from_envs();
    let mut tls = Connector::connect(&client.config.hostname, client.config.port).unwrap();
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
