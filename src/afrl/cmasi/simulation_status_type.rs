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
pub enum SimulationStatusType {
    Stopped = 0,
    Running = 1,
    Paused = 2,
    Reset = 3,
}

impl SimulationStatusType {
    fn from_i32(x: i32) -> Option<SimulationStatusType> {
        match x {
            0 => Some(SimulationStatusType::Stopped),
            1 => Some(SimulationStatusType::Running),
            2 => Some(SimulationStatusType::Paused),
            3 => Some(SimulationStatusType::Reset),

            _ => None,
        }
    }
}

impl Lmcp for SimulationStatusType {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(SimulationStatusType, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = SimulationStatusType::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for SimulationStatusType {
    fn default() -> SimulationStatusType {
        SimulationStatusType::Stopped
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SimulationStatusType {
        fn arbitrary<G: Gen>(g: &mut G) -> SimulationStatusType {
            let choices = &[SimulationStatusType::Stopped,SimulationStatusType::Running,SimulationStatusType::Paused,SimulationStatusType::Reset,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: SimulationStatusType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SimulationStatusType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SimulationStatusType::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
