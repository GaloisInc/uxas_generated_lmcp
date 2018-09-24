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
pub struct TaskImplementationRequest {
    pub request_id: i64,
    pub corresponding_automation_request_id: i64,
    pub starting_waypoint_id: i64,
    pub vehicle_id: i64,
    pub start_position: Box<::afrl::cmasi::location3d::Location3DT>,
    pub start_heading: f32,
    pub start_time: i64,
    pub region_id: i64,
    pub task_id: i64,
    pub option_id: i64,
    pub time_threshold: i64,
    pub neighbor_locations: Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>,
}

impl PartialEq for TaskImplementationRequest {
    fn eq(&self, _other: &TaskImplementationRequest) -> bool {
        true
        && &self.request_id == &_other.request_id
        && &self.corresponding_automation_request_id == &_other.corresponding_automation_request_id
        && &self.starting_waypoint_id == &_other.starting_waypoint_id
        && &self.vehicle_id == &_other.vehicle_id
        && &self.start_position == &_other.start_position
        && &self.start_heading == &_other.start_heading
        && &self.start_time == &_other.start_time
        && &self.region_id == &_other.region_id
        && &self.task_id == &_other.task_id
        && &self.option_id == &_other.option_id
        && &self.time_threshold == &_other.time_threshold
        && &self.neighbor_locations == &_other.neighbor_locations

    }
}

impl LmcpSubscription for TaskImplementationRequest {
    fn subscription() -> &'static str { "uxas.messages.task.TaskImplementationRequest" }
}

impl Struct for TaskImplementationRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 14,
        }
    }
}

impl Lmcp for TaskImplementationRequest {
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
            let writeb: usize = self.corresponding_automation_request_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.starting_waypoint_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_position.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_heading.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_time.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.region_id.ser(r)?;
            pos += writeb;
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
            let writeb: usize = self.time_threshold.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.neighbor_locations.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskImplementationRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskImplementationRequest::struct_info() {
            let mut out: TaskImplementationRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.corresponding_automation_request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.starting_waypoint_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.start_position = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.start_heading = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.start_time = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.region_id = x;
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
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.option_id = x;
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
                let (x, readb): (Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>, usize) = Lmcp::deser(r)?;
                out.neighbor_locations = x;
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
        size += self.corresponding_automation_request_id.size();
        size += self.starting_waypoint_id.size();
        size += self.vehicle_id.size();
        size += self.start_position.size();
        size += self.start_heading.size();
        size += self.start_time.size();
        size += self.region_id.size();
        size += self.task_id.size();
        size += self.option_id.size();
        size += self.time_threshold.size();
        size += self.neighbor_locations.size();

        size
    }
}

pub trait TaskImplementationRequestT: Debug + Send  {
    fn as_uxas_messages_task_task_implementation_request(&self) -> Option<&TaskImplementationRequest> { None }
    fn as_mut_uxas_messages_task_task_implementation_request(&mut self) -> Option<&mut TaskImplementationRequest> { None }
    fn request_id(&self) -> i64;
    fn request_id_mut(&mut self) -> &mut i64;
    fn corresponding_automation_request_id(&self) -> i64;
    fn corresponding_automation_request_id_mut(&mut self) -> &mut i64;
    fn starting_waypoint_id(&self) -> i64;
    fn starting_waypoint_id_mut(&mut self) -> &mut i64;
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn start_position(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn start_position_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn start_heading(&self) -> f32;
    fn start_heading_mut(&mut self) -> &mut f32;
    fn start_time(&self) -> i64;
    fn start_time_mut(&mut self) -> &mut i64;
    fn region_id(&self) -> i64;
    fn region_id_mut(&mut self) -> &mut i64;
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn option_id(&self) -> i64;
    fn option_id_mut(&mut self) -> &mut i64;
    fn time_threshold(&self) -> i64;
    fn time_threshold_mut(&mut self) -> &mut i64;
    fn neighbor_locations(&self) -> &Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>;
    fn neighbor_locations_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>;

}

impl Clone for Box<TaskImplementationRequestT> {
    fn clone(&self) -> Box<TaskImplementationRequestT> {
        if let Some(x) = TaskImplementationRequestT::as_uxas_messages_task_task_implementation_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskImplementationRequestT> {
    fn default() -> Box<TaskImplementationRequestT> { Box::new(TaskImplementationRequest::default()) }
}

impl PartialEq for Box<TaskImplementationRequestT> {
    fn eq(&self, other: &Box<TaskImplementationRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskImplementationRequestT::as_uxas_messages_task_task_implementation_request(self.as_ref()),
             TaskImplementationRequestT::as_uxas_messages_task_task_implementation_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskImplementationRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskImplementationRequestT::as_uxas_messages_task_task_implementation_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskImplementationRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskImplementationRequest::struct_info() {
            let (x, readb) = TaskImplementationRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskImplementationRequestT::as_uxas_messages_task_task_implementation_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskImplementationRequestT for TaskImplementationRequest {
    fn as_uxas_messages_task_task_implementation_request(&self) -> Option<&TaskImplementationRequest> { Some(self) }
    fn as_mut_uxas_messages_task_task_implementation_request(&mut self) -> Option<&mut TaskImplementationRequest> { Some(self) }
    fn request_id(&self) -> i64 { self.request_id }
    fn request_id_mut(&mut self) -> &mut i64 { &mut self.request_id }
    fn corresponding_automation_request_id(&self) -> i64 { self.corresponding_automation_request_id }
    fn corresponding_automation_request_id_mut(&mut self) -> &mut i64 { &mut self.corresponding_automation_request_id }
    fn starting_waypoint_id(&self) -> i64 { self.starting_waypoint_id }
    fn starting_waypoint_id_mut(&mut self) -> &mut i64 { &mut self.starting_waypoint_id }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn start_position(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.start_position }
    fn start_position_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.start_position }
    fn start_heading(&self) -> f32 { self.start_heading }
    fn start_heading_mut(&mut self) -> &mut f32 { &mut self.start_heading }
    fn start_time(&self) -> i64 { self.start_time }
    fn start_time_mut(&mut self) -> &mut i64 { &mut self.start_time }
    fn region_id(&self) -> i64 { self.region_id }
    fn region_id_mut(&mut self) -> &mut i64 { &mut self.region_id }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn option_id(&self) -> i64 { self.option_id }
    fn option_id_mut(&mut self) -> &mut i64 { &mut self.option_id }
    fn time_threshold(&self) -> i64 { self.time_threshold }
    fn time_threshold_mut(&mut self) -> &mut i64 { &mut self.time_threshold }
    fn neighbor_locations(&self) -> &Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>> { &self.neighbor_locations }
    fn neighbor_locations_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>> { &mut self.neighbor_locations }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskImplementationRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskImplementationRequest {
            TaskImplementationRequest {
                request_id: Arbitrary::arbitrary(_g),
                corresponding_automation_request_id: Arbitrary::arbitrary(_g),
                starting_waypoint_id: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                start_position: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                start_heading: Arbitrary::arbitrary(_g),
                start_time: Arbitrary::arbitrary(_g),
                region_id: Arbitrary::arbitrary(_g),
                task_id: Arbitrary::arbitrary(_g),
                option_id: Arbitrary::arbitrary(_g),
                time_threshold: Arbitrary::arbitrary(_g),
                neighbor_locations: Vec::<::uxas::messages::task::planning_state::PlanningState>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::task::planning_state::PlanningStateT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskImplementationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.neighbor_locations.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskImplementationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.neighbor_locations.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskImplementationRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
