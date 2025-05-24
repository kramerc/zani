use regex::Regex;
use scraper::{Html, Selector};
use std::time::Duration;

pub fn filtered_hostmask(hostmask: &str) -> String {
    if !hostmask.contains("!") {
        return hostmask.to_string();
    }

    hostmask.replace("~", "").split("!").collect::<Vec<&str>>()[1].to_lowercase().to_string()
}

pub fn parse_nick(hostmask: &str) -> String {
    hostmask.split("!").collect::<Vec<&str>>()[0].to_string()
}

/// Detect URLs in a message text
pub fn extract_urls(text: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let mut detected_domains = Vec::new();

    // Regex for HTTP/HTTPS URLs - simpler version first
    let http_regex = Regex::new(r"https?://[^\s]+").unwrap();
    for mat in http_regex.find_iter(text) {
        let mut url = mat.as_str().to_string();
        // Remove common trailing punctuation that shouldn't be part of URL
        let trailing_chars = ['.', ',', '!', '?', ';', ':', ')', ']', '}'];
        url = url.trim_end_matches(&trailing_chars[..]).to_string();

        // Extract domain from the URL to avoid duplicates later
        if let Some(domain_start) = url.find("://") {
            if let Some(domain_end) = url[domain_start + 3..].find('/') {
                let domain = &url[domain_start + 3..domain_start + 3 + domain_end];
                detected_domains.push(domain.to_string());
            } else {
                let domain = &url[domain_start + 3..];
                detected_domains.push(domain.to_string());
            }
        }

        urls.push(url);
    }

    // Regex for domain-like patterns (example.com, example.tld)
    let domain_regex = Regex::new(r"\b[a-zA-Z0-9][a-zA-Z0-9-]*\.[a-zA-Z]{2,}\b").unwrap();
    for mat in domain_regex.find_iter(text) {
        let domain = mat.as_str();
        // Skip if it's already captured as HTTP URL
        if !detected_domains.contains(&domain.to_string()) && !domain.starts_with("www.") {
            urls.push(format!("http://{}", domain));
        }
    }

    // Also capture www. domains without protocol
    let www_regex = Regex::new(r"\bwww\.[a-zA-Z0-9][a-zA-Z0-9-]*\.[a-zA-Z]{2,}\b").unwrap();
    for mat in www_regex.find_iter(text) {
        let domain = mat.as_str();
        if !detected_domains.contains(&domain.to_string()) {
            urls.push(format!("http://{}", domain));
        }
    }

    urls
}

/// Fetch the title of a webpage
pub async fn fetch_page_title(url: &str) -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (compatible; IRC Bot)")
        .redirect(reqwest::redirect::Policy::limited(3))
        .build()
        .ok()?;

    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                // Check content type to avoid fetching non-HTML content
                if let Some(content_type) = response.headers().get("content-type") {
                    if let Ok(content_type_str) = content_type.to_str() {
                        if !content_type_str.starts_with("text/html") {
                            return None;
                        }
                    }
                }

                if let Ok(body) = response.text().await {
                    return extract_title_from_html(&body);
                }
            }
        }
        Err(_) => return None,
    }

    None
}

/// Extract title from HTML content
fn extract_title_from_html(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let title_selector = Selector::parse("title").ok()?;

    if let Some(title_element) = document.select(&title_selector).next() {
        let title = title_element.text().collect::<String>()
            .trim()
            .replace('\n', " ")
            .replace('\r', " ")
            .replace('\t', " ");

        // Collapse multiple spaces into single spaces
        let title = title.split_whitespace().collect::<Vec<&str>>().join(" ");

        if !title.is_empty() && title.len() <= 300 { // Reasonable title length limit
            return Some(title);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_urls() {
        // Test HTTP/HTTPS URLs
        let text1 = "Check out https://example.com and http://test.org for more info";
        let urls1 = extract_urls(text1);
        assert!(urls1.contains(&"https://example.com".to_string()));
        assert!(urls1.contains(&"http://test.org".to_string()));

        // Test domain-only URLs
        let text2 = "Visit google.com or stackoverflow.com for help";
        let urls2 = extract_urls(text2);
        assert!(urls2.contains(&"http://google.com".to_string()));
        assert!(urls2.contains(&"http://stackoverflow.com".to_string()));

        // Test mixed URLs
        let text3 = "https://github.com is great, also check rust-lang.org";
        let urls3 = extract_urls(text3);
        assert!(urls3.contains(&"https://github.com".to_string()));
        assert!(urls3.contains(&"http://rust-lang.org".to_string()));

        // Test www URLs
        let text4 = "Go to www.example.com for more info";
        let urls4 = extract_urls(text4);
        assert!(urls4.contains(&"http://www.example.com".to_string()));

        // Test no URLs
        let text5 = "This is just regular text with no URLs";
        let urls5 = extract_urls(text5);
        assert_eq!(urls5.len(), 0);

        // Test URL with trailing punctuation
        let text6 = "Check https://example.com, it's great!";
        let urls6 = extract_urls(text6);
        assert!(urls6.contains(&"https://example.com".to_string()));
    }

    #[test]
    fn test_extract_title_from_html() {
        // Test basic HTML with title
        let html1 = r#"
            <html>
                <head>
                    <title>Example Page Title</title>
                </head>
                <body>Content</body>
            </html>
        "#;
        let title1 = extract_title_from_html(html1);
        assert_eq!(title1, Some("Example Page Title".to_string()));

        // Test HTML without title
        let html2 = r#"
            <html>
                <head>
                </head>
                <body>Content</body>
            </html>
        "#;
        let title2 = extract_title_from_html(html2);
        assert_eq!(title2, None);

        // Test HTML with empty title
        let html3 = r#"
            <html>
                <head>
                    <title></title>
                </head>
                <body>Content</body>
            </html>
        "#;
        let title3 = extract_title_from_html(html3);
        assert_eq!(title3, None);

        // Test HTML with whitespace-only title
        let html4 = r#"
            <html>
                <head>
                    <title>   </title>
                </head>
                <body>Content</body>
            </html>
        "#;
        let title4 = extract_title_from_html(html4);
        assert_eq!(title4, None);
    }
}
