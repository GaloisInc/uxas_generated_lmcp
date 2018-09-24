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
pub enum GimbalPointingMode {
    Unknown = 0,
    AirVehicleRelativeAngle = 1,
    AirVehicleRelativeSlewRate = 2,
    LatLonSlaved = 3,
    InertialRelativeSlewRate = 4,
    Scan = 5,
    Stowed = 6,
}

impl GimbalPointingMode {
    fn from_i32(x: i32) -> Option<GimbalPointingMode> {
        match x {
            0 => Some(GimbalPointingMode::Unknown),
            1 => Some(GimbalPointingMode::AirVehicleRelativeAngle),
            2 => Some(GimbalPointingMode::AirVehicleRelativeSlewRate),
            3 => Some(GimbalPointingMode::LatLonSlaved),
            4 => Some(GimbalPointingMode::InertialRelativeSlewRate),
            5 => Some(GimbalPointingMode::Scan),
            6 => Some(GimbalPointingMode::Stowed),

            _ => None,
        }
    }
}

impl Lmcp for GimbalPointingMode {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(GimbalPointingMode, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = GimbalPointingMode::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for GimbalPointingMode {
    fn default() -> GimbalPointingMode {
        GimbalPointingMode::Unknown
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for GimbalPointingMode {
        fn arbitrary<G: Gen>(g: &mut G) -> GimbalPointingMode {
            let choices = &[GimbalPointingMode::Unknown,GimbalPointingMode::AirVehicleRelativeAngle,GimbalPointingMode::AirVehicleRelativeSlewRate,GimbalPointingMode::LatLonSlaved,GimbalPointingMode::InertialRelativeSlewRate,GimbalPointingMode::Scan,GimbalPointingMode::Stowed,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: GimbalPointingMode) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: GimbalPointingMode) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = GimbalPointingMode::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
