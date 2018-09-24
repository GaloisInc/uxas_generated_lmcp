// ===============================================================================
// Authors: AFRL/RQQA
// Organization: Air Force Research Laboratory, Aerospace Systems Directorate, Power and Control Division
// 
// Copyright (c) 2017 Government of the United State of America, as represented by
// the Secretary of the Air Force.  No copyright is claimed in the United States under
// Title 17, U.S. Code.  All Other Rights Reserved.
// ===============================================================================

// This file was auto-created by LmcpGen. Modifications will be overwritten.

use avtas::lmcp::{Error, ErrorType, Lmcp, LmcpSubscription, SrcLoc, Struct, StructInfo};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct StartupComplete {
}

impl PartialEq for StartupComplete {
    fn eq(&self, _other: &StartupComplete) -> bool {
        true

    }
}

impl LmcpSubscription for StartupComplete {
    fn subscription() -> &'static str { "uxas.messages.uxnative.StartupComplete" }
}

impl Struct for StartupComplete {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 2,
        }
    }
}

impl Lmcp for StartupComplete {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(StartupComplete, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == StartupComplete::struct_info() {
            let out: StartupComplete = Default::default();

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let size = 15;

        size
    }
}

pub trait StartupCompleteT: Debug + Send  {
    fn as_uxas_messages_uxnative_startup_complete(&self) -> Option<&StartupComplete> { None }
    fn as_mut_uxas_messages_uxnative_startup_complete(&mut self) -> Option<&mut StartupComplete> { None }

}

impl Clone for Box<StartupCompleteT> {
    fn clone(&self) -> Box<StartupCompleteT> {
        if let Some(x) = StartupCompleteT::as_uxas_messages_uxnative_startup_complete(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<StartupCompleteT> {
    fn default() -> Box<StartupCompleteT> { Box::new(StartupComplete::default()) }
}

impl PartialEq for Box<StartupCompleteT> {
    fn eq(&self, other: &Box<StartupCompleteT>) -> bool {
        if let (Some(x), Some(y)) =
            (StartupCompleteT::as_uxas_messages_uxnative_startup_complete(self.as_ref()),
             StartupCompleteT::as_uxas_messages_uxnative_startup_complete(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<StartupCompleteT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = StartupCompleteT::as_uxas_messages_uxnative_startup_complete(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<StartupCompleteT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == StartupComplete::struct_info() {
            let (x, readb) = StartupComplete::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = StartupCompleteT::as_uxas_messages_uxnative_startup_complete(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl StartupCompleteT for StartupComplete {
    fn as_uxas_messages_uxnative_startup_complete(&self) -> Option<&StartupComplete> { Some(self) }
    fn as_mut_uxas_messages_uxnative_startup_complete(&mut self) -> Option<&mut StartupComplete> { Some(self) }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for StartupComplete {
        fn arbitrary<G: Gen>(_g: &mut G) -> StartupComplete {
            StartupComplete {

            }
        }
    }

    quickcheck! {
        fn serializes(x: StartupComplete) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: StartupComplete) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = StartupComplete::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
