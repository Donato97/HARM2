use hypertext::prelude::*;

use crate::layouts;

pub fn sign_up() -> Rendered<String> {
    let markup = rsx! {
        <div class="h-screen flex flex-col items-center justify-center">
            <form
                class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4"
                method="post"
                action="/sign-up"
            >
                <label class="label">"Email"</label>
                <input class="input" type="text" name="email" placeholder="example@example.com">

                <label class="label">"Password"</label>
                <input class="input" type="password" name="password" placeholder="Password">

                <button class="btn btn-neutral mt-4">"Sign Up"</button>
            </form>

            <p class="mt-4">
                "Already have an account? "
                <a class="link link-hover link-primary" href="/sign-in">"Sign In"</a>
            </p>
        </div>
    };

    layouts::auth(markup)
}

pub fn sign_in() -> Rendered<String> {
    let markup = rsx! {
        <div class="h-screen flex flex-col items-center justify-center">
            <form
                class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4"
                method="post"
                action="/sign-in"
            >
                <label class="label">"Email"</label>
                <input class="input" type="text" name="email" placeholder="example@example.com">

                <label class="label">"Password"</label>
                <input class="input" type="password" name="password" placeholder="Password">

                <button class="btn btn-neutral mt-4">"Sign In"</button>
            </form>

            <p class="mt-4">
                "First time? "
                <a class="link link-hover link-primary" href="/sign-up">"Sign Up"</a>
            </p>
        </div>
    };

    layouts::auth(markup)
}