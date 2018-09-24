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
pub struct RadioTowerState {
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
    pub enabled: bool,
}

impl PartialEq for RadioTowerState {
    fn eq(&self, _other: &RadioTowerState) -> bool {
        true
        && &self.enabled == &_other.enabled

    }
}

impl LmcpSubscription for RadioTowerState {
    fn subscription() -> &'static str { "afrl.impact.RadioTowerState" }
}

impl Struct for RadioTowerState {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 5,
        }
    }
}

impl Lmcp for RadioTowerState {
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
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.enabled.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RadioTowerState, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RadioTowerState::struct_info() {
            let mut out: RadioTowerState = Default::default();
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
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.enabled = x;
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
        size += self.enabled.size();

        size
    }
}

pub trait RadioTowerStateT: Debug + Send + ::afrl::cmasi::entity_state::EntityStateT {
    fn as_afrl_impact_radio_tower_state(&self) -> Option<&RadioTowerState> { None }
    fn as_mut_afrl_impact_radio_tower_state(&mut self) -> Option<&mut RadioTowerState> { None }
    fn enabled(&self) -> bool;
    fn enabled_mut(&mut self) -> &mut bool;

}

impl Clone for Box<RadioTowerStateT> {
    fn clone(&self) -> Box<RadioTowerStateT> {
        if let Some(x) = RadioTowerStateT::as_afrl_impact_radio_tower_state(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RadioTowerStateT> {
    fn default() -> Box<RadioTowerStateT> { Box::new(RadioTowerState::default()) }
}

impl PartialEq for Box<RadioTowerStateT> {
    fn eq(&self, other: &Box<RadioTowerStateT>) -> bool {
        if let (Some(x), Some(y)) =
            (RadioTowerStateT::as_afrl_impact_radio_tower_state(self.as_ref()),
             RadioTowerStateT::as_afrl_impact_radio_tower_state(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RadioTowerStateT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RadioTowerStateT::as_afrl_impact_radio_tower_state(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RadioTowerStateT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RadioTowerState::struct_info() {
            let (x, readb) = RadioTowerState::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RadioTowerStateT::as_afrl_impact_radio_tower_state(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::entity_state::EntityStateT for RadioTowerState {
    fn as_afrl_impact_radio_tower_state(&self) -> Option<&RadioTowerState> { Some(self) }
    fn as_mut_afrl_impact_radio_tower_state(&mut self) -> Option<&mut RadioTowerState> { Some(self) }
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
impl RadioTowerStateT for RadioTowerState {
    fn as_afrl_impact_radio_tower_state(&self) -> Option<&RadioTowerState> { Some(self) }
    fn as_mut_afrl_impact_radio_tower_state(&mut self) -> Option<&mut RadioTowerState> { Some(self) }
    fn enabled(&self) -> bool { self.enabled }
    fn enabled_mut(&mut self) -> &mut bool { &mut self.enabled }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RadioTowerState {
        fn arbitrary<G: Gen>(_g: &mut G) -> RadioTowerState {
            RadioTowerState {
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
                enabled: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RadioTowerState) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_state_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.associated_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RadioTowerState) -> Result<TestResult, Error> {
            use std::u16;
            if x.payload_state_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.associated_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RadioTowerState::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
