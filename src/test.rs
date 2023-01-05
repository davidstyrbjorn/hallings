#[cfg(target_arch = "wasm32")]

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use std::time::Duration;

use crate::prelude::*;
use gloo_utils::*;
use nom::bytes::complete::{tag, take_until};
use nom::sequence::preceded;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_test::*;
use web_sys::{Element, HtmlElement};
use yew::platform::time::sleep;
use yew::prelude::*;

// Find color: XXX; and return XXX
// should work good with color: XXX; existing in any place inside a string
fn get_style_color(style: String) -> String {
    let Ok(x) = preceded(tag("color:"), tag(";")){
        x
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
            console::log_1(&button_element);
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

    let obtain_text_and_color = move || -> (String, String) {
        let element = gloo_utils::document()
            .get_elements_by_class_name("text_class")
            .get_with_index(0)
            .expect("Cant't find Text component");

        let html_element = element.dyn_into::<HtmlElement>();
        let mut color_string: String = String::from("");
        if html_element.is_ok() {
            if let Some(html_element) = html_element.ok() {
                let style_string = html_element.style().css_text();
                if style_string.contains("color") {
                    let result = style_string.split("color:").collect::<Vec<&str>>();
                    color_string = result.concat();
                }
            }
        }

        (String::from(color_string), String::from("Hej"))
    };

    let result = obtain_text_and_color();
    console::log_1(&result.0.into());
    assert_eq!(1, 1);
}

// #[wasm_bindgen_test]
// async fn steps_left_circle_test() {

//     #[function_component]
//     fn App() -> Html {
//         html! {
//             <p id="result">{"hello"}</p>
//         }
//     }

//     yew::Renderer::<App>::with_root(gloo_utils::document().get_element_by_id("output").unwrap())
//         .render();

//     sleep(Duration::from_millis(100)).await;

//     let result = obtain_result();
//     assert_eq!(result.as_str(), "hello");

//     pub fn obtain_result() -> String {
//         gloo_utils::document()
//             .get_element_by_id("result")
//             .expect("No result found. Most likely, the application crashed and burned")
//             .inner_html()
//     }
// }

// //test theme context
// #[wasm_bindgen_test]
// async fn maestro_test() {

//     #[function_component]
//     fn App() -> Html {
//         html! {
//             <p id="result">{"hello"}</p>
//         }
//     }

//     yew::Renderer::<App>::with_root(gloo_utils::document().get_element_by_id("output").unwrap())
//         .render();

//     sleep(Duration::from_millis(100)).await;

//     let result = obtain_result();
//     assert_eq!(result.as_str(), "hello");

//     pub fn obtain_result() -> String {
//         gloo_utils::document()
//             .get_element_by_id("result")
//             .expect("No result found. Most likely, the application crashed and burned")
//             .inner_html()
//     }
// }
