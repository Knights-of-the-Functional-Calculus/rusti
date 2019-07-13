use env_logger;
use lapin_async as lapin;
use log::info;
use std::collections::HashMap;

use crate::lapin::{
  BasicProperties, Channel, Connection, ConnectionProperties, ConsumerSubscriber,
  message::Delivery,
  options::*,
  types::FieldTable,
};

#[derive(Clone,Debug)]
struct Subscriber {
  channel: Channel,
  callback: Fn(&str)
}

impl ConsumerSubscriber for Subscriber {
	fn new_delivery(&self, delivery: Delivery) {
		self.channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).as_error().expect("basic_ack");
		self.callback(str::from_utf8(buf));
	}
	fn drop_prefetched_messages(&self) {}
	fn cancel(&self) {}
}

env_logger::init();

let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
let conn = Connection::connect(&addr, ConnectionProperties::default()).wait().expect("connection error");
let publish_channel = conn.create_channel().wait().expect("create_channel");
let mut subcribe_channels = HashMap::new();

pub fn send_message<T>(queue_name: &str, payload: T)  {
	publish_channel.queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::default()).wait().expect("queue_declare");
	publish_channel.basic_publish("", queue_name, BasicPublishOptions::default(), payload.to_vec(), BasicProperties::default()).wait().expect("basic_publish");
}

pub fn attach_consumer<T>(queue_name: &str,  consumer_name: &str, callback: &Fn(T)) {
	let channel = match subcribe_channels.get(consumer_name) {
		Some(channel) => channel,
		None => conn.create_channel().wait().expect("create_channel");,
	}
	let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::default()).wait().expect("queue_declare");
	channel.basic_consume(&queue, consumer_name,BasicConsumeOptions::default(), FieldTable::default(), 
		Box::new(Subscriber { channel: channel_b.clone(), callback: callback })).wait().expect("basic_consume");
}