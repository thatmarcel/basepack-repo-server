use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};
use crate::sileo::sileo_depiction_view_alignment::SileoDepictionViewAlignment;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionRatingView")]
pub struct SileoDepictionRatingView {
    #[sdv_rename("rating")]
    pub rating_star_count: f64,
    pub alignment: SileoDepictionViewAlignment
}