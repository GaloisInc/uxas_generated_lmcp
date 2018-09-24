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
pub enum NavigationMode {
    Waypoint = 0,
    Loiter = 1,
    FlightDirector = 2,
    TargetTrack = 3,
    FollowLeader = 4,
    LostComm = 5,
}

impl NavigationMode {
    fn from_i32(x: i32) -> Option<NavigationMode> {
        match x {
            0 => Some(NavigationMode::Waypoint),
            1 => Some(NavigationMode::Loiter),
            2 => Some(NavigationMode::FlightDirector),
            3 => Some(NavigationMode::TargetTrack),
            4 => Some(NavigationMode::FollowLeader),
            5 => Some(NavigationMode::LostComm),

            _ => None,
        }
    }
}

impl Lmcp for NavigationMode {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(NavigationMode, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = NavigationMode::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for NavigationMode {
    fn default() -> NavigationMode {
        NavigationMode::Waypoint
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for NavigationMode {
        fn arbitrary<G: Gen>(g: &mut G) -> NavigationMode {
            let choices = &[NavigationMode::Waypoint,NavigationMode::Loiter,NavigationMode::FlightDirector,NavigationMode::TargetTrack,NavigationMode::FollowLeader,NavigationMode::LostComm,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: NavigationMode) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: NavigationMode) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = NavigationMode::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
