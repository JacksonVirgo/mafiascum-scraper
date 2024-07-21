use maud::{html, Markup};

pub fn gen_spinner() -> Markup {
    html! {
        div class="flex justify-center items-center h-auto" {
            div class="w-12 h-12 border-4 border-t-4 border-gray-300 border-solid rounded-full animate-spin" style="border-top-color: #ffffff;" {}
        }
    }
}
