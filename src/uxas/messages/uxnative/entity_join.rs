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
pub struct EntityJoin {
    pub entity_id: i64,
    pub label: Vec<u8>,
}

impl PartialEq for EntityJoin {
    fn eq(&self, _other: &EntityJoin) -> bool {
        true
        && &self.entity_id == &_other.entity_id
        && &self.label == &_other.label

    }
}

impl LmcpSubscription for EntityJoin {
    fn subscription() -> &'static str { "uxas.messages.uxnative.EntityJoin" }
}

impl Struct for EntityJoin {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 14,
        }
    }
}

impl Lmcp for EntityJoin {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entity_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.label.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(EntityJoin, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == EntityJoin::struct_info() {
            let mut out: EntityJoin = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.entity_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.label = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.entity_id.size();
        size += self.label.size();

        size
    }
}

pub trait EntityJoinT: Debug + Send  {
    fn as_uxas_messages_uxnative_entity_join(&self) -> Option<&EntityJoin> { None }
    fn as_mut_uxas_messages_uxnative_entity_join(&mut self) -> Option<&mut EntityJoin> { None }
    fn entity_id(&self) -> i64;
    fn entity_id_mut(&mut self) -> &mut i64;
    fn label(&self) -> &Vec<u8>;
    fn label_mut(&mut self) -> &mut Vec<u8>;

}

impl Clone for Box<EntityJoinT> {
    fn clone(&self) -> Box<EntityJoinT> {
        if let Some(x) = EntityJoinT::as_uxas_messages_uxnative_entity_join(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<EntityJoinT> {
    fn default() -> Box<EntityJoinT> { Box::new(EntityJoin::default()) }
}

impl PartialEq for Box<EntityJoinT> {
    fn eq(&self, other: &Box<EntityJoinT>) -> bool {
        if let (Some(x), Some(y)) =
            (EntityJoinT::as_uxas_messages_uxnative_entity_join(self.as_ref()),
             EntityJoinT::as_uxas_messages_uxnative_entity_join(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<EntityJoinT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = EntityJoinT::as_uxas_messages_uxnative_entity_join(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<EntityJoinT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == EntityJoin::struct_info() {
            let (x, readb) = EntityJoin::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = EntityJoinT::as_uxas_messages_uxnative_entity_join(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl EntityJoinT for EntityJoin {
    fn as_uxas_messages_uxnative_entity_join(&self) -> Option<&EntityJoin> { Some(self) }
    fn as_mut_uxas_messages_uxnative_entity_join(&mut self) -> Option<&mut EntityJoin> { Some(self) }
    fn entity_id(&self) -> i64 { self.entity_id }
    fn entity_id_mut(&mut self) -> &mut i64 { &mut self.entity_id }
    fn label(&self) -> &Vec<u8> { &self.label }
    fn label_mut(&mut self) -> &mut Vec<u8> { &mut self.label }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for EntityJoin {
        fn arbitrary<G: Gen>(_g: &mut G) -> EntityJoin {
            EntityJoin {
                entity_id: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: EntityJoin) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: EntityJoin) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = EntityJoin::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
