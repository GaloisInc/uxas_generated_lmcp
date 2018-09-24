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
pub struct OnboardStatusReport {
    pub vehicle_id: i64,
    pub connected_entities: Vec<i64>,
    pub current_task_list: Vec<i64>,
    pub valid_state: bool,
    pub valid_authorization: bool,
    pub speed_authorization: bool,
    pub gimbal_authorization: bool,
    pub vehicle_time: i64,
}

impl PartialEq for OnboardStatusReport {
    fn eq(&self, _other: &OnboardStatusReport) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id
        && &self.connected_entities == &_other.connected_entities
        && &self.current_task_list == &_other.current_task_list
        && &self.valid_state == &_other.valid_state
        && &self.valid_authorization == &_other.valid_authorization
        && &self.speed_authorization == &_other.speed_authorization
        && &self.gimbal_authorization == &_other.gimbal_authorization
        && &self.vehicle_time == &_other.vehicle_time

    }
}

impl LmcpSubscription for OnboardStatusReport {
    fn subscription() -> &'static str { "uxas.messages.uxnative.OnboardStatusReport" }
}

impl Struct for OnboardStatusReport {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 13,
        }
    }
}

impl Lmcp for OnboardStatusReport {
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
            let writeb: usize = self.connected_entities.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.current_task_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.valid_state.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.valid_authorization.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.speed_authorization.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.gimbal_authorization.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_time.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(OnboardStatusReport, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == OnboardStatusReport::struct_info() {
            let mut out: OnboardStatusReport = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.connected_entities = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.current_task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.valid_state = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.valid_authorization = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.speed_authorization = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.gimbal_authorization = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_time = x;
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
        size += self.connected_entities.size();
        size += self.current_task_list.size();
        size += self.valid_state.size();
        size += self.valid_authorization.size();
        size += self.speed_authorization.size();
        size += self.gimbal_authorization.size();
        size += self.vehicle_time.size();

        size
    }
}

pub trait OnboardStatusReportT: Debug + Send  {
    fn as_uxas_messages_uxnative_onboard_status_report(&self) -> Option<&OnboardStatusReport> { None }
    fn as_mut_uxas_messages_uxnative_onboard_status_report(&mut self) -> Option<&mut OnboardStatusReport> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn connected_entities(&self) -> &Vec<i64>;
    fn connected_entities_mut(&mut self) -> &mut Vec<i64>;
    fn current_task_list(&self) -> &Vec<i64>;
    fn current_task_list_mut(&mut self) -> &mut Vec<i64>;
    fn valid_state(&self) -> bool;
    fn valid_state_mut(&mut self) -> &mut bool;
    fn valid_authorization(&self) -> bool;
    fn valid_authorization_mut(&mut self) -> &mut bool;
    fn speed_authorization(&self) -> bool;
    fn speed_authorization_mut(&mut self) -> &mut bool;
    fn gimbal_authorization(&self) -> bool;
    fn gimbal_authorization_mut(&mut self) -> &mut bool;
    fn vehicle_time(&self) -> i64;
    fn vehicle_time_mut(&mut self) -> &mut i64;

}

impl Clone for Box<OnboardStatusReportT> {
    fn clone(&self) -> Box<OnboardStatusReportT> {
        if let Some(x) = OnboardStatusReportT::as_uxas_messages_uxnative_onboard_status_report(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<OnboardStatusReportT> {
    fn default() -> Box<OnboardStatusReportT> { Box::new(OnboardStatusReport::default()) }
}

impl PartialEq for Box<OnboardStatusReportT> {
    fn eq(&self, other: &Box<OnboardStatusReportT>) -> bool {
        if let (Some(x), Some(y)) =
            (OnboardStatusReportT::as_uxas_messages_uxnative_onboard_status_report(self.as_ref()),
             OnboardStatusReportT::as_uxas_messages_uxnative_onboard_status_report(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<OnboardStatusReportT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = OnboardStatusReportT::as_uxas_messages_uxnative_onboard_status_report(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<OnboardStatusReportT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == OnboardStatusReport::struct_info() {
            let (x, readb) = OnboardStatusReport::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = OnboardStatusReportT::as_uxas_messages_uxnative_onboard_status_report(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl OnboardStatusReportT for OnboardStatusReport {
    fn as_uxas_messages_uxnative_onboard_status_report(&self) -> Option<&OnboardStatusReport> { Some(self) }
    fn as_mut_uxas_messages_uxnative_onboard_status_report(&mut self) -> Option<&mut OnboardStatusReport> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn connected_entities(&self) -> &Vec<i64> { &self.connected_entities }
    fn connected_entities_mut(&mut self) -> &mut Vec<i64> { &mut self.connected_entities }
    fn current_task_list(&self) -> &Vec<i64> { &self.current_task_list }
    fn current_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.current_task_list }
    fn valid_state(&self) -> bool { self.valid_state }
    fn valid_state_mut(&mut self) -> &mut bool { &mut self.valid_state }
    fn valid_authorization(&self) -> bool { self.valid_authorization }
    fn valid_authorization_mut(&mut self) -> &mut bool { &mut self.valid_authorization }
    fn speed_authorization(&self) -> bool { self.speed_authorization }
    fn speed_authorization_mut(&mut self) -> &mut bool { &mut self.speed_authorization }
    fn gimbal_authorization(&self) -> bool { self.gimbal_authorization }
    fn gimbal_authorization_mut(&mut self) -> &mut bool { &mut self.gimbal_authorization }
    fn vehicle_time(&self) -> i64 { self.vehicle_time }
    fn vehicle_time_mut(&mut self) -> &mut i64 { &mut self.vehicle_time }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for OnboardStatusReport {
        fn arbitrary<G: Gen>(_g: &mut G) -> OnboardStatusReport {
            OnboardStatusReport {
                vehicle_id: Arbitrary::arbitrary(_g),
                connected_entities: Arbitrary::arbitrary(_g),
                current_task_list: Arbitrary::arbitrary(_g),
                valid_state: Arbitrary::arbitrary(_g),
                valid_authorization: Arbitrary::arbitrary(_g),
                speed_authorization: Arbitrary::arbitrary(_g),
                gimbal_authorization: Arbitrary::arbitrary(_g),
                vehicle_time: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: OnboardStatusReport) -> Result<TestResult, Error> {
            use std::u16;
            if x.connected_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.current_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: OnboardStatusReport) -> Result<TestResult, Error> {
            use std::u16;
            if x.connected_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.current_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = OnboardStatusReport::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
