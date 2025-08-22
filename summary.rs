pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for &str {
    fn summarize(&self) -> String {
        "Notícia".to_string()
    }
}

impl Summary for String {
    fn summarize(&self) -> String {
        "Notícia".to_string()
    }
}

pub struct News {
    title: String,
    content: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        self.title.clone() + " " + &self.content
    }
}

impl News {
    pub fn create_news(title: String, content: String) -> Self {
        News { title, content }
    }
}

pub fn notify(summary: &dyn Summary) {
    println!("{}", summary.summarize());
}

fn main() {
    let news = News::create_news("title".to_string(), "content".to_string());
    notify(&news);
}
