use maud::{html, Markup};

pub struct TextInput {
    pub placeholder: String,
    pub name: String,
    pub is_required: Option<bool>,
    pub default_value: Option<String>,
}

pub struct SelectMenuInput {
    pub placeholder: String,
    pub name: String,
    pub is_required: Option<bool>,
    pub options: Vec<String>,
    pub default_value: Option<String>,
}

pub enum InputType {
    TextInput(TextInput),
    SelectMenuInput(SelectMenuInput),
}

pub fn gen_input(raw_input: InputType) -> Markup {
    match raw_input {
        InputType::TextInput(input) => {
            html! {
                input."w-full px-4 py-2 border border-gray-300 rounded text-white bg-zinc-700" type="text" name=(input.name) id=(input.name) placeholder=(input.placeholder) required=(input.is_required.unwrap_or(false)) value=(input.default_value.unwrap_or("".to_string())) {}
            }
        }
        InputType::SelectMenuInput(input) => {
            html! {
                select."w-full px-4 py-2 border border-gray-300 rounded text-white bg-zinc-700" name=(input.name) id=(input.name) placeholder=(input.placeholder) required=(input.is_required.unwrap_or(false)) {
                    @for option in &input.options {
                        option value=(option.clone()) selected=(Some(option.clone()) == input.default_value.clone()) {
                            (option)
                        }
                    }
                }
            }
        }
    }
}
