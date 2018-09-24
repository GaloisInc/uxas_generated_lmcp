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
pub struct SensorFootprintResponse {
    pub response_id: i64,
    pub footprints: Vec<Box<::uxas::messages::task::sensor_footprint::SensorFootprintT>>,
}

impl PartialEq for SensorFootprintResponse {
    fn eq(&self, _other: &SensorFootprintResponse) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.footprints == &_other.footprints

    }
}

impl LmcpSubscription for SensorFootprintResponse {
    fn subscription() -> &'static str { "uxas.messages.task.SensorFootprintResponse" }
}

impl Struct for SensorFootprintResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 13,
        }
    }
}

impl Lmcp for SensorFootprintResponse {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.response_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.footprints.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SensorFootprintResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SensorFootprintResponse::struct_info() {
            let mut out: SensorFootprintResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::task::sensor_footprint::SensorFootprintT>>, usize) = Lmcp::deser(r)?;
                out.footprints = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.response_id.size();
        size += self.footprints.size();

        size
    }
}

pub trait SensorFootprintResponseT: Debug + Send  {
    fn as_uxas_messages_task_sensor_footprint_response(&self) -> Option<&SensorFootprintResponse> { None }
    fn as_mut_uxas_messages_task_sensor_footprint_response(&mut self) -> Option<&mut SensorFootprintResponse> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn footprints(&self) -> &Vec<Box<::uxas::messages::task::sensor_footprint::SensorFootprintT>>;
    fn footprints_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::sensor_footprint::SensorFootprintT>>;

}

impl Clone for Box<SensorFootprintResponseT> {
    fn clone(&self) -> Box<SensorFootprintResponseT> {
        if let Some(x) = SensorFootprintResponseT::as_uxas_messages_task_sensor_footprint_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SensorFootprintResponseT> {
    fn default() -> Box<SensorFootprintResponseT> { Box::new(SensorFootprintResponse::default()) }
}

impl PartialEq for Box<SensorFootprintResponseT> {
    fn eq(&self, other: &Box<SensorFootprintResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (SensorFootprintResponseT::as_uxas_messages_task_sensor_footprint_response(self.as_ref()),
             SensorFootprintResponseT::as_uxas_messages_task_sensor_footprint_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SensorFootprintResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SensorFootprintResponseT::as_uxas_messages_task_sensor_footprint_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SensorFootprintResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SensorFootprintResponse::struct_info() {
            let (x, readb) = SensorFootprintResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SensorFootprintResponseT::as_uxas_messages_task_sensor_footprint_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl SensorFootprintResponseT for SensorFootprintResponse {
    fn as_uxas_messages_task_sensor_footprint_response(&self) -> Option<&SensorFootprintResponse> { Some(self) }
    fn as_mut_uxas_messages_task_sensor_footprint_response(&mut self) -> Option<&mut SensorFootprintResponse> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn footprints(&self) -> &Vec<Box<::uxas::messages::task::sensor_footprint::SensorFootprintT>> { &self.footprints }
    fn footprints_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::sensor_footprint::SensorFootprintT>> { &mut self.footprints }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SensorFootprintResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> SensorFootprintResponse {
            SensorFootprintResponse {
                response_id: Arbitrary::arbitrary(_g),
                footprints: Vec::<::uxas::messages::task::sensor_footprint::SensorFootprint>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::task::sensor_footprint::SensorFootprintT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SensorFootprintResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.footprints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SensorFootprintResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.footprints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SensorFootprintResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
