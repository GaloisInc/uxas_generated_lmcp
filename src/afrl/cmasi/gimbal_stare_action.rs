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
pub struct GimbalStareAction {
    pub associated_task_list: Vec<i64>,
    pub payload_id: i64,
    pub starepoint: Box<::afrl::cmasi::location3d::Location3DT>,
    pub duration: i64,
}

impl PartialEq for GimbalStareAction {
    fn eq(&self, _other: &GimbalStareAction) -> bool {
        true
        && &self.starepoint == &_other.starepoint
        && &self.duration == &_other.duration

    }
}

impl LmcpSubscription for GimbalStareAction {
    fn subscription() -> &'static str { "afrl.cmasi.GimbalStareAction" }
}

impl Struct for GimbalStareAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 26,
        }
    }
}

impl Lmcp for GimbalStareAction {
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
            let writeb: usize = self.starepoint.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.duration.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(GimbalStareAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == GimbalStareAction::struct_info() {
            let mut out: GimbalStareAction = Default::default();
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
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.starepoint = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.duration = x;
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
        size += self.starepoint.size();
        size += self.duration.size();

        size
    }
}

pub trait GimbalStareActionT: Debug + Send + ::afrl::cmasi::payload_action::PayloadActionT {
    fn as_afrl_cmasi_gimbal_stare_action(&self) -> Option<&GimbalStareAction> { None }
    fn as_mut_afrl_cmasi_gimbal_stare_action(&mut self) -> Option<&mut GimbalStareAction> { None }
    fn starepoint(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn starepoint_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn duration(&self) -> i64;
    fn duration_mut(&mut self) -> &mut i64;

}

impl Clone for Box<GimbalStareActionT> {
    fn clone(&self) -> Box<GimbalStareActionT> {
        if let Some(x) = GimbalStareActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<GimbalStareActionT> {
    fn default() -> Box<GimbalStareActionT> { Box::new(GimbalStareAction::default()) }
}

impl PartialEq for Box<GimbalStareActionT> {
    fn eq(&self, other: &Box<GimbalStareActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (GimbalStareActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()),
             GimbalStareActionT::as_afrl_cmasi_gimbal_stare_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<GimbalStareActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = GimbalStareActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<GimbalStareActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == GimbalStareAction::struct_info() {
            let (x, readb) = GimbalStareAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = GimbalStareActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for GimbalStareAction {
    fn as_afrl_cmasi_gimbal_stare_action(&self) -> Option<&GimbalStareAction> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_stare_action(&mut self) -> Option<&mut GimbalStareAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl ::afrl::cmasi::payload_action::PayloadActionT for GimbalStareAction {
    fn as_afrl_cmasi_gimbal_stare_action(&self) -> Option<&GimbalStareAction> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_stare_action(&mut self) -> Option<&mut GimbalStareAction> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
}
impl GimbalStareActionT for GimbalStareAction {
    fn as_afrl_cmasi_gimbal_stare_action(&self) -> Option<&GimbalStareAction> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_stare_action(&mut self) -> Option<&mut GimbalStareAction> { Some(self) }
    fn starepoint(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.starepoint }
    fn starepoint_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.starepoint }
    fn duration(&self) -> i64 { self.duration }
    fn duration_mut(&mut self) -> &mut i64 { &mut self.duration }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for GimbalStareAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> GimbalStareAction {
            GimbalStareAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                payload_id: Arbitrary::arbitrary(_g),
                starepoint: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                duration: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: GimbalStareAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: GimbalStareAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = GimbalStareAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
