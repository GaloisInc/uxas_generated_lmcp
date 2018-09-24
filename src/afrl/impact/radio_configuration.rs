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
pub struct RadioConfiguration {
    pub payload_id: i64,
    pub payload_kind: Vec<u8>,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub range: f32,
    pub rally_point: Option<Box<::afrl::cmasi::location3d::Location3DT>>,
    pub timeout: i64,
}

impl PartialEq for RadioConfiguration {
    fn eq(&self, _other: &RadioConfiguration) -> bool {
        true
        && &self.range == &_other.range
        && &self.rally_point == &_other.rally_point
        && &self.timeout == &_other.timeout

    }
}

impl LmcpSubscription for RadioConfiguration {
    fn subscription() -> &'static str { "afrl.impact.RadioConfiguration" }
}

impl Struct for RadioConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 2,
        }
    }
}

impl Lmcp for RadioConfiguration {
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
            let writeb: usize = self.range.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.rally_point.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.timeout.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RadioConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RadioConfiguration::struct_info() {
            let mut out: RadioConfiguration = Default::default();
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
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.range = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Option<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.rally_point = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.timeout = x;
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
        size += self.range.size();
        size += self.rally_point.size();
        size += self.timeout.size();

        size
    }
}

pub trait RadioConfigurationT: Debug + Send + ::afrl::cmasi::payload_configuration::PayloadConfigurationT {
    fn as_afrl_impact_radio_configuration(&self) -> Option<&RadioConfiguration> { None }
    fn as_mut_afrl_impact_radio_configuration(&mut self) -> Option<&mut RadioConfiguration> { None }
    fn range(&self) -> f32;
    fn range_mut(&mut self) -> &mut f32;
    fn rally_point(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn rally_point_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn timeout(&self) -> i64;
    fn timeout_mut(&mut self) -> &mut i64;

}

impl Clone for Box<RadioConfigurationT> {
    fn clone(&self) -> Box<RadioConfigurationT> {
        if let Some(x) = RadioConfigurationT::as_afrl_impact_radio_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RadioConfigurationT> {
    fn default() -> Box<RadioConfigurationT> { Box::new(RadioConfiguration::default()) }
}

impl PartialEq for Box<RadioConfigurationT> {
    fn eq(&self, other: &Box<RadioConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (RadioConfigurationT::as_afrl_impact_radio_configuration(self.as_ref()),
             RadioConfigurationT::as_afrl_impact_radio_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RadioConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RadioConfigurationT::as_afrl_impact_radio_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RadioConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RadioConfiguration::struct_info() {
            let (x, readb) = RadioConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RadioConfigurationT::as_afrl_impact_radio_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_configuration::PayloadConfigurationT for RadioConfiguration {
    fn as_afrl_impact_radio_configuration(&self) -> Option<&RadioConfiguration> { Some(self) }
    fn as_mut_afrl_impact_radio_configuration(&mut self) -> Option<&mut RadioConfiguration> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn payload_kind(&self) -> &Vec<u8> { &self.payload_kind }
    fn payload_kind_mut(&mut self) -> &mut Vec<u8> { &mut self.payload_kind }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl RadioConfigurationT for RadioConfiguration {
    fn as_afrl_impact_radio_configuration(&self) -> Option<&RadioConfiguration> { Some(self) }
    fn as_mut_afrl_impact_radio_configuration(&mut self) -> Option<&mut RadioConfiguration> { Some(self) }
    fn range(&self) -> f32 { self.range }
    fn range_mut(&mut self) -> &mut f32 { &mut self.range }
    fn rally_point(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>> { &self.rally_point }
    fn rally_point_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.rally_point }
    fn timeout(&self) -> i64 { self.timeout }
    fn timeout_mut(&mut self) -> &mut i64 { &mut self.timeout }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RadioConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> RadioConfiguration {
            RadioConfiguration {
                payload_id: Arbitrary::arbitrary(_g),
                payload_kind: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                range: Arbitrary::arbitrary(_g),
                rally_point: {
                    if _g.gen() {
                        Some(Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)))
                    } else {
                        None
                    }
                },
                timeout: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RadioConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RadioConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RadioConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
