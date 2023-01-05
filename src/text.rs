use crate::prelude::*;
use std::fmt;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct TextProps {
    pub label: String,
}

impl fmt::Debug for TextProps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextProps")
            .field("label", &self.label)
            .finish()
    }
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
