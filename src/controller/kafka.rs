use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{BaseConsumer, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::{ClientConfig, TopicPartitionList, ClientContext};
use std::thread;
use std::time::Duration;
use std::{io::Error, io::ErrorKind};

pub struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
	fn pre_rebalance(&self, consumer: &BaseConsumer<Self>, rebalance: &Rebalance) {
		match consumer.assignment() {
			Ok(assignment) => {
				log::info!("Consumer {:?}, Pre rebalance {:?} successful", rebalance, assignment);
			},
			Err(e) => {
				log::error!("Error during pre rebalance assignment: {}", e);
			},
		}
	}

	fn post_rebalance(&self, consumer: &BaseConsumer<Self>, rebalance: &Rebalance) {
		match consumer.assignment() {
			Ok(assignment) => {
				log::info!("Consumer {:?}, Post rebalance {:?} successful", rebalance, assignment);
			},
			Err(e) => {
				log::error!("Error during post rebalance assignment: {}", e);
			},
		}
	}

	fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
		match result {
			Ok(_) => {
				log::info!("Offsets committed successfully: {:?}", _offsets)
			},
			Err(e) => {
				log::error!("Error committing offsets: {}", e)
			},
		}
	}
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

pub fn create_consumer(
	brokers: &str,
	group_id: &str,
	topics_list: &[&str]
) -> Result<LoggingConsumer, Error> {
	let context: CustomContext = CustomContext;

	let client_config: ClientConfig = {
		let mut config: ClientConfig = ClientConfig::new();
		config
			.set("group.id", group_id)
			.set("bootstrap.servers", brokers)
			.set("enable.partition.eof", "false")
			// TODO: Add support to dynamic SSL
			// .set("security.protocol", "ssl")
			// .set("ssl.ca.location", "/path/to/ca.pem")
			.set("session.timeout.ms", "6000")
			.set("enable.auto.commit", "true")
			.set("message.max.bytes", "1000000000")
			.set("message.copy.max.bytes", "1000000000")
			.set("receive.message.max.bytes", "2147483647")
			.set("socket.send.buffer.bytes", "100000000")
			.set("socket.receive.buffer.bytes", "100000000")
			.set("queued.max.messages.kbytes", "2097151")
			.set("fetch.message.max.bytes", "1000000000")
			.set("max.partition.fetch.bytes", "1000000000")
			.set("max.poll.interval.ms", "86400000")
			.set("fetch.max.bytes", "2147483135")
			.set("auto.offset.reset", "earliest")
			.set_log_level(RDKafkaLogLevel::Debug)
		;
		config
	};

	let consumer: LoggingConsumer = match client_config.create_with_context(context) {
		Ok(consumer) => {
			log::info!("Successfully connected to Kafka brokers: {}", brokers);
			consumer
		}
		Err(e) => {
			log::error!("Failed to create Kafka consumer: {}", e);
			return Err(Error::new(ErrorKind::Other, format!("Kafka error: {}", e)));
		}
	};

	loop {
		match consumer.subscribe(&topics_list) {
			Ok(()) => {
				log::info!("Subscribed to kafka topics successfully: {:?}", topics_list.join(" "));
				break
			},
			Err(e) => {
				log::warn!("Can't subscribe to kafka specified topics '{:?}': {}", topics_list, e);
			},
		}

		let waiting_time: Duration = Duration::from_secs(5);
		thread::sleep(waiting_time);
	}

	Ok(consumer)
}


// pub fn create_consumer_with_config(
// 	brokers: &str,
// 	group_id: &str,
// 	topics: &[&str],
// 	config_overrides: &[(&str, &str)],
// ) -> Result<LoggingConsumer, Error> {
//     let mut consumer = create_consumer(brokers, group_id, topics)?;
// 	for (key, value) in config_overrides {
// 		match consumer.set_config_option(key, value) {
// 			Ok(_) => log::info!("Set Kafka consumer config option: {} = {}", key, value),
// 			Err(e) => log::warn!("Failed to set Kafka consumer config option {}: {}", key, e),
// 		}
// 	}

// 	Ok(consumer)
// }
