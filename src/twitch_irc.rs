use tokio::task::JoinHandle;

use crate::{events::Event, state::AppState};

pub async fn twitch_chat(state: AppState) -> anyhow::Result<()> {
    const CHANNELS: &[&str] = &["#kn4ppster"];

    let mut client = tmi::Client::anonymous().await?;
    client.join_all(CHANNELS).await?;

    let handle: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        loop {
            let message = client.recv().await?;
            match message.as_typed()? {
                tmi::Message::Privmsg(message) => {
                    tracing::info!(
                        "Message ({}): {} - {}",
                        message.channel(),
                        message.sender().name(),
                        message.text()
                    );

                    if message.text().starts_with("!") {
                        state
                            .events
                            .lock()
                            .await
                            .publish(Event {
                                event_type: "chat.command".to_string(),
                                payload: serde_json::json!({"message": message.text()}),
                            })
                            .await;
                    }
                }
                tmi::Message::Reconnect => {
                    tracing::info!("Reconnecting!");
                    client.reconnect().await?;
                    client.join_all(CHANNELS).await?;
                }
                tmi::Message::Ping(ping) => {
                    client.pong(&ping).await?;
                }
                _ => {}
            }
        }
    });

    let result = handle.await?;
    result?;

    Ok(())
}
