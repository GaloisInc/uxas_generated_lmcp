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
pub enum ZoneAvoidanceType {
    Physical = 1,
    Regulatory = 2,
    Acoustic = 3,
    Threat = 4,
    Visual = 5,
}

impl ZoneAvoidanceType {
    fn from_i32(x: i32) -> Option<ZoneAvoidanceType> {
        match x {
            1 => Some(ZoneAvoidanceType::Physical),
            2 => Some(ZoneAvoidanceType::Regulatory),
            3 => Some(ZoneAvoidanceType::Acoustic),
            4 => Some(ZoneAvoidanceType::Threat),
            5 => Some(ZoneAvoidanceType::Visual),

            _ => None,
        }
    }
}

impl Lmcp for ZoneAvoidanceType {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(ZoneAvoidanceType, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = ZoneAvoidanceType::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for ZoneAvoidanceType {
    fn default() -> ZoneAvoidanceType {
        ZoneAvoidanceType::Physical
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for ZoneAvoidanceType {
        fn arbitrary<G: Gen>(g: &mut G) -> ZoneAvoidanceType {
            let choices = &[ZoneAvoidanceType::Physical,ZoneAvoidanceType::Regulatory,ZoneAvoidanceType::Acoustic,ZoneAvoidanceType::Threat,ZoneAvoidanceType::Visual,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: ZoneAvoidanceType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: ZoneAvoidanceType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = ZoneAvoidanceType::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
