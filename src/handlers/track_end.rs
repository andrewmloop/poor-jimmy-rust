use std::{sync::Arc, time::Duration};

use serenity::{async_trait, http::Http, model::prelude::ChannelId, prelude::Mutex, utils::Color};

use songbird::{Call, Event, EventContext, EventHandler as VoiceEventHandler};
use tokio::time::sleep;

pub struct TrackEndNotifier {
    pub channel_id: ChannelId,
    pub http: Arc<Http>,
    pub call: Arc<Mutex<Call>>,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        // Continue only if this is a Track event
        let EventContext::Track(_) = ctx else {
            return None;
        };

        // Attempt to grab the next song that will be playing
        let handler = self.call.lock().await;
        let queue = handler.queue().current_queue();
        let next_song = queue.first();

        drop(handler);

        // Artificial delay added here before sending message notifying
        // of the next song to play. Often times, this message is sent before
        // the response from other commands making the messages appear
        // out of order. This is a quick/dirty fix for that.
        sleep(Duration::from_secs(2)).await;

        match next_song {
            // A song was found, notify that it will be playing next
            Some(song) => {
                let title = &song.metadata().title;

                match title {
                    Some(title) => {
                        let _ = self
                            .channel_id
                            .send_message(&self.http, |message| {
                                message.add_embed(|embed| {
                                    embed
                                        .description(format!("**Now playing:** {}!", title))
                                        .color(Color::DARK_GREEN)
                                })
                            })
                            .await;
                    }
                    None => {
                        let _ = self
                            .channel_id
                            .send_message(&self.http, |message| {
                                message.add_embed(|embed| {
                                    embed
                                        .description("**Now playing:** Mystery song!")
                                        .color(Color::DARK_GREEN)
                                })
                            })
                            .await;
                    }
                }
            }
            // No song was picked up, the queue is most likely done
            None => {
                let _ = self
                    .channel_id
                    .send_message(&self.http, |message| {
                        message.add_embed(|embed| {
                            embed
                                .description("Queue has **ended!**")
                                .color(Color::DARK_GREEN)
                        })
                    })
                    .await;
            }
        }

        None
    }
}
