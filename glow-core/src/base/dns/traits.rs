use bytes::BytesMut;

pub trait EncodeRData {
    fn encode(&self) -> BytesMut;
}

pub trait DecodeRData {
    fn decode(&self, raw: &[u8]);
}
