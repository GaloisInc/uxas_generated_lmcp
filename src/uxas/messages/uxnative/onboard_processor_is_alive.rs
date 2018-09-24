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
pub struct OnboardProcessorIsAlive {
    pub vehicle_id: i64,
    pub time_sent: i64,
}

impl PartialEq for OnboardProcessorIsAlive {
    fn eq(&self, _other: &OnboardProcessorIsAlive) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id
        && &self.time_sent == &_other.time_sent

    }
}

impl LmcpSubscription for OnboardProcessorIsAlive {
    fn subscription() -> &'static str { "uxas.messages.uxnative.OnboardProcessorIsAlive" }
}

impl Struct for OnboardProcessorIsAlive {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 3,
            struct_ty: 12,
        }
    }
}

impl Lmcp for OnboardProcessorIsAlive {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_sent.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(OnboardProcessorIsAlive, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == OnboardProcessorIsAlive::struct_info() {
            let mut out: OnboardProcessorIsAlive = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time_sent = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.vehicle_id.size();
        size += self.time_sent.size();

        size
    }
}

pub trait OnboardProcessorIsAliveT: Debug + Send  {
    fn as_uxas_messages_uxnative_onboard_processor_is_alive(&self) -> Option<&OnboardProcessorIsAlive> { None }
    fn as_mut_uxas_messages_uxnative_onboard_processor_is_alive(&mut self) -> Option<&mut OnboardProcessorIsAlive> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn time_sent(&self) -> i64;
    fn time_sent_mut(&mut self) -> &mut i64;

}

impl Clone for Box<OnboardProcessorIsAliveT> {
    fn clone(&self) -> Box<OnboardProcessorIsAliveT> {
        if let Some(x) = OnboardProcessorIsAliveT::as_uxas_messages_uxnative_onboard_processor_is_alive(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<OnboardProcessorIsAliveT> {
    fn default() -> Box<OnboardProcessorIsAliveT> { Box::new(OnboardProcessorIsAlive::default()) }
}

impl PartialEq for Box<OnboardProcessorIsAliveT> {
    fn eq(&self, other: &Box<OnboardProcessorIsAliveT>) -> bool {
        if let (Some(x), Some(y)) =
            (OnboardProcessorIsAliveT::as_uxas_messages_uxnative_onboard_processor_is_alive(self.as_ref()),
             OnboardProcessorIsAliveT::as_uxas_messages_uxnative_onboard_processor_is_alive(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<OnboardProcessorIsAliveT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = OnboardProcessorIsAliveT::as_uxas_messages_uxnative_onboard_processor_is_alive(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<OnboardProcessorIsAliveT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == OnboardProcessorIsAlive::struct_info() {
            let (x, readb) = OnboardProcessorIsAlive::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = OnboardProcessorIsAliveT::as_uxas_messages_uxnative_onboard_processor_is_alive(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl OnboardProcessorIsAliveT for OnboardProcessorIsAlive {
    fn as_uxas_messages_uxnative_onboard_processor_is_alive(&self) -> Option<&OnboardProcessorIsAlive> { Some(self) }
    fn as_mut_uxas_messages_uxnative_onboard_processor_is_alive(&mut self) -> Option<&mut OnboardProcessorIsAlive> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn time_sent(&self) -> i64 { self.time_sent }
    fn time_sent_mut(&mut self) -> &mut i64 { &mut self.time_sent }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for OnboardProcessorIsAlive {
        fn arbitrary<G: Gen>(_g: &mut G) -> OnboardProcessorIsAlive {
            OnboardProcessorIsAlive {
                vehicle_id: Arbitrary::arbitrary(_g),
                time_sent: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: OnboardProcessorIsAlive) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: OnboardProcessorIsAlive) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = OnboardProcessorIsAlive::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
