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
pub struct UniqueAutomationResponse {
    pub response_id: i64,
    pub original_response: Box<::afrl::cmasi::automation_response::AutomationResponseT>,
    pub final_states: Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>,
}

impl PartialEq for UniqueAutomationResponse {
    fn eq(&self, _other: &UniqueAutomationResponse) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.original_response == &_other.original_response
        && &self.final_states == &_other.final_states

    }
}

impl LmcpSubscription for UniqueAutomationResponse {
    fn subscription() -> &'static str { "uxas.messages.task.UniqueAutomationResponse" }
}

impl Struct for UniqueAutomationResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 9,
        }
    }
}

impl Lmcp for UniqueAutomationResponse {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.response_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.original_response.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.final_states.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(UniqueAutomationResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == UniqueAutomationResponse::struct_info() {
            let mut out: UniqueAutomationResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::automation_response::AutomationResponseT>, usize) = Lmcp::deser(r)?;
                out.original_response = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>, usize) = Lmcp::deser(r)?;
                out.final_states = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.response_id.size();
        size += self.original_response.size();
        size += self.final_states.size();

        size
    }
}

pub trait UniqueAutomationResponseT: Debug + Send  {
    fn as_uxas_messages_task_unique_automation_response(&self) -> Option<&UniqueAutomationResponse> { None }
    fn as_mut_uxas_messages_task_unique_automation_response(&mut self) -> Option<&mut UniqueAutomationResponse> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn original_response(&self) -> &Box<::afrl::cmasi::automation_response::AutomationResponseT>;
    fn original_response_mut(&mut self) -> &mut Box<::afrl::cmasi::automation_response::AutomationResponseT>;
    fn final_states(&self) -> &Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>;
    fn final_states_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>;

}

impl Clone for Box<UniqueAutomationResponseT> {
    fn clone(&self) -> Box<UniqueAutomationResponseT> {
        if let Some(x) = UniqueAutomationResponseT::as_uxas_messages_task_unique_automation_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<UniqueAutomationResponseT> {
    fn default() -> Box<UniqueAutomationResponseT> { Box::new(UniqueAutomationResponse::default()) }
}

impl PartialEq for Box<UniqueAutomationResponseT> {
    fn eq(&self, other: &Box<UniqueAutomationResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (UniqueAutomationResponseT::as_uxas_messages_task_unique_automation_response(self.as_ref()),
             UniqueAutomationResponseT::as_uxas_messages_task_unique_automation_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<UniqueAutomationResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = UniqueAutomationResponseT::as_uxas_messages_task_unique_automation_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<UniqueAutomationResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == UniqueAutomationResponse::struct_info() {
            let (x, readb) = UniqueAutomationResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = UniqueAutomationResponseT::as_uxas_messages_task_unique_automation_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl UniqueAutomationResponseT for UniqueAutomationResponse {
    fn as_uxas_messages_task_unique_automation_response(&self) -> Option<&UniqueAutomationResponse> { Some(self) }
    fn as_mut_uxas_messages_task_unique_automation_response(&mut self) -> Option<&mut UniqueAutomationResponse> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn original_response(&self) -> &Box<::afrl::cmasi::automation_response::AutomationResponseT> { &self.original_response }
    fn original_response_mut(&mut self) -> &mut Box<::afrl::cmasi::automation_response::AutomationResponseT> { &mut self.original_response }
    fn final_states(&self) -> &Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>> { &self.final_states }
    fn final_states_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>> { &mut self.final_states }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for UniqueAutomationResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> UniqueAutomationResponse {
            UniqueAutomationResponse {
                response_id: Arbitrary::arbitrary(_g),
                original_response: Box::new(::afrl::cmasi::automation_response::AutomationResponse::arbitrary(_g)),
                final_states: Vec::<::uxas::messages::task::planning_state::PlanningState>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::task::planning_state::PlanningStateT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: UniqueAutomationResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.final_states.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: UniqueAutomationResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.final_states.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = UniqueAutomationResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
