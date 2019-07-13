use env_logger;
use failure::Error;
use futures::future;
use futures::future::Future;
use lapin_futures as lapin;
use crate::lapin::{BasicProperties, Client, ConnectionProperties};
use crate::lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use crate::lapin::types::FieldTable;
use log::info;
use tokio;
use tokio::runtime::Runtime;

fn main() {
  env_logger::init();

  let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

  Runtime::new().unwrap().block_on_all(
   Client::connect(&addr, ConnectionProperties::default()).map_err(Error::from).and_then(|client| {
      // create_channel returns a future that is resolved
      // once the channel is successfully created
      client.create_channel().map_err(Error::from)
    }).and_then(|mut channel| {
      let id = channel.id();
      info!("created channel with id: {}", id);

      // we using a "move" closure to reuse the channel
      // once the queue is declared. We could also clone
      // the channel
      channel.queue_declare("hello", QueueDeclareOptions::default(), FieldTable::default()).and_then(move |_| {
        info!("channel {} declared queue {}", id, "hello");

        channel.basic_publish("", "hello", b"hello from tokio".to_vec(), BasicPublishOptions::default(), BasicProperties::default())
      }).map_err(Error::from)
    })
  ).expect("runtime failure");
}

const AUDIO_CHANNEL_GLOB: &str = "audio-{}";
const INTERPRETED_AUDIO_CHANNEL_GLOB: &str = "interpreted-audio-*";
const HANDLER_TIMEOUT_SECONDS: i32 = 5;

let client = redis::Client::open(format!("amqp://{}:{}", env!("AMQP_ADDR").expect("rabbitmq host"), env!("AMQP_PORT").expect("rabbitmq port")))?;
let mut con = client.get_connection()?;
let mut pubsub = con.as_pubsub();

pubsub.psubscribe(INTERPRETED_AUDIO_CHANNEL_GLOB)?;


pub fn send_audio_buffer(source: &str, callback: &Fn(RedisResult<Msg>), buffer: &[i16]) -> RedisResult {
	let channel_name = format!(AUDIO_CHANNEL_GLOB, source);
	con.publish(channel_name, buffer);
	handle_message(channel_name, callback);
}

async fn handle_message(channel: &str, callback: &Fn(RedisResult<Msg>)) -> i32 {
	let mut msg = pubsub.get_message()?;
	let now = Instant::now();
	while now.elapsed().as_secs() < HANDLER_TIMEOUT_SECONDS {
		msg = pubsub.get_message()?;
	}
	callback()
}