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
pub struct MissionCommand {
    pub command_id: i64,
    pub vehicle_id: i64,
    pub vehicle_action_list: Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>>,
    pub status: ::afrl::cmasi::command_status_type::CommandStatusType,
    pub waypoint_list: Vec<Box<::afrl::cmasi::waypoint::WaypointT>>,
    pub first_waypoint: i64,
}

impl PartialEq for MissionCommand {
    fn eq(&self, _other: &MissionCommand) -> bool {
        true
        && &self.waypoint_list == &_other.waypoint_list
        && &self.first_waypoint == &_other.first_waypoint

    }
}

impl LmcpSubscription for MissionCommand {
    fn subscription() -> &'static str { "afrl.cmasi.MissionCommand" }
}

impl Struct for MissionCommand {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 36,
        }
    }
}

impl Lmcp for MissionCommand {
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
            let writeb: usize = self.waypoint_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.first_waypoint.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(MissionCommand, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == MissionCommand::struct_info() {
            let mut out: MissionCommand = Default::default();
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
                let (x, readb): (Vec<Box<::afrl::cmasi::waypoint::WaypointT>>, usize) = Lmcp::deser(r)?;
                out.waypoint_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.first_waypoint = x;
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
        size += self.waypoint_list.size();
        size += self.first_waypoint.size();

        size
    }
}

pub trait MissionCommandT: Debug + Send + ::afrl::cmasi::vehicle_action_command::VehicleActionCommandT {
    fn as_afrl_cmasi_mission_command(&self) -> Option<&MissionCommand> { None }
    fn as_mut_afrl_cmasi_mission_command(&mut self) -> Option<&mut MissionCommand> { None }
    fn waypoint_list(&self) -> &Vec<Box<::afrl::cmasi::waypoint::WaypointT>>;
    fn waypoint_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::waypoint::WaypointT>>;
    fn first_waypoint(&self) -> i64;
    fn first_waypoint_mut(&mut self) -> &mut i64;

}

impl Clone for Box<MissionCommandT> {
    fn clone(&self) -> Box<MissionCommandT> {
        if let Some(x) = MissionCommandT::as_afrl_cmasi_mission_command(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<MissionCommandT> {
    fn default() -> Box<MissionCommandT> { Box::new(MissionCommand::default()) }
}

impl PartialEq for Box<MissionCommandT> {
    fn eq(&self, other: &Box<MissionCommandT>) -> bool {
        if let (Some(x), Some(y)) =
            (MissionCommandT::as_afrl_cmasi_mission_command(self.as_ref()),
             MissionCommandT::as_afrl_cmasi_mission_command(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<MissionCommandT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = MissionCommandT::as_afrl_cmasi_mission_command(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<MissionCommandT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == MissionCommand::struct_info() {
            let (x, readb) = MissionCommand::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = MissionCommandT::as_afrl_cmasi_mission_command(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action_command::VehicleActionCommandT for MissionCommand {
    fn as_afrl_cmasi_mission_command(&self) -> Option<&MissionCommand> { Some(self) }
    fn as_mut_afrl_cmasi_mission_command(&mut self) -> Option<&mut MissionCommand> { Some(self) }
    fn command_id(&self) -> i64 { self.command_id }
    fn command_id_mut(&mut self) -> &mut i64 { &mut self.command_id }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn vehicle_action_list(&self) -> &Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>> { &self.vehicle_action_list }
    fn vehicle_action_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>> { &mut self.vehicle_action_list }
    fn status(&self) -> ::afrl::cmasi::command_status_type::CommandStatusType { self.status }
    fn status_mut(&mut self) -> &mut ::afrl::cmasi::command_status_type::CommandStatusType { &mut self.status }
}
impl MissionCommandT for MissionCommand {
    fn as_afrl_cmasi_mission_command(&self) -> Option<&MissionCommand> { Some(self) }
    fn as_mut_afrl_cmasi_mission_command(&mut self) -> Option<&mut MissionCommand> { Some(self) }
    fn waypoint_list(&self) -> &Vec<Box<::afrl::cmasi::waypoint::WaypointT>> { &self.waypoint_list }
    fn waypoint_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::waypoint::WaypointT>> { &mut self.waypoint_list }
    fn first_waypoint(&self) -> i64 { self.first_waypoint }
    fn first_waypoint_mut(&mut self) -> &mut i64 { &mut self.first_waypoint }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for MissionCommand {
        fn arbitrary<G: Gen>(_g: &mut G) -> MissionCommand {
            MissionCommand {
                command_id: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                vehicle_action_list: Vec::<::afrl::cmasi::vehicle_action::VehicleAction>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::vehicle_action::VehicleActionT>).collect(),
                status: Arbitrary::arbitrary(_g),
                waypoint_list: Vec::<::afrl::cmasi::waypoint::Waypoint>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::waypoint::WaypointT>).collect(),
                first_waypoint: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: MissionCommand) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_action_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.waypoint_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: MissionCommand) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_action_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.waypoint_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = MissionCommand::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
