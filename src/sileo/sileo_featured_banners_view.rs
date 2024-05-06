use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};

use crate::sileo::sileo_featured_banner::SileoFeaturedBanner;
use crate::sileo::sileo_item_size::SileoItemSize;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("FeaturedBannersView")]
pub struct SileoFeaturedBannersView {
    #[sdv_rename("itemSize")]
    pub item_size: SileoItemSize,
    #[sdv_rename("itemCornerRadius")]
    pub item_corner_radius: u16,
    pub banners: Vec<SileoFeaturedBanner>
}