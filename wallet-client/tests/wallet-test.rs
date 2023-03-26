use btk::{bitcoin, secp256k1};
use btk::types::{Address, PrivateKey};

use crate::wallet-client;

#[test]
fn test_generate_address() {
    let secp = secp256k1::Secp256k1::new();
    let network = bitcoin::Network::Bitcoin;

    let (private_key, public_key, address) = generate_address(&secp, network);

    assert_eq!(private_key.to_string().len(), 64);
    assert_eq!(public_key.serialize().len(), 33);
    assert!(address.to_string().starts_with("1"));
}
