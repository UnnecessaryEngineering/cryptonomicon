#[derive(Clone, Debug, Eq, PartialEq)]
pub struct X500Subject {
    common_name: String,
}

impl X500Subject {
    pub fn new<S>(common_name: S) -> Self where S: Into<String>{
        Self {
            common_name: common_name.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn x500subject_new() {
        let expected = "Test CN";
        let subject = X500Subject::new(expected);
        let actual = subject.common_name;
        assert_eq!(expected, actual);
    }
}