use event::Event;
#[cfg(feature = "protobuf")]
use protos::channel_message::ChannelMessage as ProtoChannelMessage;
#[cfg(feature = "json")]
use serde_json::Value;

#[cfg(feature = "json")]
pub type Payload = Value;

#[cfg(feature = "protobuf")]
pub type Payload = Vec<u8>;

#[cfg(feature = "json")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub topic: String,
    pub event: Event,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    pub join_ref: Option<String>,
    pub payload: Payload,
}

#[cfg(feature = "protobuf")]
pub struct Message {
    pub topic: String,
    pub event: Event,
    pub reference: Option<String>,
    pub join_ref: Option<String>,
    pub payload: Payload,
}

#[cfg(feature = "protobuf")]
impl Into<ProtoChannelMessage> for Message {
    fn into(self) -> ProtoChannelMessage {
        let Message {
            topic,
            event,
            reference,
            join_ref,
            payload,
        } = self;

        let mut msg = ProtoChannelMessage::new();
        msg.set_topic(topic);
        msg.set_event(event.into());
        if let Some(reference) = reference {
            msg.set_field_ref(reference);
        }
        if let Some(join_ref) = join_ref {
            msg.set_join_ref(join_ref);
        }
        msg.set_payload(payload.into());

        msg
    }
}
