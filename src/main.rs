use reqwest::Error;
use scraper::{Html, Selector};
use tokio::task;

#[derive(Debug)]
struct ParseData {
    titles: Vec<String>,
    links: Vec<String>,
}

async fn fetch_html(url: String) -> Result<String, Error> {
    let response = reqwest::get(&url).await?;
    return response.text().await;
}

fn parse_html(html: &str) -> ParseData {
    let document = Html::parse_document(html);
    let title_selsctor = Selector::parse("h1, h2, h3").unwrap();
    let links_selsctor = Selector::parse("a").unwrap();

    let title = document
        .select(&title_selsctor)
        .map(|element| element.text().collect::<Vec<_>>().join(" "))
        .collect();

    let links = document
        .select(&links_selsctor)
        .filter_map(|element| element.value().attr("href"))
        .map(String::from)
        .collect();

    return ParseData { titles: title, links: links };
}

#[tokio::main]
fn main() {
    println!("Hello, world!");
}
