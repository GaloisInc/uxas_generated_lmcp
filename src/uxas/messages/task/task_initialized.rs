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
pub struct TaskInitialized {
    pub task_id: i64,
}

impl PartialEq for TaskInitialized {
    fn eq(&self, _other: &TaskInitialized) -> bool {
        true
        && &self.task_id == &_other.task_id

    }
}

impl LmcpSubscription for TaskInitialized {
    fn subscription() -> &'static str { "uxas.messages.task.TaskInitialized" }
}

impl Struct for TaskInitialized {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 26,
        }
    }
}

impl Lmcp for TaskInitialized {
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

    fn deser(buf: &[u8]) -> Result<(TaskInitialized, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskInitialized::struct_info() {
            let mut out: TaskInitialized = Default::default();
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

pub trait TaskInitializedT: Debug + Send  {
    fn as_uxas_messages_task_task_initialized(&self) -> Option<&TaskInitialized> { None }
    fn as_mut_uxas_messages_task_task_initialized(&mut self) -> Option<&mut TaskInitialized> { None }
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;

}

impl Clone for Box<TaskInitializedT> {
    fn clone(&self) -> Box<TaskInitializedT> {
        if let Some(x) = TaskInitializedT::as_uxas_messages_task_task_initialized(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskInitializedT> {
    fn default() -> Box<TaskInitializedT> { Box::new(TaskInitialized::default()) }
}

impl PartialEq for Box<TaskInitializedT> {
    fn eq(&self, other: &Box<TaskInitializedT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskInitializedT::as_uxas_messages_task_task_initialized(self.as_ref()),
             TaskInitializedT::as_uxas_messages_task_task_initialized(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskInitializedT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskInitializedT::as_uxas_messages_task_task_initialized(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskInitializedT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskInitialized::struct_info() {
            let (x, readb) = TaskInitialized::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskInitializedT::as_uxas_messages_task_task_initialized(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskInitializedT for TaskInitialized {
    fn as_uxas_messages_task_task_initialized(&self) -> Option<&TaskInitialized> { Some(self) }
    fn as_mut_uxas_messages_task_task_initialized(&mut self) -> Option<&mut TaskInitialized> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskInitialized {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskInitialized {
            TaskInitialized {
                task_id: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskInitialized) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskInitialized) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskInitialized::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
