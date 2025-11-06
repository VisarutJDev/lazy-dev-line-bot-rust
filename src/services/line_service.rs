use reqwest::Client;
use serde_json::json;

const LINE_API_URL: &str = "https://api.line.me/v2/bot/message/reply";

pub async fn process_message(text: String, reply_token: Option<String>) {
    if let Some(token) = reply_token {
        // Get the channel access token from environment variable
        let channel_access_token = std::env::var("LINE_CHANNEL_ACCESS_TOKEN")
            .expect("LINE_CHANNEL_ACCESS_TOKEN must be set");
        // Create a response message based on the received text
        let response = create_response_message(&text);
        // Send the response back to LINE
        send_reply(token, response, &channel_access_token).await;
    }
}

async fn send_reply(reply_token: String, message: serde_json::Value, channel_access_token: &str) {
    let client = Client::new();

    let payload = json!({
        "replyToken": reply_token,
        "messages": [message]
    });
    // Send the request and handle both network errors and non-2xx responses.
    match client
        .post(LINE_API_URL)
        .header("Authorization", format!("Bearer {}", channel_access_token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) => {
            // let status = resp.status();
            // if status.is_success() {
            //     // Successful reply
            //     println!("LINE reply sent successfully (status: {})", status);
            // } else {
            //     // Non-success status: try to read the response body for diagnostics
            //     match resp.text().await {
            //         Ok(body) => eprintln!(
            //             "Failed to send LINE reply (status: {}). Response body: {}",
            //             status, body
            //         ),
            //         Err(e) => eprintln!(
            //             "Failed to send LINE reply (status: {}). Also failed to read body: {}",
            //             status, e
            //         ),
            //     }
            // }
        }
        Err(err) => {
            // Network / request building error
            eprintln!("Error sending LINE reply: {}", err);
        }
    }
}

fn create_response_message(received_text: &str) -> serde_json::Value {
    use crate::models::line_messages;

    match received_text.to_lowercase().as_str() {
        "port" => line_messages::get_portfolio_message(),
        "resume" | "cv" => line_messages::get_resume_cv_message(),
        "github" => line_messages::get_github_message(),
        "medium" => line_messages::get_blog_carousel_message(),
        "hello" => line_messages::get_simple_message(),
        _ => json!({
            "type": "text",
            "text": format!("You said: {}, Line message ของ lazy-dev ในรองรับเพียงคำสั่ง \n
        portfolio, medium, resume, cv, github เท่านั้น เพราะ lazy", received_text)
        }),
    }
}
