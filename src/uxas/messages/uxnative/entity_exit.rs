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
pub struct EntityExit {
    pub entity_id: i64,
    pub label: Vec<u8>,
}

impl PartialEq for EntityExit {
    fn eq(&self, _other: &EntityExit) -> bool {
        true
        && &self.entity_id == &_other.entity_id
        && &self.label == &_other.label

    }
}

impl LmcpSubscription for EntityExit {
    fn subscription() -> &'static str { "uxas.messages.uxnative.EntityExit" }
}

impl Struct for EntityExit {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 15,
        }
    }
}

impl Lmcp for EntityExit {
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

    fn deser(buf: &[u8]) -> Result<(EntityExit, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == EntityExit::struct_info() {
            let mut out: EntityExit = Default::default();
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

pub trait EntityExitT: Debug + Send  {
    fn as_uxas_messages_uxnative_entity_exit(&self) -> Option<&EntityExit> { None }
    fn as_mut_uxas_messages_uxnative_entity_exit(&mut self) -> Option<&mut EntityExit> { None }
    fn entity_id(&self) -> i64;
    fn entity_id_mut(&mut self) -> &mut i64;
    fn label(&self) -> &Vec<u8>;
    fn label_mut(&mut self) -> &mut Vec<u8>;

}

impl Clone for Box<EntityExitT> {
    fn clone(&self) -> Box<EntityExitT> {
        if let Some(x) = EntityExitT::as_uxas_messages_uxnative_entity_exit(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<EntityExitT> {
    fn default() -> Box<EntityExitT> { Box::new(EntityExit::default()) }
}

impl PartialEq for Box<EntityExitT> {
    fn eq(&self, other: &Box<EntityExitT>) -> bool {
        if let (Some(x), Some(y)) =
            (EntityExitT::as_uxas_messages_uxnative_entity_exit(self.as_ref()),
             EntityExitT::as_uxas_messages_uxnative_entity_exit(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<EntityExitT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = EntityExitT::as_uxas_messages_uxnative_entity_exit(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<EntityExitT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == EntityExit::struct_info() {
            let (x, readb) = EntityExit::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = EntityExitT::as_uxas_messages_uxnative_entity_exit(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl EntityExitT for EntityExit {
    fn as_uxas_messages_uxnative_entity_exit(&self) -> Option<&EntityExit> { Some(self) }
    fn as_mut_uxas_messages_uxnative_entity_exit(&mut self) -> Option<&mut EntityExit> { Some(self) }
    fn entity_id(&self) -> i64 { self.entity_id }
    fn entity_id_mut(&mut self) -> &mut i64 { &mut self.entity_id }
    fn label(&self) -> &Vec<u8> { &self.label }
    fn label_mut(&mut self) -> &mut Vec<u8> { &mut self.label }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for EntityExit {
        fn arbitrary<G: Gen>(_g: &mut G) -> EntityExit {
            EntityExit {
                entity_id: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: EntityExit) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: EntityExit) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = EntityExit::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
