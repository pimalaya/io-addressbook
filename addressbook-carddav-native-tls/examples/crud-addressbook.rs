use std::io::stderr;

use addressbook::{carddav::Client, tcp, Addressbook};
use addressbook_carddav_native_tls::Connector;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new_from_envs();

    let mut addressbook = Addressbook::default();
    addressbook.name = "Test".into();
    addressbook.desc = Some("Testing addressbook".into());

    let mut tls = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.create_addressbook(addressbook);
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

    let mut addressbook = flow.output().unwrap();
    println!();
    println!("created addressbook: {addressbook:#?}");

    addressbook.name = "Test updated".into();
    addressbook.desc = Some("".into());
    addressbook.color = Some("#abcdef".into());

    tls = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.update_addressbook(addressbook);
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

    let addressbook = flow.output().unwrap();
    println!();
    println!("updated addressbook: {addressbook:#?}");

    tls = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.delete_addressbook(&addressbook.id);
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

    let success = flow.output().unwrap();
    println!();
    println!("addressbook {} deleted: {success}", addressbook.id);
}
