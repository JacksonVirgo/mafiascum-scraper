use crate::components::{
    buttons::{gen_button, ButtonType, FormSubmitButton},
    forms::input::{gen_input, InputType, SelectMenuInput},
};
use actix_web::{get, HttpResponse, Responder};
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

#[get("/votes")]
async fn vote_data() -> impl Responder {
    HttpResponse::Ok().body(
        html! {
            div."w-full h-full flex flex-col p-4" {
                h1."text-3xl text-white font-bold pb-2" { "Player Data" }
                div."text-xl text-white pb-2" { "Enter the data for the players in the game" }
                form."flex flex-col gap-2" {
                    label."text-xl" for="game_queue" { "Placeholder" }
                    (gen_input(InputType::SelectMenuInput(SelectMenuInput {
                        name: "game_queue".to_string(),
                        placeholder: "Select the game queue".to_string(),
                        options: vec![String::from("Open"), String::from("Newbie"), String::from("Normal"), String::from("Mini/Micro Theme"), String::from("Large Theme"), String::from("Other/Unknown")],
                        is_required: Some(true),
                        default_value: Some(String::from("Other/Unknown"))
                    })))
                    (gen_button(ButtonType::FormSubmit(FormSubmitButton {
                        text: "Save".to_string(),
                    })))
                }
                table."min-w-full bg-zinc-700 text-white" {
                    thead {
                        tr {
                            th."px-4 py-2 border-gray-200 bg-zinc-800" { "Player Name" }
                            th."px-4 py-2 border-l border-gray-200 bg-zinc-800" { "Alignment" }
                            th."px-4 py-2 border-l border-gray-200 bg-zinc-800" { "Role" }
                            th."px-4 py-2 border-l border-gray-200 bg-zinc-800" { "Replacements" }
                        }
                    }
                    tbody id="player-table-body" {
                        (format_table_row(TableRow {
                            name: "Player 1".to_string(),
                            alignment: "Town".to_string(),
                            role: "Cop".to_string(),
                            replacements: "None".to_string(),
                        }))
                        (format_table_row(TableRow {
                            name: "Player 2".to_string(),
                            alignment: "Mafia".to_string(),
                            role: "Goon".to_string(),
                            replacements: "Player 4".to_string(),
                        }))
                        (format_table_row(TableRow {
                            name: "Player 3".to_string(),
                            alignment: "Town".to_string(),
                            role: "Cop".to_string(),
                            replacements: "None".to_string(),
                        }))
                    }
                }
            }
        }
        .into_string(),
    )
}
