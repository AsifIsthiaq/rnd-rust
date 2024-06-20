mod test;
mod main1;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};
use std::fmt;

#[derive(Clone, Debug)]
enum ExitedBy {
    EndSessionBox,
    SendToQueueBox,
    Empty,
}

impl Serialize for ExitedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            ExitedBy::EndSessionBox => serializer.serialize_str("endSessionBox"),
            ExitedBy::SendToQueueBox => serializer.serialize_str("sendToQueueBox"),
            ExitedBy::Empty => serializer.serialize_str(""),
        }
    }
}

struct ExitedByVisitor;

impl<'de> Visitor<'de> for ExitedByVisitor {
    type Value = ExitedBy;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing an exited by value")
    }

    fn visit_str<E>(self, value: &str) -> Result<ExitedBy, E>
        where
            E: de::Error,
    {
        match value {
            "endSessionBox" => Ok(ExitedBy::EndSessionBox),
            "sendToQueueBox" => Ok(ExitedBy::SendToQueueBox),
            "" => Ok(ExitedBy::Empty),
            _ => Err(de::Error::unknown_variant(value, &["endSessionBox", "sendToQueueBox", ""])),
        }
    }
}

impl<'de> Deserialize<'de> for ExitedBy {
    fn deserialize<D>(deserializer: D) -> Result<ExitedBy, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ExitedByVisitor)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatsEvent {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "entityType")]
    pub entity_type: String,
    pub id: String,
    #[serde(rename = "timeInMsec")]
    pub time_in_m_sec: i64,
    pub event: String,
    #[serde(rename = "channelAccountId")]
    pub channel_account_id: String,
    #[serde(rename = "channelType")]
    pub channel_type: String,
    #[serde(rename = "queueId")]
    pub queue_id: String,
    #[serde(rename = "exitedBy")]
    pub exited_by: ExitedBy,
}

fn main() {
    let stats_event = StatsEvent {
        tenant_id: "tenant1".to_string(),
        thread_id: "thread1".to_string(),
        entity_type: "entity".to_string(),
        id: "id1".to_string(),
        time_in_m_sec: 1718376867236,
        event: "event".to_string(),
        channel_account_id: "account1".to_string(),
        channel_type: "type1".to_string(),
        queue_id: "queue1".to_string(),
        exited_by: ExitedBy::Empty,
    };

    // Serialize to JSON
    let serialized = serde_json::to_string(&stats_event).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize from JSON
    let deserialized: StatsEvent = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}


// use tokio::time::{sleep, Duration};
//
// async fn long_running_task() {
//     println!("Long running task started");
//     sleep(Duration::from_secs(5)).await; // Simulate a long task
//     println!("Long running task finished");
// }
//
// async fn main_function() {
//     println!("Main function started");
//
//     // Spawn the long-running task as a separate async task
//     tokio::spawn(async {
//         long_running_task().await;
//     });
//
//     println!("Main function continues executing without waiting for the task");
//
//     // Simulate some other work in the main function
//     sleep(Duration::from_secs(2)).await;
//     println!("Main function finished");
// }
//
// #[tokio::main]
// async fn main() {
//     main_function().await;
// }

//----

// use tokio::task;
//
// async fn async_task() {
//     println!("Async task is starting...");
//     // Simulate asynchronous operation (e.g., network request)
//     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
//     println!("Async task is done!");
// }
//
// #[tokio::main]
// async fn main() {
//     println!("Main thread is starting...");
//
//     // Spawn an asynchronous task
//     let t1 = task::spawn(async_task());
//
//     println!("Main thread is still running...");
//
//     // Wait for the spawned task to complete
//     t1.await.expect("Task panicked!");
//
//     println!("Main thread is ending...");
// }

//----

// use std::sync::Arc;
// use std::thread;
//
// fn main() {
//     // Create an `Arc` around a string
//     let message = Arc::new(String::from("Hello, world!"));
//
//     // Clone the `Arc` to share ownership with another thread
//     let message_clone = Arc::clone(&message);
//
//     // Spawn a new thread and move the cloned `Arc` into it
//     let handle = thread::spawn(move || {
//         println!("t{}", message_clone); // Output: Hello, world!
//     });
//
//     // Wait for the thread to finish
//     handle.join().unwrap();
//
//     // The original `Arc` can still be used in the main thread
//     println!("{}", message); // Output: Hello, world!
// }

//----

// use std::time::Duration;
// use tokio::time::sleep;
//
// async fn say_hello() {
//     println!("Hello");
//     sleep(Duration::from_secs(1)).await;
//     println!("World!");
// }
//
// #[tokio::main]
// async fn main() {
//     println!("Before");
//     say_hello().await;
//     println!("after")
// }
