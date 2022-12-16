mod props;
mod style_util;
mod text;
mod theme;

mod prelude {
    pub use crate::props::*;
    pub use crate::style_util::*;
    pub use crate::text::*;
    pub use crate::theme::*;
    pub use stylist::css;
    pub use stylist::style;
    pub use stylist::yew::styled_component;
    pub use stylist::yew::use_style;
    pub use stylist::Style;
    pub use yew::prelude::*;
    pub use yew::Properties;
}

use prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <ThemeProvider >
            <Text>{"hej"}</Text>
        </ThemeProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
