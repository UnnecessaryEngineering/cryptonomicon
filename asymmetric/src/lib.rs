pub trait PublicKey {}

pub trait KeyPair {
    fn public_key(&self) -> Box<dyn PublicKey>;
}
