use crate::data::items::legacy_package::LegacyPackage;
use crate::data::items::package::{Package, PackageChangelogEntry};
use crate::data::traits::into_package::IntoPackage;

impl IntoPackage for LegacyPackage {
    fn into_package(self) -> Package {
        Package {
            identifier: self.identifier,
            size: self.size,
            icon_url: self.icon_url,
            package_name: self.package_name,
            long_description: self.long_description,
            short_description: self.short_description,
            name: self.name,
            is_approved: self.approved == "yes",
            author_name: self.author_name,
            section: self.section,
            screenshots: self.screenshots,
            hash_sha1: self.hash_sha1,
            hash_md5: self.hash_md5,
            hash_sha256: self.hash_sha256,
            dependencies: self.dependencies,
            header_image_url: self.header_image_url,
            version: self.version,
            download_url: self.download_url,
            is_featured: self.is_featured,
            author_email: self.author_email,
            min_os: self.min_os,
            featured_image_url: self.featured_image_url,
            price: self.price,
            installed_size: self.installed_size.unwrap_or_else(|| -1),
            conflicts: self.conflicts,
            changelog: match self.changelog {
                Some(cl) => Some(cl.iter().map(|cle| {
                    PackageChangelogEntry {
                        version: cle.version.clone(),
                        content: cle.content.clone()
                    }
                }).collect::<Vec<PackageChangelogEntry>>()),
                None => None
            }
        }
    }
}