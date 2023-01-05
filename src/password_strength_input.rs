use crate::prelude::*;
use std::fmt;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, Copy)]
pub enum StrengthLevel {
    LOW,
    MEDIUM,
    HIGH,
}

#[derive(PartialEq, Properties, Clone, Copy)]
pub struct PasswordStrengthInputProps {
    pub calculate_strength_level: Option<fn(value: String) -> StrengthLevel>,
    pub strength_level_to_text_and_color: Option<fn(value: StrengthLevel) -> (String, String)>,
    pub strength_callback: Option<fn(strength: StrengthLevel)>,
}

impl fmt::Debug for PasswordStrengthInputProps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PasswordStrengthInputProps").finish()
    }
}

// Default behaviour expects characters > 6 & for the str to contain a lowercase character
pub fn calculate_strength_level(value: &str) -> StrengthLevel {
    let does_not_contain_lowercase = value.chars().all(|c| c.is_lowercase());
    if does_not_contain_lowercase {
        return StrengthLevel::LOW;
    }
    match value.len() {
        0..=5 => StrengthLevel::LOW,
        6..=12 => StrengthLevel::MEDIUM,
        _ => StrengthLevel::HIGH,
    }
}

fn strength_level_to_text_and_color(value: &StrengthLevel) -> (String, String) {
    match value {
        StrengthLevel::LOW => ("Low level".into(), "red".into()),
        StrengthLevel::MEDIUM => ("Medium level".into(), "yellow".into()),
        StrengthLevel::HIGH => ("High level".into(), "green".into()),
    }
}

#[function_component]
pub fn PasswordStrengthInput(props: &CommonProps<PasswordStrengthInputProps>) -> Html {
    let text = use_state(|| String::default());
    let text_value = (*text).clone();
    let input_width = use_state(|| 0);
    let div_ref = use_node_ref();

    let input_width_value = (*input_width).clone();
    {
        let div_ref = div_ref.clone();

        use_effect_with_deps(
            move |div_ref| {
                let div = div_ref
                    .cast::<HtmlElement>()
                    .expect("div_ref not attached to div element");
                input_width.set(div.client_width());

                move || {}
            },
            div_ref,
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

    let mut str_level = calculate_strength_level(&text_value.clone());
    let mut text_and_color = strength_level_to_text_and_color(&str_level);

    if let Some(custom) = props.custom {
        if let Some(calculate_strength_level) = custom.calculate_strength_level {
            str_level = calculate_strength_level(text_value.clone());
        }
        if let Some(strength_level_to_text_and_color) = custom.strength_level_to_text_and_color {
            text_and_color = strength_level_to_text_and_color(str_level);
        }
    }

    // Call callback
    if let Some(custom) = props.custom {
        if let Some(strength_callback) = custom.strength_callback {
            strength_callback(str_level);
        }
    }

    html! {
        <div>
            <input class={classes!("hallings-input")} placeholder="Password" style={format!("width: {}px; min-width: 150px;", input_width_value.to_string())} type="password" value={text_value} oninput={on_change} />
            <div
                style="display: flex; align-items: center; gap: 5px; width: fit-content;"
                ref={div_ref}
            >
                <div style={format!("width: 60px; height: 15px; border-radius: 5px; background-color: {}", text_and_color.1)}/>
                <Text class={classes!("text_class")} color={text_and_color.1}>{text_and_color.0}</Text>
            </div>
        </div>
    }
}
