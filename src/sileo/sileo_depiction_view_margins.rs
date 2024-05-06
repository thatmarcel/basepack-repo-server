use serde::{Serialize, Serializer};

#[derive(Clone)]
pub struct SileoDepictionViewMargins {
    pub top: u16,
    pub left: u16,
    pub bottom: u16,
    pub right: u16
}

impl Serialize for SileoDepictionViewMargins {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(format!("{{{},{},{},{}}}", self.top, self.left, self.bottom, self.right).as_str())
    }
}