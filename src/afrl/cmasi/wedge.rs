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

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Wedge {
    pub azimuth_centerline: f32,
    pub vertical_centerline: f32,
    pub azimuth_extent: f32,
    pub vertical_extent: f32,
}

impl PartialEq for Wedge {
    fn eq(&self, _other: &Wedge) -> bool {
        true
        && &self.azimuth_centerline == &_other.azimuth_centerline
        && &self.vertical_centerline == &_other.vertical_centerline
        && &self.azimuth_extent == &_other.azimuth_extent
        && &self.vertical_extent == &_other.vertical_extent

    }
}

impl LmcpSubscription for Wedge {
    fn subscription() -> &'static str { "afrl.cmasi.Wedge" }
}

impl Struct for Wedge {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 16,
        }
    }
}

impl Lmcp for Wedge {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.azimuth_centerline.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vertical_centerline.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.azimuth_extent.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vertical_extent.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(Wedge, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == Wedge::struct_info() {
            let mut out: Wedge = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.azimuth_centerline = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.vertical_centerline = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.azimuth_extent = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.vertical_extent = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.azimuth_centerline.size();
        size += self.vertical_centerline.size();
        size += self.azimuth_extent.size();
        size += self.vertical_extent.size();

        size
    }
}

pub trait WedgeT: Debug + Send  {
    fn as_afrl_cmasi_wedge(&self) -> Option<&Wedge> { None }
    fn as_mut_afrl_cmasi_wedge(&mut self) -> Option<&mut Wedge> { None }
    fn azimuth_centerline(&self) -> f32;
    fn azimuth_centerline_mut(&mut self) -> &mut f32;
    fn vertical_centerline(&self) -> f32;
    fn vertical_centerline_mut(&mut self) -> &mut f32;
    fn azimuth_extent(&self) -> f32;
    fn azimuth_extent_mut(&mut self) -> &mut f32;
    fn vertical_extent(&self) -> f32;
    fn vertical_extent_mut(&mut self) -> &mut f32;

}

impl Clone for Box<WedgeT> {
    fn clone(&self) -> Box<WedgeT> {
        if let Some(x) = WedgeT::as_afrl_cmasi_wedge(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<WedgeT> {
    fn default() -> Box<WedgeT> { Box::new(Wedge::default()) }
}

impl PartialEq for Box<WedgeT> {
    fn eq(&self, other: &Box<WedgeT>) -> bool {
        if let (Some(x), Some(y)) =
            (WedgeT::as_afrl_cmasi_wedge(self.as_ref()),
             WedgeT::as_afrl_cmasi_wedge(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<WedgeT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = WedgeT::as_afrl_cmasi_wedge(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<WedgeT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == Wedge::struct_info() {
            let (x, readb) = Wedge::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = WedgeT::as_afrl_cmasi_wedge(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl WedgeT for Wedge {
    fn as_afrl_cmasi_wedge(&self) -> Option<&Wedge> { Some(self) }
    fn as_mut_afrl_cmasi_wedge(&mut self) -> Option<&mut Wedge> { Some(self) }
    fn azimuth_centerline(&self) -> f32 { self.azimuth_centerline }
    fn azimuth_centerline_mut(&mut self) -> &mut f32 { &mut self.azimuth_centerline }
    fn vertical_centerline(&self) -> f32 { self.vertical_centerline }
    fn vertical_centerline_mut(&mut self) -> &mut f32 { &mut self.vertical_centerline }
    fn azimuth_extent(&self) -> f32 { self.azimuth_extent }
    fn azimuth_extent_mut(&mut self) -> &mut f32 { &mut self.azimuth_extent }
    fn vertical_extent(&self) -> f32 { self.vertical_extent }
    fn vertical_extent_mut(&mut self) -> &mut f32 { &mut self.vertical_extent }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for Wedge {
        fn arbitrary<G: Gen>(_g: &mut G) -> Wedge {
            Wedge {
                azimuth_centerline: Arbitrary::arbitrary(_g),
                vertical_centerline: Arbitrary::arbitrary(_g),
                azimuth_extent: Arbitrary::arbitrary(_g),
                vertical_extent: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: Wedge) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: Wedge) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = Wedge::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
