use crate::scraping::scraper::{self, PageData};

pub struct ForumURL {
    thread_id: String,
    ppp: i32,
    start: i32,
}

pub enum URLType {
    Thread,
}

impl ForumURL {
    pub fn new(thread_id: String) -> ForumURL {
        ForumURL {
            thread_id: thread_id,
            ppp: 200,
            start: 0,
        }
    }

    pub fn new_from_post(post_id: String) -> ForumURL {
        ForumURL::new("thread_url".to_string())
    }

    pub fn url(&self, url_type: URLType) -> String {
        match url_type {
            URLType::Thread => format!(
                "https://forum.mafiascum.com/t/{}/?ppp={}&start={}",
                self.thread_id, self.ppp, self.start
            ),
        }
    }

    pub async fn scrape(&self) -> Option<PageData> {
        scraper::get_page_details(self.url(URLType::Thread)).await
    }
}
