use crate::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct TextProps {
    pub label: String,
}

#[function_component]
pub fn Text(props: &CommonProps<TextProps>) -> Html {
    let theme = use_context::<ThemeContext>();

    let x = StyleUtil::create_text_style(props, &theme.unwrap());
    // info!("{}", x);
    log::info!("{}", x);

    html! {
        if let Some(label) = props.custom.clone() {
            <p class={classes!("hallings-text", props.class.clone())}
                style={x}
            >
                {label.label}
            </p>
        }else {
            <p class={classes!("hallings-text", props.class.clone())}
                style={x}
            >
                {props.children.clone()}
            </p>
        }
    }
}
