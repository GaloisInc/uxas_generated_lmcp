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
pub struct AutomationResponse {
    pub mission_command_list: Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>>,
    pub vehicle_command_list: Vec<Box<::afrl::cmasi::vehicle_action_command::VehicleActionCommandT>>,
    pub info: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
}

impl PartialEq for AutomationResponse {
    fn eq(&self, _other: &AutomationResponse) -> bool {
        true
        && &self.mission_command_list == &_other.mission_command_list
        && &self.vehicle_command_list == &_other.vehicle_command_list
        && &self.info == &_other.info

    }
}

impl LmcpSubscription for AutomationResponse {
    fn subscription() -> &'static str { "afrl.cmasi.AutomationResponse" }
}

impl Struct for AutomationResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 51,
        }
    }
}

impl Lmcp for AutomationResponse {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.mission_command_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_command_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.info.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(AutomationResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == AutomationResponse::struct_info() {
            let mut out: AutomationResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>>, usize) = Lmcp::deser(r)?;
                out.mission_command_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::vehicle_action_command::VehicleActionCommandT>>, usize) = Lmcp::deser(r)?;
                out.vehicle_command_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>, usize) = Lmcp::deser(r)?;
                out.info = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.mission_command_list.size();
        size += self.vehicle_command_list.size();
        size += self.info.size();

        size
    }
}

pub trait AutomationResponseT: Debug + Send  {
    fn as_afrl_cmasi_automation_response(&self) -> Option<&AutomationResponse> { None }
    fn as_mut_afrl_cmasi_automation_response(&mut self) -> Option<&mut AutomationResponse> { None }
    fn mission_command_list(&self) -> &Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>>;
    fn mission_command_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>>;
    fn vehicle_command_list(&self) -> &Vec<Box<::afrl::cmasi::vehicle_action_command::VehicleActionCommandT>>;
    fn vehicle_command_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::vehicle_action_command::VehicleActionCommandT>>;
    fn info(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn info_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;

}

impl Clone for Box<AutomationResponseT> {
    fn clone(&self) -> Box<AutomationResponseT> {
        if let Some(x) = AutomationResponseT::as_afrl_cmasi_automation_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<AutomationResponseT> {
    fn default() -> Box<AutomationResponseT> { Box::new(AutomationResponse::default()) }
}

impl PartialEq for Box<AutomationResponseT> {
    fn eq(&self, other: &Box<AutomationResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (AutomationResponseT::as_afrl_cmasi_automation_response(self.as_ref()),
             AutomationResponseT::as_afrl_cmasi_automation_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<AutomationResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = AutomationResponseT::as_afrl_cmasi_automation_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<AutomationResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == AutomationResponse::struct_info() {
            let (x, readb) = AutomationResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = AutomationResponseT::as_afrl_cmasi_automation_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl AutomationResponseT for AutomationResponse {
    fn as_afrl_cmasi_automation_response(&self) -> Option<&AutomationResponse> { Some(self) }
    fn as_mut_afrl_cmasi_automation_response(&mut self) -> Option<&mut AutomationResponse> { Some(self) }
    fn mission_command_list(&self) -> &Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>> { &self.mission_command_list }
    fn mission_command_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>> { &mut self.mission_command_list }
    fn vehicle_command_list(&self) -> &Vec<Box<::afrl::cmasi::vehicle_action_command::VehicleActionCommandT>> { &self.vehicle_command_list }
    fn vehicle_command_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::vehicle_action_command::VehicleActionCommandT>> { &mut self.vehicle_command_list }
    fn info(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.info }
    fn info_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.info }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AutomationResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> AutomationResponse {
            AutomationResponse {
                mission_command_list: Vec::<::afrl::cmasi::mission_command::MissionCommand>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::mission_command::MissionCommandT>).collect(),
                vehicle_command_list: Vec::<::afrl::cmasi::vehicle_action_command::VehicleActionCommand>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::vehicle_action_command::VehicleActionCommandT>).collect(),
                info: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: AutomationResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.mission_command_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.vehicle_command_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AutomationResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.mission_command_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.vehicle_command_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AutomationResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
