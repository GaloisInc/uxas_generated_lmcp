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
pub struct SubTaskExecution {
    pub sub_tasks: Vec<Box<::afrl::cmasi::task::TaskT>>,
    pub strict_order: bool,
}

impl PartialEq for SubTaskExecution {
    fn eq(&self, _other: &SubTaskExecution) -> bool {
        true
        && &self.sub_tasks == &_other.sub_tasks
        && &self.strict_order == &_other.strict_order

    }
}

impl LmcpSubscription for SubTaskExecution {
    fn subscription() -> &'static str { "uxas.messages.uxnative.SubTaskExecution" }
}

impl Struct for SubTaskExecution {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 10,
        }
    }
}

impl Lmcp for SubTaskExecution {
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
            let writeb: usize = self.strict_order.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SubTaskExecution, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SubTaskExecution::struct_info() {
            let mut out: SubTaskExecution = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::task::TaskT>>, usize) = Lmcp::deser(r)?;
                out.sub_tasks = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.strict_order = x;
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
        size += self.strict_order.size();

        size
    }
}

pub trait SubTaskExecutionT: Debug + Send  {
    fn as_uxas_messages_uxnative_sub_task_execution(&self) -> Option<&SubTaskExecution> { None }
    fn as_mut_uxas_messages_uxnative_sub_task_execution(&mut self) -> Option<&mut SubTaskExecution> { None }
    fn sub_tasks(&self) -> &Vec<Box<::afrl::cmasi::task::TaskT>>;
    fn sub_tasks_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::task::TaskT>>;
    fn strict_order(&self) -> bool;
    fn strict_order_mut(&mut self) -> &mut bool;

}

impl Clone for Box<SubTaskExecutionT> {
    fn clone(&self) -> Box<SubTaskExecutionT> {
        if let Some(x) = SubTaskExecutionT::as_uxas_messages_uxnative_sub_task_execution(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SubTaskExecutionT> {
    fn default() -> Box<SubTaskExecutionT> { Box::new(SubTaskExecution::default()) }
}

impl PartialEq for Box<SubTaskExecutionT> {
    fn eq(&self, other: &Box<SubTaskExecutionT>) -> bool {
        if let (Some(x), Some(y)) =
            (SubTaskExecutionT::as_uxas_messages_uxnative_sub_task_execution(self.as_ref()),
             SubTaskExecutionT::as_uxas_messages_uxnative_sub_task_execution(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SubTaskExecutionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SubTaskExecutionT::as_uxas_messages_uxnative_sub_task_execution(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SubTaskExecutionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SubTaskExecution::struct_info() {
            let (x, readb) = SubTaskExecution::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SubTaskExecutionT::as_uxas_messages_uxnative_sub_task_execution(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl SubTaskExecutionT for SubTaskExecution {
    fn as_uxas_messages_uxnative_sub_task_execution(&self) -> Option<&SubTaskExecution> { Some(self) }
    fn as_mut_uxas_messages_uxnative_sub_task_execution(&mut self) -> Option<&mut SubTaskExecution> { Some(self) }
    fn sub_tasks(&self) -> &Vec<Box<::afrl::cmasi::task::TaskT>> { &self.sub_tasks }
    fn sub_tasks_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::task::TaskT>> { &mut self.sub_tasks }
    fn strict_order(&self) -> bool { self.strict_order }
    fn strict_order_mut(&mut self) -> &mut bool { &mut self.strict_order }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SubTaskExecution {
        fn arbitrary<G: Gen>(_g: &mut G) -> SubTaskExecution {
            SubTaskExecution {
                sub_tasks: Vec::<::afrl::cmasi::task::Task>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::task::TaskT>).collect(),
                strict_order: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SubTaskExecution) -> Result<TestResult, Error> {
            use std::u16;
            if x.sub_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SubTaskExecution) -> Result<TestResult, Error> {
            use std::u16;
            if x.sub_tasks.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SubTaskExecution::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
