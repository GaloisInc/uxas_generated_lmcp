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
pub struct TaskSummary {
    pub task_id: i64,
    pub performing_vehicles: Vec<Box<::afrl::impact::vehicle_summary::VehicleSummaryT>>,
    pub best_effort: f32,
}

impl PartialEq for TaskSummary {
    fn eq(&self, _other: &TaskSummary) -> bool {
        true
        && &self.task_id == &_other.task_id
        && &self.performing_vehicles == &_other.performing_vehicles
        && &self.best_effort == &_other.best_effort

    }
}

impl LmcpSubscription for TaskSummary {
    fn subscription() -> &'static str { "afrl.impact.TaskSummary" }
}

impl Struct for TaskSummary {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 14,
        }
    }
}

impl Lmcp for TaskSummary {
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
            let writeb: usize = self.performing_vehicles.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.best_effort.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskSummary, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskSummary::struct_info() {
            let mut out: TaskSummary = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::impact::vehicle_summary::VehicleSummaryT>>, usize) = Lmcp::deser(r)?;
                out.performing_vehicles = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.best_effort = x;
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
        size += self.performing_vehicles.size();
        size += self.best_effort.size();

        size
    }
}

pub trait TaskSummaryT: Debug + Send  {
    fn as_afrl_impact_task_summary(&self) -> Option<&TaskSummary> { None }
    fn as_mut_afrl_impact_task_summary(&mut self) -> Option<&mut TaskSummary> { None }
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn performing_vehicles(&self) -> &Vec<Box<::afrl::impact::vehicle_summary::VehicleSummaryT>>;
    fn performing_vehicles_mut(&mut self) -> &mut Vec<Box<::afrl::impact::vehicle_summary::VehicleSummaryT>>;
    fn best_effort(&self) -> f32;
    fn best_effort_mut(&mut self) -> &mut f32;

}

impl Clone for Box<TaskSummaryT> {
    fn clone(&self) -> Box<TaskSummaryT> {
        if let Some(x) = TaskSummaryT::as_afrl_impact_task_summary(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskSummaryT> {
    fn default() -> Box<TaskSummaryT> { Box::new(TaskSummary::default()) }
}

impl PartialEq for Box<TaskSummaryT> {
    fn eq(&self, other: &Box<TaskSummaryT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskSummaryT::as_afrl_impact_task_summary(self.as_ref()),
             TaskSummaryT::as_afrl_impact_task_summary(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskSummaryT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskSummaryT::as_afrl_impact_task_summary(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskSummaryT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskSummary::struct_info() {
            let (x, readb) = TaskSummary::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskSummaryT::as_afrl_impact_task_summary(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskSummaryT for TaskSummary {
    fn as_afrl_impact_task_summary(&self) -> Option<&TaskSummary> { Some(self) }
    fn as_mut_afrl_impact_task_summary(&mut self) -> Option<&mut TaskSummary> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn performing_vehicles(&self) -> &Vec<Box<::afrl::impact::vehicle_summary::VehicleSummaryT>> { &self.performing_vehicles }
    fn performing_vehicles_mut(&mut self) -> &mut Vec<Box<::afrl::impact::vehicle_summary::VehicleSummaryT>> { &mut self.performing_vehicles }
    fn best_effort(&self) -> f32 { self.best_effort }
    fn best_effort_mut(&mut self) -> &mut f32 { &mut self.best_effort }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskSummary {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskSummary {
            TaskSummary {
                task_id: Arbitrary::arbitrary(_g),
                performing_vehicles: Vec::<::afrl::impact::vehicle_summary::VehicleSummary>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::impact::vehicle_summary::VehicleSummaryT>).collect(),
                best_effort: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskSummary) -> Result<TestResult, Error> {
            use std::u16;
            if x.performing_vehicles.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskSummary) -> Result<TestResult, Error> {
            use std::u16;
            if x.performing_vehicles.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskSummary::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
