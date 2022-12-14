mod props;
mod text;
mod theme;

mod prelude {
    pub use crate::props::*;
    pub use crate::text::*;
    pub use crate::theme::*;
    pub use stylist::yew::styled_component;
    pub use stylist::yew::use_style;
    pub use stylist::Style;
    pub use yew::prelude::*;
    pub use stylist::style;
    pub use yew::Properties;
    pub use stylist::css;
}

// Provider for theming
// Button
//
//
// Password strength meter
// Steps left

use prelude::*;
use stylist::StyleSource;
use stylist::ast::Sheet;


fn create_theme() -> Theme {
    Theme {
        text_color: "#0000ff".into(),
    }
}

#[function_component]
fn App() -> Html {

    let s: String = format!("color: {};", "red");
    let new = s.as_str();
    let mut st_source: StyleSource; 
    st_source = StyleSource::from_str(new);
    let styles = use_style(st_source);
    
    html! {
        <ThemeProvider theme={create_theme()}>
            <p class={styles}>{"hej"}</p>
        </ThemeProvider>
    }
}


fn main() {
    
    yew::Renderer::<App>::new().render();
}
