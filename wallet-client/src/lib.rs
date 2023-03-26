use bdk::{Wallet, wallet};
use bdk::bitcoin::network;
use bdk::database;
use bdk::wallet::AddressIndex;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// wpkh([e7caaa9b/84h/0h/0h]xpub6CyF1N1LHu39xUmBPzUx72L9QbYqq7gve7acL88AJL2nVciGEMDeWq2BY2dpCT2G4N8ba9Cyna7FNX3ptzLRri5hFNwXZenqW2MqnUvrzjW/<0;1>/*)#cnfwgdt9



fn create_wallet() -> Wallet<database::MemoryDatabase> {

    let descriptor = "wpkh([746f838e/84'/0'/0']xpub6BzkNiuXbY2M8ZBqwnh5gEgakBZM5ab8MsdFmZNHLRpLDVCoePADt8eyYCVEK6yMdkZBkCspAuHgaRFYJLdafuVP2a92BNcjB2Ewo6ZwgSV/1/*)#ewktdt5y";
    let db = database::MemoryDatabase::default();
    let network = network::constants::Network::Bitcoin;
    // create a bitcoin wallet with bdk
    let wallet = Wallet::new(descriptor, None, network,db).unwrap();
    // get the wallet's address
    let address = wallet.get_address(AddressIndex::New).unwrap();
    // print the address
    println!("Address: {}", address);
    wallet  
}

