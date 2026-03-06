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

async fn process_urls(urls: Vec<String>) {
    let mut handels = Vec::new();

    for url in urls {
        let handle = task::spawn(async move {
            match fetch_html(url.clone()).await {
                Ok(html) => {
                    let data = parse_html(&html);
                    println!("\n\nParsed Data: {:?}", data);
                }
                Err(e) => {
                    println!("Failed to fetch URL: {}", e)
                }
            }
        });
        handels.push(handle);
    }

    for handle in handels {
        handle.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "http://google.com".to_string(),
        "http://youtube.com".to_string(),
    ];

    process_urls(urls).await;
}
