use crate::models::Error;
use url::Url;

pub struct IsuAssociationConfigForm {
    pub name: NameString,
    pub url: Url,
}

impl IsuAssociationConfigForm {
    pub fn build(name: String, url: String) -> Result<Self, Error> {
        let url = Url::parse(&url)?;
        let name = NameString::parse(name)?;
        Ok(Self { name, url })
    }
}

pub struct NameString(String);

impl NameString {
    pub fn parse(name: String) -> Result<Self, Error> {
        Ok(Self { 0: name })
    }
}

impl ToString for NameString {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
