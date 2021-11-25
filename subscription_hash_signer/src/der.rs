use types::{
  crypto::{
    SecretKey,
    SYSTEM_TAG,
    ED25519_TAG, 
    SECP256K1_TAG,
  },
};

use pem;

use derp::{
    Tag,
};

use untrusted::Input;

mod errors;
use errors::Error;

const ED25519_OBJECT_IDENTIFIER: [u8; 3] = [43, 101, 112];
const ED25519_PEM_SECRET_KEY_TAG: &str = "PRIVATE KEY";

const SECP256K1_OBJECT_IDENTIFIER: [u8; 5] = [43, 129, 4, 0, 10];
const SECP256K1_PEM_SECRET_KEY_TAG: &str = "EC PRIVATE KEY";

pub fn from_pem<T: AsRef<[u8]>>(input: T) -> Result<SecretKey, Error> {
    let pem = pem::parse(input)?;

    let secret_key = from_der(&pem.contents)?;

    let bad_tag = |expected_tag: &str| {
        Error::FromPem(format!(
            "invalid tag: expected {}, got {}",
            expected_tag, pem.tag
        ))
    };

    match secret_key {
        SecretKey::System => return Err(Error::System(String::from("from_pem"))),
        SecretKey::Ed25519(_) => {
            if pem.tag != ED25519_PEM_SECRET_KEY_TAG {
                return Err(bad_tag(ED25519_PEM_SECRET_KEY_TAG));
            }
        }
        SecretKey::Secp256k1(_) => {
            if pem.tag != SECP256K1_PEM_SECRET_KEY_TAG {
                return Err(bad_tag(SECP256K1_PEM_SECRET_KEY_TAG));
            }
        }
    }

    Ok(secret_key)
}

pub fn from_der<T: AsRef<[u8]>>(input: T) -> Result<SecretKey, Error> {
    let input = Input::from(input.as_ref());

    let (key_type_tag, raw_bytes) = input.read_all(derp::Error::Read, |input| {
    derp::nested(input, Tag::Sequence, |input| {
          // Safe to ignore the first value which should be an integer.
          let version_slice = derp::expect_tag_and_get_value(input, Tag::Integer)?.as_slice_less_safe();
          if version_slice.len() != 1 {
              return Err(derp::Error::NonZeroUnusedBits);
          }
          let version = version_slice[0];

          // Read the next value.
          let (tag, value) = derp::read_tag_and_get_value(input)?;
          if tag == Tag::Sequence as u8 {
              // Expecting an Ed25519 key.
              if version != 0 {
                  return Err(derp::Error::WrongValue);
              }
              // The sequence should have one element: an object identifier defining Ed25519.
              let object_identifier = value.read_all(derp::Error::Read, |input| {
                  derp::expect_tag_and_get_value(input, Tag::Oid)
              })?;

              if object_identifier.as_slice_less_safe() != ED25519_OBJECT_IDENTIFIER {
                  return Err(derp::Error::WrongValue);
              }

              // The third and final value should be the raw bytes of the secret key as an
              // octet string in an octet string.
              let raw_bytes = derp::nested(input, Tag::OctetString, |input| {
                  derp::expect_tag_and_get_value(input, Tag::OctetString)
              })?
              .as_slice_less_safe();

              return Ok((ED25519_TAG, raw_bytes));
          } else if tag == Tag::OctetString as u8 {
              // Expecting a secp256k1 key.
              if version != 1 {
                  return Err(derp::Error::WrongValue);
              }

              // The octet string is the secret key.
              let raw_bytes = value.as_slice_less_safe();

              // The object identifier is next.
              let parameter0 =
                  derp::expect_tag_and_get_value(input, Tag::ContextSpecificConstructed0)?;
              let object_identifier = parameter0.read_all(derp::Error::Read, |input| {
                  derp::expect_tag_and_get_value(input, Tag::Oid)
              })?;
              if object_identifier.as_slice_less_safe() != SECP256K1_OBJECT_IDENTIFIER {
                  return Err(derp::Error::WrongValue);
              }

              // There might be an optional public key as the final value, but we're not
              // interested in parsing that.  Read it to ensure `input.read_all` doesn't fail
              // with unused bytes error.
              let _ = derp::read_tag_and_get_value(input);

              return Ok((SECP256K1_TAG, raw_bytes));
          }

          Err(derp::Error::WrongValue)
      })
  })?;

  match key_type_tag {
      SYSTEM_TAG => Err(Error::AsymmetricKey("cannot construct variant".to_string())),
      ED25519_TAG => SecretKey::ed25519_from_bytes(raw_bytes).map_err(Into::into),
      SECP256K1_TAG => SecretKey::secp256k1_from_bytes(raw_bytes).map_err(Into::into),
      _ => Err(Error::AsymmetricKey("unknown type tag".to_string())),
  }
}
