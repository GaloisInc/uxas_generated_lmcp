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
pub struct KillService {
    pub service_id: i64,
}

impl PartialEq for KillService {
    fn eq(&self, _other: &KillService) -> bool {
        true
        && &self.service_id == &_other.service_id

    }
}

impl LmcpSubscription for KillService {
    fn subscription() -> &'static str { "uxas.messages.uxnative.KillService" }
}

impl Struct for KillService {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 4,
        }
    }
}

impl Lmcp for KillService {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.service_id.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(KillService, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == KillService::struct_info() {
            let mut out: KillService = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.service_id = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.service_id.size();

        size
    }
}

pub trait KillServiceT: Debug + Send  {
    fn as_uxas_messages_uxnative_kill_service(&self) -> Option<&KillService> { None }
    fn as_mut_uxas_messages_uxnative_kill_service(&mut self) -> Option<&mut KillService> { None }
    fn service_id(&self) -> i64;
    fn service_id_mut(&mut self) -> &mut i64;

}

impl Clone for Box<KillServiceT> {
    fn clone(&self) -> Box<KillServiceT> {
        if let Some(x) = KillServiceT::as_uxas_messages_uxnative_kill_service(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<KillServiceT> {
    fn default() -> Box<KillServiceT> { Box::new(KillService::default()) }
}

impl PartialEq for Box<KillServiceT> {
    fn eq(&self, other: &Box<KillServiceT>) -> bool {
        if let (Some(x), Some(y)) =
            (KillServiceT::as_uxas_messages_uxnative_kill_service(self.as_ref()),
             KillServiceT::as_uxas_messages_uxnative_kill_service(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<KillServiceT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = KillServiceT::as_uxas_messages_uxnative_kill_service(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<KillServiceT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == KillService::struct_info() {
            let (x, readb) = KillService::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = KillServiceT::as_uxas_messages_uxnative_kill_service(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl KillServiceT for KillService {
    fn as_uxas_messages_uxnative_kill_service(&self) -> Option<&KillService> { Some(self) }
    fn as_mut_uxas_messages_uxnative_kill_service(&mut self) -> Option<&mut KillService> { Some(self) }
    fn service_id(&self) -> i64 { self.service_id }
    fn service_id_mut(&mut self) -> &mut i64 { &mut self.service_id }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for KillService {
        fn arbitrary<G: Gen>(_g: &mut G) -> KillService {
            KillService {
                service_id: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: KillService) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: KillService) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = KillService::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
