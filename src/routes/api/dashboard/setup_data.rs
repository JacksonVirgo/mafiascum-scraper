use crate::components::{
    buttons::{gen_button, ButtonType, FormSubmitButton},
    forms::input::{gen_input, InputType, SelectMenuInput, TextInput},
};
use actix_web::{get, HttpResponse, Responder};
use maud::html;

#[get("/setup")]
async fn setup_data() -> impl Responder {
    HttpResponse::Ok().body(
        html! {
            div."w-full h-full flex flex-col p-4" {
                h1."text-3xl text-white font-bold pb-2" { "Setup Data" }
                div."text-xl text-white pb-2" { "Enter the data for the setup" }
                form."flex flex-col gap-2" {
                    label."text-xl" for="game_queue" { "Game Queue" }
                    (gen_input(InputType::SelectMenuInput(SelectMenuInput {
                        name: "game_queue".to_string(),
                        placeholder: "Select the game queue".to_string(),
                        options: vec![String::from("Open"), String::from("Newbie"), String::from("Normal"), String::from("Mini/Micro Theme"), String::from("Large Theme"), String::from("Other/Unknown")],
                        is_required: Some(true),
                        default_value: Some(String::from("Other/Unknown"))
                    })))
                    label."text-xl" for="game_index" { "Game Index" }
                    (gen_input(InputType::TextInput(TextInput {
                        name: "game_index".to_string(),
                        placeholder: "Game Index".to_string(),
                        is_required: Some(true),
                        default_value: None
                    })))
                    label."text-xl" for="title" { "Title" }
                    (gen_input(InputType::TextInput(TextInput {
                        name: "title".to_string(),
                        placeholder: "Enter the game title".to_string(),
                        is_required: Some(true),
                        default_value: None
                    })))
                    (gen_button(ButtonType::FormSubmit(FormSubmitButton {
                        text: "Save".to_string(),
                    })))
                }
            }
        }
        .into_string(),
    )
}
