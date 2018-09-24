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
pub struct RouteRequest {
    pub request_id: i64,
    pub associated_task_id: i64,
    pub vehicle_id: Vec<i64>,
    pub operating_region: i64,
    pub route_requests: Vec<Box<::uxas::messages::route::route_constraints::RouteConstraintsT>>,
    pub is_cost_only_request: bool,
}

impl PartialEq for RouteRequest {
    fn eq(&self, _other: &RouteRequest) -> bool {
        true
        && &self.request_id == &_other.request_id
        && &self.associated_task_id == &_other.associated_task_id
        && &self.vehicle_id == &_other.vehicle_id
        && &self.operating_region == &_other.operating_region
        && &self.route_requests == &_other.route_requests
        && &self.is_cost_only_request == &_other.is_cost_only_request

    }
}

impl LmcpSubscription for RouteRequest {
    fn subscription() -> &'static str { "uxas.messages.route.RouteRequest" }
}

impl Struct for RouteRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 5,
        }
    }
}

impl Lmcp for RouteRequest {
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
            let writeb: usize = self.route_requests.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.is_cost_only_request.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RouteRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RouteRequest::struct_info() {
            let mut out: RouteRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.request_id = x;
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
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
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
                let (x, readb): (Vec<Box<::uxas::messages::route::route_constraints::RouteConstraintsT>>, usize) = Lmcp::deser(r)?;
                out.route_requests = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.is_cost_only_request = x;
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
        size += self.associated_task_id.size();
        size += self.vehicle_id.size();
        size += self.operating_region.size();
        size += self.route_requests.size();
        size += self.is_cost_only_request.size();

        size
    }
}

pub trait RouteRequestT: Debug + Send  {
    fn as_uxas_messages_route_route_request(&self) -> Option<&RouteRequest> { None }
    fn as_mut_uxas_messages_route_route_request(&mut self) -> Option<&mut RouteRequest> { None }
    fn request_id(&self) -> i64;
    fn request_id_mut(&mut self) -> &mut i64;
    fn associated_task_id(&self) -> i64;
    fn associated_task_id_mut(&mut self) -> &mut i64;
    fn vehicle_id(&self) -> &Vec<i64>;
    fn vehicle_id_mut(&mut self) -> &mut Vec<i64>;
    fn operating_region(&self) -> i64;
    fn operating_region_mut(&mut self) -> &mut i64;
    fn route_requests(&self) -> &Vec<Box<::uxas::messages::route::route_constraints::RouteConstraintsT>>;
    fn route_requests_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::route_constraints::RouteConstraintsT>>;
    fn is_cost_only_request(&self) -> bool;
    fn is_cost_only_request_mut(&mut self) -> &mut bool;

}

impl Clone for Box<RouteRequestT> {
    fn clone(&self) -> Box<RouteRequestT> {
        if let Some(x) = RouteRequestT::as_uxas_messages_route_route_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RouteRequestT> {
    fn default() -> Box<RouteRequestT> { Box::new(RouteRequest::default()) }
}

impl PartialEq for Box<RouteRequestT> {
    fn eq(&self, other: &Box<RouteRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (RouteRequestT::as_uxas_messages_route_route_request(self.as_ref()),
             RouteRequestT::as_uxas_messages_route_route_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RouteRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RouteRequestT::as_uxas_messages_route_route_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RouteRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RouteRequest::struct_info() {
            let (x, readb) = RouteRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RouteRequestT::as_uxas_messages_route_route_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RouteRequestT for RouteRequest {
    fn as_uxas_messages_route_route_request(&self) -> Option<&RouteRequest> { Some(self) }
    fn as_mut_uxas_messages_route_route_request(&mut self) -> Option<&mut RouteRequest> { Some(self) }
    fn request_id(&self) -> i64 { self.request_id }
    fn request_id_mut(&mut self) -> &mut i64 { &mut self.request_id }
    fn associated_task_id(&self) -> i64 { self.associated_task_id }
    fn associated_task_id_mut(&mut self) -> &mut i64 { &mut self.associated_task_id }
    fn vehicle_id(&self) -> &Vec<i64> { &self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut Vec<i64> { &mut self.vehicle_id }
    fn operating_region(&self) -> i64 { self.operating_region }
    fn operating_region_mut(&mut self) -> &mut i64 { &mut self.operating_region }
    fn route_requests(&self) -> &Vec<Box<::uxas::messages::route::route_constraints::RouteConstraintsT>> { &self.route_requests }
    fn route_requests_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::route_constraints::RouteConstraintsT>> { &mut self.route_requests }
    fn is_cost_only_request(&self) -> bool { self.is_cost_only_request }
    fn is_cost_only_request_mut(&mut self) -> &mut bool { &mut self.is_cost_only_request }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RouteRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> RouteRequest {
            RouteRequest {
                request_id: Arbitrary::arbitrary(_g),
                associated_task_id: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                operating_region: Arbitrary::arbitrary(_g),
                route_requests: Vec::<::uxas::messages::route::route_constraints::RouteConstraints>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::route::route_constraints::RouteConstraintsT>).collect(),
                is_cost_only_request: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RouteRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_id.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.route_requests.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RouteRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_id.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.route_requests.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RouteRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
