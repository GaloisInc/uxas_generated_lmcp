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
pub enum FOVOperationMode {
    Continuous = 0,
    Discrete = 1,
}

impl FOVOperationMode {
    fn from_i32(x: i32) -> Option<FOVOperationMode> {
        match x {
            0 => Some(FOVOperationMode::Continuous),
            1 => Some(FOVOperationMode::Discrete),

            _ => None,
        }
    }
}

impl Lmcp for FOVOperationMode {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(FOVOperationMode, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = FOVOperationMode::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for FOVOperationMode {
    fn default() -> FOVOperationMode {
        FOVOperationMode::Continuous
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for FOVOperationMode {
        fn arbitrary<G: Gen>(g: &mut G) -> FOVOperationMode {
            let choices = &[FOVOperationMode::Continuous,FOVOperationMode::Discrete,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: FOVOperationMode) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: FOVOperationMode) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = FOVOperationMode::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
