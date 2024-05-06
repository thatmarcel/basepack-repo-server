use crate::kv;

pub async fn release_route() -> String {
    kv! {
        "Origin": crate::repo_metadata::REPO_NAME,
        "Label": crate::repo_metadata::REPO_NAME,
        "Suite": crate::repo_metadata::REPO_SUITE,
        "Version": crate::repo_metadata::REPO_VERSION,
        "Codename": crate::repo_metadata::REPO_CODENAME,
        "Architectures": crate::repo_metadata::REPO_ARCHITECTURES,
        "Components": crate::repo_metadata::REPO_COMPONENTS,
        "Description": crate::repo_metadata::REPO_DESCRIPTION,
    }
}