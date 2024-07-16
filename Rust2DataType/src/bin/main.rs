use std::collections::HashMap;

use Rust2DataType::DataType::integer;
fn main() {
    println!("Hello, world!");
    integer::println_interger();
    integer::println_interger();

    let rss_feeds = HashMap::from([
        (
            "Hacker News".to_string(),
            "https://news.ycombinator.com/rss".to_string(),
        ),
        (
            "TechCrunch".to_string(),
            "https://techcrunch.com/feed/".to_string(),
        ),
    ]);
}
