use trunk_template::components::header::Header;
use yew::{function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <main>
                <h1>{ "Hello World!" }</h1>
            </main>
        </>
    }
}
