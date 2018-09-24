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
pub struct TaskResume {
    pub task_id: i64,
    pub restart_completely: bool,
    pub re_assign: Option<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>>,
}

impl PartialEq for TaskResume {
    fn eq(&self, _other: &TaskResume) -> bool {
        true
        && &self.task_id == &_other.task_id
        && &self.restart_completely == &_other.restart_completely
        && &self.re_assign == &_other.re_assign

    }
}

impl LmcpSubscription for TaskResume {
    fn subscription() -> &'static str { "uxas.messages.task.TaskResume" }
}

impl Struct for TaskResume {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 23,
        }
    }
}

impl Lmcp for TaskResume {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.restart_completely.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.re_assign.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskResume, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskResume::struct_info() {
            let mut out: TaskResume = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.restart_completely = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Option<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>>, usize) = Lmcp::deser(r)?;
                out.re_assign = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.task_id.size();
        size += self.restart_completely.size();
        size += self.re_assign.size();

        size
    }
}

pub trait TaskResumeT: Debug + Send  {
    fn as_uxas_messages_task_task_resume(&self) -> Option<&TaskResume> { None }
    fn as_mut_uxas_messages_task_task_resume(&mut self) -> Option<&mut TaskResume> { None }
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn restart_completely(&self) -> bool;
    fn restart_completely_mut(&mut self) -> &mut bool;
    fn re_assign(&self) -> &Option<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>>;
    fn re_assign_mut(&mut self) -> &mut Option<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>>;

}

impl Clone for Box<TaskResumeT> {
    fn clone(&self) -> Box<TaskResumeT> {
        if let Some(x) = TaskResumeT::as_uxas_messages_task_task_resume(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskResumeT> {
    fn default() -> Box<TaskResumeT> { Box::new(TaskResume::default()) }
}

impl PartialEq for Box<TaskResumeT> {
    fn eq(&self, other: &Box<TaskResumeT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskResumeT::as_uxas_messages_task_task_resume(self.as_ref()),
             TaskResumeT::as_uxas_messages_task_task_resume(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskResumeT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskResumeT::as_uxas_messages_task_task_resume(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskResumeT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskResume::struct_info() {
            let (x, readb) = TaskResume::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskResumeT::as_uxas_messages_task_task_resume(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskResumeT for TaskResume {
    fn as_uxas_messages_task_task_resume(&self) -> Option<&TaskResume> { Some(self) }
    fn as_mut_uxas_messages_task_task_resume(&mut self) -> Option<&mut TaskResume> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn restart_completely(&self) -> bool { self.restart_completely }
    fn restart_completely_mut(&mut self) -> &mut bool { &mut self.restart_completely }
    fn re_assign(&self) -> &Option<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>> { &self.re_assign }
    fn re_assign_mut(&mut self) -> &mut Option<Box<::uxas::messages::task::task_assignment::TaskAssignmentT>> { &mut self.re_assign }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskResume {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskResume {
            TaskResume {
                task_id: Arbitrary::arbitrary(_g),
                restart_completely: Arbitrary::arbitrary(_g),
                re_assign: {
                    if _g.gen() {
                        Some(Box::new(::uxas::messages::task::task_assignment::TaskAssignment::arbitrary(_g)))
                    } else {
                        None
                    }
                },

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskResume) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskResume) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskResume::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
