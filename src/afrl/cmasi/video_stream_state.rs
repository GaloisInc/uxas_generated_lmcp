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

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct VideoStreamState {
    pub payload_id: i64,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub active_sensor: i64,
}

impl PartialEq for VideoStreamState {
    fn eq(&self, _other: &VideoStreamState) -> bool {
        true
        && &self.active_sensor == &_other.active_sensor

    }
}

impl LmcpSubscription for VideoStreamState {
    fn subscription() -> &'static str { "afrl.cmasi.VideoStreamState" }
}

impl Struct for VideoStreamState {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 50,
        }
    }
}

impl Lmcp for VideoStreamState {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.payload_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.parameters.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.active_sensor.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(VideoStreamState, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == VideoStreamState::struct_info() {
            let mut out: VideoStreamState = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.payload_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>, usize) = Lmcp::deser(r)?;
                out.parameters = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.active_sensor = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.payload_id.size();
        size += self.parameters.size();
        size += self.active_sensor.size();

        size
    }
}

pub trait VideoStreamStateT: Debug + Send + ::afrl::cmasi::payload_state::PayloadStateT {
    fn as_afrl_cmasi_video_stream_state(&self) -> Option<&VideoStreamState> { None }
    fn as_mut_afrl_cmasi_video_stream_state(&mut self) -> Option<&mut VideoStreamState> { None }
    fn active_sensor(&self) -> i64;
    fn active_sensor_mut(&mut self) -> &mut i64;

}

impl Clone for Box<VideoStreamStateT> {
    fn clone(&self) -> Box<VideoStreamStateT> {
        if let Some(x) = VideoStreamStateT::as_afrl_cmasi_video_stream_state(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<VideoStreamStateT> {
    fn default() -> Box<VideoStreamStateT> { Box::new(VideoStreamState::default()) }
}

impl PartialEq for Box<VideoStreamStateT> {
    fn eq(&self, other: &Box<VideoStreamStateT>) -> bool {
        if let (Some(x), Some(y)) =
            (VideoStreamStateT::as_afrl_cmasi_video_stream_state(self.as_ref()),
             VideoStreamStateT::as_afrl_cmasi_video_stream_state(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<VideoStreamStateT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = VideoStreamStateT::as_afrl_cmasi_video_stream_state(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<VideoStreamStateT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == VideoStreamState::struct_info() {
            let (x, readb) = VideoStreamState::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = VideoStreamStateT::as_afrl_cmasi_video_stream_state(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_state::PayloadStateT for VideoStreamState {
    fn as_afrl_cmasi_video_stream_state(&self) -> Option<&VideoStreamState> { Some(self) }
    fn as_mut_afrl_cmasi_video_stream_state(&mut self) -> Option<&mut VideoStreamState> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl VideoStreamStateT for VideoStreamState {
    fn as_afrl_cmasi_video_stream_state(&self) -> Option<&VideoStreamState> { Some(self) }
    fn as_mut_afrl_cmasi_video_stream_state(&mut self) -> Option<&mut VideoStreamState> { Some(self) }
    fn active_sensor(&self) -> i64 { self.active_sensor }
    fn active_sensor_mut(&mut self) -> &mut i64 { &mut self.active_sensor }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for VideoStreamState {
        fn arbitrary<G: Gen>(_g: &mut G) -> VideoStreamState {
            VideoStreamState {
                payload_id: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                active_sensor: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: VideoStreamState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: VideoStreamState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = VideoStreamState::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
