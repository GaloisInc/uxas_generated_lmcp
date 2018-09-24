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
pub struct RouteConstraints {
    pub route_id: i64,
    pub start_location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub start_heading: f32,
    pub use_start_heading: bool,
    pub end_location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub end_heading: f32,
    pub use_end_heading: bool,
}

impl PartialEq for RouteConstraints {
    fn eq(&self, _other: &RouteConstraints) -> bool {
        true
        && &self.route_id == &_other.route_id
        && &self.start_location == &_other.start_location
        && &self.start_heading == &_other.start_heading
        && &self.use_start_heading == &_other.use_start_heading
        && &self.end_location == &_other.end_location
        && &self.end_heading == &_other.end_heading
        && &self.use_end_heading == &_other.use_end_heading

    }
}

impl LmcpSubscription for RouteConstraints {
    fn subscription() -> &'static str { "uxas.messages.route.RouteConstraints" }
}

impl Struct for RouteConstraints {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 4,
        }
    }
}

impl Lmcp for RouteConstraints {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.route_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_heading.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.use_start_heading.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.end_location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.end_heading.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.use_end_heading.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RouteConstraints, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RouteConstraints::struct_info() {
            let mut out: RouteConstraints = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.route_id = x;
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
                out.start_heading = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.use_start_heading = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.end_location = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.end_heading = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.use_end_heading = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.route_id.size();
        size += self.start_location.size();
        size += self.start_heading.size();
        size += self.use_start_heading.size();
        size += self.end_location.size();
        size += self.end_heading.size();
        size += self.use_end_heading.size();

        size
    }
}

pub trait RouteConstraintsT: Debug + Send  {
    fn as_uxas_messages_route_route_constraints(&self) -> Option<&RouteConstraints> { None }
    fn as_mut_uxas_messages_route_route_constraints(&mut self) -> Option<&mut RouteConstraints> { None }
    fn route_id(&self) -> i64;
    fn route_id_mut(&mut self) -> &mut i64;
    fn start_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn start_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn start_heading(&self) -> f32;
    fn start_heading_mut(&mut self) -> &mut f32;
    fn use_start_heading(&self) -> bool;
    fn use_start_heading_mut(&mut self) -> &mut bool;
    fn end_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn end_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn end_heading(&self) -> f32;
    fn end_heading_mut(&mut self) -> &mut f32;
    fn use_end_heading(&self) -> bool;
    fn use_end_heading_mut(&mut self) -> &mut bool;

}

impl Clone for Box<RouteConstraintsT> {
    fn clone(&self) -> Box<RouteConstraintsT> {
        if let Some(x) = RouteConstraintsT::as_uxas_messages_route_route_constraints(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RouteConstraintsT> {
    fn default() -> Box<RouteConstraintsT> { Box::new(RouteConstraints::default()) }
}

impl PartialEq for Box<RouteConstraintsT> {
    fn eq(&self, other: &Box<RouteConstraintsT>) -> bool {
        if let (Some(x), Some(y)) =
            (RouteConstraintsT::as_uxas_messages_route_route_constraints(self.as_ref()),
             RouteConstraintsT::as_uxas_messages_route_route_constraints(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RouteConstraintsT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RouteConstraintsT::as_uxas_messages_route_route_constraints(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RouteConstraintsT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RouteConstraints::struct_info() {
            let (x, readb) = RouteConstraints::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RouteConstraintsT::as_uxas_messages_route_route_constraints(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RouteConstraintsT for RouteConstraints {
    fn as_uxas_messages_route_route_constraints(&self) -> Option<&RouteConstraints> { Some(self) }
    fn as_mut_uxas_messages_route_route_constraints(&mut self) -> Option<&mut RouteConstraints> { Some(self) }
    fn route_id(&self) -> i64 { self.route_id }
    fn route_id_mut(&mut self) -> &mut i64 { &mut self.route_id }
    fn start_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.start_location }
    fn start_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.start_location }
    fn start_heading(&self) -> f32 { self.start_heading }
    fn start_heading_mut(&mut self) -> &mut f32 { &mut self.start_heading }
    fn use_start_heading(&self) -> bool { self.use_start_heading }
    fn use_start_heading_mut(&mut self) -> &mut bool { &mut self.use_start_heading }
    fn end_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.end_location }
    fn end_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.end_location }
    fn end_heading(&self) -> f32 { self.end_heading }
    fn end_heading_mut(&mut self) -> &mut f32 { &mut self.end_heading }
    fn use_end_heading(&self) -> bool { self.use_end_heading }
    fn use_end_heading_mut(&mut self) -> &mut bool { &mut self.use_end_heading }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RouteConstraints {
        fn arbitrary<G: Gen>(_g: &mut G) -> RouteConstraints {
            RouteConstraints {
                route_id: Arbitrary::arbitrary(_g),
                start_location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                start_heading: Arbitrary::arbitrary(_g),
                use_start_heading: Arbitrary::arbitrary(_g),
                end_location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                end_heading: Arbitrary::arbitrary(_g),
                use_end_heading: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RouteConstraints) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RouteConstraints) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RouteConstraints::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
