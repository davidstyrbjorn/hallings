use gloo_events::EventListener;
use wasm_bindgen::{JsCast, __rt::IntoJsResult};
use web_sys::{EventTarget, HtmlElement};
use std::cell::Cell;

use crate::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ButtonProps {
    pub label: String,
    pub onclick: Callback<yew::MouseEvent>,
}

enum MouseInsideAction {
    ChangeMouseInside,
}

struct MouseInsideState{
    mouse_inside: bool,   
}

impl Default for MouseInsideState {
    fn default() -> Self {
        Self {mouse_inside: false}
    }
}

impl Reducible for MouseInsideState{
    type Action = MouseInsideAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let next_inside = match action {
           MouseInsideAction::ChangeMouseInside => !self.mouse_inside,
        };
        Self {mouse_inside: next_inside}.into()
    }
}

#[function_component]
pub fn Button(props: &CommonProps<ButtonProps>) -> Html {
    let cb;
    let theme = use_context::<ThemeContext>();
    let styles = StyleUtil::create_button_style(props, &theme.unwrap());

    //replaste usetate with use_reducer
    let mouse_inside = use_state(|| false);

    

    let mouse_enter = | e: MouseEvent| {
        let mouse_inside = mouse_inside.clone();
        let a = 
        mouse_inside.set(true);
        console::log_1(&"mouse entered".into());
    };

    let mouse_leave = | e: MouseEvent| {
        let mouse_inside = mouse_inside.clone();

        mouse_inside.set(false);
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
