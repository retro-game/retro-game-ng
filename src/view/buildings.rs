use crate::context::Context;
use crate::view::layout;
use maud::html;
use uuid::Uuid;

pub fn buildings(context: &Context, body_id: Uuid) -> String {
    let bodies = context.bodies();
    let body = bodies.get(&body_id).unwrap();

    let markup = html! {
        table {
            tr {
                th colspan="3" { "Buildings" }
            }
            @for (kind, level) in body.buildings.iter() {
                tr class="item" {
                    td class="item-image" {
                        a href={ "/details/building?body=" (body_id) "&kind=" (kind) } {
                            img src={ "/static/skins/EpicBlue/items/" (kind) ".gif" };
                        }
                    }
                    td class="item-info" {
                        h2 {
                            a href={ "/details/building?body=" (body_id) "&kind=" (kind) } {
                                (kind)
                            }
                            @if *level > 0 {
                                " (" (level) ")"
                            }
                        }
                        p {
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                        }
                        @if false {
                            p {
                                a href={ "/jump-gate?body=" (body_id) } {
                                    "Go to Jump Gate"
                                }
                            }
                        }
                        p {
                            "Metal: "
                            strong { "200" }
                            " Crystal: "
                            strong { "50" }
                            " Deuterium: "
                            strong { "0" }
                        }
                        p {
                            "Construction time: "
                            strong { "01:02:03" }
                        }
                    }
                    td class="item-actions" {
                        form action="/buildings/construct" method="post" {
                            input name="body" type="hidden" value={ (body_id) };
                            input name="kind" type="hidden" value={ (kind) };
                            button { "Construct" }
                        }
                    }
                }
            }
        }
    };
    layout(markup).into()
}
