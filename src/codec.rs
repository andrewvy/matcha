use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use bincode::{serialize, deserialize, Infinite};
use protocol::{Message};
use failure::Error;

pub struct MessageCodec;

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Message>, Error> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            Ok(deserialize(&line).ok())
        } else {
            Ok(None)
        }
    }
}


impl Encoder for MessageCodec {
    type Item = Message;
    type Error = Error;

    fn encode(&mut self, msg: Message, buf: &mut BytesMut) -> Result<(), Error> {
        let serialized_message = serialize(&msg, Infinite)?;
        buf.extend(serialized_message);
        buf.extend(b"\n");
        Ok(())
    }
}
