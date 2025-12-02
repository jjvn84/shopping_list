use crate::DATABASE;
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::tid;
use dioxus_material_icons::MaterialIcon;

#[component]
pub fn Home() -> Element {
    let mut nombre_nueva_lista = use_signal(|| "".to_string());
    let mut listas = use_signal(|| DATABASE.with(|f| f.get_list_of_lists()).unwrap_or_default());
    let mut editing_list_id = use_signal(|| 0);
    let mut editing_list_name = use_signal(|| "".to_string());
    rsx! {
        div { id: "home", class: "space-y-6",
            div { class: "my-5 columns-1 md:columns-2 lg:columns-3 xl:columns-4 2xl:columns-5",
                div { class: "break-inside-avoid-column",
                    label {
                        class: "block px-1 mb-2 text-sm font-medium text-gray-900 dark:text-white",
                        r#for: "nueva_lista",
                        {tid!("create_new_list")}
                    }
                    div { class: "flex gap-x-2 px-2 col-span-1",
                        input {
                            r#type: "text",
                            class: "bg-gray-50 border border-gray-300 text-gray-900 text-md rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                            placeholder: tid!("new_list_name"),
                            value: nombre_nueva_lista,
                            oninput: move |event| nombre_nueva_lista.set(event.value()),
                        }
                        button {
                            class: "basis-1/6 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                            onclick: move |_| {
                                let nombre = nombre_nueva_lista.read().to_string();
                                let _ = DATABASE.with(|f| f.create_new_list(nombre));
                                nombre_nueva_lista.set("".to_string());
                                listas.set(DATABASE.with(|f| f.get_list_of_lists()).unwrap_or_default());
                            },
                            MaterialIcon { name: "add", size: 24 }
                        }
                    }
                }
            }
            if listas.len() > 0 {
                h3 { class: "my-5 text-sm px-1 row-start-2", {tid!("created_lists")} }
                div { class: "px-2 space-y-6 columns-1 md:columns-2 lg:columns-3 xl:columns-4 2xl:columns-5 row-start-3",
                    for lista in listas.cloned() {
                        div {
                            key: "{lista.id}",
                            class: "flex flex-row p-3 text-lg rounded-lg items-center mb-2 justify-between bg-gray-300",
                            if editing_list_id() != lista.id {
                                Link {
                                    class: "flex-1",
                                    to: Route::ListaView { id: lista.id },
                                    "{lista.nombre}"
                                }
                                button {
                                    r#type: "button",
                                    class: "text-blue-600 focus:outline-none rounded-full px-5 text-center",
                                    onclick: move |_| {
                                        editing_list_id.set(lista.id);
                                        editing_list_name.set(lista.nombre.clone());
                                    },
                                    MaterialIcon { name: "edit", size: 24 }
                                }
                                button {
                                    r#type: "button",
                                    class: "text-red-600 focus:outline-none rounded-full px-5 text-center",
                                    onclick: move |_| {
                                        let _ = DATABASE.with(|f| f.delete_list(lista.id));
                                        listas.set(DATABASE.with(|f| f.get_list_of_lists()).unwrap_or_default());
                                    },
                                    MaterialIcon { name: "delete", size: 24 }
                                }
                            } else {
                                input {
                                    r#type: "text",
                                    class: "flex-1",
                                    value: editing_list_name,
                                    oninput: move |event| editing_list_name.set(event.value()),
                                }
                                button {
                                    r#type: "button",
                                    class: "text-green-600 focus:outline-none rounded-full px-5 text-center",
                                    onclick: move |_| {
                                        let modo_simple_int = if lista.modo_simple { 1 } else { 0 };
                                        _ = DATABASE
                                            .with(|f| f.update_list(lista.id, editing_list_name(), modo_simple_int));
                                        editing_list_id.set(0);
                                        listas.set(DATABASE.with(|f| f.get_list_of_lists()).unwrap_or_default());
                                    },
                                    MaterialIcon { name: "check", size: 24 }
                                }
                                button {
                                    r#type: "button",
                                    class: "text-red-600 focus:outline-none rounded-full px-5 text-center",
                                    onclick: move |_| editing_list_id.set(0),
                                    MaterialIcon { name: "clear", size: 24 }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
