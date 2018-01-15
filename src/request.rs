use std::fmt::Display;

use bytes::BytesMut;

pub trait ScpiRequest: Display {
    fn decode(message: &str) -> Option<Self>
    where
        Self: Sized;

    fn encode(&self, buffer: &mut BytesMut) {
        buffer.extend(self.to_string().as_bytes());
        buffer.extend("\n".as_bytes())
    }
}
