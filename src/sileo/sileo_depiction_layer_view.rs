use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};

use crate::sileo::sileo_depiction_view::SileoDepictionView;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionLayerView")]
pub struct SileoDepictionLayerView {
    pub views: Vec<Box<dyn SileoDepictionView>>,
    #[sdv_rename("tintColor")]
    pub link_color: Option<String>
}