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
pub struct CreateNewService {
    pub service_id: i64,
    pub xml_configuration: Vec<u8>,
    pub entity_configurations: Vec<Box<::afrl::cmasi::entity_configuration::EntityConfigurationT>>,
    pub entity_states: Vec<Box<::afrl::cmasi::entity_state::EntityStateT>>,
    pub mission_commands: Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>>,
    pub areas: Vec<Box<::afrl::impact::area_of_interest::AreaOfInterestT>>,
    pub lines: Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>>,
    pub points: Vec<Box<::afrl::impact::point_of_interest::PointOfInterestT>>,
}

impl PartialEq for CreateNewService {
    fn eq(&self, _other: &CreateNewService) -> bool {
        true
        && &self.service_id == &_other.service_id
        && &self.xml_configuration == &_other.xml_configuration
        && &self.entity_configurations == &_other.entity_configurations
        && &self.entity_states == &_other.entity_states
        && &self.mission_commands == &_other.mission_commands
        && &self.areas == &_other.areas
        && &self.lines == &_other.lines
        && &self.points == &_other.points

    }
}

impl LmcpSubscription for CreateNewService {
    fn subscription() -> &'static str { "uxas.messages.uxnative.CreateNewService" }
}

impl Struct for CreateNewService {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 3,
        }
    }
}

impl Lmcp for CreateNewService {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.service_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.xml_configuration.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entity_configurations.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entity_states.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.mission_commands.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.areas.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.lines.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.points.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(CreateNewService, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == CreateNewService::struct_info() {
            let mut out: CreateNewService = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.service_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.xml_configuration = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::entity_configuration::EntityConfigurationT>>, usize) = Lmcp::deser(r)?;
                out.entity_configurations = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::entity_state::EntityStateT>>, usize) = Lmcp::deser(r)?;
                out.entity_states = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>>, usize) = Lmcp::deser(r)?;
                out.mission_commands = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::impact::area_of_interest::AreaOfInterestT>>, usize) = Lmcp::deser(r)?;
                out.areas = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>>, usize) = Lmcp::deser(r)?;
                out.lines = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::impact::point_of_interest::PointOfInterestT>>, usize) = Lmcp::deser(r)?;
                out.points = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.service_id.size();
        size += self.xml_configuration.size();
        size += self.entity_configurations.size();
        size += self.entity_states.size();
        size += self.mission_commands.size();
        size += self.areas.size();
        size += self.lines.size();
        size += self.points.size();

        size
    }
}

pub trait CreateNewServiceT: Debug + Send  {
    fn as_uxas_messages_uxnative_create_new_service(&self) -> Option<&CreateNewService> { None }
    fn as_mut_uxas_messages_uxnative_create_new_service(&mut self) -> Option<&mut CreateNewService> { None }
    fn service_id(&self) -> i64;
    fn service_id_mut(&mut self) -> &mut i64;
    fn xml_configuration(&self) -> &Vec<u8>;
    fn xml_configuration_mut(&mut self) -> &mut Vec<u8>;
    fn entity_configurations(&self) -> &Vec<Box<::afrl::cmasi::entity_configuration::EntityConfigurationT>>;
    fn entity_configurations_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::entity_configuration::EntityConfigurationT>>;
    fn entity_states(&self) -> &Vec<Box<::afrl::cmasi::entity_state::EntityStateT>>;
    fn entity_states_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::entity_state::EntityStateT>>;
    fn mission_commands(&self) -> &Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>>;
    fn mission_commands_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>>;
    fn areas(&self) -> &Vec<Box<::afrl::impact::area_of_interest::AreaOfInterestT>>;
    fn areas_mut(&mut self) -> &mut Vec<Box<::afrl::impact::area_of_interest::AreaOfInterestT>>;
    fn lines(&self) -> &Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>>;
    fn lines_mut(&mut self) -> &mut Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>>;
    fn points(&self) -> &Vec<Box<::afrl::impact::point_of_interest::PointOfInterestT>>;
    fn points_mut(&mut self) -> &mut Vec<Box<::afrl::impact::point_of_interest::PointOfInterestT>>;

}

impl Clone for Box<CreateNewServiceT> {
    fn clone(&self) -> Box<CreateNewServiceT> {
        if let Some(x) = CreateNewServiceT::as_uxas_messages_uxnative_create_new_service(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<CreateNewServiceT> {
    fn default() -> Box<CreateNewServiceT> { Box::new(CreateNewService::default()) }
}

impl PartialEq for Box<CreateNewServiceT> {
    fn eq(&self, other: &Box<CreateNewServiceT>) -> bool {
        if let (Some(x), Some(y)) =
            (CreateNewServiceT::as_uxas_messages_uxnative_create_new_service(self.as_ref()),
             CreateNewServiceT::as_uxas_messages_uxnative_create_new_service(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<CreateNewServiceT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = CreateNewServiceT::as_uxas_messages_uxnative_create_new_service(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<CreateNewServiceT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == CreateNewService::struct_info() {
            let (x, readb) = CreateNewService::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = CreateNewServiceT::as_uxas_messages_uxnative_create_new_service(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl CreateNewServiceT for CreateNewService {
    fn as_uxas_messages_uxnative_create_new_service(&self) -> Option<&CreateNewService> { Some(self) }
    fn as_mut_uxas_messages_uxnative_create_new_service(&mut self) -> Option<&mut CreateNewService> { Some(self) }
    fn service_id(&self) -> i64 { self.service_id }
    fn service_id_mut(&mut self) -> &mut i64 { &mut self.service_id }
    fn xml_configuration(&self) -> &Vec<u8> { &self.xml_configuration }
    fn xml_configuration_mut(&mut self) -> &mut Vec<u8> { &mut self.xml_configuration }
    fn entity_configurations(&self) -> &Vec<Box<::afrl::cmasi::entity_configuration::EntityConfigurationT>> { &self.entity_configurations }
    fn entity_configurations_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::entity_configuration::EntityConfigurationT>> { &mut self.entity_configurations }
    fn entity_states(&self) -> &Vec<Box<::afrl::cmasi::entity_state::EntityStateT>> { &self.entity_states }
    fn entity_states_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::entity_state::EntityStateT>> { &mut self.entity_states }
    fn mission_commands(&self) -> &Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>> { &self.mission_commands }
    fn mission_commands_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::mission_command::MissionCommandT>> { &mut self.mission_commands }
    fn areas(&self) -> &Vec<Box<::afrl::impact::area_of_interest::AreaOfInterestT>> { &self.areas }
    fn areas_mut(&mut self) -> &mut Vec<Box<::afrl::impact::area_of_interest::AreaOfInterestT>> { &mut self.areas }
    fn lines(&self) -> &Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>> { &self.lines }
    fn lines_mut(&mut self) -> &mut Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>> { &mut self.lines }
    fn points(&self) -> &Vec<Box<::afrl::impact::point_of_interest::PointOfInterestT>> { &self.points }
    fn points_mut(&mut self) -> &mut Vec<Box<::afrl::impact::point_of_interest::PointOfInterestT>> { &mut self.points }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for CreateNewService {
        fn arbitrary<G: Gen>(_g: &mut G) -> CreateNewService {
            CreateNewService {
                service_id: Arbitrary::arbitrary(_g),
                xml_configuration: Arbitrary::arbitrary(_g),
                entity_configurations: Vec::<::afrl::cmasi::entity_configuration::EntityConfiguration>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::entity_configuration::EntityConfigurationT>).collect(),
                entity_states: Vec::<::afrl::cmasi::entity_state::EntityState>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::entity_state::EntityStateT>).collect(),
                mission_commands: Vec::<::afrl::cmasi::mission_command::MissionCommand>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::mission_command::MissionCommandT>).collect(),
                areas: Vec::<::afrl::impact::area_of_interest::AreaOfInterest>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::impact::area_of_interest::AreaOfInterestT>).collect(),
                lines: Vec::<::afrl::impact::line_of_interest::LineOfInterest>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::impact::line_of_interest::LineOfInterestT>).collect(),
                points: Vec::<::afrl::impact::point_of_interest::PointOfInterest>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::impact::point_of_interest::PointOfInterestT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: CreateNewService) -> Result<TestResult, Error> {
            use std::u16;
            if x.entity_configurations.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.entity_states.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.mission_commands.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.areas.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.lines.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.points.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: CreateNewService) -> Result<TestResult, Error> {
            use std::u16;
            if x.entity_configurations.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.entity_states.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.mission_commands.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.areas.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.lines.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.points.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = CreateNewService::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
