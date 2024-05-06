use crate::data::items::package::Package;
use crate::data::traits::into_kv::IntoKV;
use crate::kv;

impl IntoKV for Vec<Package> {
    fn into_kv(self) -> String {
        let mut package_strings: Vec<String> = Vec::new();
        
        for package in self {
            let long_description = &package.long_description
                .split("\n\n")
                .collect::<Vec<&str>>()
                .join("\n.")
                .split("\n")
                .collect::<Vec<&str>>()
                .join("\n ");

            let description = format!("{}\n {}", &package.short_description, &long_description);

            let package_string = kv! {
            "Package": &package.identifier,
            "Version": &package.version,
            "Architecture": "iphoneos-arm",
            "Maintainer": format!("{} <{}>", &package.author_name, &package.author_email),
            "Author": format!("{} <{}>", &package.author_name, &package.author_email),
            "Tag": format!("compatible_min::ios{}", &package.min_os),
            "Depends": &package.dependencies.join(", "),
            "Conflicts": &package.conflicts.join(", "),
            "Filename": format!("download/{}.deb", &package.identifier),
            "Section": "Tweaks",
            "Description": &description,
            "Depiction": format!("https://basepack.co/d/{}", &package.identifier),
            "Icon": &package.icon_url,
            "Name": &package.name,
            "SHA256": &package.hash_sha256,
            "SHA1": &package.hash_sha1,
            "MD5sum": &package.hash_md5,
            "Installed-Size": match &package.installed_size {
                -1 => String::new(),
                x => x.to_string()
            },
            "Size": &package.size,
            "SileoDepiction": format!("https://repo.basepack.co/sileo-depictions/{}", &package.identifier),
            "Prefer-Native": "Yes",
            "Header": &package.header_image_url,
            "Previews": &package.screenshots.join(","),
            "Homepage": format!("https://basepack.co/p/{}", &package.identifier),
            "Support": format!("https://basepack.co/p/{}/support", &package.identifier),
        };

            package_strings.push(package_string);
        }

        package_strings.join("\n\n")
    }
}