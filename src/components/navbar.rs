use crate::Route;
use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon, MaterialIconStylesheet};

#[component]
pub fn Navbar() -> Element {
    let current_route: Route = use_route();
    rsx! {
        MaterialIconStylesheet {}
        div {
            id: "navbar",
            class: "sticky top-0 flex text-white bg-blue-600 text-2xl p-3 justify-between items-center",
            MaterialIcon { name: "shopping_cart" }
            "Shopping Lists"
            if current_route.to_string() == "/" {
                MaterialIcon { name: "home" }
            } else {
                Link { to: Route::Home {},
                    MaterialIcon { name: "home" }
                }
            }
        }
        Outlet::<Route> {}
    }
}
