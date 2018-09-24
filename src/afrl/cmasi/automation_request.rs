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
pub struct AutomationRequest {
    pub entity_list: Vec<i64>,
    pub task_list: Vec<i64>,
    pub task_relationships: Vec<u8>,
    pub operating_region: i64,
    pub redo_all_tasks: bool,
}

impl PartialEq for AutomationRequest {
    fn eq(&self, _other: &AutomationRequest) -> bool {
        true
        && &self.entity_list == &_other.entity_list
        && &self.task_list == &_other.task_list
        && &self.task_relationships == &_other.task_relationships
        && &self.operating_region == &_other.operating_region
        && &self.redo_all_tasks == &_other.redo_all_tasks

    }
}

impl LmcpSubscription for AutomationRequest {
    fn subscription() -> &'static str { "afrl.cmasi.AutomationRequest" }
}

impl Struct for AutomationRequest {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 40,
        }
    }
}

impl Lmcp for AutomationRequest {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entity_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_relationships.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.operating_region.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.redo_all_tasks.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(AutomationRequest, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == AutomationRequest::struct_info() {
            let mut out: AutomationRequest = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.entity_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.task_relationships = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.operating_region = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.redo_all_tasks = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.entity_list.size();
        size += self.task_list.size();
        size += self.task_relationships.size();
        size += self.operating_region.size();
        size += self.redo_all_tasks.size();

        size
    }
}

pub trait AutomationRequestT: Debug + Send  {
    fn as_afrl_cmasi_automation_request(&self) -> Option<&AutomationRequest> { None }
    fn as_mut_afrl_cmasi_automation_request(&mut self) -> Option<&mut AutomationRequest> { None }
    fn entity_list(&self) -> &Vec<i64>;
    fn entity_list_mut(&mut self) -> &mut Vec<i64>;
    fn task_list(&self) -> &Vec<i64>;
    fn task_list_mut(&mut self) -> &mut Vec<i64>;
    fn task_relationships(&self) -> &Vec<u8>;
    fn task_relationships_mut(&mut self) -> &mut Vec<u8>;
    fn operating_region(&self) -> i64;
    fn operating_region_mut(&mut self) -> &mut i64;
    fn redo_all_tasks(&self) -> bool;
    fn redo_all_tasks_mut(&mut self) -> &mut bool;

}

impl Clone for Box<AutomationRequestT> {
    fn clone(&self) -> Box<AutomationRequestT> {
        if let Some(x) = AutomationRequestT::as_afrl_cmasi_automation_request(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<AutomationRequestT> {
    fn default() -> Box<AutomationRequestT> { Box::new(AutomationRequest::default()) }
}

impl PartialEq for Box<AutomationRequestT> {
    fn eq(&self, other: &Box<AutomationRequestT>) -> bool {
        if let (Some(x), Some(y)) =
            (AutomationRequestT::as_afrl_cmasi_automation_request(self.as_ref()),
             AutomationRequestT::as_afrl_cmasi_automation_request(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<AutomationRequestT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = AutomationRequestT::as_afrl_cmasi_automation_request(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<AutomationRequestT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == AutomationRequest::struct_info() {
            let (x, readb) = AutomationRequest::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = AutomationRequestT::as_afrl_cmasi_automation_request(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl AutomationRequestT for AutomationRequest {
    fn as_afrl_cmasi_automation_request(&self) -> Option<&AutomationRequest> { Some(self) }
    fn as_mut_afrl_cmasi_automation_request(&mut self) -> Option<&mut AutomationRequest> { Some(self) }
    fn entity_list(&self) -> &Vec<i64> { &self.entity_list }
    fn entity_list_mut(&mut self) -> &mut Vec<i64> { &mut self.entity_list }
    fn task_list(&self) -> &Vec<i64> { &self.task_list }
    fn task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.task_list }
    fn task_relationships(&self) -> &Vec<u8> { &self.task_relationships }
    fn task_relationships_mut(&mut self) -> &mut Vec<u8> { &mut self.task_relationships }
    fn operating_region(&self) -> i64 { self.operating_region }
    fn operating_region_mut(&mut self) -> &mut i64 { &mut self.operating_region }
    fn redo_all_tasks(&self) -> bool { self.redo_all_tasks }
    fn redo_all_tasks_mut(&mut self) -> &mut bool { &mut self.redo_all_tasks }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AutomationRequest {
        fn arbitrary<G: Gen>(_g: &mut G) -> AutomationRequest {
            AutomationRequest {
                entity_list: Arbitrary::arbitrary(_g),
                task_list: Arbitrary::arbitrary(_g),
                task_relationships: Arbitrary::arbitrary(_g),
                operating_region: Arbitrary::arbitrary(_g),
                redo_all_tasks: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: AutomationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.entity_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AutomationRequest) -> Result<TestResult, Error> {
            use std::u16;
            if x.entity_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AutomationRequest::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
