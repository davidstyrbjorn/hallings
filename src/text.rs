use crate::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct TextProps {
    pub label: String,
}

#[function_component]
pub fn Text(props: &CommonProps<TextProps>) -> Html {
    let theme = use_context::<ThemeContext>();

    let mut t = "None";
    if theme.is_some() {
        // println!("{}", theme.unwrap().text_color);
        // log::info!("{}", theme.unwrap().text_color);
        t = "Some";
    } else {
        log::info!("Y");
        println!("XXXXXXXXXXXXXXXX");
    }

    html! {
        if let Some(label) = props.custom.clone() {
            <p class={classes!("hallings-text", props.class.clone())}
                style={format!("color: {};", theme.unwrap().text_color)}
            >
                {label.label}
            </p>
        }else {
            <p class={classes!("hallings-text", props.class.clone())}>
                {props.children.clone()}
            </p>
        }

    }
}
