use crate::context::Context;
use crate::service::buildings::BuildingsAndQueuePair;
use crate::view::layout;
use maud::html;
use uuid::Uuid;

pub fn buildings(
    context: &Context,
    body_id: Uuid,
    buildings_and_queue: &BuildingsAndQueuePair,
) -> String {
    let markup = html! {
        table {
            tr {
                th colspan="3" { "Buildings" }
            }
            @for b in buildings_and_queue.buildings.iter() {
                tr class="item" {
                    td class="item-image" {
                        a href={ "/details/building?body=" (body_id) "&kind=" (b.kind) } {
                            img src={ "/static/skins/EpicBlue/items/" (b.kind.image_id()) ".gif" };
                        }
                    }
                    td class="item-info" {
                        h2 {
                            a href={ "/details/building?body=" (body_id) "&kind=" (b.kind) } {
                                (b.kind)
                            }
                            @if b.future_level > 0 {
                                " ("
                                (b.current_level)
                                @if b.future_level != b.current_level {
                                    " → "
                                    (b.future_level)
                                }
                                ")"
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
                            strong { (b.cost.metal) }
                            " Crystal: "
                            strong { (b.cost.crystal) }
                            " Deuterium: "
                            strong { (b.cost.deuterium) }
                        }
                        p {
                            "Construction time: "
                            strong { "01:02:03" }
                        }
                    }
                    td class="item-actions" {
                        form action="/buildings/build" method="post" {
                            input name="body" type="hidden" value={ (body_id) };
                            input name="kind" type="hidden" value={ (b.kind) };
                            button { "Build" }
                        }
                    }
                }
            }
        }
    };
    layout(context, body_id, markup).into()
}
