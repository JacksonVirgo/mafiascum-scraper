use crate::scraping::scraper::{self, PageData};

pub struct ForumURL {
    thread_id: String,
    ppp: i32,
    start: i32,
}

pub enum URLType {
    Thread,
    Post(String),
}

impl ForumURL {
    pub fn new(thread_id: String) -> ForumURL {
        ForumURL {
            thread_id: thread_id,
            ppp: 200,
            start: 0,
        }
    }

    pub async fn new_from_post(post_id: String) -> Option<ForumURL> {
        match scraper::get_page_details(format!(
            "https://forum.mafiascum.net/viewtopic.php?&p={}&ppp=1",
            post_id
        ))
        .await
        {
            Some(page) => Some(ForumURL::new(page.thread_id)),
            None => return None,
        }
    }

    pub fn url(&self, url_type: URLType) -> String {
        match url_type {
            URLType::Thread => format!(
                "https://forum.mafiascum.net/viewtopic.php?t={}&ppp={}&start={}",
                self.thread_id, self.ppp, self.start
            ),
            URLType::Post(post_id) => {
                format!("https://forum.mafiascum.net/viewtopic.php?&p={}", post_id)
            }
        }
    }

    pub async fn scrape(&self) -> Option<PageData> {
        scraper::get_page_details(self.url(URLType::Thread)).await
    }
}
