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
pub struct KeyValuePair {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

impl PartialEq for KeyValuePair {
    fn eq(&self, _other: &KeyValuePair) -> bool {
        true
        && &self.key == &_other.key
        && &self.value == &_other.value

    }
}

impl LmcpSubscription for KeyValuePair {
    fn subscription() -> &'static str { "afrl.cmasi.KeyValuePair" }
}

impl Struct for KeyValuePair {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 2,
        }
    }
}

impl Lmcp for KeyValuePair {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.key.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.value.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(KeyValuePair, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == KeyValuePair::struct_info() {
            let mut out: KeyValuePair = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.key = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.value = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.key.size();
        size += self.value.size();

        size
    }
}

pub trait KeyValuePairT: Debug + Send  {
    fn as_afrl_cmasi_key_value_pair(&self) -> Option<&KeyValuePair> { None }
    fn as_mut_afrl_cmasi_key_value_pair(&mut self) -> Option<&mut KeyValuePair> { None }
    fn key(&self) -> &Vec<u8>;
    fn key_mut(&mut self) -> &mut Vec<u8>;
    fn value(&self) -> &Vec<u8>;
    fn value_mut(&mut self) -> &mut Vec<u8>;

}

impl Clone for Box<KeyValuePairT> {
    fn clone(&self) -> Box<KeyValuePairT> {
        if let Some(x) = KeyValuePairT::as_afrl_cmasi_key_value_pair(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<KeyValuePairT> {
    fn default() -> Box<KeyValuePairT> { Box::new(KeyValuePair::default()) }
}

impl PartialEq for Box<KeyValuePairT> {
    fn eq(&self, other: &Box<KeyValuePairT>) -> bool {
        if let (Some(x), Some(y)) =
            (KeyValuePairT::as_afrl_cmasi_key_value_pair(self.as_ref()),
             KeyValuePairT::as_afrl_cmasi_key_value_pair(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<KeyValuePairT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = KeyValuePairT::as_afrl_cmasi_key_value_pair(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<KeyValuePairT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == KeyValuePair::struct_info() {
            let (x, readb) = KeyValuePair::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = KeyValuePairT::as_afrl_cmasi_key_value_pair(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl KeyValuePairT for KeyValuePair {
    fn as_afrl_cmasi_key_value_pair(&self) -> Option<&KeyValuePair> { Some(self) }
    fn as_mut_afrl_cmasi_key_value_pair(&mut self) -> Option<&mut KeyValuePair> { Some(self) }
    fn key(&self) -> &Vec<u8> { &self.key }
    fn key_mut(&mut self) -> &mut Vec<u8> { &mut self.key }
    fn value(&self) -> &Vec<u8> { &self.value }
    fn value_mut(&mut self) -> &mut Vec<u8> { &mut self.value }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for KeyValuePair {
        fn arbitrary<G: Gen>(_g: &mut G) -> KeyValuePair {
            KeyValuePair {
                key: Arbitrary::arbitrary(_g),
                value: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: KeyValuePair) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: KeyValuePair) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = KeyValuePair::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
