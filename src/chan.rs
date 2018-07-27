#[cfg(feature = "json")]
use serde_json;
#[cfg(feature = "json")]
use serde_json::Value;
use std::error::Error;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::Sender as ChannelSender;

#[cfg(feature = "protobuf")]
use protobuf::Message as ProtoMessage;
#[cfg(not(target_arch = "wasm32"))]
use websocket::OwnedMessage;

use event::{Event, PhoenixEvent};
use message::Message;
#[cfg(feature = "protobuf")]
use protos::channel_message::ChannelMessage as ProtoChannelMessage;

#[cfg(not(target_arch = "wasm32"))]
type ChannelMessage = OwnedMessage;

#[cfg(all(target_arch = "wasm32", feature = "json"))]
type ChannelMessage = String;

#[cfg(all(target_arch = "wasm32", feature = "protobuf"))]
type ChannelMessage = Vec<u8>;

#[cfg(not(target_arch = "wasm32"))]
type Sender = ChannelSender<ChannelMessage>;

#[cfg(target_arch = "wasm32")]
pub struct FauxSender<T>(fn(T) -> ());

#[cfg(target_arch = "wasm32")]
impl<T> FauxSender<T> {
    pub fn send(&self, msg: T) -> Result<(), !> {
        Ok((self.0)(msg))
    }
}

#[cfg(target_arch = "wasm32")]
type Sender = FauxSender<ChannelMessage>;

pub struct Channel {
    topic: String,
    tx: Sender,
    reference: String,
}

#[cfg(all(not(target_arch = "wasm32"), feature = "json"))]
fn msg_to_channel_msg(msg: Message) -> Result<ChannelMessage, Box<Error>> {
    Ok(OwnedMessage::Text(serde_json::to_string(&msg)?))
}

#[cfg(all(target_arch = "wasm32", feature = "json"))]
fn msg_to_channel_msg(msg: Message) -> Result<ChannelMessage, Box<Error>> {
    serde_json::to_string(&msg)?
}

#[cfg(all(target_arch = "wasm32", feature = "protobuf"))]
fn msg_to_channel_msg(msg: Message) -> Result<ChannelMessage, Box<Error>> {
    let proto_msg: ProtoChannelMessage = msg.into();
    Ok(proto_msg.write_to_bytes()?)
}

#[cfg(all(not(target_arch = "wasm32"), feature = "protobuf"))]
fn msg_to_channel_msg(msg: Message) -> Result<ChannelMessage, Box<Error>> {
    let proto_msg: ProtoChannelMessage = msg.into();
    Ok(OwnedMessage::Binary(proto_msg.write_to_bytes()?))
}

#[cfg(feature = "protobuf")]
fn null_value() -> Vec<u8> {
    Vec::new()
}

#[cfg(feature = "json")]
fn null_value() -> Value {
    Value::Null
}

impl Channel {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(topic: &str, tx: Sender, reference: &str) -> Self {
        Channel {
            topic: topic.to_owned(),
            tx,
            reference: reference.to_owned(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(topic: &str, tx: fn(ChannelMessage) -> (), reference: &str) -> Self {
        Channel {
            topic: topic.to_owned(),
            tx: FauxSender(tx),
            reference: reference.to_owned(),
        }
    }

    #[cfg(not(feature = "protobuf"))]
    pub fn send(&mut self, event: Event, msg: Value) -> Result<(), Box<Error>> {
        let msg = Message {
            topic: self.topic.to_owned(),
            event,
            reference: Some(self.reference.to_owned()),
            join_ref: Some(self.reference.to_owned()),
            payload: msg.to_owned(),
        };

        self.send_msg(msg)
    }

    #[cfg(feature = "protobuf")]
    pub fn send<M: ProtoMessage>(&mut self, event: Event, msg: M) -> Result<(), Box<Error>> {
        let msg = Message {
            topic: self.topic.to_owned(),
            event,
            reference: Some(self.reference.to_owned()),
            join_ref: Some(self.reference.to_owned()),
            payload: msg.write_to_bytes()?,
        };

        self.send_msg(msg)
    }

    pub fn join(&mut self) -> Result<(), Box<Error>> {
        let msg = Message {
            topic: self.topic.to_owned(),
            event: Event::Defined(PhoenixEvent::Join),
            reference: Some(self.reference.to_owned()),
            join_ref: Some(self.reference.to_owned()),
            payload: null_value(),
        };

        self.send_msg(msg)
    }

    #[cfg_attr(target_arch = "wasm32", allow(unreachable_code))]
    fn send_msg(&self, msg: Message) -> Result<(), Box<Error>> {
        let chan_msg = msg_to_channel_msg(msg)?;
        self.tx
            .send(chan_msg)
            .map_err(|err| -> Box<Error> { box err })
    }
}
