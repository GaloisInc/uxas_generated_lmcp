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
pub struct VehicleAction {
    pub associated_task_list: Vec<i64>,
}

impl PartialEq for VehicleAction {
    fn eq(&self, _other: &VehicleAction) -> bool {
        true
        && &self.associated_task_list == &_other.associated_task_list

    }
}

impl LmcpSubscription for VehicleAction {
    fn subscription() -> &'static str { "afrl.cmasi.VehicleAction" }
}

impl Struct for VehicleAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 7,
        }
    }
}

impl Lmcp for VehicleAction {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.associated_task_list.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(VehicleAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == VehicleAction::struct_info() {
            let mut out: VehicleAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.associated_task_list.size();

        size
    }
}

pub trait VehicleActionT: Debug + Send  {
    fn as_afrl_cmasi_vehicle_action(&self) -> Option<&VehicleAction> { None }
    fn as_mut_afrl_cmasi_vehicle_action(&mut self) -> Option<&mut VehicleAction> { None }
    fn as_afrl_cmasi_camera_action(&self) -> Option<&::afrl::cmasi::camera_action::CameraAction> { None }
    fn as_mut_afrl_cmasi_camera_action(&mut self) -> Option<&mut ::afrl::cmasi::camera_action::CameraAction> { None }
    fn as_afrl_cmasi_loiter_action(&self) -> Option<&::afrl::cmasi::loiter_action::LoiterAction> { None }
    fn as_mut_afrl_cmasi_loiter_action(&mut self) -> Option<&mut ::afrl::cmasi::loiter_action::LoiterAction> { None }
    fn as_afrl_cmasi_go_to_waypoint_action(&self) -> Option<&::afrl::cmasi::go_to_waypoint_action::GoToWaypointAction> { None }
    fn as_mut_afrl_cmasi_go_to_waypoint_action(&mut self) -> Option<&mut ::afrl::cmasi::go_to_waypoint_action::GoToWaypointAction> { None }
    fn as_afrl_impact_deploy_impact_payload(&self) -> Option<&::afrl::impact::deploy_impact_payload::DeployImpactPayload> { None }
    fn as_mut_afrl_impact_deploy_impact_payload(&mut self) -> Option<&mut ::afrl::impact::deploy_impact_payload::DeployImpactPayload> { None }
    fn as_afrl_cmasi_gimbal_stare_action(&self) -> Option<&::afrl::cmasi::gimbal_stare_action::GimbalStareAction> { None }
    fn as_mut_afrl_cmasi_gimbal_stare_action(&mut self) -> Option<&mut ::afrl::cmasi::gimbal_stare_action::GimbalStareAction> { None }
    fn as_afrl_cmasi_stop_movement_action(&self) -> Option<&::afrl::cmasi::stop_movement_action::StopMovementAction> { None }
    fn as_mut_afrl_cmasi_stop_movement_action(&mut self) -> Option<&mut ::afrl::cmasi::stop_movement_action::StopMovementAction> { None }
    fn as_afrl_cmasi_flight_director_action(&self) -> Option<&::afrl::cmasi::flight_director_action::FlightDirectorAction> { None }
    fn as_mut_afrl_cmasi_flight_director_action(&mut self) -> Option<&mut ::afrl::cmasi::flight_director_action::FlightDirectorAction> { None }
    fn as_afrl_cmasi_payload_action(&self) -> Option<&::afrl::cmasi::payload_action::PayloadAction> { None }
    fn as_mut_afrl_cmasi_payload_action(&mut self) -> Option<&mut ::afrl::cmasi::payload_action::PayloadAction> { None }
    fn as_uxas_messages_uxnative_speed_override_action(&self) -> Option<&::uxas::messages::uxnative::speed_override_action::SpeedOverrideAction> { None }
    fn as_mut_uxas_messages_uxnative_speed_override_action(&mut self) -> Option<&mut ::uxas::messages::uxnative::speed_override_action::SpeedOverrideAction> { None }
    fn as_afrl_cmasi_video_stream_action(&self) -> Option<&::afrl::cmasi::video_stream_action::VideoStreamAction> { None }
    fn as_mut_afrl_cmasi_video_stream_action(&mut self) -> Option<&mut ::afrl::cmasi::video_stream_action::VideoStreamAction> { None }
    fn as_afrl_cmasi_gimbal_scan_action(&self) -> Option<&::afrl::cmasi::gimbal_scan_action::GimbalScanAction> { None }
    fn as_mut_afrl_cmasi_gimbal_scan_action(&mut self) -> Option<&mut ::afrl::cmasi::gimbal_scan_action::GimbalScanAction> { None }
    fn as_uxas_messages_uxnative_safe_heading_action(&self) -> Option<&::uxas::messages::uxnative::safe_heading_action::SafeHeadingAction> { None }
    fn as_mut_uxas_messages_uxnative_safe_heading_action(&mut self) -> Option<&mut ::uxas::messages::uxnative::safe_heading_action::SafeHeadingAction> { None }
    fn as_afrl_cmasi_gimbal_angle_action(&self) -> Option<&::afrl::cmasi::gimbal_angle_action::GimbalAngleAction> { None }
    fn as_mut_afrl_cmasi_gimbal_angle_action(&mut self) -> Option<&mut ::afrl::cmasi::gimbal_angle_action::GimbalAngleAction> { None }
    fn as_afrl_cmasi_navigation_action(&self) -> Option<&::afrl::cmasi::navigation_action::NavigationAction> { None }
    fn as_mut_afrl_cmasi_navigation_action(&mut self) -> Option<&mut ::afrl::cmasi::navigation_action::NavigationAction> { None }
    fn as_afrl_cmasi_perceive_track_entity_action(&self) -> Option<&::afrl::cmasi::perceive::track_entity_action::TrackEntityAction> { None }
    fn as_mut_afrl_cmasi_perceive_track_entity_action(&mut self) -> Option<&mut ::afrl::cmasi::perceive::track_entity_action::TrackEntityAction> { None }
    fn associated_task_list(&self) -> &Vec<i64>;
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<VehicleActionT> {
    fn clone(&self) -> Box<VehicleActionT> {
        if let Some(x) = VehicleActionT::as_afrl_cmasi_vehicle_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_camera_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_loiter_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_impact_deploy_impact_payload(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_stop_movement_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_flight_director_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_payload_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_uxas_messages_uxnative_speed_override_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_video_stream_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_uxas_messages_uxnative_safe_heading_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_navigation_action(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_perceive_track_entity_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<VehicleActionT> {
    fn default() -> Box<VehicleActionT> { Box::new(VehicleAction::default()) }
}

impl PartialEq for Box<VehicleActionT> {
    fn eq(&self, other: &Box<VehicleActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_vehicle_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_vehicle_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_camera_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_camera_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_loiter_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_loiter_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_go_to_waypoint_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_impact_deploy_impact_payload(self.as_ref()),
             VehicleActionT::as_afrl_impact_deploy_impact_payload(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_gimbal_stare_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_stop_movement_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_stop_movement_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_flight_director_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_flight_director_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_payload_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_payload_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_uxas_messages_uxnative_speed_override_action(self.as_ref()),
             VehicleActionT::as_uxas_messages_uxnative_speed_override_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_video_stream_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_video_stream_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_gimbal_scan_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_uxas_messages_uxnative_safe_heading_action(self.as_ref()),
             VehicleActionT::as_uxas_messages_uxnative_safe_heading_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_gimbal_angle_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_navigation_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_navigation_action(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (VehicleActionT::as_afrl_cmasi_perceive_track_entity_action(self.as_ref()),
             VehicleActionT::as_afrl_cmasi_perceive_track_entity_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<VehicleActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = VehicleActionT::as_afrl_cmasi_vehicle_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_camera_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_loiter_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_impact_deploy_impact_payload(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_stop_movement_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_flight_director_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_payload_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_uxas_messages_uxnative_speed_override_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_video_stream_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_uxas_messages_uxnative_safe_heading_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_navigation_action(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_perceive_track_entity_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<VehicleActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == VehicleAction::struct_info() {
            let (x, readb) = VehicleAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::camera_action::CameraAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::camera_action::CameraAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::loiter_action::LoiterAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::loiter_action::LoiterAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::go_to_waypoint_action::GoToWaypointAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::go_to_waypoint_action::GoToWaypointAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::deploy_impact_payload::DeployImpactPayload::struct_info() {
            let (x, readb) = ::afrl::impact::deploy_impact_payload::DeployImpactPayload::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::gimbal_stare_action::GimbalStareAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::gimbal_stare_action::GimbalStareAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::stop_movement_action::StopMovementAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::stop_movement_action::StopMovementAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::flight_director_action::FlightDirectorAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::flight_director_action::FlightDirectorAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::payload_action::PayloadAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::payload_action::PayloadAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::uxas::messages::uxnative::speed_override_action::SpeedOverrideAction::struct_info() {
            let (x, readb) = ::uxas::messages::uxnative::speed_override_action::SpeedOverrideAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::video_stream_action::VideoStreamAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::video_stream_action::VideoStreamAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::gimbal_scan_action::GimbalScanAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::gimbal_scan_action::GimbalScanAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::uxas::messages::uxnative::safe_heading_action::SafeHeadingAction::struct_info() {
            let (x, readb) = ::uxas::messages::uxnative::safe_heading_action::SafeHeadingAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::gimbal_angle_action::GimbalAngleAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::gimbal_angle_action::GimbalAngleAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::navigation_action::NavigationAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::navigation_action::NavigationAction::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::perceive::track_entity_action::TrackEntityAction::struct_info() {
            let (x, readb) = ::afrl::cmasi::perceive::track_entity_action::TrackEntityAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = VehicleActionT::as_afrl_cmasi_vehicle_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_camera_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_loiter_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_go_to_waypoint_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_impact_deploy_impact_payload(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_gimbal_stare_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_stop_movement_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_flight_director_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_payload_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_uxas_messages_uxnative_speed_override_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_video_stream_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_uxas_messages_uxnative_safe_heading_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_gimbal_angle_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_navigation_action(self.as_ref()) {
            x.size()
        } else if let Some(x) = VehicleActionT::as_afrl_cmasi_perceive_track_entity_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl VehicleActionT for VehicleAction {
    fn as_afrl_cmasi_vehicle_action(&self) -> Option<&VehicleAction> { Some(self) }
    fn as_mut_afrl_cmasi_vehicle_action(&mut self) -> Option<&mut VehicleAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for VehicleAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> VehicleAction {
            VehicleAction {
                associated_task_list: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: VehicleAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: VehicleAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = VehicleAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
