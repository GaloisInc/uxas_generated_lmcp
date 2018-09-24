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
pub struct PowerPlantState {
    pub payload_id: i64,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub active_power_plant: ::afrl::impact::power_plant::PowerPlant,
}

impl PartialEq for PowerPlantState {
    fn eq(&self, _other: &PowerPlantState) -> bool {
        true
        && &self.active_power_plant == &_other.active_power_plant

    }
}

impl LmcpSubscription for PowerPlantState {
    fn subscription() -> &'static str { "afrl.impact.PowerPlantState" }
}

impl Struct for PowerPlantState {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 8,
        }
    }
}

impl Lmcp for PowerPlantState {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.payload_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.parameters.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.active_power_plant.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PowerPlantState, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PowerPlantState::struct_info() {
            let mut out: PowerPlantState = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.payload_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>, usize) = Lmcp::deser(r)?;
                out.parameters = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::impact::power_plant::PowerPlant, usize) = Lmcp::deser(r)?;
                out.active_power_plant = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.payload_id.size();
        size += self.parameters.size();
        size += self.active_power_plant.size();

        size
    }
}

pub trait PowerPlantStateT: Debug + Send + ::afrl::cmasi::payload_state::PayloadStateT {
    fn as_afrl_impact_power_plant_state(&self) -> Option<&PowerPlantState> { None }
    fn as_mut_afrl_impact_power_plant_state(&mut self) -> Option<&mut PowerPlantState> { None }
    fn active_power_plant(&self) -> ::afrl::impact::power_plant::PowerPlant;
    fn active_power_plant_mut(&mut self) -> &mut ::afrl::impact::power_plant::PowerPlant;

}

impl Clone for Box<PowerPlantStateT> {
    fn clone(&self) -> Box<PowerPlantStateT> {
        if let Some(x) = PowerPlantStateT::as_afrl_impact_power_plant_state(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PowerPlantStateT> {
    fn default() -> Box<PowerPlantStateT> { Box::new(PowerPlantState::default()) }
}

impl PartialEq for Box<PowerPlantStateT> {
    fn eq(&self, other: &Box<PowerPlantStateT>) -> bool {
        if let (Some(x), Some(y)) =
            (PowerPlantStateT::as_afrl_impact_power_plant_state(self.as_ref()),
             PowerPlantStateT::as_afrl_impact_power_plant_state(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PowerPlantStateT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PowerPlantStateT::as_afrl_impact_power_plant_state(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PowerPlantStateT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PowerPlantState::struct_info() {
            let (x, readb) = PowerPlantState::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PowerPlantStateT::as_afrl_impact_power_plant_state(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_state::PayloadStateT for PowerPlantState {
    fn as_afrl_impact_power_plant_state(&self) -> Option<&PowerPlantState> { Some(self) }
    fn as_mut_afrl_impact_power_plant_state(&mut self) -> Option<&mut PowerPlantState> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl PowerPlantStateT for PowerPlantState {
    fn as_afrl_impact_power_plant_state(&self) -> Option<&PowerPlantState> { Some(self) }
    fn as_mut_afrl_impact_power_plant_state(&mut self) -> Option<&mut PowerPlantState> { Some(self) }
    fn active_power_plant(&self) -> ::afrl::impact::power_plant::PowerPlant { self.active_power_plant }
    fn active_power_plant_mut(&mut self) -> &mut ::afrl::impact::power_plant::PowerPlant { &mut self.active_power_plant }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PowerPlantState {
        fn arbitrary<G: Gen>(_g: &mut G) -> PowerPlantState {
            PowerPlantState {
                payload_id: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                active_power_plant: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PowerPlantState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PowerPlantState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PowerPlantState::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
