use chrono::prelude::*;
use cryptonomicon_asymmetric::PublicKey;
use cryptonomicon_x500::X500Subject;

pub struct X509Certificate<'a> {
    pub serial: u64,
    pub subject: X500Subject,
    pub public_key: Box<&'a dyn PublicKey>,
    pub issuer: X500Subject,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
}

impl<'a> X509Certificate<'a> {
    pub fn new(
        serial: u64,
        subject: X500Subject,
        public_key: &'a dyn PublicKey,
        issuer: X500Subject,
        not_before: DateTime<Utc>,
        not_after: DateTime<Utc>,
    ) -> Self {
        Self {
            serial,
            subject,
            public_key: Box::new(public_key),
            issuer,
            not_before,
            not_after,
        }
    }
}
