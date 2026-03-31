use leptos::prelude::RenderHtml;
use leptos_basic_template::{APP_HEADING, App, BROWSER_ONLY_MESSAGE};

#[test]
fn given_basic_app_when_rendered_to_html_then_root_markup_should_match_expected_structure() {
    let html = App().to_html();

    assert_eq!(html, format!("<div><h1>{APP_HEADING}</h1></div>"));
}

#[test]
fn given_basic_app_when_rendered_to_html_then_heading_copy_should_include_expected_title() {
    let html = App().to_html();

    assert!(html.contains(APP_HEADING));
}

#[test]
fn given_native_environment_when_requesting_browser_only_message_then_text_should_match_expected_copy()
 {
    assert_eq!(
        BROWSER_ONLY_MESSAGE,
        "This example only works when compiled to WebAssembly."
    );
}
