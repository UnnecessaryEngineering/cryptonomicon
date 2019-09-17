use cryptonomicon_asymmetric::KeyPair;

pub mod curve25519;

#[derive(Debug)]
pub enum Error {
    RandomFailure { cause: rand::Error },
}

impl From<rand::Error> for Error {
    fn from(cause: rand::Error) -> Self {
        Self::RandomFailure { cause }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum EllipticCurve {
    Curve25519,
}

impl EllipticCurve {
    pub fn generate_keypair(self) -> Result<Box<dyn KeyPair>> {
        Ok(Box::new(match self {
            Self::Curve25519 => curve25519::generate_keypair()?,
        }))
    }
}
