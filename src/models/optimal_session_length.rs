use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy)]
pub enum OptimalSessionLength {
    Short,
    Medium,
    Long,
}
