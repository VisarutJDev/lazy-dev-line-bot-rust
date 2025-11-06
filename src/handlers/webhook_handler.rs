use crate::{models::line_webhook::LineWebhookRequest, services::line_service};
use axum::Json;

pub async fn handle_webhook(Json(payload): Json<LineWebhookRequest>) {
    // Process each event in the webhook request
    for event in payload.events {
        if let Some(message) = event.message {
            if let Some(text) = message.text {
                // Handle the message based on its content
                line_service::process_message(text, event.reply_token).await;
            }
        }
    }
}
