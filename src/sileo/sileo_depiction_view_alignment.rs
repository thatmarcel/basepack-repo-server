use serde::{Serialize, Serializer};

#[allow(dead_code)]
#[derive(Clone)]
pub enum SileoDepictionViewAlignment {
    Left,
    Center,
    Right
}

impl Serialize for SileoDepictionViewAlignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_u8(match self {
            SileoDepictionViewAlignment::Left => 0,
            SileoDepictionViewAlignment::Center => 1,
            SileoDepictionViewAlignment::Right => 2
        })
    }
}