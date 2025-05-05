use tokio::task::JoinHandle;

use crate::{events::Event, state::StateType};

pub async fn start_twitch_irc(state: StateType) -> anyhow::Result<()> {
    let channel = "#kn4ppster";

    tracing::info!("Starting IRC connection.");

    let mut client = tmi::Client::anonymous().await?;
    client.join(channel).await?;

    tracing::info!("Joining channel: {}", channel);

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

                    let state = state.lock().await.clone();

                    if message.text().starts_with("!") {
                        state
                            .events
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
                    client.join(channel).await?;
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
