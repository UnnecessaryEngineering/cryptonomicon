#![no_std]

pub trait PublicKey {
}

pub trait PrivateKey {
}

pub trait KeyPair {
    fn public_key(&self) -> &PublicKey;
    fn private_key(&self) -> &PrivateKey;
}