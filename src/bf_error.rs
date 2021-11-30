use std::fmt::Display;

#[derive(Debug)]
pub struct BfError {
    details: String,
}

impl BfError {
    pub fn new(details: &str) -> Self {
        BfError {
            details: details.to_string(),
        }
    }

    pub fn details(&self) -> &str {
        self.details.as_ref()
    }
}

impl Display for BfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}
