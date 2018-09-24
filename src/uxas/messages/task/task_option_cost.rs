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

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TaskOptionCost {
    pub vehicle_id: i64,
    pub intial_task_id: i64,
    pub intial_task_option: i64,
    pub destination_task_id: i64,
    pub destination_task_option: i64,
    pub time_to_go: i64,
}

impl PartialEq for TaskOptionCost {
    fn eq(&self, _other: &TaskOptionCost) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id
        && &self.intial_task_id == &_other.intial_task_id
        && &self.intial_task_option == &_other.intial_task_option
        && &self.destination_task_id == &_other.destination_task_id
        && &self.destination_task_option == &_other.destination_task_option
        && &self.time_to_go == &_other.time_to_go

    }
}

impl LmcpSubscription for TaskOptionCost {
    fn subscription() -> &'static str { "uxas.messages.task.TaskOptionCost" }
}

impl Struct for TaskOptionCost {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 17,
        }
    }
}

impl Lmcp for TaskOptionCost {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.intial_task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.intial_task_option.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.destination_task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.destination_task_option.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_to_go.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskOptionCost, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskOptionCost::struct_info() {
            let mut out: TaskOptionCost = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.intial_task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.intial_task_option = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.destination_task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.destination_task_option = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time_to_go = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.vehicle_id.size();
        size += self.intial_task_id.size();
        size += self.intial_task_option.size();
        size += self.destination_task_id.size();
        size += self.destination_task_option.size();
        size += self.time_to_go.size();

        size
    }
}

pub trait TaskOptionCostT: Debug + Send  {
    fn as_uxas_messages_task_task_option_cost(&self) -> Option<&TaskOptionCost> { None }
    fn as_mut_uxas_messages_task_task_option_cost(&mut self) -> Option<&mut TaskOptionCost> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn intial_task_id(&self) -> i64;
    fn intial_task_id_mut(&mut self) -> &mut i64;
    fn intial_task_option(&self) -> i64;
    fn intial_task_option_mut(&mut self) -> &mut i64;
    fn destination_task_id(&self) -> i64;
    fn destination_task_id_mut(&mut self) -> &mut i64;
    fn destination_task_option(&self) -> i64;
    fn destination_task_option_mut(&mut self) -> &mut i64;
    fn time_to_go(&self) -> i64;
    fn time_to_go_mut(&mut self) -> &mut i64;

}

impl Clone for Box<TaskOptionCostT> {
    fn clone(&self) -> Box<TaskOptionCostT> {
        if let Some(x) = TaskOptionCostT::as_uxas_messages_task_task_option_cost(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskOptionCostT> {
    fn default() -> Box<TaskOptionCostT> { Box::new(TaskOptionCost::default()) }
}

impl PartialEq for Box<TaskOptionCostT> {
    fn eq(&self, other: &Box<TaskOptionCostT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskOptionCostT::as_uxas_messages_task_task_option_cost(self.as_ref()),
             TaskOptionCostT::as_uxas_messages_task_task_option_cost(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskOptionCostT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskOptionCostT::as_uxas_messages_task_task_option_cost(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskOptionCostT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskOptionCost::struct_info() {
            let (x, readb) = TaskOptionCost::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskOptionCostT::as_uxas_messages_task_task_option_cost(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskOptionCostT for TaskOptionCost {
    fn as_uxas_messages_task_task_option_cost(&self) -> Option<&TaskOptionCost> { Some(self) }
    fn as_mut_uxas_messages_task_task_option_cost(&mut self) -> Option<&mut TaskOptionCost> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn intial_task_id(&self) -> i64 { self.intial_task_id }
    fn intial_task_id_mut(&mut self) -> &mut i64 { &mut self.intial_task_id }
    fn intial_task_option(&self) -> i64 { self.intial_task_option }
    fn intial_task_option_mut(&mut self) -> &mut i64 { &mut self.intial_task_option }
    fn destination_task_id(&self) -> i64 { self.destination_task_id }
    fn destination_task_id_mut(&mut self) -> &mut i64 { &mut self.destination_task_id }
    fn destination_task_option(&self) -> i64 { self.destination_task_option }
    fn destination_task_option_mut(&mut self) -> &mut i64 { &mut self.destination_task_option }
    fn time_to_go(&self) -> i64 { self.time_to_go }
    fn time_to_go_mut(&mut self) -> &mut i64 { &mut self.time_to_go }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskOptionCost {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskOptionCost {
            TaskOptionCost {
                vehicle_id: Arbitrary::arbitrary(_g),
                intial_task_id: Arbitrary::arbitrary(_g),
                intial_task_option: Arbitrary::arbitrary(_g),
                destination_task_id: Arbitrary::arbitrary(_g),
                destination_task_option: Arbitrary::arbitrary(_g),
                time_to_go: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskOptionCost) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskOptionCost) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskOptionCost::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
