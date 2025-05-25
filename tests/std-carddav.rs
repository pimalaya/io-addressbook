use std::{collections::HashSet, io::ErrorKind, net::TcpStream};

use calcard::vcard::VCard;
use http::Version;
use io_addressbook::{
    carddav::{config::Authentication, coroutines::*, Config},
    Addressbook, Card,
};
use io_stream::runtimes::std::handle;

#[test]
fn std_carddav() {
    env_logger::init();

    let config = Config {
        host: "127.0.0.1".into(),
        port: 8001,
        http_version: Version::HTTP_11,
        home_uri: "/user".into(),
        authentication: Authentication::Basic("user".into(), "".into()),
    };

    // should list empty addressbooks

    let mut arg = None;
    let mut list = ListAddressbooks::new(&config);
    let mut stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    let addressbooks = loop {
        match list.resume(arg) {
            Ok(addressbooks) => break addressbooks,
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    assert!(addressbooks.is_empty());

    // should create addressbook without metadata

    let mut addressbook = Addressbook::new();
    addressbook.display_name = Some("name".into());

    let mut arg = None;
    let mut create = CreateAddressbook::new(&config, addressbook.clone());
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    while let Err(io) = create.resume(arg) {
        arg = Some(handle(&mut stream, io).unwrap());
    }

    let mut arg = None;
    let mut list = ListAddressbooks::new(&config);
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    let addressbooks = loop {
        match list.resume(arg) {
            Ok(addressbooks) => break addressbooks,
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    let expected_addressbooks = HashSet::from_iter([addressbook.clone()]);

    assert_eq!(addressbooks, expected_addressbooks);

    // should not re-create existing addressbook

    let mut arg = None;
    let mut create = CreateAddressbook::new(&config, addressbook.clone());
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    loop {
        match create.resume(arg) {
            Ok(()) => unreachable!("should not be OK"),
            Err(io) => match handle(&mut stream, io) {
                Ok(output) => arg = Some(output),
                Err(err) => break assert_eq!(err.kind(), ErrorKind::Other),
            },
        }
    }

    // should update addressbook with metadata

    addressbook.display_name = Some("Custom addressbook name".into());
    addressbook.description = Some("This is a description.".into());
    addressbook.color = Some("#000000".into());

    let mut arg = None;
    let mut update = UpdateAddressbook::new(&config, addressbook.clone());
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    while let Err(io) = update.resume(arg) {
        arg = Some(handle(&mut stream, io).unwrap());
    }

    let mut arg = None;
    let mut list = ListAddressbooks::new(&&config);
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    let cards = loop {
        match list.resume(arg) {
            Ok(addressbooks) => break addressbooks,
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    let expected_addressbooks = HashSet::from_iter([addressbook.clone()]);

    assert_eq!(cards, expected_addressbooks);

    // should create card

    let vcard = concat!(
        "BEGIN:VCARD\r\n",
        "VERSION: 4.0\r\n",
        "UID: abc123\r\n",
        "FN: test\r\n",
        "END:VCARD\r\n"
    );

    let mut card = Card::new(&addressbook.id, VCard::parse(vcard).unwrap());

    let mut arg = None;
    let mut create = CreateCard::new(&config, card.clone());
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    while let Err(io) = create.resume(arg) {
        arg = Some(handle(&mut stream, io).unwrap());
    }

    let mut arg = None;
    let mut list = ListCards::new(&config, &addressbook.id);
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    let cards = loop {
        match list.resume(arg) {
            Ok(cards) => break cards,
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    assert_eq!(cards.len(), 1);

    let first_card = cards.into_iter().next().unwrap();

    assert_eq!(first_card.to_string(), vcard);

    // should update card

    let vcard = concat!(
        "BEGIN:VCARD\r\n",
        "VERSION: 4.0\r\n",
        "UID: abc123\r\n",
        "FN: test updated\r\n",
        "END:VCARD\r\n"
    );

    card.vcard = VCard::parse(vcard).unwrap();

    let mut arg = None;
    let mut update = UpdateCard::new(&config, card);
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    while let Err(io) = update.resume(arg) {
        arg = Some(handle(&mut stream, io).unwrap());
    }

    let mut arg = None;
    let mut list = ListCards::new(&config, &addressbook.id);
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    let cards = loop {
        match list.resume(arg) {
            Ok(cards) => break cards,
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    assert_eq!(cards.len(), 1);

    let card = cards.into_iter().next().unwrap();

    assert_eq!(card.to_string(), vcard);

    // // should read card

    // let mut output = None;
    // let mut fs = ReadCard::vcard(addressbook.path(), "card");

    // let expected_card = loop {
    //     match fs.resume(output) {
    //         Ok(card) => break card,
    //         Err(input) => output = Some(handle(input).unwrap()),
    //     }
    // };

    // assert_eq!(card, expected_card);

    // should delete card

    let mut arg = None;
    let mut delete = DeleteCard::new(&config, &addressbook.id, &card.id);
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    while let Err(io) = delete.resume(arg) {
        arg = Some(handle(&mut stream, io).unwrap());
    }

    let mut arg = None;
    let mut list = ListCards::new(&config, &addressbook.id);
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    let cards = loop {
        match list.resume(arg) {
            Ok(cards) => break cards,
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    assert_eq!(cards.into_iter().count(), 0);

    // should delete addressbook

    let mut arg = None;
    let mut delete = DeleteAddressbook::new(&config, &addressbook.id);
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    while let Err(io) = delete.resume(arg) {
        arg = Some(handle(&mut stream, io).unwrap());
    }

    let mut arg = None;
    let mut list = ListAddressbooks::new(&config);
    stream = TcpStream::connect((config.host.as_str(), config.port)).unwrap();

    let addressbooks = loop {
        match list.resume(arg) {
            Ok(addressbooks) => break addressbooks,
            Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
        }
    };

    assert!(addressbooks.is_empty());
}
