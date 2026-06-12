use crate::helper;
use maud::{html, Markup, DOCTYPE};

pub fn auth(slot: Markup) -> Markup {
    let manifest = helper::load_manifest().ok();

    html! {
        (DOCTYPE)
        html lang="it" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "HARM2" }
                @if let Some(manifest) = &manifest {
                    @for item in manifest.values() {
                        @if let Some(css) = &item.css {
                            @for style in css {
                                link rel="stylesheet" href=(format!("/{}", style));
                            }
                        }
                    }
                }
            }
            body { (slot) }
        }
    }
}
