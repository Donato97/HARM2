use maud::{html, Markup, DOCTYPE};

use crate::helper::vite::load_manifest_entry;

pub struct Props {
    pub slot: Markup,
    pub path: String,
}

pub fn default(props: Props) -> Markup {
    html! {
        (DOCTYPE)
        html lang="it" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "HARM2" }

                (load_manifest_entry("frontend/style.css"))
                (load_manifest_entry("frontend/htmx.ts"))
            }
            body {
                div class="flex flex-col md:flex-row h-screen" {
                    nav id="desktop-nav" class="hidden md:flex flex-col items-center grow-0 shrink-0 bg-base-200 border-r border-stone-800" {
                        div {

                        }

                        ul hx-boost:inherited="true" class="menu" {
                            li {
                                a."menu-active text-primary"[props.path == ""] href="/" {
                                    span class="icon-[material-symbols--note-stack-outline] my-1.5 inline-block size-5" {}
                                }
                            }
                            li {
                                a."menu-active text-primary"[props.path == "/games"] href="/games" {
                                    span class="icon-[material-symbols--stadia-controller-outline] my-1.5 inline-block size-5" {}
                                }
                            }
                            li {
                                a."menu-active text-primary"[props.path == "/movies"] href="/movies" {
                                    span class="icon-[material-symbols--movie-outline] my-1.5 inline-block size-5" {}
                                }
                            }
                        }
                    }

                    div id="content" class="h-full flex-1 min-h-0" {
                        (props.slot)
                    }

                    nav id="mobile-nav" class="md:hidden dock dock-sm static bg-base-200 justify-center border-t border-stone-800" {
                        a."basis-1/2"."dock-active text-primary"[props.path == ""] href="/" {
                            span class="icon-[material-symbols--note-stack-outline] my-1.5 inline-block size-5" {}
                        }
                        a."basis-1/2"."dock-active text-primary"[props.path == "/games"] href="/games" {
                            span class="icon-[material-symbols--stadia-controller-outline] my-1.5 inline-block size-5" {}
                        }
                        a."basis-1/2"."dock-active text-primary"[props.path == "/movies"] href="/movies" {
                            span class="icon-[material-symbols--movie-outline] my-1.5 inline-block size-5" {}
                        }
                    }
                }
            }
        }
    }
}
