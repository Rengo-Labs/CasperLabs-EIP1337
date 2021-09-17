use ed25519_dalek::{
  Keypair,
  PublicKey,
  Signature,
  Signer,
};

use types::{
  SecretKey,
};

pub fn sign(secret_key: SecretKey, message_bytes: [u8;32]) -> String {
  match secret_key {
    SecretKey::Ed25519(secret_key) => {
        let pub_key: PublicKey = (&secret_key).into();
        let pair = Keypair{
            secret: secret_key,
            public: pub_key,
        };

        let signature: Signature = pair.sign(&message_bytes);
        hex::encode(signature.to_bytes())
    },
    _ => panic!("secret key should be a Ed25519"),
  }
}