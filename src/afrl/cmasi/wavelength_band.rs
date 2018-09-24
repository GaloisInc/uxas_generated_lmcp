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
pub enum WavelengthBand {
    AllAny = 0,
    EO = 1,
    LWIR = 2,
    SWIR = 3,
    MWIR = 4,
    Other = 5,
}

impl WavelengthBand {
    fn from_i32(x: i32) -> Option<WavelengthBand> {
        match x {
            0 => Some(WavelengthBand::AllAny),
            1 => Some(WavelengthBand::EO),
            2 => Some(WavelengthBand::LWIR),
            3 => Some(WavelengthBand::SWIR),
            4 => Some(WavelengthBand::MWIR),
            5 => Some(WavelengthBand::Other),

            _ => None,
        }
    }
}

impl Lmcp for WavelengthBand {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(WavelengthBand, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = WavelengthBand::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for WavelengthBand {
    fn default() -> WavelengthBand {
        WavelengthBand::AllAny
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for WavelengthBand {
        fn arbitrary<G: Gen>(g: &mut G) -> WavelengthBand {
            let choices = &[WavelengthBand::AllAny,WavelengthBand::EO,WavelengthBand::LWIR,WavelengthBand::SWIR,WavelengthBand::MWIR,WavelengthBand::Other,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: WavelengthBand) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: WavelengthBand) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = WavelengthBand::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
