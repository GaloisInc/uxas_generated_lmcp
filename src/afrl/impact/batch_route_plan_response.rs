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
pub struct BatchRoutePlanResponse {
    pub response_id: i64,
    pub vehicle_timing: Vec<Box<::afrl::impact::task_timing_pair::TaskTimingPairT>>,
}

impl PartialEq for BatchRoutePlanResponse {
    fn eq(&self, _other: &BatchRoutePlanResponse) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.vehicle_timing == &_other.vehicle_timing

    }
}

impl LmcpSubscription for BatchRoutePlanResponse {
    fn subscription() -> &'static str { "afrl.impact.BatchRoutePlanResponse" }
}

impl Struct for BatchRoutePlanResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 10,
        }
    }
}

impl Lmcp for BatchRoutePlanResponse {
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
            let writeb: usize = self.vehicle_timing.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(BatchRoutePlanResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == BatchRoutePlanResponse::struct_info() {
            let mut out: BatchRoutePlanResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::impact::task_timing_pair::TaskTimingPairT>>, usize) = Lmcp::deser(r)?;
                out.vehicle_timing = x;
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
        size += self.vehicle_timing.size();

        size
    }
}

pub trait BatchRoutePlanResponseT: Debug + Send  {
    fn as_afrl_impact_batch_route_plan_response(&self) -> Option<&BatchRoutePlanResponse> { None }
    fn as_mut_afrl_impact_batch_route_plan_response(&mut self) -> Option<&mut BatchRoutePlanResponse> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn vehicle_timing(&self) -> &Vec<Box<::afrl::impact::task_timing_pair::TaskTimingPairT>>;
    fn vehicle_timing_mut(&mut self) -> &mut Vec<Box<::afrl::impact::task_timing_pair::TaskTimingPairT>>;

}

impl Clone for Box<BatchRoutePlanResponseT> {
    fn clone(&self) -> Box<BatchRoutePlanResponseT> {
        if let Some(x) = BatchRoutePlanResponseT::as_afrl_impact_batch_route_plan_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<BatchRoutePlanResponseT> {
    fn default() -> Box<BatchRoutePlanResponseT> { Box::new(BatchRoutePlanResponse::default()) }
}

impl PartialEq for Box<BatchRoutePlanResponseT> {
    fn eq(&self, other: &Box<BatchRoutePlanResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (BatchRoutePlanResponseT::as_afrl_impact_batch_route_plan_response(self.as_ref()),
             BatchRoutePlanResponseT::as_afrl_impact_batch_route_plan_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<BatchRoutePlanResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = BatchRoutePlanResponseT::as_afrl_impact_batch_route_plan_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<BatchRoutePlanResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == BatchRoutePlanResponse::struct_info() {
            let (x, readb) = BatchRoutePlanResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = BatchRoutePlanResponseT::as_afrl_impact_batch_route_plan_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl BatchRoutePlanResponseT for BatchRoutePlanResponse {
    fn as_afrl_impact_batch_route_plan_response(&self) -> Option<&BatchRoutePlanResponse> { Some(self) }
    fn as_mut_afrl_impact_batch_route_plan_response(&mut self) -> Option<&mut BatchRoutePlanResponse> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn vehicle_timing(&self) -> &Vec<Box<::afrl::impact::task_timing_pair::TaskTimingPairT>> { &self.vehicle_timing }
    fn vehicle_timing_mut(&mut self) -> &mut Vec<Box<::afrl::impact::task_timing_pair::TaskTimingPairT>> { &mut self.vehicle_timing }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for BatchRoutePlanResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> BatchRoutePlanResponse {
            BatchRoutePlanResponse {
                response_id: Arbitrary::arbitrary(_g),
                vehicle_timing: Vec::<::afrl::impact::task_timing_pair::TaskTimingPair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::impact::task_timing_pair::TaskTimingPairT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: BatchRoutePlanResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_timing.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: BatchRoutePlanResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_timing.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = BatchRoutePlanResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
