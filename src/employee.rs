use crate::email_address::EmailAddress;
use crate::phone_number::PhoneNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Employee {
    first_name: Option<String>,
    last_name: Option<String>,
    pub email: Option<EmailAddress>,
    pub phone: Option<PhoneNumber>,
}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email
    }
}
