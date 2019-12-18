use crate::service::galaxy::Slot;
use maud::{html, DOCTYPE};

// FIXME: This does not work, we need BodyType.
fn make_body_image_url(type_: i32, image: i32) -> String {
    format!("/static/skin/EpicBlue/bodies/{}_{}.jpg", type_, image)
}

pub fn create_homeworld(slots: &[Option<Slot>]) -> String {
    let markup = html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "Create homeworld - Retro Game" }
            link href="/static/skins/EpicBlue/style.css" rel="stylesheet";
        }
        body {
            div.container {
                div {
                    div {
                        table {
                            tr {
                                th width="30" { "Slot" }
                                th width="30" { "Planet" }
                                th width="130" { "Planet name" }
                                th width="30" { "Moon" }
                                th width="200" { "Debris" }
                                th width="150" { "User name" }
                            }
                            @for i in 1..16 {
                                tr {
                                    td { (i) }
                                    @if let Some(slot) = &slots[i - 1] {
                                        td {
                                            img src=(make_body_image_url(slot.type_, slot.image)) width="30" height="30";
                                        }
                                        td { (slot.name) }
                                    } @else {
                                        td {}
                                        td {
                                            @if i >= 4 && i <= 12 {
                                                form action="/create-homeworld" method="post" {
                                                    input name="galaxy" type="hidden" value="1";
                                                    input name="system" type="hidden" value="1";
                                                    input name="position" type="hidden" value=(i);
                                                    button { "Create homeworld" }
                                                }
                                            }
                                        }
                                    }
                                    td {}
                                    td {}
                                    td {}
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    markup.into()
}
