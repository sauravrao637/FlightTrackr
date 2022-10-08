use serde::{Deserialize, Serialize};

use super::airport_code::AirportCode;

#[derive(Debug, Serialize, Deserialize)]
pub struct Flight {
    pub starting: AirportCode,
    pub ending: AirportCode,
}

impl std::fmt::Display for Flight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write! {f,"starting:{}, ending: {}",self.starting, self.ending}
    }
}
