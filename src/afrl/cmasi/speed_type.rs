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
pub enum SpeedType {
    Airspeed = 0,
    Groundspeed = 1,
}

impl SpeedType {
    fn from_i32(x: i32) -> Option<SpeedType> {
        match x {
            0 => Some(SpeedType::Airspeed),
            1 => Some(SpeedType::Groundspeed),

            _ => None,
        }
    }
}

impl Lmcp for SpeedType {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(SpeedType, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = SpeedType::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for SpeedType {
    fn default() -> SpeedType {
        SpeedType::Airspeed
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SpeedType {
        fn arbitrary<G: Gen>(g: &mut G) -> SpeedType {
            let choices = &[SpeedType::Airspeed,SpeedType::Groundspeed,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: SpeedType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SpeedType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SpeedType::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
