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

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct OperatorSignal {
    pub signals: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
}

impl PartialEq for OperatorSignal {
    fn eq(&self, _other: &OperatorSignal) -> bool {
        true
        && &self.signals == &_other.signals

    }
}

impl LmcpSubscription for OperatorSignal {
    fn subscription() -> &'static str { "afrl.cmasi.OperatorSignal" }
}

impl Struct for OperatorSignal {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 38,
        }
    }
}

impl Lmcp for OperatorSignal {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.signals.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(OperatorSignal, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == OperatorSignal::struct_info() {
            let mut out: OperatorSignal = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>, usize) = Lmcp::deser(r)?;
                out.signals = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.signals.size();

        size
    }
}

pub trait OperatorSignalT: Debug + Send  {
    fn as_afrl_cmasi_operator_signal(&self) -> Option<&OperatorSignal> { None }
    fn as_mut_afrl_cmasi_operator_signal(&mut self) -> Option<&mut OperatorSignal> { None }
    fn signals(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn signals_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;

}

impl Clone for Box<OperatorSignalT> {
    fn clone(&self) -> Box<OperatorSignalT> {
        if let Some(x) = OperatorSignalT::as_afrl_cmasi_operator_signal(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<OperatorSignalT> {
    fn default() -> Box<OperatorSignalT> { Box::new(OperatorSignal::default()) }
}

impl PartialEq for Box<OperatorSignalT> {
    fn eq(&self, other: &Box<OperatorSignalT>) -> bool {
        if let (Some(x), Some(y)) =
            (OperatorSignalT::as_afrl_cmasi_operator_signal(self.as_ref()),
             OperatorSignalT::as_afrl_cmasi_operator_signal(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<OperatorSignalT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = OperatorSignalT::as_afrl_cmasi_operator_signal(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<OperatorSignalT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == OperatorSignal::struct_info() {
            let (x, readb) = OperatorSignal::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = OperatorSignalT::as_afrl_cmasi_operator_signal(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl OperatorSignalT for OperatorSignal {
    fn as_afrl_cmasi_operator_signal(&self) -> Option<&OperatorSignal> { Some(self) }
    fn as_mut_afrl_cmasi_operator_signal(&mut self) -> Option<&mut OperatorSignal> { Some(self) }
    fn signals(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.signals }
    fn signals_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.signals }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for OperatorSignal {
        fn arbitrary<G: Gen>(_g: &mut G) -> OperatorSignal {
            OperatorSignal {
                signals: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: OperatorSignal) -> Result<TestResult, Error> {
            use std::u16;
            if x.signals.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: OperatorSignal) -> Result<TestResult, Error> {
            use std::u16;
            if x.signals.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = OperatorSignal::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
