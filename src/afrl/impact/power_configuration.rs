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
pub struct PowerConfiguration {
    pub payload_id: i64,
    pub payload_kind: Vec<u8>,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub nominal_power_configuration: ::afrl::impact::power_plant::PowerPlant,
}

impl PartialEq for PowerConfiguration {
    fn eq(&self, _other: &PowerConfiguration) -> bool {
        true
        && &self.nominal_power_configuration == &_other.nominal_power_configuration

    }
}

impl LmcpSubscription for PowerConfiguration {
    fn subscription() -> &'static str { "afrl.impact.PowerConfiguration" }
}

impl Struct for PowerConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 1,
        }
    }
}

impl Lmcp for PowerConfiguration {
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
            let writeb: usize = self.payload_kind.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.parameters.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.nominal_power_configuration.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PowerConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PowerConfiguration::struct_info() {
            let mut out: PowerConfiguration = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.payload_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.payload_kind = x;
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
                out.nominal_power_configuration = x;
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
        size += self.payload_kind.size();
        size += self.parameters.size();
        size += self.nominal_power_configuration.size();

        size
    }
}

pub trait PowerConfigurationT: Debug + Send + ::afrl::cmasi::payload_configuration::PayloadConfigurationT {
    fn as_afrl_impact_power_configuration(&self) -> Option<&PowerConfiguration> { None }
    fn as_mut_afrl_impact_power_configuration(&mut self) -> Option<&mut PowerConfiguration> { None }
    fn nominal_power_configuration(&self) -> ::afrl::impact::power_plant::PowerPlant;
    fn nominal_power_configuration_mut(&mut self) -> &mut ::afrl::impact::power_plant::PowerPlant;

}

impl Clone for Box<PowerConfigurationT> {
    fn clone(&self) -> Box<PowerConfigurationT> {
        if let Some(x) = PowerConfigurationT::as_afrl_impact_power_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PowerConfigurationT> {
    fn default() -> Box<PowerConfigurationT> { Box::new(PowerConfiguration::default()) }
}

impl PartialEq for Box<PowerConfigurationT> {
    fn eq(&self, other: &Box<PowerConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (PowerConfigurationT::as_afrl_impact_power_configuration(self.as_ref()),
             PowerConfigurationT::as_afrl_impact_power_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PowerConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PowerConfigurationT::as_afrl_impact_power_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PowerConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PowerConfiguration::struct_info() {
            let (x, readb) = PowerConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PowerConfigurationT::as_afrl_impact_power_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_configuration::PayloadConfigurationT for PowerConfiguration {
    fn as_afrl_impact_power_configuration(&self) -> Option<&PowerConfiguration> { Some(self) }
    fn as_mut_afrl_impact_power_configuration(&mut self) -> Option<&mut PowerConfiguration> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn payload_kind(&self) -> &Vec<u8> { &self.payload_kind }
    fn payload_kind_mut(&mut self) -> &mut Vec<u8> { &mut self.payload_kind }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl PowerConfigurationT for PowerConfiguration {
    fn as_afrl_impact_power_configuration(&self) -> Option<&PowerConfiguration> { Some(self) }
    fn as_mut_afrl_impact_power_configuration(&mut self) -> Option<&mut PowerConfiguration> { Some(self) }
    fn nominal_power_configuration(&self) -> ::afrl::impact::power_plant::PowerPlant { self.nominal_power_configuration }
    fn nominal_power_configuration_mut(&mut self) -> &mut ::afrl::impact::power_plant::PowerPlant { &mut self.nominal_power_configuration }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PowerConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> PowerConfiguration {
            PowerConfiguration {
                payload_id: Arbitrary::arbitrary(_g),
                payload_kind: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                nominal_power_configuration: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PowerConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PowerConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PowerConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
