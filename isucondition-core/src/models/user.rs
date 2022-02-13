use std::fmt::{Display, Formatter};

#[derive(Debug, sqlx::Type)]
pub struct UserID(String);

impl UserID {
    pub fn new(str: String) -> Self {
        Self(str)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Display for UserID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
