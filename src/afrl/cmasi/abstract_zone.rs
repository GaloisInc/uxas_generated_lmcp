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
pub struct AbstractZone {
    pub zone_id: i64,
    pub min_altitude: f32,
    pub min_altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
    pub max_altitude: f32,
    pub max_altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
    pub affected_aircraft: Vec<i64>,
    pub start_time: i64,
    pub end_time: i64,
    pub padding: f32,
    pub label: Vec<u8>,
    pub boundary: Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>,
}

impl PartialEq for AbstractZone {
    fn eq(&self, _other: &AbstractZone) -> bool {
        true
        && &self.zone_id == &_other.zone_id
        && &self.min_altitude == &_other.min_altitude
        && &self.min_altitude_type == &_other.min_altitude_type
        && &self.max_altitude == &_other.max_altitude
        && &self.max_altitude_type == &_other.max_altitude_type
        && &self.affected_aircraft == &_other.affected_aircraft
        && &self.start_time == &_other.start_time
        && &self.end_time == &_other.end_time
        && &self.padding == &_other.padding
        && &self.label == &_other.label
        && &self.boundary == &_other.boundary

    }
}

impl LmcpSubscription for AbstractZone {
    fn subscription() -> &'static str { "afrl.cmasi.AbstractZone" }
}

impl Struct for AbstractZone {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 10,
        }
    }
}

impl Lmcp for AbstractZone {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.zone_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.min_altitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.min_altitude_type.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_altitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_altitude_type.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.affected_aircraft.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_time.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.end_time.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.padding.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.label.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.boundary.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(AbstractZone, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == AbstractZone::struct_info() {
            let mut out: AbstractZone = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.zone_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.min_altitude = x;
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
                out.max_altitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::altitude_type::AltitudeType, usize) = Lmcp::deser(r)?;
                out.max_altitude_type = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.affected_aircraft = x;
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
                out.end_time = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.padding = x;
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
                let (x, readb): (Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>, usize) = Lmcp::deser(r)?;
                out.boundary = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.zone_id.size();
        size += self.min_altitude.size();
        size += self.min_altitude_type.size();
        size += self.max_altitude.size();
        size += self.max_altitude_type.size();
        size += self.affected_aircraft.size();
        size += self.start_time.size();
        size += self.end_time.size();
        size += self.padding.size();
        size += self.label.size();
        size += self.boundary.size();

        size
    }
}

pub trait AbstractZoneT: Debug + Send  {
    fn as_afrl_cmasi_abstract_zone(&self) -> Option<&AbstractZone> { None }
    fn as_mut_afrl_cmasi_abstract_zone(&mut self) -> Option<&mut AbstractZone> { None }
    fn as_afrl_cmasi_keep_out_zone(&self) -> Option<&::afrl::cmasi::keep_out_zone::KeepOutZone> { None }
    fn as_mut_afrl_cmasi_keep_out_zone(&mut self) -> Option<&mut ::afrl::cmasi::keep_out_zone::KeepOutZone> { None }
    fn as_afrl_impact_water_zone(&self) -> Option<&::afrl::impact::water_zone::WaterZone> { None }
    fn as_mut_afrl_impact_water_zone(&mut self) -> Option<&mut ::afrl::impact::water_zone::WaterZone> { None }
    fn as_afrl_cmasi_keep_in_zone(&self) -> Option<&::afrl::cmasi::keep_in_zone::KeepInZone> { None }
    fn as_mut_afrl_cmasi_keep_in_zone(&mut self) -> Option<&mut ::afrl::cmasi::keep_in_zone::KeepInZone> { None }
    fn zone_id(&self) -> i64;
    fn zone_id_mut(&mut self) -> &mut i64;
    fn min_altitude(&self) -> f32;
    fn min_altitude_mut(&mut self) -> &mut f32;
    fn min_altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType;
    fn min_altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType;
    fn max_altitude(&self) -> f32;
    fn max_altitude_mut(&mut self) -> &mut f32;
    fn max_altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType;
    fn max_altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType;
    fn affected_aircraft(&self) -> &Vec<i64>;
    fn affected_aircraft_mut(&mut self) -> &mut Vec<i64>;
    fn start_time(&self) -> i64;
    fn start_time_mut(&mut self) -> &mut i64;
    fn end_time(&self) -> i64;
    fn end_time_mut(&mut self) -> &mut i64;
    fn padding(&self) -> f32;
    fn padding_mut(&mut self) -> &mut f32;
    fn label(&self) -> &Vec<u8>;
    fn label_mut(&mut self) -> &mut Vec<u8>;
    fn boundary(&self) -> &Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>;
    fn boundary_mut(&mut self) -> &mut Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>;

}

impl Clone for Box<AbstractZoneT> {
    fn clone(&self) -> Box<AbstractZoneT> {
        if let Some(x) = AbstractZoneT::as_afrl_cmasi_abstract_zone(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = AbstractZoneT::as_afrl_cmasi_keep_out_zone(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = AbstractZoneT::as_afrl_impact_water_zone(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = AbstractZoneT::as_afrl_cmasi_keep_in_zone(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<AbstractZoneT> {
    fn default() -> Box<AbstractZoneT> { Box::new(AbstractZone::default()) }
}

impl PartialEq for Box<AbstractZoneT> {
    fn eq(&self, other: &Box<AbstractZoneT>) -> bool {
        if let (Some(x), Some(y)) =
            (AbstractZoneT::as_afrl_cmasi_abstract_zone(self.as_ref()),
             AbstractZoneT::as_afrl_cmasi_abstract_zone(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (AbstractZoneT::as_afrl_cmasi_keep_out_zone(self.as_ref()),
             AbstractZoneT::as_afrl_cmasi_keep_out_zone(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (AbstractZoneT::as_afrl_impact_water_zone(self.as_ref()),
             AbstractZoneT::as_afrl_impact_water_zone(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (AbstractZoneT::as_afrl_cmasi_keep_in_zone(self.as_ref()),
             AbstractZoneT::as_afrl_cmasi_keep_in_zone(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<AbstractZoneT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = AbstractZoneT::as_afrl_cmasi_abstract_zone(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = AbstractZoneT::as_afrl_cmasi_keep_out_zone(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = AbstractZoneT::as_afrl_impact_water_zone(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = AbstractZoneT::as_afrl_cmasi_keep_in_zone(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<AbstractZoneT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == AbstractZone::struct_info() {
            let (x, readb) = AbstractZone::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::keep_out_zone::KeepOutZone::struct_info() {
            let (x, readb) = ::afrl::cmasi::keep_out_zone::KeepOutZone::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::water_zone::WaterZone::struct_info() {
            let (x, readb) = ::afrl::impact::water_zone::WaterZone::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::keep_in_zone::KeepInZone::struct_info() {
            let (x, readb) = ::afrl::cmasi::keep_in_zone::KeepInZone::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = AbstractZoneT::as_afrl_cmasi_abstract_zone(self.as_ref()) {
            x.size()
        } else if let Some(x) = AbstractZoneT::as_afrl_cmasi_keep_out_zone(self.as_ref()) {
            x.size()
        } else if let Some(x) = AbstractZoneT::as_afrl_impact_water_zone(self.as_ref()) {
            x.size()
        } else if let Some(x) = AbstractZoneT::as_afrl_cmasi_keep_in_zone(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl AbstractZoneT for AbstractZone {
    fn as_afrl_cmasi_abstract_zone(&self) -> Option<&AbstractZone> { Some(self) }
    fn as_mut_afrl_cmasi_abstract_zone(&mut self) -> Option<&mut AbstractZone> { Some(self) }
    fn zone_id(&self) -> i64 { self.zone_id }
    fn zone_id_mut(&mut self) -> &mut i64 { &mut self.zone_id }
    fn min_altitude(&self) -> f32 { self.min_altitude }
    fn min_altitude_mut(&mut self) -> &mut f32 { &mut self.min_altitude }
    fn min_altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType { self.min_altitude_type }
    fn min_altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType { &mut self.min_altitude_type }
    fn max_altitude(&self) -> f32 { self.max_altitude }
    fn max_altitude_mut(&mut self) -> &mut f32 { &mut self.max_altitude }
    fn max_altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType { self.max_altitude_type }
    fn max_altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType { &mut self.max_altitude_type }
    fn affected_aircraft(&self) -> &Vec<i64> { &self.affected_aircraft }
    fn affected_aircraft_mut(&mut self) -> &mut Vec<i64> { &mut self.affected_aircraft }
    fn start_time(&self) -> i64 { self.start_time }
    fn start_time_mut(&mut self) -> &mut i64 { &mut self.start_time }
    fn end_time(&self) -> i64 { self.end_time }
    fn end_time_mut(&mut self) -> &mut i64 { &mut self.end_time }
    fn padding(&self) -> f32 { self.padding }
    fn padding_mut(&mut self) -> &mut f32 { &mut self.padding }
    fn label(&self) -> &Vec<u8> { &self.label }
    fn label_mut(&mut self) -> &mut Vec<u8> { &mut self.label }
    fn boundary(&self) -> &Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT> { &self.boundary }
    fn boundary_mut(&mut self) -> &mut Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT> { &mut self.boundary }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AbstractZone {
        fn arbitrary<G: Gen>(_g: &mut G) -> AbstractZone {
            AbstractZone {
                zone_id: Arbitrary::arbitrary(_g),
                min_altitude: Arbitrary::arbitrary(_g),
                min_altitude_type: Arbitrary::arbitrary(_g),
                max_altitude: Arbitrary::arbitrary(_g),
                max_altitude_type: Arbitrary::arbitrary(_g),
                affected_aircraft: Arbitrary::arbitrary(_g),
                start_time: Arbitrary::arbitrary(_g),
                end_time: Arbitrary::arbitrary(_g),
                padding: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                boundary: Box::new(::afrl::cmasi::abstract_geometry::AbstractGeometry::arbitrary(_g)),

            }
        }
    }

    quickcheck! {
        fn serializes(x: AbstractZone) -> Result<TestResult, Error> {
            use std::u16;
            if x.affected_aircraft.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AbstractZone) -> Result<TestResult, Error> {
            use std::u16;
            if x.affected_aircraft.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AbstractZone::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
