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
pub struct FlightProfile {
    pub name: Vec<u8>,
    pub airspeed: f32,
    pub pitch_angle: f32,
    pub vertical_speed: f32,
    pub max_bank_angle: f32,
    pub energy_rate: f32,
}

impl PartialEq for FlightProfile {
    fn eq(&self, _other: &FlightProfile) -> bool {
        true
        && &self.name == &_other.name
        && &self.airspeed == &_other.airspeed
        && &self.pitch_angle == &_other.pitch_angle
        && &self.vertical_speed == &_other.vertical_speed
        && &self.max_bank_angle == &_other.max_bank_angle
        && &self.energy_rate == &_other.energy_rate

    }
}

impl LmcpSubscription for FlightProfile {
    fn subscription() -> &'static str { "afrl.cmasi.FlightProfile" }
}

impl Struct for FlightProfile {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 12,
        }
    }
}

impl Lmcp for FlightProfile {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.name.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.airspeed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.pitch_angle.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vertical_speed.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_bank_angle.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.energy_rate.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(FlightProfile, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == FlightProfile::struct_info() {
            let mut out: FlightProfile = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.name = x;
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
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.pitch_angle = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.vertical_speed = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.max_bank_angle = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.energy_rate = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.name.size();
        size += self.airspeed.size();
        size += self.pitch_angle.size();
        size += self.vertical_speed.size();
        size += self.max_bank_angle.size();
        size += self.energy_rate.size();

        size
    }
}

pub trait FlightProfileT: Debug + Send  {
    fn as_afrl_cmasi_flight_profile(&self) -> Option<&FlightProfile> { None }
    fn as_mut_afrl_cmasi_flight_profile(&mut self) -> Option<&mut FlightProfile> { None }
    fn name(&self) -> &Vec<u8>;
    fn name_mut(&mut self) -> &mut Vec<u8>;
    fn airspeed(&self) -> f32;
    fn airspeed_mut(&mut self) -> &mut f32;
    fn pitch_angle(&self) -> f32;
    fn pitch_angle_mut(&mut self) -> &mut f32;
    fn vertical_speed(&self) -> f32;
    fn vertical_speed_mut(&mut self) -> &mut f32;
    fn max_bank_angle(&self) -> f32;
    fn max_bank_angle_mut(&mut self) -> &mut f32;
    fn energy_rate(&self) -> f32;
    fn energy_rate_mut(&mut self) -> &mut f32;

}

impl Clone for Box<FlightProfileT> {
    fn clone(&self) -> Box<FlightProfileT> {
        if let Some(x) = FlightProfileT::as_afrl_cmasi_flight_profile(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<FlightProfileT> {
    fn default() -> Box<FlightProfileT> { Box::new(FlightProfile::default()) }
}

impl PartialEq for Box<FlightProfileT> {
    fn eq(&self, other: &Box<FlightProfileT>) -> bool {
        if let (Some(x), Some(y)) =
            (FlightProfileT::as_afrl_cmasi_flight_profile(self.as_ref()),
             FlightProfileT::as_afrl_cmasi_flight_profile(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<FlightProfileT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = FlightProfileT::as_afrl_cmasi_flight_profile(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<FlightProfileT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == FlightProfile::struct_info() {
            let (x, readb) = FlightProfile::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = FlightProfileT::as_afrl_cmasi_flight_profile(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl FlightProfileT for FlightProfile {
    fn as_afrl_cmasi_flight_profile(&self) -> Option<&FlightProfile> { Some(self) }
    fn as_mut_afrl_cmasi_flight_profile(&mut self) -> Option<&mut FlightProfile> { Some(self) }
    fn name(&self) -> &Vec<u8> { &self.name }
    fn name_mut(&mut self) -> &mut Vec<u8> { &mut self.name }
    fn airspeed(&self) -> f32 { self.airspeed }
    fn airspeed_mut(&mut self) -> &mut f32 { &mut self.airspeed }
    fn pitch_angle(&self) -> f32 { self.pitch_angle }
    fn pitch_angle_mut(&mut self) -> &mut f32 { &mut self.pitch_angle }
    fn vertical_speed(&self) -> f32 { self.vertical_speed }
    fn vertical_speed_mut(&mut self) -> &mut f32 { &mut self.vertical_speed }
    fn max_bank_angle(&self) -> f32 { self.max_bank_angle }
    fn max_bank_angle_mut(&mut self) -> &mut f32 { &mut self.max_bank_angle }
    fn energy_rate(&self) -> f32 { self.energy_rate }
    fn energy_rate_mut(&mut self) -> &mut f32 { &mut self.energy_rate }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for FlightProfile {
        fn arbitrary<G: Gen>(_g: &mut G) -> FlightProfile {
            FlightProfile {
                name: Arbitrary::arbitrary(_g),
                airspeed: Arbitrary::arbitrary(_g),
                pitch_angle: Arbitrary::arbitrary(_g),
                vertical_speed: Arbitrary::arbitrary(_g),
                max_bank_angle: Arbitrary::arbitrary(_g),
                energy_rate: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: FlightProfile) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: FlightProfile) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = FlightProfile::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
