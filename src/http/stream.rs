use std::pin::Pin;

use futures::{Stream, StreamExt};
use reqwest_eventsource::{Event, EventSource};
use serde::de::DeserializeOwned;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::error::Error;

pub async fn stream<O: DeserializeOwned + Send + 'static>(
    mut event_source: EventSource,
    stream_done_message: &'static str,
) -> Result<Pin<Box<dyn Stream<Item = Result<O, Error>> + Send>>, Error> {
    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(event) = event_source.next().await {
            match event {
                Err(e) => {
                    if let Err(_) = tx.send(Err(Error::Stream(e.to_string()))) {
                        break;
                    }
                }
                Ok(event) => match event {
                    Event::Open => continue,
                    Event::Message(event) => {
                        if event.data == stream_done_message {
                            break;
                        }

                        let output: Result<O, Error> =
                            serde_json::from_str::<O>(&event.data).map_err(|e| e.into());
                        if let Err(_) = tx.send(output) {
                            break;
                        }
                    }
                },
            }
        }
        event_source.close();
    });

    Ok(Box::pin(UnboundedReceiverStream::new(rx)))
}
