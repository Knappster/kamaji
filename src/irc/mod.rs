pub mod error;

use error::IrcError;
use tokio::task::JoinHandle;

use crate::{config::ConfigType, events::Event, state::StateType};

pub async fn irc_connect(config: ConfigType, state: StateType) -> Result<(), IrcError> {
    let config = config.lock().await.clone();

    // TODO: Use channel name from Twitch users API.
    let channel = "#".to_owned() + &config.irc_channel;

    tracing::info!("Joining channel: {}", channel.clone());

    let mut client = tmi::Client::anonymous().await?;
    client.join(channel.clone()).await?;

    let handle: JoinHandle<Result<(), IrcError>> = tokio::spawn(async move {
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
                    client.join(channel.clone()).await?;
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
