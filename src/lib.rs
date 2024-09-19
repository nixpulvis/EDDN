use chrono::prelude::*;
use elite_journal::entry::{Entry, Event, Market};
use miniz_oxide::inflate;
use serde::Deserialize;

// TODO: Error type.

pub const URL: &'static str = "tcp://eddn.edcd.io:9500";

/// Top level EDDN message wrapper
#[derive(Debug, Deserialize)]
pub struct Envelope {
    #[serde(rename = "$schemaRef")]
    pub schema_ref: String,
    pub header: Header,
    pub message: Message,
}

/// Message uploader metadata
#[derive(Debug, Deserialize)]
pub struct Header {
    #[serde(rename = "gatewayTimestamp")]
    pub gateway_timestamp: DateTime<Utc>,
    #[serde(rename = "softwareName")]
    pub software_name: String,
    #[serde(rename = "softwareVersion")]
    pub software_version: String,
    #[serde(rename = "uploaderID")]
    pub uploader_id: String,
}

/// Payload of the message containing the parsed data
#[derive(Debug, Deserialize)]
// TODO: Don't use untagged, we need to write a custom deserialized that uses the $schemaRef.
// NOTE: [ "Docked", "FSDJump", "Scan", "Location", "SAASignalsFound", "CarrierJump" ]
//       https://github.com/EDCD/EDDN/blob/d9b5586a4ef5a5c4c1117ec4105b773697b468ac/schemas/journal-v1.0.json#L43
#[serde(untagged)]
pub enum Message {
    Journal(Entry<Event>),
    Commodity(Entry<Market>),
    // TODO
    // Shipyard,
    // Outfitting,
    // Blackmarket,

    // Untagged catchall, must be at the end.
    Other(serde_json::Value),
}

/// Subscribe to EDDN's ZMQ socket receiving all messages
pub fn subscribe(url: &str) -> EnvelopeIterator {
    println!("Starting EDDN ZeroMQ consumer...");

    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).expect("failed to open socket");

    socket.connect(url).expect("connection failed");
    socket
        .set_subscribe(&vec![]) // Required to subscribe to everything
        .expect("failed to subscribe");

    println!("Connected and subscribed.");

    EnvelopeIterator { socket }
}

/// Decompress and parses each message from the ZMQ socket
pub struct EnvelopeIterator {
    socket: zmq::Socket,
}

impl Iterator for EnvelopeIterator {
    type Item = Result<Envelope, serde_json::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let compressed =
            self.socket.recv_bytes(0).expect("failed to receive bytes");
        let json = inflate::decompress_to_vec_zlib(&compressed)
            .expect("failed to decompress");
        Some(serde_json::from_slice(&json))
    }
}

// TODO: Make use of the schema service
// const SCHEMA_JOURNAL : &str = "https://eddn.edcd.io/schemas/journal/1";
