use gloo_events::EventListener;
use wasm_bindgen::{JsCast, __rt::IntoJsResult};
use web_sys::{EventTarget, HtmlElement};
use std::cell::Cell;
use std::borrow::{Cow, Borrow};
use std::rc::Rc;
use std::borrow::BorrowMut;

use crate::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ButtonProps {
    pub label: String,
    pub onclick: Callback<yew::MouseEvent>,
}

#[function_component]
pub fn Button(props: &CommonProps<ButtonProps>) -> Html {
    let cb;
    let theme = use_context::<ThemeContext>();
    let styles = StyleUtil::create_button_style(props, &theme.unwrap());

    //replaste usetate with use_reducer
    let mouse_inside = use_state(|| Rc::new(false));

    let mouse_enter = move | e: MouseEvent| {
        mouse_inside.set(Rc::new(true));
        console::log_1(&"mouse entered".into());
    };

    let mouse_leave = move | e: MouseEvent| {
        
        console::log_1(&"mouse left".into());
    };

    if let Some(c_props) = props.custom.clone() {
        cb = c_props.onclick;
        html! {
            if let Some(label) = props.custom.clone() {
                <button onmouseenter={mouse_enter} onmouseleave={mouse_leave} onclick={move |e: yew::MouseEvent| cb.emit(e)}
                 style={styles}
                >
                    <Text color={"white"}>{label.label}</Text>
                </button>
            }
        }
    }
    else {
        html!{
            <button>{"Error"}</button>
        }
    }
}
