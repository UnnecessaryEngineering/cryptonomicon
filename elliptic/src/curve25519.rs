use crate::Result;
use cryptonomicon_asymmetric::{KeyPair, PublicKey};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

pub fn generate_keypair() -> Result<Curve25519KeyPair> {
    let mut csprng: OsRng = OsRng::new()?;
    let keypair: Keypair = Keypair::generate(&mut csprng);
    Ok(Curve25519KeyPair { keypair })
}

#[derive(Debug)]
pub struct Curve25519KeyPair {
    keypair: Keypair,
}

impl KeyPair for Curve25519KeyPair {
    fn public_key(&self) -> Box<dyn PublicKey> {
        Box::new(Curve25519PublicKey {})
    }
}

#[derive(Clone, Debug)]
pub struct Curve25519PublicKey {}

impl PublicKey for Curve25519PublicKey {}
