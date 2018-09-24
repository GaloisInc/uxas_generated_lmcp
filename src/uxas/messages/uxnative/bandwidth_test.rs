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
pub struct BandwidthTest {
    pub entity_id: i64,
    pub position: Box<::afrl::cmasi::location3d::Location3DT>,
    pub time: i64,
    pub message_id: i64,
    pub payload: Vec<u8>,
}

impl PartialEq for BandwidthTest {
    fn eq(&self, _other: &BandwidthTest) -> bool {
        true
        && &self.message_id == &_other.message_id
        && &self.payload == &_other.payload

    }
}

impl LmcpSubscription for BandwidthTest {
    fn subscription() -> &'static str { "uxas.messages.uxnative.BandwidthTest" }
}

impl Struct for BandwidthTest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 8,
        }
    }
}

impl Lmcp for BandwidthTest {
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
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.message_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.payload.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(BandwidthTest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == BandwidthTest::struct_info() {
            let mut out: BandwidthTest = Default::default();
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
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.message_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.payload = x;
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
        size += self.message_id.size();
        size += self.payload.size();

        size
    }
}

pub trait BandwidthTestT: Debug + Send + ::uxas::messages::uxnative::entity_location::EntityLocationT {
    fn as_uxas_messages_uxnative_bandwidth_test(&self) -> Option<&BandwidthTest> { None }
    fn as_mut_uxas_messages_uxnative_bandwidth_test(&mut self) -> Option<&mut BandwidthTest> { None }
    fn message_id(&self) -> i64;
    fn message_id_mut(&mut self) -> &mut i64;
    fn payload(&self) -> &Vec<u8>;
    fn payload_mut(&mut self) -> &mut Vec<u8>;

}

impl Clone for Box<BandwidthTestT> {
    fn clone(&self) -> Box<BandwidthTestT> {
        if let Some(x) = BandwidthTestT::as_uxas_messages_uxnative_bandwidth_test(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<BandwidthTestT> {
    fn default() -> Box<BandwidthTestT> { Box::new(BandwidthTest::default()) }
}

impl PartialEq for Box<BandwidthTestT> {
    fn eq(&self, other: &Box<BandwidthTestT>) -> bool {
        if let (Some(x), Some(y)) =
            (BandwidthTestT::as_uxas_messages_uxnative_bandwidth_test(self.as_ref()),
             BandwidthTestT::as_uxas_messages_uxnative_bandwidth_test(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<BandwidthTestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = BandwidthTestT::as_uxas_messages_uxnative_bandwidth_test(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<BandwidthTestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == BandwidthTest::struct_info() {
            let (x, readb) = BandwidthTest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = BandwidthTestT::as_uxas_messages_uxnative_bandwidth_test(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::uxas::messages::uxnative::entity_location::EntityLocationT for BandwidthTest {
    fn as_uxas_messages_uxnative_bandwidth_test(&self) -> Option<&BandwidthTest> { Some(self) }
    fn as_mut_uxas_messages_uxnative_bandwidth_test(&mut self) -> Option<&mut BandwidthTest> { Some(self) }
    fn entity_id(&self) -> i64 { self.entity_id }
    fn entity_id_mut(&mut self) -> &mut i64 { &mut self.entity_id }
    fn position(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.position }
    fn position_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.position }
    fn time(&self) -> i64 { self.time }
    fn time_mut(&mut self) -> &mut i64 { &mut self.time }
}
impl BandwidthTestT for BandwidthTest {
    fn as_uxas_messages_uxnative_bandwidth_test(&self) -> Option<&BandwidthTest> { Some(self) }
    fn as_mut_uxas_messages_uxnative_bandwidth_test(&mut self) -> Option<&mut BandwidthTest> { Some(self) }
    fn message_id(&self) -> i64 { self.message_id }
    fn message_id_mut(&mut self) -> &mut i64 { &mut self.message_id }
    fn payload(&self) -> &Vec<u8> { &self.payload }
    fn payload_mut(&mut self) -> &mut Vec<u8> { &mut self.payload }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for BandwidthTest {
        fn arbitrary<G: Gen>(_g: &mut G) -> BandwidthTest {
            BandwidthTest {
                entity_id: Arbitrary::arbitrary(_g),
                position: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                time: Arbitrary::arbitrary(_g),
                message_id: Arbitrary::arbitrary(_g),
                payload: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: BandwidthTest) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: BandwidthTest) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = BandwidthTest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
