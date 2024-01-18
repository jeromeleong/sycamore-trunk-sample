use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, StaticRouter};

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/list")]
    List,
    #[not_found]
    NotFound,
}

fn switch<'a, G: Html>(cx: Scope<'a>, route: &'a ReadSignal<AppRoutes>) -> View<G> {
    view! { cx,
        div {
            (match route.get().as_ref() {
                AppRoutes::Index => view! { cx,
                    p(class="text-4xl h-dvh"){
                        "Index"
                    }
                },
                AppRoutes::List => view! { cx,
                    p{
                        "list"
                    }
                },
                AppRoutes::NotFound => view! { cx,
                    p{
                        "404 Not Found"
                    }
                },
            })
        }
    }
}

#[component]
pub fn App<G: Html>(cx: BoundedScope , app_path: Option<String>) -> View<G> {
    match app_path {
        Some(app_path) => {
            let route = AppRoutes::default().match_path(&app_path);
            view! { cx,
                StaticRouter (
                    view = switch,
                    route = route,
                )
            }
        }
        None => view! { cx,
            Router (
                view = switch,
                integration = HistoryIntegration::new(),
            )
        },
    }

}