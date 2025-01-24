use std::io::stderr;

use addressbook::{vdir::Client, Addressbook, Card};
use addressbook_vdir::Connector;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new_from_envs();
    let mut vdir = Connector::new();

    let mut addressbook = Addressbook::default();
    addressbook.name = "Test".into();
    addressbook.desc = Some("Testing addressbook".into());

    let mut flow = client.create_addressbook(addressbook);
    while let Some(io) = flow.next() {
        vdir.execute(&mut flow, io).unwrap();
    }

    let addressbook = flow.output().unwrap();
    println!();
    println!("created addressbook: {addressbook:#?}");

    let mut card = Card::new_v3_0();
    card.properties.insert("FN".into(), "Test".into());

    let mut flow = client.create_card(&addressbook.id, card);
    while let Some(io) = flow.next() {
        vdir.execute(&mut flow, io).unwrap();
    }

    let card = flow.output().unwrap();
    println!();
    println!("created card: {card:#?}");

    let mut flow = client.read_card(&addressbook.id, &card.id);
    while let Some(io) = flow.next() {
        vdir.execute(&mut flow, io).unwrap();
    }

    let card = flow.output().unwrap();
    println!();
    println!("read card: {card:#?}");

    // card.properties.insert("FN".into(), "Test updated".into());

    // tcp = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    // let mut flow = client.update_card(&addressbook.id, card);
    // while let Some(io) = flow.next() {
    //     match io {
    //         tcp::Io::Read => {
    //             tcp.read(&mut flow).unwrap();
    //         }
    //         tcp::Io::Write => {
    //             tcp.write(&mut flow).unwrap();
    //         }
    //     }
    // }

    // let card = flow.output();
    // println!();
    // println!("updated card: {card:#?}");

    // tcp = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    // let mut flow = client.delete_card(&addressbook.id, &card.id);
    // while let Some(io) = flow.next() {
    //     match io {
    //         tcp::Io::Read => {
    //             tcp.read(&mut flow).unwrap();
    //         }
    //         tcp::Io::Write => {
    //             tcp.write(&mut flow).unwrap();
    //         }
    //     }
    // }

    // println!();
    // println!("card {} deleted", card.id);

    let mut flow = client.delete_addressbook(&addressbook.id);
    while let Some(io) = flow.next() {
        vdir.execute(&mut flow, io).unwrap();
    }

    flow.output().unwrap();
    println!();
    println!("addressbook {} deleted", addressbook.id);
}
