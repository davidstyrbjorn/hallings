use crate::prelude::*;

use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct TextProps {
    pub label: String,
}

#[function_component]
pub fn Text(props: &CommonProps<TextProps>) -> Html {
    let theme = use_context::<ThemeContext>();
    let x = StyleUtil::create_text_style(props, &theme.unwrap());

    html! {
        if let Some(custom) = props.custom.clone() {
            <p class={classes!(props.class.clone())} style={x}>
                {custom.label}
            </p>
        }else {
            <p class={classes!(props.class.clone())} style={x}>
                {props.children.clone()}
            </p>
        }
    }
}
