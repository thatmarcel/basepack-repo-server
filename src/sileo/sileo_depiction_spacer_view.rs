use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionSpacerView")]
pub struct SileoDepictionSpacerView {
    pub spacing: f64
}