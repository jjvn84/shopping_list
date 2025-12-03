use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    text_size_class: String,
    checked: bool,
    value: String,
    onchange: Callback<Event<FormData>>,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    rsx! {
        div {
            class: format!(
                "relative flex justify-between items-center group p-2 {}",
                props.text_size_class,
            ),
            input {
                r#type: "checkbox",
                class: "absolute left-1/2 -translate-x-1/2 w-full h-full peer appearance-none rounded-md",
                checked: props.checked,
                value: props.value,
                onchange: props.onchange,
            }
            span { class: "w-[6ch] h-[3ch] flex items-center shrink-0 ml-4 p-1 bg-gray-300 rounded-full duration-300 ease-in-out peer-checked:bg-blue-800 after:w-[3ch] after:h-[3ch] after:bg-white after:rounded-full after:shadow-md after:duration-300 peer-checked:after:translate-x-[2ch] group-hover:after:translate-x-[1ch]" }
        }
    }
}
