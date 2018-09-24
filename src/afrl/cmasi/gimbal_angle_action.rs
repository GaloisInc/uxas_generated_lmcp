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
pub struct GimbalAngleAction {
    pub associated_task_list: Vec<i64>,
    pub payload_id: i64,
    pub azimuth: f32,
    pub elevation: f32,
    pub rotation: f32,
}

impl PartialEq for GimbalAngleAction {
    fn eq(&self, _other: &GimbalAngleAction) -> bool {
        true
        && &self.azimuth == &_other.azimuth
        && &self.elevation == &_other.elevation
        && &self.rotation == &_other.rotation

    }
}

impl LmcpSubscription for GimbalAngleAction {
    fn subscription() -> &'static str { "afrl.cmasi.GimbalAngleAction" }
}

impl Struct for GimbalAngleAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 23,
        }
    }
}

impl Lmcp for GimbalAngleAction {
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
            let writeb: usize = self.payload_id.ser(r)?;
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

    fn deser(buf: &[u8]) -> Result<(GimbalAngleAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == GimbalAngleAction::struct_info() {
            let mut out: GimbalAngleAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.payload_id = x;
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
        size += self.associated_task_list.size();
        size += self.payload_id.size();
        size += self.azimuth.size();
        size += self.elevation.size();
        size += self.rotation.size();

        size
    }
}

pub trait GimbalAngleActionT: Debug + Send + ::afrl::cmasi::payload_action::PayloadActionT {
    fn as_afrl_cmasi_gimbal_angle_action(&self) -> Option<&GimbalAngleAction> { None }
    fn as_mut_afrl_cmasi_gimbal_angle_action(&mut self) -> Option<&mut GimbalAngleAction> { None }
    fn azimuth(&self) -> f32;
    fn azimuth_mut(&mut self) -> &mut f32;
    fn elevation(&self) -> f32;
    fn elevation_mut(&mut self) -> &mut f32;
    fn rotation(&self) -> f32;
    fn rotation_mut(&mut self) -> &mut f32;

}

impl Clone for Box<GimbalAngleActionT> {
    fn clone(&self) -> Box<GimbalAngleActionT> {
        if let Some(x) = GimbalAngleActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<GimbalAngleActionT> {
    fn default() -> Box<GimbalAngleActionT> { Box::new(GimbalAngleAction::default()) }
}

impl PartialEq for Box<GimbalAngleActionT> {
    fn eq(&self, other: &Box<GimbalAngleActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (GimbalAngleActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()),
             GimbalAngleActionT::as_afrl_cmasi_gimbal_angle_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<GimbalAngleActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = GimbalAngleActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<GimbalAngleActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == GimbalAngleAction::struct_info() {
            let (x, readb) = GimbalAngleAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = GimbalAngleActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for GimbalAngleAction {
    fn as_afrl_cmasi_gimbal_angle_action(&self) -> Option<&GimbalAngleAction> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_angle_action(&mut self) -> Option<&mut GimbalAngleAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl ::afrl::cmasi::payload_action::PayloadActionT for GimbalAngleAction {
    fn as_afrl_cmasi_gimbal_angle_action(&self) -> Option<&GimbalAngleAction> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_angle_action(&mut self) -> Option<&mut GimbalAngleAction> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
}
impl GimbalAngleActionT for GimbalAngleAction {
    fn as_afrl_cmasi_gimbal_angle_action(&self) -> Option<&GimbalAngleAction> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_angle_action(&mut self) -> Option<&mut GimbalAngleAction> { Some(self) }
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

    impl Arbitrary for GimbalAngleAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> GimbalAngleAction {
            GimbalAngleAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                payload_id: Arbitrary::arbitrary(_g),
                azimuth: Arbitrary::arbitrary(_g),
                elevation: Arbitrary::arbitrary(_g),
                rotation: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: GimbalAngleAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: GimbalAngleAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = GimbalAngleAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
