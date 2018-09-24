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
pub struct VideoStreamAction {
    pub associated_task_list: Vec<i64>,
    pub video_stream_id: i32,
    pub active_sensor: i32,
}

impl PartialEq for VideoStreamAction {
    fn eq(&self, _other: &VideoStreamAction) -> bool {
        true
        && &self.video_stream_id == &_other.video_stream_id
        && &self.active_sensor == &_other.active_sensor

    }
}

impl LmcpSubscription for VideoStreamAction {
    fn subscription() -> &'static str { "afrl.cmasi.VideoStreamAction" }
}

impl Struct for VideoStreamAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 48,
        }
    }
}

impl Lmcp for VideoStreamAction {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.associated_task_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.video_stream_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.active_sensor.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(VideoStreamAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == VideoStreamAction::struct_info() {
            let mut out: VideoStreamAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i32, usize) = Lmcp::deser(r)?;
                out.video_stream_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i32, usize) = Lmcp::deser(r)?;
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
        size += self.associated_task_list.size();
        size += self.video_stream_id.size();
        size += self.active_sensor.size();

        size
    }
}

pub trait VideoStreamActionT: Debug + Send + ::afrl::cmasi::vehicle_action::VehicleActionT {
    fn as_afrl_cmasi_video_stream_action(&self) -> Option<&VideoStreamAction> { None }
    fn as_mut_afrl_cmasi_video_stream_action(&mut self) -> Option<&mut VideoStreamAction> { None }
    fn video_stream_id(&self) -> i32;
    fn video_stream_id_mut(&mut self) -> &mut i32;
    fn active_sensor(&self) -> i32;
    fn active_sensor_mut(&mut self) -> &mut i32;

}

impl Clone for Box<VideoStreamActionT> {
    fn clone(&self) -> Box<VideoStreamActionT> {
        if let Some(x) = VideoStreamActionT::as_afrl_cmasi_video_stream_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<VideoStreamActionT> {
    fn default() -> Box<VideoStreamActionT> { Box::new(VideoStreamAction::default()) }
}

impl PartialEq for Box<VideoStreamActionT> {
    fn eq(&self, other: &Box<VideoStreamActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (VideoStreamActionT::as_afrl_cmasi_video_stream_action(self.as_ref()),
             VideoStreamActionT::as_afrl_cmasi_video_stream_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<VideoStreamActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = VideoStreamActionT::as_afrl_cmasi_video_stream_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<VideoStreamActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == VideoStreamAction::struct_info() {
            let (x, readb) = VideoStreamAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = VideoStreamActionT::as_afrl_cmasi_video_stream_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for VideoStreamAction {
    fn as_afrl_cmasi_video_stream_action(&self) -> Option<&VideoStreamAction> { Some(self) }
    fn as_mut_afrl_cmasi_video_stream_action(&mut self) -> Option<&mut VideoStreamAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl VideoStreamActionT for VideoStreamAction {
    fn as_afrl_cmasi_video_stream_action(&self) -> Option<&VideoStreamAction> { Some(self) }
    fn as_mut_afrl_cmasi_video_stream_action(&mut self) -> Option<&mut VideoStreamAction> { Some(self) }
    fn video_stream_id(&self) -> i32 { self.video_stream_id }
    fn video_stream_id_mut(&mut self) -> &mut i32 { &mut self.video_stream_id }
    fn active_sensor(&self) -> i32 { self.active_sensor }
    fn active_sensor_mut(&mut self) -> &mut i32 { &mut self.active_sensor }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for VideoStreamAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> VideoStreamAction {
            VideoStreamAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                video_stream_id: Arbitrary::arbitrary(_g),
                active_sensor: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: VideoStreamAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: VideoStreamAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = VideoStreamAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
