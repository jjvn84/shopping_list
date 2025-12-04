use crate::DATABASE;
use crate::components::Toggle;
use crate::model::{Item, ItemForm, Lista};
use dioxus::prelude::*;
use dioxus_i18n::tid;
use dioxus_material_icons::MaterialIcon;

const CLASE_COLOR_ITEM_COMPRADO: &str = "bg-green-300";
const CLASE_COLOR_ITEM_NO_COMPRADO: &str = "bg-gray-300";

#[derive(Clone, Copy)]
struct ListaViewState {
    lista: Signal<Lista>,
}

#[component]
pub fn ListaView(id: usize) -> Element {
    use_context_provider(|| ListaViewState {
        lista: Signal::new(DATABASE.with(|f| f.get_list(id).unwrap())),
    });
    let mut lista = use_context::<ListaViewState>().lista;
    let mut modo_simple = use_signal(|| lista().modo_simple);

    rsx! {
        div { class: "sticky top-14 text-white bg-blue-600 flex justify-between p-1 text-xl mb-2 items-center",
            h1 { class: "flex-none", "{lista().nombre}" }
            Toggle {
                text_size_class: "text-sm",
                checked: modo_simple(),
                value: String::default(),
                onchange: move |_| {
                    let new_value = !modo_simple();
                    let modo_simple_int = if new_value { 1 } else { 0 };
                    _ = DATABASE
                        .with(|f| f.update_list(lista().id, lista().nombre, modo_simple_int));
                    modo_simple.set(new_value);
                },
            }
            h1 { class: if !modo_simple() { "flex-none" } else { "flex-none invisible" },
                {format!("{} {:.2}", tid!("grand_total"), lista().total)}
            }
        }
        div { class: "px-2 mb-2 columns-1 md:columns-2 lg:columns-3 xl:columns-4 2xl:columns-5",
            for item in lista().items.unwrap() {
                if modo_simple() {
                    ItemCardSimple { key: "{item.id}", item }
                } else {
                    ItemCard { key: "{item.id}", item }
                }
            }
        }
        button {
            class: "text-white bg-blue-700 font-medium rounded-lg text-sm m-2 px-5 py-2.5 text-center",
            onclick: move |_| {
                let mut items = lista().items.unwrap();
                items.push(Item::default());
                lista.write().items = Some(items);
            },
            MaterialIcon { name: "add_shopping_cart", size: 24 }
        }
        button {
            class: "text-white bg-blue-700 font-medium rounded-lg text-sm m-2 px-5 py-2.5 text-center",
            onclick: move |_| {
                _ = DATABASE.with(|f| f.clear_list_items(lista().id));
                lista.set(DATABASE.with(|f| f.get_list(lista().id).unwrap()));
            },
            MaterialIcon { name: "remove_shopping_cart", size: 24 }
        }
    }
}

#[component]
fn ItemCard(item: Item) -> Element {
    let mut lista: Signal<Lista> = use_context::<ListaViewState>().lista;

    let bg_card_color = if item.cantidad_comprada > 0 as f32 {
        CLASE_COLOR_ITEM_COMPRADO
    } else {
        CLASE_COLOR_ITEM_NO_COMPRADO
    };

    let precio_total = item.cantidad_comprada * item.precio;

    fn handle_change(event: Event<FormData>) {
        let item = event.parsed_values::<ItemForm>().unwrap().into_item();
        if item.id == 0 {
            _ = DATABASE.with(|f| f.create_new_list_item(item.id_lista, item));
        } else {
            _ = DATABASE.with(|f| f.update_list_item(item));
        };
    }

    rsx! {
        form {
            class: "{bg_card_color} rounded-lg p-2 break-inside-avoid-column mb-2",
            onchange: {
                move |event: Event<FormData>| {
                    handle_change(event);
                    lista.set(DATABASE.with(|f| f.get_list(lista().id).unwrap()));
                }
            },
            input { r#type: "hidden", name: "id", value: "{item.id}" }
            input { r#type: "hidden", name: "id_lista", value: "{lista().id}" }
            div { class: "flex text-md justify-between",
                div { class: "flex",
                    input {
                        r#type: "number",
                        class: "w-[5ch]",
                        name: "cantidad_requerida",
                        value: "{item.cantidad_requerida:.3}",
                    }
                    select { class: "w-[8ch]", name: "unidad",
                        option {
                            value: "unidad",
                            selected: item.unidad == "unidad",
                            {tid!("unidad")}
                        }
                        option { value: "kg", selected: item.unidad == "kg", {tid!("kg")} }
                        option {
                            value: "docena",
                            selected: item.unidad == "docena",
                            {tid!("docena")}
                        }
                    }
                }
                div { class: "flex",
                    input {
                        r#type: "number",
                        class: "w-[4ch]",
                        name: "precio",
                        value: "{item.precio:.2}",
                    }
                    {format!(" {} {}", tid!("per"), tid!(& item.unidad))}
                }
                button {
                    r#type: "button",
                    class: "text-red-600 rounded-full px-5 text-center",
                    onclick: move |_| {
                        let _ = DATABASE.with(|f| f.delete_item(item.id));
                        lista.set(DATABASE.with(|f| f.get_list(lista().id).unwrap()));
                    },
                    MaterialIcon { name: "delete" }
                }
            }
            div { class: "flex text-lg font-bold justify-between",
                div {
                    input {
                        r#type: "text",
                        class: "w-42",
                        name: "nombre",
                        value: item.nombre,
                    }
                }
                div {
                    input {
                        r#type: "number",
                        class: "w-[5ch]",
                        name: "cantidad_comprada",
                        value: "{item.cantidad_comprada:.3}",
                    }
                    {format!("{} {:.2}", tid!("total"), precio_total)}
                }
            }
        }
    }
}

#[component]
fn ItemCardSimple(item: Item) -> Element {
    let mut lista: Signal<Lista> = use_context::<ListaViewState>().lista;

    let bg_card_color = if item.cantidad_comprada > 0 as f32 {
        CLASE_COLOR_ITEM_COMPRADO
    } else {
        CLASE_COLOR_ITEM_NO_COMPRADO
    };

    fn handle_change(event: Event<FormData>) {
        let item = event.parsed_values::<ItemForm>().unwrap().into_item();
        if item.id == 0 {
            _ = DATABASE.with(|f| f.create_new_list_item(item.id_lista, item));
        } else {
            _ = DATABASE.with(|f| f.update_list_item(item));
        };
    }

    rsx! {
        form {
            class: "{bg_card_color} rounded-lg p-2 break-inside-avoid-column mb-2 flex text-lg font-bold justify-between",
            onchange: {
                move |event: Event<FormData>| {
                    handle_change(event);
                    lista.set(DATABASE.with(|f| f.get_list(lista().id).unwrap()));
                }
            },
            input { r#type: "hidden", name: "id", value: "{item.id}" }
            input { r#type: "hidden", name: "id_lista", value: "{lista().id}" }
            input { r#type: "hidden", name: "unidad", value: "{item.unidad}" }
            input {
                r#type: "hidden",
                name: "cantidad_requerida",
                value: "{item.cantidad_requerida}",
            }
            input { r#type: "hidden", name: "precio", value: "{item.precio}" }
            input {
                r#type: "text",
                class: "w-42",
                name: "nombre",
                value: item.nombre,
            }
            input {
                r#type: "checkbox",
                role: "switch",
                class: "input",
                value: "1.000",
                name: "cantidad_comprada",
                checked: item.cantidad_comprada >= 0.001,
            }
            button {
                r#type: "button",
                class: "text-red-600 rounded-full px-5 text-center",
                onclick: move |_| {
                    let _ = DATABASE.with(|f| f.delete_item(item.id));
                    lista.set(DATABASE.with(|f| f.get_list(lista().id).unwrap()));
                },
                MaterialIcon { name: "delete" }
            }
        }
    }
}
