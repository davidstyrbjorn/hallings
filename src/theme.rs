use std::{fmt, rc::Rc};

use yew::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub struct Theme {
    pub text_color: String,
    pub main: String,
    pub secondary: String,
    pub white: String,
    pub foreground: String,
}

impl fmt::Debug for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Theme")
            .field("text_color", &self.text_color)
            .field("main", &self.main)
            .field("secondary", &self.secondary)
            .field("white", &self.white)
            .field("foreground", &self.foreground)
            .finish()
    }
}

pub trait ToSVGHex {
    fn convert_to_svg(&self) -> Self;
}

impl ToSVGHex for String {
    fn convert_to_svg(&self) -> Self {
        return format!("#{}", &self[1..self.len()]);
    }
}

// Context needs to be reducable if we want to be able to change it
impl Reducible for Theme {
    type Action = String;

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
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
pub fn MaestroProvider(props: &ThemeProviderProps) -> Html {
    let theme = use_reducer(|| Theme {
        text_color: "#000".to_string(),
        main: "#E519AD".to_string(),
        secondary: "#b4609d".to_string(),
        white: "#F0EAD6".to_string(),
        foreground: "#808080".to_string(),
    });
    html! {
        <ContextProvider<ThemeContext> context={theme}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}
