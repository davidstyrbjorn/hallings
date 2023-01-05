use crate::prelude::*;
use std::fmt;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ButtonProps {
    pub onclick: Callback<yew::MouseEvent>,
}

impl fmt::Debug for ButtonProps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ButtonProps").finish()
    }
}

#[function_component(Button)]
pub fn button_component(props: &CommonPropsNoOption<ButtonProps>) -> Html {
    let theme = use_context::<ThemeContext>();
    // Retrieve correct color for hovered
    let hovered_color = theme.as_ref().unwrap().secondary.clone();
    let mut styles = StyleUtil::create_button_style(props, &theme.as_ref().unwrap());

    let mouse_inside = use_state(|| false);

    let mouse_enter = {
        let mouse_inside = mouse_inside.clone();
        Callback::from(move |_e: MouseEvent| {
            mouse_inside.set(true);
        })
    };

    let mouse_leave = {
        let mouse_inside = mouse_inside.clone();
        Callback::from(move |_e: MouseEvent| {
            mouse_inside.set(false);
        })
    };

    let on_click = {
        let onclick = props.custom.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            onclick.emit(e);
        })
    };

    if *mouse_inside {
        let temp = &*format!("background-color: {}", hovered_color);
        styles += temp;
    }

    // let props = props.clone();
    html! {
        <button
            onmouseenter={move |e: MouseEvent| mouse_enter.emit(e)}
            onmouseleave={move |e: MouseEvent| mouse_leave.emit(e)}
            onclick={move |e: yew::MouseEvent| on_click.emit(e)}
            style={styles}
            class={classes!(props.class.clone())}
        >
            {props.children.clone()}
        </button>
    }
}
