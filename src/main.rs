use webapp::App;
use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx, App(None) });
}
