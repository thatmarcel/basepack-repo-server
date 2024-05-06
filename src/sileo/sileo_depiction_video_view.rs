use basepack_repo_server_proc_macros::{sdv_class_name, SileoDepictionView};
use crate::sileo::sileo_depiction_view_alignment::SileoDepictionViewAlignment;

#[derive(SileoDepictionView, Clone)]
#[sdv_class_name("DepictionVideoView")]
pub struct SileoDepictionVideoView {
    #[sdv_rename("URL")]
    pub url: String,
    pub width: f64,
    pub height: f64,
    #[sdv_rename("cornerRadius")]
    pub corner_radius: f64,
    pub alignment: Option<SileoDepictionViewAlignment>,
    #[sdv_rename("autoplay")]
    pub should_autoplay: Option<bool>,
    #[sdv_rename("showPlaybackControls")]
    pub should_show_playback_controls: Option<bool>,
    #[sdv_rename("loop")]
    pub should_loop: Option<bool>
}