// use crate::{components::home::Home, Routing};
use yew::prelude::*;
use yew_router::prelude::*;
use material_yew::MatButton;

#[function_component(Top)]
fn top() -> Html{
    html!{
        <h1>{ "Hello World" }</h1>
    }
}

#[function_component(Button)]
fn button() -> Html{
    html!{
        <MatButton label="button" />
    }
}

#[function_component(Test)]
fn test() -> Html{
    html!{
        <MatButton label="test" />
    }
}

#[function_component(NotFound)]
fn not_found_404() -> Html{
    html!{
        {"Not found"}
    }
}

macro_rules! base {
    ($url:expr) => {
        at(/titech-tools$url)
    };
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/titech-tools")]
    TOP,
    #[at("/titech-tools/test")]
    TEST,
    #[at("/titech-tools/button")]
    BUTTON,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::TOP => html! {
            <Top />
        },
        Route::TEST => html!{
            <Test />
        },
        Route::BUTTON => html! {
            <Button  />
        },
        Route::NotFound => html! {<NotFound />},
    }
}

fn main() {
    println!("Hello, world!");
    yew::Renderer::<App>::new().render();
}
