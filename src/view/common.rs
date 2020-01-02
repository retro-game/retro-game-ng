use crate::context::Context;
use crate::model::Body;
use maud::{html, Markup, DOCTYPE};
use uuid::Uuid;

fn top_bar(context: &Context, body: &Body) -> Markup {
    let prod = body.get_production();
    html! {
        div id="top-bar" {
            div id="top-bar-bodies" {
                img src="/static/skins/EpicBlue/bodies/JUNGLE_1.jpg";
                div {
                    form id="top-bar-body-list" {
                        select name="body" {
                            @for (id, b) in context.bodies().iter() {
                                option value=(id) selected?[id == &body.id] {
                                    (b.name)
                                    " ["
                                    (b.coordinates)
                                    "]"
                                }
                            }
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
                @let total = &prod.total_prod;
                @let make_res_tooltip = |prod| format!("<p>Production: <strong>{}</strong></p>", prod);
                div data-tooltip? data-tooltip-html? data-tooltip-title={ (make_res_tooltip(total.metal)) } {
                    img src="/static/skins/EpicBlue/resources/METAL.gif";
                    p { "Metal" }
                    p { (body.metal) }
                }
                div data-tooltip? data-tooltip-html? data-tooltip-title={ (make_res_tooltip(total.crystal)) } {
                    img src="/static/skins/EpicBlue/resources/CRYSTAL.gif";
                    p { "Crystal" }
                    p { (body.crystal) }
                }
                div data-tooltip? data-tooltip-html? data-tooltip-title={ (make_res_tooltip(total.deuterium)) } {
                    img src="/static/skins/EpicBlue/resources/DEUTERIUM.gif";
                    p { "Deuterium" }
                    p { (body.deuterium) }
                }
                @let balance = &prod.energy_balance;
                div data-tooltip? data-tooltip-html? data-tooltip-title={ "<p>Required energy: <strong>" (balance.used) "</strong></p>" } {
                    img src="/static/skins/EpicBlue/resources/ENERGY.gif";
                    p { "Energy" }
                    p { (balance.available) " / " (balance.total()) }
                }
            }
        }
    }
}

fn sidebar(body: &Body) -> Markup {
    let body_id = body.id;
    let c = &body.coordinates;
    html! {
        nav id="sidebar" {
            h2 {
                a href="https://github.com/retro-game/retro-game-ng" { "Retro Game NG" }
                " "
                a href={"/changelog?body=" (body_id) } { "v0.1" }
            }
            ul {
                li {
                    a href={ "/overview?body=" (body_id) } { "Overview" }
                }
                li {
                    a href={ "/flights?body=" (body_id) } { "Flights" }
                }
                li {
                    a href={ "/flights/send?body=" (body_id) } { "Send fleet" }
                }
                li {
                    a href={ "/resources?body=" (body_id) } { "Resources" }
                }
                li {
                    a href={ "/buildings?body=" (body_id) } { "Buildings" }
                }
                li {
                    a href={ "/technologies?body=" (body_id) } { "Technologies" }
                }
                li {
                    a href={ "/shipyard?body=" (body_id) "&type=FLEET" } { "Shipyard" }
                }
                li {
                    a href={ "/shipyard?body=" (body_id) "&type=DEFENSE" } { "Defense" }
                }
                li {
                    a href={ "/galaxy?body=" (body_id) "&galaxy=" (c.galaxy) "&system=" (c.system) } { "Galaxy" }
                }
            }
        }
    }
}

pub fn layout(context: &Context, body_id: Uuid, content: Markup) -> Markup {
    let bodies = context.bodies();
    let body = bodies.get(&body_id).unwrap();
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "Retro Game" }
            link href="/static/skins/EpicBlue/style.css" rel="stylesheet";
            script src="/static/js/jquery-3.3.1.min.js" {}
            script src="/static/js/popper-1.14.4.min.js" {}
            script src="/static/js/tooltip-1.3.0.min.js" {}
            script src="/static/js/retrogame.js" {}
        }
        body {
            div.container {
                div {
                    div {
                        (top_bar(context, body))
                        div { (content) }
                    }
                }
                (sidebar(body))
            }
        }
    }
}
