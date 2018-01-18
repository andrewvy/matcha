use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use tokio_io::codec::{Encoder, Decoder};
use matcha_pb::{Message};
use failure::Error;
pub struct MessageCodec;

use protobuf::{self, Message as PBMessage};

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Message>, Error> {
        let buffer_len = buffer.len();

        // If buffer doesn't have our length prefix, skip.
        if buffer_len < 4 {
            return Ok(None);
        }

        let payload_len = BigEndian::read_u32(&(buffer.as_ref()[0..4])) as usize;
        let full_len = 4 + payload_len;

        if buffer_len < full_len {
            return Ok(None);
        }

        let mut data = buffer.split_to(full_len);
        data.split_to(4);

        match protobuf::parse_from_bytes(data.to_vec().as_slice()) {
            Ok(message) => Ok(Some(message)),
            Err(_) => Err(format_err!("Could not decode message")),
        }
    }
}


impl Encoder for MessageCodec {
    type Item = Message;
    type Error = Error;

    fn encode(&mut self, message: Message, buffer: &mut BytesMut) -> Result<(), Error> {
        buffer.put_u32::<BigEndian>(message.compute_size());
        let mut writer = buffer.writer();

        message.write_to_writer(&mut writer).map_err(|_| {
            format_err!("Could not serialize message")
        })
    }
}
