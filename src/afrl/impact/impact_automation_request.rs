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
pub struct ImpactAutomationRequest {
    pub request_id: i64,
    pub trial_request: Box<::afrl::cmasi::automation_request::AutomationRequestT>,
    pub override_planning_conditions: Vec<Box<::afrl::impact::speed_alt_pair::SpeedAltPairT>>,
    pub play_id: i64,
    pub solution_id: i64,
    pub sandbox: bool,
}

impl PartialEq for ImpactAutomationRequest {
    fn eq(&self, _other: &ImpactAutomationRequest) -> bool {
        true
        && &self.request_id == &_other.request_id
        && &self.trial_request == &_other.trial_request
        && &self.override_planning_conditions == &_other.override_planning_conditions
        && &self.play_id == &_other.play_id
        && &self.solution_id == &_other.solution_id
        && &self.sandbox == &_other.sandbox

    }
}

impl LmcpSubscription for ImpactAutomationRequest {
    fn subscription() -> &'static str { "afrl.impact.ImpactAutomationRequest" }
}

impl Struct for ImpactAutomationRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 17,
        }
    }
}

impl Lmcp for ImpactAutomationRequest {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.request_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.trial_request.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.override_planning_conditions.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.play_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.solution_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.sandbox.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(ImpactAutomationRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == ImpactAutomationRequest::struct_info() {
            let mut out: ImpactAutomationRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::automation_request::AutomationRequestT>, usize) = Lmcp::deser(r)?;
                out.trial_request = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::impact::speed_alt_pair::SpeedAltPairT>>, usize) = Lmcp::deser(r)?;
                out.override_planning_conditions = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.play_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.solution_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.sandbox = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.request_id.size();
        size += self.trial_request.size();
        size += self.override_planning_conditions.size();
        size += self.play_id.size();
        size += self.solution_id.size();
        size += self.sandbox.size();

        size
    }
}

pub trait ImpactAutomationRequestT: Debug + Send  {
    fn as_afrl_impact_impact_automation_request(&self) -> Option<&ImpactAutomationRequest> { None }
    fn as_mut_afrl_impact_impact_automation_request(&mut self) -> Option<&mut ImpactAutomationRequest> { None }
    fn request_id(&self) -> i64;
    fn request_id_mut(&mut self) -> &mut i64;
    fn trial_request(&self) -> &Box<::afrl::cmasi::automation_request::AutomationRequestT>;
    fn trial_request_mut(&mut self) -> &mut Box<::afrl::cmasi::automation_request::AutomationRequestT>;
    fn override_planning_conditions(&self) -> &Vec<Box<::afrl::impact::speed_alt_pair::SpeedAltPairT>>;
    fn override_planning_conditions_mut(&mut self) -> &mut Vec<Box<::afrl::impact::speed_alt_pair::SpeedAltPairT>>;
    fn play_id(&self) -> i64;
    fn play_id_mut(&mut self) -> &mut i64;
    fn solution_id(&self) -> i64;
    fn solution_id_mut(&mut self) -> &mut i64;
    fn sandbox(&self) -> bool;
    fn sandbox_mut(&mut self) -> &mut bool;

}

impl Clone for Box<ImpactAutomationRequestT> {
    fn clone(&self) -> Box<ImpactAutomationRequestT> {
        if let Some(x) = ImpactAutomationRequestT::as_afrl_impact_impact_automation_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<ImpactAutomationRequestT> {
    fn default() -> Box<ImpactAutomationRequestT> { Box::new(ImpactAutomationRequest::default()) }
}

impl PartialEq for Box<ImpactAutomationRequestT> {
    fn eq(&self, other: &Box<ImpactAutomationRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (ImpactAutomationRequestT::as_afrl_impact_impact_automation_request(self.as_ref()),
             ImpactAutomationRequestT::as_afrl_impact_impact_automation_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<ImpactAutomationRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = ImpactAutomationRequestT::as_afrl_impact_impact_automation_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<ImpactAutomationRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == ImpactAutomationRequest::struct_info() {
            let (x, readb) = ImpactAutomationRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = ImpactAutomationRequestT::as_afrl_impact_impact_automation_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ImpactAutomationRequestT for ImpactAutomationRequest {
    fn as_afrl_impact_impact_automation_request(&self) -> Option<&ImpactAutomationRequest> { Some(self) }
    fn as_mut_afrl_impact_impact_automation_request(&mut self) -> Option<&mut ImpactAutomationRequest> { Some(self) }
    fn request_id(&self) -> i64 { self.request_id }
    fn request_id_mut(&mut self) -> &mut i64 { &mut self.request_id }
    fn trial_request(&self) -> &Box<::afrl::cmasi::automation_request::AutomationRequestT> { &self.trial_request }
    fn trial_request_mut(&mut self) -> &mut Box<::afrl::cmasi::automation_request::AutomationRequestT> { &mut self.trial_request }
    fn override_planning_conditions(&self) -> &Vec<Box<::afrl::impact::speed_alt_pair::SpeedAltPairT>> { &self.override_planning_conditions }
    fn override_planning_conditions_mut(&mut self) -> &mut Vec<Box<::afrl::impact::speed_alt_pair::SpeedAltPairT>> { &mut self.override_planning_conditions }
    fn play_id(&self) -> i64 { self.play_id }
    fn play_id_mut(&mut self) -> &mut i64 { &mut self.play_id }
    fn solution_id(&self) -> i64 { self.solution_id }
    fn solution_id_mut(&mut self) -> &mut i64 { &mut self.solution_id }
    fn sandbox(&self) -> bool { self.sandbox }
    fn sandbox_mut(&mut self) -> &mut bool { &mut self.sandbox }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for ImpactAutomationRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> ImpactAutomationRequest {
            ImpactAutomationRequest {
                request_id: Arbitrary::arbitrary(_g),
                trial_request: Box::new(::afrl::cmasi::automation_request::AutomationRequest::arbitrary(_g)),
                override_planning_conditions: Vec::<::afrl::impact::speed_alt_pair::SpeedAltPair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::impact::speed_alt_pair::SpeedAltPairT>).collect(),
                play_id: Arbitrary::arbitrary(_g),
                solution_id: Arbitrary::arbitrary(_g),
                sandbox: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: ImpactAutomationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.override_planning_conditions.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: ImpactAutomationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.override_planning_conditions.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = ImpactAutomationRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
