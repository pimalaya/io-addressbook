use std::io::stderr;

use addressbook::vdir::Client;
use addressbook_vdir::Connector;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new_from_envs();
    let mut fs = Connector::new();
    let mut flow = client.list_addressbooks();

    while let Some(io) = flow.next() {
        fs.execute(&mut flow, io).unwrap();
    }

    let addressbooks = flow.output();
    println!();
    println!("{addressbooks:#?}");
}
