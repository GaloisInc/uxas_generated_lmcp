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
pub struct NavigationAction {
    pub associated_task_list: Vec<i64>,
}

impl PartialEq for NavigationAction {
    fn eq(&self, _other: &NavigationAction) -> bool {
        true

    }
}

impl LmcpSubscription for NavigationAction {
    fn subscription() -> &'static str { "afrl.cmasi.NavigationAction" }
}

impl Struct for NavigationAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 32,
        }
    }
}

impl Lmcp for NavigationAction {
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

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(NavigationAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == NavigationAction::struct_info() {
            let mut out: NavigationAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
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

        size
    }
}

pub trait NavigationActionT: Debug + Send + ::afrl::cmasi::vehicle_action::VehicleActionT {
    fn as_afrl_cmasi_navigation_action(&self) -> Option<&NavigationAction> { None }
    fn as_mut_afrl_cmasi_navigation_action(&mut self) -> Option<&mut NavigationAction> { None }
    fn as_afrl_cmasi_loiter_action(&self) -> Option<&::afrl::cmasi::loiter_action::LoiterAction> { None }
    fn as_mut_afrl_cmasi_loiter_action(&mut self) -> Option<&mut ::afrl::cmasi::loiter_action::LoiterAction> { None }
    fn as_afrl_cmasi_go_to_waypoint_action(&self) -> Option<&::afrl::cmasi::go_to_waypoint_action::GoToWaypointAction> { None }
    fn as_mut_afrl_cmasi_go_to_waypoint_action(&mut self) -> Option<&mut ::afrl::cmasi::go_to_waypoint_action::GoToWaypointAction> { None }
    fn as_afrl_cmasi_flight_director_action(&self) -> Option<&::afrl::cmasi::flight_director_action::FlightDirectorAction> { None }
    fn as_mut_afrl_cmasi_flight_director_action(&mut self) -> Option<&mut ::afrl::cmasi::flight_director_action::FlightDirectorAction> { None }

}

impl Clone for Box<NavigationActionT> {
    fn clone(&self) -> Box<NavigationActionT> {
        if let Some(x) = NavigationActionT::as_afrl_cmasi_navigation_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = NavigationActionT::as_afrl_cmasi_loiter_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = NavigationActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = NavigationActionT::as_afrl_cmasi_flight_director_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<NavigationActionT> {
    fn default() -> Box<NavigationActionT> { Box::new(NavigationAction::default()) }
}

impl PartialEq for Box<NavigationActionT> {
    fn eq(&self, other: &Box<NavigationActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (NavigationActionT::as_afrl_cmasi_navigation_action(self.as_ref()),
             NavigationActionT::as_afrl_cmasi_navigation_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (NavigationActionT::as_afrl_cmasi_loiter_action(self.as_ref()),
             NavigationActionT::as_afrl_cmasi_loiter_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (NavigationActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()),
             NavigationActionT::as_afrl_cmasi_go_to_waypoint_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (NavigationActionT::as_afrl_cmasi_flight_director_action(self.as_ref()),
             NavigationActionT::as_afrl_cmasi_flight_director_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<NavigationActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = NavigationActionT::as_afrl_cmasi_navigation_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = NavigationActionT::as_afrl_cmasi_loiter_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = NavigationActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = NavigationActionT::as_afrl_cmasi_flight_director_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<NavigationActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == NavigationAction::struct_info() {
            let (x, readb) = NavigationAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::loiter_action::LoiterAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::loiter_action::LoiterAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::go_to_waypoint_action::GoToWaypointAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::go_to_waypoint_action::GoToWaypointAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::flight_director_action::FlightDirectorAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::flight_director_action::FlightDirectorAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = NavigationActionT::as_afrl_cmasi_navigation_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = NavigationActionT::as_afrl_cmasi_loiter_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = NavigationActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = NavigationActionT::as_afrl_cmasi_flight_director_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for NavigationAction {
    fn as_afrl_cmasi_navigation_action(&self) -> Option<&NavigationAction> { Some(self) }
    fn as_mut_afrl_cmasi_navigation_action(&mut self) -> Option<&mut NavigationAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl NavigationActionT for NavigationAction {
    fn as_afrl_cmasi_navigation_action(&self) -> Option<&NavigationAction> { Some(self) }
    fn as_mut_afrl_cmasi_navigation_action(&mut self) -> Option<&mut NavigationAction> { Some(self) }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for NavigationAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> NavigationAction {
            NavigationAction {
                associated_task_list: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: NavigationAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: NavigationAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = NavigationAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
