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
pub struct BatchSummaryResponse {
    pub response_id: i64,
    pub summaries: Vec<Box<::afrl::impact::task_summary::TaskSummaryT>>,
}

impl PartialEq for BatchSummaryResponse {
    fn eq(&self, _other: &BatchSummaryResponse) -> bool {
        true
        && &self.response_id == &_other.response_id
        && &self.summaries == &_other.summaries

    }
}

impl LmcpSubscription for BatchSummaryResponse {
    fn subscription() -> &'static str { "afrl.impact.BatchSummaryResponse" }
}

impl Struct for BatchSummaryResponse {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 13,
        }
    }
}

impl Lmcp for BatchSummaryResponse {
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
            let writeb: usize = self.summaries.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(BatchSummaryResponse, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == BatchSummaryResponse::struct_info() {
            let mut out: BatchSummaryResponse = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.response_id = x;
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
        size += self.summaries.size();

        size
    }
}

pub trait BatchSummaryResponseT: Debug + Send  {
    fn as_afrl_impact_batch_summary_response(&self) -> Option<&BatchSummaryResponse> { None }
    fn as_mut_afrl_impact_batch_summary_response(&mut self) -> Option<&mut BatchSummaryResponse> { None }
    fn response_id(&self) -> i64;
    fn response_id_mut(&mut self) -> &mut i64;
    fn summaries(&self) -> &Vec<Box<::afrl::impact::task_summary::TaskSummaryT>>;
    fn summaries_mut(&mut self) -> &mut Vec<Box<::afrl::impact::task_summary::TaskSummaryT>>;

}

impl Clone for Box<BatchSummaryResponseT> {
    fn clone(&self) -> Box<BatchSummaryResponseT> {
        if let Some(x) = BatchSummaryResponseT::as_afrl_impact_batch_summary_response(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<BatchSummaryResponseT> {
    fn default() -> Box<BatchSummaryResponseT> { Box::new(BatchSummaryResponse::default()) }
}

impl PartialEq for Box<BatchSummaryResponseT> {
    fn eq(&self, other: &Box<BatchSummaryResponseT>) -> bool {
        if let (Some(x), Some(y)) =
            (BatchSummaryResponseT::as_afrl_impact_batch_summary_response(self.as_ref()),
             BatchSummaryResponseT::as_afrl_impact_batch_summary_response(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<BatchSummaryResponseT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = BatchSummaryResponseT::as_afrl_impact_batch_summary_response(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<BatchSummaryResponseT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == BatchSummaryResponse::struct_info() {
            let (x, readb) = BatchSummaryResponse::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = BatchSummaryResponseT::as_afrl_impact_batch_summary_response(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl BatchSummaryResponseT for BatchSummaryResponse {
    fn as_afrl_impact_batch_summary_response(&self) -> Option<&BatchSummaryResponse> { Some(self) }
    fn as_mut_afrl_impact_batch_summary_response(&mut self) -> Option<&mut BatchSummaryResponse> { Some(self) }
    fn response_id(&self) -> i64 { self.response_id }
    fn response_id_mut(&mut self) -> &mut i64 { &mut self.response_id }
    fn summaries(&self) -> &Vec<Box<::afrl::impact::task_summary::TaskSummaryT>> { &self.summaries }
    fn summaries_mut(&mut self) -> &mut Vec<Box<::afrl::impact::task_summary::TaskSummaryT>> { &mut self.summaries }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for BatchSummaryResponse {
        fn arbitrary<G: Gen>(_g: &mut G) -> BatchSummaryResponse {
            BatchSummaryResponse {
                response_id: Arbitrary::arbitrary(_g),
                summaries: Vec::<::afrl::impact::task_summary::TaskSummary>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::impact::task_summary::TaskSummaryT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: BatchSummaryResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.summaries.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: BatchSummaryResponse) -> Result<TestResult, Error> {
            use std::u16;
            if x.summaries.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = BatchSummaryResponse::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
