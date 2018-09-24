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
pub struct ServiceStatus {
    pub percent_complete: f32,
    pub info: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub status_type: ::afrl::cmasi::service_status_type::ServiceStatusType,
}

impl PartialEq for ServiceStatus {
    fn eq(&self, _other: &ServiceStatus) -> bool {
        true
        && &self.percent_complete == &_other.percent_complete
        && &self.info == &_other.info
        && &self.status_type == &_other.status_type

    }
}

impl LmcpSubscription for ServiceStatus {
    fn subscription() -> &'static str { "afrl.cmasi.ServiceStatus" }
}

impl Struct for ServiceStatus {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 45,
        }
    }
}

impl Lmcp for ServiceStatus {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.percent_complete.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.info.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.status_type.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(ServiceStatus, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == ServiceStatus::struct_info() {
            let mut out: ServiceStatus = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.percent_complete = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>, usize) = Lmcp::deser(r)?;
                out.info = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::service_status_type::ServiceStatusType, usize) = Lmcp::deser(r)?;
                out.status_type = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.percent_complete.size();
        size += self.info.size();
        size += self.status_type.size();

        size
    }
}

pub trait ServiceStatusT: Debug + Send  {
    fn as_afrl_cmasi_service_status(&self) -> Option<&ServiceStatus> { None }
    fn as_mut_afrl_cmasi_service_status(&mut self) -> Option<&mut ServiceStatus> { None }
    fn percent_complete(&self) -> f32;
    fn percent_complete_mut(&mut self) -> &mut f32;
    fn info(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn info_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn status_type(&self) -> ::afrl::cmasi::service_status_type::ServiceStatusType;
    fn status_type_mut(&mut self) -> &mut ::afrl::cmasi::service_status_type::ServiceStatusType;

}

impl Clone for Box<ServiceStatusT> {
    fn clone(&self) -> Box<ServiceStatusT> {
        if let Some(x) = ServiceStatusT::as_afrl_cmasi_service_status(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<ServiceStatusT> {
    fn default() -> Box<ServiceStatusT> { Box::new(ServiceStatus::default()) }
}

impl PartialEq for Box<ServiceStatusT> {
    fn eq(&self, other: &Box<ServiceStatusT>) -> bool {
        if let (Some(x), Some(y)) =
            (ServiceStatusT::as_afrl_cmasi_service_status(self.as_ref()),
             ServiceStatusT::as_afrl_cmasi_service_status(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<ServiceStatusT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = ServiceStatusT::as_afrl_cmasi_service_status(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<ServiceStatusT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == ServiceStatus::struct_info() {
            let (x, readb) = ServiceStatus::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = ServiceStatusT::as_afrl_cmasi_service_status(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ServiceStatusT for ServiceStatus {
    fn as_afrl_cmasi_service_status(&self) -> Option<&ServiceStatus> { Some(self) }
    fn as_mut_afrl_cmasi_service_status(&mut self) -> Option<&mut ServiceStatus> { Some(self) }
    fn percent_complete(&self) -> f32 { self.percent_complete }
    fn percent_complete_mut(&mut self) -> &mut f32 { &mut self.percent_complete }
    fn info(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.info }
    fn info_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.info }
    fn status_type(&self) -> ::afrl::cmasi::service_status_type::ServiceStatusType { self.status_type }
    fn status_type_mut(&mut self) -> &mut ::afrl::cmasi::service_status_type::ServiceStatusType { &mut self.status_type }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for ServiceStatus {
        fn arbitrary<G: Gen>(_g: &mut G) -> ServiceStatus {
            ServiceStatus {
                percent_complete: Arbitrary::arbitrary(_g),
                info: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                status_type: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: ServiceStatus) -> Result<TestResult, Error> {
            use std::u16;
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: ServiceStatus) -> Result<TestResult, Error> {
            use std::u16;
            if x.info.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = ServiceStatus::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
