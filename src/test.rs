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
