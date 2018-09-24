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
pub struct SpeedOverrideAction {
    pub associated_task_list: Vec<i64>,
    pub vehicle_id: i64,
    pub speed: f32,
}

impl PartialEq for SpeedOverrideAction {
    fn eq(&self, _other: &SpeedOverrideAction) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id
        && &self.speed == &_other.speed

    }
}

impl LmcpSubscription for SpeedOverrideAction {
    fn subscription() -> &'static str { "uxas.messages.uxnative.SpeedOverrideAction" }
}

impl Struct for SpeedOverrideAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 17,
        }
    }
}

impl Lmcp for SpeedOverrideAction {
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
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.speed.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SpeedOverrideAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SpeedOverrideAction::struct_info() {
            let mut out: SpeedOverrideAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.speed = x;
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
        size += self.vehicle_id.size();
        size += self.speed.size();

        size
    }
}

pub trait SpeedOverrideActionT: Debug + Send + ::afrl::cmasi::vehicle_action::VehicleActionT {
    fn as_uxas_messages_uxnative_speed_override_action(&self) -> Option<&SpeedOverrideAction> { None }
    fn as_mut_uxas_messages_uxnative_speed_override_action(&mut self) -> Option<&mut SpeedOverrideAction> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn speed(&self) -> f32;
    fn speed_mut(&mut self) -> &mut f32;

}

impl Clone for Box<SpeedOverrideActionT> {
    fn clone(&self) -> Box<SpeedOverrideActionT> {
        if let Some(x) = SpeedOverrideActionT::as_uxas_messages_uxnative_speed_override_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SpeedOverrideActionT> {
    fn default() -> Box<SpeedOverrideActionT> { Box::new(SpeedOverrideAction::default()) }
}

impl PartialEq for Box<SpeedOverrideActionT> {
    fn eq(&self, other: &Box<SpeedOverrideActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (SpeedOverrideActionT::as_uxas_messages_uxnative_speed_override_action(self.as_ref()),
             SpeedOverrideActionT::as_uxas_messages_uxnative_speed_override_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SpeedOverrideActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SpeedOverrideActionT::as_uxas_messages_uxnative_speed_override_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SpeedOverrideActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SpeedOverrideAction::struct_info() {
            let (x, readb) = SpeedOverrideAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SpeedOverrideActionT::as_uxas_messages_uxnative_speed_override_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for SpeedOverrideAction {
    fn as_uxas_messages_uxnative_speed_override_action(&self) -> Option<&SpeedOverrideAction> { Some(self) }
    fn as_mut_uxas_messages_uxnative_speed_override_action(&mut self) -> Option<&mut SpeedOverrideAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl SpeedOverrideActionT for SpeedOverrideAction {
    fn as_uxas_messages_uxnative_speed_override_action(&self) -> Option<&SpeedOverrideAction> { Some(self) }
    fn as_mut_uxas_messages_uxnative_speed_override_action(&mut self) -> Option<&mut SpeedOverrideAction> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn speed(&self) -> f32 { self.speed }
    fn speed_mut(&mut self) -> &mut f32 { &mut self.speed }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SpeedOverrideAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> SpeedOverrideAction {
            SpeedOverrideAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                speed: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SpeedOverrideAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SpeedOverrideAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SpeedOverrideAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
