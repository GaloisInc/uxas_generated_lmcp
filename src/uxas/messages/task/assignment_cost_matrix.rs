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
pub struct AssignmentCostMatrix {
    pub corresponding_automation_request_id: i64,
    pub task_level_relationship: Vec<u8>,
    pub task_list: Vec<i64>,
    pub operating_region: i64,
    pub cost_matrix: Vec<Box<::uxas::messages::task::task_option_cost::TaskOptionCostT>>,
}

impl PartialEq for AssignmentCostMatrix {
    fn eq(&self, _other: &AssignmentCostMatrix) -> bool {
        true
        && &self.corresponding_automation_request_id == &_other.corresponding_automation_request_id
        && &self.task_level_relationship == &_other.task_level_relationship
        && &self.task_list == &_other.task_list
        && &self.operating_region == &_other.operating_region
        && &self.cost_matrix == &_other.cost_matrix

    }
}

impl LmcpSubscription for AssignmentCostMatrix {
    fn subscription() -> &'static str { "uxas.messages.task.AssignmentCostMatrix" }
}

impl Struct for AssignmentCostMatrix {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 16,
        }
    }
}

impl Lmcp for AssignmentCostMatrix {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.corresponding_automation_request_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_level_relationship.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.operating_region.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.cost_matrix.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(AssignmentCostMatrix, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == AssignmentCostMatrix::struct_info() {
            let mut out: AssignmentCostMatrix = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.corresponding_automation_request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.task_level_relationship = x;
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
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.operating_region = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::task::task_option_cost::TaskOptionCostT>>, usize) = Lmcp::deser(r)?;
                out.cost_matrix = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.corresponding_automation_request_id.size();
        size += self.task_level_relationship.size();
        size += self.task_list.size();
        size += self.operating_region.size();
        size += self.cost_matrix.size();

        size
    }
}

pub trait AssignmentCostMatrixT: Debug + Send  {
    fn as_uxas_messages_task_assignment_cost_matrix(&self) -> Option<&AssignmentCostMatrix> { None }
    fn as_mut_uxas_messages_task_assignment_cost_matrix(&mut self) -> Option<&mut AssignmentCostMatrix> { None }
    fn corresponding_automation_request_id(&self) -> i64;
    fn corresponding_automation_request_id_mut(&mut self) -> &mut i64;
    fn task_level_relationship(&self) -> &Vec<u8>;
    fn task_level_relationship_mut(&mut self) -> &mut Vec<u8>;
    fn task_list(&self) -> &Vec<i64>;
    fn task_list_mut(&mut self) -> &mut Vec<i64>;
    fn operating_region(&self) -> i64;
    fn operating_region_mut(&mut self) -> &mut i64;
    fn cost_matrix(&self) -> &Vec<Box<::uxas::messages::task::task_option_cost::TaskOptionCostT>>;
    fn cost_matrix_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::task_option_cost::TaskOptionCostT>>;

}

impl Clone for Box<AssignmentCostMatrixT> {
    fn clone(&self) -> Box<AssignmentCostMatrixT> {
        if let Some(x) = AssignmentCostMatrixT::as_uxas_messages_task_assignment_cost_matrix(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<AssignmentCostMatrixT> {
    fn default() -> Box<AssignmentCostMatrixT> { Box::new(AssignmentCostMatrix::default()) }
}

impl PartialEq for Box<AssignmentCostMatrixT> {
    fn eq(&self, other: &Box<AssignmentCostMatrixT>) -> bool {
        if let (Some(x), Some(y)) =
            (AssignmentCostMatrixT::as_uxas_messages_task_assignment_cost_matrix(self.as_ref()),
             AssignmentCostMatrixT::as_uxas_messages_task_assignment_cost_matrix(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<AssignmentCostMatrixT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = AssignmentCostMatrixT::as_uxas_messages_task_assignment_cost_matrix(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<AssignmentCostMatrixT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == AssignmentCostMatrix::struct_info() {
            let (x, readb) = AssignmentCostMatrix::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = AssignmentCostMatrixT::as_uxas_messages_task_assignment_cost_matrix(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl AssignmentCostMatrixT for AssignmentCostMatrix {
    fn as_uxas_messages_task_assignment_cost_matrix(&self) -> Option<&AssignmentCostMatrix> { Some(self) }
    fn as_mut_uxas_messages_task_assignment_cost_matrix(&mut self) -> Option<&mut AssignmentCostMatrix> { Some(self) }
    fn corresponding_automation_request_id(&self) -> i64 { self.corresponding_automation_request_id }
    fn corresponding_automation_request_id_mut(&mut self) -> &mut i64 { &mut self.corresponding_automation_request_id }
    fn task_level_relationship(&self) -> &Vec<u8> { &self.task_level_relationship }
    fn task_level_relationship_mut(&mut self) -> &mut Vec<u8> { &mut self.task_level_relationship }
    fn task_list(&self) -> &Vec<i64> { &self.task_list }
    fn task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.task_list }
    fn operating_region(&self) -> i64 { self.operating_region }
    fn operating_region_mut(&mut self) -> &mut i64 { &mut self.operating_region }
    fn cost_matrix(&self) -> &Vec<Box<::uxas::messages::task::task_option_cost::TaskOptionCostT>> { &self.cost_matrix }
    fn cost_matrix_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::task_option_cost::TaskOptionCostT>> { &mut self.cost_matrix }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AssignmentCostMatrix {
        fn arbitrary<G: Gen>(_g: &mut G) -> AssignmentCostMatrix {
            AssignmentCostMatrix {
                corresponding_automation_request_id: Arbitrary::arbitrary(_g),
                task_level_relationship: Arbitrary::arbitrary(_g),
                task_list: Arbitrary::arbitrary(_g),
                operating_region: Arbitrary::arbitrary(_g),
                cost_matrix: Vec::<::uxas::messages::task::task_option_cost::TaskOptionCost>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::task::task_option_cost::TaskOptionCostT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: AssignmentCostMatrix) -> Result<TestResult, Error> {
            use std::u16;
            if x.task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.cost_matrix.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AssignmentCostMatrix) -> Result<TestResult, Error> {
            use std::u16;
            if x.task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.cost_matrix.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AssignmentCostMatrix::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
