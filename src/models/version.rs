use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Version {
  pub version: String,
}
