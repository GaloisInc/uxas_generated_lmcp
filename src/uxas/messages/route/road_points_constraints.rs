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
pub struct RoadPointsConstraints {
    pub road_points_id: i64,
    pub start_location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub end_location: Box<::afrl::cmasi::location3d::Location3DT>,
}

impl PartialEq for RoadPointsConstraints {
    fn eq(&self, _other: &RoadPointsConstraints) -> bool {
        true
        && &self.road_points_id == &_other.road_points_id
        && &self.start_location == &_other.start_location
        && &self.end_location == &_other.end_location

    }
}

impl LmcpSubscription for RoadPointsConstraints {
    fn subscription() -> &'static str { "uxas.messages.route.RoadPointsConstraints" }
}

impl Struct for RoadPointsConstraints {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 12,
        }
    }
}

impl Lmcp for RoadPointsConstraints {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.road_points_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.end_location.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RoadPointsConstraints, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RoadPointsConstraints::struct_info() {
            let mut out: RoadPointsConstraints = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.road_points_id = x;
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
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.end_location = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.road_points_id.size();
        size += self.start_location.size();
        size += self.end_location.size();

        size
    }
}

pub trait RoadPointsConstraintsT: Debug + Send  {
    fn as_uxas_messages_route_road_points_constraints(&self) -> Option<&RoadPointsConstraints> { None }
    fn as_mut_uxas_messages_route_road_points_constraints(&mut self) -> Option<&mut RoadPointsConstraints> { None }
    fn road_points_id(&self) -> i64;
    fn road_points_id_mut(&mut self) -> &mut i64;
    fn start_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn start_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn end_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn end_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;

}

impl Clone for Box<RoadPointsConstraintsT> {
    fn clone(&self) -> Box<RoadPointsConstraintsT> {
        if let Some(x) = RoadPointsConstraintsT::as_uxas_messages_route_road_points_constraints(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RoadPointsConstraintsT> {
    fn default() -> Box<RoadPointsConstraintsT> { Box::new(RoadPointsConstraints::default()) }
}

impl PartialEq for Box<RoadPointsConstraintsT> {
    fn eq(&self, other: &Box<RoadPointsConstraintsT>) -> bool {
        if let (Some(x), Some(y)) =
            (RoadPointsConstraintsT::as_uxas_messages_route_road_points_constraints(self.as_ref()),
             RoadPointsConstraintsT::as_uxas_messages_route_road_points_constraints(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RoadPointsConstraintsT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RoadPointsConstraintsT::as_uxas_messages_route_road_points_constraints(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RoadPointsConstraintsT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RoadPointsConstraints::struct_info() {
            let (x, readb) = RoadPointsConstraints::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RoadPointsConstraintsT::as_uxas_messages_route_road_points_constraints(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RoadPointsConstraintsT for RoadPointsConstraints {
    fn as_uxas_messages_route_road_points_constraints(&self) -> Option<&RoadPointsConstraints> { Some(self) }
    fn as_mut_uxas_messages_route_road_points_constraints(&mut self) -> Option<&mut RoadPointsConstraints> { Some(self) }
    fn road_points_id(&self) -> i64 { self.road_points_id }
    fn road_points_id_mut(&mut self) -> &mut i64 { &mut self.road_points_id }
    fn start_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.start_location }
    fn start_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.start_location }
    fn end_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.end_location }
    fn end_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.end_location }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RoadPointsConstraints {
        fn arbitrary<G: Gen>(_g: &mut G) -> RoadPointsConstraints {
            RoadPointsConstraints {
                road_points_id: Arbitrary::arbitrary(_g),
                start_location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                end_location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RoadPointsConstraints) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RoadPointsConstraints) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RoadPointsConstraints::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
