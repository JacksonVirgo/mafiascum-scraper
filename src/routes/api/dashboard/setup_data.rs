use crate::models::thread::{get_thread, update_thread};
use crate::{
    components::{
        buttons::{gen_button, ButtonType, FormSubmitButton},
        forms::input::{
            number_input::NumberInputBuilder, select_menu::SelectMenuBuilder,
            text_input::TextInputBuilder,
        },
    },
    AppState,
};
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use maud::html;

#[derive(serde::Deserialize, Debug)]
struct FormData {
    game_queue: String,
    game_index: i32,
    title: String,
}

#[get("/setup/{thread_id}")]
async fn setup_data(state: Data<AppState>, raw_thread_id: web::Path<String>) -> impl Responder {
    let raw_thread_id = raw_thread_id.into_inner();
    let thread_id = match raw_thread_id.parse::<String>() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Found()
                .insert_header(("Location", format!("/")))
                .finish();
        }
    };

    let thread = match get_thread(&state, &thread_id).await {
        Some(thread) => thread,
        None => {
            return HttpResponse::Found()
                .insert_header(("Location", format!("/")))
                .finish();
        }
    };

    println!("{:?}", thread);

    let game_queue: Option<String> = thread.queue.clone();
    let game_index: Option<i32> = thread.queue_index;
    let title: Option<String> = thread.title.clone();

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
        .default_value_option(game_queue)
        .build_html();

    let game_index = NumberInputBuilder::new()
        .name("game_index")
        .placeholder("Game Index")
        .is_required(true)
        .default_value_option(game_index)
        .build_html();

    let title = TextInputBuilder::new()
        .name("title")
        .placeholder("Enter the game title")
        .is_required(true)
        .default_value_optional(title)
        .build_html();

    let form_post_uri = format!("/api/dashboard/setup/{}", thread_id);

    HttpResponse::Ok().body(
        html! {
            div."w-full h-full flex flex-col p-4" id="setup-wrapper" {
                h1."text-3xl text-white font-bold pb-2" { "Setup Data" }
                div."text-xl text-white pb-2" { "Enter the data for the setup" }
                form."flex flex-col gap-2" hx-post=(form_post_uri) hx-target="#setup-wrapper" hx-swap="outerHTML" {
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

#[post("/setup/{thread_id}")]
async fn submit_setup_data(
    raw_thread_id: web::Path<String>,
    state: Data<AppState>,
    form: web::Form<FormData>,
) -> impl Responder {
    let raw_thread_id = raw_thread_id.into_inner();
    let thread_id = match raw_thread_id.parse::<String>() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Found()
                .insert_header(("Location", format!("/")))
                .finish();
        }
    };

    let form_data = form.into_inner();
    let _ = update_thread(
        &state,
        &thread_id,
        crate::models::thread::ThreadUpdate {
            title: form_data.title,
            queue: form_data.game_queue,
            queue_index: form_data.game_index,
        },
    )
    .await;

    HttpResponse::Found()
        .insert_header(("Location", format!("/api/dashboard/setup/{}", thread_id)))
        .finish()
}
