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
pub struct GimbalState {
    pub payload_id: i64,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub pointing_mode: ::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode,
    pub azimuth: f32,
    pub elevation: f32,
    pub rotation: f32,
}

impl PartialEq for GimbalState {
    fn eq(&self, _other: &GimbalState) -> bool {
        true
        && &self.pointing_mode == &_other.pointing_mode
        && &self.azimuth == &_other.azimuth
        && &self.elevation == &_other.elevation
        && &self.rotation == &_other.rotation

    }
}

impl LmcpSubscription for GimbalState {
    fn subscription() -> &'static str { "afrl.cmasi.GimbalState" }
}

impl Struct for GimbalState {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 27,
        }
    }
}

impl Lmcp for GimbalState {
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
            let writeb: usize = self.pointing_mode.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.azimuth.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.elevation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.rotation.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(GimbalState, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == GimbalState::struct_info() {
            let mut out: GimbalState = Default::default();
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
                let (x, readb): (::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode, usize) = Lmcp::deser(r)?;
                out.pointing_mode = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.azimuth = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.elevation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.rotation = x;
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
        size += self.pointing_mode.size();
        size += self.azimuth.size();
        size += self.elevation.size();
        size += self.rotation.size();

        size
    }
}

pub trait GimbalStateT: Debug + Send + ::afrl::cmasi::payload_state::PayloadStateT {
    fn as_afrl_cmasi_gimbal_state(&self) -> Option<&GimbalState> { None }
    fn as_mut_afrl_cmasi_gimbal_state(&mut self) -> Option<&mut GimbalState> { None }
    fn pointing_mode(&self) -> ::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode;
    fn pointing_mode_mut(&mut self) -> &mut ::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode;
    fn azimuth(&self) -> f32;
    fn azimuth_mut(&mut self) -> &mut f32;
    fn elevation(&self) -> f32;
    fn elevation_mut(&mut self) -> &mut f32;
    fn rotation(&self) -> f32;
    fn rotation_mut(&mut self) -> &mut f32;

}

impl Clone for Box<GimbalStateT> {
    fn clone(&self) -> Box<GimbalStateT> {
        if let Some(x) = GimbalStateT::as_afrl_cmasi_gimbal_state(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<GimbalStateT> {
    fn default() -> Box<GimbalStateT> { Box::new(GimbalState::default()) }
}

impl PartialEq for Box<GimbalStateT> {
    fn eq(&self, other: &Box<GimbalStateT>) -> bool {
        if let (Some(x), Some(y)) =
            (GimbalStateT::as_afrl_cmasi_gimbal_state(self.as_ref()),
             GimbalStateT::as_afrl_cmasi_gimbal_state(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<GimbalStateT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = GimbalStateT::as_afrl_cmasi_gimbal_state(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<GimbalStateT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == GimbalState::struct_info() {
            let (x, readb) = GimbalState::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = GimbalStateT::as_afrl_cmasi_gimbal_state(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_state::PayloadStateT for GimbalState {
    fn as_afrl_cmasi_gimbal_state(&self) -> Option<&GimbalState> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_state(&mut self) -> Option<&mut GimbalState> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl GimbalStateT for GimbalState {
    fn as_afrl_cmasi_gimbal_state(&self) -> Option<&GimbalState> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_state(&mut self) -> Option<&mut GimbalState> { Some(self) }
    fn pointing_mode(&self) -> ::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode { self.pointing_mode }
    fn pointing_mode_mut(&mut self) -> &mut ::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode { &mut self.pointing_mode }
    fn azimuth(&self) -> f32 { self.azimuth }
    fn azimuth_mut(&mut self) -> &mut f32 { &mut self.azimuth }
    fn elevation(&self) -> f32 { self.elevation }
    fn elevation_mut(&mut self) -> &mut f32 { &mut self.elevation }
    fn rotation(&self) -> f32 { self.rotation }
    fn rotation_mut(&mut self) -> &mut f32 { &mut self.rotation }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for GimbalState {
        fn arbitrary<G: Gen>(_g: &mut G) -> GimbalState {
            GimbalState {
                payload_id: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                pointing_mode: Arbitrary::arbitrary(_g),
                azimuth: Arbitrary::arbitrary(_g),
                elevation: Arbitrary::arbitrary(_g),
                rotation: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: GimbalState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: GimbalState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = GimbalState::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
