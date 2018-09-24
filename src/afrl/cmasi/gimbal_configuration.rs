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
pub struct GimbalConfiguration {
    pub payload_id: i64,
    pub payload_kind: Vec<u8>,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub supported_pointing_modes: Vec<::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode>,
    pub min_azimuth: f32,
    pub max_azimuth: f32,
    pub is_azimuth_clamped: bool,
    pub min_elevation: f32,
    pub max_elevation: f32,
    pub is_elevation_clamped: bool,
    pub min_rotation: f32,
    pub max_rotation: f32,
    pub is_rotation_clamped: bool,
    pub max_azimuth_slew_rate: f32,
    pub max_elevation_slew_rate: f32,
    pub max_rotation_rate: f32,
    pub contained_payload_list: Vec<i64>,
}

impl PartialEq for GimbalConfiguration {
    fn eq(&self, _other: &GimbalConfiguration) -> bool {
        true
        && &self.supported_pointing_modes == &_other.supported_pointing_modes
        && &self.min_azimuth == &_other.min_azimuth
        && &self.max_azimuth == &_other.max_azimuth
        && &self.is_azimuth_clamped == &_other.is_azimuth_clamped
        && &self.min_elevation == &_other.min_elevation
        && &self.max_elevation == &_other.max_elevation
        && &self.is_elevation_clamped == &_other.is_elevation_clamped
        && &self.min_rotation == &_other.min_rotation
        && &self.max_rotation == &_other.max_rotation
        && &self.is_rotation_clamped == &_other.is_rotation_clamped
        && &self.max_azimuth_slew_rate == &_other.max_azimuth_slew_rate
        && &self.max_elevation_slew_rate == &_other.max_elevation_slew_rate
        && &self.max_rotation_rate == &_other.max_rotation_rate
        && &self.contained_payload_list == &_other.contained_payload_list

    }
}

impl LmcpSubscription for GimbalConfiguration {
    fn subscription() -> &'static str { "afrl.cmasi.GimbalConfiguration" }
}

impl Struct for GimbalConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 24,
        }
    }
}

impl Lmcp for GimbalConfiguration {
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
            let writeb: usize = self.supported_pointing_modes.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.min_azimuth.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_azimuth.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.is_azimuth_clamped.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.min_elevation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_elevation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.is_elevation_clamped.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.min_rotation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_rotation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.is_rotation_clamped.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_azimuth_slew_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_elevation_slew_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_rotation_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.contained_payload_list.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(GimbalConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == GimbalConfiguration::struct_info() {
            let mut out: GimbalConfiguration = Default::default();
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
                let (x, readb): (Vec<::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode>, usize) = Lmcp::deser(r)?;
                out.supported_pointing_modes = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.min_azimuth = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.max_azimuth = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.is_azimuth_clamped = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.min_elevation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.max_elevation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.is_elevation_clamped = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.min_rotation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.max_rotation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.is_rotation_clamped = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.max_azimuth_slew_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.max_elevation_slew_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.max_rotation_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.contained_payload_list = x;
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
        size += self.supported_pointing_modes.size();
        size += self.min_azimuth.size();
        size += self.max_azimuth.size();
        size += self.is_azimuth_clamped.size();
        size += self.min_elevation.size();
        size += self.max_elevation.size();
        size += self.is_elevation_clamped.size();
        size += self.min_rotation.size();
        size += self.max_rotation.size();
        size += self.is_rotation_clamped.size();
        size += self.max_azimuth_slew_rate.size();
        size += self.max_elevation_slew_rate.size();
        size += self.max_rotation_rate.size();
        size += self.contained_payload_list.size();

        size
    }
}

pub trait GimbalConfigurationT: Debug + Send + ::afrl::cmasi::payload_configuration::PayloadConfigurationT {
    fn as_afrl_cmasi_gimbal_configuration(&self) -> Option<&GimbalConfiguration> { None }
    fn as_mut_afrl_cmasi_gimbal_configuration(&mut self) -> Option<&mut GimbalConfiguration> { None }
    fn supported_pointing_modes(&self) -> &Vec<::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode>;
    fn supported_pointing_modes_mut(&mut self) -> &mut Vec<::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode>;
    fn min_azimuth(&self) -> f32;
    fn min_azimuth_mut(&mut self) -> &mut f32;
    fn max_azimuth(&self) -> f32;
    fn max_azimuth_mut(&mut self) -> &mut f32;
    fn is_azimuth_clamped(&self) -> bool;
    fn is_azimuth_clamped_mut(&mut self) -> &mut bool;
    fn min_elevation(&self) -> f32;
    fn min_elevation_mut(&mut self) -> &mut f32;
    fn max_elevation(&self) -> f32;
    fn max_elevation_mut(&mut self) -> &mut f32;
    fn is_elevation_clamped(&self) -> bool;
    fn is_elevation_clamped_mut(&mut self) -> &mut bool;
    fn min_rotation(&self) -> f32;
    fn min_rotation_mut(&mut self) -> &mut f32;
    fn max_rotation(&self) -> f32;
    fn max_rotation_mut(&mut self) -> &mut f32;
    fn is_rotation_clamped(&self) -> bool;
    fn is_rotation_clamped_mut(&mut self) -> &mut bool;
    fn max_azimuth_slew_rate(&self) -> f32;
    fn max_azimuth_slew_rate_mut(&mut self) -> &mut f32;
    fn max_elevation_slew_rate(&self) -> f32;
    fn max_elevation_slew_rate_mut(&mut self) -> &mut f32;
    fn max_rotation_rate(&self) -> f32;
    fn max_rotation_rate_mut(&mut self) -> &mut f32;
    fn contained_payload_list(&self) -> &Vec<i64>;
    fn contained_payload_list_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<GimbalConfigurationT> {
    fn clone(&self) -> Box<GimbalConfigurationT> {
        if let Some(x) = GimbalConfigurationT::as_afrl_cmasi_gimbal_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<GimbalConfigurationT> {
    fn default() -> Box<GimbalConfigurationT> { Box::new(GimbalConfiguration::default()) }
}

impl PartialEq for Box<GimbalConfigurationT> {
    fn eq(&self, other: &Box<GimbalConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (GimbalConfigurationT::as_afrl_cmasi_gimbal_configuration(self.as_ref()),
             GimbalConfigurationT::as_afrl_cmasi_gimbal_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<GimbalConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = GimbalConfigurationT::as_afrl_cmasi_gimbal_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<GimbalConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == GimbalConfiguration::struct_info() {
            let (x, readb) = GimbalConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = GimbalConfigurationT::as_afrl_cmasi_gimbal_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_configuration::PayloadConfigurationT for GimbalConfiguration {
    fn as_afrl_cmasi_gimbal_configuration(&self) -> Option<&GimbalConfiguration> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_configuration(&mut self) -> Option<&mut GimbalConfiguration> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn payload_kind(&self) -> &Vec<u8> { &self.payload_kind }
    fn payload_kind_mut(&mut self) -> &mut Vec<u8> { &mut self.payload_kind }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl GimbalConfigurationT for GimbalConfiguration {
    fn as_afrl_cmasi_gimbal_configuration(&self) -> Option<&GimbalConfiguration> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_configuration(&mut self) -> Option<&mut GimbalConfiguration> { Some(self) }
    fn supported_pointing_modes(&self) -> &Vec<::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode> { &self.supported_pointing_modes }
    fn supported_pointing_modes_mut(&mut self) -> &mut Vec<::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode> { &mut self.supported_pointing_modes }
    fn min_azimuth(&self) -> f32 { self.min_azimuth }
    fn min_azimuth_mut(&mut self) -> &mut f32 { &mut self.min_azimuth }
    fn max_azimuth(&self) -> f32 { self.max_azimuth }
    fn max_azimuth_mut(&mut self) -> &mut f32 { &mut self.max_azimuth }
    fn is_azimuth_clamped(&self) -> bool { self.is_azimuth_clamped }
    fn is_azimuth_clamped_mut(&mut self) -> &mut bool { &mut self.is_azimuth_clamped }
    fn min_elevation(&self) -> f32 { self.min_elevation }
    fn min_elevation_mut(&mut self) -> &mut f32 { &mut self.min_elevation }
    fn max_elevation(&self) -> f32 { self.max_elevation }
    fn max_elevation_mut(&mut self) -> &mut f32 { &mut self.max_elevation }
    fn is_elevation_clamped(&self) -> bool { self.is_elevation_clamped }
    fn is_elevation_clamped_mut(&mut self) -> &mut bool { &mut self.is_elevation_clamped }
    fn min_rotation(&self) -> f32 { self.min_rotation }
    fn min_rotation_mut(&mut self) -> &mut f32 { &mut self.min_rotation }
    fn max_rotation(&self) -> f32 { self.max_rotation }
    fn max_rotation_mut(&mut self) -> &mut f32 { &mut self.max_rotation }
    fn is_rotation_clamped(&self) -> bool { self.is_rotation_clamped }
    fn is_rotation_clamped_mut(&mut self) -> &mut bool { &mut self.is_rotation_clamped }
    fn max_azimuth_slew_rate(&self) -> f32 { self.max_azimuth_slew_rate }
    fn max_azimuth_slew_rate_mut(&mut self) -> &mut f32 { &mut self.max_azimuth_slew_rate }
    fn max_elevation_slew_rate(&self) -> f32 { self.max_elevation_slew_rate }
    fn max_elevation_slew_rate_mut(&mut self) -> &mut f32 { &mut self.max_elevation_slew_rate }
    fn max_rotation_rate(&self) -> f32 { self.max_rotation_rate }
    fn max_rotation_rate_mut(&mut self) -> &mut f32 { &mut self.max_rotation_rate }
    fn contained_payload_list(&self) -> &Vec<i64> { &self.contained_payload_list }
    fn contained_payload_list_mut(&mut self) -> &mut Vec<i64> { &mut self.contained_payload_list }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for GimbalConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> GimbalConfiguration {
            GimbalConfiguration {
                payload_id: Arbitrary::arbitrary(_g),
                payload_kind: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                supported_pointing_modes: Arbitrary::arbitrary(_g),
                min_azimuth: Arbitrary::arbitrary(_g),
                max_azimuth: Arbitrary::arbitrary(_g),
                is_azimuth_clamped: Arbitrary::arbitrary(_g),
                min_elevation: Arbitrary::arbitrary(_g),
                max_elevation: Arbitrary::arbitrary(_g),
                is_elevation_clamped: Arbitrary::arbitrary(_g),
                min_rotation: Arbitrary::arbitrary(_g),
                max_rotation: Arbitrary::arbitrary(_g),
                is_rotation_clamped: Arbitrary::arbitrary(_g),
                max_azimuth_slew_rate: Arbitrary::arbitrary(_g),
                max_elevation_slew_rate: Arbitrary::arbitrary(_g),
                max_rotation_rate: Arbitrary::arbitrary(_g),
                contained_payload_list: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: GimbalConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.supported_pointing_modes.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.contained_payload_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: GimbalConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.supported_pointing_modes.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.contained_payload_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = GimbalConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
