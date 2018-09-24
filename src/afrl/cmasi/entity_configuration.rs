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
pub struct EntityConfiguration {
    pub id: i64,
    pub affiliation: Vec<u8>,
    pub entity_type: Vec<u8>,
    pub label: Vec<u8>,
    pub nominal_speed: f32,
    pub nominal_altitude: f32,
    pub nominal_altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
    pub payload_configuration_list: Vec<Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>>,
    pub info: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
}

impl PartialEq for EntityConfiguration {
    fn eq(&self, _other: &EntityConfiguration) -> bool {
        true
        && &self.id == &_other.id
        && &self.affiliation == &_other.affiliation
        && &self.entity_type == &_other.entity_type
        && &self.label == &_other.label
        && &self.nominal_speed == &_other.nominal_speed
        && &self.nominal_altitude == &_other.nominal_altitude
        && &self.nominal_altitude_type == &_other.nominal_altitude_type
        && &self.payload_configuration_list == &_other.payload_configuration_list
        && &self.info == &_other.info

    }
}

impl LmcpSubscription for EntityConfiguration {
    fn subscription() -> &'static str { "afrl.cmasi.EntityConfiguration" }
}

impl Struct for EntityConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 11,
        }
    }
}

impl Lmcp for EntityConfiguration {
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

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(EntityConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == EntityConfiguration::struct_info() {
            let mut out: EntityConfiguration = Default::default();
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

        size
    }
}

pub trait EntityConfigurationT: Debug + Send  {
    fn as_afrl_cmasi_entity_configuration(&self) -> Option<&EntityConfiguration> { None }
    fn as_mut_afrl_cmasi_entity_configuration(&mut self) -> Option<&mut EntityConfiguration> { None }
    fn as_afrl_vehicles_surface_vehicle_configuration(&self) -> Option<&::afrl::vehicles::surface_vehicle_configuration::SurfaceVehicleConfiguration> { None }
    fn as_mut_afrl_vehicles_surface_vehicle_configuration(&mut self) -> Option<&mut ::afrl::vehicles::surface_vehicle_configuration::SurfaceVehicleConfiguration> { None }
    fn as_afrl_cmasi_air_vehicle_configuration(&self) -> Option<&::afrl::cmasi::air_vehicle_configuration::AirVehicleConfiguration> { None }
    fn as_mut_afrl_cmasi_air_vehicle_configuration(&mut self) -> Option<&mut ::afrl::cmasi::air_vehicle_configuration::AirVehicleConfiguration> { None }
    fn as_afrl_impact_radio_tower_configuration(&self) -> Option<&::afrl::impact::radio_tower_configuration::RadioTowerConfiguration> { None }
    fn as_mut_afrl_impact_radio_tower_configuration(&mut self) -> Option<&mut ::afrl::impact::radio_tower_configuration::RadioTowerConfiguration> { None }
    fn as_afrl_vehicles_ground_vehicle_configuration(&self) -> Option<&::afrl::vehicles::ground_vehicle_configuration::GroundVehicleConfiguration> { None }
    fn as_mut_afrl_vehicles_ground_vehicle_configuration(&mut self) -> Option<&mut ::afrl::vehicles::ground_vehicle_configuration::GroundVehicleConfiguration> { None }
    fn as_afrl_vehicles_stationary_sensor_configuration(&self) -> Option<&::afrl::vehicles::stationary_sensor_configuration::StationarySensorConfiguration> { None }
    fn as_mut_afrl_vehicles_stationary_sensor_configuration(&mut self) -> Option<&mut ::afrl::vehicles::stationary_sensor_configuration::StationarySensorConfiguration> { None }
    fn id(&self) -> i64;
    fn id_mut(&mut self) -> &mut i64;
    fn affiliation(&self) -> &Vec<u8>;
    fn affiliation_mut(&mut self) -> &mut Vec<u8>;
    fn entity_type(&self) -> &Vec<u8>;
    fn entity_type_mut(&mut self) -> &mut Vec<u8>;
    fn label(&self) -> &Vec<u8>;
    fn label_mut(&mut self) -> &mut Vec<u8>;
    fn nominal_speed(&self) -> f32;
    fn nominal_speed_mut(&mut self) -> &mut f32;
    fn nominal_altitude(&self) -> f32;
    fn nominal_altitude_mut(&mut self) -> &mut f32;
    fn nominal_altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType;
    fn nominal_altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType;
    fn payload_configuration_list(&self) -> &Vec<Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>>;
    fn payload_configuration_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>>;
    fn info(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn info_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;

}

impl Clone for Box<EntityConfigurationT> {
    fn clone(&self) -> Box<EntityConfigurationT> {
        if let Some(x) = EntityConfigurationT::as_afrl_cmasi_entity_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityConfigurationT::as_afrl_vehicles_surface_vehicle_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityConfigurationT::as_afrl_cmasi_air_vehicle_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityConfigurationT::as_afrl_impact_radio_tower_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityConfigurationT::as_afrl_vehicles_ground_vehicle_configuration(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityConfigurationT::as_afrl_vehicles_stationary_sensor_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<EntityConfigurationT> {
    fn default() -> Box<EntityConfigurationT> { Box::new(EntityConfiguration::default()) }
}

impl PartialEq for Box<EntityConfigurationT> {
    fn eq(&self, other: &Box<EntityConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (EntityConfigurationT::as_afrl_cmasi_entity_configuration(self.as_ref()),
             EntityConfigurationT::as_afrl_cmasi_entity_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityConfigurationT::as_afrl_vehicles_surface_vehicle_configuration(self.as_ref()),
             EntityConfigurationT::as_afrl_vehicles_surface_vehicle_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityConfigurationT::as_afrl_cmasi_air_vehicle_configuration(self.as_ref()),
             EntityConfigurationT::as_afrl_cmasi_air_vehicle_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityConfigurationT::as_afrl_impact_radio_tower_configuration(self.as_ref()),
             EntityConfigurationT::as_afrl_impact_radio_tower_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityConfigurationT::as_afrl_vehicles_ground_vehicle_configuration(self.as_ref()),
             EntityConfigurationT::as_afrl_vehicles_ground_vehicle_configuration(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityConfigurationT::as_afrl_vehicles_stationary_sensor_configuration(self.as_ref()),
             EntityConfigurationT::as_afrl_vehicles_stationary_sensor_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<EntityConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = EntityConfigurationT::as_afrl_cmasi_entity_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityConfigurationT::as_afrl_vehicles_surface_vehicle_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityConfigurationT::as_afrl_cmasi_air_vehicle_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityConfigurationT::as_afrl_impact_radio_tower_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityConfigurationT::as_afrl_vehicles_ground_vehicle_configuration(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityConfigurationT::as_afrl_vehicles_stationary_sensor_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<EntityConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == EntityConfiguration::struct_info() {
            let (x, readb) = EntityConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::vehicles::surface_vehicle_configuration::SurfaceVehicleConfiguration::struct_info() {
            let (x, readb) = ::afrl::vehicles::surface_vehicle_configuration::SurfaceVehicleConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::air_vehicle_configuration::AirVehicleConfiguration::struct_info() {
            let (x, readb) = ::afrl::cmasi::air_vehicle_configuration::AirVehicleConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::radio_tower_configuration::RadioTowerConfiguration::struct_info() {
            let (x, readb) = ::afrl::impact::radio_tower_configuration::RadioTowerConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::vehicles::ground_vehicle_configuration::GroundVehicleConfiguration::struct_info() {
            let (x, readb) = ::afrl::vehicles::ground_vehicle_configuration::GroundVehicleConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::vehicles::stationary_sensor_configuration::StationarySensorConfiguration::struct_info() {
            let (x, readb) = ::afrl::vehicles::stationary_sensor_configuration::StationarySensorConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = EntityConfigurationT::as_afrl_cmasi_entity_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityConfigurationT::as_afrl_vehicles_surface_vehicle_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityConfigurationT::as_afrl_cmasi_air_vehicle_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityConfigurationT::as_afrl_impact_radio_tower_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityConfigurationT::as_afrl_vehicles_ground_vehicle_configuration(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityConfigurationT::as_afrl_vehicles_stationary_sensor_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl EntityConfigurationT for EntityConfiguration {
    fn as_afrl_cmasi_entity_configuration(&self) -> Option<&EntityConfiguration> { Some(self) }
    fn as_mut_afrl_cmasi_entity_configuration(&mut self) -> Option<&mut EntityConfiguration> { Some(self) }
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


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for EntityConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> EntityConfiguration {
            EntityConfiguration {
                id: Arbitrary::arbitrary(_g),
                affiliation: Arbitrary::arbitrary(_g),
                entity_type: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                nominal_speed: Arbitrary::arbitrary(_g),
                nominal_altitude: Arbitrary::arbitrary(_g),
                nominal_altitude_type: Arbitrary::arbitrary(_g),
                payload_configuration_list: Vec::<::afrl::cmasi::payload_configuration::PayloadConfiguration>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>).collect(),
                info: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: EntityConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_configuration_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: EntityConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_configuration_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = EntityConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
