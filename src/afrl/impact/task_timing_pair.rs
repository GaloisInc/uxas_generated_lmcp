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
pub struct TaskTimingPair {
    pub vehicle_id: i64,
    pub initial_task_id: i64,
    pub initial_task_percentage: f32,
    pub destination_task_id: i64,
    pub time_to_go: i64,
}

impl PartialEq for TaskTimingPair {
    fn eq(&self, _other: &TaskTimingPair) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id
        && &self.initial_task_id == &_other.initial_task_id
        && &self.initial_task_percentage == &_other.initial_task_percentage
        && &self.destination_task_id == &_other.destination_task_id
        && &self.time_to_go == &_other.time_to_go

    }
}

impl LmcpSubscription for TaskTimingPair {
    fn subscription() -> &'static str { "afrl.impact.TaskTimingPair" }
}

impl Struct for TaskTimingPair {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 11,
        }
    }
}

impl Lmcp for TaskTimingPair {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.initial_task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.initial_task_percentage.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.destination_task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_to_go.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskTimingPair, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskTimingPair::struct_info() {
            let mut out: TaskTimingPair = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.initial_task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.initial_task_percentage = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.destination_task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time_to_go = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.vehicle_id.size();
        size += self.initial_task_id.size();
        size += self.initial_task_percentage.size();
        size += self.destination_task_id.size();
        size += self.time_to_go.size();

        size
    }
}

pub trait TaskTimingPairT: Debug + Send  {
    fn as_afrl_impact_task_timing_pair(&self) -> Option<&TaskTimingPair> { None }
    fn as_mut_afrl_impact_task_timing_pair(&mut self) -> Option<&mut TaskTimingPair> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn initial_task_id(&self) -> i64;
    fn initial_task_id_mut(&mut self) -> &mut i64;
    fn initial_task_percentage(&self) -> f32;
    fn initial_task_percentage_mut(&mut self) -> &mut f32;
    fn destination_task_id(&self) -> i64;
    fn destination_task_id_mut(&mut self) -> &mut i64;
    fn time_to_go(&self) -> i64;
    fn time_to_go_mut(&mut self) -> &mut i64;

}

impl Clone for Box<TaskTimingPairT> {
    fn clone(&self) -> Box<TaskTimingPairT> {
        if let Some(x) = TaskTimingPairT::as_afrl_impact_task_timing_pair(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskTimingPairT> {
    fn default() -> Box<TaskTimingPairT> { Box::new(TaskTimingPair::default()) }
}

impl PartialEq for Box<TaskTimingPairT> {
    fn eq(&self, other: &Box<TaskTimingPairT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskTimingPairT::as_afrl_impact_task_timing_pair(self.as_ref()),
             TaskTimingPairT::as_afrl_impact_task_timing_pair(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskTimingPairT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskTimingPairT::as_afrl_impact_task_timing_pair(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskTimingPairT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskTimingPair::struct_info() {
            let (x, readb) = TaskTimingPair::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskTimingPairT::as_afrl_impact_task_timing_pair(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskTimingPairT for TaskTimingPair {
    fn as_afrl_impact_task_timing_pair(&self) -> Option<&TaskTimingPair> { Some(self) }
    fn as_mut_afrl_impact_task_timing_pair(&mut self) -> Option<&mut TaskTimingPair> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn initial_task_id(&self) -> i64 { self.initial_task_id }
    fn initial_task_id_mut(&mut self) -> &mut i64 { &mut self.initial_task_id }
    fn initial_task_percentage(&self) -> f32 { self.initial_task_percentage }
    fn initial_task_percentage_mut(&mut self) -> &mut f32 { &mut self.initial_task_percentage }
    fn destination_task_id(&self) -> i64 { self.destination_task_id }
    fn destination_task_id_mut(&mut self) -> &mut i64 { &mut self.destination_task_id }
    fn time_to_go(&self) -> i64 { self.time_to_go }
    fn time_to_go_mut(&mut self) -> &mut i64 { &mut self.time_to_go }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskTimingPair {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskTimingPair {
            TaskTimingPair {
                vehicle_id: Arbitrary::arbitrary(_g),
                initial_task_id: Arbitrary::arbitrary(_g),
                initial_task_percentage: Arbitrary::arbitrary(_g),
                destination_task_id: Arbitrary::arbitrary(_g),
                time_to_go: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskTimingPair) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskTimingPair) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskTimingPair::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
