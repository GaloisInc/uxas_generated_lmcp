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
pub struct TaskImplementationResponse {
    pub response_id: i64,
    pub corresponding_automation_request_id: i64,
    pub task_id: i64,
    pub option_id: i64,
    pub vehicle_id: i64,
    pub task_waypoints: Vec<Box<::afrl::cmasi::waypoint::WaypointT>>,
    pub final_location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub final_heading: f32,
    pub final_time: i64,
}

impl PartialEq for TaskImplementationResponse {
    fn eq(&self, _other: &TaskImplementationResponse) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.corresponding_automation_request_id == &_other.corresponding_automation_request_id
        && &self.task_id == &_other.task_id
        && &self.option_id == &_other.option_id
        && &self.vehicle_id == &_other.vehicle_id
        && &self.task_waypoints == &_other.task_waypoints
        && &self.final_location == &_other.final_location
        && &self.final_heading == &_other.final_heading
        && &self.final_time == &_other.final_time

    }
}

impl LmcpSubscription for TaskImplementationResponse {
    fn subscription() -> &'static str { "uxas.messages.task.TaskImplementationResponse" }
}

impl Struct for TaskImplementationResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 15,
        }
    }
}

impl Lmcp for TaskImplementationResponse {
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
            let writeb: usize = self.corresponding_automation_request_id.ser(r)?;
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
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_waypoints.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.final_location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.final_heading.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.final_time.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskImplementationResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskImplementationResponse::struct_info() {
            let mut out: TaskImplementationResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
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
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::waypoint::WaypointT>>, usize) = Lmcp::deser(r)?;
                out.task_waypoints = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.final_location = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.final_heading = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.final_time = x;
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
        size += self.corresponding_automation_request_id.size();
        size += self.task_id.size();
        size += self.option_id.size();
        size += self.vehicle_id.size();
        size += self.task_waypoints.size();
        size += self.final_location.size();
        size += self.final_heading.size();
        size += self.final_time.size();

        size
    }
}

pub trait TaskImplementationResponseT: Debug + Send  {
    fn as_uxas_messages_task_task_implementation_response(&self) -> Option<&TaskImplementationResponse> { None }
    fn as_mut_uxas_messages_task_task_implementation_response(&mut self) -> Option<&mut TaskImplementationResponse> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn corresponding_automation_request_id(&self) -> i64;
    fn corresponding_automation_request_id_mut(&mut self) -> &mut i64;
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn option_id(&self) -> i64;
    fn option_id_mut(&mut self) -> &mut i64;
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn task_waypoints(&self) -> &Vec<Box<::afrl::cmasi::waypoint::WaypointT>>;
    fn task_waypoints_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::waypoint::WaypointT>>;
    fn final_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn final_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn final_heading(&self) -> f32;
    fn final_heading_mut(&mut self) -> &mut f32;
    fn final_time(&self) -> i64;
    fn final_time_mut(&mut self) -> &mut i64;

}

impl Clone for Box<TaskImplementationResponseT> {
    fn clone(&self) -> Box<TaskImplementationResponseT> {
        if let Some(x) = TaskImplementationResponseT::as_uxas_messages_task_task_implementation_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskImplementationResponseT> {
    fn default() -> Box<TaskImplementationResponseT> { Box::new(TaskImplementationResponse::default()) }
}

impl PartialEq for Box<TaskImplementationResponseT> {
    fn eq(&self, other: &Box<TaskImplementationResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskImplementationResponseT::as_uxas_messages_task_task_implementation_response(self.as_ref()),
             TaskImplementationResponseT::as_uxas_messages_task_task_implementation_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskImplementationResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskImplementationResponseT::as_uxas_messages_task_task_implementation_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskImplementationResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskImplementationResponse::struct_info() {
            let (x, readb) = TaskImplementationResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskImplementationResponseT::as_uxas_messages_task_task_implementation_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskImplementationResponseT for TaskImplementationResponse {
    fn as_uxas_messages_task_task_implementation_response(&self) -> Option<&TaskImplementationResponse> { Some(self) }
    fn as_mut_uxas_messages_task_task_implementation_response(&mut self) -> Option<&mut TaskImplementationResponse> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn corresponding_automation_request_id(&self) -> i64 { self.corresponding_automation_request_id }
    fn corresponding_automation_request_id_mut(&mut self) -> &mut i64 { &mut self.corresponding_automation_request_id }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn option_id(&self) -> i64 { self.option_id }
    fn option_id_mut(&mut self) -> &mut i64 { &mut self.option_id }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn task_waypoints(&self) -> &Vec<Box<::afrl::cmasi::waypoint::WaypointT>> { &self.task_waypoints }
    fn task_waypoints_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::waypoint::WaypointT>> { &mut self.task_waypoints }
    fn final_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.final_location }
    fn final_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.final_location }
    fn final_heading(&self) -> f32 { self.final_heading }
    fn final_heading_mut(&mut self) -> &mut f32 { &mut self.final_heading }
    fn final_time(&self) -> i64 { self.final_time }
    fn final_time_mut(&mut self) -> &mut i64 { &mut self.final_time }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskImplementationResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskImplementationResponse {
            TaskImplementationResponse {
                response_id: Arbitrary::arbitrary(_g),
                corresponding_automation_request_id: Arbitrary::arbitrary(_g),
                task_id: Arbitrary::arbitrary(_g),
                option_id: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                task_waypoints: Vec::<::afrl::cmasi::waypoint::Waypoint>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::waypoint::WaypointT>).collect(),
                final_location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                final_heading: Arbitrary::arbitrary(_g),
                final_time: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskImplementationResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.task_waypoints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskImplementationResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.task_waypoints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskImplementationResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
