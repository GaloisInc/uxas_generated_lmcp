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
pub struct EgressRouteResponse {
    pub response_id: i64,
    pub node_locations: Vec<Box<::afrl::cmasi::location3d::Location3DT>>,
    pub headings: Vec<f32>,
}

impl PartialEq for EgressRouteResponse {
    fn eq(&self, _other: &EgressRouteResponse) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.node_locations == &_other.node_locations
        && &self.headings == &_other.headings

    }
}

impl LmcpSubscription for EgressRouteResponse {
    fn subscription() -> &'static str { "uxas.messages.route.EgressRouteResponse" }
}

impl Struct for EgressRouteResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 11,
        }
    }
}

impl Lmcp for EgressRouteResponse {
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
            let writeb: usize = self.node_locations.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.headings.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(EgressRouteResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == EgressRouteResponse::struct_info() {
            let mut out: EgressRouteResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.node_locations = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.headings = x;
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
        size += self.node_locations.size();
        size += self.headings.size();

        size
    }
}

pub trait EgressRouteResponseT: Debug + Send  {
    fn as_uxas_messages_route_egress_route_response(&self) -> Option<&EgressRouteResponse> { None }
    fn as_mut_uxas_messages_route_egress_route_response(&mut self) -> Option<&mut EgressRouteResponse> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn node_locations(&self) -> &Vec<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn node_locations_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn headings(&self) -> &Vec<f32>;
    fn headings_mut(&mut self) -> &mut Vec<f32>;

}

impl Clone for Box<EgressRouteResponseT> {
    fn clone(&self) -> Box<EgressRouteResponseT> {
        if let Some(x) = EgressRouteResponseT::as_uxas_messages_route_egress_route_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<EgressRouteResponseT> {
    fn default() -> Box<EgressRouteResponseT> { Box::new(EgressRouteResponse::default()) }
}

impl PartialEq for Box<EgressRouteResponseT> {
    fn eq(&self, other: &Box<EgressRouteResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (EgressRouteResponseT::as_uxas_messages_route_egress_route_response(self.as_ref()),
             EgressRouteResponseT::as_uxas_messages_route_egress_route_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<EgressRouteResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = EgressRouteResponseT::as_uxas_messages_route_egress_route_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<EgressRouteResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == EgressRouteResponse::struct_info() {
            let (x, readb) = EgressRouteResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = EgressRouteResponseT::as_uxas_messages_route_egress_route_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl EgressRouteResponseT for EgressRouteResponse {
    fn as_uxas_messages_route_egress_route_response(&self) -> Option<&EgressRouteResponse> { Some(self) }
    fn as_mut_uxas_messages_route_egress_route_response(&mut self) -> Option<&mut EgressRouteResponse> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn node_locations(&self) -> &Vec<Box<::afrl::cmasi::location3d::Location3DT>> { &self.node_locations }
    fn node_locations_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.node_locations }
    fn headings(&self) -> &Vec<f32> { &self.headings }
    fn headings_mut(&mut self) -> &mut Vec<f32> { &mut self.headings }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for EgressRouteResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> EgressRouteResponse {
            EgressRouteResponse {
                response_id: Arbitrary::arbitrary(_g),
                node_locations: Vec::<::afrl::cmasi::location3d::Location3D>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::location3d::Location3DT>).collect(),
                headings: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: EgressRouteResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.node_locations.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.headings.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: EgressRouteResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.node_locations.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.headings.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = EgressRouteResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
