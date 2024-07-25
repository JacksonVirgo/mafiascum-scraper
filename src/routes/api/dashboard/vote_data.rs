use crate::components::buttons::{gen_button, ButtonType, FormSubmitButton};
use actix_web::{get, HttpResponse, Responder};
use maud::{html, Markup};

struct TableRow {
    author: String,
    target: String,
    corrected_target: String,
    post_number: i32,
    validity: bool
}
fn format_table_row(row: TableRow) -> Markup {
    html!({
        tr."even:bg-zinc-600" {
            td."px-4 py-2" { (row.author) }
            td."px-4 py-2 border-l border-gray-200" { (row.target) }
            td."px-4 py-2 border-l border-gray-200" { (row.corrected_target) }
            td."px-4 py-2 border-l border-gray-200" { (row.post_number) }
            td."px-4 py-2 border-l border-gray-200" { (row.validity) }
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
                    (gen_button(ButtonType::FormSubmit(FormSubmitButton {
                        text: "Save".to_string(),
                    })))
                }
                table."min-w-full bg-zinc-700 text-white" {
                    thead {
                        tr {
                            th."px-4 py-2 border-gray-200 bg-zinc-800" { "Author" }
                            th."px-4 py-2 border-l border-gray-200 bg-zinc-800" { "Target" }
                            th."px-4 py-2 border-l border-gray-200 bg-zinc-800" { "Corrected Target" }
                            th."px-4 py-2 border-l border-gray-200 bg-zinc-800" { "Post Num" }
                            th."px-4 py-2 border-l border-gray-200 bg-zinc-800" { "Validity" }
                            
                        }
                    }
                    tbody id="player-table-body" {
                        (format_table_row(TableRow {
                            author: "Bob Smith".to_string(),
                            target: "Jaen Doe".to_string(),
                            corrected_target: "Jane Doe".to_string(),
                            post_number: 1,
                            validity: true
                        }))
                        (format_table_row(TableRow {
                            author: "Deadpool".to_string(),
                            target: "Spiderman".to_string(),
                            corrected_target: "Spider-Man".to_string(),
                            post_number: 2,
                            validity: false
                        }))
                    }
                }
            }
        }
        .into_string(),
    )
}
