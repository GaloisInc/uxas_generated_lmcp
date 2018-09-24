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
pub enum CommandStatusType {
    Pending = 0,
    Approved = 1,
    InProcess = 2,
    Executed = 3,
    Cancelled = 4,
}

impl CommandStatusType {
    fn from_i32(x: i32) -> Option<CommandStatusType> {
        match x {
            0 => Some(CommandStatusType::Pending),
            1 => Some(CommandStatusType::Approved),
            2 => Some(CommandStatusType::InProcess),
            3 => Some(CommandStatusType::Executed),
            4 => Some(CommandStatusType::Cancelled),

            _ => None,
        }
    }
}

impl Lmcp for CommandStatusType {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        (*self as i32).ser(buf)
    }

    fn deser(buf: &[u8]) -> Result<(CommandStatusType, usize), Error> {
        let (i, readb) = i32::deser(buf)?;
        let out = CommandStatusType::from_i32(i).ok_or(error!(ErrorType::InvalidEnumValue))?;
        Ok((out, readb))
    }

    fn size(&self) -> usize { 0i32.size() }
}

impl Default for CommandStatusType {
    fn default() -> CommandStatusType {
        CommandStatusType::Pending
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for CommandStatusType {
        fn arbitrary<G: Gen>(g: &mut G) -> CommandStatusType {
            let choices = &[CommandStatusType::Pending,CommandStatusType::Approved,CommandStatusType::InProcess,CommandStatusType::Executed,CommandStatusType::Cancelled,];
            *g.choose(choices).unwrap()
        }
    }

    quickcheck! {
        fn serializes(x: CommandStatusType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: CommandStatusType) -> Result<TestResult, Error> {
            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = CommandStatusType::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
