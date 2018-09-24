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
pub struct PayloadAction {
    pub associated_task_list: Vec<i64>,
    pub payload_id: i64,
}

impl PartialEq for PayloadAction {
    fn eq(&self, _other: &PayloadAction) -> bool {
        true
        && &self.payload_id == &_other.payload_id

    }
}

impl LmcpSubscription for PayloadAction {
    fn subscription() -> &'static str { "afrl.cmasi.PayloadAction" }
}

impl Struct for PayloadAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 4,
        }
    }
}

impl Lmcp for PayloadAction {
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

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PayloadAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PayloadAction::struct_info() {
            let mut out: PayloadAction = Default::default();
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

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.associated_task_list.size();
        size += self.payload_id.size();

        size
    }
}

pub trait PayloadActionT: Debug + Send + ::afrl::cmasi::vehicle_action::VehicleActionT {
    fn as_afrl_cmasi_payload_action(&self) -> Option<&PayloadAction> { None }
    fn as_mut_afrl_cmasi_payload_action(&mut self) -> Option<&mut PayloadAction> { None }
    fn as_afrl_cmasi_camera_action(&self) -> Option<&::afrl::cmasi::camera_action::CameraAction> { None }
    fn as_mut_afrl_cmasi_camera_action(&mut self) -> Option<&mut ::afrl::cmasi::camera_action::CameraAction> { None }
    fn as_afrl_cmasi_gimbal_stare_action(&self) -> Option<&::afrl::cmasi::gimbal_stare_action::GimbalStareAction> { None }
    fn as_mut_afrl_cmasi_gimbal_stare_action(&mut self) -> Option<&mut ::afrl::cmasi::gimbal_stare_action::GimbalStareAction> { None }
    fn as_afrl_cmasi_gimbal_scan_action(&self) -> Option<&::afrl::cmasi::gimbal_scan_action::GimbalScanAction> { None }
    fn as_mut_afrl_cmasi_gimbal_scan_action(&mut self) -> Option<&mut ::afrl::cmasi::gimbal_scan_action::GimbalScanAction> { None }
    fn as_afrl_cmasi_gimbal_angle_action(&self) -> Option<&::afrl::cmasi::gimbal_angle_action::GimbalAngleAction> { None }
    fn as_mut_afrl_cmasi_gimbal_angle_action(&mut self) -> Option<&mut ::afrl::cmasi::gimbal_angle_action::GimbalAngleAction> { None }
    fn payload_id(&self) -> i64;
    fn payload_id_mut(&mut self) -> &mut i64;

}

impl Clone for Box<PayloadActionT> {
    fn clone(&self) -> Box<PayloadActionT> {
        if let Some(x) = PayloadActionT::as_afrl_cmasi_payload_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_camera_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PayloadActionT> {
    fn default() -> Box<PayloadActionT> { Box::new(PayloadAction::default()) }
}

impl PartialEq for Box<PayloadActionT> {
    fn eq(&self, other: &Box<PayloadActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (PayloadActionT::as_afrl_cmasi_payload_action(self.as_ref()),
             PayloadActionT::as_afrl_cmasi_payload_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadActionT::as_afrl_cmasi_camera_action(self.as_ref()),
             PayloadActionT::as_afrl_cmasi_camera_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()),
             PayloadActionT::as_afrl_cmasi_gimbal_stare_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()),
             PayloadActionT::as_afrl_cmasi_gimbal_scan_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()),
             PayloadActionT::as_afrl_cmasi_gimbal_angle_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PayloadActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PayloadActionT::as_afrl_cmasi_payload_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_camera_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PayloadActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PayloadAction::struct_info() {
            let (x, readb) = PayloadAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::camera_action::CameraAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::camera_action::CameraAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::gimbal_stare_action::GimbalStareAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::gimbal_stare_action::GimbalStareAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::gimbal_scan_action::GimbalScanAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::gimbal_scan_action::GimbalScanAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::gimbal_angle_action::GimbalAngleAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::gimbal_angle_action::GimbalAngleAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PayloadActionT::as_afrl_cmasi_payload_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_camera_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for PayloadAction {
    fn as_afrl_cmasi_payload_action(&self) -> Option<&PayloadAction> { Some(self) }
    fn as_mut_afrl_cmasi_payload_action(&mut self) -> Option<&mut PayloadAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl PayloadActionT for PayloadAction {
    fn as_afrl_cmasi_payload_action(&self) -> Option<&PayloadAction> { Some(self) }
    fn as_mut_afrl_cmasi_payload_action(&mut self) -> Option<&mut PayloadAction> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PayloadAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> PayloadAction {
            PayloadAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                payload_id: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PayloadAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PayloadAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PayloadAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
