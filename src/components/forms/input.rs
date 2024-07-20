use maud::{html, Markup};

pub struct TextInput {
    pub placeholder: String,
    pub name: String,
    pub is_required: Option<bool>,
}

pub enum InputType {
    TextInput(TextInput),
}

pub fn gen_input(raw_input: InputType) -> Markup {
    match raw_input {
        InputType::TextInput(input) => {
            html! {
                input."w-full px-4 py-2 border border-gray-300 rounded text-white bg-zinc-700" type="text" name=(input.name) id=(input.name) placeholder=(input.placeholder) required=(input.is_required.unwrap_or(false)) {}
            }
        }
    }
}
