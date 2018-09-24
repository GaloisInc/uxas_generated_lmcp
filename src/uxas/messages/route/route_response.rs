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
pub struct RouteResponse {
    pub response_id: i64,
    pub routes: Vec<Box<::uxas::messages::route::route_plan_response::RoutePlanResponseT>>,
}

impl PartialEq for RouteResponse {
    fn eq(&self, _other: &RouteResponse) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.routes == &_other.routes

    }
}

impl LmcpSubscription for RouteResponse {
    fn subscription() -> &'static str { "uxas.messages.route.RouteResponse" }
}

impl Struct for RouteResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 9,
        }
    }
}

impl Lmcp for RouteResponse {
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
            let writeb: usize = self.routes.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RouteResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RouteResponse::struct_info() {
            let mut out: RouteResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::route::route_plan_response::RoutePlanResponseT>>, usize) = Lmcp::deser(r)?;
                out.routes = x;
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
        size += self.routes.size();

        size
    }
}

pub trait RouteResponseT: Debug + Send  {
    fn as_uxas_messages_route_route_response(&self) -> Option<&RouteResponse> { None }
    fn as_mut_uxas_messages_route_route_response(&mut self) -> Option<&mut RouteResponse> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn routes(&self) -> &Vec<Box<::uxas::messages::route::route_plan_response::RoutePlanResponseT>>;
    fn routes_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::route_plan_response::RoutePlanResponseT>>;

}

impl Clone for Box<RouteResponseT> {
    fn clone(&self) -> Box<RouteResponseT> {
        if let Some(x) = RouteResponseT::as_uxas_messages_route_route_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RouteResponseT> {
    fn default() -> Box<RouteResponseT> { Box::new(RouteResponse::default()) }
}

impl PartialEq for Box<RouteResponseT> {
    fn eq(&self, other: &Box<RouteResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (RouteResponseT::as_uxas_messages_route_route_response(self.as_ref()),
             RouteResponseT::as_uxas_messages_route_route_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RouteResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RouteResponseT::as_uxas_messages_route_route_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RouteResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RouteResponse::struct_info() {
            let (x, readb) = RouteResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RouteResponseT::as_uxas_messages_route_route_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RouteResponseT for RouteResponse {
    fn as_uxas_messages_route_route_response(&self) -> Option<&RouteResponse> { Some(self) }
    fn as_mut_uxas_messages_route_route_response(&mut self) -> Option<&mut RouteResponse> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn routes(&self) -> &Vec<Box<::uxas::messages::route::route_plan_response::RoutePlanResponseT>> { &self.routes }
    fn routes_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::route_plan_response::RoutePlanResponseT>> { &mut self.routes }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RouteResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> RouteResponse {
            RouteResponse {
                response_id: Arbitrary::arbitrary(_g),
                routes: Vec::<::uxas::messages::route::route_plan_response::RoutePlanResponse>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::route::route_plan_response::RoutePlanResponseT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RouteResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.routes.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RouteResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.routes.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RouteResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
