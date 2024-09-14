mod discord_logger;
use discord_logger::DiscordLogger;

#[tokio::main]
async fn main() {
    let logger = DiscordLogger::new(
        "https://discord.com/api/webhooks/1284413410217426984/zsCu-D-RRQf7VeVoEumHQ2sb8FHBZgCMxWuNhBWge9M6tRgeRDVOJuTEwSSRQFOV9foo".to_string(),
        "Mas Mas Pws".to_string(),
        false
    );

    logger.info(
        "Test Dek".to_string(),
        "This is a test info message".to_string(),
        None
    ).await.unwrap();
}