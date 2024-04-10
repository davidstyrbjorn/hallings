use crate::prelude::*;
use yew::prelude::*;
use std::time::Duration;
use async_std::task;

#[derive(PartialEq, Properties, Clone)]
pub struct JumpScareProps {}

#[function_component]
pub fn JumpScare(props: &CommonProps<JumpScareProps>) -> Html {
    let theme = use_context::<ThemeContext>();
    let show = use_state(|| false);
    let mut styles = StyleUtil::create_jumpscare_style(props, theme.as_ref().unwrap(), false);

    let mut onclick = {
        let show = show.clone();
        Callback::from(move |e: yew::MouseEvent| {
            show.set(!(*show));
            if *show {
                task::spawn(async move {
                    task::sleep(Duration::from_secs(1)).await; // wait some
                    show.set(false);
                });
            }
        })
    };

    html! {
        <>
        <Button custom={
            ButtonProps{ onclick: onclick }
        }>
            <Text color={"white"}>{"Go"}</Text>
        </Button>
        if *show {
            <img
                style={styles}
                src="freddy.png"
                class={classes!("jumpscare-show")}
            />
        } else {
            <img
                style={styles}
                src="freddy.png"
                class={classes!("jumpscare-hide")}
            />
        }
        </>
    }
}
