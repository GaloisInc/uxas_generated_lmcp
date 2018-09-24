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
pub enum LoiterDirection {
    VehicleDefault = 0,
    CounterClockwise = 1,
    Clockwise = 2,
}

impl LoiterDirection {
    fn from_i32(x: i32) -> Option<LoiterDirection> {
        match x {
            0 => Some(LoiterDirection::VehicleDefault),
            1 => Some(LoiterDirection::CounterClockwise),
            2 => Some(LoiterDirection::Clockwise),

            _ => None,
        }
    }
}

impl Lmcp for LoiterDirection {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(LoiterDirection, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = LoiterDirection::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for LoiterDirection {
    fn default() -> LoiterDirection {
        LoiterDirection::VehicleDefault
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for LoiterDirection {
        fn arbitrary<G: Gen>(g: &mut G) -> LoiterDirection {
            let choices = &[LoiterDirection::VehicleDefault,LoiterDirection::CounterClockwise,LoiterDirection::Clockwise,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: LoiterDirection) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: LoiterDirection) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = LoiterDirection::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
