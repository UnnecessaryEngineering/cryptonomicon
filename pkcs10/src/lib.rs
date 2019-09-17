use cryptonomicon_asymmetric::PublicKey;
use cryptonomicon_x500::X500Subject;

pub struct PKCS10<'a> {
    pub subject: X500Subject,
    pub public_key: Box<&'a dyn PublicKey>,
}

impl<'a> PKCS10<'a> {
    pub fn new(subject: X500Subject, public_key: &'a dyn PublicKey) -> Self {
        Self { subject, public_key: Box::new(public_key) }
    }
}