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
pub struct EntityState {
    pub id: i64,
    pub u: f32,
    pub v: f32,
    pub w: f32,
    pub udot: f32,
    pub vdot: f32,
    pub wdot: f32,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub p: f32,
    pub q: f32,
    pub r: f32,
    pub course: f32,
    pub groundspeed: f32,
    pub location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub energy_available: f32,
    pub actual_energy_rate: f32,
    pub payload_state_list: Vec<Box<::afrl::cmasi::payload_state::PayloadStateT>>,
    pub current_waypoint: i64,
    pub current_command: i64,
    pub mode: ::afrl::cmasi::navigation_mode::NavigationMode,
    pub associated_tasks: Vec<i64>,
    pub time: i64,
    pub info: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
}

impl PartialEq for EntityState {
    fn eq(&self, _other: &EntityState) -> bool {
        true
        && &self.id == &_other.id
        && &self.u == &_other.u
        && &self.v == &_other.v
        && &self.w == &_other.w
        && &self.udot == &_other.udot
        && &self.vdot == &_other.vdot
        && &self.wdot == &_other.wdot
        && &self.heading == &_other.heading
        && &self.pitch == &_other.pitch
        && &self.roll == &_other.roll
        && &self.p == &_other.p
        && &self.q == &_other.q
        && &self.r == &_other.r
        && &self.course == &_other.course
        && &self.groundspeed == &_other.groundspeed
        && &self.location == &_other.location
        && &self.energy_available == &_other.energy_available
        && &self.actual_energy_rate == &_other.actual_energy_rate
        && &self.payload_state_list == &_other.payload_state_list
        && &self.current_waypoint == &_other.current_waypoint
        && &self.current_command == &_other.current_command
        && &self.mode == &_other.mode
        && &self.associated_tasks == &_other.associated_tasks
        && &self.time == &_other.time
        && &self.info == &_other.info

    }
}

impl LmcpSubscription for EntityState {
    fn subscription() -> &'static str { "afrl.cmasi.EntityState" }
}

impl Struct for EntityState {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 14,
        }
    }
}

impl Lmcp for EntityState {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.u.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.v.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.w.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.udot.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vdot.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.wdot.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.heading.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.pitch.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.roll.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.p.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.q.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.r.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.course.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.groundspeed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.energy_available.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.actual_energy_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.payload_state_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.current_waypoint.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.current_command.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.mode.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.associated_tasks.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.info.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(EntityState, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == EntityState::struct_info() {
            let mut out: EntityState = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.u = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.v = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.w = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.udot = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.vdot = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.wdot = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.heading = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.pitch = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.roll = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.p = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.q = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.r = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.course = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.groundspeed = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.location = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.energy_available = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.actual_energy_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::payload_state::PayloadStateT>>, usize) = Lmcp::deser(r)?;
                out.payload_state_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.current_waypoint = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.current_command = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::navigation_mode::NavigationMode, usize) = Lmcp::deser(r)?;
                out.mode = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_tasks = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>, usize) = Lmcp::deser(r)?;
                out.info = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.id.size();
        size += self.u.size();
        size += self.v.size();
        size += self.w.size();
        size += self.udot.size();
        size += self.vdot.size();
        size += self.wdot.size();
        size += self.heading.size();
        size += self.pitch.size();
        size += self.roll.size();
        size += self.p.size();
        size += self.q.size();
        size += self.r.size();
        size += self.course.size();
        size += self.groundspeed.size();
        size += self.location.size();
        size += self.energy_available.size();
        size += self.actual_energy_rate.size();
        size += self.payload_state_list.size();
        size += self.current_waypoint.size();
        size += self.current_command.size();
        size += self.mode.size();
        size += self.associated_tasks.size();
        size += self.time.size();
        size += self.info.size();

        size
    }
}

pub trait EntityStateT: Debug + Send  {
    fn as_afrl_cmasi_entity_state(&self) -> Option<&EntityState> { None }
    fn as_mut_afrl_cmasi_entity_state(&mut self) -> Option<&mut EntityState> { None }
    fn as_afrl_vehicles_ground_vehicle_state(&self) -> Option<&::afrl::vehicles::ground_vehicle_state::GroundVehicleState> { None }
    fn as_mut_afrl_vehicles_ground_vehicle_state(&mut self) -> Option<&mut ::afrl::vehicles::ground_vehicle_state::GroundVehicleState> { None }
    fn as_afrl_vehicles_surface_vehicle_state(&self) -> Option<&::afrl::vehicles::surface_vehicle_state::SurfaceVehicleState> { None }
    fn as_mut_afrl_vehicles_surface_vehicle_state(&mut self) -> Option<&mut ::afrl::vehicles::surface_vehicle_state::SurfaceVehicleState> { None }
    fn as_afrl_vehicles_stationary_sensor_state(&self) -> Option<&::afrl::vehicles::stationary_sensor_state::StationarySensorState> { None }
    fn as_mut_afrl_vehicles_stationary_sensor_state(&mut self) -> Option<&mut ::afrl::vehicles::stationary_sensor_state::StationarySensorState> { None }
    fn as_afrl_impact_radio_tower_state(&self) -> Option<&::afrl::impact::radio_tower_state::RadioTowerState> { None }
    fn as_mut_afrl_impact_radio_tower_state(&mut self) -> Option<&mut ::afrl::impact::radio_tower_state::RadioTowerState> { None }
    fn as_afrl_cmasi_air_vehicle_state(&self) -> Option<&::afrl::cmasi::air_vehicle_state::AirVehicleState> { None }
    fn as_mut_afrl_cmasi_air_vehicle_state(&mut self) -> Option<&mut ::afrl::cmasi::air_vehicle_state::AirVehicleState> { None }
    fn id(&self) -> i64;
    fn id_mut(&mut self) -> &mut i64;
    fn u(&self) -> f32;
    fn u_mut(&mut self) -> &mut f32;
    fn v(&self) -> f32;
    fn v_mut(&mut self) -> &mut f32;
    fn w(&self) -> f32;
    fn w_mut(&mut self) -> &mut f32;
    fn udot(&self) -> f32;
    fn udot_mut(&mut self) -> &mut f32;
    fn vdot(&self) -> f32;
    fn vdot_mut(&mut self) -> &mut f32;
    fn wdot(&self) -> f32;
    fn wdot_mut(&mut self) -> &mut f32;
    fn heading(&self) -> f32;
    fn heading_mut(&mut self) -> &mut f32;
    fn pitch(&self) -> f32;
    fn pitch_mut(&mut self) -> &mut f32;
    fn roll(&self) -> f32;
    fn roll_mut(&mut self) -> &mut f32;
    fn p(&self) -> f32;
    fn p_mut(&mut self) -> &mut f32;
    fn q(&self) -> f32;
    fn q_mut(&mut self) -> &mut f32;
    fn r(&self) -> f32;
    fn r_mut(&mut self) -> &mut f32;
    fn course(&self) -> f32;
    fn course_mut(&mut self) -> &mut f32;
    fn groundspeed(&self) -> f32;
    fn groundspeed_mut(&mut self) -> &mut f32;
    fn location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn energy_available(&self) -> f32;
    fn energy_available_mut(&mut self) -> &mut f32;
    fn actual_energy_rate(&self) -> f32;
    fn actual_energy_rate_mut(&mut self) -> &mut f32;
    fn payload_state_list(&self) -> &Vec<Box<::afrl::cmasi::payload_state::PayloadStateT>>;
    fn payload_state_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::payload_state::PayloadStateT>>;
    fn current_waypoint(&self) -> i64;
    fn current_waypoint_mut(&mut self) -> &mut i64;
    fn current_command(&self) -> i64;
    fn current_command_mut(&mut self) -> &mut i64;
    fn mode(&self) -> ::afrl::cmasi::navigation_mode::NavigationMode;
    fn mode_mut(&mut self) -> &mut ::afrl::cmasi::navigation_mode::NavigationMode;
    fn associated_tasks(&self) -> &Vec<i64>;
    fn associated_tasks_mut(&mut self) -> &mut Vec<i64>;
    fn time(&self) -> i64;
    fn time_mut(&mut self) -> &mut i64;
    fn info(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn info_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;

}

impl Clone for Box<EntityStateT> {
    fn clone(&self) -> Box<EntityStateT> {
        if let Some(x) = EntityStateT::as_afrl_cmasi_entity_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityStateT::as_afrl_vehicles_ground_vehicle_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityStateT::as_afrl_vehicles_surface_vehicle_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityStateT::as_afrl_vehicles_stationary_sensor_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityStateT::as_afrl_impact_radio_tower_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = EntityStateT::as_afrl_cmasi_air_vehicle_state(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<EntityStateT> {
    fn default() -> Box<EntityStateT> { Box::new(EntityState::default()) }
}

impl PartialEq for Box<EntityStateT> {
    fn eq(&self, other: &Box<EntityStateT>) -> bool {
        if let (Some(x), Some(y)) =
            (EntityStateT::as_afrl_cmasi_entity_state(self.as_ref()),
             EntityStateT::as_afrl_cmasi_entity_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityStateT::as_afrl_vehicles_ground_vehicle_state(self.as_ref()),
             EntityStateT::as_afrl_vehicles_ground_vehicle_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityStateT::as_afrl_vehicles_surface_vehicle_state(self.as_ref()),
             EntityStateT::as_afrl_vehicles_surface_vehicle_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityStateT::as_afrl_vehicles_stationary_sensor_state(self.as_ref()),
             EntityStateT::as_afrl_vehicles_stationary_sensor_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityStateT::as_afrl_impact_radio_tower_state(self.as_ref()),
             EntityStateT::as_afrl_impact_radio_tower_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (EntityStateT::as_afrl_cmasi_air_vehicle_state(self.as_ref()),
             EntityStateT::as_afrl_cmasi_air_vehicle_state(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<EntityStateT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = EntityStateT::as_afrl_cmasi_entity_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityStateT::as_afrl_vehicles_ground_vehicle_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityStateT::as_afrl_vehicles_surface_vehicle_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityStateT::as_afrl_vehicles_stationary_sensor_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityStateT::as_afrl_impact_radio_tower_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = EntityStateT::as_afrl_cmasi_air_vehicle_state(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<EntityStateT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == EntityState::struct_info() {
            let (x, readb) = EntityState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::vehicles::ground_vehicle_state::GroundVehicleState::struct_info() {
            let (x, readb) = ::afrl::vehicles::ground_vehicle_state::GroundVehicleState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::vehicles::surface_vehicle_state::SurfaceVehicleState::struct_info() {
            let (x, readb) = ::afrl::vehicles::surface_vehicle_state::SurfaceVehicleState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::vehicles::stationary_sensor_state::StationarySensorState::struct_info() {
            let (x, readb) = ::afrl::vehicles::stationary_sensor_state::StationarySensorState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::radio_tower_state::RadioTowerState::struct_info() {
            let (x, readb) = ::afrl::impact::radio_tower_state::RadioTowerState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::air_vehicle_state::AirVehicleState::struct_info() {
            let (x, readb) = ::afrl::cmasi::air_vehicle_state::AirVehicleState::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = EntityStateT::as_afrl_cmasi_entity_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityStateT::as_afrl_vehicles_ground_vehicle_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityStateT::as_afrl_vehicles_surface_vehicle_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityStateT::as_afrl_vehicles_stationary_sensor_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityStateT::as_afrl_impact_radio_tower_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = EntityStateT::as_afrl_cmasi_air_vehicle_state(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl EntityStateT for EntityState {
    fn as_afrl_cmasi_entity_state(&self) -> Option<&EntityState> { Some(self) }
    fn as_mut_afrl_cmasi_entity_state(&mut self) -> Option<&mut EntityState> { Some(self) }
    fn id(&self) -> i64 { self.id }
    fn id_mut(&mut self) -> &mut i64 { &mut self.id }
    fn u(&self) -> f32 { self.u }
    fn u_mut(&mut self) -> &mut f32 { &mut self.u }
    fn v(&self) -> f32 { self.v }
    fn v_mut(&mut self) -> &mut f32 { &mut self.v }
    fn w(&self) -> f32 { self.w }
    fn w_mut(&mut self) -> &mut f32 { &mut self.w }
    fn udot(&self) -> f32 { self.udot }
    fn udot_mut(&mut self) -> &mut f32 { &mut self.udot }
    fn vdot(&self) -> f32 { self.vdot }
    fn vdot_mut(&mut self) -> &mut f32 { &mut self.vdot }
    fn wdot(&self) -> f32 { self.wdot }
    fn wdot_mut(&mut self) -> &mut f32 { &mut self.wdot }
    fn heading(&self) -> f32 { self.heading }
    fn heading_mut(&mut self) -> &mut f32 { &mut self.heading }
    fn pitch(&self) -> f32 { self.pitch }
    fn pitch_mut(&mut self) -> &mut f32 { &mut self.pitch }
    fn roll(&self) -> f32 { self.roll }
    fn roll_mut(&mut self) -> &mut f32 { &mut self.roll }
    fn p(&self) -> f32 { self.p }
    fn p_mut(&mut self) -> &mut f32 { &mut self.p }
    fn q(&self) -> f32 { self.q }
    fn q_mut(&mut self) -> &mut f32 { &mut self.q }
    fn r(&self) -> f32 { self.r }
    fn r_mut(&mut self) -> &mut f32 { &mut self.r }
    fn course(&self) -> f32 { self.course }
    fn course_mut(&mut self) -> &mut f32 { &mut self.course }
    fn groundspeed(&self) -> f32 { self.groundspeed }
    fn groundspeed_mut(&mut self) -> &mut f32 { &mut self.groundspeed }
    fn location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.location }
    fn location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.location }
    fn energy_available(&self) -> f32 { self.energy_available }
    fn energy_available_mut(&mut self) -> &mut f32 { &mut self.energy_available }
    fn actual_energy_rate(&self) -> f32 { self.actual_energy_rate }
    fn actual_energy_rate_mut(&mut self) -> &mut f32 { &mut self.actual_energy_rate }
    fn payload_state_list(&self) -> &Vec<Box<::afrl::cmasi::payload_state::PayloadStateT>> { &self.payload_state_list }
    fn payload_state_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::payload_state::PayloadStateT>> { &mut self.payload_state_list }
    fn current_waypoint(&self) -> i64 { self.current_waypoint }
    fn current_waypoint_mut(&mut self) -> &mut i64 { &mut self.current_waypoint }
    fn current_command(&self) -> i64 { self.current_command }
    fn current_command_mut(&mut self) -> &mut i64 { &mut self.current_command }
    fn mode(&self) -> ::afrl::cmasi::navigation_mode::NavigationMode { self.mode }
    fn mode_mut(&mut self) -> &mut ::afrl::cmasi::navigation_mode::NavigationMode { &mut self.mode }
    fn associated_tasks(&self) -> &Vec<i64> { &self.associated_tasks }
    fn associated_tasks_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_tasks }
    fn time(&self) -> i64 { self.time }
    fn time_mut(&mut self) -> &mut i64 { &mut self.time }
    fn info(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.info }
    fn info_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.info }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for EntityState {
        fn arbitrary<G: Gen>(_g: &mut G) -> EntityState {
            EntityState {
                id: Arbitrary::arbitrary(_g),
                u: Arbitrary::arbitrary(_g),
                v: Arbitrary::arbitrary(_g),
                w: Arbitrary::arbitrary(_g),
                udot: Arbitrary::arbitrary(_g),
                vdot: Arbitrary::arbitrary(_g),
                wdot: Arbitrary::arbitrary(_g),
                heading: Arbitrary::arbitrary(_g),
                pitch: Arbitrary::arbitrary(_g),
                roll: Arbitrary::arbitrary(_g),
                p: Arbitrary::arbitrary(_g),
                q: Arbitrary::arbitrary(_g),
                r: Arbitrary::arbitrary(_g),
                course: Arbitrary::arbitrary(_g),
                groundspeed: Arbitrary::arbitrary(_g),
                location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                energy_available: Arbitrary::arbitrary(_g),
                actual_energy_rate: Arbitrary::arbitrary(_g),
                payload_state_list: Vec::<::afrl::cmasi::payload_state::PayloadState>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::payload_state::PayloadStateT>).collect(),
                current_waypoint: Arbitrary::arbitrary(_g),
                current_command: Arbitrary::arbitrary(_g),
                mode: Arbitrary::arbitrary(_g),
                associated_tasks: Arbitrary::arbitrary(_g),
                time: Arbitrary::arbitrary(_g),
                info: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: EntityState) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_state_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.associated_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: EntityState) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_state_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.associated_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = EntityState::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
