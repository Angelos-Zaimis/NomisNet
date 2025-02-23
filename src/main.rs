use rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use crate::blockchain::transaction::Transaction;

mod blockchain;


fn main() {
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    // Generate a random private key
    let private_key = SecretKey::new(&mut rng);
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    let private_key_hex = hex::encode(private_key.secret_bytes());

    let transaction = Transaction::new(
        "Alice".to_string(),
        "Bob".to_string(),
        10.0, // Sending 10 coins
    );

    // Sign the transaction
    match Transaction::sing_transaction(&transaction, &private_key_hex) {
        Ok(signed_tx) => {
            println!("✅ Transaction Signed Successfully");
            println!("📜 Transaction: {:?}", signed_tx.transaction);
            println!("✍️  Signature: {}", hex::encode(&signed_tx.signature));

            // Verify the transaction
            let is_valid = Transaction::verify_transaction(&public_key, &signed_tx.transaction, &signed_tx.signature);
            if is_valid {
                println!("🎉 Signature successfully verified!");
            } else {
                println!("❌ Signature verification failed.");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}
