use crate::blockchain::transaction::Transaction;

mod blockchain;


fn main() {


    // Alice wants to send 0.3 BTC to Bob
    let transaction = Transaction::new(String::from("Angelos"), String::from("Bill"), 34.4, );

    // Alice's Private Key (Example - NEVER USE IN REAL LIFE)
    let private_key = "c8a2f842e2a3b8df4d450ddbd617f498c2f24b4b5f59c598d98b4dca15b3be62";

    match Transaction::sing_transaction(&transaction, private_key) {
        Ok(signed_tx) => println!("✅ Signed Transaction: {:?}", signed_tx),
        Err(e) => println!("❌ Error signing transaction: {}", e),
    }
}
