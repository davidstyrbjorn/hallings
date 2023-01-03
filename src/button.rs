use gloo_events::EventListener;
use std::borrow::BorrowMut;
use std::borrow::{Borrow, Cow};
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, __rt::IntoJsResult};
use web_sys::{EventTarget, HtmlElement};

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
    let mut styles = StyleUtil::create_button_style(props, &theme.unwrap());

    //replaste usetate with use_reducer
    let mouse_inside = use_state(|| false);

    let mouse_enter = {
        let mouse_inside = mouse_inside.clone();
        Callback::from(move |e: MouseEvent| {
            mouse_inside.set(true);
        })
    };

    let mouse_leave = {
        let mouse_inside = mouse_inside.clone();
        Callback::from(move |e: MouseEvent| {
            mouse_inside.set(false);
        })
    };

    // let mouse_leave = move | e: MouseEvent| {
    if *mouse_inside {
        styles += "background-color: red;"
    } else {
        console::log_1(&"mouse left".into());
    }

    // Build style

    if let Some(c_props) = props.custom.clone() {
        cb = c_props.onclick;
        html! {
            if let Some(label) = props.custom.clone() {
                <button onmouseenter={move |e: MouseEvent| mouse_enter.emit(e)} onmouseleave={move |e: MouseEvent| mouse_leave.emit(e)} onclick={move |e: yew::MouseEvent| cb.emit(e)}
                 style={styles}
                >
                    <Text color={"white"}>{label.label}</Text>
                </button>
            }
        }
    } else {
        html! {
            <button>{"Error"}</button>
        }
    }
}
