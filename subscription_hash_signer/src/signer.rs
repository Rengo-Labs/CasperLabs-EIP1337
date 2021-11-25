use ed25519_dalek::{
  Signer,
};

use types::{
  AsymmetricType, 
  SecretKey, 
  crypto:: {
    PublicKey,
    Signature,
  },
};

use hex;

use std::env;
use std::fs;

mod der;

pub fn sign(secret_key: SecretKey, message_bytes: [u8;32]) -> String {
  match secret_key {
    SecretKey::Ed25519(secret_key) => {
        let pub_key: ed25519_dalek::PublicKey = (&secret_key).into();
        let pair = ed25519_dalek::Keypair{
            secret: secret_key,
            public: pub_key,
        };

        let signature: ed25519_dalek::Signature = pair.sign(&message_bytes);
        hex::encode(signature.to_bytes())
    },
    _ => panic!("secret key should be a Ed25519"),
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  assert_eq!(args.len(), 3);

  let message_key_str = &args[2];
  let mut message_bytes =  [0u8;32];
  hex::decode_to_slice(message_key_str, &mut message_bytes as &mut [u8]).unwrap();

  let secret_key_file = &args[1];
  let secret_key_str = fs::read_to_string(secret_key_file).expect("Secret key file could not be read.");
  let secret_key = der::from_pem(secret_key_str.clone().as_bytes()).unwrap();

  let public_key: PublicKey = (&secret_key).into();

  let signature_hex = sign(secret_key, message_bytes);

  println!("Account Hash {:?}", public_key.to_account_hash().to_formatted_string());
  println!("Public Key Hex {:?}", public_key.to_hex());
  println!("Signature {:?}", signature_hex);

  if let PublicKey::Ed25519(ed_public_key) = public_key {
    let mut signature_bytes = [0u8;64];
    hex::decode_to_slice(signature_hex, &mut signature_bytes as &mut [u8]).unwrap();

    let signature = Signature::ed25519(signature_bytes).unwrap();
    if let Signature::Ed25519(ed_signature) = signature 
    {
      ed_public_key.verify_strict(&message_bytes, &ed_signature).unwrap();
    
      println!("Signature Verification Success");
    } else {
      println!("Signature Verification Failed");
    }
  } else {
    println!("Signature Verification Failed");
  }
}
