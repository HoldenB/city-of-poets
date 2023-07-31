#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            TitleCardComp {
                title: "Personal Site & Blog"
            },
            TitleCardInlineComp{ title: "Another Title Inline".to_string()}
        }
        div {
            AboutComp {},
            LikesComp { score: 10,}
        }
    })
}

pub fn AboutComp(cx: Scope) -> Element {
    cx.render(rsx!(p {
            b {"Holden's Personal Site & Blog"}
            " A personal site and blog space to share my thoughts and progress."
        }
    ))
}

// Remember: Owned props must implement `PartialEq`!
#[derive(PartialEq, Props)]
struct LikesProps {
    score: i32,
}

fn LikesComp(cx: Scope<LikesProps>) -> Element {
    cx.render(rsx! {
        div {
            "This site has ",
            b { "{cx.props.score}" },
            " likes!"
        }
    })
}

#[derive(Props)]
struct TitleCardProps<'a> {
    // Borrowed str
    title: &'a str,
    #[props(default = "Default TODO!")]
    // subtitle: Option<&'a str>,
    subtitle: &'a str,
}

fn TitleCardComp<'a>(cx: Scope<'a, TitleCardProps<'a>>) -> Element {
    cx.render(rsx! {
        h1 { "{cx.props.title}" },
        cx.props.subtitle
    })
}

#[inline_props]
fn TitleCardInlineComp(cx: Scope, title: String) -> Element {
    cx.render(rsx! {
        h2 { "{title}" }
    })
}
