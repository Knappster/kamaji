pub mod error;

use tokio::task::JoinHandle;

use crate::{events::Event, state::State};
use error::IrcError;

pub async fn irc_connect(state: State) -> Result<(), IrcError> {
    let config = state.config.clone();

    // TODO: Use channel name from Twitch users API.
    let channel = "#".to_owned() + &config.irc_channel;

    tracing::info!("joining channel: {}", channel.clone());

    let mut client = tmi::Client::anonymous().await?;
    client.join(channel.clone()).await?;

    let handle: JoinHandle<Result<(), IrcError>> = tokio::spawn(async move {
        loop {
            let message = client.recv().await?;
            match message.as_typed()? {
                tmi::Message::Privmsg(message) => {
                    tracing::info!(
                        "message ({}): {} - {}",
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
                    tracing::info!("reconnecting");
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
