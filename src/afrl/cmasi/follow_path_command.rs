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
pub struct FollowPathCommand {
    pub command_id: i64,
    pub vehicle_id: i64,
    pub vehicle_action_list: Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>>,
    pub status: ::afrl::cmasi::command_status_type::CommandStatusType,
    pub first_waypoint: i64,
    pub waypoint_list: Vec<Box<::afrl::cmasi::path_waypoint::PathWaypointT>>,
    pub start_time: i64,
    pub stop_time: i64,
    pub repeat_mode: ::afrl::cmasi::travel_mode::TravelMode,
}

impl PartialEq for FollowPathCommand {
    fn eq(&self, _other: &FollowPathCommand) -> bool {
        true
        && &self.first_waypoint == &_other.first_waypoint
        && &self.waypoint_list == &_other.waypoint_list
        && &self.start_time == &_other.start_time
        && &self.stop_time == &_other.stop_time
        && &self.repeat_mode == &_other.repeat_mode

    }
}

impl LmcpSubscription for FollowPathCommand {
    fn subscription() -> &'static str { "afrl.cmasi.FollowPathCommand" }
}

impl Struct for FollowPathCommand {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 56,
        }
    }
}

impl Lmcp for FollowPathCommand {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.command_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_action_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.status.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.first_waypoint.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.waypoint_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_time.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.stop_time.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.repeat_mode.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(FollowPathCommand, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == FollowPathCommand::struct_info() {
            let mut out: FollowPathCommand = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.command_id = x;
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
                let (x, readb): (Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>>, usize) = Lmcp::deser(r)?;
                out.vehicle_action_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::command_status_type::CommandStatusType, usize) = Lmcp::deser(r)?;
                out.status = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.first_waypoint = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::path_waypoint::PathWaypointT>>, usize) = Lmcp::deser(r)?;
                out.waypoint_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.start_time = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.stop_time = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::travel_mode::TravelMode, usize) = Lmcp::deser(r)?;
                out.repeat_mode = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.command_id.size();
        size += self.vehicle_id.size();
        size += self.vehicle_action_list.size();
        size += self.status.size();
        size += self.first_waypoint.size();
        size += self.waypoint_list.size();
        size += self.start_time.size();
        size += self.stop_time.size();
        size += self.repeat_mode.size();

        size
    }
}

pub trait FollowPathCommandT: Debug + Send + ::afrl::cmasi::vehicle_action_command::VehicleActionCommandT {
    fn as_afrl_cmasi_follow_path_command(&self) -> Option<&FollowPathCommand> { None }
    fn as_mut_afrl_cmasi_follow_path_command(&mut self) -> Option<&mut FollowPathCommand> { None }
    fn first_waypoint(&self) -> i64;
    fn first_waypoint_mut(&mut self) -> &mut i64;
    fn waypoint_list(&self) -> &Vec<Box<::afrl::cmasi::path_waypoint::PathWaypointT>>;
    fn waypoint_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::path_waypoint::PathWaypointT>>;
    fn start_time(&self) -> i64;
    fn start_time_mut(&mut self) -> &mut i64;
    fn stop_time(&self) -> i64;
    fn stop_time_mut(&mut self) -> &mut i64;
    fn repeat_mode(&self) -> ::afrl::cmasi::travel_mode::TravelMode;
    fn repeat_mode_mut(&mut self) -> &mut ::afrl::cmasi::travel_mode::TravelMode;

}

impl Clone for Box<FollowPathCommandT> {
    fn clone(&self) -> Box<FollowPathCommandT> {
        if let Some(x) = FollowPathCommandT::as_afrl_cmasi_follow_path_command(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<FollowPathCommandT> {
    fn default() -> Box<FollowPathCommandT> { Box::new(FollowPathCommand::default()) }
}

impl PartialEq for Box<FollowPathCommandT> {
    fn eq(&self, other: &Box<FollowPathCommandT>) -> bool {
        if let (Some(x), Some(y)) =
            (FollowPathCommandT::as_afrl_cmasi_follow_path_command(self.as_ref()),
             FollowPathCommandT::as_afrl_cmasi_follow_path_command(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<FollowPathCommandT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = FollowPathCommandT::as_afrl_cmasi_follow_path_command(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<FollowPathCommandT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == FollowPathCommand::struct_info() {
            let (x, readb) = FollowPathCommand::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = FollowPathCommandT::as_afrl_cmasi_follow_path_command(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action_command::VehicleActionCommandT for FollowPathCommand {
    fn as_afrl_cmasi_follow_path_command(&self) -> Option<&FollowPathCommand> { Some(self) }
    fn as_mut_afrl_cmasi_follow_path_command(&mut self) -> Option<&mut FollowPathCommand> { Some(self) }
    fn command_id(&self) -> i64 { self.command_id }
    fn command_id_mut(&mut self) -> &mut i64 { &mut self.command_id }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn vehicle_action_list(&self) -> &Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>> { &self.vehicle_action_list }
    fn vehicle_action_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>> { &mut self.vehicle_action_list }
    fn status(&self) -> ::afrl::cmasi::command_status_type::CommandStatusType { self.status }
    fn status_mut(&mut self) -> &mut ::afrl::cmasi::command_status_type::CommandStatusType { &mut self.status }
}
impl FollowPathCommandT for FollowPathCommand {
    fn as_afrl_cmasi_follow_path_command(&self) -> Option<&FollowPathCommand> { Some(self) }
    fn as_mut_afrl_cmasi_follow_path_command(&mut self) -> Option<&mut FollowPathCommand> { Some(self) }
    fn first_waypoint(&self) -> i64 { self.first_waypoint }
    fn first_waypoint_mut(&mut self) -> &mut i64 { &mut self.first_waypoint }
    fn waypoint_list(&self) -> &Vec<Box<::afrl::cmasi::path_waypoint::PathWaypointT>> { &self.waypoint_list }
    fn waypoint_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::path_waypoint::PathWaypointT>> { &mut self.waypoint_list }
    fn start_time(&self) -> i64 { self.start_time }
    fn start_time_mut(&mut self) -> &mut i64 { &mut self.start_time }
    fn stop_time(&self) -> i64 { self.stop_time }
    fn stop_time_mut(&mut self) -> &mut i64 { &mut self.stop_time }
    fn repeat_mode(&self) -> ::afrl::cmasi::travel_mode::TravelMode { self.repeat_mode }
    fn repeat_mode_mut(&mut self) -> &mut ::afrl::cmasi::travel_mode::TravelMode { &mut self.repeat_mode }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for FollowPathCommand {
        fn arbitrary<G: Gen>(_g: &mut G) -> FollowPathCommand {
            FollowPathCommand {
                command_id: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                vehicle_action_list: Vec::<::afrl::cmasi::vehicle_action::VehicleAction>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::vehicle_action::VehicleActionT>).collect(),
                status: Arbitrary::arbitrary(_g),
                first_waypoint: Arbitrary::arbitrary(_g),
                waypoint_list: Vec::<::afrl::cmasi::path_waypoint::PathWaypoint>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::path_waypoint::PathWaypointT>).collect(),
                start_time: Arbitrary::arbitrary(_g),
                stop_time: Arbitrary::arbitrary(_g),
                repeat_mode: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: FollowPathCommand) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_action_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.waypoint_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: FollowPathCommand) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_action_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.waypoint_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = FollowPathCommand::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
