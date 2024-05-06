use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};
use crate::sileo::sileo_depiction_orientation::SileoDepictionOrientation;
use crate::sileo::sileo_depiction_view::SileoDepictionView;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionStackView")]
pub struct SileoDepictionStackView {
    #[sdv_rename("tabname")]
    pub name: String,
    pub views: Vec<Box<dyn SileoDepictionView>>,
    pub orientation: Option<SileoDepictionOrientation>,
    #[sdv_rename("xPadding")]
    pub horizontal_padding: Option<f64>
}