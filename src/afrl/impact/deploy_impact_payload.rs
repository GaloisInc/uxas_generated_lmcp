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
pub struct DeployImpactPayload {
    pub associated_task_list: Vec<i64>,
    pub vehicle_id: i64,
    pub deployed_payload: ::afrl::impact::impact_payload_type::ImpactPayloadType,
    pub target_entity_id: i64,
}

impl PartialEq for DeployImpactPayload {
    fn eq(&self, _other: &DeployImpactPayload) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id
        && &self.deployed_payload == &_other.deployed_payload
        && &self.target_entity_id == &_other.target_entity_id

    }
}

impl LmcpSubscription for DeployImpactPayload {
    fn subscription() -> &'static str { "afrl.impact.DeployImpactPayload" }
}

impl Struct for DeployImpactPayload {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 7,
        }
    }
}

impl Lmcp for DeployImpactPayload {
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
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.deployed_payload.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.target_entity_id.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(DeployImpactPayload, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == DeployImpactPayload::struct_info() {
            let mut out: DeployImpactPayload = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::impact::impact_payload_type::ImpactPayloadType, usize) = Lmcp::deser(r)?;
                out.deployed_payload = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.target_entity_id = x;
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
        size += self.vehicle_id.size();
        size += self.deployed_payload.size();
        size += self.target_entity_id.size();

        size
    }
}

pub trait DeployImpactPayloadT: Debug + Send + ::afrl::cmasi::vehicle_action::VehicleActionT {
    fn as_afrl_impact_deploy_impact_payload(&self) -> Option<&DeployImpactPayload> { None }
    fn as_mut_afrl_impact_deploy_impact_payload(&mut self) -> Option<&mut DeployImpactPayload> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn deployed_payload(&self) -> ::afrl::impact::impact_payload_type::ImpactPayloadType;
    fn deployed_payload_mut(&mut self) -> &mut ::afrl::impact::impact_payload_type::ImpactPayloadType;
    fn target_entity_id(&self) -> i64;
    fn target_entity_id_mut(&mut self) -> &mut i64;

}

impl Clone for Box<DeployImpactPayloadT> {
    fn clone(&self) -> Box<DeployImpactPayloadT> {
        if let Some(x) = DeployImpactPayloadT::as_afrl_impact_deploy_impact_payload(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<DeployImpactPayloadT> {
    fn default() -> Box<DeployImpactPayloadT> { Box::new(DeployImpactPayload::default()) }
}

impl PartialEq for Box<DeployImpactPayloadT> {
    fn eq(&self, other: &Box<DeployImpactPayloadT>) -> bool {
        if let (Some(x), Some(y)) =
            (DeployImpactPayloadT::as_afrl_impact_deploy_impact_payload(self.as_ref()),
             DeployImpactPayloadT::as_afrl_impact_deploy_impact_payload(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<DeployImpactPayloadT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = DeployImpactPayloadT::as_afrl_impact_deploy_impact_payload(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<DeployImpactPayloadT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == DeployImpactPayload::struct_info() {
            let (x, readb) = DeployImpactPayload::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = DeployImpactPayloadT::as_afrl_impact_deploy_impact_payload(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for DeployImpactPayload {
    fn as_afrl_impact_deploy_impact_payload(&self) -> Option<&DeployImpactPayload> { Some(self) }
    fn as_mut_afrl_impact_deploy_impact_payload(&mut self) -> Option<&mut DeployImpactPayload> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl DeployImpactPayloadT for DeployImpactPayload {
    fn as_afrl_impact_deploy_impact_payload(&self) -> Option<&DeployImpactPayload> { Some(self) }
    fn as_mut_afrl_impact_deploy_impact_payload(&mut self) -> Option<&mut DeployImpactPayload> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn deployed_payload(&self) -> ::afrl::impact::impact_payload_type::ImpactPayloadType { self.deployed_payload }
    fn deployed_payload_mut(&mut self) -> &mut ::afrl::impact::impact_payload_type::ImpactPayloadType { &mut self.deployed_payload }
    fn target_entity_id(&self) -> i64 { self.target_entity_id }
    fn target_entity_id_mut(&mut self) -> &mut i64 { &mut self.target_entity_id }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for DeployImpactPayload {
        fn arbitrary<G: Gen>(_g: &mut G) -> DeployImpactPayload {
            DeployImpactPayload {
                associated_task_list: Arbitrary::arbitrary(_g),
                vehicle_id: Arbitrary::arbitrary(_g),
                deployed_payload: Arbitrary::arbitrary(_g),
                target_entity_id: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: DeployImpactPayload) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: DeployImpactPayload) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = DeployImpactPayload::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
