use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};
use crate::sileo::sileo_depiction_font_weight::SileoDepictionFontWeight;
use crate::sileo::sileo_depiction_view_alignment::SileoDepictionViewAlignment;
use crate::sileo::sileo_depiction_view_margins::SileoDepictionViewMargins;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionLabelView")]
pub struct SileoDepictionLabelView {
    pub text: String,
    pub margins: Option<SileoDepictionViewMargins>,
    #[sdv_rename("useMargins")]
    pub should_use_margins: Option<bool>,
    #[sdv_rename("usePadding")]
    pub should_use_vertical_padding: Option<bool>,
    #[sdv_rename("fontWeight")]
    pub font_weight: Option<SileoDepictionFontWeight>,
    #[sdv_rename("fontSize")]
    pub font_size: Option<f64>,
    #[sdv_rename("textColortextColor")]
    pub text_color: Option<String>,
    pub alignment: Option<SileoDepictionViewAlignment>
}