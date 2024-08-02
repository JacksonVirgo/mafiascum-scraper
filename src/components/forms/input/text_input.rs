use maud::{html, Markup};

#[derive(Debug, Default)]
pub struct TextInput {
    pub placeholder: String,
    pub name: String,
    pub is_required: bool,
    pub default_value: Option<String>,
    pub is_hidden: bool,
}

pub struct TextInputBuilder {
    pub placeholder: String,
    pub name: String,
    pub is_required: bool,
    pub default_value: Option<String>,
    pub is_hidden: bool,
}

impl TextInputBuilder {
    pub fn new() -> TextInputBuilder {
        TextInputBuilder {
            placeholder: String::new(),
            name: String::new(),
            is_required: false,
            default_value: None,
            is_hidden: false,
        }
    }

    pub fn build(self) -> TextInput {
        TextInput {
            placeholder: self.placeholder,
            name: self.name,
            is_required: self.is_required,
            default_value: self.default_value,
            is_hidden: self.is_hidden,
        }
    }

    pub fn build_html(self) -> Markup {
        let input = self.build();

        let mut style =
            "w-full px-4 py-2 border border-gray-300 rounded text-white bg-zinc-700".to_string();
        if input.is_hidden {
            style.push_str(" hidden");
        }

        if input.is_required {
            return html! {
                input.(style) type="text" name=(input.name) id=(input.name) placeholder=(input.placeholder) required value=(input.default_value.unwrap_or("".to_string())) {}
            };
        }

        html! {
            input.(style) type="text" name=(input.name) id=(input.name) placeholder=(input.placeholder) value=(input.default_value.unwrap_or("".to_string())) {}
        }
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn is_required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        self
    }

    pub fn default_value(mut self, default_value: &str) -> Self {
        self.default_value = Some(default_value.to_string());
        self
    }

    pub fn default_value_optional(mut self, default_value: Option<String>) -> Self {
        self.default_value = default_value;
        self
    }
}
