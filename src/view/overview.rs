use crate::context::Context;
use crate::view::layout;
use maud::html;

pub fn overview(context: &Context) -> String {
    let markup = html! {
        table {
            tr {
                th colspan="2" { "Overview" }
            }
            @for (id, body) in context.bodies().iter() {
                tr {
                    td witdh="50%" { (id) }
                    td width="50%" { (body.name) }
                }
            }
        }
    };
    layout(markup).into()
}
