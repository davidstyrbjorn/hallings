mod button;
mod password_strength_input;
mod props;
mod steps_left;
mod style_util;
mod text;
mod theme;

pub mod prelude {
    pub use crate::button::*;
    pub use crate::button::*;
    pub use crate::password_strength_input::*;
    pub use crate::props::*;
    pub use crate::steps_left::*;
    pub use crate::style_util::*;
    pub use crate::text::*;
    pub use crate::theme::*;
    pub use gloo_events::*;
    pub use gloo_events::*;
    pub use web_sys::console;
    pub use yew::prelude::*;
    pub use yew::Properties;
    pub use yew::ServerRenderer;
}

use prelude::*;

pub fn calculate_strength_level(value: String) -> StrengthLevel {
    if value.contains("secure") {
        return StrengthLevel::HIGH;
    }
    StrengthLevel::LOW
}

pub fn on_level_change(strength_level: StrengthLevel) {
    // match strength_level {
    //     StrengthLevel::LOW => console::log_1(&"LOW".into()),
    //     StrengthLevel::MEDIUM => console::log_1(&"MEDIUM".into()),
    //     StrengthLevel::HIGH => console::log_1(&"HIGH".into()),
    // }
}

fn strength_level_to_text_and_color(value: StrengthLevel) -> (String, String) {
    match value {
        StrengthLevel::LOW => ("Password not strong enough".into(), "red".into()),
        StrengthLevel::MEDIUM => ("Password weak but passable".into(), "blue".into()),
        StrengthLevel::HIGH => ("Password strong".into(), "green".into()),
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);

    let click = {
        let counter = counter.clone();
        Callback::from(move |_s: yew::MouseEvent| {
            counter.set(*counter + 1);
        })
    };

    html! {
        <MaestroProvider>
            <Text size={"40px"}>{"HALLINGS PLAYGROUND"}</Text>
            <Text size={"40px"} custom={TextProps {
                label: "Test".into()
            }}/>
            // <Text color="red" size="64px">{"Hallingos"}</Text>
            <PasswordStrengthInput custom={PasswordStrengthInputProps {
                calculate_strength_level: Some(calculate_strength_level),
                strength_level_to_text_and_color: Some(strength_level_to_text_and_color),
                strength_callback: Some(on_level_change)
            }} />
            <PasswordStrengthInput custom={PasswordStrengthInputProps {
                calculate_strength_level: None,
                strength_level_to_text_and_color: None,
                strength_callback: None
            }} />
            <Button custom={
                ButtonProps{ onclick: click}}
            >
                <Text color={"white"}>{"Next"}</Text>
            </Button>
            <StepsLeft
                custom= {
                    StepsLeftProps {
                        width: 800,
                        height: 200,
                        current_step: (*counter).clone(),
                        steps: vec![
                            Step {
                                label: "Check out".into()
                            },
                            Step {
                                label: "Confirm".into()
                            },
                            Step {
                                label: "Pay".into()
                            },
                        ]
                    }
                }
            />
        </MaestroProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
pub mod tests {
    use crate::{button, prelude::*};

    #[function_component]
    fn App() -> Html {
        html! {
            <p>{"hej"}</p>
        }
    }

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
//     use super::*;
//     use wasm_bindgen::JsCast;
//     use wasm_bindgen_test::wasm_bindgen_test;
//     use web_sys::HtmlElement;

//     // This macro should be only called once per binary in crate
//     wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

//     #[wasm_bindgen_test]
//     fn clicking_on_button_should_increment_value() {
//         let document = gloo_utils::document();
//         let body = document.body().unwrap();
//         let div = document.create_element("div").unwrap();
//         body.append_child(&div).unwrap();
//         yew::start_app_in_element::<Model>(div);

//         let value = body.query_selector(".panel > p").unwrap().unwrap();
//         let button = body.query_selector(".panel > button").unwrap().unwrap();
//         let button = button.dyn_into::<HtmlElement>().unwrap();

//         assert_eq!("0", value.text_content().unwrap());
//         button.click();
//         assert_eq!("1", value.text_content().unwrap());
//     }
// }
