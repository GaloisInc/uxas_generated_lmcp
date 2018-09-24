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
pub struct AssignmentCoordination {
    pub coordinated_automation_request_id: i64,
    pub planning_state: Box<::uxas::messages::task::planning_state::PlanningStateT>,
}

impl PartialEq for AssignmentCoordination {
    fn eq(&self, _other: &AssignmentCoordination) -> bool {
        true
        && &self.coordinated_automation_request_id == &_other.coordinated_automation_request_id
        && &self.planning_state == &_other.planning_state

    }
}

impl LmcpSubscription for AssignmentCoordination {
    fn subscription() -> &'static str { "uxas.messages.task.AssignmentCoordination" }
}

impl Struct for AssignmentCoordination {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 4,
        }
    }
}

impl Lmcp for AssignmentCoordination {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.coordinated_automation_request_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.planning_state.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(AssignmentCoordination, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == AssignmentCoordination::struct_info() {
            let mut out: AssignmentCoordination = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.coordinated_automation_request_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::uxas::messages::task::planning_state::PlanningStateT>, usize) = Lmcp::deser(r)?;
                out.planning_state = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.coordinated_automation_request_id.size();
        size += self.planning_state.size();

        size
    }
}

pub trait AssignmentCoordinationT: Debug + Send  {
    fn as_uxas_messages_task_assignment_coordination(&self) -> Option<&AssignmentCoordination> { None }
    fn as_mut_uxas_messages_task_assignment_coordination(&mut self) -> Option<&mut AssignmentCoordination> { None }
    fn coordinated_automation_request_id(&self) -> i64;
    fn coordinated_automation_request_id_mut(&mut self) -> &mut i64;
    fn planning_state(&self) -> &Box<::uxas::messages::task::planning_state::PlanningStateT>;
    fn planning_state_mut(&mut self) -> &mut Box<::uxas::messages::task::planning_state::PlanningStateT>;

}

impl Clone for Box<AssignmentCoordinationT> {
    fn clone(&self) -> Box<AssignmentCoordinationT> {
        if let Some(x) = AssignmentCoordinationT::as_uxas_messages_task_assignment_coordination(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<AssignmentCoordinationT> {
    fn default() -> Box<AssignmentCoordinationT> { Box::new(AssignmentCoordination::default()) }
}

impl PartialEq for Box<AssignmentCoordinationT> {
    fn eq(&self, other: &Box<AssignmentCoordinationT>) -> bool {
        if let (Some(x), Some(y)) =
            (AssignmentCoordinationT::as_uxas_messages_task_assignment_coordination(self.as_ref()),
             AssignmentCoordinationT::as_uxas_messages_task_assignment_coordination(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<AssignmentCoordinationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = AssignmentCoordinationT::as_uxas_messages_task_assignment_coordination(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<AssignmentCoordinationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == AssignmentCoordination::struct_info() {
            let (x, readb) = AssignmentCoordination::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = AssignmentCoordinationT::as_uxas_messages_task_assignment_coordination(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl AssignmentCoordinationT for AssignmentCoordination {
    fn as_uxas_messages_task_assignment_coordination(&self) -> Option<&AssignmentCoordination> { Some(self) }
    fn as_mut_uxas_messages_task_assignment_coordination(&mut self) -> Option<&mut AssignmentCoordination> { Some(self) }
    fn coordinated_automation_request_id(&self) -> i64 { self.coordinated_automation_request_id }
    fn coordinated_automation_request_id_mut(&mut self) -> &mut i64 { &mut self.coordinated_automation_request_id }
    fn planning_state(&self) -> &Box<::uxas::messages::task::planning_state::PlanningStateT> { &self.planning_state }
    fn planning_state_mut(&mut self) -> &mut Box<::uxas::messages::task::planning_state::PlanningStateT> { &mut self.planning_state }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AssignmentCoordination {
        fn arbitrary<G: Gen>(_g: &mut G) -> AssignmentCoordination {
            AssignmentCoordination {
                coordinated_automation_request_id: Arbitrary::arbitrary(_g),
                planning_state: Box::new(::uxas::messages::task::planning_state::PlanningState::arbitrary(_g)),

            }
        }
    }

    quickcheck! {
        fn serializes(x: AssignmentCoordination) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AssignmentCoordination) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AssignmentCoordination::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
