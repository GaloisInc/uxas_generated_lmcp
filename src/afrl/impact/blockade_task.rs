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
pub struct BlockadeTask {
    pub task_id: i64,
    pub label: Vec<u8>,
    pub eligible_entities: Vec<i64>,
    pub revisit_rate: f32,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub priority: u8,
    pub required: bool,
    pub blocked_entity_id: i64,
    pub standoff_distance: f32,
    pub number_vehicles: u8,
    pub protected_location: Option<Box<::afrl::cmasi::location3d::Location3DT>>,
}

impl PartialEq for BlockadeTask {
    fn eq(&self, _other: &BlockadeTask) -> bool {
        true
        && &self.blocked_entity_id == &_other.blocked_entity_id
        && &self.standoff_distance == &_other.standoff_distance
        && &self.number_vehicles == &_other.number_vehicles
        && &self.protected_location == &_other.protected_location

    }
}

impl LmcpSubscription for BlockadeTask {
    fn subscription() -> &'static str { "afrl.impact.BlockadeTask" }
}

impl Struct for BlockadeTask {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 30,
        }
    }
}

impl Lmcp for BlockadeTask {
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
            let writeb: usize = self.blocked_entity_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.standoff_distance.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.number_vehicles.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.protected_location.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(BlockadeTask, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == BlockadeTask::struct_info() {
            let mut out: BlockadeTask = Default::default();
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
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.blocked_entity_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.standoff_distance = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u8, usize) = Lmcp::deser(r)?;
                out.number_vehicles = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Option<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.protected_location = x;
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
        size += self.blocked_entity_id.size();
        size += self.standoff_distance.size();
        size += self.number_vehicles.size();
        size += self.protected_location.size();

        size
    }
}

pub trait BlockadeTaskT: Debug + Send + ::afrl::cmasi::task::TaskT {
    fn as_afrl_impact_blockade_task(&self) -> Option<&BlockadeTask> { None }
    fn as_mut_afrl_impact_blockade_task(&mut self) -> Option<&mut BlockadeTask> { None }
    fn blocked_entity_id(&self) -> i64;
    fn blocked_entity_id_mut(&mut self) -> &mut i64;
    fn standoff_distance(&self) -> f32;
    fn standoff_distance_mut(&mut self) -> &mut f32;
    fn number_vehicles(&self) -> u8;
    fn number_vehicles_mut(&mut self) -> &mut u8;
    fn protected_location(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn protected_location_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>>;

}

impl Clone for Box<BlockadeTaskT> {
    fn clone(&self) -> Box<BlockadeTaskT> {
        if let Some(x) = BlockadeTaskT::as_afrl_impact_blockade_task(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<BlockadeTaskT> {
    fn default() -> Box<BlockadeTaskT> { Box::new(BlockadeTask::default()) }
}

impl PartialEq for Box<BlockadeTaskT> {
    fn eq(&self, other: &Box<BlockadeTaskT>) -> bool {
        if let (Some(x), Some(y)) =
            (BlockadeTaskT::as_afrl_impact_blockade_task(self.as_ref()),
             BlockadeTaskT::as_afrl_impact_blockade_task(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<BlockadeTaskT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = BlockadeTaskT::as_afrl_impact_blockade_task(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<BlockadeTaskT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == BlockadeTask::struct_info() {
            let (x, readb) = BlockadeTask::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = BlockadeTaskT::as_afrl_impact_blockade_task(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::task::TaskT for BlockadeTask {
    fn as_afrl_impact_blockade_task(&self) -> Option<&BlockadeTask> { Some(self) }
    fn as_mut_afrl_impact_blockade_task(&mut self) -> Option<&mut BlockadeTask> { Some(self) }
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
impl BlockadeTaskT for BlockadeTask {
    fn as_afrl_impact_blockade_task(&self) -> Option<&BlockadeTask> { Some(self) }
    fn as_mut_afrl_impact_blockade_task(&mut self) -> Option<&mut BlockadeTask> { Some(self) }
    fn blocked_entity_id(&self) -> i64 { self.blocked_entity_id }
    fn blocked_entity_id_mut(&mut self) -> &mut i64 { &mut self.blocked_entity_id }
    fn standoff_distance(&self) -> f32 { self.standoff_distance }
    fn standoff_distance_mut(&mut self) -> &mut f32 { &mut self.standoff_distance }
    fn number_vehicles(&self) -> u8 { self.number_vehicles }
    fn number_vehicles_mut(&mut self) -> &mut u8 { &mut self.number_vehicles }
    fn protected_location(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>> { &self.protected_location }
    fn protected_location_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.protected_location }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for BlockadeTask {
        fn arbitrary<G: Gen>(_g: &mut G) -> BlockadeTask {
            BlockadeTask {
                task_id: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                eligible_entities: Arbitrary::arbitrary(_g),
                revisit_rate: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                priority: Arbitrary::arbitrary(_g),
                required: Arbitrary::arbitrary(_g),
                blocked_entity_id: Arbitrary::arbitrary(_g),
                standoff_distance: Arbitrary::arbitrary(_g),
                number_vehicles: Arbitrary::arbitrary(_g),
                protected_location: {
                    if _g.gen() {
                        Some(Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)))
                    } else {
                        None
                    }
                },

            }
        }
    }

    quickcheck! {
        fn serializes(x: BlockadeTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: BlockadeTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = BlockadeTask::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
