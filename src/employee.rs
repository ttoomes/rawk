use crate::email_address::EmailAddress;
use crate::phone_number::PhoneNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Employee {
    first_name: Option<String>,
    last_name: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub email: Option<EmailAddress>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub phone: Option<PhoneNumber>,
}
