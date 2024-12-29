use std::collections::HashSet;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use clap::Parser;
use futures_util::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use scraper::{Html, Selector};

/// Command-line arguments structure
#[derive(Parser)]
struct Args {
    /// Base URL of the archived website
    base_url: String,
    /// Output directory to save the posts
    output_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();

    // Create HTTP client
    let client = reqwest::Client::builder()
        .user_agent("blog_archiver/1.0")
        .build()?;

    // Fetch and parse the base URL
    let base_html = fetch_html(&client, &args.base_url).await?;
    let post_links = extract_post_links(&base_html)?;
    println!("Found {} post links", post_links.len());
    println!("Post links: {:?}", post_links);

    // Ensure the output directory exists
    create_dir_all(&args.output_dir)?;

    // Initialize the progress bar
    let pb = ProgressBar::new(100);

    // Set the progress bar style
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
            .progress_chars("#>-"),
    );

    // Process posts with limited concurrency
    let fetches = post_links.into_iter().map(|post_url| {
        let client = client.clone();
        let output_dir = args.output_dir.clone();
        let pb = pb.clone();
        tokio::spawn(async move {
            if let Err(e) = process_post(&client, &post_url, &output_dir).await {
                eprintln!("Error processing {}: {}", post_url, e);
            }
            pb.inc(1);
        })
    });

    // Await all tasks
    join_all(fetches).await;

    // Finish the progress bar
    pb.finish_with_message("Processing complete");

    Ok(())
}

/// Fetches the HTML content of a given URL
async fn fetch_html(client: &reqwest::Client, url: &str) -> Result<String, reqwest::Error> {
    let response = client.get(url).send().await?;
    response.text().await
}

async fn process_post(client: &reqwest::Client, post_url: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let post_html = fetch_html(client, post_url).await?;
    if let Some((title, content)) = extract_post_content(&post_html) {
        let filename = format_filename(post_url);
        let filepath = Path::new(output_dir).join(filename);
        save_as_markdown(&filepath, &title, &content)?;
    }
    Ok(())
}

/// Extracts post links from the base HTML by composing potential post URLs
/// and verifying their presence in the base HTML.
fn extract_post_links(html: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let mut unique_links = HashSet::new();

    // Define a regex pattern to match URLs with a date and a title
    let re = Regex::new(r"https://web\.archive\.org/web/\d+/http://angaatopzoek\.be/\d{4}/\d{2}/\d{2}/[^/]+/$")?;

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            // Check if the href matches the desired pattern
            if re.is_match(href) {
                // Further filter out URLs containing a fragment identifier
                if !href.contains('#') {
                    unique_links.insert(href.to_string());
                }
            }
        }
    }

    Ok(unique_links.into_iter().collect())
}

/// Extracts the content from a post's HTML
fn extract_post_content(html: &str) -> Option<(String, String)> {
    let document = Html::parse_document(html);

    // Extract the title from the first <h4> tag (if present)
    let title = document
        .select(&Selector::parse("h4").ok()?)
        .next()
        .map(|node| node.text().collect::<Vec<_>>().join(" "))
        .unwrap_or_else(|| "Untitled".to_string());

    // Extract content in the natural order of tags
    let mut content_parts = Vec::new();
    for node in document.select(&Selector::parse("h4, p, cite").unwrap()) {
        content_parts.push(node.text().collect::<Vec<_>>().join(" "));
    }

    // Remove duplicate title if it was extracted from the first <h4>
    if let Some(first_content) = content_parts.first() {
        if *first_content == title {
            content_parts.remove(0);
        }
    }

    let content = content_parts.join("\n\n");

    Some((title, content))
}

fn format_filename(url: &str) -> String {
    // Extract the path after the domain
    if let Some(path_start) = url.find("angaatopzoek.be") {
        let path = &url[path_start + "angaatopzoek.be".len()..];
        let sanitized_path = path
            .trim_matches('/')
            .replace('/', "_")
            .replace('-', "_");

        if sanitized_path.is_empty() {
            "default_post.md".to_string()
        } else {
            format!("{}.md", sanitized_path)
        }
    } else {
        // Fallback in case the URL doesn't match the expected format
        "unknown_post.md".to_string()
    }
}

/// Saves the content as a Markdown file
fn save_as_markdown(path: &Path, title: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    // Write the title
    if !title.is_empty() {
        writeln!(file, "# {}
", title)?;
    }

    // Write the content
    writeln!(file, "{}", content)?;

    Ok(())
}
