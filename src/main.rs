use rdkafka::Timestamp;

fn main() {
    let timestamp = Timestamp::now();
    println!("{timestamp:?}");
}
