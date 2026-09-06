use hypertext::prelude::*;

use crate::helper::vite::load_manifest_entry;

pub struct Props<S: Renderable> {
    pub slot: S,
    pub path: String,
}

#[component]
fn mobile_bar_link<'a>(current_path: &String, href: &'a str, text: &'a str) -> impl Renderable {
    let current_path = current_path.strip_prefix("/").unwrap_or("");
    let href_path = href.strip_prefix("/").unwrap_or("");

    let class = if current_path == href_path {
        "flex-row basis-1/2 gap-2 dock-active text-primary after:content-none"
    } else {
        "flex-row basis-1/2 gap-2"
    };

    rsx! {
        <a class=(class) href=(href)>
            <span class="inline-block my-1.5 size-5 icon-[material-symbols--note-stack-outline]"></span>
            <span class="dock-label">(text)</span>
        </a>
    }
}

#[component]
fn bar_link<'a>(current_path: &String, href: &'a str, icon: &'a str) -> impl Renderable {
    let current_path = current_path.strip_prefix("/").unwrap_or("");
    let href_path = href.strip_prefix("/").unwrap_or("");

    let class = if current_path == href_path {
        "menu-active text-primary"
    } else {
        ""
    };

    rsx! {
        <a class=(class) href=(href)>
            <span class=(format!("inline-block my-1.5 size-5 {icon}"))></span>
        </a>
    }
}

#[component]
fn status_bar() -> impl Renderable {
    rsx! {
        <div x-data>
           <div class="flex items-center bg-base-300 border-neutral border-t">
               <div class="bg-primary px-2.5 font-medium text-primary-content cursor-pointer"
                   "x-bind"="$store.statusBar.modeButton"
                   x-text="$store.statusBar.mode">
                   NORMAL
                </div>

               <span class="px-2.5 truncate" x-text="$store.statusBar.url"></span>

               <span class="flex gap-2.5 ml-auto px-2.5">
                   <span>"12:4"</span>
               </span>
            </div>

            <div class="bg-base-100 px-2.5 text-error text-sm h-6" x-text="$store.statusBar.error"></div>
        </div>
    }
}

pub fn default<S: Renderable>(props: Props<S>) -> Rendered<String> {
    rsx! {
        <!DOCTYPE html>
        <html lang="it" "data-theme"="tui">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"HARM2"</title>

                (load_manifest_entry("frontend/style.css"))
                (load_manifest_entry("frontend/main.ts"))
            </head>
            <body>
                <div class="flex md:flex-row flex-col h-screen">
                    <nav id="desktop-nav" class="hidden md:flex flex-col items-center bg-base-200 border-stone-800 border-r grow-0 shrink-0">
                        <div></div>

                        <ul "hx-boost:inherited"="true" class="menu">
                            <li>
                            	<BarLink current_path=(props.path.to_owned()) href="/" icon="icon-[material-symbols--note-stack-outline]" />
                            </li>
                            <li>
                            	<BarLink current_path=(props.path.to_owned()) href="/games" icon="icon-[material-symbols--stadia-controller-outline]" />
                            </li>
                            <li>
                                <BarLink current_path=(props.path.to_owned()) href="/movies" icon="icon-[material-symbols--movie-outline]" />
                            </li>
                        </ul>
                    </nav>

                    <div id="content" class="flex flex-col flex-1 h-full min-h-0">
                        <div class="flex flex-1 min-h-0 overflow-y-auto">
                            (props.slot)
                        </div>

                        <nav id="mobile-nav" "hx-boost:inherited"="true" class="md:hidden static justify-center bg-base-200 border-base-300 border-t dock dock-sm">
                        	<MobileBarLink current_path=(props.path.to_owned()) href="/" text="Notes" />
                         	<MobileBarLink current_path=(props.path.to_owned()) href="/games" text="Games" />
                         	<MobileBarLink current_path=(props.path.to_owned()) href="/movies" text="Movies" />
                        </nav>

                        <StatusBar />
                    </div>
                </div>

            </body>
        </html>
    }.render()
}
