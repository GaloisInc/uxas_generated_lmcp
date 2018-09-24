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

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct SpeedAltPair {
    pub vehicle_id: i64,
    pub task_id: i64,
    pub speed: f32,
    pub altitude: f32,
    pub altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
}

impl PartialEq for SpeedAltPair {
    fn eq(&self, _other: &SpeedAltPair) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id
        && &self.task_id == &_other.task_id
        && &self.speed == &_other.speed
        && &self.altitude == &_other.altitude
        && &self.altitude_type == &_other.altitude_type

    }
}

impl LmcpSubscription for SpeedAltPair {
    fn subscription() -> &'static str { "afrl.impact.SpeedAltPair" }
}

impl Struct for SpeedAltPair {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 16,
        }
    }
}

impl Lmcp for SpeedAltPair {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.speed.ser(r)?;
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

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SpeedAltPair, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SpeedAltPair::struct_info() {
            let mut out: SpeedAltPair = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
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
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.speed = x;
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

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.vehicle_id.size();
        size += self.task_id.size();
        size += self.speed.size();
        size += self.altitude.size();
        size += self.altitude_type.size();

        size
    }
}

pub trait SpeedAltPairT: Debug + Send  {
    fn as_afrl_impact_speed_alt_pair(&self) -> Option<&SpeedAltPair> { None }
    fn as_mut_afrl_impact_speed_alt_pair(&mut self) -> Option<&mut SpeedAltPair> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn speed(&self) -> f32;
    fn speed_mut(&mut self) -> &mut f32;
    fn altitude(&self) -> f32;
    fn altitude_mut(&mut self) -> &mut f32;
    fn altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType;
    fn altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType;

}

impl Clone for Box<SpeedAltPairT> {
    fn clone(&self) -> Box<SpeedAltPairT> {
        if let Some(x) = SpeedAltPairT::as_afrl_impact_speed_alt_pair(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SpeedAltPairT> {
    fn default() -> Box<SpeedAltPairT> { Box::new(SpeedAltPair::default()) }
}

impl PartialEq for Box<SpeedAltPairT> {
    fn eq(&self, other: &Box<SpeedAltPairT>) -> bool {
        if let (Some(x), Some(y)) =
            (SpeedAltPairT::as_afrl_impact_speed_alt_pair(self.as_ref()),
             SpeedAltPairT::as_afrl_impact_speed_alt_pair(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SpeedAltPairT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SpeedAltPairT::as_afrl_impact_speed_alt_pair(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SpeedAltPairT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SpeedAltPair::struct_info() {
            let (x, readb) = SpeedAltPair::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SpeedAltPairT::as_afrl_impact_speed_alt_pair(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl SpeedAltPairT for SpeedAltPair {
    fn as_afrl_impact_speed_alt_pair(&self) -> Option<&SpeedAltPair> { Some(self) }
    fn as_mut_afrl_impact_speed_alt_pair(&mut self) -> Option<&mut SpeedAltPair> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn speed(&self) -> f32 { self.speed }
    fn speed_mut(&mut self) -> &mut f32 { &mut self.speed }
    fn altitude(&self) -> f32 { self.altitude }
    fn altitude_mut(&mut self) -> &mut f32 { &mut self.altitude }
    fn altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType { self.altitude_type }
    fn altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType { &mut self.altitude_type }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SpeedAltPair {
        fn arbitrary<G: Gen>(_g: &mut G) -> SpeedAltPair {
            SpeedAltPair {
                vehicle_id: Arbitrary::arbitrary(_g),
                task_id: Arbitrary::arbitrary(_g),
                speed: Arbitrary::arbitrary(_g),
                altitude: Arbitrary::arbitrary(_g),
                altitude_type: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SpeedAltPair) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SpeedAltPair) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SpeedAltPair::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
