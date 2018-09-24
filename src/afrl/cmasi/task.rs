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
pub struct Task {
    pub task_id: i64,
    pub label: Vec<u8>,
    pub eligible_entities: Vec<i64>,
    pub revisit_rate: f32,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub priority: u8,
    pub required: bool,
}

impl PartialEq for Task {
    fn eq(&self, _other: &Task) -> bool {
        true
        && &self.task_id == &_other.task_id
        && &self.label == &_other.label
        && &self.eligible_entities == &_other.eligible_entities
        && &self.revisit_rate == &_other.revisit_rate
        && &self.parameters == &_other.parameters
        && &self.priority == &_other.priority
        && &self.required == &_other.required

    }
}

impl LmcpSubscription for Task {
    fn subscription() -> &'static str { "afrl.cmasi.Task" }
}

impl Struct for Task {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 8,
        }
    }
}

impl Lmcp for Task {
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
            let writeb: usize = self.label.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.eligible_entities.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.revisit_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.parameters.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.priority.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.required.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(Task, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == Task::struct_info() {
            let mut out: Task = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.label = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.eligible_entities = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.revisit_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>, usize) = Lmcp::deser(r)?;
                out.parameters = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u8, usize) = Lmcp::deser(r)?;
                out.priority = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.required = x;
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
        size += self.label.size();
        size += self.eligible_entities.size();
        size += self.revisit_rate.size();
        size += self.parameters.size();
        size += self.priority.size();
        size += self.required.size();

        size
    }
}

pub trait TaskT: Debug + Send  {
    fn as_afrl_cmasi_task(&self) -> Option<&Task> { None }
    fn as_mut_afrl_cmasi_task(&mut self) -> Option<&mut Task> { None }
    fn as_afrl_cmasi_line_search_task(&self) -> Option<&::afrl::cmasi::line_search_task::LineSearchTask> { None }
    fn as_mut_afrl_cmasi_line_search_task(&mut self) -> Option<&mut ::afrl::cmasi::line_search_task::LineSearchTask> { None }
    fn as_afrl_cmasi_point_search_task(&self) -> Option<&::afrl::cmasi::point_search_task::PointSearchTask> { None }
    fn as_mut_afrl_cmasi_point_search_task(&mut self) -> Option<&mut ::afrl::cmasi::point_search_task::PointSearchTask> { None }
    fn as_afrl_cmasi_search_task(&self) -> Option<&::afrl::cmasi::search_task::SearchTask> { None }
    fn as_mut_afrl_cmasi_search_task(&mut self) -> Option<&mut ::afrl::cmasi::search_task::SearchTask> { None }
    fn as_afrl_cmasi_area_search_task(&self) -> Option<&::afrl::cmasi::area_search_task::AreaSearchTask> { None }
    fn as_mut_afrl_cmasi_area_search_task(&mut self) -> Option<&mut ::afrl::cmasi::area_search_task::AreaSearchTask> { None }
    fn as_afrl_impact_multi_vehicle_watch_task(&self) -> Option<&::afrl::impact::multi_vehicle_watch_task::MultiVehicleWatchTask> { None }
    fn as_mut_afrl_impact_multi_vehicle_watch_task(&mut self) -> Option<&mut ::afrl::impact::multi_vehicle_watch_task::MultiVehicleWatchTask> { None }
    fn as_afrl_impact_angled_area_search_task(&self) -> Option<&::afrl::impact::angled_area_search_task::AngledAreaSearchTask> { None }
    fn as_mut_afrl_impact_angled_area_search_task(&mut self) -> Option<&mut ::afrl::impact::angled_area_search_task::AngledAreaSearchTask> { None }
    fn as_afrl_impact_impact_line_search_task(&self) -> Option<&::afrl::impact::impact_line_search_task::ImpactLineSearchTask> { None }
    fn as_mut_afrl_impact_impact_line_search_task(&mut self) -> Option<&mut ::afrl::impact::impact_line_search_task::ImpactLineSearchTask> { None }
    fn as_afrl_impact_cordon_task(&self) -> Option<&::afrl::impact::cordon_task::CordonTask> { None }
    fn as_mut_afrl_impact_cordon_task(&mut self) -> Option<&mut ::afrl::impact::cordon_task::CordonTask> { None }
    fn as_afrl_impact_escort_task(&self) -> Option<&::afrl::impact::escort_task::EscortTask> { None }
    fn as_mut_afrl_impact_escort_task(&mut self) -> Option<&mut ::afrl::impact::escort_task::EscortTask> { None }
    fn as_afrl_impact_comm_relay_task(&self) -> Option<&::afrl::impact::comm_relay_task::CommRelayTask> { None }
    fn as_mut_afrl_impact_comm_relay_task(&mut self) -> Option<&mut ::afrl::impact::comm_relay_task::CommRelayTask> { None }
    fn as_afrl_cmasi_perceive_track_entity_task(&self) -> Option<&::afrl::cmasi::perceive::track_entity_task::TrackEntityTask> { None }
    fn as_mut_afrl_cmasi_perceive_track_entity_task(&mut self) -> Option<&mut ::afrl::cmasi::perceive::track_entity_task::TrackEntityTask> { None }
    fn as_afrl_cmasi_loiter_task(&self) -> Option<&::afrl::cmasi::loiter_task::LoiterTask> { None }
    fn as_mut_afrl_cmasi_loiter_task(&mut self) -> Option<&mut ::afrl::cmasi::loiter_task::LoiterTask> { None }
    fn as_afrl_impact_blockade_task(&self) -> Option<&::afrl::impact::blockade_task::BlockadeTask> { None }
    fn as_mut_afrl_impact_blockade_task(&mut self) -> Option<&mut ::afrl::impact::blockade_task::BlockadeTask> { None }
    fn as_afrl_impact_payload_drop_task(&self) -> Option<&::afrl::impact::payload_drop_task::PayloadDropTask> { None }
    fn as_mut_afrl_impact_payload_drop_task(&mut self) -> Option<&mut ::afrl::impact::payload_drop_task::PayloadDropTask> { None }
    fn as_uxas_messages_task_rendezvous_task(&self) -> Option<&::uxas::messages::task::rendezvous_task::RendezvousTask> { None }
    fn as_mut_uxas_messages_task_rendezvous_task(&mut self) -> Option<&mut ::uxas::messages::task::rendezvous_task::RendezvousTask> { None }
    fn as_afrl_impact_pattern_search_task(&self) -> Option<&::afrl::impact::pattern_search_task::PatternSearchTask> { None }
    fn as_mut_afrl_impact_pattern_search_task(&mut self) -> Option<&mut ::afrl::impact::pattern_search_task::PatternSearchTask> { None }
    fn as_afrl_cmasi_must_fly_task(&self) -> Option<&::afrl::cmasi::must_fly_task::MustFlyTask> { None }
    fn as_mut_afrl_cmasi_must_fly_task(&mut self) -> Option<&mut ::afrl::cmasi::must_fly_task::MustFlyTask> { None }
    fn as_uxas_messages_task_assignment_coordinator_task(&self) -> Option<&::uxas::messages::task::assignment_coordinator_task::AssignmentCoordinatorTask> { None }
    fn as_mut_uxas_messages_task_assignment_coordinator_task(&mut self) -> Option<&mut ::uxas::messages::task::assignment_coordinator_task::AssignmentCoordinatorTask> { None }
    fn as_afrl_impact_impact_point_search_task(&self) -> Option<&::afrl::impact::impact_point_search_task::ImpactPointSearchTask> { None }
    fn as_mut_afrl_impact_impact_point_search_task(&mut self) -> Option<&mut ::afrl::impact::impact_point_search_task::ImpactPointSearchTask> { None }
    fn as_afrl_impact_watch_task(&self) -> Option<&::afrl::impact::watch_task::WatchTask> { None }
    fn as_mut_afrl_impact_watch_task(&mut self) -> Option<&mut ::afrl::impact::watch_task::WatchTask> { None }
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn label(&self) -> &Vec<u8>;
    fn label_mut(&mut self) -> &mut Vec<u8>;
    fn eligible_entities(&self) -> &Vec<i64>;
    fn eligible_entities_mut(&mut self) -> &mut Vec<i64>;
    fn revisit_rate(&self) -> f32;
    fn revisit_rate_mut(&mut self) -> &mut f32;
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn priority(&self) -> u8;
    fn priority_mut(&mut self) -> &mut u8;
    fn required(&self) -> bool;
    fn required_mut(&mut self) -> &mut bool;

}

impl Clone for Box<TaskT> {
    fn clone(&self) -> Box<TaskT> {
        if let Some(x) = TaskT::as_afrl_cmasi_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_cmasi_line_search_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_cmasi_point_search_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_cmasi_search_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_cmasi_area_search_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_multi_vehicle_watch_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_angled_area_search_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_impact_line_search_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_cordon_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_escort_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_comm_relay_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_cmasi_perceive_track_entity_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_cmasi_loiter_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_blockade_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_payload_drop_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_uxas_messages_task_rendezvous_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_pattern_search_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_cmasi_must_fly_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_uxas_messages_task_assignment_coordinator_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_impact_point_search_task(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = TaskT::as_afrl_impact_watch_task(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskT> {
    fn default() -> Box<TaskT> { Box::new(Task::default()) }
}

impl PartialEq for Box<TaskT> {
    fn eq(&self, other: &Box<TaskT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskT::as_afrl_cmasi_task(self.as_ref()),
             TaskT::as_afrl_cmasi_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_cmasi_line_search_task(self.as_ref()),
             TaskT::as_afrl_cmasi_line_search_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_cmasi_point_search_task(self.as_ref()),
             TaskT::as_afrl_cmasi_point_search_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_cmasi_search_task(self.as_ref()),
             TaskT::as_afrl_cmasi_search_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_cmasi_area_search_task(self.as_ref()),
             TaskT::as_afrl_cmasi_area_search_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_multi_vehicle_watch_task(self.as_ref()),
             TaskT::as_afrl_impact_multi_vehicle_watch_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_angled_area_search_task(self.as_ref()),
             TaskT::as_afrl_impact_angled_area_search_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_impact_line_search_task(self.as_ref()),
             TaskT::as_afrl_impact_impact_line_search_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_cordon_task(self.as_ref()),
             TaskT::as_afrl_impact_cordon_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_escort_task(self.as_ref()),
             TaskT::as_afrl_impact_escort_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_comm_relay_task(self.as_ref()),
             TaskT::as_afrl_impact_comm_relay_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_cmasi_perceive_track_entity_task(self.as_ref()),
             TaskT::as_afrl_cmasi_perceive_track_entity_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_cmasi_loiter_task(self.as_ref()),
             TaskT::as_afrl_cmasi_loiter_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_blockade_task(self.as_ref()),
             TaskT::as_afrl_impact_blockade_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_payload_drop_task(self.as_ref()),
             TaskT::as_afrl_impact_payload_drop_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_uxas_messages_task_rendezvous_task(self.as_ref()),
             TaskT::as_uxas_messages_task_rendezvous_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_pattern_search_task(self.as_ref()),
             TaskT::as_afrl_impact_pattern_search_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_cmasi_must_fly_task(self.as_ref()),
             TaskT::as_afrl_cmasi_must_fly_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_uxas_messages_task_assignment_coordinator_task(self.as_ref()),
             TaskT::as_uxas_messages_task_assignment_coordinator_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_impact_point_search_task(self.as_ref()),
             TaskT::as_afrl_impact_impact_point_search_task(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (TaskT::as_afrl_impact_watch_task(self.as_ref()),
             TaskT::as_afrl_impact_watch_task(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskT::as_afrl_cmasi_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_cmasi_line_search_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_cmasi_point_search_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_cmasi_search_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_cmasi_area_search_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_multi_vehicle_watch_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_angled_area_search_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_impact_line_search_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_cordon_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_escort_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_comm_relay_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_cmasi_perceive_track_entity_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_cmasi_loiter_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_blockade_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_payload_drop_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_uxas_messages_task_rendezvous_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_pattern_search_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_cmasi_must_fly_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_uxas_messages_task_assignment_coordinator_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_impact_point_search_task(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = TaskT::as_afrl_impact_watch_task(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == Task::struct_info() {
            let (x, readb) = Task::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::line_search_task::LineSearchTask::struct_info() {
            let (x, readb) = ::afrl::cmasi::line_search_task::LineSearchTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::point_search_task::PointSearchTask::struct_info() {
            let (x, readb) = ::afrl::cmasi::point_search_task::PointSearchTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::search_task::SearchTask::struct_info() {
            let (x, readb) = ::afrl::cmasi::search_task::SearchTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::area_search_task::AreaSearchTask::struct_info() {
            let (x, readb) = ::afrl::cmasi::area_search_task::AreaSearchTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::multi_vehicle_watch_task::MultiVehicleWatchTask::struct_info() {
            let (x, readb) = ::afrl::impact::multi_vehicle_watch_task::MultiVehicleWatchTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::angled_area_search_task::AngledAreaSearchTask::struct_info() {
            let (x, readb) = ::afrl::impact::angled_area_search_task::AngledAreaSearchTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::impact_line_search_task::ImpactLineSearchTask::struct_info() {
            let (x, readb) = ::afrl::impact::impact_line_search_task::ImpactLineSearchTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::cordon_task::CordonTask::struct_info() {
            let (x, readb) = ::afrl::impact::cordon_task::CordonTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::escort_task::EscortTask::struct_info() {
            let (x, readb) = ::afrl::impact::escort_task::EscortTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::comm_relay_task::CommRelayTask::struct_info() {
            let (x, readb) = ::afrl::impact::comm_relay_task::CommRelayTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::perceive::track_entity_task::TrackEntityTask::struct_info() {
            let (x, readb) = ::afrl::cmasi::perceive::track_entity_task::TrackEntityTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::loiter_task::LoiterTask::struct_info() {
            let (x, readb) = ::afrl::cmasi::loiter_task::LoiterTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::blockade_task::BlockadeTask::struct_info() {
            let (x, readb) = ::afrl::impact::blockade_task::BlockadeTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::payload_drop_task::PayloadDropTask::struct_info() {
            let (x, readb) = ::afrl::impact::payload_drop_task::PayloadDropTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::uxas::messages::task::rendezvous_task::RendezvousTask::struct_info() {
            let (x, readb) = ::uxas::messages::task::rendezvous_task::RendezvousTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::pattern_search_task::PatternSearchTask::struct_info() {
            let (x, readb) = ::afrl::impact::pattern_search_task::PatternSearchTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::must_fly_task::MustFlyTask::struct_info() {
            let (x, readb) = ::afrl::cmasi::must_fly_task::MustFlyTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::uxas::messages::task::assignment_coordinator_task::AssignmentCoordinatorTask::struct_info() {
            let (x, readb) = ::uxas::messages::task::assignment_coordinator_task::AssignmentCoordinatorTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::impact_point_search_task::ImpactPointSearchTask::struct_info() {
            let (x, readb) = ::afrl::impact::impact_point_search_task::ImpactPointSearchTask::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::watch_task::WatchTask::struct_info() {
            let (x, readb) = ::afrl::impact::watch_task::WatchTask::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskT::as_afrl_cmasi_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_cmasi_line_search_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_cmasi_point_search_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_cmasi_search_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_cmasi_area_search_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_multi_vehicle_watch_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_angled_area_search_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_impact_line_search_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_cordon_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_escort_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_comm_relay_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_cmasi_perceive_track_entity_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_cmasi_loiter_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_blockade_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_payload_drop_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_uxas_messages_task_rendezvous_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_pattern_search_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_cmasi_must_fly_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_uxas_messages_task_assignment_coordinator_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_impact_point_search_task(self.as_ref()) {
            x.size()
        } else if let Some(x) = TaskT::as_afrl_impact_watch_task(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskT for Task {
    fn as_afrl_cmasi_task(&self) -> Option<&Task> { Some(self) }
    fn as_mut_afrl_cmasi_task(&mut self) -> Option<&mut Task> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn label(&self) -> &Vec<u8> { &self.label }
    fn label_mut(&mut self) -> &mut Vec<u8> { &mut self.label }
    fn eligible_entities(&self) -> &Vec<i64> { &self.eligible_entities }
    fn eligible_entities_mut(&mut self) -> &mut Vec<i64> { &mut self.eligible_entities }
    fn revisit_rate(&self) -> f32 { self.revisit_rate }
    fn revisit_rate_mut(&mut self) -> &mut f32 { &mut self.revisit_rate }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
    fn priority(&self) -> u8 { self.priority }
    fn priority_mut(&mut self) -> &mut u8 { &mut self.priority }
    fn required(&self) -> bool { self.required }
    fn required_mut(&mut self) -> &mut bool { &mut self.required }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for Task {
        fn arbitrary<G: Gen>(_g: &mut G) -> Task {
            Task {
                task_id: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                eligible_entities: Arbitrary::arbitrary(_g),
                revisit_rate: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                priority: Arbitrary::arbitrary(_g),
                required: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: Task) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: Task) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = Task::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
