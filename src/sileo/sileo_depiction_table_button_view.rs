use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionTableButtonView")]
pub struct SileoDepictionTableButtonView {
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
    pub tint_color: Option<String>
}