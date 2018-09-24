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
pub struct TaskPlanOptions {
    pub corresponding_automation_request_id: i64,
    pub task_id: i64,
    pub composition: Vec<u8>,
    pub options: Vec<Box<::uxas::messages::task::task_option::TaskOptionT>>,
}

impl PartialEq for TaskPlanOptions {
    fn eq(&self, _other: &TaskPlanOptions) -> bool {
        true
        && &self.corresponding_automation_request_id == &_other.corresponding_automation_request_id
        && &self.task_id == &_other.task_id
        && &self.composition == &_other.composition
        && &self.options == &_other.options

    }
}

impl LmcpSubscription for TaskPlanOptions {
    fn subscription() -> &'static str { "uxas.messages.task.TaskPlanOptions" }
}

impl Struct for TaskPlanOptions {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 21,
        }
    }
}

impl Lmcp for TaskPlanOptions {
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
            let writeb: usize = self.task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.composition.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.options.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskPlanOptions, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskPlanOptions::struct_info() {
            let mut out: TaskPlanOptions = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.corresponding_automation_request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.composition = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::task::task_option::TaskOptionT>>, usize) = Lmcp::deser(r)?;
                out.options = x;
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
        size += self.task_id.size();
        size += self.composition.size();
        size += self.options.size();

        size
    }
}

pub trait TaskPlanOptionsT: Debug + Send  {
    fn as_uxas_messages_task_task_plan_options(&self) -> Option<&TaskPlanOptions> { None }
    fn as_mut_uxas_messages_task_task_plan_options(&mut self) -> Option<&mut TaskPlanOptions> { None }
    fn corresponding_automation_request_id(&self) -> i64;
    fn corresponding_automation_request_id_mut(&mut self) -> &mut i64;
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn composition(&self) -> &Vec<u8>;
    fn composition_mut(&mut self) -> &mut Vec<u8>;
    fn options(&self) -> &Vec<Box<::uxas::messages::task::task_option::TaskOptionT>>;
    fn options_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::task_option::TaskOptionT>>;

}

impl Clone for Box<TaskPlanOptionsT> {
    fn clone(&self) -> Box<TaskPlanOptionsT> {
        if let Some(x) = TaskPlanOptionsT::as_uxas_messages_task_task_plan_options(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskPlanOptionsT> {
    fn default() -> Box<TaskPlanOptionsT> { Box::new(TaskPlanOptions::default()) }
}

impl PartialEq for Box<TaskPlanOptionsT> {
    fn eq(&self, other: &Box<TaskPlanOptionsT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskPlanOptionsT::as_uxas_messages_task_task_plan_options(self.as_ref()),
             TaskPlanOptionsT::as_uxas_messages_task_task_plan_options(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskPlanOptionsT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskPlanOptionsT::as_uxas_messages_task_task_plan_options(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskPlanOptionsT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskPlanOptions::struct_info() {
            let (x, readb) = TaskPlanOptions::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskPlanOptionsT::as_uxas_messages_task_task_plan_options(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskPlanOptionsT for TaskPlanOptions {
    fn as_uxas_messages_task_task_plan_options(&self) -> Option<&TaskPlanOptions> { Some(self) }
    fn as_mut_uxas_messages_task_task_plan_options(&mut self) -> Option<&mut TaskPlanOptions> { Some(self) }
    fn corresponding_automation_request_id(&self) -> i64 { self.corresponding_automation_request_id }
    fn corresponding_automation_request_id_mut(&mut self) -> &mut i64 { &mut self.corresponding_automation_request_id }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn composition(&self) -> &Vec<u8> { &self.composition }
    fn composition_mut(&mut self) -> &mut Vec<u8> { &mut self.composition }
    fn options(&self) -> &Vec<Box<::uxas::messages::task::task_option::TaskOptionT>> { &self.options }
    fn options_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::task_option::TaskOptionT>> { &mut self.options }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskPlanOptions {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskPlanOptions {
            TaskPlanOptions {
                corresponding_automation_request_id: Arbitrary::arbitrary(_g),
                task_id: Arbitrary::arbitrary(_g),
                composition: Arbitrary::arbitrary(_g),
                options: Vec::<::uxas::messages::task::task_option::TaskOption>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::task::task_option::TaskOptionT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskPlanOptions) -> Result<TestResult, Error> {
            use std::u16;
            if x.options.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskPlanOptions) -> Result<TestResult, Error> {
            use std::u16;
            if x.options.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskPlanOptions::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
