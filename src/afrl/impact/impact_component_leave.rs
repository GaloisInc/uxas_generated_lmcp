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
pub struct ImpactComponentLeave {
    pub component_label: Vec<u8>,
}

impl PartialEq for ImpactComponentLeave {
    fn eq(&self, _other: &ImpactComponentLeave) -> bool {
        true
        && &self.component_label == &_other.component_label

    }
}

impl LmcpSubscription for ImpactComponentLeave {
    fn subscription() -> &'static str { "afrl.impact.ImpactComponentLeave" }
}

impl Struct for ImpactComponentLeave {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 13,
            struct_ty: 18,
        }
    }
}

impl Lmcp for ImpactComponentLeave {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.component_label.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(ImpactComponentLeave, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == ImpactComponentLeave::struct_info() {
            let mut out: ImpactComponentLeave = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.component_label = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.component_label.size();

        size
    }
}

pub trait ImpactComponentLeaveT: Debug + Send  {
    fn as_afrl_impact_impact_component_leave(&self) -> Option<&ImpactComponentLeave> { None }
    fn as_mut_afrl_impact_impact_component_leave(&mut self) -> Option<&mut ImpactComponentLeave> { None }
    fn component_label(&self) -> &Vec<u8>;
    fn component_label_mut(&mut self) -> &mut Vec<u8>;

}

impl Clone for Box<ImpactComponentLeaveT> {
    fn clone(&self) -> Box<ImpactComponentLeaveT> {
        if let Some(x) = ImpactComponentLeaveT::as_afrl_impact_impact_component_leave(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<ImpactComponentLeaveT> {
    fn default() -> Box<ImpactComponentLeaveT> { Box::new(ImpactComponentLeave::default()) }
}

impl PartialEq for Box<ImpactComponentLeaveT> {
    fn eq(&self, other: &Box<ImpactComponentLeaveT>) -> bool {
        if let (Some(x), Some(y)) =
            (ImpactComponentLeaveT::as_afrl_impact_impact_component_leave(self.as_ref()),
             ImpactComponentLeaveT::as_afrl_impact_impact_component_leave(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<ImpactComponentLeaveT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = ImpactComponentLeaveT::as_afrl_impact_impact_component_leave(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<ImpactComponentLeaveT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == ImpactComponentLeave::struct_info() {
            let (x, readb) = ImpactComponentLeave::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = ImpactComponentLeaveT::as_afrl_impact_impact_component_leave(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ImpactComponentLeaveT for ImpactComponentLeave {
    fn as_afrl_impact_impact_component_leave(&self) -> Option<&ImpactComponentLeave> { Some(self) }
    fn as_mut_afrl_impact_impact_component_leave(&mut self) -> Option<&mut ImpactComponentLeave> { Some(self) }
    fn component_label(&self) -> &Vec<u8> { &self.component_label }
    fn component_label_mut(&mut self) -> &mut Vec<u8> { &mut self.component_label }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for ImpactComponentLeave {
        fn arbitrary<G: Gen>(_g: &mut G) -> ImpactComponentLeave {
            ImpactComponentLeave {
                component_label: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: ImpactComponentLeave) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: ImpactComponentLeave) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = ImpactComponentLeave::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
