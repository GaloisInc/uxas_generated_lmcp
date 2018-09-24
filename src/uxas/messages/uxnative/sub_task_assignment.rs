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
pub struct SubTaskAssignment {
    pub sub_tasks: Vec<Box<::afrl::cmasi::task::TaskT>>,
    pub neighbors: Vec<Box<::afrl::cmasi::entity_state::EntityStateT>>,
}

impl PartialEq for SubTaskAssignment {
    fn eq(&self, _other: &SubTaskAssignment) -> bool {
        true
        && &self.sub_tasks == &_other.sub_tasks
        && &self.neighbors == &_other.neighbors

    }
}

impl LmcpSubscription for SubTaskAssignment {
    fn subscription() -> &'static str { "uxas.messages.uxnative.SubTaskAssignment" }
}

impl Struct for SubTaskAssignment {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 11,
        }
    }
}

impl Lmcp for SubTaskAssignment {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.sub_tasks.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.neighbors.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SubTaskAssignment, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SubTaskAssignment::struct_info() {
            let mut out: SubTaskAssignment = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::task::TaskT>>, usize) = Lmcp::deser(r)?;
                out.sub_tasks = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::entity_state::EntityStateT>>, usize) = Lmcp::deser(r)?;
                out.neighbors = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.sub_tasks.size();
        size += self.neighbors.size();

        size
    }
}

pub trait SubTaskAssignmentT: Debug + Send  {
    fn as_uxas_messages_uxnative_sub_task_assignment(&self) -> Option<&SubTaskAssignment> { None }
    fn as_mut_uxas_messages_uxnative_sub_task_assignment(&mut self) -> Option<&mut SubTaskAssignment> { None }
    fn sub_tasks(&self) -> &Vec<Box<::afrl::cmasi::task::TaskT>>;
    fn sub_tasks_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::task::TaskT>>;
    fn neighbors(&self) -> &Vec<Box<::afrl::cmasi::entity_state::EntityStateT>>;
    fn neighbors_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::entity_state::EntityStateT>>;

}

impl Clone for Box<SubTaskAssignmentT> {
    fn clone(&self) -> Box<SubTaskAssignmentT> {
        if let Some(x) = SubTaskAssignmentT::as_uxas_messages_uxnative_sub_task_assignment(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SubTaskAssignmentT> {
    fn default() -> Box<SubTaskAssignmentT> { Box::new(SubTaskAssignment::default()) }
}

impl PartialEq for Box<SubTaskAssignmentT> {
    fn eq(&self, other: &Box<SubTaskAssignmentT>) -> bool {
        if let (Some(x), Some(y)) =
            (SubTaskAssignmentT::as_uxas_messages_uxnative_sub_task_assignment(self.as_ref()),
             SubTaskAssignmentT::as_uxas_messages_uxnative_sub_task_assignment(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SubTaskAssignmentT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SubTaskAssignmentT::as_uxas_messages_uxnative_sub_task_assignment(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SubTaskAssignmentT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SubTaskAssignment::struct_info() {
            let (x, readb) = SubTaskAssignment::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SubTaskAssignmentT::as_uxas_messages_uxnative_sub_task_assignment(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl SubTaskAssignmentT for SubTaskAssignment {
    fn as_uxas_messages_uxnative_sub_task_assignment(&self) -> Option<&SubTaskAssignment> { Some(self) }
    fn as_mut_uxas_messages_uxnative_sub_task_assignment(&mut self) -> Option<&mut SubTaskAssignment> { Some(self) }
    fn sub_tasks(&self) -> &Vec<Box<::afrl::cmasi::task::TaskT>> { &self.sub_tasks }
    fn sub_tasks_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::task::TaskT>> { &mut self.sub_tasks }
    fn neighbors(&self) -> &Vec<Box<::afrl::cmasi::entity_state::EntityStateT>> { &self.neighbors }
    fn neighbors_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::entity_state::EntityStateT>> { &mut self.neighbors }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SubTaskAssignment {
        fn arbitrary<G: Gen>(_g: &mut G) -> SubTaskAssignment {
            SubTaskAssignment {
                sub_tasks: Vec::<::afrl::cmasi::task::Task>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::task::TaskT>).collect(),
                neighbors: Vec::<::afrl::cmasi::entity_state::EntityState>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::entity_state::EntityStateT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SubTaskAssignment) -> Result<TestResult, Error> {
            use std::u16;
            if x.sub_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.neighbors.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SubTaskAssignment) -> Result<TestResult, Error> {
            use std::u16;
            if x.sub_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.neighbors.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SubTaskAssignment::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
