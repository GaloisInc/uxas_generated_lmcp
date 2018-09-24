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
pub struct TaskAssignmentSummary {
    pub corresponding_automation_request_id: i64,
    pub operating_region: i64,
    pub task_list: Vec<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>>,
}

impl PartialEq for TaskAssignmentSummary {
    fn eq(&self, _other: &TaskAssignmentSummary) -> bool {
        true
        && &self.corresponding_automation_request_id == &_other.corresponding_automation_request_id
        && &self.operating_region == &_other.operating_region
        && &self.task_list == &_other.task_list

    }
}

impl LmcpSubscription for TaskAssignmentSummary {
    fn subscription() -> &'static str { "uxas.messages.task.TaskAssignmentSummary" }
}

impl Struct for TaskAssignmentSummary {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 19,
        }
    }
}

impl Lmcp for TaskAssignmentSummary {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.corresponding_automation_request_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.operating_region.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_list.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskAssignmentSummary, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskAssignmentSummary::struct_info() {
            let mut out: TaskAssignmentSummary = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.corresponding_automation_request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.operating_region = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>>, usize) = Lmcp::deser(r)?;
                out.task_list = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.corresponding_automation_request_id.size();
        size += self.operating_region.size();
        size += self.task_list.size();

        size
    }
}

pub trait TaskAssignmentSummaryT: Debug + Send  {
    fn as_uxas_messages_task_task_assignment_summary(&self) -> Option<&TaskAssignmentSummary> { None }
    fn as_mut_uxas_messages_task_task_assignment_summary(&mut self) -> Option<&mut TaskAssignmentSummary> { None }
    fn corresponding_automation_request_id(&self) -> i64;
    fn corresponding_automation_request_id_mut(&mut self) -> &mut i64;
    fn operating_region(&self) -> i64;
    fn operating_region_mut(&mut self) -> &mut i64;
    fn task_list(&self) -> &Vec<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>>;
    fn task_list_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>>;

}

impl Clone for Box<TaskAssignmentSummaryT> {
    fn clone(&self) -> Box<TaskAssignmentSummaryT> {
        if let Some(x) = TaskAssignmentSummaryT::as_uxas_messages_task_task_assignment_summary(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskAssignmentSummaryT> {
    fn default() -> Box<TaskAssignmentSummaryT> { Box::new(TaskAssignmentSummary::default()) }
}

impl PartialEq for Box<TaskAssignmentSummaryT> {
    fn eq(&self, other: &Box<TaskAssignmentSummaryT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskAssignmentSummaryT::as_uxas_messages_task_task_assignment_summary(self.as_ref()),
             TaskAssignmentSummaryT::as_uxas_messages_task_task_assignment_summary(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskAssignmentSummaryT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskAssignmentSummaryT::as_uxas_messages_task_task_assignment_summary(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskAssignmentSummaryT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskAssignmentSummary::struct_info() {
            let (x, readb) = TaskAssignmentSummary::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskAssignmentSummaryT::as_uxas_messages_task_task_assignment_summary(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskAssignmentSummaryT for TaskAssignmentSummary {
    fn as_uxas_messages_task_task_assignment_summary(&self) -> Option<&TaskAssignmentSummary> { Some(self) }
    fn as_mut_uxas_messages_task_task_assignment_summary(&mut self) -> Option<&mut TaskAssignmentSummary> { Some(self) }
    fn corresponding_automation_request_id(&self) -> i64 { self.corresponding_automation_request_id }
    fn corresponding_automation_request_id_mut(&mut self) -> &mut i64 { &mut self.corresponding_automation_request_id }
    fn operating_region(&self) -> i64 { self.operating_region }
    fn operating_region_mut(&mut self) -> &mut i64 { &mut self.operating_region }
    fn task_list(&self) -> &Vec<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>> { &self.task_list }
    fn task_list_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>> { &mut self.task_list }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskAssignmentSummary {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskAssignmentSummary {
            TaskAssignmentSummary {
                corresponding_automation_request_id: Arbitrary::arbitrary(_g),
                operating_region: Arbitrary::arbitrary(_g),
                task_list: Vec::<::uxas::messages::task::task_assignment::TaskAssignment>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::task::task_assignment::TaskAssignmentT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskAssignmentSummary) -> Result<TestResult, Error> {
            use std::u16;
            if x.task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskAssignmentSummary) -> Result<TestResult, Error> {
            use std::u16;
            if x.task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskAssignmentSummary::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
