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
pub struct RadioTowerConfiguration {
    pub id: i64,
    pub affiliation: Vec<u8>,
    pub entity_type: Vec<u8>,
    pub label: Vec<u8>,
    pub nominal_speed: f32,
    pub nominal_altitude: f32,
    pub nominal_altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
    pub payload_configuration_list: Vec<Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>>,
    pub info: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub position: Box<::afrl::cmasi::location3d::Location3DT>,
    pub range: f32,
    pub enabled: bool,
}

impl PartialEq for RadioTowerConfiguration {
    fn eq(&self, _other: &RadioTowerConfiguration) -> bool {
        true
        && &self.position == &_other.position
        && &self.range == &_other.range
        && &self.enabled == &_other.enabled

    }
}

impl LmcpSubscription for RadioTowerConfiguration {
    fn subscription() -> &'static str { "afrl.impact.RadioTowerConfiguration" }
}

impl Struct for RadioTowerConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 3,
        }
    }
}

impl Lmcp for RadioTowerConfiguration {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.affiliation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entity_type.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.label.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.nominal_speed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.nominal_altitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.nominal_altitude_type.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.payload_configuration_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.info.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.position.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.range.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.enabled.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RadioTowerConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RadioTowerConfiguration::struct_info() {
            let mut out: RadioTowerConfiguration = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.affiliation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.entity_type = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.label = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.nominal_speed = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.nominal_altitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::altitude_type::AltitudeType, usize) = Lmcp::deser(r)?;
                out.nominal_altitude_type = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>>, usize) = Lmcp::deser(r)?;
                out.payload_configuration_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>, usize) = Lmcp::deser(r)?;
                out.info = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.position = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.range = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.enabled = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.id.size();
        size += self.affiliation.size();
        size += self.entity_type.size();
        size += self.label.size();
        size += self.nominal_speed.size();
        size += self.nominal_altitude.size();
        size += self.nominal_altitude_type.size();
        size += self.payload_configuration_list.size();
        size += self.info.size();
        size += self.position.size();
        size += self.range.size();
        size += self.enabled.size();

        size
    }
}

pub trait RadioTowerConfigurationT: Debug + Send + ::afrl::cmasi::entity_configuration::EntityConfigurationT {
    fn as_afrl_impact_radio_tower_configuration(&self) -> Option<&RadioTowerConfiguration> { None }
    fn as_mut_afrl_impact_radio_tower_configuration(&mut self) -> Option<&mut RadioTowerConfiguration> { None }
    fn position(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn position_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn range(&self) -> f32;
    fn range_mut(&mut self) -> &mut f32;
    fn enabled(&self) -> bool;
    fn enabled_mut(&mut self) -> &mut bool;

}

impl Clone for Box<RadioTowerConfigurationT> {
    fn clone(&self) -> Box<RadioTowerConfigurationT> {
        if let Some(x) = RadioTowerConfigurationT::as_afrl_impact_radio_tower_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RadioTowerConfigurationT> {
    fn default() -> Box<RadioTowerConfigurationT> { Box::new(RadioTowerConfiguration::default()) }
}

impl PartialEq for Box<RadioTowerConfigurationT> {
    fn eq(&self, other: &Box<RadioTowerConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (RadioTowerConfigurationT::as_afrl_impact_radio_tower_configuration(self.as_ref()),
             RadioTowerConfigurationT::as_afrl_impact_radio_tower_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RadioTowerConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RadioTowerConfigurationT::as_afrl_impact_radio_tower_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RadioTowerConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RadioTowerConfiguration::struct_info() {
            let (x, readb) = RadioTowerConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RadioTowerConfigurationT::as_afrl_impact_radio_tower_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::entity_configuration::EntityConfigurationT for RadioTowerConfiguration {
    fn as_afrl_impact_radio_tower_configuration(&self) -> Option<&RadioTowerConfiguration> { Some(self) }
    fn as_mut_afrl_impact_radio_tower_configuration(&mut self) -> Option<&mut RadioTowerConfiguration> { Some(self) }
    fn id(&self) -> i64 { self.id }
    fn id_mut(&mut self) -> &mut i64 { &mut self.id }
    fn affiliation(&self) -> &Vec<u8> { &self.affiliation }
    fn affiliation_mut(&mut self) -> &mut Vec<u8> { &mut self.affiliation }
    fn entity_type(&self) -> &Vec<u8> { &self.entity_type }
    fn entity_type_mut(&mut self) -> &mut Vec<u8> { &mut self.entity_type }
    fn label(&self) -> &Vec<u8> { &self.label }
    fn label_mut(&mut self) -> &mut Vec<u8> { &mut self.label }
    fn nominal_speed(&self) -> f32 { self.nominal_speed }
    fn nominal_speed_mut(&mut self) -> &mut f32 { &mut self.nominal_speed }
    fn nominal_altitude(&self) -> f32 { self.nominal_altitude }
    fn nominal_altitude_mut(&mut self) -> &mut f32 { &mut self.nominal_altitude }
    fn nominal_altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType { self.nominal_altitude_type }
    fn nominal_altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType { &mut self.nominal_altitude_type }
    fn payload_configuration_list(&self) -> &Vec<Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>> { &self.payload_configuration_list }
    fn payload_configuration_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>> { &mut self.payload_configuration_list }
    fn info(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.info }
    fn info_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.info }
}
impl RadioTowerConfigurationT for RadioTowerConfiguration {
    fn as_afrl_impact_radio_tower_configuration(&self) -> Option<&RadioTowerConfiguration> { Some(self) }
    fn as_mut_afrl_impact_radio_tower_configuration(&mut self) -> Option<&mut RadioTowerConfiguration> { Some(self) }
    fn position(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.position }
    fn position_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.position }
    fn range(&self) -> f32 { self.range }
    fn range_mut(&mut self) -> &mut f32 { &mut self.range }
    fn enabled(&self) -> bool { self.enabled }
    fn enabled_mut(&mut self) -> &mut bool { &mut self.enabled }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RadioTowerConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> RadioTowerConfiguration {
            RadioTowerConfiguration {
                id: Arbitrary::arbitrary(_g),
                affiliation: Arbitrary::arbitrary(_g),
                entity_type: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                nominal_speed: Arbitrary::arbitrary(_g),
                nominal_altitude: Arbitrary::arbitrary(_g),
                nominal_altitude_type: Arbitrary::arbitrary(_g),
                payload_configuration_list: Vec::<::afrl::cmasi::payload_configuration::PayloadConfiguration>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>).collect(),
                info: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                position: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                range: Arbitrary::arbitrary(_g),
                enabled: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RadioTowerConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_configuration_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RadioTowerConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_configuration_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RadioTowerConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
