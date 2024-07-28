use crate::{
    components::{
        buttons::{gen_button, ButtonType, FormSubmitButton},
        forms::input::select_menu::SelectMenuBuilder,
    },
    models::players::{get_players, PlayerAlignment},
    AppState,
};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use maud::{html, Markup};

struct TableRow {
    name: String,
    alignment: String,
    role: String,
    replacements: String,
}
fn format_table_row(row: TableRow) -> Markup {
    html!({
        tr."even:bg-zinc-600" {
            td."px-4 py-2" { (row.name) }
            td."px-4 py-2 border-l border-gray-200" { (row.alignment) }
            td."px-4 py-2 border-l border-gray-200" { (row.role) }
            td."px-4 py-2 border-l border-gray-200" { (row.replacements) }
        }
    })
}

#[get("/players/{thread_id}")]
async fn player_data(state: Data<AppState>, raw_thread_id: web::Path<String>) -> impl Responder {
    let raw_thread_id = raw_thread_id.into_inner();
    let thread_id = match raw_thread_id.parse::<String>() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Found()
                .insert_header(("HX-Location", format!("/")))
                .finish();
        }
    };

    let players = match get_players(&state, &thread_id).await {
        Some(thread) => thread,
        None => {
            return HttpResponse::Found()
                .insert_header(("HX-Location", format!("/")))
                .finish();
        }
    };

    let player_rows: Vec<Markup> = players
        .iter()
        .map(|p| {
            format_table_row(TableRow {
                name: p.name.clone(),
                alignment: match p.alignment.clone() {
                    Some(a) => a.to_string(),
                    None => "Not Set".to_string(),
                },
                role: p.role.clone().unwrap_or("None".to_string()),
                replacements: p.role.clone().unwrap_or("None".to_string()),
            })
        })
        .collect();

    let player_row_count = player_rows.len();

    let alignments = PlayerAlignment::to_vec();
    let default_alignment = match alignments.last() {
        Some(alignment) => Some(alignment.to_string()),
        None => None,
    };

    let alignment = SelectMenuBuilder::new()
        .name("alignment")
        .placeholder("Select the players alignment")
        .options(PlayerAlignment::to_vec())
        .is_required(true)
        .default_value_option(default_alignment)
        .build_html();

    HttpResponse::Ok().body(
        html! {
            div."w-full h-full flex flex-col p-4" {
                h1."text-3xl text-white font-bold pb-2" { "Player Data" }
                div."text-xl text-white pb-2" { "Enter the data for the players in the game" }
                form."flex flex-col gap-2" {
                    label."text-xl" for="game_queue" { "Placeholder" }
                    (alignment)
                    (gen_button(ButtonType::FormSubmit(FormSubmitButton {
                        text: "Save".to_string(),
                    })))
                }
                div."min-w-full" {
                table."w-full bg-zinc-700 text-white" {
                    thead {
                        tr {
                            th."px-4 py-2 border-gray-200 bg-zinc-800" { "Player Name" }
                            th."px-4 py-2 border-l border-gray-200 bg-zinc-800" { "Alignment" }
                            th."px-4 py-2 border-l border-gray-200 bg-zinc-800" { "Role" }
                            th."px-4 py-2 border-l border-gray-200 bg-zinc-800" { "Replacements" }
                        }
                    }
                    tbody id="player-table-body" {
                        @for row in player_rows {
                            (row)
                        }
                    }
                }
                @if player_row_count == 0 {
                    div."bg-zinc-700 w-full" {
                        div."px-4 py-2 text-center w-full"{ "No players found" }
                    }
                }}
            }
        }
        .into_string(),
    )
}
