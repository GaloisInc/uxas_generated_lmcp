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
pub struct RoutePlanResponse {
    pub response_id: i64,
    pub associated_task_id: i64,
    pub vehicle_id: i64,
    pub operating_region: i64,
    pub route_responses: Vec<Box<::uxas::messages::route::route_plan::RoutePlanT>>,
}

impl PartialEq for RoutePlanResponse {
    fn eq(&self, _other: &RoutePlanResponse) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.associated_task_id == &_other.associated_task_id
        && &self.vehicle_id == &_other.vehicle_id
        && &self.operating_region == &_other.operating_region
        && &self.route_responses == &_other.route_responses

    }
}

impl LmcpSubscription for RoutePlanResponse {
    fn subscription() -> &'static str { "uxas.messages.route.RoutePlanResponse" }
}

impl Struct for RoutePlanResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 8,
        }
    }
}

impl Lmcp for RoutePlanResponse {
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
            let writeb: usize = self.associated_task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.operating_region.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.route_responses.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RoutePlanResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RoutePlanResponse::struct_info() {
            let mut out: RoutePlanResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.associated_task_id = x;
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
                out.operating_region = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::route::route_plan::RoutePlanT>>, usize) = Lmcp::deser(r)?;
                out.route_responses = x;
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
        size += self.associated_task_id.size();
        size += self.vehicle_id.size();
        size += self.operating_region.size();
        size += self.route_responses.size();

        size
    }
}

pub trait RoutePlanResponseT: Debug + Send  {
    fn as_uxas_messages_route_route_plan_response(&self) -> Option<&RoutePlanResponse> { None }
    fn as_mut_uxas_messages_route_route_plan_response(&mut self) -> Option<&mut RoutePlanResponse> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn associated_task_id(&self) -> i64;
    fn associated_task_id_mut(&mut self) -> &mut i64;
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn operating_region(&self) -> i64;
    fn operating_region_mut(&mut self) -> &mut i64;
    fn route_responses(&self) -> &Vec<Box<::uxas::messages::route::route_plan::RoutePlanT>>;
    fn route_responses_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::route_plan::RoutePlanT>>;

}

impl Clone for Box<RoutePlanResponseT> {
    fn clone(&self) -> Box<RoutePlanResponseT> {
        if let Some(x) = RoutePlanResponseT::as_uxas_messages_route_route_plan_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RoutePlanResponseT> {
    fn default() -> Box<RoutePlanResponseT> { Box::new(RoutePlanResponse::default()) }
}

impl PartialEq for Box<RoutePlanResponseT> {
    fn eq(&self, other: &Box<RoutePlanResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (RoutePlanResponseT::as_uxas_messages_route_route_plan_response(self.as_ref()),
             RoutePlanResponseT::as_uxas_messages_route_route_plan_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RoutePlanResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RoutePlanResponseT::as_uxas_messages_route_route_plan_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RoutePlanResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RoutePlanResponse::struct_info() {
            let (x, readb) = RoutePlanResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RoutePlanResponseT::as_uxas_messages_route_route_plan_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RoutePlanResponseT for RoutePlanResponse {
    fn as_uxas_messages_route_route_plan_response(&self) -> Option<&RoutePlanResponse> { Some(self) }
    fn as_mut_uxas_messages_route_route_plan_response(&mut self) -> Option<&mut RoutePlanResponse> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn associated_task_id(&self) -> i64 { self.associated_task_id }
    fn associated_task_id_mut(&mut self) -> &mut i64 { &mut self.associated_task_id }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn operating_region(&self) -> i64 { self.operating_region }
    fn operating_region_mut(&mut self) -> &mut i64 { &mut self.operating_region }
    fn route_responses(&self) -> &Vec<Box<::uxas::messages::route::route_plan::RoutePlanT>> { &self.route_responses }
    fn route_responses_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::route_plan::RoutePlanT>> { &mut self.route_responses }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RoutePlanResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> RoutePlanResponse {
            RoutePlanResponse {
                response_id: Arbitrary::arbitrary(_g),
                associated_task_id: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                operating_region: Arbitrary::arbitrary(_g),
                route_responses: Vec::<::uxas::messages::route::route_plan::RoutePlan>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::route::route_plan::RoutePlanT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RoutePlanResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.route_responses.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RoutePlanResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.route_responses.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RoutePlanResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
