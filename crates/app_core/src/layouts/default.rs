use crate::helper;
use maud::{html, Markup, DOCTYPE};

pub fn default(slot: Markup) -> Markup {
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
                        script src=(format!("/{}", item.file)) type="module" {}
                        @if let Some(css) = &item.css {
                            @for style in css {
                                link rel="stylesheet" href=(format!("/{}", style));
                            }
                        }
                    }
                }
            }
            body {
                nav class="fixed top-0 left-0 w-full flex items-center" {
                    a href="/" { "HARM2" }
                    a href="/sign-in" { "Sign In" }
                    a href="/sign-up" { "Sign Up" }
                    form method="post" action="/sign-out" {
                        button type="submit" { "Logout" }
                    }
                }

                (slot)
            }
        }
    }
}
