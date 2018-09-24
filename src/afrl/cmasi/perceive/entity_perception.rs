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
pub struct EntityPerception {
    pub perceived_entity_id: u32,
    pub perceiver_id: u32,
    pub perceiver_payloads: Vec<u32>,
    pub velocity: Vec<f32>,
    pub velocity_error: Vec<f32>,
    pub velocity_valid: bool,
    pub attitude: Vec<f32>,
    pub attitude_error: Vec<f32>,
    pub attitude_valid: bool,
    pub location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub location_error: Vec<f32>,
    pub time_last_seen: i64,
}

impl PartialEq for EntityPerception {
    fn eq(&self, _other: &EntityPerception) -> bool {
        true
        && &self.perceived_entity_id == &_other.perceived_entity_id
        && &self.perceiver_id == &_other.perceiver_id
        && &self.perceiver_payloads == &_other.perceiver_payloads
        && &self.velocity == &_other.velocity
        && &self.velocity_error == &_other.velocity_error
        && &self.velocity_valid == &_other.velocity_valid
        && &self.attitude == &_other.attitude
        && &self.attitude_error == &_other.attitude_error
        && &self.attitude_valid == &_other.attitude_valid
        && &self.location == &_other.location
        && &self.location_error == &_other.location_error
        && &self.time_last_seen == &_other.time_last_seen

    }
}

impl LmcpSubscription for EntityPerception {
    fn subscription() -> &'static str { "afrl.cmasi.perceive.EntityPerception" }
}

impl Struct for EntityPerception {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5784119745305990725u64,
            version: 1,
            struct_ty: 1,
        }
    }
}

impl Lmcp for EntityPerception {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.perceived_entity_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.perceiver_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.perceiver_payloads.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.velocity.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.velocity_error.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.velocity_valid.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.attitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.attitude_error.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.attitude_valid.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.location_error.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_last_seen.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(EntityPerception, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == EntityPerception::struct_info() {
            let mut out: EntityPerception = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u32, usize) = Lmcp::deser(r)?;
                out.perceived_entity_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u32, usize) = Lmcp::deser(r)?;
                out.perceiver_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u32>, usize) = Lmcp::deser(r)?;
                out.perceiver_payloads = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.velocity = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.velocity_error = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.velocity_valid = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.attitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.attitude_error = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.attitude_valid = x;
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
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.location_error = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time_last_seen = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.perceived_entity_id.size();
        size += self.perceiver_id.size();
        size += self.perceiver_payloads.size();
        size += self.velocity.size();
        size += self.velocity_error.size();
        size += self.velocity_valid.size();
        size += self.attitude.size();
        size += self.attitude_error.size();
        size += self.attitude_valid.size();
        size += self.location.size();
        size += self.location_error.size();
        size += self.time_last_seen.size();

        size
    }
}

pub trait EntityPerceptionT: Debug + Send  {
    fn as_afrl_cmasi_perceive_entity_perception(&self) -> Option<&EntityPerception> { None }
    fn as_mut_afrl_cmasi_perceive_entity_perception(&mut self) -> Option<&mut EntityPerception> { None }
    fn perceived_entity_id(&self) -> u32;
    fn perceived_entity_id_mut(&mut self) -> &mut u32;
    fn perceiver_id(&self) -> u32;
    fn perceiver_id_mut(&mut self) -> &mut u32;
    fn perceiver_payloads(&self) -> &Vec<u32>;
    fn perceiver_payloads_mut(&mut self) -> &mut Vec<u32>;
    fn velocity(&self) -> &Vec<f32>;
    fn velocity_mut(&mut self) -> &mut Vec<f32>;
    fn velocity_error(&self) -> &Vec<f32>;
    fn velocity_error_mut(&mut self) -> &mut Vec<f32>;
    fn velocity_valid(&self) -> bool;
    fn velocity_valid_mut(&mut self) -> &mut bool;
    fn attitude(&self) -> &Vec<f32>;
    fn attitude_mut(&mut self) -> &mut Vec<f32>;
    fn attitude_error(&self) -> &Vec<f32>;
    fn attitude_error_mut(&mut self) -> &mut Vec<f32>;
    fn attitude_valid(&self) -> bool;
    fn attitude_valid_mut(&mut self) -> &mut bool;
    fn location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn location_error(&self) -> &Vec<f32>;
    fn location_error_mut(&mut self) -> &mut Vec<f32>;
    fn time_last_seen(&self) -> i64;
    fn time_last_seen_mut(&mut self) -> &mut i64;

}

impl Clone for Box<EntityPerceptionT> {
    fn clone(&self) -> Box<EntityPerceptionT> {
        if let Some(x) = EntityPerceptionT::as_afrl_cmasi_perceive_entity_perception(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<EntityPerceptionT> {
    fn default() -> Box<EntityPerceptionT> { Box::new(EntityPerception::default()) }
}

impl PartialEq for Box<EntityPerceptionT> {
    fn eq(&self, other: &Box<EntityPerceptionT>) -> bool {
        if let (Some(x), Some(y)) =
            (EntityPerceptionT::as_afrl_cmasi_perceive_entity_perception(self.as_ref()),
             EntityPerceptionT::as_afrl_cmasi_perceive_entity_perception(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<EntityPerceptionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = EntityPerceptionT::as_afrl_cmasi_perceive_entity_perception(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<EntityPerceptionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == EntityPerception::struct_info() {
            let (x, readb) = EntityPerception::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = EntityPerceptionT::as_afrl_cmasi_perceive_entity_perception(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl EntityPerceptionT for EntityPerception {
    fn as_afrl_cmasi_perceive_entity_perception(&self) -> Option<&EntityPerception> { Some(self) }
    fn as_mut_afrl_cmasi_perceive_entity_perception(&mut self) -> Option<&mut EntityPerception> { Some(self) }
    fn perceived_entity_id(&self) -> u32 { self.perceived_entity_id }
    fn perceived_entity_id_mut(&mut self) -> &mut u32 { &mut self.perceived_entity_id }
    fn perceiver_id(&self) -> u32 { self.perceiver_id }
    fn perceiver_id_mut(&mut self) -> &mut u32 { &mut self.perceiver_id }
    fn perceiver_payloads(&self) -> &Vec<u32> { &self.perceiver_payloads }
    fn perceiver_payloads_mut(&mut self) -> &mut Vec<u32> { &mut self.perceiver_payloads }
    fn velocity(&self) -> &Vec<f32> { &self.velocity }
    fn velocity_mut(&mut self) -> &mut Vec<f32> { &mut self.velocity }
    fn velocity_error(&self) -> &Vec<f32> { &self.velocity_error }
    fn velocity_error_mut(&mut self) -> &mut Vec<f32> { &mut self.velocity_error }
    fn velocity_valid(&self) -> bool { self.velocity_valid }
    fn velocity_valid_mut(&mut self) -> &mut bool { &mut self.velocity_valid }
    fn attitude(&self) -> &Vec<f32> { &self.attitude }
    fn attitude_mut(&mut self) -> &mut Vec<f32> { &mut self.attitude }
    fn attitude_error(&self) -> &Vec<f32> { &self.attitude_error }
    fn attitude_error_mut(&mut self) -> &mut Vec<f32> { &mut self.attitude_error }
    fn attitude_valid(&self) -> bool { self.attitude_valid }
    fn attitude_valid_mut(&mut self) -> &mut bool { &mut self.attitude_valid }
    fn location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.location }
    fn location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.location }
    fn location_error(&self) -> &Vec<f32> { &self.location_error }
    fn location_error_mut(&mut self) -> &mut Vec<f32> { &mut self.location_error }
    fn time_last_seen(&self) -> i64 { self.time_last_seen }
    fn time_last_seen_mut(&mut self) -> &mut i64 { &mut self.time_last_seen }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for EntityPerception {
        fn arbitrary<G: Gen>(_g: &mut G) -> EntityPerception {
            EntityPerception {
                perceived_entity_id: Arbitrary::arbitrary(_g),
                perceiver_id: Arbitrary::arbitrary(_g),
                perceiver_payloads: Arbitrary::arbitrary(_g),
                velocity: Arbitrary::arbitrary(_g),
                velocity_error: Arbitrary::arbitrary(_g),
                velocity_valid: Arbitrary::arbitrary(_g),
                attitude: Arbitrary::arbitrary(_g),
                attitude_error: Arbitrary::arbitrary(_g),
                attitude_valid: Arbitrary::arbitrary(_g),
                location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                location_error: Arbitrary::arbitrary(_g),
                time_last_seen: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: EntityPerception) -> Result<TestResult, Error> {
            use std::u16;
            if x.perceiver_payloads.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.velocity.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.velocity_error.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.attitude.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.attitude_error.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.location_error.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: EntityPerception) -> Result<TestResult, Error> {
            use std::u16;
            if x.perceiver_payloads.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.velocity.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.velocity_error.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.attitude.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.attitude_error.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.location_error.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = EntityPerception::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
