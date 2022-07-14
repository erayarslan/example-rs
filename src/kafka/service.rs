use std::borrow::Cow;
use std::time::Duration;
use futures::{TryFutureExt, TryStreamExt};
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use tokio::spawn;
use tokio::task::spawn_blocking;
use crate::settings;

pub struct KafkaService {
    producer: FutureProducer,
    consumer: StreamConsumer,
}

impl KafkaService {
    pub fn new(base_config: &ClientConfig) -> KafkaService {
        let producer: FutureProducer = base_config.clone()
            .clone()
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        let consumer: StreamConsumer = base_config.clone()
            .set("group.id", &settings::SETTINGS.kafka.group_id)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "45000")
            .set("enable.auto.commit", "false")
            .create()
            .expect("Consumer creation failed");

        KafkaService { producer, consumer }
    }

    pub async fn produce<'a>(&self, key: &str, payload: &'a str) -> Result<bool, Cow<'a, str>> {
        let record = FutureRecord::to("2jr5id3d-default")
            .payload(payload)
            .key(key);

        let result = self.producer
            .send(record, Duration::from_secs(0))
            .await;

        match result {
            Ok(_) => Ok(true),
            Err((e, _)) => Err(Cow::from(e.to_string()))
        }
    }

    async fn record_borrowed_message_receipt(&self, msg: &BorrowedMessage<'_>) {
        println!("[Borrowed] Message received: {}", std::str::from_utf8(msg.payload().unwrap()).unwrap());
    }

    async fn record_owned_message_receipt(&self, msg: &OwnedMessage) {
        println!("[Owned] Message received: {}", std::str::from_utf8(msg.payload().unwrap()).unwrap());
    }

    fn expensive_computation<'a>(msg: OwnedMessage) -> Cow<'a, str> {
        match msg.payload_view::<str>() {
            Some(Ok(payload)) => Cow::from(format!("Payload len for {} is {}", payload, payload.len())),
            Some(Err(_)) => Cow::from("Message payload is not a string"),
            None => Cow::from("No payload"),
        }
    }

    pub async fn consume(&self, topic: &str) -> Result<(), KafkaError> {
        self.consumer
            .subscribe(&[topic])
            .expect("Can't subscribe to specified topic");

        self.consumer
            .stream()
            .try_for_each(|borrowed_message| async move {
                self.record_borrowed_message_receipt(&borrowed_message).await;
                let owned_message = borrowed_message.detach();
                self.record_owned_message_receipt(&owned_message).await;

                spawn
                    (async {
                        spawn_blocking(|| KafkaService::expensive_computation(owned_message))
                            .and_then(|data| async move {
                                println!("[Processed] Data: {}", data.to_string());
                                Ok(())
                            })
                            .map_err(|_| println!("Could not be processed"))
                            .await
                    });

                Ok(())
            })
            .await
    }
}