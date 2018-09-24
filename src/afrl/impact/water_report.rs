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
pub struct WaterReport {
    pub area: Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>,
    pub current_speed: f32,
    pub current_direction: f32,
    pub wave_direction: f32,
    pub wave_height: f32,
}

impl PartialEq for WaterReport {
    fn eq(&self, _other: &WaterReport) -> bool {
        true
        && &self.area == &_other.area
        && &self.current_speed == &_other.current_speed
        && &self.current_direction == &_other.current_direction
        && &self.wave_direction == &_other.wave_direction
        && &self.wave_height == &_other.wave_height

    }
}

impl LmcpSubscription for WaterReport {
    fn subscription() -> &'static str { "afrl.impact.WaterReport" }
}

impl Struct for WaterReport {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 33,
        }
    }
}

impl Lmcp for WaterReport {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.area.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.current_speed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.current_direction.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.wave_direction.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.wave_height.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(WaterReport, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == WaterReport::struct_info() {
            let mut out: WaterReport = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>, usize) = Lmcp::deser(r)?;
                out.area = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.current_speed = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.current_direction = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.wave_direction = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.wave_height = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.area.size();
        size += self.current_speed.size();
        size += self.current_direction.size();
        size += self.wave_direction.size();
        size += self.wave_height.size();

        size
    }
}

pub trait WaterReportT: Debug + Send  {
    fn as_afrl_impact_water_report(&self) -> Option<&WaterReport> { None }
    fn as_mut_afrl_impact_water_report(&mut self) -> Option<&mut WaterReport> { None }
    fn area(&self) -> &Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>;
    fn area_mut(&mut self) -> &mut Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>;
    fn current_speed(&self) -> f32;
    fn current_speed_mut(&mut self) -> &mut f32;
    fn current_direction(&self) -> f32;
    fn current_direction_mut(&mut self) -> &mut f32;
    fn wave_direction(&self) -> f32;
    fn wave_direction_mut(&mut self) -> &mut f32;
    fn wave_height(&self) -> f32;
    fn wave_height_mut(&mut self) -> &mut f32;

}

impl Clone for Box<WaterReportT> {
    fn clone(&self) -> Box<WaterReportT> {
        if let Some(x) = WaterReportT::as_afrl_impact_water_report(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<WaterReportT> {
    fn default() -> Box<WaterReportT> { Box::new(WaterReport::default()) }
}

impl PartialEq for Box<WaterReportT> {
    fn eq(&self, other: &Box<WaterReportT>) -> bool {
        if let (Some(x), Some(y)) =
            (WaterReportT::as_afrl_impact_water_report(self.as_ref()),
             WaterReportT::as_afrl_impact_water_report(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<WaterReportT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = WaterReportT::as_afrl_impact_water_report(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<WaterReportT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == WaterReport::struct_info() {
            let (x, readb) = WaterReport::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = WaterReportT::as_afrl_impact_water_report(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl WaterReportT for WaterReport {
    fn as_afrl_impact_water_report(&self) -> Option<&WaterReport> { Some(self) }
    fn as_mut_afrl_impact_water_report(&mut self) -> Option<&mut WaterReport> { Some(self) }
    fn area(&self) -> &Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT> { &self.area }
    fn area_mut(&mut self) -> &mut Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT> { &mut self.area }
    fn current_speed(&self) -> f32 { self.current_speed }
    fn current_speed_mut(&mut self) -> &mut f32 { &mut self.current_speed }
    fn current_direction(&self) -> f32 { self.current_direction }
    fn current_direction_mut(&mut self) -> &mut f32 { &mut self.current_direction }
    fn wave_direction(&self) -> f32 { self.wave_direction }
    fn wave_direction_mut(&mut self) -> &mut f32 { &mut self.wave_direction }
    fn wave_height(&self) -> f32 { self.wave_height }
    fn wave_height_mut(&mut self) -> &mut f32 { &mut self.wave_height }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for WaterReport {
        fn arbitrary<G: Gen>(_g: &mut G) -> WaterReport {
            WaterReport {
                area: Box::new(::afrl::cmasi::abstract_geometry::AbstractGeometry::arbitrary(_g)),
                current_speed: Arbitrary::arbitrary(_g),
                current_direction: Arbitrary::arbitrary(_g),
                wave_direction: Arbitrary::arbitrary(_g),
                wave_height: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: WaterReport) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: WaterReport) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = WaterReport::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
