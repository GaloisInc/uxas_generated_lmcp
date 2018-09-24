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
pub struct VideoStreamConfiguration {
    pub payload_id: i64,
    pub payload_kind: Vec<u8>,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub available_sensor_list: Vec<i64>,
}

impl PartialEq for VideoStreamConfiguration {
    fn eq(&self, _other: &VideoStreamConfiguration) -> bool {
        true
        && &self.available_sensor_list == &_other.available_sensor_list

    }
}

impl LmcpSubscription for VideoStreamConfiguration {
    fn subscription() -> &'static str { "afrl.cmasi.VideoStreamConfiguration" }
}

impl Struct for VideoStreamConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 49,
        }
    }
}

impl Lmcp for VideoStreamConfiguration {
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
            let writeb: usize = self.payload_kind.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.parameters.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.available_sensor_list.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(VideoStreamConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == VideoStreamConfiguration::struct_info() {
            let mut out: VideoStreamConfiguration = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.payload_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.payload_kind = x;
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
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.available_sensor_list = x;
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
        size += self.payload_kind.size();
        size += self.parameters.size();
        size += self.available_sensor_list.size();

        size
    }
}

pub trait VideoStreamConfigurationT: Debug + Send + ::afrl::cmasi::payload_configuration::PayloadConfigurationT {
    fn as_afrl_cmasi_video_stream_configuration(&self) -> Option<&VideoStreamConfiguration> { None }
    fn as_mut_afrl_cmasi_video_stream_configuration(&mut self) -> Option<&mut VideoStreamConfiguration> { None }
    fn available_sensor_list(&self) -> &Vec<i64>;
    fn available_sensor_list_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<VideoStreamConfigurationT> {
    fn clone(&self) -> Box<VideoStreamConfigurationT> {
        if let Some(x) = VideoStreamConfigurationT::as_afrl_cmasi_video_stream_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<VideoStreamConfigurationT> {
    fn default() -> Box<VideoStreamConfigurationT> { Box::new(VideoStreamConfiguration::default()) }
}

impl PartialEq for Box<VideoStreamConfigurationT> {
    fn eq(&self, other: &Box<VideoStreamConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (VideoStreamConfigurationT::as_afrl_cmasi_video_stream_configuration(self.as_ref()),
             VideoStreamConfigurationT::as_afrl_cmasi_video_stream_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<VideoStreamConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = VideoStreamConfigurationT::as_afrl_cmasi_video_stream_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<VideoStreamConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == VideoStreamConfiguration::struct_info() {
            let (x, readb) = VideoStreamConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = VideoStreamConfigurationT::as_afrl_cmasi_video_stream_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_configuration::PayloadConfigurationT for VideoStreamConfiguration {
    fn as_afrl_cmasi_video_stream_configuration(&self) -> Option<&VideoStreamConfiguration> { Some(self) }
    fn as_mut_afrl_cmasi_video_stream_configuration(&mut self) -> Option<&mut VideoStreamConfiguration> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn payload_kind(&self) -> &Vec<u8> { &self.payload_kind }
    fn payload_kind_mut(&mut self) -> &mut Vec<u8> { &mut self.payload_kind }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl VideoStreamConfigurationT for VideoStreamConfiguration {
    fn as_afrl_cmasi_video_stream_configuration(&self) -> Option<&VideoStreamConfiguration> { Some(self) }
    fn as_mut_afrl_cmasi_video_stream_configuration(&mut self) -> Option<&mut VideoStreamConfiguration> { Some(self) }
    fn available_sensor_list(&self) -> &Vec<i64> { &self.available_sensor_list }
    fn available_sensor_list_mut(&mut self) -> &mut Vec<i64> { &mut self.available_sensor_list }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for VideoStreamConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> VideoStreamConfiguration {
            VideoStreamConfiguration {
                payload_id: Arbitrary::arbitrary(_g),
                payload_kind: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                available_sensor_list: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: VideoStreamConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.available_sensor_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: VideoStreamConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.available_sensor_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = VideoStreamConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
