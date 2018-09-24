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
pub struct WeatherReport {
    pub area: Option<Box<::afrl::cmasi::abstract_zone::AbstractZoneT>>,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub visibility: f32,
    pub cloud_ceiling: f32,
    pub cloud_coverage: f32,
}

impl PartialEq for WeatherReport {
    fn eq(&self, _other: &WeatherReport) -> bool {
        true
        && &self.area == &_other.area
        && &self.wind_speed == &_other.wind_speed
        && &self.wind_direction == &_other.wind_direction
        && &self.visibility == &_other.visibility
        && &self.cloud_ceiling == &_other.cloud_ceiling
        && &self.cloud_coverage == &_other.cloud_coverage

    }
}

impl LmcpSubscription for WeatherReport {
    fn subscription() -> &'static str { "afrl.cmasi.WeatherReport" }
}

impl Struct for WeatherReport {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 55,
        }
    }
}

impl Lmcp for WeatherReport {
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
            let writeb: usize = self.wind_speed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.wind_direction.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.visibility.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.cloud_ceiling.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.cloud_coverage.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(WeatherReport, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == WeatherReport::struct_info() {
            let mut out: WeatherReport = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Option<Box<::afrl::cmasi::abstract_zone::AbstractZoneT>>, usize) = Lmcp::deser(r)?;
                out.area = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.wind_speed = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.wind_direction = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.visibility = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.cloud_ceiling = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.cloud_coverage = x;
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
        size += self.wind_speed.size();
        size += self.wind_direction.size();
        size += self.visibility.size();
        size += self.cloud_ceiling.size();
        size += self.cloud_coverage.size();

        size
    }
}

pub trait WeatherReportT: Debug + Send  {
    fn as_afrl_cmasi_weather_report(&self) -> Option<&WeatherReport> { None }
    fn as_mut_afrl_cmasi_weather_report(&mut self) -> Option<&mut WeatherReport> { None }
    fn area(&self) -> &Option<Box<::afrl::cmasi::abstract_zone::AbstractZoneT>>;
    fn area_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::abstract_zone::AbstractZoneT>>;
    fn wind_speed(&self) -> f32;
    fn wind_speed_mut(&mut self) -> &mut f32;
    fn wind_direction(&self) -> f32;
    fn wind_direction_mut(&mut self) -> &mut f32;
    fn visibility(&self) -> f32;
    fn visibility_mut(&mut self) -> &mut f32;
    fn cloud_ceiling(&self) -> f32;
    fn cloud_ceiling_mut(&mut self) -> &mut f32;
    fn cloud_coverage(&self) -> f32;
    fn cloud_coverage_mut(&mut self) -> &mut f32;

}

impl Clone for Box<WeatherReportT> {
    fn clone(&self) -> Box<WeatherReportT> {
        if let Some(x) = WeatherReportT::as_afrl_cmasi_weather_report(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<WeatherReportT> {
    fn default() -> Box<WeatherReportT> { Box::new(WeatherReport::default()) }
}

impl PartialEq for Box<WeatherReportT> {
    fn eq(&self, other: &Box<WeatherReportT>) -> bool {
        if let (Some(x), Some(y)) =
            (WeatherReportT::as_afrl_cmasi_weather_report(self.as_ref()),
             WeatherReportT::as_afrl_cmasi_weather_report(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<WeatherReportT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = WeatherReportT::as_afrl_cmasi_weather_report(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<WeatherReportT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == WeatherReport::struct_info() {
            let (x, readb) = WeatherReport::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = WeatherReportT::as_afrl_cmasi_weather_report(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl WeatherReportT for WeatherReport {
    fn as_afrl_cmasi_weather_report(&self) -> Option<&WeatherReport> { Some(self) }
    fn as_mut_afrl_cmasi_weather_report(&mut self) -> Option<&mut WeatherReport> { Some(self) }
    fn area(&self) -> &Option<Box<::afrl::cmasi::abstract_zone::AbstractZoneT>> { &self.area }
    fn area_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::abstract_zone::AbstractZoneT>> { &mut self.area }
    fn wind_speed(&self) -> f32 { self.wind_speed }
    fn wind_speed_mut(&mut self) -> &mut f32 { &mut self.wind_speed }
    fn wind_direction(&self) -> f32 { self.wind_direction }
    fn wind_direction_mut(&mut self) -> &mut f32 { &mut self.wind_direction }
    fn visibility(&self) -> f32 { self.visibility }
    fn visibility_mut(&mut self) -> &mut f32 { &mut self.visibility }
    fn cloud_ceiling(&self) -> f32 { self.cloud_ceiling }
    fn cloud_ceiling_mut(&mut self) -> &mut f32 { &mut self.cloud_ceiling }
    fn cloud_coverage(&self) -> f32 { self.cloud_coverage }
    fn cloud_coverage_mut(&mut self) -> &mut f32 { &mut self.cloud_coverage }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for WeatherReport {
        fn arbitrary<G: Gen>(_g: &mut G) -> WeatherReport {
            WeatherReport {
                area: {
                    if _g.gen() {
                        Some(Box::new(::afrl::cmasi::abstract_zone::AbstractZone::arbitrary(_g)))
                    } else {
                        None
                    }
                },
                wind_speed: Arbitrary::arbitrary(_g),
                wind_direction: Arbitrary::arbitrary(_g),
                visibility: Arbitrary::arbitrary(_g),
                cloud_ceiling: Arbitrary::arbitrary(_g),
                cloud_coverage: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: WeatherReport) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: WeatherReport) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = WeatherReport::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
