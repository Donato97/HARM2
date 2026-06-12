use maud::{html, Markup};

use crate::layouts;

pub async fn index() -> Markup {
    let markup = html! {
        div id="root" {}
    };

    layouts::default(markup)
}
