use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};
use crate::sileo::sileo_depiction_view_alignment::SileoDepictionViewAlignment;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionImageView")]
pub struct SileoDepictionImageView {
    #[sdv_rename("URL")]
    pub url: String,
    pub width: f64,
    pub height: f64,
    #[sdv_rename("cornerRadius")]
    pub corner_radius: f64,
    pub alignment: Option<SileoDepictionViewAlignment>,
    #[sdv_rename("xPadding")]
    pub horizontal_padding: Option<f64>
}