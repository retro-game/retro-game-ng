use maud::{html, Markup, DOCTYPE};

fn top_bar() -> Markup {
    html! {
        div id="top-bar" {
            div id="top-bar-bodies" {
                img src="/static/skins/EpicBlue/bodies/JUNGLE_1.jpg";
                div {
                    form id="top-bar-body-list" {
                        select name="body" {
                            option { "Test [1:2:3:P]" }
                        }
                    }
                    div id="top-bar-body-pointers" {
                        form {
                            input name="body" type="hidden" value="TODO";
                            button title="Test [1:2:3:P]" { "←" }
                        }
                        form {
                            input name="body" type="hidden" value="TODO";
                            button title="Test [1:2:3:P]" { "→" }
                        }
                    }
                }
            }
            div id="top-bar-resources" {
                div {
                    img src="/static/skins/EpicBlue/resources/METAL.gif";
                    p { "Metal" }
                    p { "123" }
                }
                div {
                    img src="/static/skins/EpicBlue/resources/CRYSTAL.gif";
                    p { "Crystal" }
                    p { "123" }
                }
                div {
                    img src="/static/skins/EpicBlue/resources/DEUTERIUM.gif";
                    p { "Deuterium" }
                    p { "123" }
                }
                div {
                    img src="/static/skins/EpicBlue/resources/ENERGY.gif";
                    p { "Energy" }
                    p { "123 / 123" }
                }
            }
        }
    }
}

fn sidebar() -> Markup {
    html! {
        nav id="sidebar" {
            h2 {
                a href="https://github.com/retro-game/retro-game-ng" { "Retro Game NG" }
                " "
                a href="/changelog?body=TODO" { "v0.1" }
            }
            ul {
                li {
                    a href="/overview?body=TODO" { "Overview" }
                }
                li {
                    a href="/flights?body=TODO" { "Flights" }
                }
                li {
                    a href="/flights/send?body=TODO" { "Send fleet" }
                }
                li {
                    a href="/resources?body=TODO" { "Resources" }
                }
                li {
                    a href="/buildings?body=TODO" { "Buildings" }
                }
                li {
                    a href="/technologies?body=TODO" { "Technologies" }
                }
                li {
                    a href="/shipyard?body=TODO&type=FLEET" { "Shipyard" }
                }
                li {
                    a href="/shipyard?body=TODO&type=DEFENSE" { "Defense" }
                }
                li {
                    a href="/galaxy?body=TODO" { "Galaxy" }
                }
            }
        }
    }
}

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
                        (top_bar())
                        div { (content) }
                    }
                }
                (sidebar())
            }
        }
    }
}
