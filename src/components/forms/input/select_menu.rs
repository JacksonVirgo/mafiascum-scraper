use maud::{html, Markup};

pub struct SelectMenuInput {
    pub placeholder: String,
    pub name: String,
    pub is_required: bool,
    pub options: Vec<String>,
    pub default_value: Option<String>,
}

pub struct SelectMenuBuilder {
    pub placeholder: String,
    pub name: String,
    pub is_required: bool,
    pub options: Vec<String>,
    pub default_value: Option<String>,
}

impl SelectMenuBuilder {
    pub fn new() -> SelectMenuBuilder {
        SelectMenuBuilder {
            placeholder: String::new(),
            name: String::new(),
            is_required: false,
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
        let default_value = input.default_value.clone();

        let add_row = |option: String| match &default_value {
            Some(default) => {
                if &option == default {
                    html! {
                        option value=(option.clone()) selected {
                            (option)
                        }
                    }
                } else {
                    html! {
                        option value=(option.clone()) {
                            (option)
                        }
                    }
                }
            }
            None => {
                html! {
                    option value=(option.clone()) {
                        (option)
                    }
                }
            }
        };

        if input.is_required {
            return html! {
                select."w-full px-4 py-2 border border-gray-300 rounded text-white bg-zinc-700" name=(input.name) id=(input.name) required {
                    option value="" disabled selected {
                        (input.placeholder)
                    }

                    @for option in &input.options {
                        (add_row(option.clone()))
                    }
                }
            };
        }

        html! {
            select."w-full px-4 py-2 border border-gray-300 rounded text-white bg-zinc-700" name=(input.name) id=(input.name) {
                option value="" disabled selected {
                    (input.placeholder)
                }

                @for option in &input.options {
                    (add_row(option.clone()))
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
        self.is_required = is_required;
        self
    }

    pub fn options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }

    pub fn default_value_option(mut self, default_value: Option<String>) -> Self {
        if let Some(default_value) = default_value {
            self.default_value = Some(default_value.to_string());
        }
        self
    }
}
