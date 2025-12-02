use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use sys_locale::get_locale;
use unic_langid::{LanguageIdentifier, langid};
use views::{Home, ListaView};

mod components;
mod model;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(components::Navbar)]
    #[route("/")]
    Home {},
    #[route("/lista/:id")]
    ListaView { id: usize },
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

thread_local! {static DATABASE: Box<dyn model::DBConnector> = Box::new(model::SQLiteConnector::new());}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));

    let locale_lang_id: LanguageIdentifier = locale.parse().unwrap();

    use_init_i18n(|| {
        I18nConfig::new(locale_lang_id)
            .with_locale((langid!("en"), include_str!("../assets/i18n/en.ftl")))
            .with_locale((langid!("es"), include_str!("../assets/i18n/es.ftl")))
    });
    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
