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
pub struct AreaOfInterest {
    pub area_id: i64,
    pub area: Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>,
    pub area_action: ::afrl::impact::area_action_options::AreaActionOptions,
    pub area_label: Vec<u8>,
    pub background_behavior_area: bool,
}

impl PartialEq for AreaOfInterest {
    fn eq(&self, _other: &AreaOfInterest) -> bool {
        true
        && &self.area_id == &_other.area_id
        && &self.area == &_other.area
        && &self.area_action == &_other.area_action
        && &self.area_label == &_other.area_label
        && &self.background_behavior_area == &_other.background_behavior_area

    }
}

impl LmcpSubscription for AreaOfInterest {
    fn subscription() -> &'static str { "afrl.impact.AreaOfInterest" }
}

impl Struct for AreaOfInterest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 21,
        }
    }
}

impl Lmcp for AreaOfInterest {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.area_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.area.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.area_action.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.area_label.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.background_behavior_area.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(AreaOfInterest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == AreaOfInterest::struct_info() {
            let mut out: AreaOfInterest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.area_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>, usize) = Lmcp::deser(r)?;
                out.area = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::impact::area_action_options::AreaActionOptions, usize) = Lmcp::deser(r)?;
                out.area_action = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.area_label = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.background_behavior_area = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.area_id.size();
        size += self.area.size();
        size += self.area_action.size();
        size += self.area_label.size();
        size += self.background_behavior_area.size();

        size
    }
}

pub trait AreaOfInterestT: Debug + Send  {
    fn as_afrl_impact_area_of_interest(&self) -> Option<&AreaOfInterest> { None }
    fn as_mut_afrl_impact_area_of_interest(&mut self) -> Option<&mut AreaOfInterest> { None }
    fn area_id(&self) -> i64;
    fn area_id_mut(&mut self) -> &mut i64;
    fn area(&self) -> &Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>;
    fn area_mut(&mut self) -> &mut Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT>;
    fn area_action(&self) -> ::afrl::impact::area_action_options::AreaActionOptions;
    fn area_action_mut(&mut self) -> &mut ::afrl::impact::area_action_options::AreaActionOptions;
    fn area_label(&self) -> &Vec<u8>;
    fn area_label_mut(&mut self) -> &mut Vec<u8>;
    fn background_behavior_area(&self) -> bool;
    fn background_behavior_area_mut(&mut self) -> &mut bool;

}

impl Clone for Box<AreaOfInterestT> {
    fn clone(&self) -> Box<AreaOfInterestT> {
        if let Some(x) = AreaOfInterestT::as_afrl_impact_area_of_interest(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<AreaOfInterestT> {
    fn default() -> Box<AreaOfInterestT> { Box::new(AreaOfInterest::default()) }
}

impl PartialEq for Box<AreaOfInterestT> {
    fn eq(&self, other: &Box<AreaOfInterestT>) -> bool {
        if let (Some(x), Some(y)) =
            (AreaOfInterestT::as_afrl_impact_area_of_interest(self.as_ref()),
             AreaOfInterestT::as_afrl_impact_area_of_interest(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<AreaOfInterestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = AreaOfInterestT::as_afrl_impact_area_of_interest(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<AreaOfInterestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == AreaOfInterest::struct_info() {
            let (x, readb) = AreaOfInterest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = AreaOfInterestT::as_afrl_impact_area_of_interest(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl AreaOfInterestT for AreaOfInterest {
    fn as_afrl_impact_area_of_interest(&self) -> Option<&AreaOfInterest> { Some(self) }
    fn as_mut_afrl_impact_area_of_interest(&mut self) -> Option<&mut AreaOfInterest> { Some(self) }
    fn area_id(&self) -> i64 { self.area_id }
    fn area_id_mut(&mut self) -> &mut i64 { &mut self.area_id }
    fn area(&self) -> &Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT> { &self.area }
    fn area_mut(&mut self) -> &mut Box<::afrl::cmasi::abstract_geometry::AbstractGeometryT> { &mut self.area }
    fn area_action(&self) -> ::afrl::impact::area_action_options::AreaActionOptions { self.area_action }
    fn area_action_mut(&mut self) -> &mut ::afrl::impact::area_action_options::AreaActionOptions { &mut self.area_action }
    fn area_label(&self) -> &Vec<u8> { &self.area_label }
    fn area_label_mut(&mut self) -> &mut Vec<u8> { &mut self.area_label }
    fn background_behavior_area(&self) -> bool { self.background_behavior_area }
    fn background_behavior_area_mut(&mut self) -> &mut bool { &mut self.background_behavior_area }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AreaOfInterest {
        fn arbitrary<G: Gen>(_g: &mut G) -> AreaOfInterest {
            AreaOfInterest {
                area_id: Arbitrary::arbitrary(_g),
                area: Box::new(::afrl::cmasi::abstract_geometry::AbstractGeometry::arbitrary(_g)),
                area_action: Arbitrary::arbitrary(_g),
                area_label: Arbitrary::arbitrary(_g),
                background_behavior_area: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: AreaOfInterest) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AreaOfInterest) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AreaOfInterest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
