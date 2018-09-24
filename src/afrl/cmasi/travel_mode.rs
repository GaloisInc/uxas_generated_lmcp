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
pub enum TravelMode {
    SinglePass = 0,
    ReverseCourse = 1,
    Loop = 2,
}

impl TravelMode {
    fn from_i32(x: i32) -> Option<TravelMode> {
        match x {
            0 => Some(TravelMode::SinglePass),
            1 => Some(TravelMode::ReverseCourse),
            2 => Some(TravelMode::Loop),

            _ => None,
        }
    }
}

impl Lmcp for TravelMode {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(TravelMode, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = TravelMode::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for TravelMode {
    fn default() -> TravelMode {
        TravelMode::SinglePass
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TravelMode {
        fn arbitrary<G: Gen>(g: &mut G) -> TravelMode {
            let choices = &[TravelMode::SinglePass,TravelMode::ReverseCourse,TravelMode::Loop,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: TravelMode) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TravelMode) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TravelMode::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
