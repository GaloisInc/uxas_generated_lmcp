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
pub struct PointSearchTask {
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
    pub search_location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub standoff_distance: f32,
    pub view_angle_list: Vec<Box<::afrl::cmasi::wedge::WedgeT>>,
}

impl PartialEq for PointSearchTask {
    fn eq(&self, _other: &PointSearchTask) -> bool {
        true
        && &self.search_location == &_other.search_location
        && &self.standoff_distance == &_other.standoff_distance
        && &self.view_angle_list == &_other.view_angle_list

    }
}

impl LmcpSubscription for PointSearchTask {
    fn subscription() -> &'static str { "afrl.cmasi.PointSearchTask" }
}

impl Struct for PointSearchTask {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 41,
        }
    }
}

impl Lmcp for PointSearchTask {
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
            let writeb: usize = self.search_location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.standoff_distance.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.view_angle_list.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PointSearchTask, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PointSearchTask::struct_info() {
            let mut out: PointSearchTask = Default::default();
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
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.search_location = x;
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
                let (x, readb): (Vec<Box<::afrl::cmasi::wedge::WedgeT>>, usize) = Lmcp::deser(r)?;
                out.view_angle_list = x;
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
        size += self.search_location.size();
        size += self.standoff_distance.size();
        size += self.view_angle_list.size();

        size
    }
}

pub trait PointSearchTaskT: Debug + Send + ::afrl::cmasi::search_task::SearchTaskT {
    fn as_afrl_cmasi_point_search_task(&self) -> Option<&PointSearchTask> { None }
    fn as_mut_afrl_cmasi_point_search_task(&mut self) -> Option<&mut PointSearchTask> { None }
    fn search_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn search_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn standoff_distance(&self) -> f32;
    fn standoff_distance_mut(&mut self) -> &mut f32;
    fn view_angle_list(&self) -> &Vec<Box<::afrl::cmasi::wedge::WedgeT>>;
    fn view_angle_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::wedge::WedgeT>>;

}

impl Clone for Box<PointSearchTaskT> {
    fn clone(&self) -> Box<PointSearchTaskT> {
        if let Some(x) = PointSearchTaskT::as_afrl_cmasi_point_search_task(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PointSearchTaskT> {
    fn default() -> Box<PointSearchTaskT> { Box::new(PointSearchTask::default()) }
}

impl PartialEq for Box<PointSearchTaskT> {
    fn eq(&self, other: &Box<PointSearchTaskT>) -> bool {
        if let (Some(x), Some(y)) =
            (PointSearchTaskT::as_afrl_cmasi_point_search_task(self.as_ref()),
             PointSearchTaskT::as_afrl_cmasi_point_search_task(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PointSearchTaskT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PointSearchTaskT::as_afrl_cmasi_point_search_task(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PointSearchTaskT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PointSearchTask::struct_info() {
            let (x, readb) = PointSearchTask::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PointSearchTaskT::as_afrl_cmasi_point_search_task(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::task::TaskT for PointSearchTask {
    fn as_afrl_cmasi_point_search_task(&self) -> Option<&PointSearchTask> { Some(self) }
    fn as_mut_afrl_cmasi_point_search_task(&mut self) -> Option<&mut PointSearchTask> { Some(self) }
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
impl ::afrl::cmasi::search_task::SearchTaskT for PointSearchTask {
    fn as_afrl_cmasi_point_search_task(&self) -> Option<&PointSearchTask> { Some(self) }
    fn as_mut_afrl_cmasi_point_search_task(&mut self) -> Option<&mut PointSearchTask> { Some(self) }
    fn desired_wavelength_bands(&self) -> &Vec<::afrl::cmasi::wavelength_band::WavelengthBand> { &self.desired_wavelength_bands }
    fn desired_wavelength_bands_mut(&mut self) -> &mut Vec<::afrl::cmasi::wavelength_band::WavelengthBand> { &mut self.desired_wavelength_bands }
    fn dwell_time(&self) -> i64 { self.dwell_time }
    fn dwell_time_mut(&mut self) -> &mut i64 { &mut self.dwell_time }
    fn ground_sample_distance(&self) -> f32 { self.ground_sample_distance }
    fn ground_sample_distance_mut(&mut self) -> &mut f32 { &mut self.ground_sample_distance }
}
impl PointSearchTaskT for PointSearchTask {
    fn as_afrl_cmasi_point_search_task(&self) -> Option<&PointSearchTask> { Some(self) }
    fn as_mut_afrl_cmasi_point_search_task(&mut self) -> Option<&mut PointSearchTask> { Some(self) }
    fn search_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.search_location }
    fn search_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.search_location }
    fn standoff_distance(&self) -> f32 { self.standoff_distance }
    fn standoff_distance_mut(&mut self) -> &mut f32 { &mut self.standoff_distance }
    fn view_angle_list(&self) -> &Vec<Box<::afrl::cmasi::wedge::WedgeT>> { &self.view_angle_list }
    fn view_angle_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::wedge::WedgeT>> { &mut self.view_angle_list }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PointSearchTask {
        fn arbitrary<G: Gen>(_g: &mut G) -> PointSearchTask {
            PointSearchTask {
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
                search_location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                standoff_distance: Arbitrary::arbitrary(_g),
                view_angle_list: Vec::<::afrl::cmasi::wedge::Wedge>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::wedge::WedgeT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PointSearchTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.desired_wavelength_bands.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.view_angle_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PointSearchTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.desired_wavelength_bands.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.view_angle_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PointSearchTask::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
