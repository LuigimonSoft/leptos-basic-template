use leptos::prelude::RenderHtml;
use leptos_basic_template::{APP_HEADING, App, BROWSER_ONLY_MESSAGE};

#[test]
fn given_public_application_contract_when_rendering_and_reading_runtime_copy_then_integration_should_validate_complete_basic_experience()
 {
    let rendered_html = App().to_html();

    assert_eq!(APP_HEADING, "Hello, Leptos!");
    assert_eq!(
        BROWSER_ONLY_MESSAGE,
        "This example only works when compiled to WebAssembly."
    );
    assert_eq!(rendered_html, format!("<div><h1>{APP_HEADING}</h1></div>"));
    assert!(rendered_html.contains(APP_HEADING));
}
