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
pub struct TaskPause {
    pub task_id: i64,
}

impl PartialEq for TaskPause {
    fn eq(&self, _other: &TaskPause) -> bool {
        true
        && &self.task_id == &_other.task_id

    }
}

impl LmcpSubscription for TaskPause {
    fn subscription() -> &'static str { "uxas.messages.task.TaskPause" }
}

impl Struct for TaskPause {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 22,
        }
    }
}

impl Lmcp for TaskPause {
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

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskPause, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskPause::struct_info() {
            let mut out: TaskPause = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
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

        size
    }
}

pub trait TaskPauseT: Debug + Send  {
    fn as_uxas_messages_task_task_pause(&self) -> Option<&TaskPause> { None }
    fn as_mut_uxas_messages_task_task_pause(&mut self) -> Option<&mut TaskPause> { None }
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;

}

impl Clone for Box<TaskPauseT> {
    fn clone(&self) -> Box<TaskPauseT> {
        if let Some(x) = TaskPauseT::as_uxas_messages_task_task_pause(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskPauseT> {
    fn default() -> Box<TaskPauseT> { Box::new(TaskPause::default()) }
}

impl PartialEq for Box<TaskPauseT> {
    fn eq(&self, other: &Box<TaskPauseT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskPauseT::as_uxas_messages_task_task_pause(self.as_ref()),
             TaskPauseT::as_uxas_messages_task_task_pause(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskPauseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskPauseT::as_uxas_messages_task_task_pause(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskPauseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskPause::struct_info() {
            let (x, readb) = TaskPause::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskPauseT::as_uxas_messages_task_task_pause(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskPauseT for TaskPause {
    fn as_uxas_messages_task_task_pause(&self) -> Option<&TaskPause> { Some(self) }
    fn as_mut_uxas_messages_task_task_pause(&mut self) -> Option<&mut TaskPause> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskPause {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskPause {
            TaskPause {
                task_id: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskPause) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskPause) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskPause::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
