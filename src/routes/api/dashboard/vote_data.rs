use std::thread::current;

use crate::{ components::buttons::{gen_button, ButtonType, FormSubmitButton}, models::votes::{create_vote, get_votes, NewVote}, scraping::scraper::Vote, utils::{app_state::AppState, url::ForumURL}};
use actix_web::{get, post, web::{self, Data}, HttpResponse, Responder};
use maud::{html, Markup};

struct TableRow {
    author: String,
    target: String,
    corrected_target: Option<String>,
    post_number: i32,
    validity: Option<bool>
}
fn format_table_row(row: TableRow) -> Markup {
    html!({
        tr."even:bg-zinc-600" {
            td."px-4 py-2" { (row.author) }
            td."px-4 py-2 border-l border-gray-200" { (row.target) }
            td."px-4 py-2 border-l border-gray-200" { 
                (match row.corrected_target {
                    Some(corrected_target) => corrected_target,
                    None => "N/A".to_string()
                }) 
            }
            td."px-4 py-2 border-l border-gray-200" { (row.post_number) }
            td."px-4 py-2 border-l border-gray-200" { 
                (match row.validity {
                    Some(validity) => if validity { "Yes" } else { "No" }.to_string(),
                    None => "N/A".to_string()
                }) 
            }
        }
    })
}

#[get("/votes/{thread_id}")]
async fn vote_data(state: Data<AppState>, path: web::Path<String>) -> impl Responder {
    let thread_id = path.into_inner();

    let all_votes = match get_votes(&state, &thread_id).await {
        Some(votes) => votes,
        None => Vec::new()
    };

    HttpResponse::Ok().body(
        html! {
            div."w-full h-full flex flex-col p-4" id="vote-wrapper" {
                h1."text-3xl text-white font-bold pb-2" { "Player Data" }
                div."text-xl text-white pb-2" { "Enter the data for the players in the game" }
                form."flex flex-col pb-2 mb-2" hx-post=(format!("/api/dashboard/votes/{}", thread_id)) hx-target="#vote-wrapper" hx-swap="outerHTML" {
                    (gen_button(ButtonType::FormSubmit(FormSubmitButton {
                        text: "Scrape Votes".to_string(),
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
                        @if all_votes.is_empty() {
                            tr."even:bg-zinc-600" {
                                td."px-4 py-2" { "No votes found" }
                            }
                        } else {
                            @for vote in all_votes {
                                (format_table_row(TableRow {
                                    author: vote.author,
                                    target: vote.target,
                                    corrected_target: vote.target_correction,
                                    post_number: vote.post_number,
                                    validity: None
                                }))
                            }
                        }
                    }
                }
            }
        }
        .into_string(),
    )
}

#[post("/votes/{thread_id}")]
async fn scrape_votes(state: Data<AppState>, path: web::Path<String>) -> impl Responder {
    // TODO: Make this a polling process, rather than one endpoint that takes a long time

    let thread_id = path.into_inner();
    let mut url = ForumURL::new(thread_id.clone());


    let mut is_last_page = false;
    let mut current_page = 0;
    let mut last_page = 1;
    while current_page < last_page {
        match url.ppp(200).start(current_page * 200).scrape().await {
            Some(page) => {
                for vote in page.votes {
                    let vote_copy: Vote = vote.clone();
                    let pg = create_vote(&state, NewVote {
                        thread_id: thread_id.clone(),
                        author: vote.author,
                        target: vote.target,
                        target_correction: None,
                        post_number: vote.post_number
                    }).await;
        
                    match pg {
                        Some(_) => (),
                        None => {
                            println!("Failed to create vote: {:?}", vote_copy);
                        }
                    }
                }
                last_page = page.last_page;
            },
            None => {
                println!("Failed to get page data for page {:?}", current_page);
                is_last_page = true;
            }
        };

        println!("Scraped page {}/{}", current_page, last_page);
        current_page += 1;
    }

    HttpResponse::Found()
        .insert_header(("HX-Redirect", format!("/dashboard/{}?d=3", thread_id))).finish()
}