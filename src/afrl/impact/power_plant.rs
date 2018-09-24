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
pub enum PowerPlant {
    Gasoline = 0,
    JP5 = 1,
    JP8 = 2,
    FuelCell = 3,
    Hybrid = 4,
    Electric = 5,
}

impl PowerPlant {
    fn from_i32(x: i32) -> Option<PowerPlant> {
        match x {
            0 => Some(PowerPlant::Gasoline),
            1 => Some(PowerPlant::JP5),
            2 => Some(PowerPlant::JP8),
            3 => Some(PowerPlant::FuelCell),
            4 => Some(PowerPlant::Hybrid),
            5 => Some(PowerPlant::Electric),

            _ => None,
        }
    }
}

impl Lmcp for PowerPlant {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(PowerPlant, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = PowerPlant::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for PowerPlant {
    fn default() -> PowerPlant {
        PowerPlant::Gasoline
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PowerPlant {
        fn arbitrary<G: Gen>(g: &mut G) -> PowerPlant {
            let choices = &[PowerPlant::Gasoline,PowerPlant::JP5,PowerPlant::JP8,PowerPlant::FuelCell,PowerPlant::Hybrid,PowerPlant::Electric,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: PowerPlant) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PowerPlant) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PowerPlant::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
