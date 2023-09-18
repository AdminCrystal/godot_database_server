use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct DevMessage {
    pub message: String
}
