use btk::{bitcoin, secp256k1, util};
use btk::types::{Address, PrivateKey};

fn generate_address(secp: &secp256k1::Secp256k1, network: bitcoin::Network) -> (PrivateKey, secp256k1::PublicKey, Address) {
    let private_key = PrivateKey::new(&secp, &mut util::rng());
    let public_key = secp256k1::PublicKey::from_secret_key(&private_key);
    let address = Address::p2pkh(&public_key, network);

    (private_key, public_key, address)
}

fn main() {
    let secp = secp256k1::Secp256k1::new();
    let network = bitcoin::Network::Bitcoin;
    let (private_key, public_key, address) = generate_address(&secp, network);

    println!("Private key: {}", private_key);
    println!("Public key: {}", public_key);
    println!("Address: {}", address);
}
