use wasm_bindgen::JsCast;
use web_sys::{console, EventTarget, HtmlInputElement};

use crate::prelude::*;

#[derive(Clone)]
pub enum StrengthLevel {
    LOW,
    MEDIUM,
    HIGH,
}

#[derive(PartialEq, Properties, Clone)]
pub struct PasswordStrengthInputProps {
    strengthCallback: fn(value: String) -> StrengthLevel,
}

pub fn strength_callback(value: &str) -> StrengthLevel {
    match value.len() {
        0..=5 => StrengthLevel::LOW,
        6..=12 => StrengthLevel::MEDIUM,
        _ => StrengthLevel::HIGH,
    }
}

fn strength_level_to_string(value: StrengthLevel) -> String {
    match value {
        StrengthLevel::LOW => "Low level".into(),
        StrengthLevel::MEDIUM => "Medium level".into(),
        StrengthLevel::HIGH => "High level".into(),
        _ => "None".into(),
    }
}

#[function_component]
pub fn PasswordStrengthInput(props: &CommonProps<PasswordStrengthInputProps>) -> Html {
    let text = use_state(|| String::default());
    let text_value = (*text).clone();

    let strength = use_state(|| strength_callback(&text_value));

    {
        use_effect_with_deps(
            move |_| {
                // ...
                strength.set(strength_callback(&text_value));
                || ()
            },
            (),
        );
    }

    let on_change = {
        Callback::from(move |e: InputEvent| {
            // Grab target event as EventTarget
            let target: Option<EventTarget> = e.target();

            // JScast into HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            // Grab if value exists for the event
            if let Some(input) = input {
                text.set(input.value());
            }
        })
    };

    // let str_level = strength_callback(&text_value.clone());

    html! {
        <div>
            // <input type="password" value={text_value} oninput={on_change} />
            // <Text>{strength_level_to_string((*strength).clone())}</Text>
        </div>
    }
}
