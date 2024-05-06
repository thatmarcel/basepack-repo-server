use crate::data::items::package::Package;

pub trait IntoPackages {
    fn into_packages(self) -> Vec<Package>;
}