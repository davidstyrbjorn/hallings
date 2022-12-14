pub use crate::prelude::*;

use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Theme {
    pub text_color: String,
}

fn default_theme() -> Theme {
    Theme {
        text_color: "#ffffff".into(),
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default(default_theme)]
    pub theme: Theme,
}

#[function_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme = use_memo(|_| props.theme.clone(), ());
    html! {
        <ContextProvider<Rc<Theme>> context={theme}>
            {props.children.clone()}
        </ContextProvider<Rc<Theme>>>
    }
}
