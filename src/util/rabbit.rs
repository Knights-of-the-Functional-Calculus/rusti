use std::sync::mpsc;
use std::sync::mpsc::SyncSender;

use std::collections::HashMap;

use lapin::{
    message::Delivery, options::*, types::FieldTable, BasicProperties, Channel, Connection,
    ConnectionProperties, ConsumerSubscriber,
};
use lapin_async as lapin;

use std::fmt;

struct Subscriber {
    channel: Channel,
    sync_sender: SyncSender<Vec<u8>>,
}

impl fmt::Debug for Subscriber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.channel)
    }
}

impl ConsumerSubscriber for Subscriber {
    fn new_delivery(&self, delivery: Delivery) {
        self.channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .as_error()
            .expect("basic_ack");
        if let Ok(_) = self.sync_sender.send(delivery.data) {
            println!("Data sent");
        }
    }
    fn drop_prefetched_messages(&self) {}
    fn cancel(&self) {}
}

pub fn send_message(publish_channel: &Channel, queue_name: &str, payload: &[u8]) {
    publish_channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .wait()
        .expect("queue_declare");
    publish_channel
        .basic_publish(
            "",
            queue_name,
            BasicPublishOptions::default(),
            payload.to_vec(),
            BasicProperties::default(),
        )
        .wait()
        .expect("basic_publish");
    println!("Payload sent to {}", queue_name);
}

pub fn attach_consumer(
    queue_name: &str,
    consumer_name: &'static str,
    conn: Connection,
    subcribe_channels: &mut HashMap<&'static str, Channel>,
    sync_sender: &SyncSender<Vec<u8>>,
) {
    let channel: &Channel = subcribe_channels
        .entry(consumer_name)
        .or_insert(conn.create_channel().wait().expect("create_channel"));

    let queue = channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .wait()
        .expect("queue_declare");
    channel
        .basic_consume(
            &queue,
            consumer_name,
            BasicConsumeOptions::default(),
            FieldTable::default(),
            Box::new(Subscriber {
                channel: channel.clone(),
                sync_sender: sync_sender.clone(),
            }),
        )
        .wait()
        .expect("basic_consume");
    println!("Consumer attached to {}", queue_name);
}