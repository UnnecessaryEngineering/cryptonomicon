use cryptonomicon_asymmetric::{PublicKey, PrivateKey, KeyPair};

pub struct ECPublicKey;

impl PublicKey for ECPublicKey {
}

pub struct ECPrivateKey;

impl PrivateKey for ECPrivateKey {
}

pub struct ECKeyPair {
    public_key: ECPublicKey,
    private_key: ECPrivateKey,
}

impl ECKeyPair {
    pub fn generate_prime256_v1() -> Self {
        Self {
            public_key: ECPublicKey {},
            private_key: ECPrivateKey {},
        }
    }
}

impl KeyPair for ECKeyPair {
    fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    fn private_key(&self) -> &PrivateKey {
        &self.private_key
    }
}