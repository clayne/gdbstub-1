use super::prelude::*;

#[derive(Debug)]
pub struct qTfV;

impl<'a> ParseCommand<'a> for qTfV {
    #[inline(always)]
    fn from_packet(buf: PacketBuf<'a>) -> Option<Self> {
        if !buf.into_body().is_empty() {
            return None;
        }
        Some(qTfV)
    }
}
