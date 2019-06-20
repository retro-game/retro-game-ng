use maud::{html, Markup, DOCTYPE};

pub fn layout(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "Retro Game" }
            link href="/static/skins/EpicBlue/style.css" rel="stylesheet";
        }
        body {
            div.container {
                div {
                    div {
                        (content)
                    }
                }
            }
        }
    }
}
