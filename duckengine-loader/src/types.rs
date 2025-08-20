use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Int,
    Num,
    Str,
}
