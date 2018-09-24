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
pub struct SensorFootprintRequests {
    pub request_id: i64,
    pub footprints: Vec<Box<::uxas::messages::task::footprint_request::FootprintRequestT>>,
}

impl PartialEq for SensorFootprintRequests {
    fn eq(&self, _other: &SensorFootprintRequests) -> bool {
        true
        && &self.request_id == &_other.request_id
        && &self.footprints == &_other.footprints

    }
}

impl LmcpSubscription for SensorFootprintRequests {
    fn subscription() -> &'static str { "uxas.messages.task.SensorFootprintRequests" }
}

impl Struct for SensorFootprintRequests {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 10,
        }
    }
}

impl Lmcp for SensorFootprintRequests {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.request_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.footprints.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SensorFootprintRequests, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SensorFootprintRequests::struct_info() {
            let mut out: SensorFootprintRequests = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::task::footprint_request::FootprintRequestT>>, usize) = Lmcp::deser(r)?;
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
        size += self.request_id.size();
        size += self.footprints.size();

        size
    }
}

pub trait SensorFootprintRequestsT: Debug + Send  {
    fn as_uxas_messages_task_sensor_footprint_requests(&self) -> Option<&SensorFootprintRequests> { None }
    fn as_mut_uxas_messages_task_sensor_footprint_requests(&mut self) -> Option<&mut SensorFootprintRequests> { None }
    fn request_id(&self) -> i64;
    fn request_id_mut(&mut self) -> &mut i64;
    fn footprints(&self) -> &Vec<Box<::uxas::messages::task::footprint_request::FootprintRequestT>>;
    fn footprints_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::footprint_request::FootprintRequestT>>;

}

impl Clone for Box<SensorFootprintRequestsT> {
    fn clone(&self) -> Box<SensorFootprintRequestsT> {
        if let Some(x) = SensorFootprintRequestsT::as_uxas_messages_task_sensor_footprint_requests(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SensorFootprintRequestsT> {
    fn default() -> Box<SensorFootprintRequestsT> { Box::new(SensorFootprintRequests::default()) }
}

impl PartialEq for Box<SensorFootprintRequestsT> {
    fn eq(&self, other: &Box<SensorFootprintRequestsT>) -> bool {
        if let (Some(x), Some(y)) =
            (SensorFootprintRequestsT::as_uxas_messages_task_sensor_footprint_requests(self.as_ref()),
             SensorFootprintRequestsT::as_uxas_messages_task_sensor_footprint_requests(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SensorFootprintRequestsT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SensorFootprintRequestsT::as_uxas_messages_task_sensor_footprint_requests(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SensorFootprintRequestsT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SensorFootprintRequests::struct_info() {
            let (x, readb) = SensorFootprintRequests::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SensorFootprintRequestsT::as_uxas_messages_task_sensor_footprint_requests(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl SensorFootprintRequestsT for SensorFootprintRequests {
    fn as_uxas_messages_task_sensor_footprint_requests(&self) -> Option<&SensorFootprintRequests> { Some(self) }
    fn as_mut_uxas_messages_task_sensor_footprint_requests(&mut self) -> Option<&mut SensorFootprintRequests> { Some(self) }
    fn request_id(&self) -> i64 { self.request_id }
    fn request_id_mut(&mut self) -> &mut i64 { &mut self.request_id }
    fn footprints(&self) -> &Vec<Box<::uxas::messages::task::footprint_request::FootprintRequestT>> { &self.footprints }
    fn footprints_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::footprint_request::FootprintRequestT>> { &mut self.footprints }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SensorFootprintRequests {
        fn arbitrary<G: Gen>(_g: &mut G) -> SensorFootprintRequests {
            SensorFootprintRequests {
                request_id: Arbitrary::arbitrary(_g),
                footprints: Vec::<::uxas::messages::task::footprint_request::FootprintRequest>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::task::footprint_request::FootprintRequestT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SensorFootprintRequests) -> Result<TestResult, Error> {
            use std::u16;
            if x.footprints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SensorFootprintRequests) -> Result<TestResult, Error> {
            use std::u16;
            if x.footprints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SensorFootprintRequests::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
