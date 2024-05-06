use serde::{Serialize, Serializer};

#[derive(Clone)]
pub struct SileoItemSize {
    pub width: u16,
    pub height: u16
}

impl Serialize for SileoItemSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(format!("{{{},{}}}", self.width, self.height).as_str())
    }
}