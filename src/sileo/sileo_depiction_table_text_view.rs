use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionTableTextView")]
pub struct SileoDepictionTableTextView {
    pub title: String,
    pub text: String
}