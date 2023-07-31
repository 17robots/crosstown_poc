use events::UserCreatedEventMessage;
use crosstown_bus::CrosstownBus;

fn main() {
    let mut publisher = CrosstownBus::new_queue_publisher("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    _ = publisher.publish_event("user_created".to_owned(), UserCreatedEventMessage { user_id: "1".to_owned(), user_name: "Bob".to_owned() });
}
