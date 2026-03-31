use leptos::prelude::RenderHtml;
use leptos_basic_template::{APP_HEADING, App, BROWSER_ONLY_MESSAGE};
use std::process::Command;

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

#[test]
fn given_native_binary_when_executed_then_stdout_should_match_browser_only_message() {
    let output = Command::new(env!("CARGO_BIN_EXE_leptos-basic-template"))
        .output()
        .expect("the native binary should execute successfully during integration tests");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout should be valid UTF-8"),
        format!("{BROWSER_ONLY_MESSAGE}\n")
    );
}
