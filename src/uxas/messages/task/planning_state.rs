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
pub struct PlanningState {
    pub entity_id: i64,
    pub planning_position: Box<::afrl::cmasi::location3d::Location3DT>,
    pub planning_heading: f32,
}

impl PartialEq for PlanningState {
    fn eq(&self, _other: &PlanningState) -> bool {
        true
        && &self.entity_id == &_other.entity_id
        && &self.planning_position == &_other.planning_position
        && &self.planning_heading == &_other.planning_heading

    }
}

impl LmcpSubscription for PlanningState {
    fn subscription() -> &'static str { "uxas.messages.task.PlanningState" }
}

impl Struct for PlanningState {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 3,
        }
    }
}

impl Lmcp for PlanningState {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entity_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.planning_position.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.planning_heading.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PlanningState, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PlanningState::struct_info() {
            let mut out: PlanningState = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.entity_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.planning_position = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.planning_heading = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.entity_id.size();
        size += self.planning_position.size();
        size += self.planning_heading.size();

        size
    }
}

pub trait PlanningStateT: Debug + Send  {
    fn as_uxas_messages_task_planning_state(&self) -> Option<&PlanningState> { None }
    fn as_mut_uxas_messages_task_planning_state(&mut self) -> Option<&mut PlanningState> { None }
    fn entity_id(&self) -> i64;
    fn entity_id_mut(&mut self) -> &mut i64;
    fn planning_position(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn planning_position_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn planning_heading(&self) -> f32;
    fn planning_heading_mut(&mut self) -> &mut f32;

}

impl Clone for Box<PlanningStateT> {
    fn clone(&self) -> Box<PlanningStateT> {
        if let Some(x) = PlanningStateT::as_uxas_messages_task_planning_state(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PlanningStateT> {
    fn default() -> Box<PlanningStateT> { Box::new(PlanningState::default()) }
}

impl PartialEq for Box<PlanningStateT> {
    fn eq(&self, other: &Box<PlanningStateT>) -> bool {
        if let (Some(x), Some(y)) =
            (PlanningStateT::as_uxas_messages_task_planning_state(self.as_ref()),
             PlanningStateT::as_uxas_messages_task_planning_state(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PlanningStateT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PlanningStateT::as_uxas_messages_task_planning_state(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PlanningStateT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PlanningState::struct_info() {
            let (x, readb) = PlanningState::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PlanningStateT::as_uxas_messages_task_planning_state(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl PlanningStateT for PlanningState {
    fn as_uxas_messages_task_planning_state(&self) -> Option<&PlanningState> { Some(self) }
    fn as_mut_uxas_messages_task_planning_state(&mut self) -> Option<&mut PlanningState> { Some(self) }
    fn entity_id(&self) -> i64 { self.entity_id }
    fn entity_id_mut(&mut self) -> &mut i64 { &mut self.entity_id }
    fn planning_position(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.planning_position }
    fn planning_position_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.planning_position }
    fn planning_heading(&self) -> f32 { self.planning_heading }
    fn planning_heading_mut(&mut self) -> &mut f32 { &mut self.planning_heading }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PlanningState {
        fn arbitrary<G: Gen>(_g: &mut G) -> PlanningState {
            PlanningState {
                entity_id: Arbitrary::arbitrary(_g),
                planning_position: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                planning_heading: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PlanningState) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PlanningState) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PlanningState::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
