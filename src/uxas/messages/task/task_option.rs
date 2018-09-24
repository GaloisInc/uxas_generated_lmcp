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
pub struct TaskOption {
    pub task_id: i64,
    pub option_id: i64,
    pub eligible_entities: Vec<i64>,
    pub cost: i64,
    pub start_location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub start_heading: f32,
    pub end_location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub end_heading: f32,
}

impl PartialEq for TaskOption {
    fn eq(&self, _other: &TaskOption) -> bool {
        true
        && &self.task_id == &_other.task_id
        && &self.option_id == &_other.option_id
        && &self.eligible_entities == &_other.eligible_entities
        && &self.cost == &_other.cost
        && &self.start_location == &_other.start_location
        && &self.start_heading == &_other.start_heading
        && &self.end_location == &_other.end_location
        && &self.end_heading == &_other.end_heading

    }
}

impl LmcpSubscription for TaskOption {
    fn subscription() -> &'static str { "uxas.messages.task.TaskOption" }
}

impl Struct for TaskOption {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 20,
        }
    }
}

impl Lmcp for TaskOption {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.option_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.eligible_entities.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.cost.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_heading.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.end_location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.end_heading.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TaskOption, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TaskOption::struct_info() {
            let mut out: TaskOption = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.option_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.eligible_entities = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.cost = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.start_location = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.start_heading = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.end_location = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.end_heading = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.task_id.size();
        size += self.option_id.size();
        size += self.eligible_entities.size();
        size += self.cost.size();
        size += self.start_location.size();
        size += self.start_heading.size();
        size += self.end_location.size();
        size += self.end_heading.size();

        size
    }
}

pub trait TaskOptionT: Debug + Send  {
    fn as_uxas_messages_task_task_option(&self) -> Option<&TaskOption> { None }
    fn as_mut_uxas_messages_task_task_option(&mut self) -> Option<&mut TaskOption> { None }
    fn task_id(&self) -> i64;
    fn task_id_mut(&mut self) -> &mut i64;
    fn option_id(&self) -> i64;
    fn option_id_mut(&mut self) -> &mut i64;
    fn eligible_entities(&self) -> &Vec<i64>;
    fn eligible_entities_mut(&mut self) -> &mut Vec<i64>;
    fn cost(&self) -> i64;
    fn cost_mut(&mut self) -> &mut i64;
    fn start_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn start_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn start_heading(&self) -> f32;
    fn start_heading_mut(&mut self) -> &mut f32;
    fn end_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn end_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn end_heading(&self) -> f32;
    fn end_heading_mut(&mut self) -> &mut f32;

}

impl Clone for Box<TaskOptionT> {
    fn clone(&self) -> Box<TaskOptionT> {
        if let Some(x) = TaskOptionT::as_uxas_messages_task_task_option(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TaskOptionT> {
    fn default() -> Box<TaskOptionT> { Box::new(TaskOption::default()) }
}

impl PartialEq for Box<TaskOptionT> {
    fn eq(&self, other: &Box<TaskOptionT>) -> bool {
        if let (Some(x), Some(y)) =
            (TaskOptionT::as_uxas_messages_task_task_option(self.as_ref()),
             TaskOptionT::as_uxas_messages_task_task_option(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TaskOptionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TaskOptionT::as_uxas_messages_task_task_option(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TaskOptionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TaskOption::struct_info() {
            let (x, readb) = TaskOption::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TaskOptionT::as_uxas_messages_task_task_option(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl TaskOptionT for TaskOption {
    fn as_uxas_messages_task_task_option(&self) -> Option<&TaskOption> { Some(self) }
    fn as_mut_uxas_messages_task_task_option(&mut self) -> Option<&mut TaskOption> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn option_id(&self) -> i64 { self.option_id }
    fn option_id_mut(&mut self) -> &mut i64 { &mut self.option_id }
    fn eligible_entities(&self) -> &Vec<i64> { &self.eligible_entities }
    fn eligible_entities_mut(&mut self) -> &mut Vec<i64> { &mut self.eligible_entities }
    fn cost(&self) -> i64 { self.cost }
    fn cost_mut(&mut self) -> &mut i64 { &mut self.cost }
    fn start_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.start_location }
    fn start_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.start_location }
    fn start_heading(&self) -> f32 { self.start_heading }
    fn start_heading_mut(&mut self) -> &mut f32 { &mut self.start_heading }
    fn end_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.end_location }
    fn end_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.end_location }
    fn end_heading(&self) -> f32 { self.end_heading }
    fn end_heading_mut(&mut self) -> &mut f32 { &mut self.end_heading }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TaskOption {
        fn arbitrary<G: Gen>(_g: &mut G) -> TaskOption {
            TaskOption {
                task_id: Arbitrary::arbitrary(_g),
                option_id: Arbitrary::arbitrary(_g),
                eligible_entities: Arbitrary::arbitrary(_g),
                cost: Arbitrary::arbitrary(_g),
                start_location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                start_heading: Arbitrary::arbitrary(_g),
                end_location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                end_heading: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TaskOption) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TaskOption) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TaskOption::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
