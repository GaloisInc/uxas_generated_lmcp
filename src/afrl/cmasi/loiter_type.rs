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
pub enum LoiterType {
    VehicleDefault = 0,
    Circular = 1,
    Racetrack = 2,
    FigureEight = 3,
    Hover = 4,
}

impl LoiterType {
    fn from_i32(x: i32) -> Option<LoiterType> {
        match x {
            0 => Some(LoiterType::VehicleDefault),
            1 => Some(LoiterType::Circular),
            2 => Some(LoiterType::Racetrack),
            3 => Some(LoiterType::FigureEight),
            4 => Some(LoiterType::Hover),

            _ => None,
        }
    }
}

impl Lmcp for LoiterType {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(LoiterType, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = LoiterType::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for LoiterType {
    fn default() -> LoiterType {
        LoiterType::VehicleDefault
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for LoiterType {
        fn arbitrary<G: Gen>(g: &mut G) -> LoiterType {
            let choices = &[LoiterType::VehicleDefault,LoiterType::Circular,LoiterType::Racetrack,LoiterType::FigureEight,LoiterType::Hover,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: LoiterType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: LoiterType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = LoiterType::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
