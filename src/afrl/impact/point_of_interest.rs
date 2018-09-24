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
pub struct PointOfInterest {
    pub point_id: i64,
    pub location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub point_action: ::afrl::impact::area_action_options::AreaActionOptions,
    pub point_label: Vec<u8>,
    pub background_behavior_point: bool,
}

impl PartialEq for PointOfInterest {
    fn eq(&self, _other: &PointOfInterest) -> bool {
        true
        && &self.point_id == &_other.point_id
        && &self.location == &_other.location
        && &self.point_action == &_other.point_action
        && &self.point_label == &_other.point_label
        && &self.background_behavior_point == &_other.background_behavior_point

    }
}

impl LmcpSubscription for PointOfInterest {
    fn subscription() -> &'static str { "afrl.impact.PointOfInterest" }
}

impl Struct for PointOfInterest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 19,
        }
    }
}

impl Lmcp for PointOfInterest {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.point_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.point_action.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.point_label.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.background_behavior_point.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PointOfInterest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PointOfInterest::struct_info() {
            let mut out: PointOfInterest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.point_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.location = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::impact::area_action_options::AreaActionOptions, usize) = Lmcp::deser(r)?;
                out.point_action = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.point_label = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.background_behavior_point = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.point_id.size();
        size += self.location.size();
        size += self.point_action.size();
        size += self.point_label.size();
        size += self.background_behavior_point.size();

        size
    }
}

pub trait PointOfInterestT: Debug + Send  {
    fn as_afrl_impact_point_of_interest(&self) -> Option<&PointOfInterest> { None }
    fn as_mut_afrl_impact_point_of_interest(&mut self) -> Option<&mut PointOfInterest> { None }
    fn point_id(&self) -> i64;
    fn point_id_mut(&mut self) -> &mut i64;
    fn location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn point_action(&self) -> ::afrl::impact::area_action_options::AreaActionOptions;
    fn point_action_mut(&mut self) -> &mut ::afrl::impact::area_action_options::AreaActionOptions;
    fn point_label(&self) -> &Vec<u8>;
    fn point_label_mut(&mut self) -> &mut Vec<u8>;
    fn background_behavior_point(&self) -> bool;
    fn background_behavior_point_mut(&mut self) -> &mut bool;

}

impl Clone for Box<PointOfInterestT> {
    fn clone(&self) -> Box<PointOfInterestT> {
        if let Some(x) = PointOfInterestT::as_afrl_impact_point_of_interest(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PointOfInterestT> {
    fn default() -> Box<PointOfInterestT> { Box::new(PointOfInterest::default()) }
}

impl PartialEq for Box<PointOfInterestT> {
    fn eq(&self, other: &Box<PointOfInterestT>) -> bool {
        if let (Some(x), Some(y)) =
            (PointOfInterestT::as_afrl_impact_point_of_interest(self.as_ref()),
             PointOfInterestT::as_afrl_impact_point_of_interest(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PointOfInterestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PointOfInterestT::as_afrl_impact_point_of_interest(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PointOfInterestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PointOfInterest::struct_info() {
            let (x, readb) = PointOfInterest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PointOfInterestT::as_afrl_impact_point_of_interest(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl PointOfInterestT for PointOfInterest {
    fn as_afrl_impact_point_of_interest(&self) -> Option<&PointOfInterest> { Some(self) }
    fn as_mut_afrl_impact_point_of_interest(&mut self) -> Option<&mut PointOfInterest> { Some(self) }
    fn point_id(&self) -> i64 { self.point_id }
    fn point_id_mut(&mut self) -> &mut i64 { &mut self.point_id }
    fn location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.location }
    fn location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.location }
    fn point_action(&self) -> ::afrl::impact::area_action_options::AreaActionOptions { self.point_action }
    fn point_action_mut(&mut self) -> &mut ::afrl::impact::area_action_options::AreaActionOptions { &mut self.point_action }
    fn point_label(&self) -> &Vec<u8> { &self.point_label }
    fn point_label_mut(&mut self) -> &mut Vec<u8> { &mut self.point_label }
    fn background_behavior_point(&self) -> bool { self.background_behavior_point }
    fn background_behavior_point_mut(&mut self) -> &mut bool { &mut self.background_behavior_point }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PointOfInterest {
        fn arbitrary<G: Gen>(_g: &mut G) -> PointOfInterest {
            PointOfInterest {
                point_id: Arbitrary::arbitrary(_g),
                location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                point_action: Arbitrary::arbitrary(_g),
                point_label: Arbitrary::arbitrary(_g),
                background_behavior_point: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PointOfInterest) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PointOfInterest) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PointOfInterest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
