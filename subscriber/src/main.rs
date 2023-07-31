use events::UserCreatedEventMessage;
use borsh::{BorshSerialize, BorshDeserialize};
use crosstown_bus::{MessageHandler, CrosstownBus, HandleError, QueueProperties};

struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> 
            where events::UserCreatedEventMessage: Clone + BorshDeserialize + BorshSerialize + 'static {
        println!("Message received on handler 1 {:?}", message);
        Ok(())
    }
}

fn main() {
    let listener = CrosstownBus::new_queue_listener("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    _ = listener.listen("user_created".to_owned(), UserCreatedHandler{}, QueueProperties { auto_delete: false, durable: false, use_dead_letter: true, });
    loop{}
}
