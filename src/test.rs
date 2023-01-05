#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use crate::prelude::*;
use std::time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::{html::onchange::Event, platform::time::sleep};

// Find color: XXX; and return XXX
// should work good with color: XXX; existing in any place inside a string
fn get_style_color(style: &str) -> Option<String> {
    let idx = style.find("color:")?;
    let style = &style[idx..style.len()];
    let idx2 = style.find(";")?;
    let style = String::from(&style[(idx + 6)..idx2]);
    Some(String::from(style.trim()))
}

#[wasm_bindgen_test]
async fn text_inner_html_test() {
    #[function_component]
    fn App() -> Html {
        html! {
            <MaestroProvider>
                <Text class={classes!("text_class")}>{"test text"}</Text>
            </MaestroProvider>
        }
    }

    yew::Renderer::<App>::with_root(gloo_utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(100)).await;

    let result = obtain_text_value();

    assert_eq!(result.to_string(), "test text");

    pub fn obtain_text_value() -> String {
        gloo_utils::document()
            .get_elements_by_class_name("text_class")
            .get_with_index(0)
            .expect("Cant't find Text component")
            .inner_html()
    }
}

#[wasm_bindgen_test]
async fn button_onclick_test() {
    #[function_component]
    fn App() -> Html {
        let text = use_state(|| "not clicked");

        let click = {
            let text = text.clone();
            Callback::from(move |_s: yew::MouseEvent| {
                text.set("clicked");
            })
        };

        html! {
            <MaestroProvider>
                <Button
                    class={classes!("button_class")}
                    custom={
                        ButtonProps{ onclick: click}
                    }
                >
                    <Text
                        class={classes!("text_class")}
                        color={"white"}
                    >
                        {*text}
                    </Text>
                </Button>
            </MaestroProvider>
        }
    }

    yew::Renderer::<App>::with_root(gloo_utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(100)).await;

    // Pre click
    let text_value = obtain_text_value();
    assert_eq!(&text_value, "not clicked");

    // Simulate click
    let simulate_click = move || {
        let button_collection = gloo_utils::document().get_elements_by_class_name("button_class");
        let binding = button_collection
            .get_with_index(0)
            .expect("Cant't find Text component");
        let button = binding.dyn_ref::<HtmlElement>();
        if let Some(button_element) = button {
            button_element.click();
        }
    };
    simulate_click();

    sleep(Duration::from_millis(100)).await;

    // Post click
    let text_value = obtain_text_value();
    assert_eq!(&text_value, "clicked");

    pub fn obtain_text_value() -> String {
        gloo_utils::document()
            .get_elements_by_class_name("text_class")
            .get_with_index(0)
            .expect("Cant't find Text component")
            .inner_html()
    }
}

#[wasm_bindgen_test]
async fn password_strength_level_test() {
    #[function_component]
    fn App() -> Html {
        html! {
            <MaestroProvider>
                <PasswordStrengthInput custom = { PasswordStrengthInputProps {
                    calculate_strength_level: None,
                    strength_level_to_text_and_color: None,
                    strength_callback: None
                }} />
            </MaestroProvider>
        }
    }

    yew::Renderer::<App>::with_root(gloo_utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(500)).await;

    let obtain_color = move || -> Option<String> {
        let element = gloo_utils::document()
            .get_elements_by_class_name("text_class")
            .get_with_index(0)
            .expect("Cant't find Text component");

        let html_element = element.dyn_into::<HtmlElement>();

        if let Some(html_element) = html_element.ok() {
            let style_string = html_element.style().css_text();
            return get_style_color(&style_string);
        }

        None
    };

    let update_input_field = move |with: &str| {
        let element = gloo_utils::document()
            .get_elements_by_class_name("hallings-input")
            .get_with_index(0)
            .expect("Cant't find Text component");

        let html_input_element = element
            .dyn_into::<HtmlInputElement>()
            .expect("Couldn't convert to HtmlInputElement in update_input_field");

        html_input_element.set_value(&with);

        let event =
            Event::new("input").expect("Failed to create input event in update_input_field");
        html_input_element
            .dispatch_event(&event)
            .expect("Failed to dispatch input event in update_input_field");
    };

    let color = obtain_color().expect("No color entry found in text!");
    assert_eq!(&color, "red");

    // Input text into the password strength input field and update the component

    update_input_field("hhfhhh11");
    sleep(Duration::from_millis(1000)).await;

    let color = obtain_color().expect("No color entry found in text!");
    assert_eq!(&color, "yellow");

    update_input_field("hhfhhfdafasdfasdfdafh11");
    sleep(Duration::from_millis(1000)).await;

    let color = obtain_color().expect("No color entry found in text!");
    assert_eq!(&color, "green");
}
