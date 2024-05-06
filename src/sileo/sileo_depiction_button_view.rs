use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};
use crate::sileo::sileo_depiction_view::SileoDepictionView;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionButtonView")]
pub struct SileoDepictionButtonView {
    #[sdv_rename("text")]
    pub title: String,
    #[sdv_rename("action")]
    pub link_url: String,
    #[sdv_rename("backupAction")]
    pub fallback_link_url: Option<String>,
    #[sdv_rename("openExternal")]
    pub should_open_in_external_app: Option<bool>,
    #[sdv_rename("yPadding")]
    pub vertical_padding: Option<f64>,
    #[sdv_rename("tintColor")]
    pub tint_color: Option<String>,
    #[sdv_rename("view")]
    pub view_to_use_instead_of_title: Option<Box<dyn SileoDepictionView>>
}