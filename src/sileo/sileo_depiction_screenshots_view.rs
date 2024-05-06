use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};
use crate::sileo::sileo_depiction_screenshot::SileoDepictionScreenshot;
use crate::sileo::sileo_item_size::SileoItemSize;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionScreenshotsView")]
pub struct SileoDepictionScreenshotsView {
    pub screenshots: Vec<SileoDepictionScreenshot>,
    #[sdv_rename("itemCornerRadius")]
    pub item_corner_radius: f64,
    #[sdv_rename("itemSize")]
    pub item_size: SileoItemSize,
    #[sdv_rename("iphone")]
    pub replace_with_on_iphone: Option<Box<SileoDepictionScreenshotsView>>,
    #[sdv_rename("ipad")]
    pub replace_with_on_ipad: Option<Box<SileoDepictionScreenshotsView>>
}