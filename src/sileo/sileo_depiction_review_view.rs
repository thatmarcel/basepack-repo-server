use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionReviewView")]
pub struct SileoDepictionReviewView {
    pub title: String,
    pub author: String,
    #[sdv_rename("markdown")]
    pub markdown_text: String,
    #[sdv_rename("rating")]
    pub rating_star_count: Option<f64>,
    #[sdv_rename("tintColor")]
    pub tint_color: Option<String>
}