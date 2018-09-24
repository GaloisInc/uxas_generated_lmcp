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
pub struct TrackEntityAction {
    pub associated_task_list: Vec<i64>,
    pub entity_id: u32,
    pub sensor_id: u32,
    pub return_to_waypoint: u32,
}

impl PartialEq for TrackEntityAction {
    fn eq(&self, _other: &TrackEntityAction) -> bool {
        true
        && &self.entity_id == &_other.entity_id
        && &self.sensor_id == &_other.sensor_id
        && &self.return_to_waypoint == &_other.return_to_waypoint

    }
}

impl LmcpSubscription for TrackEntityAction {
    fn subscription() -> &'static str { "afrl.cmasi.perceive.TrackEntityAction" }
}

impl Struct for TrackEntityAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5784119745305990725u64,
            version: 1,
            struct_ty: 2,
        }
    }
}

impl Lmcp for TrackEntityAction {
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
            let writeb: usize = self.entity_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.sensor_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.return_to_waypoint.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TrackEntityAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TrackEntityAction::struct_info() {
            let mut out: TrackEntityAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u32, usize) = Lmcp::deser(r)?;
                out.entity_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u32, usize) = Lmcp::deser(r)?;
                out.sensor_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u32, usize) = Lmcp::deser(r)?;
                out.return_to_waypoint = x;
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
        size += self.entity_id.size();
        size += self.sensor_id.size();
        size += self.return_to_waypoint.size();

        size
    }
}

pub trait TrackEntityActionT: Debug + Send + ::afrl::cmasi::vehicle_action::VehicleActionT {
    fn as_afrl_cmasi_perceive_track_entity_action(&self) -> Option<&TrackEntityAction> { None }
    fn as_mut_afrl_cmasi_perceive_track_entity_action(&mut self) -> Option<&mut TrackEntityAction> { None }
    fn entity_id(&self) -> u32;
    fn entity_id_mut(&mut self) -> &mut u32;
    fn sensor_id(&self) -> u32;
    fn sensor_id_mut(&mut self) -> &mut u32;
    fn return_to_waypoint(&self) -> u32;
    fn return_to_waypoint_mut(&mut self) -> &mut u32;

}

impl Clone for Box<TrackEntityActionT> {
    fn clone(&self) -> Box<TrackEntityActionT> {
        if let Some(x) = TrackEntityActionT::as_afrl_cmasi_perceive_track_entity_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TrackEntityActionT> {
    fn default() -> Box<TrackEntityActionT> { Box::new(TrackEntityAction::default()) }
}

impl PartialEq for Box<TrackEntityActionT> {
    fn eq(&self, other: &Box<TrackEntityActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (TrackEntityActionT::as_afrl_cmasi_perceive_track_entity_action(self.as_ref()),
             TrackEntityActionT::as_afrl_cmasi_perceive_track_entity_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TrackEntityActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TrackEntityActionT::as_afrl_cmasi_perceive_track_entity_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TrackEntityActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TrackEntityAction::struct_info() {
            let (x, readb) = TrackEntityAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TrackEntityActionT::as_afrl_cmasi_perceive_track_entity_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for TrackEntityAction {
    fn as_afrl_cmasi_perceive_track_entity_action(&self) -> Option<&TrackEntityAction> { Some(self) }
    fn as_mut_afrl_cmasi_perceive_track_entity_action(&mut self) -> Option<&mut TrackEntityAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl TrackEntityActionT for TrackEntityAction {
    fn as_afrl_cmasi_perceive_track_entity_action(&self) -> Option<&TrackEntityAction> { Some(self) }
    fn as_mut_afrl_cmasi_perceive_track_entity_action(&mut self) -> Option<&mut TrackEntityAction> { Some(self) }
    fn entity_id(&self) -> u32 { self.entity_id }
    fn entity_id_mut(&mut self) -> &mut u32 { &mut self.entity_id }
    fn sensor_id(&self) -> u32 { self.sensor_id }
    fn sensor_id_mut(&mut self) -> &mut u32 { &mut self.sensor_id }
    fn return_to_waypoint(&self) -> u32 { self.return_to_waypoint }
    fn return_to_waypoint_mut(&mut self) -> &mut u32 { &mut self.return_to_waypoint }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TrackEntityAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> TrackEntityAction {
            TrackEntityAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                entity_id: Arbitrary::arbitrary(_g),
                sensor_id: Arbitrary::arbitrary(_g),
                return_to_waypoint: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TrackEntityAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TrackEntityAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TrackEntityAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
