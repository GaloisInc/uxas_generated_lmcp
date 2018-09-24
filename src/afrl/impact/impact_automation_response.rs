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
pub struct ImpactAutomationResponse {
    pub response_id: i64,
    pub trial_response: Box<::afrl::cmasi::automation_response::AutomationResponseT>,
    pub play_id: i64,
    pub solution_id: i64,
    pub sandbox: bool,
    pub summaries: Vec<Box<::afrl::impact::task_summary::TaskSummaryT>>,
}

impl PartialEq for ImpactAutomationResponse {
    fn eq(&self, _other: &ImpactAutomationResponse) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.trial_response == &_other.trial_response
        && &self.play_id == &_other.play_id
        && &self.solution_id == &_other.solution_id
        && &self.sandbox == &_other.sandbox
        && &self.summaries == &_other.summaries

    }
}

impl LmcpSubscription for ImpactAutomationResponse {
    fn subscription() -> &'static str { "afrl.impact.ImpactAutomationResponse" }
}

impl Struct for ImpactAutomationResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 18,
        }
    }
}

impl Lmcp for ImpactAutomationResponse {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.response_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.trial_response.ser(r)?;
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
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.summaries.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(ImpactAutomationResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == ImpactAutomationResponse::struct_info() {
            let mut out: ImpactAutomationResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::automation_response::AutomationResponseT>, usize) = Lmcp::deser(r)?;
                out.trial_response = x;
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
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::impact::task_summary::TaskSummaryT>>, usize) = Lmcp::deser(r)?;
                out.summaries = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.response_id.size();
        size += self.trial_response.size();
        size += self.play_id.size();
        size += self.solution_id.size();
        size += self.sandbox.size();
        size += self.summaries.size();

        size
    }
}

pub trait ImpactAutomationResponseT: Debug + Send  {
    fn as_afrl_impact_impact_automation_response(&self) -> Option<&ImpactAutomationResponse> { None }
    fn as_mut_afrl_impact_impact_automation_response(&mut self) -> Option<&mut ImpactAutomationResponse> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn trial_response(&self) -> &Box<::afrl::cmasi::automation_response::AutomationResponseT>;
    fn trial_response_mut(&mut self) -> &mut Box<::afrl::cmasi::automation_response::AutomationResponseT>;
    fn play_id(&self) -> i64;
    fn play_id_mut(&mut self) -> &mut i64;
    fn solution_id(&self) -> i64;
    fn solution_id_mut(&mut self) -> &mut i64;
    fn sandbox(&self) -> bool;
    fn sandbox_mut(&mut self) -> &mut bool;
    fn summaries(&self) -> &Vec<Box<::afrl::impact::task_summary::TaskSummaryT>>;
    fn summaries_mut(&mut self) -> &mut Vec<Box<::afrl::impact::task_summary::TaskSummaryT>>;

}

impl Clone for Box<ImpactAutomationResponseT> {
    fn clone(&self) -> Box<ImpactAutomationResponseT> {
        if let Some(x) = ImpactAutomationResponseT::as_afrl_impact_impact_automation_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<ImpactAutomationResponseT> {
    fn default() -> Box<ImpactAutomationResponseT> { Box::new(ImpactAutomationResponse::default()) }
}

impl PartialEq for Box<ImpactAutomationResponseT> {
    fn eq(&self, other: &Box<ImpactAutomationResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (ImpactAutomationResponseT::as_afrl_impact_impact_automation_response(self.as_ref()),
             ImpactAutomationResponseT::as_afrl_impact_impact_automation_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<ImpactAutomationResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = ImpactAutomationResponseT::as_afrl_impact_impact_automation_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<ImpactAutomationResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == ImpactAutomationResponse::struct_info() {
            let (x, readb) = ImpactAutomationResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = ImpactAutomationResponseT::as_afrl_impact_impact_automation_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ImpactAutomationResponseT for ImpactAutomationResponse {
    fn as_afrl_impact_impact_automation_response(&self) -> Option<&ImpactAutomationResponse> { Some(self) }
    fn as_mut_afrl_impact_impact_automation_response(&mut self) -> Option<&mut ImpactAutomationResponse> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn trial_response(&self) -> &Box<::afrl::cmasi::automation_response::AutomationResponseT> { &self.trial_response }
    fn trial_response_mut(&mut self) -> &mut Box<::afrl::cmasi::automation_response::AutomationResponseT> { &mut self.trial_response }
    fn play_id(&self) -> i64 { self.play_id }
    fn play_id_mut(&mut self) -> &mut i64 { &mut self.play_id }
    fn solution_id(&self) -> i64 { self.solution_id }
    fn solution_id_mut(&mut self) -> &mut i64 { &mut self.solution_id }
    fn sandbox(&self) -> bool { self.sandbox }
    fn sandbox_mut(&mut self) -> &mut bool { &mut self.sandbox }
    fn summaries(&self) -> &Vec<Box<::afrl::impact::task_summary::TaskSummaryT>> { &self.summaries }
    fn summaries_mut(&mut self) -> &mut Vec<Box<::afrl::impact::task_summary::TaskSummaryT>> { &mut self.summaries }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for ImpactAutomationResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> ImpactAutomationResponse {
            ImpactAutomationResponse {
                response_id: Arbitrary::arbitrary(_g),
                trial_response: Box::new(::afrl::cmasi::automation_response::AutomationResponse::arbitrary(_g)),
                play_id: Arbitrary::arbitrary(_g),
                solution_id: Arbitrary::arbitrary(_g),
                sandbox: Arbitrary::arbitrary(_g),
                summaries: Vec::<::afrl::impact::task_summary::TaskSummary>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::impact::task_summary::TaskSummaryT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: ImpactAutomationResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.summaries.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: ImpactAutomationResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.summaries.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = ImpactAutomationResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
