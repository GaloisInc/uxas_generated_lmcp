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
pub struct EgressRouteRequest {
    pub request_id: i64,
    pub start_location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub radius: f32,
}

impl PartialEq for EgressRouteRequest {
    fn eq(&self, _other: &EgressRouteRequest) -> bool {
        true
        && &self.request_id == &_other.request_id
        && &self.start_location == &_other.start_location
        && &self.radius == &_other.radius

    }
}

impl LmcpSubscription for EgressRouteRequest {
    fn subscription() -> &'static str { "uxas.messages.route.EgressRouteRequest" }
}

impl Struct for EgressRouteRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 10,
        }
    }
}

impl Lmcp for EgressRouteRequest {
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
            let writeb: usize = self.start_location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.radius.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(EgressRouteRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == EgressRouteRequest::struct_info() {
            let mut out: EgressRouteRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.start_location = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.radius = x;
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
        size += self.start_location.size();
        size += self.radius.size();

        size
    }
}

pub trait EgressRouteRequestT: Debug + Send  {
    fn as_uxas_messages_route_egress_route_request(&self) -> Option<&EgressRouteRequest> { None }
    fn as_mut_uxas_messages_route_egress_route_request(&mut self) -> Option<&mut EgressRouteRequest> { None }
    fn request_id(&self) -> i64;
    fn request_id_mut(&mut self) -> &mut i64;
    fn start_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn start_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn radius(&self) -> f32;
    fn radius_mut(&mut self) -> &mut f32;

}

impl Clone for Box<EgressRouteRequestT> {
    fn clone(&self) -> Box<EgressRouteRequestT> {
        if let Some(x) = EgressRouteRequestT::as_uxas_messages_route_egress_route_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<EgressRouteRequestT> {
    fn default() -> Box<EgressRouteRequestT> { Box::new(EgressRouteRequest::default()) }
}

impl PartialEq for Box<EgressRouteRequestT> {
    fn eq(&self, other: &Box<EgressRouteRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (EgressRouteRequestT::as_uxas_messages_route_egress_route_request(self.as_ref()),
             EgressRouteRequestT::as_uxas_messages_route_egress_route_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<EgressRouteRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = EgressRouteRequestT::as_uxas_messages_route_egress_route_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<EgressRouteRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == EgressRouteRequest::struct_info() {
            let (x, readb) = EgressRouteRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = EgressRouteRequestT::as_uxas_messages_route_egress_route_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl EgressRouteRequestT for EgressRouteRequest {
    fn as_uxas_messages_route_egress_route_request(&self) -> Option<&EgressRouteRequest> { Some(self) }
    fn as_mut_uxas_messages_route_egress_route_request(&mut self) -> Option<&mut EgressRouteRequest> { Some(self) }
    fn request_id(&self) -> i64 { self.request_id }
    fn request_id_mut(&mut self) -> &mut i64 { &mut self.request_id }
    fn start_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.start_location }
    fn start_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.start_location }
    fn radius(&self) -> f32 { self.radius }
    fn radius_mut(&mut self) -> &mut f32 { &mut self.radius }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for EgressRouteRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> EgressRouteRequest {
            EgressRouteRequest {
                request_id: Arbitrary::arbitrary(_g),
                start_location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                radius: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: EgressRouteRequest) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: EgressRouteRequest) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = EgressRouteRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
