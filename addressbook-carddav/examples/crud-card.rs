use std::io::stderr;

use addressbook::{carddav::Client, Addressbook, Card};
use addressbook_carddav::Connector;
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

    let mut tcp = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.create_addressbook(addressbook);
    while let Some(io) = flow.next() {
        tcp.execute(&mut flow, io).unwrap()
    }

    let addressbook = flow.output().unwrap();
    println!();
    println!("created addressbook: {addressbook:#?}");

    let mut card = Card::new_v3_0();
    card.properties.insert("FN".into(), "Test".into());

    tcp = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.create_card(&addressbook.id, card);
    while let Some(io) = flow.next() {
        tcp.execute(&mut flow, io).unwrap()
    }

    let card = flow.output();
    println!();
    println!("created card: {card:#?}");

    tcp = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.read_card(&addressbook.id, &card.id);
    while let Some(io) = flow.next() {
        tcp.execute(&mut flow, io).unwrap()
    }

    let mut card = flow.output().unwrap();
    println!();
    println!("read card: {card:#?}");

    card.properties.insert("FN".into(), "Test updated".into());

    tcp = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.update_card(&addressbook.id, card);
    while let Some(io) = flow.next() {
        tcp.execute(&mut flow, io).unwrap()
    }

    let card = flow.output();
    println!();
    println!("updated card: {card:#?}");

    tcp = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.delete_card(&addressbook.id, &card.id);
    while let Some(io) = flow.next() {
        tcp.execute(&mut flow, io).unwrap()
    }

    println!();
    println!("card {} deleted", card.id);

    tcp = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.delete_addressbook(&addressbook.id);
    while let Some(io) = flow.next() {
        tcp.execute(&mut flow, io).unwrap()
    }

    let success = flow.output().unwrap();
    println!();
    println!("addressbook {} deleted: {success}", addressbook.id);
}
