use crate::models::user::UserID;
use std::fmt::{Display, Formatter};

#[derive(Debug, sqlx::Type)]
pub struct IsuID(i64);

impl Display for IsuID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, sqlx::Type, PartialEq)]
pub struct IsuUUID(String);

impl IsuUUID {
    pub fn new(str: String) -> Self {
        Self(str)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Display for IsuUUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Isu {
    pub id: IsuID,
    pub jia_user_id: UserID,
    pub jia_isu_uuid: IsuUUID,
    pub name: String,
    pub character: Option<String>,
}
