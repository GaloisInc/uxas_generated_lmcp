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
pub struct LoiterAction {
    pub associated_task_list: Vec<i64>,
    pub loiter_type: ::afrl::cmasi::loiter_type::LoiterType,
    pub radius: f32,
    pub axis: f32,
    pub length: f32,
    pub direction: ::afrl::cmasi::loiter_direction::LoiterDirection,
    pub duration: i64,
    pub airspeed: f32,
    pub location: Box<::afrl::cmasi::location3d::Location3DT>,
}

impl PartialEq for LoiterAction {
    fn eq(&self, _other: &LoiterAction) -> bool {
        true
        && &self.loiter_type == &_other.loiter_type
        && &self.radius == &_other.radius
        && &self.axis == &_other.axis
        && &self.length == &_other.length
        && &self.direction == &_other.direction
        && &self.duration == &_other.duration
        && &self.airspeed == &_other.airspeed
        && &self.location == &_other.location

    }
}

impl LmcpSubscription for LoiterAction {
    fn subscription() -> &'static str { "afrl.cmasi.LoiterAction" }
}

impl Struct for LoiterAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 33,
        }
    }
}

impl Lmcp for LoiterAction {
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
            let writeb: usize = self.loiter_type.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.radius.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.axis.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.length.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.direction.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.duration.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.airspeed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.location.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(LoiterAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == LoiterAction::struct_info() {
            let mut out: LoiterAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::loiter_type::LoiterType, usize) = Lmcp::deser(r)?;
                out.loiter_type = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.radius = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.axis = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.length = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::loiter_direction::LoiterDirection, usize) = Lmcp::deser(r)?;
                out.direction = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.duration = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.airspeed = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.location = x;
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
        size += self.loiter_type.size();
        size += self.radius.size();
        size += self.axis.size();
        size += self.length.size();
        size += self.direction.size();
        size += self.duration.size();
        size += self.airspeed.size();
        size += self.location.size();

        size
    }
}

pub trait LoiterActionT: Debug + Send + ::afrl::cmasi::navigation_action::NavigationActionT {
    fn as_afrl_cmasi_loiter_action(&self) -> Option<&LoiterAction> { None }
    fn as_mut_afrl_cmasi_loiter_action(&mut self) -> Option<&mut LoiterAction> { None }
    fn loiter_type(&self) -> ::afrl::cmasi::loiter_type::LoiterType;
    fn loiter_type_mut(&mut self) -> &mut ::afrl::cmasi::loiter_type::LoiterType;
    fn radius(&self) -> f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn axis(&self) -> f32;
    fn axis_mut(&mut self) -> &mut f32;
    fn length(&self) -> f32;
    fn length_mut(&mut self) -> &mut f32;
    fn direction(&self) -> ::afrl::cmasi::loiter_direction::LoiterDirection;
    fn direction_mut(&mut self) -> &mut ::afrl::cmasi::loiter_direction::LoiterDirection;
    fn duration(&self) -> i64;
    fn duration_mut(&mut self) -> &mut i64;
    fn airspeed(&self) -> f32;
    fn airspeed_mut(&mut self) -> &mut f32;
    fn location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;

}

impl Clone for Box<LoiterActionT> {
    fn clone(&self) -> Box<LoiterActionT> {
        if let Some(x) = LoiterActionT::as_afrl_cmasi_loiter_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<LoiterActionT> {
    fn default() -> Box<LoiterActionT> { Box::new(LoiterAction::default()) }
}

impl PartialEq for Box<LoiterActionT> {
    fn eq(&self, other: &Box<LoiterActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (LoiterActionT::as_afrl_cmasi_loiter_action(self.as_ref()),
             LoiterActionT::as_afrl_cmasi_loiter_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<LoiterActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = LoiterActionT::as_afrl_cmasi_loiter_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<LoiterActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == LoiterAction::struct_info() {
            let (x, readb) = LoiterAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = LoiterActionT::as_afrl_cmasi_loiter_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for LoiterAction {
    fn as_afrl_cmasi_loiter_action(&self) -> Option<&LoiterAction> { Some(self) }
    fn as_mut_afrl_cmasi_loiter_action(&mut self) -> Option<&mut LoiterAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl ::afrl::cmasi::navigation_action::NavigationActionT for LoiterAction {
    fn as_afrl_cmasi_loiter_action(&self) -> Option<&LoiterAction> { Some(self) }
    fn as_mut_afrl_cmasi_loiter_action(&mut self) -> Option<&mut LoiterAction> { Some(self) }
}
impl LoiterActionT for LoiterAction {
    fn as_afrl_cmasi_loiter_action(&self) -> Option<&LoiterAction> { Some(self) }
    fn as_mut_afrl_cmasi_loiter_action(&mut self) -> Option<&mut LoiterAction> { Some(self) }
    fn loiter_type(&self) -> ::afrl::cmasi::loiter_type::LoiterType { self.loiter_type }
    fn loiter_type_mut(&mut self) -> &mut ::afrl::cmasi::loiter_type::LoiterType { &mut self.loiter_type }
    fn radius(&self) -> f32 { self.radius }
    fn radius_mut(&mut self) -> &mut f32 { &mut self.radius }
    fn axis(&self) -> f32 { self.axis }
    fn axis_mut(&mut self) -> &mut f32 { &mut self.axis }
    fn length(&self) -> f32 { self.length }
    fn length_mut(&mut self) -> &mut f32 { &mut self.length }
    fn direction(&self) -> ::afrl::cmasi::loiter_direction::LoiterDirection { self.direction }
    fn direction_mut(&mut self) -> &mut ::afrl::cmasi::loiter_direction::LoiterDirection { &mut self.direction }
    fn duration(&self) -> i64 { self.duration }
    fn duration_mut(&mut self) -> &mut i64 { &mut self.duration }
    fn airspeed(&self) -> f32 { self.airspeed }
    fn airspeed_mut(&mut self) -> &mut f32 { &mut self.airspeed }
    fn location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.location }
    fn location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.location }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for LoiterAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> LoiterAction {
            LoiterAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                loiter_type: Arbitrary::arbitrary(_g),
                radius: Arbitrary::arbitrary(_g),
                axis: Arbitrary::arbitrary(_g),
                length: Arbitrary::arbitrary(_g),
                direction: Arbitrary::arbitrary(_g),
                duration: Arbitrary::arbitrary(_g),
                airspeed: Arbitrary::arbitrary(_g),
                location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),

            }
        }
    }

    quickcheck! {
        fn serializes(x: LoiterAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: LoiterAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = LoiterAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
