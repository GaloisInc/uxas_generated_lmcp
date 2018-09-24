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
pub struct TrackEntityTask {
    pub task_id: i64,
    pub label: Vec<u8>,
    pub eligible_entities: Vec<i64>,
    pub revisit_rate: f32,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub priority: u8,
    pub required: bool,
    pub entity_id: u32,
    pub sensor_modality: ::afrl::cmasi::wavelength_band::WavelengthBand,
    pub ground_sample_distance: f32,
}

impl PartialEq for TrackEntityTask {
    fn eq(&self, _other: &TrackEntityTask) -> bool {
        true
        && &self.entity_id == &_other.entity_id
        && &self.sensor_modality == &_other.sensor_modality
        && &self.ground_sample_distance == &_other.ground_sample_distance

    }
}

impl LmcpSubscription for TrackEntityTask {
    fn subscription() -> &'static str { "afrl.cmasi.perceive.TrackEntityTask" }
}

impl Struct for TrackEntityTask {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5784119745305990725u64,
            version: 1,
            struct_ty: 3,
        }
    }
}

impl Lmcp for TrackEntityTask {
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
            let writeb: usize = self.entity_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.sensor_modality.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.ground_sample_distance.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(TrackEntityTask, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == TrackEntityTask::struct_info() {
            let mut out: TrackEntityTask = Default::default();
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
                let (x, readb): (u32, usize) = Lmcp::deser(r)?;
                out.entity_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::wavelength_band::WavelengthBand, usize) = Lmcp::deser(r)?;
                out.sensor_modality = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.ground_sample_distance = x;
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
        size += self.entity_id.size();
        size += self.sensor_modality.size();
        size += self.ground_sample_distance.size();

        size
    }
}

pub trait TrackEntityTaskT: Debug + Send + ::afrl::cmasi::task::TaskT {
    fn as_afrl_cmasi_perceive_track_entity_task(&self) -> Option<&TrackEntityTask> { None }
    fn as_mut_afrl_cmasi_perceive_track_entity_task(&mut self) -> Option<&mut TrackEntityTask> { None }
    fn entity_id(&self) -> u32;
    fn entity_id_mut(&mut self) -> &mut u32;
    fn sensor_modality(&self) -> ::afrl::cmasi::wavelength_band::WavelengthBand;
    fn sensor_modality_mut(&mut self) -> &mut ::afrl::cmasi::wavelength_band::WavelengthBand;
    fn ground_sample_distance(&self) -> f32;
    fn ground_sample_distance_mut(&mut self) -> &mut f32;

}

impl Clone for Box<TrackEntityTaskT> {
    fn clone(&self) -> Box<TrackEntityTaskT> {
        if let Some(x) = TrackEntityTaskT::as_afrl_cmasi_perceive_track_entity_task(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<TrackEntityTaskT> {
    fn default() -> Box<TrackEntityTaskT> { Box::new(TrackEntityTask::default()) }
}

impl PartialEq for Box<TrackEntityTaskT> {
    fn eq(&self, other: &Box<TrackEntityTaskT>) -> bool {
        if let (Some(x), Some(y)) =
            (TrackEntityTaskT::as_afrl_cmasi_perceive_track_entity_task(self.as_ref()),
             TrackEntityTaskT::as_afrl_cmasi_perceive_track_entity_task(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<TrackEntityTaskT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = TrackEntityTaskT::as_afrl_cmasi_perceive_track_entity_task(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<TrackEntityTaskT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == TrackEntityTask::struct_info() {
            let (x, readb) = TrackEntityTask::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = TrackEntityTaskT::as_afrl_cmasi_perceive_track_entity_task(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::task::TaskT for TrackEntityTask {
    fn as_afrl_cmasi_perceive_track_entity_task(&self) -> Option<&TrackEntityTask> { Some(self) }
    fn as_mut_afrl_cmasi_perceive_track_entity_task(&mut self) -> Option<&mut TrackEntityTask> { Some(self) }
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
impl TrackEntityTaskT for TrackEntityTask {
    fn as_afrl_cmasi_perceive_track_entity_task(&self) -> Option<&TrackEntityTask> { Some(self) }
    fn as_mut_afrl_cmasi_perceive_track_entity_task(&mut self) -> Option<&mut TrackEntityTask> { Some(self) }
    fn entity_id(&self) -> u32 { self.entity_id }
    fn entity_id_mut(&mut self) -> &mut u32 { &mut self.entity_id }
    fn sensor_modality(&self) -> ::afrl::cmasi::wavelength_band::WavelengthBand { self.sensor_modality }
    fn sensor_modality_mut(&mut self) -> &mut ::afrl::cmasi::wavelength_band::WavelengthBand { &mut self.sensor_modality }
    fn ground_sample_distance(&self) -> f32 { self.ground_sample_distance }
    fn ground_sample_distance_mut(&mut self) -> &mut f32 { &mut self.ground_sample_distance }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for TrackEntityTask {
        fn arbitrary<G: Gen>(_g: &mut G) -> TrackEntityTask {
            TrackEntityTask {
                task_id: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                eligible_entities: Arbitrary::arbitrary(_g),
                revisit_rate: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                priority: Arbitrary::arbitrary(_g),
                required: Arbitrary::arbitrary(_g),
                entity_id: Arbitrary::arbitrary(_g),
                sensor_modality: Arbitrary::arbitrary(_g),
                ground_sample_distance: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: TrackEntityTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: TrackEntityTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = TrackEntityTask::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
