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
pub struct RendezvousTask {
    pub task_id: i64,
    pub label: Vec<u8>,
    pub eligible_entities: Vec<i64>,
    pub revisit_rate: f32,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub priority: u8,
    pub required: bool,
    pub number_of_participants: u8,
    pub location: Option<Box<::afrl::cmasi::location3d::Location3DT>>,
    pub heading: f32,
    pub multi_location_rendezvous: bool,
    pub rendezvous_states: Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>,
}

impl PartialEq for RendezvousTask {
    fn eq(&self, _other: &RendezvousTask) -> bool {
        true
        && &self.number_of_participants == &_other.number_of_participants
        && &self.location == &_other.location
        && &self.heading == &_other.heading
        && &self.multi_location_rendezvous == &_other.multi_location_rendezvous
        && &self.rendezvous_states == &_other.rendezvous_states

    }
}

impl LmcpSubscription for RendezvousTask {
    fn subscription() -> &'static str { "uxas.messages.task.RendezvousTask" }
}

impl Struct for RendezvousTask {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149757930721443840u64,
            version: 7,
            struct_ty: 2,
        }
    }
}

impl Lmcp for RendezvousTask {
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
            let writeb: usize = self.label.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.eligible_entities.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.revisit_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.parameters.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.priority.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.required.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.number_of_participants.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.heading.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.multi_location_rendezvous.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.rendezvous_states.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RendezvousTask, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RendezvousTask::struct_info() {
            let mut out: RendezvousTask = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.label = x;
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
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.revisit_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>, usize) = Lmcp::deser(r)?;
                out.parameters = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u8, usize) = Lmcp::deser(r)?;
                out.priority = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.required = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u8, usize) = Lmcp::deser(r)?;
                out.number_of_participants = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Option<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.location = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.heading = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.multi_location_rendezvous = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>, usize) = Lmcp::deser(r)?;
                out.rendezvous_states = x;
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
        size += self.label.size();
        size += self.eligible_entities.size();
        size += self.revisit_rate.size();
        size += self.parameters.size();
        size += self.priority.size();
        size += self.required.size();
        size += self.number_of_participants.size();
        size += self.location.size();
        size += self.heading.size();
        size += self.multi_location_rendezvous.size();
        size += self.rendezvous_states.size();

        size
    }
}

pub trait RendezvousTaskT: Debug + Send + ::afrl::cmasi::task::TaskT {
    fn as_uxas_messages_task_rendezvous_task(&self) -> Option<&RendezvousTask> { None }
    fn as_mut_uxas_messages_task_rendezvous_task(&mut self) -> Option<&mut RendezvousTask> { None }
    fn number_of_participants(&self) -> u8;
    fn number_of_participants_mut(&mut self) -> &mut u8;
    fn location(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn location_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn heading(&self) -> f32;
    fn heading_mut(&mut self) -> &mut f32;
    fn multi_location_rendezvous(&self) -> bool;
    fn multi_location_rendezvous_mut(&mut self) -> &mut bool;
    fn rendezvous_states(&self) -> &Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>;
    fn rendezvous_states_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>>;

}

impl Clone for Box<RendezvousTaskT> {
    fn clone(&self) -> Box<RendezvousTaskT> {
        if let Some(x) = RendezvousTaskT::as_uxas_messages_task_rendezvous_task(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RendezvousTaskT> {
    fn default() -> Box<RendezvousTaskT> { Box::new(RendezvousTask::default()) }
}

impl PartialEq for Box<RendezvousTaskT> {
    fn eq(&self, other: &Box<RendezvousTaskT>) -> bool {
        if let (Some(x), Some(y)) =
            (RendezvousTaskT::as_uxas_messages_task_rendezvous_task(self.as_ref()),
             RendezvousTaskT::as_uxas_messages_task_rendezvous_task(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RendezvousTaskT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RendezvousTaskT::as_uxas_messages_task_rendezvous_task(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RendezvousTaskT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RendezvousTask::struct_info() {
            let (x, readb) = RendezvousTask::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RendezvousTaskT::as_uxas_messages_task_rendezvous_task(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::task::TaskT for RendezvousTask {
    fn as_uxas_messages_task_rendezvous_task(&self) -> Option<&RendezvousTask> { Some(self) }
    fn as_mut_uxas_messages_task_rendezvous_task(&mut self) -> Option<&mut RendezvousTask> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn label(&self) -> &Vec<u8> { &self.label }
    fn label_mut(&mut self) -> &mut Vec<u8> { &mut self.label }
    fn eligible_entities(&self) -> &Vec<i64> { &self.eligible_entities }
    fn eligible_entities_mut(&mut self) -> &mut Vec<i64> { &mut self.eligible_entities }
    fn revisit_rate(&self) -> f32 { self.revisit_rate }
    fn revisit_rate_mut(&mut self) -> &mut f32 { &mut self.revisit_rate }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
    fn priority(&self) -> u8 { self.priority }
    fn priority_mut(&mut self) -> &mut u8 { &mut self.priority }
    fn required(&self) -> bool { self.required }
    fn required_mut(&mut self) -> &mut bool { &mut self.required }
}
impl RendezvousTaskT for RendezvousTask {
    fn as_uxas_messages_task_rendezvous_task(&self) -> Option<&RendezvousTask> { Some(self) }
    fn as_mut_uxas_messages_task_rendezvous_task(&mut self) -> Option<&mut RendezvousTask> { Some(self) }
    fn number_of_participants(&self) -> u8 { self.number_of_participants }
    fn number_of_participants_mut(&mut self) -> &mut u8 { &mut self.number_of_participants }
    fn location(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>> { &self.location }
    fn location_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.location }
    fn heading(&self) -> f32 { self.heading }
    fn heading_mut(&mut self) -> &mut f32 { &mut self.heading }
    fn multi_location_rendezvous(&self) -> bool { self.multi_location_rendezvous }
    fn multi_location_rendezvous_mut(&mut self) -> &mut bool { &mut self.multi_location_rendezvous }
    fn rendezvous_states(&self) -> &Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>> { &self.rendezvous_states }
    fn rendezvous_states_mut(&mut self) -> &mut Vec<Box<::uxas::messages::task::planning_state::PlanningStateT>> { &mut self.rendezvous_states }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RendezvousTask {
        fn arbitrary<G: Gen>(_g: &mut G) -> RendezvousTask {
            RendezvousTask {
                task_id: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                eligible_entities: Arbitrary::arbitrary(_g),
                revisit_rate: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                priority: Arbitrary::arbitrary(_g),
                required: Arbitrary::arbitrary(_g),
                number_of_participants: Arbitrary::arbitrary(_g),
                location: {
                    if _g.gen() {
                        Some(Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)))
                    } else {
                        None
                    }
                },
                heading: Arbitrary::arbitrary(_g),
                multi_location_rendezvous: Arbitrary::arbitrary(_g),
                rendezvous_states: Vec::<::uxas::messages::task::planning_state::PlanningState>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::task::planning_state::PlanningStateT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RendezvousTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.rendezvous_states.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RendezvousTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.rendezvous_states.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RendezvousTask::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
