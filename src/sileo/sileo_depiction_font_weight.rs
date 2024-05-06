use serde::{Serialize, Serializer};

#[allow(dead_code)]
#[derive(Clone)]
pub enum SileoDepictionFontWeight {
    Black,
    Bold,
    Heavy,
    Light,
    Medium,
    SemiBold,
    Regular,
    Normal,
    Thin,
    UltraLight
}

impl Serialize for SileoDepictionFontWeight {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(match self {
            SileoDepictionFontWeight::Black => "black",
            SileoDepictionFontWeight::Bold => "bold",
            SileoDepictionFontWeight::Heavy => "heavy",
            SileoDepictionFontWeight::Light => "light",
            SileoDepictionFontWeight::Medium => "medium",
            SileoDepictionFontWeight::SemiBold => "semibold",
            SileoDepictionFontWeight::Regular => "regular",
            SileoDepictionFontWeight::Normal => "normal",
            SileoDepictionFontWeight::Thin => "thin",
            SileoDepictionFontWeight::UltraLight => "ultralight",
        })
    }
}