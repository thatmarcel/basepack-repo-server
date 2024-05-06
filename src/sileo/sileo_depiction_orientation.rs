use serde::{Serialize, Serializer};

#[allow(dead_code)]
#[derive(Clone)]
pub enum SileoDepictionOrientation {
    Landscape,
    Portrait
}

impl Serialize for SileoDepictionOrientation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(match self {
            SileoDepictionOrientation::Landscape => "landscape",
            SileoDepictionOrientation::Portrait => "portrait"
        })
    }
}