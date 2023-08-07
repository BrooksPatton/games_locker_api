use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Email(pub String);

impl Deref for Email {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
