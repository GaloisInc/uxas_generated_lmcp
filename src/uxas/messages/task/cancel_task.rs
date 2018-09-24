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
pub struct CancelTask {
    pub vehicles: Vec<i64>,
    pub canceled_tasks: Vec<i64>,
}

impl PartialEq for CancelTask {
    fn eq(&self, _other: &CancelTask) -> bool {
        true
        && &self.vehicles == &_other.vehicles
        && &self.canceled_tasks == &_other.canceled_tasks

    }
}

impl LmcpSubscription for CancelTask {
    fn subscription() -> &'static str { "uxas.messages.task.CancelTask" }
}

impl Struct for CancelTask {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 29,
        }
    }
}

impl Lmcp for CancelTask {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicles.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.canceled_tasks.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(CancelTask, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == CancelTask::struct_info() {
            let mut out: CancelTask = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.vehicles = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.canceled_tasks = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.vehicles.size();
        size += self.canceled_tasks.size();

        size
    }
}

pub trait CancelTaskT: Debug + Send  {
    fn as_uxas_messages_task_cancel_task(&self) -> Option<&CancelTask> { None }
    fn as_mut_uxas_messages_task_cancel_task(&mut self) -> Option<&mut CancelTask> { None }
    fn vehicles(&self) -> &Vec<i64>;
    fn vehicles_mut(&mut self) -> &mut Vec<i64>;
    fn canceled_tasks(&self) -> &Vec<i64>;
    fn canceled_tasks_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<CancelTaskT> {
    fn clone(&self) -> Box<CancelTaskT> {
        if let Some(x) = CancelTaskT::as_uxas_messages_task_cancel_task(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<CancelTaskT> {
    fn default() -> Box<CancelTaskT> { Box::new(CancelTask::default()) }
}

impl PartialEq for Box<CancelTaskT> {
    fn eq(&self, other: &Box<CancelTaskT>) -> bool {
        if let (Some(x), Some(y)) =
            (CancelTaskT::as_uxas_messages_task_cancel_task(self.as_ref()),
             CancelTaskT::as_uxas_messages_task_cancel_task(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<CancelTaskT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = CancelTaskT::as_uxas_messages_task_cancel_task(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<CancelTaskT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == CancelTask::struct_info() {
            let (x, readb) = CancelTask::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = CancelTaskT::as_uxas_messages_task_cancel_task(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl CancelTaskT for CancelTask {
    fn as_uxas_messages_task_cancel_task(&self) -> Option<&CancelTask> { Some(self) }
    fn as_mut_uxas_messages_task_cancel_task(&mut self) -> Option<&mut CancelTask> { Some(self) }
    fn vehicles(&self) -> &Vec<i64> { &self.vehicles }
    fn vehicles_mut(&mut self) -> &mut Vec<i64> { &mut self.vehicles }
    fn canceled_tasks(&self) -> &Vec<i64> { &self.canceled_tasks }
    fn canceled_tasks_mut(&mut self) -> &mut Vec<i64> { &mut self.canceled_tasks }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for CancelTask {
        fn arbitrary<G: Gen>(_g: &mut G) -> CancelTask {
            CancelTask {
                vehicles: Arbitrary::arbitrary(_g),
                canceled_tasks: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: CancelTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicles.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.canceled_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: CancelTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicles.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.canceled_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = CancelTask::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
