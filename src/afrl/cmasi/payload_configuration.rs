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
pub struct PayloadConfiguration {
    pub payload_id: i64,
    pub payload_kind: Vec<u8>,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
}

impl PartialEq for PayloadConfiguration {
    fn eq(&self, _other: &PayloadConfiguration) -> bool {
        true
        && &self.payload_id == &_other.payload_id
        && &self.payload_kind == &_other.payload_kind
        && &self.parameters == &_other.parameters

    }
}

impl LmcpSubscription for PayloadConfiguration {
    fn subscription() -> &'static str { "afrl.cmasi.PayloadConfiguration" }
}

impl Struct for PayloadConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 5,
        }
    }
}

impl Lmcp for PayloadConfiguration {
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

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PayloadConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PayloadConfiguration::struct_info() {
            let mut out: PayloadConfiguration = Default::default();
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

        size
    }
}

pub trait PayloadConfigurationT: Debug + Send  {
    fn as_afrl_cmasi_payload_configuration(&self) -> Option<&PayloadConfiguration> { None }
    fn as_mut_afrl_cmasi_payload_configuration(&mut self) -> Option<&mut PayloadConfiguration> { None }
    fn as_afrl_impact_radio_configuration(&self) -> Option<&::afrl::impact::radio_configuration::RadioConfiguration> { None }
    fn as_mut_afrl_impact_radio_configuration(&mut self) -> Option<&mut ::afrl::impact::radio_configuration::RadioConfiguration> { None }
    fn as_afrl_cmasi_gimbal_configuration(&self) -> Option<&::afrl::cmasi::gimbal_configuration::GimbalConfiguration> { None }
    fn as_mut_afrl_cmasi_gimbal_configuration(&mut self) -> Option<&mut ::afrl::cmasi::gimbal_configuration::GimbalConfiguration> { None }
    fn as_afrl_cmasi_camera_configuration(&self) -> Option<&::afrl::cmasi::camera_configuration::CameraConfiguration> { None }
    fn as_mut_afrl_cmasi_camera_configuration(&mut self) -> Option<&mut ::afrl::cmasi::camera_configuration::CameraConfiguration> { None }
    fn as_afrl_impact_impact_payload_configuration(&self) -> Option<&::afrl::impact::impact_payload_configuration::ImpactPayloadConfiguration> { None }
    fn as_mut_afrl_impact_impact_payload_configuration(&mut self) -> Option<&mut ::afrl::impact::impact_payload_configuration::ImpactPayloadConfiguration> { None }
    fn as_afrl_cmasi_video_stream_configuration(&self) -> Option<&::afrl::cmasi::video_stream_configuration::VideoStreamConfiguration> { None }
    fn as_mut_afrl_cmasi_video_stream_configuration(&mut self) -> Option<&mut ::afrl::cmasi::video_stream_configuration::VideoStreamConfiguration> { None }
    fn as_afrl_impact_power_configuration(&self) -> Option<&::afrl::impact::power_configuration::PowerConfiguration> { None }
    fn as_mut_afrl_impact_power_configuration(&mut self) -> Option<&mut ::afrl::impact::power_configuration::PowerConfiguration> { None }
    fn payload_id(&self) -> i64;
    fn payload_id_mut(&mut self) -> &mut i64;
    fn payload_kind(&self) -> &Vec<u8>;
    fn payload_kind_mut(&mut self) -> &mut Vec<u8>;
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;

}

impl Clone for Box<PayloadConfigurationT> {
    fn clone(&self) -> Box<PayloadConfigurationT> {
        if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_payload_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadConfigurationT::as_afrl_impact_radio_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_gimbal_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_camera_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadConfigurationT::as_afrl_impact_impact_payload_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_video_stream_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadConfigurationT::as_afrl_impact_power_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PayloadConfigurationT> {
    fn default() -> Box<PayloadConfigurationT> { Box::new(PayloadConfiguration::default()) }
}

impl PartialEq for Box<PayloadConfigurationT> {
    fn eq(&self, other: &Box<PayloadConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (PayloadConfigurationT::as_afrl_cmasi_payload_configuration(self.as_ref()),
             PayloadConfigurationT::as_afrl_cmasi_payload_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadConfigurationT::as_afrl_impact_radio_configuration(self.as_ref()),
             PayloadConfigurationT::as_afrl_impact_radio_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadConfigurationT::as_afrl_cmasi_gimbal_configuration(self.as_ref()),
             PayloadConfigurationT::as_afrl_cmasi_gimbal_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadConfigurationT::as_afrl_cmasi_camera_configuration(self.as_ref()),
             PayloadConfigurationT::as_afrl_cmasi_camera_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadConfigurationT::as_afrl_impact_impact_payload_configuration(self.as_ref()),
             PayloadConfigurationT::as_afrl_impact_impact_payload_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadConfigurationT::as_afrl_cmasi_video_stream_configuration(self.as_ref()),
             PayloadConfigurationT::as_afrl_cmasi_video_stream_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadConfigurationT::as_afrl_impact_power_configuration(self.as_ref()),
             PayloadConfigurationT::as_afrl_impact_power_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PayloadConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_payload_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadConfigurationT::as_afrl_impact_radio_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_gimbal_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_camera_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadConfigurationT::as_afrl_impact_impact_payload_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_video_stream_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadConfigurationT::as_afrl_impact_power_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PayloadConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PayloadConfiguration::struct_info() {
            let (x, readb) = PayloadConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::radio_configuration::RadioConfiguration::struct_info() {
            let (x, readb) = ::afrl::impact::radio_configuration::RadioConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::gimbal_configuration::GimbalConfiguration::struct_info() {
            let (x, readb) = ::afrl::cmasi::gimbal_configuration::GimbalConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::camera_configuration::CameraConfiguration::struct_info() {
            let (x, readb) = ::afrl::cmasi::camera_configuration::CameraConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::impact_payload_configuration::ImpactPayloadConfiguration::struct_info() {
            let (x, readb) = ::afrl::impact::impact_payload_configuration::ImpactPayloadConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::video_stream_configuration::VideoStreamConfiguration::struct_info() {
            let (x, readb) = ::afrl::cmasi::video_stream_configuration::VideoStreamConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::power_configuration::PowerConfiguration::struct_info() {
            let (x, readb) = ::afrl::impact::power_configuration::PowerConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_payload_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadConfigurationT::as_afrl_impact_radio_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_gimbal_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_camera_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadConfigurationT::as_afrl_impact_impact_payload_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadConfigurationT::as_afrl_cmasi_video_stream_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadConfigurationT::as_afrl_impact_power_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl PayloadConfigurationT for PayloadConfiguration {
    fn as_afrl_cmasi_payload_configuration(&self) -> Option<&PayloadConfiguration> { Some(self) }
    fn as_mut_afrl_cmasi_payload_configuration(&mut self) -> Option<&mut PayloadConfiguration> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn payload_kind(&self) -> &Vec<u8> { &self.payload_kind }
    fn payload_kind_mut(&mut self) -> &mut Vec<u8> { &mut self.payload_kind }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PayloadConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> PayloadConfiguration {
            PayloadConfiguration {
                payload_id: Arbitrary::arbitrary(_g),
                payload_kind: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PayloadConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PayloadConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PayloadConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
