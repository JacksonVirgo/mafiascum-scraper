use maud::{html, Markup};

pub struct ExternalCTAButton {
    pub text: String,
    pub link: String,
}

pub struct FormSubmitButton {
    pub text: String,
}

pub enum ButtonType {
    ExternalCTA(ExternalCTAButton),
    FormSubmit(FormSubmitButton),
}

pub fn gen_button(btn: ButtonType) -> Markup {
    match btn {
        ButtonType::ExternalCTA(btn) => {
            html! {
                a."text-lg bg-white border-1 border-zinc-400 rounded-full py-2 px-4 mt-4 select-none w-fit hover:cursor-pointer hover:bg-zinc-300 text-black" href=(btn.link)  {
                    (btn.text)
                }
            }
        }
        ButtonType::FormSubmit(btn) => {
            html! {
                button."text-lg bg-white border-1 border-zinc-400 rounded py-2 px-4 mt-4 select-none w-fit hover:cursor-pointer hover:bg-zinc-300 text-black" type="submit" {
                    (btn.text)
                }
            }
        }
    }
}
