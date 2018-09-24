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
pub struct TaskProgressRequest {
    pub request_id: i64,
    pub task_id: i64,
}

impl PartialEq for TaskProgressRequest {
    fn eq(&self, _other: &TaskProgressRequest) -> bool {
        true
        && &self.request_id == &_other.request_id
        && &self.task_id == &_other.task_id

    }
}

impl LmcpSubscription for TaskProgressRequest {
    fn subscription() -> &'static str { "uxas.messages.task.TaskProgressRequest" }
}

impl Struct for TaskProgressRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 25,
        }
    }
}

impl Lmcp for TaskProgressRequest {
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
            let writeb: usize = self.task_id.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskProgressRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskProgressRequest::struct_info() {
            let mut out: TaskProgressRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.request_id = x;
                pos += readb;
            }
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
        size += self.request_id.size();
        size += self.task_id.size();

        size
    }
}

pub trait TaskProgressRequestT: Debug + Send  {
    fn as_uxas_messages_task_task_progress_request(&self) -> Option<&TaskProgressRequest> { None }
    fn as_mut_uxas_messages_task_task_progress_request(&mut self) -> Option<&mut TaskProgressRequest> { None }
    fn request_id(&self) -> i64;
    fn request_id_mut(&mut self) -> &mut i64;
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;

}

impl Clone for Box<TaskProgressRequestT> {
    fn clone(&self) -> Box<TaskProgressRequestT> {
        if let Some(x) = TaskProgressRequestT::as_uxas_messages_task_task_progress_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskProgressRequestT> {
    fn default() -> Box<TaskProgressRequestT> { Box::new(TaskProgressRequest::default()) }
}

impl PartialEq for Box<TaskProgressRequestT> {
    fn eq(&self, other: &Box<TaskProgressRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskProgressRequestT::as_uxas_messages_task_task_progress_request(self.as_ref()),
             TaskProgressRequestT::as_uxas_messages_task_task_progress_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskProgressRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskProgressRequestT::as_uxas_messages_task_task_progress_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskProgressRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskProgressRequest::struct_info() {
            let (x, readb) = TaskProgressRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskProgressRequestT::as_uxas_messages_task_task_progress_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskProgressRequestT for TaskProgressRequest {
    fn as_uxas_messages_task_task_progress_request(&self) -> Option<&TaskProgressRequest> { Some(self) }
    fn as_mut_uxas_messages_task_task_progress_request(&mut self) -> Option<&mut TaskProgressRequest> { Some(self) }
    fn request_id(&self) -> i64 { self.request_id }
    fn request_id_mut(&mut self) -> &mut i64 { &mut self.request_id }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskProgressRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskProgressRequest {
            TaskProgressRequest {
                request_id: Arbitrary::arbitrary(_g),
                task_id: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskProgressRequest) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskProgressRequest) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskProgressRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
