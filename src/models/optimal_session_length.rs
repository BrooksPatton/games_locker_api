use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OptimalSessionLength {
    Short,
    Medium,
    Long,
}
