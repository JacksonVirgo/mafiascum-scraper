use crate::components::{
    buttons::{gen_button, ButtonType, FormSubmitButton},
    forms::input::{select_menu::SelectMenuBuilder, text_input::TextInputBuilder},
};
use actix_web::{get, HttpResponse, Responder};
use maud::html;

#[get("/setup")]
async fn setup_data() -> impl Responder {
    let game_queue = SelectMenuBuilder::new()
        .name("game_queue")
        .placeholder("Select the game queue")
        .options(vec![
            "Open",
            "Newbie",
            "Normal",
            "Mini/Micro Theme",
            "Large Theme",
            "Other/Unknown",
        ])
        .is_required(true)
        .default_value("Other/Unknown")
        .build_html();

    let game_index = TextInputBuilder::new()
        .name("game_index")
        .placeholder("Game Index")
        .is_required(true)
        .build_html();

    let title = TextInputBuilder::new()
        .name("title")
        .placeholder("Enter the game title")
        .is_required(true)
        .build_html();

    HttpResponse::Ok().body(
        html! {
            div."w-full h-full flex flex-col p-4" {
                h1."text-3xl text-white font-bold pb-2" { "Setup Data" }
                div."text-xl text-white pb-2" { "Enter the data for the setup" }
                form."flex flex-col gap-2" {
                    label."text-xl" for="game_queue" { "Game Queue" }
                    (game_queue)
                    label."text-xl" for="game_index" { "Game Index" }
                    (game_index)
                    label."text-xl" for="title" { "Title" }
                    (title)
                    (gen_button(ButtonType::FormSubmit(FormSubmitButton {
                        text: "Save".to_string(),
                    })))
                }
            }
        }
        .into_string(),
    )
}
