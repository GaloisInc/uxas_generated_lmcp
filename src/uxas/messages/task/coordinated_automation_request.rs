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
pub struct CoordinatedAutomationRequest {
    pub request_id: i64,
    pub maximum_response_time: i64,
    pub original_request: Box<::afrl::cmasi::automation_request::AutomationRequestT>,
    pub planning_states: Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>,
}

impl PartialEq for CoordinatedAutomationRequest {
    fn eq(&self, _other: &CoordinatedAutomationRequest) -> bool {
        true
        && &self.request_id == &_other.request_id
        && &self.maximum_response_time == &_other.maximum_response_time
        && &self.original_request == &_other.original_request
        && &self.planning_states == &_other.planning_states

    }
}

impl LmcpSubscription for CoordinatedAutomationRequest {
    fn subscription() -> &'static str { "uxas.messages.task.CoordinatedAutomationRequest" }
}

impl Struct for CoordinatedAutomationRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 5,
        }
    }
}

impl Lmcp for CoordinatedAutomationRequest {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.request_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.maximum_response_time.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.original_request.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.planning_states.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(CoordinatedAutomationRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == CoordinatedAutomationRequest::struct_info() {
            let mut out: CoordinatedAutomationRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.maximum_response_time = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::automation_request::AutomationRequestT>, usize) = Lmcp::deser(r)?;
                out.original_request = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>, usize) = Lmcp::deser(r)?;
                out.planning_states = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.request_id.size();
        size += self.maximum_response_time.size();
        size += self.original_request.size();
        size += self.planning_states.size();

        size
    }
}

pub trait CoordinatedAutomationRequestT: Debug + Send  {
    fn as_uxas_messages_task_coordinated_automation_request(&self) -> Option<&CoordinatedAutomationRequest> { None }
    fn as_mut_uxas_messages_task_coordinated_automation_request(&mut self) -> Option<&mut CoordinatedAutomationRequest> { None }
    fn request_id(&self) -> i64;
    fn request_id_mut(&mut self) -> &mut i64;
    fn maximum_response_time(&self) -> i64;
    fn maximum_response_time_mut(&mut self) -> &mut i64;
    fn original_request(&self) -> &Box<::afrl::cmasi::automation_request::AutomationRequestT>;
    fn original_request_mut(&mut self) -> &mut Box<::afrl::cmasi::automation_request::AutomationRequestT>;
    fn planning_states(&self) -> &Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>;
    fn planning_states_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>;

}

impl Clone for Box<CoordinatedAutomationRequestT> {
    fn clone(&self) -> Box<CoordinatedAutomationRequestT> {
        if let Some(x) = CoordinatedAutomationRequestT::as_uxas_messages_task_coordinated_automation_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<CoordinatedAutomationRequestT> {
    fn default() -> Box<CoordinatedAutomationRequestT> { Box::new(CoordinatedAutomationRequest::default()) }
}

impl PartialEq for Box<CoordinatedAutomationRequestT> {
    fn eq(&self, other: &Box<CoordinatedAutomationRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (CoordinatedAutomationRequestT::as_uxas_messages_task_coordinated_automation_request(self.as_ref()),
             CoordinatedAutomationRequestT::as_uxas_messages_task_coordinated_automation_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<CoordinatedAutomationRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = CoordinatedAutomationRequestT::as_uxas_messages_task_coordinated_automation_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<CoordinatedAutomationRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == CoordinatedAutomationRequest::struct_info() {
            let (x, readb) = CoordinatedAutomationRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = CoordinatedAutomationRequestT::as_uxas_messages_task_coordinated_automation_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl CoordinatedAutomationRequestT for CoordinatedAutomationRequest {
    fn as_uxas_messages_task_coordinated_automation_request(&self) -> Option<&CoordinatedAutomationRequest> { Some(self) }
    fn as_mut_uxas_messages_task_coordinated_automation_request(&mut self) -> Option<&mut CoordinatedAutomationRequest> { Some(self) }
    fn request_id(&self) -> i64 { self.request_id }
    fn request_id_mut(&mut self) -> &mut i64 { &mut self.request_id }
    fn maximum_response_time(&self) -> i64 { self.maximum_response_time }
    fn maximum_response_time_mut(&mut self) -> &mut i64 { &mut self.maximum_response_time }
    fn original_request(&self) -> &Box<::afrl::cmasi::automation_request::AutomationRequestT> { &self.original_request }
    fn original_request_mut(&mut self) -> &mut Box<::afrl::cmasi::automation_request::AutomationRequestT> { &mut self.original_request }
    fn planning_states(&self) -> &Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>> { &self.planning_states }
    fn planning_states_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>> { &mut self.planning_states }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for CoordinatedAutomationRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> CoordinatedAutomationRequest {
            CoordinatedAutomationRequest {
                request_id: Arbitrary::arbitrary(_g),
                maximum_response_time: Arbitrary::arbitrary(_g),
                original_request: Box::new(::afrl::cmasi::automation_request::AutomationRequest::arbitrary(_g)),
                planning_states: Vec::<::uxas::messages::task::planning_state::PlanningState>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::task::planning_state::PlanningStateT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: CoordinatedAutomationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.planning_states.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: CoordinatedAutomationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.planning_states.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = CoordinatedAutomationRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
