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
pub struct WaterZone {
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

impl PartialEq for WaterZone {
    fn eq(&self, _other: &WaterZone) -> bool {
        true

    }
}

impl LmcpSubscription for WaterZone {
    fn subscription() -> &'static str { "afrl.impact.WaterZone" }
}

impl Struct for WaterZone {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 34,
        }
    }
}

impl Lmcp for WaterZone {
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

    fn deser(buf: &[u8]) -> Result<(WaterZone, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == WaterZone::struct_info() {
            let mut out: WaterZone = Default::default();
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

pub trait WaterZoneT: Debug + Send + ::afrl::cmasi::abstract_zone::AbstractZoneT {
    fn as_afrl_impact_water_zone(&self) -> Option<&WaterZone> { None }
    fn as_mut_afrl_impact_water_zone(&mut self) -> Option<&mut WaterZone> { None }

}

impl Clone for Box<WaterZoneT> {
    fn clone(&self) -> Box<WaterZoneT> {
        if let Some(x) = WaterZoneT::as_afrl_impact_water_zone(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<WaterZoneT> {
    fn default() -> Box<WaterZoneT> { Box::new(WaterZone::default()) }
}

impl PartialEq for Box<WaterZoneT> {
    fn eq(&self, other: &Box<WaterZoneT>) -> bool {
        if let (Some(x), Some(y)) =
            (WaterZoneT::as_afrl_impact_water_zone(self.as_ref()),
             WaterZoneT::as_afrl_impact_water_zone(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<WaterZoneT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = WaterZoneT::as_afrl_impact_water_zone(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<WaterZoneT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == WaterZone::struct_info() {
            let (x, readb) = WaterZone::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = WaterZoneT::as_afrl_impact_water_zone(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::abstract_zone::AbstractZoneT for WaterZone {
    fn as_afrl_impact_water_zone(&self) -> Option<&WaterZone> { Some(self) }
    fn as_mut_afrl_impact_water_zone(&mut self) -> Option<&mut WaterZone> { Some(self) }
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
impl WaterZoneT for WaterZone {
    fn as_afrl_impact_water_zone(&self) -> Option<&WaterZone> { Some(self) }
    fn as_mut_afrl_impact_water_zone(&mut self) -> Option<&mut WaterZone> { Some(self) }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for WaterZone {
        fn arbitrary<G: Gen>(_g: &mut G) -> WaterZone {
            WaterZone {
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
        fn serializes(x: WaterZone) -> Result<TestResult, Error> {
            use std::u16;
            if x.affected_aircraft.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: WaterZone) -> Result<TestResult, Error> {
            use std::u16;
            if x.affected_aircraft.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = WaterZone::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
