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
pub struct RemoveZones {
    pub zone_list: Vec<i64>,
}

impl PartialEq for RemoveZones {
    fn eq(&self, _other: &RemoveZones) -> bool {
        true
        && &self.zone_list == &_other.zone_list

    }
}

impl LmcpSubscription for RemoveZones {
    fn subscription() -> &'static str { "afrl.cmasi.RemoveZones" }
}

impl Struct for RemoveZones {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 52,
        }
    }
}

impl Lmcp for RemoveZones {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.zone_list.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RemoveZones, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RemoveZones::struct_info() {
            let mut out: RemoveZones = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.zone_list = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.zone_list.size();

        size
    }
}

pub trait RemoveZonesT: Debug + Send  {
    fn as_afrl_cmasi_remove_zones(&self) -> Option<&RemoveZones> { None }
    fn as_mut_afrl_cmasi_remove_zones(&mut self) -> Option<&mut RemoveZones> { None }
    fn zone_list(&self) -> &Vec<i64>;
    fn zone_list_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<RemoveZonesT> {
    fn clone(&self) -> Box<RemoveZonesT> {
        if let Some(x) = RemoveZonesT::as_afrl_cmasi_remove_zones(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RemoveZonesT> {
    fn default() -> Box<RemoveZonesT> { Box::new(RemoveZones::default()) }
}

impl PartialEq for Box<RemoveZonesT> {
    fn eq(&self, other: &Box<RemoveZonesT>) -> bool {
        if let (Some(x), Some(y)) =
            (RemoveZonesT::as_afrl_cmasi_remove_zones(self.as_ref()),
             RemoveZonesT::as_afrl_cmasi_remove_zones(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RemoveZonesT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RemoveZonesT::as_afrl_cmasi_remove_zones(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RemoveZonesT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RemoveZones::struct_info() {
            let (x, readb) = RemoveZones::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RemoveZonesT::as_afrl_cmasi_remove_zones(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RemoveZonesT for RemoveZones {
    fn as_afrl_cmasi_remove_zones(&self) -> Option<&RemoveZones> { Some(self) }
    fn as_mut_afrl_cmasi_remove_zones(&mut self) -> Option<&mut RemoveZones> { Some(self) }
    fn zone_list(&self) -> &Vec<i64> { &self.zone_list }
    fn zone_list_mut(&mut self) -> &mut Vec<i64> { &mut self.zone_list }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RemoveZones {
        fn arbitrary<G: Gen>(_g: &mut G) -> RemoveZones {
            RemoveZones {
                zone_list: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RemoveZones) -> Result<TestResult, Error> {
            use std::u16;
            if x.zone_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RemoveZones) -> Result<TestResult, Error> {
            use std::u16;
            if x.zone_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RemoveZones::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
