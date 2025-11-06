#[derive(Debug, serde::Deserialize)]
pub struct LineWebhookRequest {
    #[serde(rename = "destination")]
    pub destination: Option<String>,
    pub events: Vec<WebhookEvent>,
}

#[derive(Debug, serde::Deserialize)]
pub struct WebhookEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub message: Option<Message>,
    pub webhookEventId: String,
    pub deliveryContext: Option<DeliveryContext>,
    pub timestamp: i64,
    pub source: EventSource,
    #[serde(rename = "replyToken")]
    pub reply_token: Option<String>,
    pub mode: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Message {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub text: Option<String>,
    pub quoteToken: Option<String>,
    pub markAsReadToken: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EventSource {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeliveryContext {
    pub isRedelivery: bool,
}
