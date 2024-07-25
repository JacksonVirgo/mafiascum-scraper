use maud::{html, Markup};

pub struct SelectMenuInput {
    pub placeholder: String,
    pub name: String,
    pub is_required: Option<bool>,
    pub options: Vec<String>,
    pub default_value: Option<String>,
}

pub struct SelectMenuBuilder {
    pub placeholder: String,
    pub name: String,
    pub is_required: Option<bool>,
    pub options: Vec<String>,
    pub default_value: Option<String>,
}

impl SelectMenuBuilder {
    pub fn new() -> SelectMenuBuilder {
        SelectMenuBuilder {
            placeholder: String::new(),
            name: String::new(),
            is_required: None,
            options: Vec::new(),
            default_value: None,
        }
    }

    pub fn build(self) -> SelectMenuInput {
        SelectMenuInput {
            placeholder: self.placeholder,
            name: self.name,
            is_required: self.is_required,
            options: self.options,
            default_value: self.default_value,
        }
    }

    pub fn build_html(self) -> Markup {
        let input = self.build();
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

    pub fn options(mut self, options: Vec<&str>) -> Self {
        self.options = options.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn default_value(mut self, default_value: &str) -> Self {
        self.default_value = Some(default_value.to_string());
        self
    }
}
