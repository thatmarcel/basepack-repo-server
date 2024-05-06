use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};
use crate::sileo::sileo_depiction_view::SileoDepictionView;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionTabView")]
pub struct SileoDepictionTabView {
    #[sdv_rename("headerImage")]
    pub header_image_url: Option<String>,
    #[sdv_rename("tintColor")]
    pub link_color: Option<String>,
    #[sdv_rename("backgroundColor")]
    pub background_color: Option<String>,
    pub tabs: Vec<Box<dyn SileoDepictionView>>
}