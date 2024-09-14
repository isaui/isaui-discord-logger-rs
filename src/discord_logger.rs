use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use chrono::{DateTime, Utc};
use chrono_tz::Asia::Jakarta;

#[derive(Debug)]
pub struct LogOptions {
    title: String,
    description: String,
    fields: Option<Vec<Field>>,
    color: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct Field {
    name: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline: Option<bool>,
}

pub struct DiscordLogger {
    webhook_url: String,
    footer_text: String,
    is_production: bool,
}

impl DiscordLogger {
    pub fn new(webhook_url: String, footer_text: String, is_production: bool) -> Self {
        Self {
            webhook_url,
            footer_text,
            is_production,
        }
    }

    fn get_environment_prefix(&self) -> &str {
        if self.is_production {
            "🚀"
        } else {
            "🛠️"
        }
    }

    fn get_formatted_timestamp(&self) -> String {
        let now: DateTime<Utc> = Utc::now();
        let jakarta_time = now.with_timezone(&Jakarta);
        jakarta_time.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub async fn log(&self, options: LogOptions) -> Result<(), Box<dyn std::error::Error>> {
        if self.webhook_url.is_empty() {
            eprintln!("Discord webhook URL is not set");
            return Ok(());
        }

        let embed = json!({
            "title": format!("{} {}", self.get_environment_prefix(), options.title),
            "description": options.description,
            "color": options.color,
            "fields": options.fields.unwrap_or_default(),
            "timestamp": Utc::now().to_rfc3339(),
            "footer": {
                "text": format!("{} | {}", self.footer_text, self.get_formatted_timestamp())
            }
        });

        let client = Client::new();
        let res = client
            .post(&self.webhook_url)
            .json(&json!({ "embeds": [embed] }))
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!("HTTP error! status: {}", res.status()).into());
        }

        println!("Message sent to Discord: {}", options.title);
        Ok(())
    }

    pub async fn info(&self, title: String, description: String, fields: Option<Vec<Field>>) -> Result<(), Box<dyn std::error::Error>> {
        self.log(LogOptions {
            title,
            description,
            fields,
            color: Some(3447003), // Blue color
        }).await
    }

    pub async fn success(&self, title: String, description: String, fields: Option<Vec<Field>>) -> Result<(), Box<dyn std::error::Error>> {
        self.log(LogOptions {
            title,
            description,
            fields,
            color: Some(3066993), // Green color
        }).await
    }

    pub async fn error(&self, title: String, description: String, fields: Option<Vec<Field>>) -> Result<(), Box<dyn std::error::Error>> {
        self.log(LogOptions {
            title,
            description,
            fields,
            color: Some(15158332), // Red color
        }).await
    }
}