use std::rc::Rc;

// pub use crate::prelude::*;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Theme {
    pub text_color: String,
    pub foreground: String,
    pub background: String,
}

// Context needs to be reducable if we want to be able to change it
impl Reducible for Theme {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        self
    }
}

pub type ThemeContext = UseReducerHandle<Theme>;

#[derive(Properties, Debug, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme = use_reducer(|| Theme {
        text_color: "black".to_string(),
        foreground: "#319795".to_string(),
        background: "#2c7a7b".to_string(),
    });
    html! {
        <ContextProvider<ThemeContext> context={theme}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}
