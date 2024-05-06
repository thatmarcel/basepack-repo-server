use crate::data::items::legacy_package::LegacyPackage;
use crate::data::items::package::Package;
use crate::data::traits::into_package::IntoPackage;
use crate::data::traits::into_packages::IntoPackages;

impl IntoPackages for Vec<LegacyPackage> {
    fn into_packages(self) -> Vec<Package> {
        let mut packages: Vec<Package> = Vec::new();

        for legacy_package in self {
            packages.push(legacy_package.into_package());
        }

        packages
    }
}