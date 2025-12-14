use serde::{Deserialize, Serialize};
/// Control commands flowing across the control plane (Kafka bus, etc.)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum CommandType {
    #[serde(rename = "load_model")]
    LoadModel,
}
