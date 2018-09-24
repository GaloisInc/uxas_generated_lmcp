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
pub enum ImpactPayloadType {
    Unknown = 0,
    EO = 1,
    FLIR = 2,
    MWIR = 3,
    LFIR = 4,
    Track = 5,
    Tag = 6,
    Megaphone = 7,
    Siren = 8,
    SearchLight = 9,
    FiftyCal = 10,
    M240B = 11,
    Flashbang = 12,
    TearGas = 13,
    Taser = 14,
    HeatBeam = 15,
    SEGM = 16,
    CommRelay = 17,
    GMTI = 18,
    LaserDesignator = 19,
    LWIR = 20,
}

impl ImpactPayloadType {
    fn from_i32(x: i32) -> Option<ImpactPayloadType> {
        match x {
            0 => Some(ImpactPayloadType::Unknown),
            1 => Some(ImpactPayloadType::EO),
            2 => Some(ImpactPayloadType::FLIR),
            3 => Some(ImpactPayloadType::MWIR),
            4 => Some(ImpactPayloadType::LFIR),
            5 => Some(ImpactPayloadType::Track),
            6 => Some(ImpactPayloadType::Tag),
            7 => Some(ImpactPayloadType::Megaphone),
            8 => Some(ImpactPayloadType::Siren),
            9 => Some(ImpactPayloadType::SearchLight),
            10 => Some(ImpactPayloadType::FiftyCal),
            11 => Some(ImpactPayloadType::M240B),
            12 => Some(ImpactPayloadType::Flashbang),
            13 => Some(ImpactPayloadType::TearGas),
            14 => Some(ImpactPayloadType::Taser),
            15 => Some(ImpactPayloadType::HeatBeam),
            16 => Some(ImpactPayloadType::SEGM),
            17 => Some(ImpactPayloadType::CommRelay),
            18 => Some(ImpactPayloadType::GMTI),
            19 => Some(ImpactPayloadType::LaserDesignator),
            20 => Some(ImpactPayloadType::LWIR),

            _ => None,
        }
    }
}

impl Lmcp for ImpactPayloadType {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(ImpactPayloadType, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = ImpactPayloadType::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for ImpactPayloadType {
    fn default() -> ImpactPayloadType {
        ImpactPayloadType::Unknown
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for ImpactPayloadType {
        fn arbitrary<G: Gen>(g: &mut G) -> ImpactPayloadType {
            let choices = &[ImpactPayloadType::Unknown,ImpactPayloadType::EO,ImpactPayloadType::FLIR,ImpactPayloadType::MWIR,ImpactPayloadType::LFIR,ImpactPayloadType::Track,ImpactPayloadType::Tag,ImpactPayloadType::Megaphone,ImpactPayloadType::Siren,ImpactPayloadType::SearchLight,ImpactPayloadType::FiftyCal,ImpactPayloadType::M240B,ImpactPayloadType::Flashbang,ImpactPayloadType::TearGas,ImpactPayloadType::Taser,ImpactPayloadType::HeatBeam,ImpactPayloadType::SEGM,ImpactPayloadType::CommRelay,ImpactPayloadType::GMTI,ImpactPayloadType::LaserDesignator,ImpactPayloadType::LWIR,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: ImpactPayloadType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: ImpactPayloadType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = ImpactPayloadType::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
