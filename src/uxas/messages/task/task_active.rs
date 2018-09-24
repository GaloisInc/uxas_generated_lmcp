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
pub struct TaskActive {
    pub task_id: i64,
    pub entity_id: i64,
    pub time_task_activated: i64,
}

impl PartialEq for TaskActive {
    fn eq(&self, _other: &TaskActive) -> bool {
        true
        && &self.task_id == &_other.task_id
        && &self.entity_id == &_other.entity_id
        && &self.time_task_activated == &_other.time_task_activated

    }
}

impl LmcpSubscription for TaskActive {
    fn subscription() -> &'static str { "uxas.messages.task.TaskActive" }
}

impl Struct for TaskActive {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 27,
        }
    }
}

impl Lmcp for TaskActive {
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
            let writeb: usize = self.entity_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_task_activated.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskActive, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskActive::struct_info() {
            let mut out: TaskActive = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.entity_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time_task_activated = x;
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
        size += self.entity_id.size();
        size += self.time_task_activated.size();

        size
    }
}

pub trait TaskActiveT: Debug + Send  {
    fn as_uxas_messages_task_task_active(&self) -> Option<&TaskActive> { None }
    fn as_mut_uxas_messages_task_task_active(&mut self) -> Option<&mut TaskActive> { None }
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn entity_id(&self) -> i64;
    fn entity_id_mut(&mut self) -> &mut i64;
    fn time_task_activated(&self) -> i64;
    fn time_task_activated_mut(&mut self) -> &mut i64;

}

impl Clone for Box<TaskActiveT> {
    fn clone(&self) -> Box<TaskActiveT> {
        if let Some(x) = TaskActiveT::as_uxas_messages_task_task_active(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskActiveT> {
    fn default() -> Box<TaskActiveT> { Box::new(TaskActive::default()) }
}

impl PartialEq for Box<TaskActiveT> {
    fn eq(&self, other: &Box<TaskActiveT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskActiveT::as_uxas_messages_task_task_active(self.as_ref()),
             TaskActiveT::as_uxas_messages_task_task_active(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskActiveT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskActiveT::as_uxas_messages_task_task_active(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskActiveT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskActive::struct_info() {
            let (x, readb) = TaskActive::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskActiveT::as_uxas_messages_task_task_active(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskActiveT for TaskActive {
    fn as_uxas_messages_task_task_active(&self) -> Option<&TaskActive> { Some(self) }
    fn as_mut_uxas_messages_task_task_active(&mut self) -> Option<&mut TaskActive> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn entity_id(&self) -> i64 { self.entity_id }
    fn entity_id_mut(&mut self) -> &mut i64 { &mut self.entity_id }
    fn time_task_activated(&self) -> i64 { self.time_task_activated }
    fn time_task_activated_mut(&mut self) -> &mut i64 { &mut self.time_task_activated }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskActive {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskActive {
            TaskActive {
                task_id: Arbitrary::arbitrary(_g),
                entity_id: Arbitrary::arbitrary(_g),
                time_task_activated: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskActive) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskActive) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskActive::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
