use crate::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct TextProps {
    pub label: String,
}

#[function_component]
pub fn Text(props: &CommonProps<TextProps>) -> Html {
    html! {
        if let Some(label) = props.custom.clone() {
            <p class={classes!("hallings-text", props.class.clone())}>
                {label.label}
            </p>
        }else {
            <p class={classes!("hallings-text", props.class.clone())}>
                {props.children.clone()}
            </p>
        }

    }
}
