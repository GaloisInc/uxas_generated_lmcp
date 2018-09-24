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
pub struct RemoveEntities {
    pub entity_list: Vec<i64>,
}

impl PartialEq for RemoveEntities {
    fn eq(&self, _other: &RemoveEntities) -> bool {
        true
        && &self.entity_list == &_other.entity_list

    }
}

impl LmcpSubscription for RemoveEntities {
    fn subscription() -> &'static str { "afrl.cmasi.RemoveEntities" }
}

impl Struct for RemoveEntities {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 53,
        }
    }
}

impl Lmcp for RemoveEntities {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entity_list.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RemoveEntities, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RemoveEntities::struct_info() {
            let mut out: RemoveEntities = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.entity_list = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.entity_list.size();

        size
    }
}

pub trait RemoveEntitiesT: Debug + Send  {
    fn as_afrl_cmasi_remove_entities(&self) -> Option<&RemoveEntities> { None }
    fn as_mut_afrl_cmasi_remove_entities(&mut self) -> Option<&mut RemoveEntities> { None }
    fn entity_list(&self) -> &Vec<i64>;
    fn entity_list_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<RemoveEntitiesT> {
    fn clone(&self) -> Box<RemoveEntitiesT> {
        if let Some(x) = RemoveEntitiesT::as_afrl_cmasi_remove_entities(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RemoveEntitiesT> {
    fn default() -> Box<RemoveEntitiesT> { Box::new(RemoveEntities::default()) }
}

impl PartialEq for Box<RemoveEntitiesT> {
    fn eq(&self, other: &Box<RemoveEntitiesT>) -> bool {
        if let (Some(x), Some(y)) =
            (RemoveEntitiesT::as_afrl_cmasi_remove_entities(self.as_ref()),
             RemoveEntitiesT::as_afrl_cmasi_remove_entities(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RemoveEntitiesT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RemoveEntitiesT::as_afrl_cmasi_remove_entities(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RemoveEntitiesT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RemoveEntities::struct_info() {
            let (x, readb) = RemoveEntities::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RemoveEntitiesT::as_afrl_cmasi_remove_entities(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RemoveEntitiesT for RemoveEntities {
    fn as_afrl_cmasi_remove_entities(&self) -> Option<&RemoveEntities> { Some(self) }
    fn as_mut_afrl_cmasi_remove_entities(&mut self) -> Option<&mut RemoveEntities> { Some(self) }
    fn entity_list(&self) -> &Vec<i64> { &self.entity_list }
    fn entity_list_mut(&mut self) -> &mut Vec<i64> { &mut self.entity_list }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RemoveEntities {
        fn arbitrary<G: Gen>(_g: &mut G) -> RemoveEntities {
            RemoveEntities {
                entity_list: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RemoveEntities) -> Result<TestResult, Error> {
            use std::u16;
            if x.entity_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RemoveEntities) -> Result<TestResult, Error> {
            use std::u16;
            if x.entity_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RemoveEntities::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
