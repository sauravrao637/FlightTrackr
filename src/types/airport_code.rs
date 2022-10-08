use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AirportCode(pub String);

impl std::fmt::Display for AirportCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write! {f,"{}",self.0}
    }
}
