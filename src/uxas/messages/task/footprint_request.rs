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
pub struct FootprintRequest {
    pub footprint_request_id: i64,
    pub vehicle_id: i64,
    pub eligible_wavelengths: Vec<::afrl::cmasi::wavelength_band::WavelengthBand>,
    pub ground_sample_distances: Vec<f32>,
    pub agl_altitudes: Vec<f32>,
    pub elevation_angles: Vec<f32>,
}

impl PartialEq for FootprintRequest {
    fn eq(&self, _other: &FootprintRequest) -> bool {
        true
        && &self.footprint_request_id == &_other.footprint_request_id
        && &self.vehicle_id == &_other.vehicle_id
        && &self.eligible_wavelengths == &_other.eligible_wavelengths
        && &self.ground_sample_distances == &_other.ground_sample_distances
        && &self.agl_altitudes == &_other.agl_altitudes
        && &self.elevation_angles == &_other.elevation_angles

    }
}

impl LmcpSubscription for FootprintRequest {
    fn subscription() -> &'static str { "uxas.messages.task.FootprintRequest" }
}

impl Struct for FootprintRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 11,
        }
    }
}

impl Lmcp for FootprintRequest {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.footprint_request_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.eligible_wavelengths.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.ground_sample_distances.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.agl_altitudes.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.elevation_angles.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(FootprintRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == FootprintRequest::struct_info() {
            let mut out: FootprintRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.footprint_request_id = x;
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
                let (x, readb): (Vec<::afrl::cmasi::wavelength_band::WavelengthBand>, usize) = Lmcp::deser(r)?;
                out.eligible_wavelengths = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.ground_sample_distances = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.agl_altitudes = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.elevation_angles = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.footprint_request_id.size();
        size += self.vehicle_id.size();
        size += self.eligible_wavelengths.size();
        size += self.ground_sample_distances.size();
        size += self.agl_altitudes.size();
        size += self.elevation_angles.size();

        size
    }
}

pub trait FootprintRequestT: Debug + Send  {
    fn as_uxas_messages_task_footprint_request(&self) -> Option<&FootprintRequest> { None }
    fn as_mut_uxas_messages_task_footprint_request(&mut self) -> Option<&mut FootprintRequest> { None }
    fn footprint_request_id(&self) -> i64;
    fn footprint_request_id_mut(&mut self) -> &mut i64;
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn eligible_wavelengths(&self) -> &Vec<::afrl::cmasi::wavelength_band::WavelengthBand>;
    fn eligible_wavelengths_mut(&mut self) -> &mut Vec<::afrl::cmasi::wavelength_band::WavelengthBand>;
    fn ground_sample_distances(&self) -> &Vec<f32>;
    fn ground_sample_distances_mut(&mut self) -> &mut Vec<f32>;
    fn agl_altitudes(&self) -> &Vec<f32>;
    fn agl_altitudes_mut(&mut self) -> &mut Vec<f32>;
    fn elevation_angles(&self) -> &Vec<f32>;
    fn elevation_angles_mut(&mut self) -> &mut Vec<f32>;

}

impl Clone for Box<FootprintRequestT> {
    fn clone(&self) -> Box<FootprintRequestT> {
        if let Some(x) = FootprintRequestT::as_uxas_messages_task_footprint_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<FootprintRequestT> {
    fn default() -> Box<FootprintRequestT> { Box::new(FootprintRequest::default()) }
}

impl PartialEq for Box<FootprintRequestT> {
    fn eq(&self, other: &Box<FootprintRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (FootprintRequestT::as_uxas_messages_task_footprint_request(self.as_ref()),
             FootprintRequestT::as_uxas_messages_task_footprint_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<FootprintRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = FootprintRequestT::as_uxas_messages_task_footprint_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<FootprintRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == FootprintRequest::struct_info() {
            let (x, readb) = FootprintRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = FootprintRequestT::as_uxas_messages_task_footprint_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl FootprintRequestT for FootprintRequest {
    fn as_uxas_messages_task_footprint_request(&self) -> Option<&FootprintRequest> { Some(self) }
    fn as_mut_uxas_messages_task_footprint_request(&mut self) -> Option<&mut FootprintRequest> { Some(self) }
    fn footprint_request_id(&self) -> i64 { self.footprint_request_id }
    fn footprint_request_id_mut(&mut self) -> &mut i64 { &mut self.footprint_request_id }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn eligible_wavelengths(&self) -> &Vec<::afrl::cmasi::wavelength_band::WavelengthBand> { &self.eligible_wavelengths }
    fn eligible_wavelengths_mut(&mut self) -> &mut Vec<::afrl::cmasi::wavelength_band::WavelengthBand> { &mut self.eligible_wavelengths }
    fn ground_sample_distances(&self) -> &Vec<f32> { &self.ground_sample_distances }
    fn ground_sample_distances_mut(&mut self) -> &mut Vec<f32> { &mut self.ground_sample_distances }
    fn agl_altitudes(&self) -> &Vec<f32> { &self.agl_altitudes }
    fn agl_altitudes_mut(&mut self) -> &mut Vec<f32> { &mut self.agl_altitudes }
    fn elevation_angles(&self) -> &Vec<f32> { &self.elevation_angles }
    fn elevation_angles_mut(&mut self) -> &mut Vec<f32> { &mut self.elevation_angles }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for FootprintRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> FootprintRequest {
            FootprintRequest {
                footprint_request_id: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                eligible_wavelengths: Arbitrary::arbitrary(_g),
                ground_sample_distances: Arbitrary::arbitrary(_g),
                agl_altitudes: Arbitrary::arbitrary(_g),
                elevation_angles: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: FootprintRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_wavelengths.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.ground_sample_distances.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.agl_altitudes.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.elevation_angles.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: FootprintRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_wavelengths.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.ground_sample_distances.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.agl_altitudes.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.elevation_angles.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = FootprintRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
