// A trait is like an interface
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// CLASS implements INTERFACE => impl TRAIT for STRUCT
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
fn main() {
    let tweet = Tweet {
        username: String::from("ElementG9"),
        content: String::from("ligma balls"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
    let news = NewsArticle {
        headline: String::from("Trump dies"),
        location: String::from("America"),
        author: String::from("The Rock"),
        content: String::from("Surprisingly, the president has died."),
    };
    println!("New article: {}", news.summarize());
}
