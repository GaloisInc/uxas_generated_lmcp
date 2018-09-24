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
pub struct OperatingRegion {
    pub id: i64,
    pub keep_in_areas: Vec<i64>,
    pub keep_out_areas: Vec<i64>,
}

impl PartialEq for OperatingRegion {
    fn eq(&self, _other: &OperatingRegion) -> bool {
        true
        && &self.id == &_other.id
        && &self.keep_in_areas == &_other.keep_in_areas
        && &self.keep_out_areas == &_other.keep_out_areas

    }
}

impl LmcpSubscription for OperatingRegion {
    fn subscription() -> &'static str { "afrl.cmasi.OperatingRegion" }
}

impl Struct for OperatingRegion {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 39,
        }
    }
}

impl Lmcp for OperatingRegion {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.keep_in_areas.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.keep_out_areas.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(OperatingRegion, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == OperatingRegion::struct_info() {
            let mut out: OperatingRegion = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.keep_in_areas = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.keep_out_areas = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.id.size();
        size += self.keep_in_areas.size();
        size += self.keep_out_areas.size();

        size
    }
}

pub trait OperatingRegionT: Debug + Send  {
    fn as_afrl_cmasi_operating_region(&self) -> Option<&OperatingRegion> { None }
    fn as_mut_afrl_cmasi_operating_region(&mut self) -> Option<&mut OperatingRegion> { None }
    fn id(&self) -> i64;
    fn id_mut(&mut self) -> &mut i64;
    fn keep_in_areas(&self) -> &Vec<i64>;
    fn keep_in_areas_mut(&mut self) -> &mut Vec<i64>;
    fn keep_out_areas(&self) -> &Vec<i64>;
    fn keep_out_areas_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<OperatingRegionT> {
    fn clone(&self) -> Box<OperatingRegionT> {
        if let Some(x) = OperatingRegionT::as_afrl_cmasi_operating_region(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<OperatingRegionT> {
    fn default() -> Box<OperatingRegionT> { Box::new(OperatingRegion::default()) }
}

impl PartialEq for Box<OperatingRegionT> {
    fn eq(&self, other: &Box<OperatingRegionT>) -> bool {
        if let (Some(x), Some(y)) =
            (OperatingRegionT::as_afrl_cmasi_operating_region(self.as_ref()),
             OperatingRegionT::as_afrl_cmasi_operating_region(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<OperatingRegionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = OperatingRegionT::as_afrl_cmasi_operating_region(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<OperatingRegionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == OperatingRegion::struct_info() {
            let (x, readb) = OperatingRegion::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = OperatingRegionT::as_afrl_cmasi_operating_region(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl OperatingRegionT for OperatingRegion {
    fn as_afrl_cmasi_operating_region(&self) -> Option<&OperatingRegion> { Some(self) }
    fn as_mut_afrl_cmasi_operating_region(&mut self) -> Option<&mut OperatingRegion> { Some(self) }
    fn id(&self) -> i64 { self.id }
    fn id_mut(&mut self) -> &mut i64 { &mut self.id }
    fn keep_in_areas(&self) -> &Vec<i64> { &self.keep_in_areas }
    fn keep_in_areas_mut(&mut self) -> &mut Vec<i64> { &mut self.keep_in_areas }
    fn keep_out_areas(&self) -> &Vec<i64> { &self.keep_out_areas }
    fn keep_out_areas_mut(&mut self) -> &mut Vec<i64> { &mut self.keep_out_areas }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for OperatingRegion {
        fn arbitrary<G: Gen>(_g: &mut G) -> OperatingRegion {
            OperatingRegion {
                id: Arbitrary::arbitrary(_g),
                keep_in_areas: Arbitrary::arbitrary(_g),
                keep_out_areas: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: OperatingRegion) -> Result<TestResult, Error> {
            use std::u16;
            if x.keep_in_areas.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.keep_out_areas.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: OperatingRegion) -> Result<TestResult, Error> {
            use std::u16;
            if x.keep_in_areas.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.keep_out_areas.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = OperatingRegion::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
