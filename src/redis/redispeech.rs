use redis::{Commands, Connection, RedisResult};

const AUDIO_CHANNEL_GLOB: String = "audio-*";
const INTERPRETED_AUDIO_CHANNEL_GLOB = "interpreted-audio-*";



fn send_audio_buffer(con: &mut Connection, , buffer: &[i16]) -> RedisResult {
	con.publish()
}