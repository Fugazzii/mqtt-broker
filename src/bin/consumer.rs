use mqtt_broker::{consumer::Consumer, utils::handle_args, traits::topic::ConsumeTopic};

const CONSUMER_ARGS_LEN: u8 = 2;

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let parsed_args = handle_args(&args, CONSUMER_ARGS_LEN);

    let mut socket = Consumer::connect("127.0.0.1:3000").await;

    println!("Consuming...");    

    let topic = &parsed_args[0];
    let response = socket.consume(topic).await;

    println!("Consumed message: {}", response.expect("No message"));
}