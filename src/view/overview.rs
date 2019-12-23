use crate::context::Context;
use crate::view::layout;
use maud::html;
use uuid::Uuid;

pub fn overview(context: &Context, body_id: Uuid) -> String {
    let bodies = context.bodies();
    let body = bodies.get(&body_id).unwrap();

    let markup = html! {
        table {
            tr {
                th colspan="4" {
                    a href={ "/body-settings?body=" (body_id) } title="Body settings" {
                        (body.name)
                    }
                    " (User name)"
                }
            }
            tr {
                td { "Messages" }
                td colspan="3" {
                    a href={ "/messages?body=" (body_id) } {
                        "You have 42 new messages!"
                    }
                }
            }
            tr {
                td { "Reports" }
                td colspan="3" {
                    a href={ "/reports?body=" (body_id) } {
                        "You have 42 new reports!"
                    }
                }
            }
            tr {
                td { "Time" }
                td colspan="3" { "2006-06-06 03:00:00" }
            }
            tr {
                th colspan="4" { "Flight events" }
            }
            tr {
                td {}
                td class="overview-body" colspan="2" {
                    img src={ "/static/skins/EpicBlue/bodies/" (body.type_) "_" (body.image) ".jpg" } width="200" height="200";
                    span { "free" }
                }
                td {
                    div id="overview-other-planets" {
                        @for (id, body) in bodies.iter() {
                            div class="overview-body" {
                                span { (body.name) }
                                a href={ "/overview?body=" (id) } {
                                    img src={ "/static/skins/EpicBlue/bodies/" (body.type_) "_" (body.image) ".jpg" } width="88" height="88";
                                }
                                span { "free" }
                            }
                        }
                    }
                }
            }
            tr {
                td { "Diameter" }
                td colspan="3" { (body.diameter) " km (42 / " (body.max_fields()) " fields)" }
            }
            tr {
                td { "Coordinates" }
                td colspan="3" {
                    a href={ "/galaxy?body=" (body_id) "&galaxy=" (body.coordinates.galaxy) "&system=" (body.coordinates.system) } title="Go to galaxy" {
                        (body.coordinates)
                    }
                }
            }
            tr {
                td { "Points" }
                td colspan="3" { "123,456 (42)" }
            }
        }
    };
    layout(markup).into()
}
