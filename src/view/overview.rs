use crate::view::layout;
use maud::html;

pub fn overview() -> String {
    let markup = html! {
        table {
            tr {
                th { "Signed in" }
            }
            tr {
                td { "Congrats, you successfully signed in!" }
            }
        }
    };
    layout(markup).into()
}
