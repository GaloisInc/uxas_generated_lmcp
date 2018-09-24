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
pub struct VehicleActionCommand {
    pub command_id: i64,
    pub vehicle_id: i64,
    pub vehicle_action_list: Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>>,
    pub status: ::afrl::cmasi::command_status_type::CommandStatusType,
}

impl PartialEq for VehicleActionCommand {
    fn eq(&self, _other: &VehicleActionCommand) -> bool {
        true
        && &self.command_id == &_other.command_id
        && &self.vehicle_id == &_other.vehicle_id
        && &self.vehicle_action_list == &_other.vehicle_action_list
        && &self.status == &_other.status

    }
}

impl LmcpSubscription for VehicleActionCommand {
    fn subscription() -> &'static str { "afrl.cmasi.VehicleActionCommand" }
}

impl Struct for VehicleActionCommand {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 47,
        }
    }
}

impl Lmcp for VehicleActionCommand {
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

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(VehicleActionCommand, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == VehicleActionCommand::struct_info() {
            let mut out: VehicleActionCommand = Default::default();
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

        size
    }
}

pub trait VehicleActionCommandT: Debug + Send  {
    fn as_afrl_cmasi_vehicle_action_command(&self) -> Option<&VehicleActionCommand> { None }
    fn as_mut_afrl_cmasi_vehicle_action_command(&mut self) -> Option<&mut VehicleActionCommand> { None }
    fn as_afrl_cmasi_follow_path_command(&self) -> Option<&::afrl::cmasi::follow_path_command::FollowPathCommand> { None }
    fn as_mut_afrl_cmasi_follow_path_command(&mut self) -> Option<&mut ::afrl::cmasi::follow_path_command::FollowPathCommand> { None }
    fn as_afrl_cmasi_mission_command(&self) -> Option<&::afrl::cmasi::mission_command::MissionCommand> { None }
    fn as_mut_afrl_cmasi_mission_command(&mut self) -> Option<&mut ::afrl::cmasi::mission_command::MissionCommand> { None }
    fn command_id(&self) -> i64;
    fn command_id_mut(&mut self) -> &mut i64;
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn vehicle_action_list(&self) -> &Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>>;
    fn vehicle_action_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>>;
    fn status(&self) -> ::afrl::cmasi::command_status_type::CommandStatusType;
    fn status_mut(&mut self) -> &mut ::afrl::cmasi::command_status_type::CommandStatusType;

}

impl Clone for Box<VehicleActionCommandT> {
    fn clone(&self) -> Box<VehicleActionCommandT> {
        if let Some(x) = VehicleActionCommandT::as_afrl_cmasi_vehicle_action_command(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionCommandT::as_afrl_cmasi_follow_path_command(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionCommandT::as_afrl_cmasi_mission_command(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<VehicleActionCommandT> {
    fn default() -> Box<VehicleActionCommandT> { Box::new(VehicleActionCommand::default()) }
}

impl PartialEq for Box<VehicleActionCommandT> {
    fn eq(&self, other: &Box<VehicleActionCommandT>) -> bool {
        if let (Some(x), Some(y)) =
            (VehicleActionCommandT::as_afrl_cmasi_vehicle_action_command(self.as_ref()),
             VehicleActionCommandT::as_afrl_cmasi_vehicle_action_command(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionCommandT::as_afrl_cmasi_follow_path_command(self.as_ref()),
             VehicleActionCommandT::as_afrl_cmasi_follow_path_command(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionCommandT::as_afrl_cmasi_mission_command(self.as_ref()),
             VehicleActionCommandT::as_afrl_cmasi_mission_command(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<VehicleActionCommandT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = VehicleActionCommandT::as_afrl_cmasi_vehicle_action_command(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionCommandT::as_afrl_cmasi_follow_path_command(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionCommandT::as_afrl_cmasi_mission_command(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<VehicleActionCommandT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == VehicleActionCommand::struct_info() {
            let (x, readb) = VehicleActionCommand::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::follow_path_command::FollowPathCommand::struct_info() {
            let (x, readb) = ::afrl::cmasi::follow_path_command::FollowPathCommand::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::mission_command::MissionCommand::struct_info() {
            let (x, readb) = ::afrl::cmasi::mission_command::MissionCommand::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = VehicleActionCommandT::as_afrl_cmasi_vehicle_action_command(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionCommandT::as_afrl_cmasi_follow_path_command(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionCommandT::as_afrl_cmasi_mission_command(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl VehicleActionCommandT for VehicleActionCommand {
    fn as_afrl_cmasi_vehicle_action_command(&self) -> Option<&VehicleActionCommand> { Some(self) }
    fn as_mut_afrl_cmasi_vehicle_action_command(&mut self) -> Option<&mut VehicleActionCommand> { Some(self) }
    fn command_id(&self) -> i64 { self.command_id }
    fn command_id_mut(&mut self) -> &mut i64 { &mut self.command_id }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn vehicle_action_list(&self) -> &Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>> { &self.vehicle_action_list }
    fn vehicle_action_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>> { &mut self.vehicle_action_list }
    fn status(&self) -> ::afrl::cmasi::command_status_type::CommandStatusType { self.status }
    fn status_mut(&mut self) -> &mut ::afrl::cmasi::command_status_type::CommandStatusType { &mut self.status }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for VehicleActionCommand {
        fn arbitrary<G: Gen>(_g: &mut G) -> VehicleActionCommand {
            VehicleActionCommand {
                command_id: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                vehicle_action_list: Vec::<::afrl::cmasi::vehicle_action::VehicleAction>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::vehicle_action::VehicleActionT>).collect(),
                status: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: VehicleActionCommand) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_action_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: VehicleActionCommand) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_action_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = VehicleActionCommand::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
