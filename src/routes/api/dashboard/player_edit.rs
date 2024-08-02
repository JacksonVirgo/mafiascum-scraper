use std::str::FromStr;

use crate::{
    components::{
        buttons::{gen_button, ButtonType, FormSubmitButton},
        forms::input::{select_menu::SelectMenuBuilder, text_input::TextInputBuilder},
    },
    models::players::{get_player, update_player, PlayerAlignment, UpdatePlayer},
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
    name: String,
    role: Option<String>,
    alignment: Option<String>,
    alias: Option<String>,
    replacements: Option<String>,
}

#[get("/playeredit/{thread_id}/{player_id}")]
async fn player_data(state: Data<AppState>, path: web::Path<(String, i32)>) -> impl Responder {
    let (raw_thread_id, player_id) = path.into_inner();

    let thread_id = match raw_thread_id.parse::<String>() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Found()
                .insert_header(("HX-Location", format!("/dashboard/{}?d=2", raw_thread_id)))
                .finish();
        }
    };

    let player = match get_player(&state, player_id).await {
        Some(player) => player,
        None => {
            return HttpResponse::Found()
                .insert_header(("HX-Location", format!("/dashboard/{}?d=2", thread_id)))
                .finish();
        }
    };

    let name_input = TextInputBuilder::new()
        .name("name")
        .placeholder("Enter the players name")
        .is_required(true)
        .default_value(&player.name)
        .build_html();

    let alignment_input = SelectMenuBuilder::new()
        .name("alignment")
        .placeholder("Select the players alignment")
        .options(PlayerAlignment::to_vec())
        .default_value_option(match player.alignment {
            Some(a) => Some(a.to_string()),
            None => None,
        })
        .build_html();

    let role_input = TextInputBuilder::new()
        .name("role")
        .placeholder("Enter the players role")
        .default_value_optional(player.role.clone())
        .build_html();

    let alias_input = TextInputBuilder::new()
        .name("alias")
        .placeholder("Enter the players alias")
        .default_value(
            &player
                .aliases
                .iter()
                .map(|alias| alias.to_string().replace(",", "%2C"))
                .collect::<Vec<String>>()
                .join(","),
        )
        .build_html();

    let replacements_input = TextInputBuilder::new()
        .name("replacements")
        .placeholder("Enter the players replacements")
        .default_value(
            &player
                .replacements
                .iter()
                .map(|replacement| replacement.to_string().replace(",", "%2C"))
                .collect::<Vec<String>>()
                .join(","),
        )
        .build_html();

    let form_post_uri = format!("/api/dashboard/playeredit/{}/{}", thread_id, player_id);
    let form_back_btn_uri = format!("/dashboard/{}?d=2", thread_id);

    HttpResponse::Ok().body(html! {
        div."w-full h-full flex flex-col p-4" id="player-wrapper" {
            h1."text-3xl text-white font-bold pb-2" { "Edit Player" }
            div."text-xl text-white pb-2" { "Enter the data for the players in the game" }
            a."text-red-400 hover:text-red-600 hover:cursor-pointer hover:underline" href=(form_back_btn_uri) { "Go Back" }
            form."flex flex-col gap-2" hx-post=(form_post_uri) hx-target="#player-wrapper" hx-swap="outerHTML" {
                label."text-xl" for="name" { "Username" }
                (name_input)

                label."text-xl" for="alignment" { "Alignment" }
                (alignment_input)

                label."text-xl" for="role" { "Role" }
                (role_input)

                label."text-xl" for="alias" { 
                    div {"Aliases"}  
                    div."text-sm text-zinc-500" { "Comma seperated list of aliases" }
                }
                (alias_input)

                label."text-xl" for="replacements" { 
                    div {"Replacements"}  
                    div."text-sm text-zinc-500" { "Comma seperated list of replacements" }
                }
                (replacements_input)

                (gen_button(ButtonType::FormSubmit(FormSubmitButton {
                    text: "Save".to_string(),
                })))
            }
        }
    }.into_string())
}

#[post("/playeredit/{thread_id}/{player_id}")]
async fn player_edit(
    state: Data<AppState>,
    path: web::Path<(String, i32)>,
    form: web::Form<FormData>,
) -> impl Responder {
    let (raw_thread_id, player_id) = path.into_inner();
    let thread_id = match raw_thread_id.parse::<String>() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Found()
                .insert_header(("HX-Location", format!("/dashboard/{}?d=2", raw_thread_id)))
                .finish();
        }
    };

    let form_data = form.into_inner();
    let _ = update_player(
        &state,
        UpdatePlayer {
            id: player_id,
            name: form_data.name,
            alignment: match form_data.alignment {
                Some(a) => {
                    Some(PlayerAlignment::from_str(a.as_str()).unwrap_or(PlayerAlignment::Unknown))
                }
                None => None,
            },
            role: match form_data.role {
                Some(r) => Some(r),
                None => None,
            },
            aliases: match form_data.alias {
                Some(a) => a
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>(),
                None => Vec::new(),
            },
            replacements: match form_data.replacements {
                Some(r) => r
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>(),
                None => Vec::new(),
            },
        },
    )
    .await;

    HttpResponse::Found()
        .insert_header(("HX-Location", format!("/dashboard/{}?d=2", thread_id)))
        .finish()
}
