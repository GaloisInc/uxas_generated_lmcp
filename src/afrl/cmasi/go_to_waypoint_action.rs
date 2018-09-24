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
pub struct GoToWaypointAction {
    pub associated_task_list: Vec<i64>,
    pub waypoint_number: i64,
}

impl PartialEq for GoToWaypointAction {
    fn eq(&self, _other: &GoToWaypointAction) -> bool {
        true
        && &self.waypoint_number == &_other.waypoint_number

    }
}

impl LmcpSubscription for GoToWaypointAction {
    fn subscription() -> &'static str { "afrl.cmasi.GoToWaypointAction" }
}

impl Struct for GoToWaypointAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 28,
        }
    }
}

impl Lmcp for GoToWaypointAction {
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
            let writeb: usize = self.waypoint_number.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(GoToWaypointAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == GoToWaypointAction::struct_info() {
            let mut out: GoToWaypointAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.waypoint_number = x;
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
        size += self.waypoint_number.size();

        size
    }
}

pub trait GoToWaypointActionT: Debug + Send + ::afrl::cmasi::navigation_action::NavigationActionT {
    fn as_afrl_cmasi_go_to_waypoint_action(&self) -> Option<&GoToWaypointAction> { None }
    fn as_mut_afrl_cmasi_go_to_waypoint_action(&mut self) -> Option<&mut GoToWaypointAction> { None }
    fn waypoint_number(&self) -> i64;
    fn waypoint_number_mut(&mut self) -> &mut i64;

}

impl Clone for Box<GoToWaypointActionT> {
    fn clone(&self) -> Box<GoToWaypointActionT> {
        if let Some(x) = GoToWaypointActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<GoToWaypointActionT> {
    fn default() -> Box<GoToWaypointActionT> { Box::new(GoToWaypointAction::default()) }
}

impl PartialEq for Box<GoToWaypointActionT> {
    fn eq(&self, other: &Box<GoToWaypointActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (GoToWaypointActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()),
             GoToWaypointActionT::as_afrl_cmasi_go_to_waypoint_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<GoToWaypointActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = GoToWaypointActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<GoToWaypointActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == GoToWaypointAction::struct_info() {
            let (x, readb) = GoToWaypointAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = GoToWaypointActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for GoToWaypointAction {
    fn as_afrl_cmasi_go_to_waypoint_action(&self) -> Option<&GoToWaypointAction> { Some(self) }
    fn as_mut_afrl_cmasi_go_to_waypoint_action(&mut self) -> Option<&mut GoToWaypointAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl ::afrl::cmasi::navigation_action::NavigationActionT for GoToWaypointAction {
    fn as_afrl_cmasi_go_to_waypoint_action(&self) -> Option<&GoToWaypointAction> { Some(self) }
    fn as_mut_afrl_cmasi_go_to_waypoint_action(&mut self) -> Option<&mut GoToWaypointAction> { Some(self) }
}
impl GoToWaypointActionT for GoToWaypointAction {
    fn as_afrl_cmasi_go_to_waypoint_action(&self) -> Option<&GoToWaypointAction> { Some(self) }
    fn as_mut_afrl_cmasi_go_to_waypoint_action(&mut self) -> Option<&mut GoToWaypointAction> { Some(self) }
    fn waypoint_number(&self) -> i64 { self.waypoint_number }
    fn waypoint_number_mut(&mut self) -> &mut i64 { &mut self.waypoint_number }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for GoToWaypointAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> GoToWaypointAction {
            GoToWaypointAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                waypoint_number: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: GoToWaypointAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: GoToWaypointAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = GoToWaypointAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
