use maud::{html, Markup};

#[derive(Debug, Default)]
pub struct NumberInput {
    pub placeholder: String,
    pub name: String,
    pub is_required: Option<bool>,
    pub default_value: Option<i32>,
}

pub struct NumberInputBuilder {
    pub placeholder: String,
    pub name: String,
    pub is_required: Option<bool>,
    pub default_value: Option<i32>,
}

impl NumberInputBuilder {
    pub fn new() -> NumberInputBuilder {
        NumberInputBuilder {
            placeholder: String::new(),
            name: String::new(),
            is_required: None,
            default_value: None,
        }
    }

    pub fn build(self) -> NumberInput {
        NumberInput {
            placeholder: self.placeholder,
            name: self.name,
            is_required: self.is_required,
            default_value: self.default_value,
        }
    }

    pub fn build_html(self) -> Markup {
        let input = self.build();
        html! {
            input."w-full px-4 py-2 border border-gray-300 rounded text-white bg-zinc-700"
                type="number"
                name=(input.name)
                id=(input.name)
                placeholder=(input.placeholder)
                required=(input.is_required.unwrap_or(false))
                value=(input.default_value.map_or_else(|| "".to_string(), |v| v.to_string()))
            {}
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
        self.is_required = Some(is_required);
        self
    }

    pub fn default_value_option(mut self, default_value: Option<i32>) -> Self {
        self.default_value = default_value;
        self
    }
}
