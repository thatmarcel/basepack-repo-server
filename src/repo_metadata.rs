pub static REPO_NAME: &str = "Basepack";
pub static REPO_SUITE: &str = "stable";
pub static REPO_VERSION: &str = "1.0";
pub static REPO_CODENAME: &str = "ios";
pub static REPO_ARCHITECTURES: &str = "iphoneos-arm iphoneos-arm64";
pub static REPO_COMPONENTS: &str = "main";
pub static REPO_DESCRIPTION: &str = "Unpack the power.";

pub static REPO_TINT_COLOR: &str = "#4299e1";

// The root path is redirected to this website
pub static REPO_WEBSITE_URL: &str = "https://basepack.co";

#[macro_export]
macro_rules! repo_package_depiction_url {
    ($package_identifier: expr) => {
        format!("https://basepack.co/d/{}", $package_identifier)
    }
}
#[macro_export]
macro_rules! repo_package_details_website_url {
    ($package_identifier: expr) => {
        format!("https://basepack.co/p/{}", $package_identifier)
    }
}

#[macro_export]
macro_rules! repo_package_support_website_url {
    ($package_identifier: expr) => {
        format!("https://basepack.co/p/{}/support", $package_identifier)
    }
}