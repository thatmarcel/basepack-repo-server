use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};
use crate::sileo::sileo_depiction_view_alignment::SileoDepictionViewAlignment;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionHeaderView")]
pub struct SileoDepictionHeaderView {
    pub title: String,
    #[sdv_rename("useMargins")]
    pub should_use_margins: Option<bool>,
    #[sdv_rename("useBottomMargin")]
    pub should_use_bottom_margin: Option<bool>,
    #[sdv_rename("useBoldText")]
    pub should_use_bold_text: Option<bool>,
    pub alignment: Option<SileoDepictionViewAlignment>
}