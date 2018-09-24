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
pub struct BatchRoutePlanRequest {
    pub request_id: i64,
    pub vehicles: Vec<i64>,
    pub task_list: Vec<i64>,
    pub operating_region: i64,
    pub compute_task_to_task_timing: bool,
    pub compute_inter_task_to_task_timing: bool,
    pub inter_task_percentage: Vec<f32>,
}

impl PartialEq for BatchRoutePlanRequest {
    fn eq(&self, _other: &BatchRoutePlanRequest) -> bool {
        true
        && &self.request_id == &_other.request_id
        && &self.vehicles == &_other.vehicles
        && &self.task_list == &_other.task_list
        && &self.operating_region == &_other.operating_region
        && &self.compute_task_to_task_timing == &_other.compute_task_to_task_timing
        && &self.compute_inter_task_to_task_timing == &_other.compute_inter_task_to_task_timing
        && &self.inter_task_percentage == &_other.inter_task_percentage

    }
}

impl LmcpSubscription for BatchRoutePlanRequest {
    fn subscription() -> &'static str { "afrl.impact.BatchRoutePlanRequest" }
}

impl Struct for BatchRoutePlanRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 9,
        }
    }
}

impl Lmcp for BatchRoutePlanRequest {
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
            let writeb: usize = self.vehicles.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.operating_region.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.compute_task_to_task_timing.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.compute_inter_task_to_task_timing.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.inter_task_percentage.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(BatchRoutePlanRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == BatchRoutePlanRequest::struct_info() {
            let mut out: BatchRoutePlanRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.vehicles = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.operating_region = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.compute_task_to_task_timing = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.compute_inter_task_to_task_timing = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.inter_task_percentage = x;
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
        size += self.vehicles.size();
        size += self.task_list.size();
        size += self.operating_region.size();
        size += self.compute_task_to_task_timing.size();
        size += self.compute_inter_task_to_task_timing.size();
        size += self.inter_task_percentage.size();

        size
    }
}

pub trait BatchRoutePlanRequestT: Debug + Send  {
    fn as_afrl_impact_batch_route_plan_request(&self) -> Option<&BatchRoutePlanRequest> { None }
    fn as_mut_afrl_impact_batch_route_plan_request(&mut self) -> Option<&mut BatchRoutePlanRequest> { None }
    fn request_id(&self) -> i64;
    fn request_id_mut(&mut self) -> &mut i64;
    fn vehicles(&self) -> &Vec<i64>;
    fn vehicles_mut(&mut self) -> &mut Vec<i64>;
    fn task_list(&self) -> &Vec<i64>;
    fn task_list_mut(&mut self) -> &mut Vec<i64>;
    fn operating_region(&self) -> i64;
    fn operating_region_mut(&mut self) -> &mut i64;
    fn compute_task_to_task_timing(&self) -> bool;
    fn compute_task_to_task_timing_mut(&mut self) -> &mut bool;
    fn compute_inter_task_to_task_timing(&self) -> bool;
    fn compute_inter_task_to_task_timing_mut(&mut self) -> &mut bool;
    fn inter_task_percentage(&self) -> &Vec<f32>;
    fn inter_task_percentage_mut(&mut self) -> &mut Vec<f32>;

}

impl Clone for Box<BatchRoutePlanRequestT> {
    fn clone(&self) -> Box<BatchRoutePlanRequestT> {
        if let Some(x) = BatchRoutePlanRequestT::as_afrl_impact_batch_route_plan_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<BatchRoutePlanRequestT> {
    fn default() -> Box<BatchRoutePlanRequestT> { Box::new(BatchRoutePlanRequest::default()) }
}

impl PartialEq for Box<BatchRoutePlanRequestT> {
    fn eq(&self, other: &Box<BatchRoutePlanRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (BatchRoutePlanRequestT::as_afrl_impact_batch_route_plan_request(self.as_ref()),
             BatchRoutePlanRequestT::as_afrl_impact_batch_route_plan_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<BatchRoutePlanRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = BatchRoutePlanRequestT::as_afrl_impact_batch_route_plan_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<BatchRoutePlanRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == BatchRoutePlanRequest::struct_info() {
            let (x, readb) = BatchRoutePlanRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = BatchRoutePlanRequestT::as_afrl_impact_batch_route_plan_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl BatchRoutePlanRequestT for BatchRoutePlanRequest {
    fn as_afrl_impact_batch_route_plan_request(&self) -> Option<&BatchRoutePlanRequest> { Some(self) }
    fn as_mut_afrl_impact_batch_route_plan_request(&mut self) -> Option<&mut BatchRoutePlanRequest> { Some(self) }
    fn request_id(&self) -> i64 { self.request_id }
    fn request_id_mut(&mut self) -> &mut i64 { &mut self.request_id }
    fn vehicles(&self) -> &Vec<i64> { &self.vehicles }
    fn vehicles_mut(&mut self) -> &mut Vec<i64> { &mut self.vehicles }
    fn task_list(&self) -> &Vec<i64> { &self.task_list }
    fn task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.task_list }
    fn operating_region(&self) -> i64 { self.operating_region }
    fn operating_region_mut(&mut self) -> &mut i64 { &mut self.operating_region }
    fn compute_task_to_task_timing(&self) -> bool { self.compute_task_to_task_timing }
    fn compute_task_to_task_timing_mut(&mut self) -> &mut bool { &mut self.compute_task_to_task_timing }
    fn compute_inter_task_to_task_timing(&self) -> bool { self.compute_inter_task_to_task_timing }
    fn compute_inter_task_to_task_timing_mut(&mut self) -> &mut bool { &mut self.compute_inter_task_to_task_timing }
    fn inter_task_percentage(&self) -> &Vec<f32> { &self.inter_task_percentage }
    fn inter_task_percentage_mut(&mut self) -> &mut Vec<f32> { &mut self.inter_task_percentage }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for BatchRoutePlanRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> BatchRoutePlanRequest {
            BatchRoutePlanRequest {
                request_id: Arbitrary::arbitrary(_g),
                vehicles: Arbitrary::arbitrary(_g),
                task_list: Arbitrary::arbitrary(_g),
                operating_region: Arbitrary::arbitrary(_g),
                compute_task_to_task_timing: Arbitrary::arbitrary(_g),
                compute_inter_task_to_task_timing: Arbitrary::arbitrary(_g),
                inter_task_percentage: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: BatchRoutePlanRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicles.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.inter_task_percentage.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: BatchRoutePlanRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicles.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.inter_task_percentage.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = BatchRoutePlanRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
