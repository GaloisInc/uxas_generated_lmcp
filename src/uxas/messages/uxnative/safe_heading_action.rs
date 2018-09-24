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
pub struct SafeHeadingAction {
    pub associated_task_list: Vec<i64>,
    pub vehicle_id: i64,
    pub operating_region: i64,
    pub lead_ahead_distance: f32,
    pub loiter_radius: f32,
    pub desired_heading: f32,
    pub desired_heading_rate: f32,
    pub use_heading_rate: bool,
    pub altitude: f32,
    pub altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
    pub use_altitude: bool,
    pub speed: f32,
    pub use_speed: bool,
}

impl PartialEq for SafeHeadingAction {
    fn eq(&self, _other: &SafeHeadingAction) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id
        && &self.operating_region == &_other.operating_region
        && &self.lead_ahead_distance == &_other.lead_ahead_distance
        && &self.loiter_radius == &_other.loiter_radius
        && &self.desired_heading == &_other.desired_heading
        && &self.desired_heading_rate == &_other.desired_heading_rate
        && &self.use_heading_rate == &_other.use_heading_rate
        && &self.altitude == &_other.altitude
        && &self.altitude_type == &_other.altitude_type
        && &self.use_altitude == &_other.use_altitude
        && &self.speed == &_other.speed
        && &self.use_speed == &_other.use_speed

    }
}

impl LmcpSubscription for SafeHeadingAction {
    fn subscription() -> &'static str { "uxas.messages.uxnative.SafeHeadingAction" }
}

impl Struct for SafeHeadingAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 6,
        }
    }
}

impl Lmcp for SafeHeadingAction {
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
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.operating_region.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.lead_ahead_distance.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.loiter_radius.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.desired_heading.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.desired_heading_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.use_heading_rate.ser(r)?;
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
            let writeb: usize = self.use_altitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.speed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.use_speed.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SafeHeadingAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SafeHeadingAction::struct_info() {
            let mut out: SafeHeadingAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
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
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.operating_region = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.lead_ahead_distance = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.loiter_radius = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.desired_heading = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.desired_heading_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.use_heading_rate = x;
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
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.use_altitude = x;
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
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.use_speed = x;
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
        size += self.vehicle_id.size();
        size += self.operating_region.size();
        size += self.lead_ahead_distance.size();
        size += self.loiter_radius.size();
        size += self.desired_heading.size();
        size += self.desired_heading_rate.size();
        size += self.use_heading_rate.size();
        size += self.altitude.size();
        size += self.altitude_type.size();
        size += self.use_altitude.size();
        size += self.speed.size();
        size += self.use_speed.size();

        size
    }
}

pub trait SafeHeadingActionT: Debug + Send + ::afrl::cmasi::vehicle_action::VehicleActionT {
    fn as_uxas_messages_uxnative_safe_heading_action(&self) -> Option<&SafeHeadingAction> { None }
    fn as_mut_uxas_messages_uxnative_safe_heading_action(&mut self) -> Option<&mut SafeHeadingAction> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn operating_region(&self) -> i64;
    fn operating_region_mut(&mut self) -> &mut i64;
    fn lead_ahead_distance(&self) -> f32;
    fn lead_ahead_distance_mut(&mut self) -> &mut f32;
    fn loiter_radius(&self) -> f32;
    fn loiter_radius_mut(&mut self) -> &mut f32;
    fn desired_heading(&self) -> f32;
    fn desired_heading_mut(&mut self) -> &mut f32;
    fn desired_heading_rate(&self) -> f32;
    fn desired_heading_rate_mut(&mut self) -> &mut f32;
    fn use_heading_rate(&self) -> bool;
    fn use_heading_rate_mut(&mut self) -> &mut bool;
    fn altitude(&self) -> f32;
    fn altitude_mut(&mut self) -> &mut f32;
    fn altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType;
    fn altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType;
    fn use_altitude(&self) -> bool;
    fn use_altitude_mut(&mut self) -> &mut bool;
    fn speed(&self) -> f32;
    fn speed_mut(&mut self) -> &mut f32;
    fn use_speed(&self) -> bool;
    fn use_speed_mut(&mut self) -> &mut bool;

}

impl Clone for Box<SafeHeadingActionT> {
    fn clone(&self) -> Box<SafeHeadingActionT> {
        if let Some(x) = SafeHeadingActionT::as_uxas_messages_uxnative_safe_heading_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SafeHeadingActionT> {
    fn default() -> Box<SafeHeadingActionT> { Box::new(SafeHeadingAction::default()) }
}

impl PartialEq for Box<SafeHeadingActionT> {
    fn eq(&self, other: &Box<SafeHeadingActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (SafeHeadingActionT::as_uxas_messages_uxnative_safe_heading_action(self.as_ref()),
             SafeHeadingActionT::as_uxas_messages_uxnative_safe_heading_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SafeHeadingActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SafeHeadingActionT::as_uxas_messages_uxnative_safe_heading_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SafeHeadingActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SafeHeadingAction::struct_info() {
            let (x, readb) = SafeHeadingAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SafeHeadingActionT::as_uxas_messages_uxnative_safe_heading_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for SafeHeadingAction {
    fn as_uxas_messages_uxnative_safe_heading_action(&self) -> Option<&SafeHeadingAction> { Some(self) }
    fn as_mut_uxas_messages_uxnative_safe_heading_action(&mut self) -> Option<&mut SafeHeadingAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl SafeHeadingActionT for SafeHeadingAction {
    fn as_uxas_messages_uxnative_safe_heading_action(&self) -> Option<&SafeHeadingAction> { Some(self) }
    fn as_mut_uxas_messages_uxnative_safe_heading_action(&mut self) -> Option<&mut SafeHeadingAction> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn operating_region(&self) -> i64 { self.operating_region }
    fn operating_region_mut(&mut self) -> &mut i64 { &mut self.operating_region }
    fn lead_ahead_distance(&self) -> f32 { self.lead_ahead_distance }
    fn lead_ahead_distance_mut(&mut self) -> &mut f32 { &mut self.lead_ahead_distance }
    fn loiter_radius(&self) -> f32 { self.loiter_radius }
    fn loiter_radius_mut(&mut self) -> &mut f32 { &mut self.loiter_radius }
    fn desired_heading(&self) -> f32 { self.desired_heading }
    fn desired_heading_mut(&mut self) -> &mut f32 { &mut self.desired_heading }
    fn desired_heading_rate(&self) -> f32 { self.desired_heading_rate }
    fn desired_heading_rate_mut(&mut self) -> &mut f32 { &mut self.desired_heading_rate }
    fn use_heading_rate(&self) -> bool { self.use_heading_rate }
    fn use_heading_rate_mut(&mut self) -> &mut bool { &mut self.use_heading_rate }
    fn altitude(&self) -> f32 { self.altitude }
    fn altitude_mut(&mut self) -> &mut f32 { &mut self.altitude }
    fn altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType { self.altitude_type }
    fn altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType { &mut self.altitude_type }
    fn use_altitude(&self) -> bool { self.use_altitude }
    fn use_altitude_mut(&mut self) -> &mut bool { &mut self.use_altitude }
    fn speed(&self) -> f32 { self.speed }
    fn speed_mut(&mut self) -> &mut f32 { &mut self.speed }
    fn use_speed(&self) -> bool { self.use_speed }
    fn use_speed_mut(&mut self) -> &mut bool { &mut self.use_speed }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SafeHeadingAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> SafeHeadingAction {
            SafeHeadingAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                operating_region: Arbitrary::arbitrary(_g),
                lead_ahead_distance: Arbitrary::arbitrary(_g),
                loiter_radius: Arbitrary::arbitrary(_g),
                desired_heading: Arbitrary::arbitrary(_g),
                desired_heading_rate: Arbitrary::arbitrary(_g),
                use_heading_rate: Arbitrary::arbitrary(_g),
                altitude: Arbitrary::arbitrary(_g),
                altitude_type: Arbitrary::arbitrary(_g),
                use_altitude: Arbitrary::arbitrary(_g),
                speed: Arbitrary::arbitrary(_g),
                use_speed: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SafeHeadingAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SafeHeadingAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SafeHeadingAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
