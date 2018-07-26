#[cfg(feature = "json")]
use serde_json;

#[cfg(feature = "protobuf")]
use protos::channel_message::{
    Event as ProtoEvent, Event_oneof_payload as ProtoEventPayload,
    PhoenixEvent as ProtoPhoenixEvent,
};

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum Event {
    Defined(PhoenixEvent),
    Custom(String),
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum PhoenixEvent {
    #[cfg_attr(feature = "json", serde(rename = "phx_join"))]
    Join,
    #[cfg_attr(feature = "json", serde(rename = "phx_close"))]
    Close,
    #[cfg_attr(feature = "json", serde(rename = "phx_reply"))]
    Reply,
    #[cfg_attr(feature = "json", serde(rename = "heartbeat"))]
    Heartbeat,
}

#[cfg(feature = "protobuf")]
impl Into<ProtoPhoenixEvent> for PhoenixEvent {
    fn into(self) -> ProtoPhoenixEvent {
        match self {
            PhoenixEvent::Close => ProtoPhoenixEvent::Close,
            PhoenixEvent::Heartbeat => ProtoPhoenixEvent::Heartbeat,
            PhoenixEvent::Join => ProtoPhoenixEvent::Join,
            PhoenixEvent::Reply => ProtoPhoenixEvent::Reply,
        }
    }
}

#[cfg(feature = "protobuf")]
impl Into<ProtoEvent> for Event {
    fn into(self) -> ProtoEvent {
        let mut evt = ProtoEvent::new();
        evt.payload = Some(match self {
            Event::Defined(phoenix_evt) => ProtoEventPayload::phoenix_event(phoenix_evt.into()),
            Event::Custom(custom_evt) => ProtoEventPayload::custom_event(custom_evt),
        });

        evt
    }
}

#[cfg(feature = "json")]
#[test]
fn test_event_serialization() {
    #[derive(Serialize, Deserialize)]
    struct Test {
        event: Event,
    }

    let t = Test {
        event: Event::Custom("blablabla".to_string()),
    };
    let val = serde_json::to_string(&t).unwrap();
    println!("{}", val);
    assert_eq!(val, "{\"event\":\"blablabla\"}");
}
