mod advanced;

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    author: String,
    headline: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("News article by {}! \"{}\"", self.author, self.headline)
    }
}

struct Tweet {
    author: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let content = match self.content.len() {
            x if x > 32 => format!("{}...", &self.content[..32]),
            _ => self.content.clone(),
        };
        format!("Tweet by {}: {}", self.author, content)
    }
}

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        author: "Circuit".to_string(),
        content: "Hello, world! ".repeat(100),
    };
    let news = NewsArticle {
        author: "Circuit".to_string(),
        headline: "Hello, world!".to_string(),
    };

    println!("{}", tweet.summarize());
    println!("{}", news.summarize());
}
