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
pub struct TaskComplete {
    pub task_id: i64,
    pub entities_involved: Vec<i64>,
    pub time_task_completed: i64,
}

impl PartialEq for TaskComplete {
    fn eq(&self, _other: &TaskComplete) -> bool {
        true
        && &self.task_id == &_other.task_id
        && &self.entities_involved == &_other.entities_involved
        && &self.time_task_completed == &_other.time_task_completed

    }
}

impl LmcpSubscription for TaskComplete {
    fn subscription() -> &'static str { "uxas.messages.task.TaskComplete" }
}

impl Struct for TaskComplete {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 28,
        }
    }
}

impl Lmcp for TaskComplete {
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
            let writeb: usize = self.entities_involved.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_task_completed.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskComplete, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskComplete::struct_info() {
            let mut out: TaskComplete = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.entities_involved = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time_task_completed = x;
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
        size += self.entities_involved.size();
        size += self.time_task_completed.size();

        size
    }
}

pub trait TaskCompleteT: Debug + Send  {
    fn as_uxas_messages_task_task_complete(&self) -> Option<&TaskComplete> { None }
    fn as_mut_uxas_messages_task_task_complete(&mut self) -> Option<&mut TaskComplete> { None }
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn entities_involved(&self) -> &Vec<i64>;
    fn entities_involved_mut(&mut self) -> &mut Vec<i64>;
    fn time_task_completed(&self) -> i64;
    fn time_task_completed_mut(&mut self) -> &mut i64;

}

impl Clone for Box<TaskCompleteT> {
    fn clone(&self) -> Box<TaskCompleteT> {
        if let Some(x) = TaskCompleteT::as_uxas_messages_task_task_complete(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskCompleteT> {
    fn default() -> Box<TaskCompleteT> { Box::new(TaskComplete::default()) }
}

impl PartialEq for Box<TaskCompleteT> {
    fn eq(&self, other: &Box<TaskCompleteT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskCompleteT::as_uxas_messages_task_task_complete(self.as_ref()),
             TaskCompleteT::as_uxas_messages_task_task_complete(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskCompleteT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskCompleteT::as_uxas_messages_task_task_complete(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskCompleteT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskComplete::struct_info() {
            let (x, readb) = TaskComplete::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskCompleteT::as_uxas_messages_task_task_complete(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskCompleteT for TaskComplete {
    fn as_uxas_messages_task_task_complete(&self) -> Option<&TaskComplete> { Some(self) }
    fn as_mut_uxas_messages_task_task_complete(&mut self) -> Option<&mut TaskComplete> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn entities_involved(&self) -> &Vec<i64> { &self.entities_involved }
    fn entities_involved_mut(&mut self) -> &mut Vec<i64> { &mut self.entities_involved }
    fn time_task_completed(&self) -> i64 { self.time_task_completed }
    fn time_task_completed_mut(&mut self) -> &mut i64 { &mut self.time_task_completed }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskComplete {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskComplete {
            TaskComplete {
                task_id: Arbitrary::arbitrary(_g),
                entities_involved: Arbitrary::arbitrary(_g),
                time_task_completed: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskComplete) -> Result<TestResult, Error> {
            use std::u16;
            if x.entities_involved.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskComplete) -> Result<TestResult, Error> {
            use std::u16;
            if x.entities_involved.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskComplete::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
