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
pub struct PayloadStowAction {
    pub payload_id: i64,
}

impl PartialEq for PayloadStowAction {
    fn eq(&self, _other: &PayloadStowAction) -> bool {
        true
        && &self.payload_id == &_other.payload_id

    }
}

impl LmcpSubscription for PayloadStowAction {
    fn subscription() -> &'static str { "afrl.cmasi.PayloadStowAction" }
}

impl Struct for PayloadStowAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 60,
        }
    }
}

impl Lmcp for PayloadStowAction {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.payload_id.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PayloadStowAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PayloadStowAction::struct_info() {
            let mut out: PayloadStowAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.payload_id = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.payload_id.size();

        size
    }
}

pub trait PayloadStowActionT: Debug + Send  {
    fn as_afrl_cmasi_payload_stow_action(&self) -> Option<&PayloadStowAction> { None }
    fn as_mut_afrl_cmasi_payload_stow_action(&mut self) -> Option<&mut PayloadStowAction> { None }
    fn payload_id(&self) -> i64;
    fn payload_id_mut(&mut self) -> &mut i64;

}

impl Clone for Box<PayloadStowActionT> {
    fn clone(&self) -> Box<PayloadStowActionT> {
        if let Some(x) = PayloadStowActionT::as_afrl_cmasi_payload_stow_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PayloadStowActionT> {
    fn default() -> Box<PayloadStowActionT> { Box::new(PayloadStowAction::default()) }
}

impl PartialEq for Box<PayloadStowActionT> {
    fn eq(&self, other: &Box<PayloadStowActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (PayloadStowActionT::as_afrl_cmasi_payload_stow_action(self.as_ref()),
             PayloadStowActionT::as_afrl_cmasi_payload_stow_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PayloadStowActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PayloadStowActionT::as_afrl_cmasi_payload_stow_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PayloadStowActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PayloadStowAction::struct_info() {
            let (x, readb) = PayloadStowAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PayloadStowActionT::as_afrl_cmasi_payload_stow_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl PayloadStowActionT for PayloadStowAction {
    fn as_afrl_cmasi_payload_stow_action(&self) -> Option<&PayloadStowAction> { Some(self) }
    fn as_mut_afrl_cmasi_payload_stow_action(&mut self) -> Option<&mut PayloadStowAction> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PayloadStowAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> PayloadStowAction {
            PayloadStowAction {
                payload_id: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PayloadStowAction) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PayloadStowAction) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PayloadStowAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
