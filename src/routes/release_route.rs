use crate::kv;

pub async fn release_route() -> String {
    kv! {
        "Origin": "Basepack",
        "Label": "Basepack",
        "Suite": "stable",
        "Version": "1.0",
        "Codename": "ios",
        "Architectures": "iphoneos-arm iphoneos-arm64",
        "Components": "main",
        "Description": "Unpack the Power.",
    }
}