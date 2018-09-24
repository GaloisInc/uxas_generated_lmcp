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

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct SensorFootprint {
    pub footprint_response_id: i64,
    pub vehicle_id: i64,
    pub camera_id: i64,
    pub gimbal_id: i64,
    pub horizontal_fov: f32,
    pub agl_altitude: f32,
    pub gimbal_elevation: f32,
    pub aspect_ratio: f32,
    pub achieved_gsd: f32,
    pub camera_wavelength: ::afrl::cmasi::wavelength_band::WavelengthBand,
    pub horizontal_to_leading_edge: f32,
    pub horizontal_to_trailing_edge: f32,
    pub horizontal_to_center: f32,
    pub width_center: f32,
    pub slant_range_to_center: f32,
}

impl PartialEq for SensorFootprint {
    fn eq(&self, _other: &SensorFootprint) -> bool {
        true
        && &self.footprint_response_id == &_other.footprint_response_id
        && &self.vehicle_id == &_other.vehicle_id
        && &self.camera_id == &_other.camera_id
        && &self.gimbal_id == &_other.gimbal_id
        && &self.horizontal_fov == &_other.horizontal_fov
        && &self.agl_altitude == &_other.agl_altitude
        && &self.gimbal_elevation == &_other.gimbal_elevation
        && &self.aspect_ratio == &_other.aspect_ratio
        && &self.achieved_gsd == &_other.achieved_gsd
        && &self.camera_wavelength == &_other.camera_wavelength
        && &self.horizontal_to_leading_edge == &_other.horizontal_to_leading_edge
        && &self.horizontal_to_trailing_edge == &_other.horizontal_to_trailing_edge
        && &self.horizontal_to_center == &_other.horizontal_to_center
        && &self.width_center == &_other.width_center
        && &self.slant_range_to_center == &_other.slant_range_to_center

    }
}

impl LmcpSubscription for SensorFootprint {
    fn subscription() -> &'static str { "uxas.messages.task.SensorFootprint" }
}

impl Struct for SensorFootprint {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 12,
        }
    }
}

impl Lmcp for SensorFootprint {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.footprint_response_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.camera_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.gimbal_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.horizontal_fov.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.agl_altitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.gimbal_elevation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.aspect_ratio.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.achieved_gsd.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.camera_wavelength.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.horizontal_to_leading_edge.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.horizontal_to_trailing_edge.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.horizontal_to_center.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.width_center.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.slant_range_to_center.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SensorFootprint, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SensorFootprint::struct_info() {
            let mut out: SensorFootprint = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.footprint_response_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.camera_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.gimbal_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.horizontal_fov = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.agl_altitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.gimbal_elevation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.aspect_ratio = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.achieved_gsd = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::wavelength_band::WavelengthBand, usize) = Lmcp::deser(r)?;
                out.camera_wavelength = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.horizontal_to_leading_edge = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.horizontal_to_trailing_edge = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.horizontal_to_center = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.width_center = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.slant_range_to_center = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.footprint_response_id.size();
        size += self.vehicle_id.size();
        size += self.camera_id.size();
        size += self.gimbal_id.size();
        size += self.horizontal_fov.size();
        size += self.agl_altitude.size();
        size += self.gimbal_elevation.size();
        size += self.aspect_ratio.size();
        size += self.achieved_gsd.size();
        size += self.camera_wavelength.size();
        size += self.horizontal_to_leading_edge.size();
        size += self.horizontal_to_trailing_edge.size();
        size += self.horizontal_to_center.size();
        size += self.width_center.size();
        size += self.slant_range_to_center.size();

        size
    }
}

pub trait SensorFootprintT: Debug + Send  {
    fn as_uxas_messages_task_sensor_footprint(&self) -> Option<&SensorFootprint> { None }
    fn as_mut_uxas_messages_task_sensor_footprint(&mut self) -> Option<&mut SensorFootprint> { None }
    fn footprint_response_id(&self) -> i64;
    fn footprint_response_id_mut(&mut self) -> &mut i64;
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn camera_id(&self) -> i64;
    fn camera_id_mut(&mut self) -> &mut i64;
    fn gimbal_id(&self) -> i64;
    fn gimbal_id_mut(&mut self) -> &mut i64;
    fn horizontal_fov(&self) -> f32;
    fn horizontal_fov_mut(&mut self) -> &mut f32;
    fn agl_altitude(&self) -> f32;
    fn agl_altitude_mut(&mut self) -> &mut f32;
    fn gimbal_elevation(&self) -> f32;
    fn gimbal_elevation_mut(&mut self) -> &mut f32;
    fn aspect_ratio(&self) -> f32;
    fn aspect_ratio_mut(&mut self) -> &mut f32;
    fn achieved_gsd(&self) -> f32;
    fn achieved_gsd_mut(&mut self) -> &mut f32;
    fn camera_wavelength(&self) -> ::afrl::cmasi::wavelength_band::WavelengthBand;
    fn camera_wavelength_mut(&mut self) -> &mut ::afrl::cmasi::wavelength_band::WavelengthBand;
    fn horizontal_to_leading_edge(&self) -> f32;
    fn horizontal_to_leading_edge_mut(&mut self) -> &mut f32;
    fn horizontal_to_trailing_edge(&self) -> f32;
    fn horizontal_to_trailing_edge_mut(&mut self) -> &mut f32;
    fn horizontal_to_center(&self) -> f32;
    fn horizontal_to_center_mut(&mut self) -> &mut f32;
    fn width_center(&self) -> f32;
    fn width_center_mut(&mut self) -> &mut f32;
    fn slant_range_to_center(&self) -> f32;
    fn slant_range_to_center_mut(&mut self) -> &mut f32;

}

impl Clone for Box<SensorFootprintT> {
    fn clone(&self) -> Box<SensorFootprintT> {
        if let Some(x) = SensorFootprintT::as_uxas_messages_task_sensor_footprint(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SensorFootprintT> {
    fn default() -> Box<SensorFootprintT> { Box::new(SensorFootprint::default()) }
}

impl PartialEq for Box<SensorFootprintT> {
    fn eq(&self, other: &Box<SensorFootprintT>) -> bool {
        if let (Some(x), Some(y)) =
            (SensorFootprintT::as_uxas_messages_task_sensor_footprint(self.as_ref()),
             SensorFootprintT::as_uxas_messages_task_sensor_footprint(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SensorFootprintT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SensorFootprintT::as_uxas_messages_task_sensor_footprint(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SensorFootprintT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SensorFootprint::struct_info() {
            let (x, readb) = SensorFootprint::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SensorFootprintT::as_uxas_messages_task_sensor_footprint(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl SensorFootprintT for SensorFootprint {
    fn as_uxas_messages_task_sensor_footprint(&self) -> Option<&SensorFootprint> { Some(self) }
    fn as_mut_uxas_messages_task_sensor_footprint(&mut self) -> Option<&mut SensorFootprint> { Some(self) }
    fn footprint_response_id(&self) -> i64 { self.footprint_response_id }
    fn footprint_response_id_mut(&mut self) -> &mut i64 { &mut self.footprint_response_id }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn camera_id(&self) -> i64 { self.camera_id }
    fn camera_id_mut(&mut self) -> &mut i64 { &mut self.camera_id }
    fn gimbal_id(&self) -> i64 { self.gimbal_id }
    fn gimbal_id_mut(&mut self) -> &mut i64 { &mut self.gimbal_id }
    fn horizontal_fov(&self) -> f32 { self.horizontal_fov }
    fn horizontal_fov_mut(&mut self) -> &mut f32 { &mut self.horizontal_fov }
    fn agl_altitude(&self) -> f32 { self.agl_altitude }
    fn agl_altitude_mut(&mut self) -> &mut f32 { &mut self.agl_altitude }
    fn gimbal_elevation(&self) -> f32 { self.gimbal_elevation }
    fn gimbal_elevation_mut(&mut self) -> &mut f32 { &mut self.gimbal_elevation }
    fn aspect_ratio(&self) -> f32 { self.aspect_ratio }
    fn aspect_ratio_mut(&mut self) -> &mut f32 { &mut self.aspect_ratio }
    fn achieved_gsd(&self) -> f32 { self.achieved_gsd }
    fn achieved_gsd_mut(&mut self) -> &mut f32 { &mut self.achieved_gsd }
    fn camera_wavelength(&self) -> ::afrl::cmasi::wavelength_band::WavelengthBand { self.camera_wavelength }
    fn camera_wavelength_mut(&mut self) -> &mut ::afrl::cmasi::wavelength_band::WavelengthBand { &mut self.camera_wavelength }
    fn horizontal_to_leading_edge(&self) -> f32 { self.horizontal_to_leading_edge }
    fn horizontal_to_leading_edge_mut(&mut self) -> &mut f32 { &mut self.horizontal_to_leading_edge }
    fn horizontal_to_trailing_edge(&self) -> f32 { self.horizontal_to_trailing_edge }
    fn horizontal_to_trailing_edge_mut(&mut self) -> &mut f32 { &mut self.horizontal_to_trailing_edge }
    fn horizontal_to_center(&self) -> f32 { self.horizontal_to_center }
    fn horizontal_to_center_mut(&mut self) -> &mut f32 { &mut self.horizontal_to_center }
    fn width_center(&self) -> f32 { self.width_center }
    fn width_center_mut(&mut self) -> &mut f32 { &mut self.width_center }
    fn slant_range_to_center(&self) -> f32 { self.slant_range_to_center }
    fn slant_range_to_center_mut(&mut self) -> &mut f32 { &mut self.slant_range_to_center }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SensorFootprint {
        fn arbitrary<G: Gen>(_g: &mut G) -> SensorFootprint {
            SensorFootprint {
                footprint_response_id: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                camera_id: Arbitrary::arbitrary(_g),
                gimbal_id: Arbitrary::arbitrary(_g),
                horizontal_fov: Arbitrary::arbitrary(_g),
                agl_altitude: Arbitrary::arbitrary(_g),
                gimbal_elevation: Arbitrary::arbitrary(_g),
                aspect_ratio: Arbitrary::arbitrary(_g),
                achieved_gsd: Arbitrary::arbitrary(_g),
                camera_wavelength: Arbitrary::arbitrary(_g),
                horizontal_to_leading_edge: Arbitrary::arbitrary(_g),
                horizontal_to_trailing_edge: Arbitrary::arbitrary(_g),
                horizontal_to_center: Arbitrary::arbitrary(_g),
                width_center: Arbitrary::arbitrary(_g),
                slant_range_to_center: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SensorFootprint) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SensorFootprint) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SensorFootprint::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
