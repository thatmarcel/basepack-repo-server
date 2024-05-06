use crate::data::items::package::Package;

pub trait IntoPackage {
    fn into_package(self) -> Package;
}