// ===============================================================================
// Authors: AFRL/RQQA
// Organization: Air Force Research Laboratory, Aerospace Systems Directorate, Power and Control Division
// 
// Copyright (c) 2017 Government of the United State of America, as represented by
// the Secretary of the Air Force.  No copyright is claimed in the United States under
// Title 17, U.S. Code.  All Other Rights Reserved.
// ===============================================================================

// This file was auto-created by LmcpGen. Modifications will be overwritten.

use avtas::lmcp::{Error, ErrorType, Lmcp, LmcpSubscription, SrcLoc, Struct, StructInfo};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct VideoRecord {
    pub record: bool,
}

impl PartialEq for VideoRecord {
    fn eq(&self, _other: &VideoRecord) -> bool {
        true
        && &self.record == &_other.record

    }
}

impl LmcpSubscription for VideoRecord {
    fn subscription() -> &'static str { "uxas.messages.uxnative.VideoRecord" }
}

impl Struct for VideoRecord {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 1,
        }
    }
}

impl Lmcp for VideoRecord {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.record.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(VideoRecord, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == VideoRecord::struct_info() {
            let mut out: VideoRecord = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.record = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.record.size();

        size
    }
}

pub trait VideoRecordT: Debug + Send  {
    fn as_uxas_messages_uxnative_video_record(&self) -> Option<&VideoRecord> { None }
    fn as_mut_uxas_messages_uxnative_video_record(&mut self) -> Option<&mut VideoRecord> { None }
    fn record(&self) -> bool;
    fn record_mut(&mut self) -> &mut bool;

}

impl Clone for Box<VideoRecordT> {
    fn clone(&self) -> Box<VideoRecordT> {
        if let Some(x) = VideoRecordT::as_uxas_messages_uxnative_video_record(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<VideoRecordT> {
    fn default() -> Box<VideoRecordT> { Box::new(VideoRecord::default()) }
}

impl PartialEq for Box<VideoRecordT> {
    fn eq(&self, other: &Box<VideoRecordT>) -> bool {
        if let (Some(x), Some(y)) =
            (VideoRecordT::as_uxas_messages_uxnative_video_record(self.as_ref()),
             VideoRecordT::as_uxas_messages_uxnative_video_record(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<VideoRecordT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = VideoRecordT::as_uxas_messages_uxnative_video_record(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<VideoRecordT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == VideoRecord::struct_info() {
            let (x, readb) = VideoRecord::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = VideoRecordT::as_uxas_messages_uxnative_video_record(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl VideoRecordT for VideoRecord {
    fn as_uxas_messages_uxnative_video_record(&self) -> Option<&VideoRecord> { Some(self) }
    fn as_mut_uxas_messages_uxnative_video_record(&mut self) -> Option<&mut VideoRecord> { Some(self) }
    fn record(&self) -> bool { self.record }
    fn record_mut(&mut self) -> &mut bool { &mut self.record }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for VideoRecord {
        fn arbitrary<G: Gen>(_g: &mut G) -> VideoRecord {
            VideoRecord {
                record: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: VideoRecord) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: VideoRecord) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = VideoRecord::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
