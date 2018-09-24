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
pub struct SurfaceVehicleConfiguration {
    pub id: i64,
    pub affiliation: Vec<u8>,
    pub entity_type: Vec<u8>,
    pub label: Vec<u8>,
    pub nominal_speed: f32,
    pub nominal_altitude: f32,
    pub nominal_altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
    pub payload_configuration_list: Vec<Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>>,
    pub info: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub water_area: i64,
    pub minimum_speed: f32,
    pub maximum_speed: f32,
    pub energy_rate: f32,
    pub max_bank_angle: f32,
    pub max_bank_rate: f32,
}

impl PartialEq for SurfaceVehicleConfiguration {
    fn eq(&self, _other: &SurfaceVehicleConfiguration) -> bool {
        true
        && &self.water_area == &_other.water_area
        && &self.minimum_speed == &_other.minimum_speed
        && &self.maximum_speed == &_other.maximum_speed
        && &self.energy_rate == &_other.energy_rate
        && &self.max_bank_angle == &_other.max_bank_angle
        && &self.max_bank_rate == &_other.max_bank_rate

    }
}

impl LmcpSubscription for SurfaceVehicleConfiguration {
    fn subscription() -> &'static str { "afrl.vehicles.SurfaceVehicleConfiguration" }
}

impl Struct for SurfaceVehicleConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6216454340153722195u64,
            version: 1,
            struct_ty: 3,
        }
    }
}

impl Lmcp for SurfaceVehicleConfiguration {
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
            let writeb: usize = self.water_area.ser(r)?;
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
            let writeb: usize = self.energy_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_bank_angle.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_bank_rate.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SurfaceVehicleConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SurfaceVehicleConfiguration::struct_info() {
            let mut out: SurfaceVehicleConfiguration = Default::default();
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
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.water_area = x;
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
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.energy_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.max_bank_angle = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.max_bank_rate = x;
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
        size += self.water_area.size();
        size += self.minimum_speed.size();
        size += self.maximum_speed.size();
        size += self.energy_rate.size();
        size += self.max_bank_angle.size();
        size += self.max_bank_rate.size();

        size
    }
}

pub trait SurfaceVehicleConfigurationT: Debug + Send + ::afrl::cmasi::entity_configuration::EntityConfigurationT {
    fn as_afrl_vehicles_surface_vehicle_configuration(&self) -> Option<&SurfaceVehicleConfiguration> { None }
    fn as_mut_afrl_vehicles_surface_vehicle_configuration(&mut self) -> Option<&mut SurfaceVehicleConfiguration> { None }
    fn water_area(&self) -> i64;
    fn water_area_mut(&mut self) -> &mut i64;
    fn minimum_speed(&self) -> f32;
    fn minimum_speed_mut(&mut self) -> &mut f32;
    fn maximum_speed(&self) -> f32;
    fn maximum_speed_mut(&mut self) -> &mut f32;
    fn energy_rate(&self) -> f32;
    fn energy_rate_mut(&mut self) -> &mut f32;
    fn max_bank_angle(&self) -> f32;
    fn max_bank_angle_mut(&mut self) -> &mut f32;
    fn max_bank_rate(&self) -> f32;
    fn max_bank_rate_mut(&mut self) -> &mut f32;

}

impl Clone for Box<SurfaceVehicleConfigurationT> {
    fn clone(&self) -> Box<SurfaceVehicleConfigurationT> {
        if let Some(x) = SurfaceVehicleConfigurationT::as_afrl_vehicles_surface_vehicle_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SurfaceVehicleConfigurationT> {
    fn default() -> Box<SurfaceVehicleConfigurationT> { Box::new(SurfaceVehicleConfiguration::default()) }
}

impl PartialEq for Box<SurfaceVehicleConfigurationT> {
    fn eq(&self, other: &Box<SurfaceVehicleConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (SurfaceVehicleConfigurationT::as_afrl_vehicles_surface_vehicle_configuration(self.as_ref()),
             SurfaceVehicleConfigurationT::as_afrl_vehicles_surface_vehicle_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SurfaceVehicleConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SurfaceVehicleConfigurationT::as_afrl_vehicles_surface_vehicle_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SurfaceVehicleConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SurfaceVehicleConfiguration::struct_info() {
            let (x, readb) = SurfaceVehicleConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SurfaceVehicleConfigurationT::as_afrl_vehicles_surface_vehicle_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::entity_configuration::EntityConfigurationT for SurfaceVehicleConfiguration {
    fn as_afrl_vehicles_surface_vehicle_configuration(&self) -> Option<&SurfaceVehicleConfiguration> { Some(self) }
    fn as_mut_afrl_vehicles_surface_vehicle_configuration(&mut self) -> Option<&mut SurfaceVehicleConfiguration> { Some(self) }
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
impl SurfaceVehicleConfigurationT for SurfaceVehicleConfiguration {
    fn as_afrl_vehicles_surface_vehicle_configuration(&self) -> Option<&SurfaceVehicleConfiguration> { Some(self) }
    fn as_mut_afrl_vehicles_surface_vehicle_configuration(&mut self) -> Option<&mut SurfaceVehicleConfiguration> { Some(self) }
    fn water_area(&self) -> i64 { self.water_area }
    fn water_area_mut(&mut self) -> &mut i64 { &mut self.water_area }
    fn minimum_speed(&self) -> f32 { self.minimum_speed }
    fn minimum_speed_mut(&mut self) -> &mut f32 { &mut self.minimum_speed }
    fn maximum_speed(&self) -> f32 { self.maximum_speed }
    fn maximum_speed_mut(&mut self) -> &mut f32 { &mut self.maximum_speed }
    fn energy_rate(&self) -> f32 { self.energy_rate }
    fn energy_rate_mut(&mut self) -> &mut f32 { &mut self.energy_rate }
    fn max_bank_angle(&self) -> f32 { self.max_bank_angle }
    fn max_bank_angle_mut(&mut self) -> &mut f32 { &mut self.max_bank_angle }
    fn max_bank_rate(&self) -> f32 { self.max_bank_rate }
    fn max_bank_rate_mut(&mut self) -> &mut f32 { &mut self.max_bank_rate }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SurfaceVehicleConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> SurfaceVehicleConfiguration {
            SurfaceVehicleConfiguration {
                id: Arbitrary::arbitrary(_g),
                affiliation: Arbitrary::arbitrary(_g),
                entity_type: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                nominal_speed: Arbitrary::arbitrary(_g),
                nominal_altitude: Arbitrary::arbitrary(_g),
                nominal_altitude_type: Arbitrary::arbitrary(_g),
                payload_configuration_list: Vec::<::afrl::cmasi::payload_configuration::PayloadConfiguration>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::payload_configuration::PayloadConfigurationT>).collect(),
                info: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                water_area: Arbitrary::arbitrary(_g),
                minimum_speed: Arbitrary::arbitrary(_g),
                maximum_speed: Arbitrary::arbitrary(_g),
                energy_rate: Arbitrary::arbitrary(_g),
                max_bank_angle: Arbitrary::arbitrary(_g),
                max_bank_rate: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SurfaceVehicleConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_configuration_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SurfaceVehicleConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_configuration_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SurfaceVehicleConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
