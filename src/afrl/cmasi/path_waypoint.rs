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
pub struct PathWaypoint {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
    pub number: i64,
    pub next_waypoint: i64,
    pub speed: f32,
    pub speed_type: ::afrl::cmasi::speed_type::SpeedType,
    pub climb_rate: f32,
    pub turn_type: ::afrl::cmasi::turn_type::TurnType,
    pub vehicle_action_list: Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>>,
    pub contingency_waypoint_a: i64,
    pub contingency_waypoint_b: i64,
    pub associated_tasks: Vec<i64>,
    pub pause_time: i64,
}

impl PartialEq for PathWaypoint {
    fn eq(&self, _other: &PathWaypoint) -> bool {
        true
        && &self.pause_time == &_other.pause_time

    }
}

impl LmcpSubscription for PathWaypoint {
    fn subscription() -> &'static str { "afrl.cmasi.PathWaypoint" }
}

impl Struct for PathWaypoint {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 57,
        }
    }
}

impl Lmcp for PathWaypoint {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.latitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.longitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.altitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.altitude_type.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.number.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.next_waypoint.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.speed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.speed_type.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.climb_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.turn_type.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_action_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.contingency_waypoint_a.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.contingency_waypoint_b.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.associated_tasks.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.pause_time.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PathWaypoint, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PathWaypoint::struct_info() {
            let mut out: PathWaypoint = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f64, usize) = Lmcp::deser(r)?;
                out.latitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f64, usize) = Lmcp::deser(r)?;
                out.longitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.altitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::altitude_type::AltitudeType, usize) = Lmcp::deser(r)?;
                out.altitude_type = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.number = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.next_waypoint = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.speed = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::speed_type::SpeedType, usize) = Lmcp::deser(r)?;
                out.speed_type = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.climb_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::turn_type::TurnType, usize) = Lmcp::deser(r)?;
                out.turn_type = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>>, usize) = Lmcp::deser(r)?;
                out.vehicle_action_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.contingency_waypoint_a = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.contingency_waypoint_b = x;
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
                out.pause_time = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.latitude.size();
        size += self.longitude.size();
        size += self.altitude.size();
        size += self.altitude_type.size();
        size += self.number.size();
        size += self.next_waypoint.size();
        size += self.speed.size();
        size += self.speed_type.size();
        size += self.climb_rate.size();
        size += self.turn_type.size();
        size += self.vehicle_action_list.size();
        size += self.contingency_waypoint_a.size();
        size += self.contingency_waypoint_b.size();
        size += self.associated_tasks.size();
        size += self.pause_time.size();

        size
    }
}

pub trait PathWaypointT: Debug + Send + ::afrl::cmasi::waypoint::WaypointT {
    fn as_afrl_cmasi_path_waypoint(&self) -> Option<&PathWaypoint> { None }
    fn as_mut_afrl_cmasi_path_waypoint(&mut self) -> Option<&mut PathWaypoint> { None }
    fn pause_time(&self) -> i64;
    fn pause_time_mut(&mut self) -> &mut i64;

}

impl Clone for Box<PathWaypointT> {
    fn clone(&self) -> Box<PathWaypointT> {
        if let Some(x) = PathWaypointT::as_afrl_cmasi_path_waypoint(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PathWaypointT> {
    fn default() -> Box<PathWaypointT> { Box::new(PathWaypoint::default()) }
}

impl PartialEq for Box<PathWaypointT> {
    fn eq(&self, other: &Box<PathWaypointT>) -> bool {
        if let (Some(x), Some(y)) =
            (PathWaypointT::as_afrl_cmasi_path_waypoint(self.as_ref()),
             PathWaypointT::as_afrl_cmasi_path_waypoint(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PathWaypointT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PathWaypointT::as_afrl_cmasi_path_waypoint(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PathWaypointT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PathWaypoint::struct_info() {
            let (x, readb) = PathWaypoint::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PathWaypointT::as_afrl_cmasi_path_waypoint(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::location3d::Location3DT for PathWaypoint {
    fn as_afrl_cmasi_path_waypoint(&self) -> Option<&PathWaypoint> { Some(self) }
    fn as_mut_afrl_cmasi_path_waypoint(&mut self) -> Option<&mut PathWaypoint> { Some(self) }
    fn latitude(&self) -> f64 { self.latitude }
    fn latitude_mut(&mut self) -> &mut f64 { &mut self.latitude }
    fn longitude(&self) -> f64 { self.longitude }
    fn longitude_mut(&mut self) -> &mut f64 { &mut self.longitude }
    fn altitude(&self) -> f32 { self.altitude }
    fn altitude_mut(&mut self) -> &mut f32 { &mut self.altitude }
    fn altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType { self.altitude_type }
    fn altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType { &mut self.altitude_type }
}
impl ::afrl::cmasi::waypoint::WaypointT for PathWaypoint {
    fn as_afrl_cmasi_path_waypoint(&self) -> Option<&PathWaypoint> { Some(self) }
    fn as_mut_afrl_cmasi_path_waypoint(&mut self) -> Option<&mut PathWaypoint> { Some(self) }
    fn number(&self) -> i64 { self.number }
    fn number_mut(&mut self) -> &mut i64 { &mut self.number }
    fn next_waypoint(&self) -> i64 { self.next_waypoint }
    fn next_waypoint_mut(&mut self) -> &mut i64 { &mut self.next_waypoint }
    fn speed(&self) -> f32 { self.speed }
    fn speed_mut(&mut self) -> &mut f32 { &mut self.speed }
    fn speed_type(&self) -> ::afrl::cmasi::speed_type::SpeedType { self.speed_type }
    fn speed_type_mut(&mut self) -> &mut ::afrl::cmasi::speed_type::SpeedType { &mut self.speed_type }
    fn climb_rate(&self) -> f32 { self.climb_rate }
    fn climb_rate_mut(&mut self) -> &mut f32 { &mut self.climb_rate }
    fn turn_type(&self) -> ::afrl::cmasi::turn_type::TurnType { self.turn_type }
    fn turn_type_mut(&mut self) -> &mut ::afrl::cmasi::turn_type::TurnType { &mut self.turn_type }
    fn vehicle_action_list(&self) -> &Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>> { &self.vehicle_action_list }
    fn vehicle_action_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::vehicle_action::VehicleActionT>> { &mut self.vehicle_action_list }
    fn contingency_waypoint_a(&self) -> i64 { self.contingency_waypoint_a }
    fn contingency_waypoint_a_mut(&mut self) -> &mut i64 { &mut self.contingency_waypoint_a }
    fn contingency_waypoint_b(&self) -> i64 { self.contingency_waypoint_b }
    fn contingency_waypoint_b_mut(&mut self) -> &mut i64 { &mut self.contingency_waypoint_b }
    fn associated_tasks(&self) -> &Vec<i64> { &self.associated_tasks }
    fn associated_tasks_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_tasks }
}
impl PathWaypointT for PathWaypoint {
    fn as_afrl_cmasi_path_waypoint(&self) -> Option<&PathWaypoint> { Some(self) }
    fn as_mut_afrl_cmasi_path_waypoint(&mut self) -> Option<&mut PathWaypoint> { Some(self) }
    fn pause_time(&self) -> i64 { self.pause_time }
    fn pause_time_mut(&mut self) -> &mut i64 { &mut self.pause_time }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PathWaypoint {
        fn arbitrary<G: Gen>(_g: &mut G) -> PathWaypoint {
            PathWaypoint {
                latitude: Arbitrary::arbitrary(_g),
                longitude: Arbitrary::arbitrary(_g),
                altitude: Arbitrary::arbitrary(_g),
                altitude_type: Arbitrary::arbitrary(_g),
                number: Arbitrary::arbitrary(_g),
                next_waypoint: Arbitrary::arbitrary(_g),
                speed: Arbitrary::arbitrary(_g),
                speed_type: Arbitrary::arbitrary(_g),
                climb_rate: Arbitrary::arbitrary(_g),
                turn_type: Arbitrary::arbitrary(_g),
                vehicle_action_list: Vec::<::afrl::cmasi::vehicle_action::VehicleAction>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::vehicle_action::VehicleActionT>).collect(),
                contingency_waypoint_a: Arbitrary::arbitrary(_g),
                contingency_waypoint_b: Arbitrary::arbitrary(_g),
                associated_tasks: Arbitrary::arbitrary(_g),
                pause_time: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PathWaypoint) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_action_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.associated_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PathWaypoint) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_action_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.associated_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PathWaypoint::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
