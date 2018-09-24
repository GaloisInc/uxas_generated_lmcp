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
pub struct TaskAssignment {
    pub task_id: i64,
    pub option_id: i64,
    pub assigned_vehicle: i64,
    pub time_threshold: i64,
    pub time_task_completed: i64,
}

impl PartialEq for TaskAssignment {
    fn eq(&self, _other: &TaskAssignment) -> bool {
        true
        && &self.task_id == &_other.task_id
        && &self.option_id == &_other.option_id
        && &self.assigned_vehicle == &_other.assigned_vehicle
        && &self.time_threshold == &_other.time_threshold
        && &self.time_task_completed == &_other.time_task_completed

    }
}

impl LmcpSubscription for TaskAssignment {
    fn subscription() -> &'static str { "uxas.messages.task.TaskAssignment" }
}

impl Struct for TaskAssignment {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 18,
        }
    }
}

impl Lmcp for TaskAssignment {
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
            let writeb: usize = self.option_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.assigned_vehicle.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_threshold.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_task_completed.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskAssignment, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskAssignment::struct_info() {
            let mut out: TaskAssignment = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.option_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.assigned_vehicle = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time_threshold = x;
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
        size += self.option_id.size();
        size += self.assigned_vehicle.size();
        size += self.time_threshold.size();
        size += self.time_task_completed.size();

        size
    }
}

pub trait TaskAssignmentT: Debug + Send  {
    fn as_uxas_messages_task_task_assignment(&self) -> Option<&TaskAssignment> { None }
    fn as_mut_uxas_messages_task_task_assignment(&mut self) -> Option<&mut TaskAssignment> { None }
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn option_id(&self) -> i64;
    fn option_id_mut(&mut self) -> &mut i64;
    fn assigned_vehicle(&self) -> i64;
    fn assigned_vehicle_mut(&mut self) -> &mut i64;
    fn time_threshold(&self) -> i64;
    fn time_threshold_mut(&mut self) -> &mut i64;
    fn time_task_completed(&self) -> i64;
    fn time_task_completed_mut(&mut self) -> &mut i64;

}

impl Clone for Box<TaskAssignmentT> {
    fn clone(&self) -> Box<TaskAssignmentT> {
        if let Some(x) = TaskAssignmentT::as_uxas_messages_task_task_assignment(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskAssignmentT> {
    fn default() -> Box<TaskAssignmentT> { Box::new(TaskAssignment::default()) }
}

impl PartialEq for Box<TaskAssignmentT> {
    fn eq(&self, other: &Box<TaskAssignmentT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskAssignmentT::as_uxas_messages_task_task_assignment(self.as_ref()),
             TaskAssignmentT::as_uxas_messages_task_task_assignment(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskAssignmentT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskAssignmentT::as_uxas_messages_task_task_assignment(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskAssignmentT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskAssignment::struct_info() {
            let (x, readb) = TaskAssignment::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskAssignmentT::as_uxas_messages_task_task_assignment(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskAssignmentT for TaskAssignment {
    fn as_uxas_messages_task_task_assignment(&self) -> Option<&TaskAssignment> { Some(self) }
    fn as_mut_uxas_messages_task_task_assignment(&mut self) -> Option<&mut TaskAssignment> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn option_id(&self) -> i64 { self.option_id }
    fn option_id_mut(&mut self) -> &mut i64 { &mut self.option_id }
    fn assigned_vehicle(&self) -> i64 { self.assigned_vehicle }
    fn assigned_vehicle_mut(&mut self) -> &mut i64 { &mut self.assigned_vehicle }
    fn time_threshold(&self) -> i64 { self.time_threshold }
    fn time_threshold_mut(&mut self) -> &mut i64 { &mut self.time_threshold }
    fn time_task_completed(&self) -> i64 { self.time_task_completed }
    fn time_task_completed_mut(&mut self) -> &mut i64 { &mut self.time_task_completed }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskAssignment {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskAssignment {
            TaskAssignment {
                task_id: Arbitrary::arbitrary(_g),
                option_id: Arbitrary::arbitrary(_g),
                assigned_vehicle: Arbitrary::arbitrary(_g),
                time_threshold: Arbitrary::arbitrary(_g),
                time_task_completed: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskAssignment) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskAssignment) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskAssignment::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
