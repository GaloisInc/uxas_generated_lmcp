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
pub struct RoadPointsRequest {
    pub request_id: i64,
    pub road_points_requests: Vec<Box<::uxas::messages::route::road_points_constraints::RoadPointsConstraintsT>>,
}

impl PartialEq for RoadPointsRequest {
    fn eq(&self, _other: &RoadPointsRequest) -> bool {
        true
        && &self.request_id == &_other.request_id
        && &self.road_points_requests == &_other.road_points_requests

    }
}

impl LmcpSubscription for RoadPointsRequest {
    fn subscription() -> &'static str { "uxas.messages.route.RoadPointsRequest" }
}

impl Struct for RoadPointsRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 13,
        }
    }
}

impl Lmcp for RoadPointsRequest {
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
            let writeb: usize = self.road_points_requests.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RoadPointsRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RoadPointsRequest::struct_info() {
            let mut out: RoadPointsRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::route::road_points_constraints::RoadPointsConstraintsT>>, usize) = Lmcp::deser(r)?;
                out.road_points_requests = x;
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
        size += self.road_points_requests.size();

        size
    }
}

pub trait RoadPointsRequestT: Debug + Send  {
    fn as_uxas_messages_route_road_points_request(&self) -> Option<&RoadPointsRequest> { None }
    fn as_mut_uxas_messages_route_road_points_request(&mut self) -> Option<&mut RoadPointsRequest> { None }
    fn request_id(&self) -> i64;
    fn request_id_mut(&mut self) -> &mut i64;
    fn road_points_requests(&self) -> &Vec<Box<::uxas::messages::route::road_points_constraints::RoadPointsConstraintsT>>;
    fn road_points_requests_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::road_points_constraints::RoadPointsConstraintsT>>;

}

impl Clone for Box<RoadPointsRequestT> {
    fn clone(&self) -> Box<RoadPointsRequestT> {
        if let Some(x) = RoadPointsRequestT::as_uxas_messages_route_road_points_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RoadPointsRequestT> {
    fn default() -> Box<RoadPointsRequestT> { Box::new(RoadPointsRequest::default()) }
}

impl PartialEq for Box<RoadPointsRequestT> {
    fn eq(&self, other: &Box<RoadPointsRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (RoadPointsRequestT::as_uxas_messages_route_road_points_request(self.as_ref()),
             RoadPointsRequestT::as_uxas_messages_route_road_points_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RoadPointsRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RoadPointsRequestT::as_uxas_messages_route_road_points_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RoadPointsRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RoadPointsRequest::struct_info() {
            let (x, readb) = RoadPointsRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RoadPointsRequestT::as_uxas_messages_route_road_points_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RoadPointsRequestT for RoadPointsRequest {
    fn as_uxas_messages_route_road_points_request(&self) -> Option<&RoadPointsRequest> { Some(self) }
    fn as_mut_uxas_messages_route_road_points_request(&mut self) -> Option<&mut RoadPointsRequest> { Some(self) }
    fn request_id(&self) -> i64 { self.request_id }
    fn request_id_mut(&mut self) -> &mut i64 { &mut self.request_id }
    fn road_points_requests(&self) -> &Vec<Box<::uxas::messages::route::road_points_constraints::RoadPointsConstraintsT>> { &self.road_points_requests }
    fn road_points_requests_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::road_points_constraints::RoadPointsConstraintsT>> { &mut self.road_points_requests }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RoadPointsRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> RoadPointsRequest {
            RoadPointsRequest {
                request_id: Arbitrary::arbitrary(_g),
                road_points_requests: Vec::<::uxas::messages::route::road_points_constraints::RoadPointsConstraints>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::route::road_points_constraints::RoadPointsConstraintsT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RoadPointsRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.road_points_requests.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RoadPointsRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.road_points_requests.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RoadPointsRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
