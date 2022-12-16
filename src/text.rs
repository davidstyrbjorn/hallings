use crate::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct TextProps {
    pub label: String,
}

#[function_component]
pub fn Text(props: &CommonProps<TextProps>) -> Html {
    let theme = use_context::<ThemeContext>();

    match theme.clone() {
        Some(theme) => log::info!("Theme {}", theme.text_color),
        _ => (),
    }

    html! {
        if let Some(label) = props.custom.clone() {
            <p class={classes!("hallings-text", props.class.clone())}
                style={format!("color: {};", theme.unwrap().text_color)}
            >
                {label.label}
            </p>
        }else {
            <p class={classes!("hallings-text", props.class.clone())}
                style={format!("color: {};", theme.unwrap().text_color)}
            >
                {props.children.clone()}
            </p>
        }

    }
}
