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
pub struct AngledAreaSearchTask {
    pub task_id: i64,
    pub label: Vec<u8>,
    pub eligible_entities: Vec<i64>,
    pub revisit_rate: f32,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub priority: u8,
    pub required: bool,
    pub desired_wavelength_bands: Vec<::afrl::cmasi::wavelength_band::WavelengthBand>,
    pub dwell_time: i64,
    pub ground_sample_distance: f32,
    pub search_area_id: i64,
    pub sweep_angle: f32,
    pub start_point: Option<Box<::afrl::cmasi::location3d::Location3DT>>,
}

impl PartialEq for AngledAreaSearchTask {
    fn eq(&self, _other: &AngledAreaSearchTask) -> bool {
        true
        && &self.search_area_id == &_other.search_area_id
        && &self.sweep_angle == &_other.sweep_angle
        && &self.start_point == &_other.start_point

    }
}

impl LmcpSubscription for AngledAreaSearchTask {
    fn subscription() -> &'static str { "afrl.impact.AngledAreaSearchTask" }
}

impl Struct for AngledAreaSearchTask {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 24,
        }
    }
}

impl Lmcp for AngledAreaSearchTask {
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
            let writeb: usize = self.desired_wavelength_bands.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.dwell_time.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.ground_sample_distance.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.search_area_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.sweep_angle.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_point.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(AngledAreaSearchTask, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == AngledAreaSearchTask::struct_info() {
            let mut out: AngledAreaSearchTask = Default::default();
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
                let (x, readb): (Vec<::afrl::cmasi::wavelength_band::WavelengthBand>, usize) = Lmcp::deser(r)?;
                out.desired_wavelength_bands = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.dwell_time = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.ground_sample_distance = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.search_area_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.sweep_angle = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Option<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.start_point = x;
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
        size += self.desired_wavelength_bands.size();
        size += self.dwell_time.size();
        size += self.ground_sample_distance.size();
        size += self.search_area_id.size();
        size += self.sweep_angle.size();
        size += self.start_point.size();

        size
    }
}

pub trait AngledAreaSearchTaskT: Debug + Send + ::afrl::cmasi::search_task::SearchTaskT {
    fn as_afrl_impact_angled_area_search_task(&self) -> Option<&AngledAreaSearchTask> { None }
    fn as_mut_afrl_impact_angled_area_search_task(&mut self) -> Option<&mut AngledAreaSearchTask> { None }
    fn search_area_id(&self) -> i64;
    fn search_area_id_mut(&mut self) -> &mut i64;
    fn sweep_angle(&self) -> f32;
    fn sweep_angle_mut(&mut self) -> &mut f32;
    fn start_point(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn start_point_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>>;

}

impl Clone for Box<AngledAreaSearchTaskT> {
    fn clone(&self) -> Box<AngledAreaSearchTaskT> {
        if let Some(x) = AngledAreaSearchTaskT::as_afrl_impact_angled_area_search_task(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<AngledAreaSearchTaskT> {
    fn default() -> Box<AngledAreaSearchTaskT> { Box::new(AngledAreaSearchTask::default()) }
}

impl PartialEq for Box<AngledAreaSearchTaskT> {
    fn eq(&self, other: &Box<AngledAreaSearchTaskT>) -> bool {
        if let (Some(x), Some(y)) =
            (AngledAreaSearchTaskT::as_afrl_impact_angled_area_search_task(self.as_ref()),
             AngledAreaSearchTaskT::as_afrl_impact_angled_area_search_task(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<AngledAreaSearchTaskT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = AngledAreaSearchTaskT::as_afrl_impact_angled_area_search_task(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<AngledAreaSearchTaskT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == AngledAreaSearchTask::struct_info() {
            let (x, readb) = AngledAreaSearchTask::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = AngledAreaSearchTaskT::as_afrl_impact_angled_area_search_task(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::task::TaskT for AngledAreaSearchTask {
    fn as_afrl_impact_angled_area_search_task(&self) -> Option<&AngledAreaSearchTask> { Some(self) }
    fn as_mut_afrl_impact_angled_area_search_task(&mut self) -> Option<&mut AngledAreaSearchTask> { Some(self) }
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
impl ::afrl::cmasi::search_task::SearchTaskT for AngledAreaSearchTask {
    fn as_afrl_impact_angled_area_search_task(&self) -> Option<&AngledAreaSearchTask> { Some(self) }
    fn as_mut_afrl_impact_angled_area_search_task(&mut self) -> Option<&mut AngledAreaSearchTask> { Some(self) }
    fn desired_wavelength_bands(&self) -> &Vec<::afrl::cmasi::wavelength_band::WavelengthBand> { &self.desired_wavelength_bands }
    fn desired_wavelength_bands_mut(&mut self) -> &mut Vec<::afrl::cmasi::wavelength_band::WavelengthBand> { &mut self.desired_wavelength_bands }
    fn dwell_time(&self) -> i64 { self.dwell_time }
    fn dwell_time_mut(&mut self) -> &mut i64 { &mut self.dwell_time }
    fn ground_sample_distance(&self) -> f32 { self.ground_sample_distance }
    fn ground_sample_distance_mut(&mut self) -> &mut f32 { &mut self.ground_sample_distance }
}
impl AngledAreaSearchTaskT for AngledAreaSearchTask {
    fn as_afrl_impact_angled_area_search_task(&self) -> Option<&AngledAreaSearchTask> { Some(self) }
    fn as_mut_afrl_impact_angled_area_search_task(&mut self) -> Option<&mut AngledAreaSearchTask> { Some(self) }
    fn search_area_id(&self) -> i64 { self.search_area_id }
    fn search_area_id_mut(&mut self) -> &mut i64 { &mut self.search_area_id }
    fn sweep_angle(&self) -> f32 { self.sweep_angle }
    fn sweep_angle_mut(&mut self) -> &mut f32 { &mut self.sweep_angle }
    fn start_point(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>> { &self.start_point }
    fn start_point_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.start_point }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AngledAreaSearchTask {
        fn arbitrary<G: Gen>(_g: &mut G) -> AngledAreaSearchTask {
            AngledAreaSearchTask {
                task_id: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                eligible_entities: Arbitrary::arbitrary(_g),
                revisit_rate: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                priority: Arbitrary::arbitrary(_g),
                required: Arbitrary::arbitrary(_g),
                desired_wavelength_bands: Arbitrary::arbitrary(_g),
                dwell_time: Arbitrary::arbitrary(_g),
                ground_sample_distance: Arbitrary::arbitrary(_g),
                search_area_id: Arbitrary::arbitrary(_g),
                sweep_angle: Arbitrary::arbitrary(_g),
                start_point: {
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
        fn serializes(x: AngledAreaSearchTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.desired_wavelength_bands.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AngledAreaSearchTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.desired_wavelength_bands.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AngledAreaSearchTask::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
