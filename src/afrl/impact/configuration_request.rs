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
pub struct ConfigurationRequest {
    pub vehicle_id: Vec<i64>,
}

impl PartialEq for ConfigurationRequest {
    fn eq(&self, _other: &ConfigurationRequest) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id

    }
}

impl LmcpSubscription for ConfigurationRequest {
    fn subscription() -> &'static str { "afrl.impact.ConfigurationRequest" }
}

impl Struct for ConfigurationRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 32,
        }
    }
}

impl Lmcp for ConfigurationRequest {
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

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(ConfigurationRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == ConfigurationRequest::struct_info() {
            let mut out: ConfigurationRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
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

        size
    }
}

pub trait ConfigurationRequestT: Debug + Send  {
    fn as_afrl_impact_configuration_request(&self) -> Option<&ConfigurationRequest> { None }
    fn as_mut_afrl_impact_configuration_request(&mut self) -> Option<&mut ConfigurationRequest> { None }
    fn vehicle_id(&self) -> &Vec<i64>;
    fn vehicle_id_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<ConfigurationRequestT> {
    fn clone(&self) -> Box<ConfigurationRequestT> {
        if let Some(x) = ConfigurationRequestT::as_afrl_impact_configuration_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<ConfigurationRequestT> {
    fn default() -> Box<ConfigurationRequestT> { Box::new(ConfigurationRequest::default()) }
}

impl PartialEq for Box<ConfigurationRequestT> {
    fn eq(&self, other: &Box<ConfigurationRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (ConfigurationRequestT::as_afrl_impact_configuration_request(self.as_ref()),
             ConfigurationRequestT::as_afrl_impact_configuration_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<ConfigurationRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = ConfigurationRequestT::as_afrl_impact_configuration_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<ConfigurationRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == ConfigurationRequest::struct_info() {
            let (x, readb) = ConfigurationRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = ConfigurationRequestT::as_afrl_impact_configuration_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ConfigurationRequestT for ConfigurationRequest {
    fn as_afrl_impact_configuration_request(&self) -> Option<&ConfigurationRequest> { Some(self) }
    fn as_mut_afrl_impact_configuration_request(&mut self) -> Option<&mut ConfigurationRequest> { Some(self) }
    fn vehicle_id(&self) -> &Vec<i64> { &self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut Vec<i64> { &mut self.vehicle_id }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for ConfigurationRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> ConfigurationRequest {
            ConfigurationRequest {
                vehicle_id: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: ConfigurationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_id.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: ConfigurationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.vehicle_id.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = ConfigurationRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
