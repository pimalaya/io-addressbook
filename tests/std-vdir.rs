#![cfg(all(feature = "client", feature = "vdir"))]

use io_addressbook::{
    addressbook::AddressbookDiff, client::AddressbookClientStd, vdir::client::VdirClient,
};
use io_vdir::{client::VdirClient as InnerVdirClient, path::VdirPath};
use tempfile::tempdir;

const VCARD_V1: &[u8] = b"BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Test\r\nEND:VCARD\r\n";
const VCARD_V2: &[u8] = b"BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Test2\r\nEND:VCARD\r\n";

#[test]
fn std_vdir() {
    let _ = env_logger::builder().is_test(true).try_init();

    let workdir = tempdir().unwrap();
    let root = VdirPath::new(workdir.path().to_string_lossy().into_owned());
    let mut client = AddressbookClientStd::from(VdirClient::new(InnerVdirClient::new(root)));

    // should list empty addressbooks
    let addressbooks = client.list_addressbooks().unwrap();
    assert!(addressbooks.is_empty());

    // should create an addressbook without metadata
    client.create_addressbook("personal", None, None).unwrap();

    let addressbooks = client.list_addressbooks().unwrap();
    assert_eq!(addressbooks.len(), 1);
    assert_eq!(addressbooks[0].id, "personal");

    // should update addressbook metadata
    let patch = AddressbookDiff {
        name: Some("Custom addressbook name".into()),
        description: Some(Some("This is a description.".into())),
        color: Some(Some("#000000".into())),
    };
    client.update_addressbook("personal", patch).unwrap();

    let addressbooks = client.list_addressbooks().unwrap();
    let addressbook = addressbooks
        .iter()
        .find(|a| a.id == "personal")
        .expect("personal addressbook present");
    assert_eq!(addressbook.name, "Custom addressbook name");
    assert_eq!(
        addressbook.description.as_deref(),
        Some("This is a description.")
    );
    assert_eq!(addressbook.color.as_deref(), Some("#000000"));

    // should create a card
    let id = client.create_card("personal", VCARD_V1.to_vec()).unwrap();
    assert!(!id.is_empty());

    let cards = client.list_cards("personal", None, None).unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].id, id);
    assert_eq!(cards[0].addressbook_id, "personal");
    assert_eq!(cards[0].contents, VCARD_V1);

    // should update the card
    client
        .update_card("personal", &id, VCARD_V2.to_vec(), None)
        .unwrap();

    let card = client.get_card("personal", &id).unwrap();
    assert_eq!(card.contents, VCARD_V2);

    // should delete the card
    client.delete_card("personal", &id).unwrap();
    let cards = client.list_cards("personal", None, None).unwrap();
    assert!(cards.is_empty());

    // should delete the addressbook
    client.delete_addressbook("personal").unwrap();
    let addressbooks = client.list_addressbooks().unwrap();
    assert!(addressbooks.is_empty());
}
