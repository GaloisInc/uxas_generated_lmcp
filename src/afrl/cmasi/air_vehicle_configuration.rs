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
pub struct AirVehicleConfiguration {
    pub id: i64,
    pub affiliation: Vec<u8>,
    pub entity_type: Vec<u8>,
    pub label: Vec<u8>,
    pub nominal_speed: f32,
    pub nominal_altitude: f32,
    pub nominal_altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
    pub payload_configuration_list: Vec<Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>>,
    pub info: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub minimum_speed: f32,
    pub maximum_speed: f32,
    pub nominal_flight_profile: Box<::afrl::cmasi::flight_profile::FlightProfileT>,
    pub alternate_flight_profiles: Vec<Box<::afrl::cmasi::flight_profile::FlightProfileT>>,
    pub available_loiter_types: Vec<::afrl::cmasi::loiter_type::LoiterType>,
    pub available_turn_types: Vec<::afrl::cmasi::turn_type::TurnType>,
    pub minimum_altitude: f32,
    pub min_altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
    pub maximum_altitude: f32,
    pub max_altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
}

impl PartialEq for AirVehicleConfiguration {
    fn eq(&self, _other: &AirVehicleConfiguration) -> bool {
        true
        && &self.minimum_speed == &_other.minimum_speed
        && &self.maximum_speed == &_other.maximum_speed
        && &self.nominal_flight_profile == &_other.nominal_flight_profile
        && &self.alternate_flight_profiles == &_other.alternate_flight_profiles
        && &self.available_loiter_types == &_other.available_loiter_types
        && &self.available_turn_types == &_other.available_turn_types
        && &self.minimum_altitude == &_other.minimum_altitude
        && &self.min_altitude_type == &_other.min_altitude_type
        && &self.maximum_altitude == &_other.maximum_altitude
        && &self.max_altitude_type == &_other.max_altitude_type

    }
}

impl LmcpSubscription for AirVehicleConfiguration {
    fn subscription() -> &'static str { "afrl.cmasi.AirVehicleConfiguration" }
}

impl Struct for AirVehicleConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 13,
        }
    }
}

impl Lmcp for AirVehicleConfiguration {
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
            let writeb: usize = self.minimum_speed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.maximum_speed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.nominal_flight_profile.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.alternate_flight_profiles.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.available_loiter_types.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.available_turn_types.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.minimum_altitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.min_altitude_type.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.maximum_altitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_altitude_type.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(AirVehicleConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == AirVehicleConfiguration::struct_info() {
            let mut out: AirVehicleConfiguration = Default::default();
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
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.minimum_speed = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.maximum_speed = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::flight_profile::FlightProfileT>, usize) = Lmcp::deser(r)?;
                out.nominal_flight_profile = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::flight_profile::FlightProfileT>>, usize) = Lmcp::deser(r)?;
                out.alternate_flight_profiles = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<::afrl::cmasi::loiter_type::LoiterType>, usize) = Lmcp::deser(r)?;
                out.available_loiter_types = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<::afrl::cmasi::turn_type::TurnType>, usize) = Lmcp::deser(r)?;
                out.available_turn_types = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.minimum_altitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::altitude_type::AltitudeType, usize) = Lmcp::deser(r)?;
                out.min_altitude_type = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.maximum_altitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::altitude_type::AltitudeType, usize) = Lmcp::deser(r)?;
                out.max_altitude_type = x;
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
        size += self.minimum_speed.size();
        size += self.maximum_speed.size();
        size += self.nominal_flight_profile.size();
        size += self.alternate_flight_profiles.size();
        size += self.available_loiter_types.size();
        size += self.available_turn_types.size();
        size += self.minimum_altitude.size();
        size += self.min_altitude_type.size();
        size += self.maximum_altitude.size();
        size += self.max_altitude_type.size();

        size
    }
}

pub trait AirVehicleConfigurationT: Debug + Send + ::afrl::cmasi::entity_configuration::EntityConfigurationT {
    fn as_afrl_cmasi_air_vehicle_configuration(&self) -> Option<&AirVehicleConfiguration> { None }
    fn as_mut_afrl_cmasi_air_vehicle_configuration(&mut self) -> Option<&mut AirVehicleConfiguration> { None }
    fn minimum_speed(&self) -> f32;
    fn minimum_speed_mut(&mut self) -> &mut f32;
    fn maximum_speed(&self) -> f32;
    fn maximum_speed_mut(&mut self) -> &mut f32;
    fn nominal_flight_profile(&self) -> &Box<::afrl::cmasi::flight_profile::FlightProfileT>;
    fn nominal_flight_profile_mut(&mut self) -> &mut Box<::afrl::cmasi::flight_profile::FlightProfileT>;
    fn alternate_flight_profiles(&self) -> &Vec<Box<::afrl::cmasi::flight_profile::FlightProfileT>>;
    fn alternate_flight_profiles_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::flight_profile::FlightProfileT>>;
    fn available_loiter_types(&self) -> &Vec<::afrl::cmasi::loiter_type::LoiterType>;
    fn available_loiter_types_mut(&mut self) -> &mut Vec<::afrl::cmasi::loiter_type::LoiterType>;
    fn available_turn_types(&self) -> &Vec<::afrl::cmasi::turn_type::TurnType>;
    fn available_turn_types_mut(&mut self) -> &mut Vec<::afrl::cmasi::turn_type::TurnType>;
    fn minimum_altitude(&self) -> f32;
    fn minimum_altitude_mut(&mut self) -> &mut f32;
    fn min_altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType;
    fn min_altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType;
    fn maximum_altitude(&self) -> f32;
    fn maximum_altitude_mut(&mut self) -> &mut f32;
    fn max_altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType;
    fn max_altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType;

}

impl Clone for Box<AirVehicleConfigurationT> {
    fn clone(&self) -> Box<AirVehicleConfigurationT> {
        if let Some(x) = AirVehicleConfigurationT::as_afrl_cmasi_air_vehicle_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<AirVehicleConfigurationT> {
    fn default() -> Box<AirVehicleConfigurationT> { Box::new(AirVehicleConfiguration::default()) }
}

impl PartialEq for Box<AirVehicleConfigurationT> {
    fn eq(&self, other: &Box<AirVehicleConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (AirVehicleConfigurationT::as_afrl_cmasi_air_vehicle_configuration(self.as_ref()),
             AirVehicleConfigurationT::as_afrl_cmasi_air_vehicle_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<AirVehicleConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = AirVehicleConfigurationT::as_afrl_cmasi_air_vehicle_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<AirVehicleConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == AirVehicleConfiguration::struct_info() {
            let (x, readb) = AirVehicleConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = AirVehicleConfigurationT::as_afrl_cmasi_air_vehicle_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::entity_configuration::EntityConfigurationT for AirVehicleConfiguration {
    fn as_afrl_cmasi_air_vehicle_configuration(&self) -> Option<&AirVehicleConfiguration> { Some(self) }
    fn as_mut_afrl_cmasi_air_vehicle_configuration(&mut self) -> Option<&mut AirVehicleConfiguration> { Some(self) }
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
impl AirVehicleConfigurationT for AirVehicleConfiguration {
    fn as_afrl_cmasi_air_vehicle_configuration(&self) -> Option<&AirVehicleConfiguration> { Some(self) }
    fn as_mut_afrl_cmasi_air_vehicle_configuration(&mut self) -> Option<&mut AirVehicleConfiguration> { Some(self) }
    fn minimum_speed(&self) -> f32 { self.minimum_speed }
    fn minimum_speed_mut(&mut self) -> &mut f32 { &mut self.minimum_speed }
    fn maximum_speed(&self) -> f32 { self.maximum_speed }
    fn maximum_speed_mut(&mut self) -> &mut f32 { &mut self.maximum_speed }
    fn nominal_flight_profile(&self) -> &Box<::afrl::cmasi::flight_profile::FlightProfileT> { &self.nominal_flight_profile }
    fn nominal_flight_profile_mut(&mut self) -> &mut Box<::afrl::cmasi::flight_profile::FlightProfileT> { &mut self.nominal_flight_profile }
    fn alternate_flight_profiles(&self) -> &Vec<Box<::afrl::cmasi::flight_profile::FlightProfileT>> { &self.alternate_flight_profiles }
    fn alternate_flight_profiles_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::flight_profile::FlightProfileT>> { &mut self.alternate_flight_profiles }
    fn available_loiter_types(&self) -> &Vec<::afrl::cmasi::loiter_type::LoiterType> { &self.available_loiter_types }
    fn available_loiter_types_mut(&mut self) -> &mut Vec<::afrl::cmasi::loiter_type::LoiterType> { &mut self.available_loiter_types }
    fn available_turn_types(&self) -> &Vec<::afrl::cmasi::turn_type::TurnType> { &self.available_turn_types }
    fn available_turn_types_mut(&mut self) -> &mut Vec<::afrl::cmasi::turn_type::TurnType> { &mut self.available_turn_types }
    fn minimum_altitude(&self) -> f32 { self.minimum_altitude }
    fn minimum_altitude_mut(&mut self) -> &mut f32 { &mut self.minimum_altitude }
    fn min_altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType { self.min_altitude_type }
    fn min_altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType { &mut self.min_altitude_type }
    fn maximum_altitude(&self) -> f32 { self.maximum_altitude }
    fn maximum_altitude_mut(&mut self) -> &mut f32 { &mut self.maximum_altitude }
    fn max_altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType { self.max_altitude_type }
    fn max_altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType { &mut self.max_altitude_type }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AirVehicleConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> AirVehicleConfiguration {
            AirVehicleConfiguration {
                id: Arbitrary::arbitrary(_g),
                affiliation: Arbitrary::arbitrary(_g),
                entity_type: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                nominal_speed: Arbitrary::arbitrary(_g),
                nominal_altitude: Arbitrary::arbitrary(_g),
                nominal_altitude_type: Arbitrary::arbitrary(_g),
                payload_configuration_list: Vec::<::afrl::cmasi::payload_configuration::PayloadConfiguration>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>).collect(),
                info: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                minimum_speed: Arbitrary::arbitrary(_g),
                maximum_speed: Arbitrary::arbitrary(_g),
                nominal_flight_profile: Box::new(::afrl::cmasi::flight_profile::FlightProfile::arbitrary(_g)),
                alternate_flight_profiles: Vec::<::afrl::cmasi::flight_profile::FlightProfile>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::flight_profile::FlightProfileT>).collect(),
                available_loiter_types: Arbitrary::arbitrary(_g),
                available_turn_types: Arbitrary::arbitrary(_g),
                minimum_altitude: Arbitrary::arbitrary(_g),
                min_altitude_type: Arbitrary::arbitrary(_g),
                maximum_altitude: Arbitrary::arbitrary(_g),
                max_altitude_type: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: AirVehicleConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_configuration_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.alternate_flight_profiles.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.available_loiter_types.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.available_turn_types.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AirVehicleConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_configuration_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.alternate_flight_profiles.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.available_loiter_types.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.available_turn_types.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AirVehicleConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
