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
pub struct ImpactPayloadConfiguration {
    pub payload_id: i64,
    pub payload_kind: Vec<u8>,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub available_payloads: Vec<::afrl::impact::impact_payload_type::ImpactPayloadType>,
}

impl PartialEq for ImpactPayloadConfiguration {
    fn eq(&self, _other: &ImpactPayloadConfiguration) -> bool {
        true
        && &self.available_payloads == &_other.available_payloads

    }
}

impl LmcpSubscription for ImpactPayloadConfiguration {
    fn subscription() -> &'static str { "afrl.impact.ImpactPayloadConfiguration" }
}

impl Struct for ImpactPayloadConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 6,
        }
    }
}

impl Lmcp for ImpactPayloadConfiguration {
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
            let writeb: usize = self.available_payloads.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(ImpactPayloadConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == ImpactPayloadConfiguration::struct_info() {
            let mut out: ImpactPayloadConfiguration = Default::default();
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
                let (x, readb): (Vec<::afrl::impact::impact_payload_type::ImpactPayloadType>, usize) = Lmcp::deser(r)?;
                out.available_payloads = x;
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
        size += self.available_payloads.size();

        size
    }
}

pub trait ImpactPayloadConfigurationT: Debug + Send + ::afrl::cmasi::payload_configuration::PayloadConfigurationT {
    fn as_afrl_impact_impact_payload_configuration(&self) -> Option<&ImpactPayloadConfiguration> { None }
    fn as_mut_afrl_impact_impact_payload_configuration(&mut self) -> Option<&mut ImpactPayloadConfiguration> { None }
    fn available_payloads(&self) -> &Vec<::afrl::impact::impact_payload_type::ImpactPayloadType>;
    fn available_payloads_mut(&mut self) -> &mut Vec<::afrl::impact::impact_payload_type::ImpactPayloadType>;

}

impl Clone for Box<ImpactPayloadConfigurationT> {
    fn clone(&self) -> Box<ImpactPayloadConfigurationT> {
        if let Some(x) = ImpactPayloadConfigurationT::as_afrl_impact_impact_payload_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<ImpactPayloadConfigurationT> {
    fn default() -> Box<ImpactPayloadConfigurationT> { Box::new(ImpactPayloadConfiguration::default()) }
}

impl PartialEq for Box<ImpactPayloadConfigurationT> {
    fn eq(&self, other: &Box<ImpactPayloadConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (ImpactPayloadConfigurationT::as_afrl_impact_impact_payload_configuration(self.as_ref()),
             ImpactPayloadConfigurationT::as_afrl_impact_impact_payload_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<ImpactPayloadConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = ImpactPayloadConfigurationT::as_afrl_impact_impact_payload_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<ImpactPayloadConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == ImpactPayloadConfiguration::struct_info() {
            let (x, readb) = ImpactPayloadConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = ImpactPayloadConfigurationT::as_afrl_impact_impact_payload_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_configuration::PayloadConfigurationT for ImpactPayloadConfiguration {
    fn as_afrl_impact_impact_payload_configuration(&self) -> Option<&ImpactPayloadConfiguration> { Some(self) }
    fn as_mut_afrl_impact_impact_payload_configuration(&mut self) -> Option<&mut ImpactPayloadConfiguration> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn payload_kind(&self) -> &Vec<u8> { &self.payload_kind }
    fn payload_kind_mut(&mut self) -> &mut Vec<u8> { &mut self.payload_kind }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl ImpactPayloadConfigurationT for ImpactPayloadConfiguration {
    fn as_afrl_impact_impact_payload_configuration(&self) -> Option<&ImpactPayloadConfiguration> { Some(self) }
    fn as_mut_afrl_impact_impact_payload_configuration(&mut self) -> Option<&mut ImpactPayloadConfiguration> { Some(self) }
    fn available_payloads(&self) -> &Vec<::afrl::impact::impact_payload_type::ImpactPayloadType> { &self.available_payloads }
    fn available_payloads_mut(&mut self) -> &mut Vec<::afrl::impact::impact_payload_type::ImpactPayloadType> { &mut self.available_payloads }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for ImpactPayloadConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> ImpactPayloadConfiguration {
            ImpactPayloadConfiguration {
                payload_id: Arbitrary::arbitrary(_g),
                payload_kind: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                available_payloads: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: ImpactPayloadConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.available_payloads.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: ImpactPayloadConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.available_payloads.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = ImpactPayloadConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
