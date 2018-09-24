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
pub struct EntityLocation {
    pub entity_id: i64,
    pub position: Box<::afrl::cmasi::location3d::Location3DT>,
    pub time: i64,
}

impl PartialEq for EntityLocation {
    fn eq(&self, _other: &EntityLocation) -> bool {
        true
        && &self.entity_id == &_other.entity_id
        && &self.position == &_other.position
        && &self.time == &_other.time

    }
}

impl LmcpSubscription for EntityLocation {
    fn subscription() -> &'static str { "uxas.messages.uxnative.EntityLocation" }
}

impl Struct for EntityLocation {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 7,
        }
    }
}

impl Lmcp for EntityLocation {
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
            let writeb: usize = self.position.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(EntityLocation, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == EntityLocation::struct_info() {
            let mut out: EntityLocation = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.entity_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.position = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time = x;
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
        size += self.position.size();
        size += self.time.size();

        size
    }
}

pub trait EntityLocationT: Debug + Send  {
    fn as_uxas_messages_uxnative_entity_location(&self) -> Option<&EntityLocation> { None }
    fn as_mut_uxas_messages_uxnative_entity_location(&mut self) -> Option<&mut EntityLocation> { None }
    fn as_uxas_messages_uxnative_bandwidth_test(&self) -> Option<&::uxas::messages::uxnative::bandwidth_test::BandwidthTest> { None }
    fn as_mut_uxas_messages_uxnative_bandwidth_test(&mut self) -> Option<&mut ::uxas::messages::uxnative::bandwidth_test::BandwidthTest> { None }
    fn entity_id(&self) -> i64;
    fn entity_id_mut(&mut self) -> &mut i64;
    fn position(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn position_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn time(&self) -> i64;
    fn time_mut(&mut self) -> &mut i64;

}

impl Clone for Box<EntityLocationT> {
    fn clone(&self) -> Box<EntityLocationT> {
        if let Some(x) = EntityLocationT::as_uxas_messages_uxnative_entity_location(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityLocationT::as_uxas_messages_uxnative_bandwidth_test(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<EntityLocationT> {
    fn default() -> Box<EntityLocationT> { Box::new(EntityLocation::default()) }
}

impl PartialEq for Box<EntityLocationT> {
    fn eq(&self, other: &Box<EntityLocationT>) -> bool {
        if let (Some(x), Some(y)) =
            (EntityLocationT::as_uxas_messages_uxnative_entity_location(self.as_ref()),
             EntityLocationT::as_uxas_messages_uxnative_entity_location(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityLocationT::as_uxas_messages_uxnative_bandwidth_test(self.as_ref()),
             EntityLocationT::as_uxas_messages_uxnative_bandwidth_test(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<EntityLocationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = EntityLocationT::as_uxas_messages_uxnative_entity_location(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityLocationT::as_uxas_messages_uxnative_bandwidth_test(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<EntityLocationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == EntityLocation::struct_info() {
            let (x, readb) = EntityLocation::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::uxas::messages::uxnative::bandwidth_test::BandwidthTest::struct_info() {
            let (x, readb) = ::uxas::messages::uxnative::bandwidth_test::BandwidthTest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = EntityLocationT::as_uxas_messages_uxnative_entity_location(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityLocationT::as_uxas_messages_uxnative_bandwidth_test(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl EntityLocationT for EntityLocation {
    fn as_uxas_messages_uxnative_entity_location(&self) -> Option<&EntityLocation> { Some(self) }
    fn as_mut_uxas_messages_uxnative_entity_location(&mut self) -> Option<&mut EntityLocation> { Some(self) }
    fn entity_id(&self) -> i64 { self.entity_id }
    fn entity_id_mut(&mut self) -> &mut i64 { &mut self.entity_id }
    fn position(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.position }
    fn position_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.position }
    fn time(&self) -> i64 { self.time }
    fn time_mut(&mut self) -> &mut i64 { &mut self.time }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for EntityLocation {
        fn arbitrary<G: Gen>(_g: &mut G) -> EntityLocation {
            EntityLocation {
                entity_id: Arbitrary::arbitrary(_g),
                position: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                time: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: EntityLocation) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: EntityLocation) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = EntityLocation::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
