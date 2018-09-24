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
pub struct RoadPointsResponse {
    pub response_id: i64,
    pub road_points_responses: Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>>,
}

impl PartialEq for RoadPointsResponse {
    fn eq(&self, _other: &RoadPointsResponse) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.road_points_responses == &_other.road_points_responses

    }
}

impl LmcpSubscription for RoadPointsResponse {
    fn subscription() -> &'static str { "uxas.messages.route.RoadPointsResponse" }
}

impl Struct for RoadPointsResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 14,
        }
    }
}

impl Lmcp for RoadPointsResponse {
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
            let writeb: usize = self.road_points_responses.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RoadPointsResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RoadPointsResponse::struct_info() {
            let mut out: RoadPointsResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>>, usize) = Lmcp::deser(r)?;
                out.road_points_responses = x;
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
        size += self.road_points_responses.size();

        size
    }
}

pub trait RoadPointsResponseT: Debug + Send  {
    fn as_uxas_messages_route_road_points_response(&self) -> Option<&RoadPointsResponse> { None }
    fn as_mut_uxas_messages_route_road_points_response(&mut self) -> Option<&mut RoadPointsResponse> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn road_points_responses(&self) -> &Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>>;
    fn road_points_responses_mut(&mut self) -> &mut Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>>;

}

impl Clone for Box<RoadPointsResponseT> {
    fn clone(&self) -> Box<RoadPointsResponseT> {
        if let Some(x) = RoadPointsResponseT::as_uxas_messages_route_road_points_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RoadPointsResponseT> {
    fn default() -> Box<RoadPointsResponseT> { Box::new(RoadPointsResponse::default()) }
}

impl PartialEq for Box<RoadPointsResponseT> {
    fn eq(&self, other: &Box<RoadPointsResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (RoadPointsResponseT::as_uxas_messages_route_road_points_response(self.as_ref()),
             RoadPointsResponseT::as_uxas_messages_route_road_points_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RoadPointsResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RoadPointsResponseT::as_uxas_messages_route_road_points_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RoadPointsResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RoadPointsResponse::struct_info() {
            let (x, readb) = RoadPointsResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RoadPointsResponseT::as_uxas_messages_route_road_points_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RoadPointsResponseT for RoadPointsResponse {
    fn as_uxas_messages_route_road_points_response(&self) -> Option<&RoadPointsResponse> { Some(self) }
    fn as_mut_uxas_messages_route_road_points_response(&mut self) -> Option<&mut RoadPointsResponse> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn road_points_responses(&self) -> &Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>> { &self.road_points_responses }
    fn road_points_responses_mut(&mut self) -> &mut Vec<Box<::afrl::impact::line_of_interest::LineOfInterestT>> { &mut self.road_points_responses }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RoadPointsResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> RoadPointsResponse {
            RoadPointsResponse {
                response_id: Arbitrary::arbitrary(_g),
                road_points_responses: Vec::<::afrl::impact::line_of_interest::LineOfInterest>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::impact::line_of_interest::LineOfInterestT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RoadPointsResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.road_points_responses.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RoadPointsResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.road_points_responses.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RoadPointsResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
