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
pub struct UniqueAutomationRequest {
    pub request_id: i64,
    pub original_request: Box<::afrl::cmasi::automation_request::AutomationRequestT>,
    pub sand_box_request: bool,
    pub planning_states: Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>,
}

impl PartialEq for UniqueAutomationRequest {
    fn eq(&self, _other: &UniqueAutomationRequest) -> bool {
        true
        && &self.request_id == &_other.request_id
        && &self.original_request == &_other.original_request
        && &self.sand_box_request == &_other.sand_box_request
        && &self.planning_states == &_other.planning_states

    }
}

impl LmcpSubscription for UniqueAutomationRequest {
    fn subscription() -> &'static str { "uxas.messages.task.UniqueAutomationRequest" }
}

impl Struct for UniqueAutomationRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 8,
        }
    }
}

impl Lmcp for UniqueAutomationRequest {
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
            let writeb: usize = self.original_request.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.sand_box_request.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.planning_states.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(UniqueAutomationRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == UniqueAutomationRequest::struct_info() {
            let mut out: UniqueAutomationRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.request_id = x;
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
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.sand_box_request = x;
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
        size += self.original_request.size();
        size += self.sand_box_request.size();
        size += self.planning_states.size();

        size
    }
}

pub trait UniqueAutomationRequestT: Debug + Send  {
    fn as_uxas_messages_task_unique_automation_request(&self) -> Option<&UniqueAutomationRequest> { None }
    fn as_mut_uxas_messages_task_unique_automation_request(&mut self) -> Option<&mut UniqueAutomationRequest> { None }
    fn request_id(&self) -> i64;
    fn request_id_mut(&mut self) -> &mut i64;
    fn original_request(&self) -> &Box<::afrl::cmasi::automation_request::AutomationRequestT>;
    fn original_request_mut(&mut self) -> &mut Box<::afrl::cmasi::automation_request::AutomationRequestT>;
    fn sand_box_request(&self) -> bool;
    fn sand_box_request_mut(&mut self) -> &mut bool;
    fn planning_states(&self) -> &Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>;
    fn planning_states_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>;

}

impl Clone for Box<UniqueAutomationRequestT> {
    fn clone(&self) -> Box<UniqueAutomationRequestT> {
        if let Some(x) = UniqueAutomationRequestT::as_uxas_messages_task_unique_automation_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<UniqueAutomationRequestT> {
    fn default() -> Box<UniqueAutomationRequestT> { Box::new(UniqueAutomationRequest::default()) }
}

impl PartialEq for Box<UniqueAutomationRequestT> {
    fn eq(&self, other: &Box<UniqueAutomationRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (UniqueAutomationRequestT::as_uxas_messages_task_unique_automation_request(self.as_ref()),
             UniqueAutomationRequestT::as_uxas_messages_task_unique_automation_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<UniqueAutomationRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = UniqueAutomationRequestT::as_uxas_messages_task_unique_automation_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<UniqueAutomationRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == UniqueAutomationRequest::struct_info() {
            let (x, readb) = UniqueAutomationRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = UniqueAutomationRequestT::as_uxas_messages_task_unique_automation_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl UniqueAutomationRequestT for UniqueAutomationRequest {
    fn as_uxas_messages_task_unique_automation_request(&self) -> Option<&UniqueAutomationRequest> { Some(self) }
    fn as_mut_uxas_messages_task_unique_automation_request(&mut self) -> Option<&mut UniqueAutomationRequest> { Some(self) }
    fn request_id(&self) -> i64 { self.request_id }
    fn request_id_mut(&mut self) -> &mut i64 { &mut self.request_id }
    fn original_request(&self) -> &Box<::afrl::cmasi::automation_request::AutomationRequestT> { &self.original_request }
    fn original_request_mut(&mut self) -> &mut Box<::afrl::cmasi::automation_request::AutomationRequestT> { &mut self.original_request }
    fn sand_box_request(&self) -> bool { self.sand_box_request }
    fn sand_box_request_mut(&mut self) -> &mut bool { &mut self.sand_box_request }
    fn planning_states(&self) -> &Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>> { &self.planning_states }
    fn planning_states_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>> { &mut self.planning_states }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for UniqueAutomationRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> UniqueAutomationRequest {
            UniqueAutomationRequest {
                request_id: Arbitrary::arbitrary(_g),
                original_request: Box::new(::afrl::cmasi::automation_request::AutomationRequest::arbitrary(_g)),
                sand_box_request: Arbitrary::arbitrary(_g),
                planning_states: Vec::<::uxas::messages::task::planning_state::PlanningState>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::task::planning_state::PlanningStateT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: UniqueAutomationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.planning_states.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: UniqueAutomationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.planning_states.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = UniqueAutomationRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
