// ===============================================================================
// Authors: AFRL/RQQA
// Organization: Air Force Research Laboratory, Aerospace Systems Directorate, Power and Control Division
// 
// Copyright (c) 2017 Government of the United State of America, as represented by
// the Secretary of the Air Force.  No copyright is claimed in the United States under
// Title 17, U.S. Code.  All Other Rights Reserved.
// ===============================================================================

// This file was auto-created by LmcpGen. Modifications will be overwritten.

use avtas::lmcp::{Error, ErrorType, Lmcp, SrcLoc};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum AreaSearchPattern {
    Spiral = 0,
    Sector = 1,
    Sweep = 2,
}

impl AreaSearchPattern {
    fn from_i32(x: i32) -> Option<AreaSearchPattern> {
        match x {
            0 => Some(AreaSearchPattern::Spiral),
            1 => Some(AreaSearchPattern::Sector),
            2 => Some(AreaSearchPattern::Sweep),

            _ => None,
        }
    }
}

impl Lmcp for AreaSearchPattern {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(AreaSearchPattern, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = AreaSearchPattern::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for AreaSearchPattern {
    fn default() -> AreaSearchPattern {
        AreaSearchPattern::Spiral
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AreaSearchPattern {
        fn arbitrary<G: Gen>(g: &mut G) -> AreaSearchPattern {
            let choices = &[AreaSearchPattern::Spiral,AreaSearchPattern::Sector,AreaSearchPattern::Sweep,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: AreaSearchPattern) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AreaSearchPattern) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AreaSearchPattern::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
