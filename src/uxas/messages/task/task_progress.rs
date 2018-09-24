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
pub struct TaskProgress {
    pub response_id: i64,
    pub task_id: i64,
    pub percent_complete: f32,
    pub entities_engaged: Vec<i64>,
}

impl PartialEq for TaskProgress {
    fn eq(&self, _other: &TaskProgress) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.task_id == &_other.task_id
        && &self.percent_complete == &_other.percent_complete
        && &self.entities_engaged == &_other.entities_engaged

    }
}

impl LmcpSubscription for TaskProgress {
    fn subscription() -> &'static str { "uxas.messages.task.TaskProgress" }
}

impl Struct for TaskProgress {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 24,
        }
    }
}

impl Lmcp for TaskProgress {
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
            let writeb: usize = self.task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.percent_complete.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entities_engaged.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskProgress, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskProgress::struct_info() {
            let mut out: TaskProgress = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.percent_complete = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.entities_engaged = x;
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
        size += self.task_id.size();
        size += self.percent_complete.size();
        size += self.entities_engaged.size();

        size
    }
}

pub trait TaskProgressT: Debug + Send  {
    fn as_uxas_messages_task_task_progress(&self) -> Option<&TaskProgress> { None }
    fn as_mut_uxas_messages_task_task_progress(&mut self) -> Option<&mut TaskProgress> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn percent_complete(&self) -> f32;
    fn percent_complete_mut(&mut self) -> &mut f32;
    fn entities_engaged(&self) -> &Vec<i64>;
    fn entities_engaged_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<TaskProgressT> {
    fn clone(&self) -> Box<TaskProgressT> {
        if let Some(x) = TaskProgressT::as_uxas_messages_task_task_progress(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskProgressT> {
    fn default() -> Box<TaskProgressT> { Box::new(TaskProgress::default()) }
}

impl PartialEq for Box<TaskProgressT> {
    fn eq(&self, other: &Box<TaskProgressT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskProgressT::as_uxas_messages_task_task_progress(self.as_ref()),
             TaskProgressT::as_uxas_messages_task_task_progress(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskProgressT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskProgressT::as_uxas_messages_task_task_progress(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskProgressT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskProgress::struct_info() {
            let (x, readb) = TaskProgress::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskProgressT::as_uxas_messages_task_task_progress(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskProgressT for TaskProgress {
    fn as_uxas_messages_task_task_progress(&self) -> Option<&TaskProgress> { Some(self) }
    fn as_mut_uxas_messages_task_task_progress(&mut self) -> Option<&mut TaskProgress> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn percent_complete(&self) -> f32 { self.percent_complete }
    fn percent_complete_mut(&mut self) -> &mut f32 { &mut self.percent_complete }
    fn entities_engaged(&self) -> &Vec<i64> { &self.entities_engaged }
    fn entities_engaged_mut(&mut self) -> &mut Vec<i64> { &mut self.entities_engaged }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskProgress {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskProgress {
            TaskProgress {
                response_id: Arbitrary::arbitrary(_g),
                task_id: Arbitrary::arbitrary(_g),
                percent_complete: Arbitrary::arbitrary(_g),
                entities_engaged: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskProgress) -> Result<TestResult, Error> {
            use std::u16;
            if x.entities_engaged.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskProgress) -> Result<TestResult, Error> {
            use std::u16;
            if x.entities_engaged.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskProgress::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
