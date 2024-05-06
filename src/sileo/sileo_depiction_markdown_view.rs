use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionMarkdownView")]
pub struct SileoDepictionMarkdownView {
    #[sdv_rename("markdown")]
    pub markdown_text: String,
    #[sdv_rename("useMargins")]
    pub should_use_margins: Option<bool>,
    #[sdv_rename("useSpacing")]
    pub should_use_vertical_padding: Option<bool>,
    #[sdv_rename("useRawFormat")]
    pub should_use_html: Option<bool>,
    #[sdv_rename("tintColor")]
    pub link_color: Option<String>,
}