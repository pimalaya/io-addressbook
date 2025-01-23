use std::io::stderr;

use addressbook::{vdir::Client, Addressbook};
use addressbook_vdir::Connector;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new_from_envs();
    let mut fs = Connector::new();

    let mut addressbook = Addressbook::default();
    addressbook.name = "Test".into();
    addressbook.desc = Some("Testing addressbook".into());

    let mut flow = client.create_addressbook(addressbook);
    while let Some(io) = flow.next() {
        fs.execute(&mut flow, io).unwrap();
    }

    let addressbook = flow.output().unwrap();
    println!();
    println!("created addressbook: {addressbook:#?}");

    let mut flow = client.list_addressbooks();
    while let Some(io) = flow.next() {
        fs.execute(&mut flow, io).unwrap();
    }

    let addressbooks = flow.output().unwrap();
    println!();
    println!("addressbooks: {addressbooks:#?}");

    // let mut addressbook = PartialAddressbook::from(addressbook);
    // addressbook.name = None;
    // addressbook.desc = Some("".into());
    // addressbook.color = Some("#abcdef".into());

    // fs = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    // let mut flow = client.update_addressbook(addressbook);
    // while let Some(io) = flow.next() {
    //     match io {
    //         tcp::Io::Read => {
    //             fs.read(&mut flow).unwrap();
    //         }
    //         tcp::Io::Write => {
    //             fs.write(&mut flow).unwrap();
    //         }
    //     }
    // }

    // let addressbook = flow.output().unwrap();
    // println!();
    // println!("updated addressbook: {addressbook:#?}");

    // fs = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    // let mut flow = client.delete_addressbook(&addressbook.id);
    // while let Some(io) = flow.next() {
    //     match io {
    //         tcp::Io::Read => {
    //             fs.read(&mut flow).unwrap();
    //         }
    //         tcp::Io::Write => {
    //             fs.write(&mut flow).unwrap();
    //         }
    //     }
    // }

    // let success = flow.output().unwrap();
    // println!();
    // println!("addressbook {} deleted: {success}", addressbook.id);
}
