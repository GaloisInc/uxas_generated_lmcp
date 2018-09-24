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
pub enum WaypointTransferMode {
    RequestWaypoints = 0,
    AddWaypoints = 1,
    ClearWaypoints = 2,
    ReportWaypoints = 3,
}

impl WaypointTransferMode {
    fn from_i32(x: i32) -> Option<WaypointTransferMode> {
        match x {
            0 => Some(WaypointTransferMode::RequestWaypoints),
            1 => Some(WaypointTransferMode::AddWaypoints),
            2 => Some(WaypointTransferMode::ClearWaypoints),
            3 => Some(WaypointTransferMode::ReportWaypoints),

            _ => None,
        }
    }
}

impl Lmcp for WaypointTransferMode {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(WaypointTransferMode, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = WaypointTransferMode::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for WaypointTransferMode {
    fn default() -> WaypointTransferMode {
        WaypointTransferMode::RequestWaypoints
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for WaypointTransferMode {
        fn arbitrary<G: Gen>(g: &mut G) -> WaypointTransferMode {
            let choices = &[WaypointTransferMode::RequestWaypoints,WaypointTransferMode::AddWaypoints,WaypointTransferMode::ClearWaypoints,WaypointTransferMode::ReportWaypoints,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: WaypointTransferMode) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: WaypointTransferMode) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = WaypointTransferMode::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
