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
pub struct RemoveTasks {
    pub task_list: Vec<i64>,
}

impl PartialEq for RemoveTasks {
    fn eq(&self, _other: &RemoveTasks) -> bool {
        true
        && &self.task_list == &_other.task_list

    }
}

impl LmcpSubscription for RemoveTasks {
    fn subscription() -> &'static str { "afrl.cmasi.RemoveTasks" }
}

impl Struct for RemoveTasks {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 44,
        }
    }
}

impl Lmcp for RemoveTasks {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_list.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RemoveTasks, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RemoveTasks::struct_info() {
            let mut out: RemoveTasks = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.task_list = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.task_list.size();

        size
    }
}

pub trait RemoveTasksT: Debug + Send  {
    fn as_afrl_cmasi_remove_tasks(&self) -> Option<&RemoveTasks> { None }
    fn as_mut_afrl_cmasi_remove_tasks(&mut self) -> Option<&mut RemoveTasks> { None }
    fn task_list(&self) -> &Vec<i64>;
    fn task_list_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<RemoveTasksT> {
    fn clone(&self) -> Box<RemoveTasksT> {
        if let Some(x) = RemoveTasksT::as_afrl_cmasi_remove_tasks(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RemoveTasksT> {
    fn default() -> Box<RemoveTasksT> { Box::new(RemoveTasks::default()) }
}

impl PartialEq for Box<RemoveTasksT> {
    fn eq(&self, other: &Box<RemoveTasksT>) -> bool {
        if let (Some(x), Some(y)) =
            (RemoveTasksT::as_afrl_cmasi_remove_tasks(self.as_ref()),
             RemoveTasksT::as_afrl_cmasi_remove_tasks(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RemoveTasksT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RemoveTasksT::as_afrl_cmasi_remove_tasks(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RemoveTasksT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RemoveTasks::struct_info() {
            let (x, readb) = RemoveTasks::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RemoveTasksT::as_afrl_cmasi_remove_tasks(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RemoveTasksT for RemoveTasks {
    fn as_afrl_cmasi_remove_tasks(&self) -> Option<&RemoveTasks> { Some(self) }
    fn as_mut_afrl_cmasi_remove_tasks(&mut self) -> Option<&mut RemoveTasks> { Some(self) }
    fn task_list(&self) -> &Vec<i64> { &self.task_list }
    fn task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.task_list }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RemoveTasks {
        fn arbitrary<G: Gen>(_g: &mut G) -> RemoveTasks {
            RemoveTasks {
                task_list: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RemoveTasks) -> Result<TestResult, Error> {
            use std::u16;
            if x.task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RemoveTasks) -> Result<TestResult, Error> {
            use std::u16;
            if x.task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RemoveTasks::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
