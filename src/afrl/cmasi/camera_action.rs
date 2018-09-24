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
pub struct CameraAction {
    pub associated_task_list: Vec<i64>,
    pub payload_id: i64,
    pub horizontal_field_of_view: f32,
}

impl PartialEq for CameraAction {
    fn eq(&self, _other: &CameraAction) -> bool {
        true
        && &self.horizontal_field_of_view == &_other.horizontal_field_of_view

    }
}

impl LmcpSubscription for CameraAction {
    fn subscription() -> &'static str { "afrl.cmasi.CameraAction" }
}

impl Struct for CameraAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 18,
        }
    }
}

impl Lmcp for CameraAction {
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
            let writeb: usize = self.horizontal_field_of_view.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(CameraAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == CameraAction::struct_info() {
            let mut out: CameraAction = Default::default();
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
                out.horizontal_field_of_view = x;
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
        size += self.horizontal_field_of_view.size();

        size
    }
}

pub trait CameraActionT: Debug + Send + ::afrl::cmasi::payload_action::PayloadActionT {
    fn as_afrl_cmasi_camera_action(&self) -> Option<&CameraAction> { None }
    fn as_mut_afrl_cmasi_camera_action(&mut self) -> Option<&mut CameraAction> { None }
    fn horizontal_field_of_view(&self) -> f32;
    fn horizontal_field_of_view_mut(&mut self) -> &mut f32;

}

impl Clone for Box<CameraActionT> {
    fn clone(&self) -> Box<CameraActionT> {
        if let Some(x) = CameraActionT::as_afrl_cmasi_camera_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<CameraActionT> {
    fn default() -> Box<CameraActionT> { Box::new(CameraAction::default()) }
}

impl PartialEq for Box<CameraActionT> {
    fn eq(&self, other: &Box<CameraActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (CameraActionT::as_afrl_cmasi_camera_action(self.as_ref()),
             CameraActionT::as_afrl_cmasi_camera_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<CameraActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = CameraActionT::as_afrl_cmasi_camera_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<CameraActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == CameraAction::struct_info() {
            let (x, readb) = CameraAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = CameraActionT::as_afrl_cmasi_camera_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for CameraAction {
    fn as_afrl_cmasi_camera_action(&self) -> Option<&CameraAction> { Some(self) }
    fn as_mut_afrl_cmasi_camera_action(&mut self) -> Option<&mut CameraAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl ::afrl::cmasi::payload_action::PayloadActionT for CameraAction {
    fn as_afrl_cmasi_camera_action(&self) -> Option<&CameraAction> { Some(self) }
    fn as_mut_afrl_cmasi_camera_action(&mut self) -> Option<&mut CameraAction> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
}
impl CameraActionT for CameraAction {
    fn as_afrl_cmasi_camera_action(&self) -> Option<&CameraAction> { Some(self) }
    fn as_mut_afrl_cmasi_camera_action(&mut self) -> Option<&mut CameraAction> { Some(self) }
    fn horizontal_field_of_view(&self) -> f32 { self.horizontal_field_of_view }
    fn horizontal_field_of_view_mut(&mut self) -> &mut f32 { &mut self.horizontal_field_of_view }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for CameraAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> CameraAction {
            CameraAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                payload_id: Arbitrary::arbitrary(_g),
                horizontal_field_of_view: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: CameraAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: CameraAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = CameraAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
