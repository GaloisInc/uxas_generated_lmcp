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
pub struct FlightDirectorAction {
    pub associated_task_list: Vec<i64>,
    pub speed: f32,
    pub speed_type: ::afrl::cmasi::speed_type::SpeedType,
    pub heading: f32,
    pub altitude: f32,
    pub altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
    pub climb_rate: f32,
}

impl PartialEq for FlightDirectorAction {
    fn eq(&self, _other: &FlightDirectorAction) -> bool {
        true
        && &self.speed == &_other.speed
        && &self.speed_type == &_other.speed_type
        && &self.heading == &_other.heading
        && &self.altitude == &_other.altitude
        && &self.altitude_type == &_other.altitude_type
        && &self.climb_rate == &_other.climb_rate

    }
}

impl LmcpSubscription for FlightDirectorAction {
    fn subscription() -> &'static str { "afrl.cmasi.FlightDirectorAction" }
}

impl Struct for FlightDirectorAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 54,
        }
    }
}

impl Lmcp for FlightDirectorAction {
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
            let writeb: usize = self.heading.ser(r)?;
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
            let writeb: usize = self.climb_rate.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(FlightDirectorAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == FlightDirectorAction::struct_info() {
            let mut out: FlightDirectorAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
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
                out.heading = x;
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
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.climb_rate = x;
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
        size += self.speed.size();
        size += self.speed_type.size();
        size += self.heading.size();
        size += self.altitude.size();
        size += self.altitude_type.size();
        size += self.climb_rate.size();

        size
    }
}

pub trait FlightDirectorActionT: Debug + Send + ::afrl::cmasi::navigation_action::NavigationActionT {
    fn as_afrl_cmasi_flight_director_action(&self) -> Option<&FlightDirectorAction> { None }
    fn as_mut_afrl_cmasi_flight_director_action(&mut self) -> Option<&mut FlightDirectorAction> { None }
    fn speed(&self) -> f32;
    fn speed_mut(&mut self) -> &mut f32;
    fn speed_type(&self) -> ::afrl::cmasi::speed_type::SpeedType;
    fn speed_type_mut(&mut self) -> &mut ::afrl::cmasi::speed_type::SpeedType;
    fn heading(&self) -> f32;
    fn heading_mut(&mut self) -> &mut f32;
    fn altitude(&self) -> f32;
    fn altitude_mut(&mut self) -> &mut f32;
    fn altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType;
    fn altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType;
    fn climb_rate(&self) -> f32;
    fn climb_rate_mut(&mut self) -> &mut f32;

}

impl Clone for Box<FlightDirectorActionT> {
    fn clone(&self) -> Box<FlightDirectorActionT> {
        if let Some(x) = FlightDirectorActionT::as_afrl_cmasi_flight_director_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<FlightDirectorActionT> {
    fn default() -> Box<FlightDirectorActionT> { Box::new(FlightDirectorAction::default()) }
}

impl PartialEq for Box<FlightDirectorActionT> {
    fn eq(&self, other: &Box<FlightDirectorActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (FlightDirectorActionT::as_afrl_cmasi_flight_director_action(self.as_ref()),
             FlightDirectorActionT::as_afrl_cmasi_flight_director_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<FlightDirectorActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = FlightDirectorActionT::as_afrl_cmasi_flight_director_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<FlightDirectorActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == FlightDirectorAction::struct_info() {
            let (x, readb) = FlightDirectorAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = FlightDirectorActionT::as_afrl_cmasi_flight_director_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for FlightDirectorAction {
    fn as_afrl_cmasi_flight_director_action(&self) -> Option<&FlightDirectorAction> { Some(self) }
    fn as_mut_afrl_cmasi_flight_director_action(&mut self) -> Option<&mut FlightDirectorAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl ::afrl::cmasi::navigation_action::NavigationActionT for FlightDirectorAction {
    fn as_afrl_cmasi_flight_director_action(&self) -> Option<&FlightDirectorAction> { Some(self) }
    fn as_mut_afrl_cmasi_flight_director_action(&mut self) -> Option<&mut FlightDirectorAction> { Some(self) }
}
impl FlightDirectorActionT for FlightDirectorAction {
    fn as_afrl_cmasi_flight_director_action(&self) -> Option<&FlightDirectorAction> { Some(self) }
    fn as_mut_afrl_cmasi_flight_director_action(&mut self) -> Option<&mut FlightDirectorAction> { Some(self) }
    fn speed(&self) -> f32 { self.speed }
    fn speed_mut(&mut self) -> &mut f32 { &mut self.speed }
    fn speed_type(&self) -> ::afrl::cmasi::speed_type::SpeedType { self.speed_type }
    fn speed_type_mut(&mut self) -> &mut ::afrl::cmasi::speed_type::SpeedType { &mut self.speed_type }
    fn heading(&self) -> f32 { self.heading }
    fn heading_mut(&mut self) -> &mut f32 { &mut self.heading }
    fn altitude(&self) -> f32 { self.altitude }
    fn altitude_mut(&mut self) -> &mut f32 { &mut self.altitude }
    fn altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType { self.altitude_type }
    fn altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType { &mut self.altitude_type }
    fn climb_rate(&self) -> f32 { self.climb_rate }
    fn climb_rate_mut(&mut self) -> &mut f32 { &mut self.climb_rate }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for FlightDirectorAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> FlightDirectorAction {
            FlightDirectorAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                speed: Arbitrary::arbitrary(_g),
                speed_type: Arbitrary::arbitrary(_g),
                heading: Arbitrary::arbitrary(_g),
                altitude: Arbitrary::arbitrary(_g),
                altitude_type: Arbitrary::arbitrary(_g),
                climb_rate: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: FlightDirectorAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: FlightDirectorAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = FlightDirectorAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
