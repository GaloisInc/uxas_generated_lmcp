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
pub enum AreaActionOptions {
    Created = 0,
    Destroyed = 1,
    Modified = 2,
}

impl AreaActionOptions {
    fn from_i32(x: i32) -> Option<AreaActionOptions> {
        match x {
            0 => Some(AreaActionOptions::Created),
            1 => Some(AreaActionOptions::Destroyed),
            2 => Some(AreaActionOptions::Modified),

            _ => None,
        }
    }
}

impl Lmcp for AreaActionOptions {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(AreaActionOptions, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = AreaActionOptions::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for AreaActionOptions {
    fn default() -> AreaActionOptions {
        AreaActionOptions::Created
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AreaActionOptions {
        fn arbitrary<G: Gen>(g: &mut G) -> AreaActionOptions {
            let choices = &[AreaActionOptions::Created,AreaActionOptions::Destroyed,AreaActionOptions::Modified,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: AreaActionOptions) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AreaActionOptions) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AreaActionOptions::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
