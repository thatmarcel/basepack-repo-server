use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::data::items::package::Package;
use crate::sileo::sileo_depiction_font_weight::SileoDepictionFontWeight;
use crate::sileo::sileo_depiction_label_view::SileoDepictionLabelView;
use crate::sileo::sileo_depiction_markdown_view::SileoDepictionMarkdownView;
use crate::sileo::sileo_depiction_screenshot::SileoDepictionScreenshot;
use crate::sileo::sileo_depiction_screenshots_view::SileoDepictionScreenshotsView;
use crate::sileo::sileo_depiction_separator_view::SileoDepictionSeparatorView;
use crate::sileo::sileo_depiction_stack_view::SileoDepictionStackView;
use crate::sileo::sileo_depiction_subheader_view::SileoDepictionSubheaderView;
use crate::sileo::sileo_depiction_tab_view::SileoDepictionTabView;
use crate::sileo::sileo_depiction_table_button_view::SileoDepictionTableButtonView;
use crate::sileo::sileo_depiction_table_text_view::SileoDepictionTableTextView;
use crate::sileo::sileo_depiction_view::SileoDepictionView;
use crate::sileo::sileo_depiction_view_margins::SileoDepictionViewMargins;
use crate::sileo::sileo_item_size::SileoItemSize;
use crate::sileo::trait_implementations::nested_sileo_depiction_views_join::NestedSileoDepictionViewsJoin;

pub async fn sileo_depiction_route(Path(package_identifier): Path<String>) -> Response {
    let package = Package::get(&*package_identifier).await;

    let package = match package {
        Some(p) => p,
        None => return StatusCode::NOT_FOUND.into_response()
    };
    
    let mut tabs: Vec<Box<dyn SileoDepictionView>> = vec![
        SileoDepictionStackView {
            name: "Details".to_string(),
            views: vec![
                SileoDepictionSubheaderView {
                    title: package.short_description,
                    should_use_margins: None,
                    should_use_bottom_margin: Some(false),
                    should_use_bold_text: Some(true)
                }.into_box(),
                
                SileoDepictionScreenshotsView {
                    screenshots: package.screenshots.iter().map(|screenshot_url| {
                        SileoDepictionScreenshot {
                            url: screenshot_url.clone(),
                            accessibility_text: "Screenshot".to_string(),
                            is_video: Some(false)
                        }
                    }).collect::<Vec<SileoDepictionScreenshot>>(),
                    item_corner_radius: 6f64,
                    item_size: SileoItemSize {
                        width: 160,
                        height: 346
                    },
                    replace_with_on_iphone: None,
                    replace_with_on_ipad: None
                }.into_box(),
                
                SileoDepictionMarkdownView {
                    markdown_text: package.long_description,
                    should_use_margins: None,
                    should_use_vertical_padding: Some(true),
                    should_use_html: None,
                    link_color: None
                }.into_box(),
                
                SileoDepictionTableTextView {
                    title: "Version".to_string(),
                    text: package.version
                }.into_box(),
                
                SileoDepictionTableButtonView {
                    title: "Support".to_string(),
                    link_url: crate::repo_package_support_website_url!(package.identifier).to_string(),
                    fallback_link_url: None,
                    should_open_in_external_app: Some(false),
                    vertical_padding: None,
                    tint_color: None
                }.into_box()
            ],
            orientation: None,
            horizontal_padding: None
        }.into_box()
    ];
    
    match package.changelog {
        Some(cl) => {
            let views: Vec<Vec<Box<dyn SileoDepictionView>>> = cl.iter().map(|cle| {
                vec![
                    SileoDepictionLabelView {
                        text: format!("Version {}", cle.version).to_string(),
                        margins: Some(
                            SileoDepictionViewMargins {
                                top: 16,
                                left: 0,
                                bottom: 8,
                                right: 0
                            }
                        ),
                        should_use_margins: Some(true),
                        should_use_vertical_padding: None,
                        font_weight: Some(SileoDepictionFontWeight::SemiBold),
                        font_size: Some(21f64),
                        text_color: None,
                        alignment: None
                    }.into_box() as Box<dyn SileoDepictionView>,
                    
                    SileoDepictionMarkdownView {
                        markdown_text: cle.content.to_string(),
                        should_use_margins: Some(true),
                        should_use_vertical_padding: None,
                        should_use_html: None,
                        link_color: None
                    }.into_box() as Box<dyn SileoDepictionView>
                ]
            }).collect::<Vec<_>>();
            
            let views = Vec::join_with(views, vec![
                SileoDepictionSeparatorView {}.into_box()
            ]);
            
            tabs.push(
                SileoDepictionStackView {
                    name: "Changelog".to_string(),
                    views,
                    orientation: None,
                    horizontal_padding: None
                }.into_box()
            );
        },
        None => {}
    };

    let result = SileoDepictionTabView {
        header_image_url: Some(package.header_image_url),
        link_color: Some(crate::repo_metadata::REPO_TINT_COLOR.to_string()),
        background_color: None,
        tabs
    };

    result.into_response()
}