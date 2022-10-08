use serde::{Deserialize, Serialize};

use super::flight::Flight;

#[derive(Debug, Serialize, Deserialize)]
pub struct Flights(pub Vec<Flight>);
