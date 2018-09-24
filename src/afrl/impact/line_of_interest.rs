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
pub struct LineOfInterest {
    pub line_id: i64,
    pub line: Vec<Box<::afrl::cmasi::location3d::Location3DT>>,
    pub line_action: ::afrl::impact::area_action_options::AreaActionOptions,
    pub line_label: Vec<u8>,
    pub background_behavior_line: bool,
}

impl PartialEq for LineOfInterest {
    fn eq(&self, _other: &LineOfInterest) -> bool {
        true
        && &self.line_id == &_other.line_id
        && &self.line == &_other.line
        && &self.line_action == &_other.line_action
        && &self.line_label == &_other.line_label
        && &self.background_behavior_line == &_other.background_behavior_line

    }
}

impl LmcpSubscription for LineOfInterest {
    fn subscription() -> &'static str { "afrl.impact.LineOfInterest" }
}

impl Struct for LineOfInterest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 20,
        }
    }
}

impl Lmcp for LineOfInterest {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.line_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.line.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.line_action.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.line_label.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.background_behavior_line.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(LineOfInterest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == LineOfInterest::struct_info() {
            let mut out: LineOfInterest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.line_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.line = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::impact::area_action_options::AreaActionOptions, usize) = Lmcp::deser(r)?;
                out.line_action = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.line_label = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.background_behavior_line = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.line_id.size();
        size += self.line.size();
        size += self.line_action.size();
        size += self.line_label.size();
        size += self.background_behavior_line.size();

        size
    }
}

pub trait LineOfInterestT: Debug + Send  {
    fn as_afrl_impact_line_of_interest(&self) -> Option<&LineOfInterest> { None }
    fn as_mut_afrl_impact_line_of_interest(&mut self) -> Option<&mut LineOfInterest> { None }
    fn line_id(&self) -> i64;
    fn line_id_mut(&mut self) -> &mut i64;
    fn line(&self) -> &Vec<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn line_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn line_action(&self) -> ::afrl::impact::area_action_options::AreaActionOptions;
    fn line_action_mut(&mut self) -> &mut ::afrl::impact::area_action_options::AreaActionOptions;
    fn line_label(&self) -> &Vec<u8>;
    fn line_label_mut(&mut self) -> &mut Vec<u8>;
    fn background_behavior_line(&self) -> bool;
    fn background_behavior_line_mut(&mut self) -> &mut bool;

}

impl Clone for Box<LineOfInterestT> {
    fn clone(&self) -> Box<LineOfInterestT> {
        if let Some(x) = LineOfInterestT::as_afrl_impact_line_of_interest(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<LineOfInterestT> {
    fn default() -> Box<LineOfInterestT> { Box::new(LineOfInterest::default()) }
}

impl PartialEq for Box<LineOfInterestT> {
    fn eq(&self, other: &Box<LineOfInterestT>) -> bool {
        if let (Some(x), Some(y)) =
            (LineOfInterestT::as_afrl_impact_line_of_interest(self.as_ref()),
             LineOfInterestT::as_afrl_impact_line_of_interest(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<LineOfInterestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = LineOfInterestT::as_afrl_impact_line_of_interest(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<LineOfInterestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == LineOfInterest::struct_info() {
            let (x, readb) = LineOfInterest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = LineOfInterestT::as_afrl_impact_line_of_interest(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl LineOfInterestT for LineOfInterest {
    fn as_afrl_impact_line_of_interest(&self) -> Option<&LineOfInterest> { Some(self) }
    fn as_mut_afrl_impact_line_of_interest(&mut self) -> Option<&mut LineOfInterest> { Some(self) }
    fn line_id(&self) -> i64 { self.line_id }
    fn line_id_mut(&mut self) -> &mut i64 { &mut self.line_id }
    fn line(&self) -> &Vec<Box<::afrl::cmasi::location3d::Location3DT>> { &self.line }
    fn line_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.line }
    fn line_action(&self) -> ::afrl::impact::area_action_options::AreaActionOptions { self.line_action }
    fn line_action_mut(&mut self) -> &mut ::afrl::impact::area_action_options::AreaActionOptions { &mut self.line_action }
    fn line_label(&self) -> &Vec<u8> { &self.line_label }
    fn line_label_mut(&mut self) -> &mut Vec<u8> { &mut self.line_label }
    fn background_behavior_line(&self) -> bool { self.background_behavior_line }
    fn background_behavior_line_mut(&mut self) -> &mut bool { &mut self.background_behavior_line }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for LineOfInterest {
        fn arbitrary<G: Gen>(_g: &mut G) -> LineOfInterest {
            LineOfInterest {
                line_id: Arbitrary::arbitrary(_g),
                line: Vec::<::afrl::cmasi::location3d::Location3D>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::location3d::Location3DT>).collect(),
                line_action: Arbitrary::arbitrary(_g),
                line_label: Arbitrary::arbitrary(_g),
                background_behavior_line: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: LineOfInterest) -> Result<TestResult, Error> {
            use std::u16;
            if x.line.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: LineOfInterest) -> Result<TestResult, Error> {
            use std::u16;
            if x.line.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = LineOfInterest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
