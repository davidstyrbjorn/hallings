pub use crate::prelude::*;

use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub text_color: String,
}

#[derive(Properties, Debug, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme = use_memo(
        |_| Theme {
            text_color: "#ffffff".into(),
        },
        (),
    );
    html! {
        <ContextProvider<Rc<Theme>> context={theme}>
            {props.children.clone()}
        </ContextProvider<Rc<Theme>>>
    }
}
