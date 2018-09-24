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
pub struct CameraState {
    pub payload_id: i64,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub pointing_mode: ::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode,
    pub azimuth: f32,
    pub elevation: f32,
    pub rotation: f32,
    pub horizontal_field_of_view: f32,
    pub vertical_field_of_view: f32,
    pub footprint: Vec<Box<::afrl::cmasi::location3d::Location3DT>>,
    pub centerpoint: Option<Box<::afrl::cmasi::location3d::Location3DT>>,
}

impl PartialEq for CameraState {
    fn eq(&self, _other: &CameraState) -> bool {
        true
        && &self.horizontal_field_of_view == &_other.horizontal_field_of_view
        && &self.vertical_field_of_view == &_other.vertical_field_of_view
        && &self.footprint == &_other.footprint
        && &self.centerpoint == &_other.centerpoint

    }
}

impl LmcpSubscription for CameraState {
    fn subscription() -> &'static str { "afrl.cmasi.CameraState" }
}

impl Struct for CameraState {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 21,
        }
    }
}

impl Lmcp for CameraState {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.payload_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.parameters.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.pointing_mode.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.azimuth.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.elevation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.rotation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.horizontal_field_of_view.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vertical_field_of_view.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.footprint.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.centerpoint.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(CameraState, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == CameraState::struct_info() {
            let mut out: CameraState = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.payload_id = x;
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
                let (x, readb): (::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode, usize) = Lmcp::deser(r)?;
                out.pointing_mode = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.azimuth = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.elevation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.rotation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.horizontal_field_of_view = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.vertical_field_of_view = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.footprint = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Option<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.centerpoint = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.payload_id.size();
        size += self.parameters.size();
        size += self.pointing_mode.size();
        size += self.azimuth.size();
        size += self.elevation.size();
        size += self.rotation.size();
        size += self.horizontal_field_of_view.size();
        size += self.vertical_field_of_view.size();
        size += self.footprint.size();
        size += self.centerpoint.size();

        size
    }
}

pub trait CameraStateT: Debug + Send + ::afrl::cmasi::gimballed_payload_state::GimballedPayloadStateT {
    fn as_afrl_cmasi_camera_state(&self) -> Option<&CameraState> { None }
    fn as_mut_afrl_cmasi_camera_state(&mut self) -> Option<&mut CameraState> { None }
    fn horizontal_field_of_view(&self) -> f32;
    fn horizontal_field_of_view_mut(&mut self) -> &mut f32;
    fn vertical_field_of_view(&self) -> f32;
    fn vertical_field_of_view_mut(&mut self) -> &mut f32;
    fn footprint(&self) -> &Vec<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn footprint_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn centerpoint(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn centerpoint_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>>;

}

impl Clone for Box<CameraStateT> {
    fn clone(&self) -> Box<CameraStateT> {
        if let Some(x) = CameraStateT::as_afrl_cmasi_camera_state(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<CameraStateT> {
    fn default() -> Box<CameraStateT> { Box::new(CameraState::default()) }
}

impl PartialEq for Box<CameraStateT> {
    fn eq(&self, other: &Box<CameraStateT>) -> bool {
        if let (Some(x), Some(y)) =
            (CameraStateT::as_afrl_cmasi_camera_state(self.as_ref()),
             CameraStateT::as_afrl_cmasi_camera_state(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<CameraStateT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = CameraStateT::as_afrl_cmasi_camera_state(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<CameraStateT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == CameraState::struct_info() {
            let (x, readb) = CameraState::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = CameraStateT::as_afrl_cmasi_camera_state(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_state::PayloadStateT for CameraState {
    fn as_afrl_cmasi_camera_state(&self) -> Option<&CameraState> { Some(self) }
    fn as_mut_afrl_cmasi_camera_state(&mut self) -> Option<&mut CameraState> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl ::afrl::cmasi::gimballed_payload_state::GimballedPayloadStateT for CameraState {
    fn as_afrl_cmasi_camera_state(&self) -> Option<&CameraState> { Some(self) }
    fn as_mut_afrl_cmasi_camera_state(&mut self) -> Option<&mut CameraState> { Some(self) }
    fn pointing_mode(&self) -> ::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode { self.pointing_mode }
    fn pointing_mode_mut(&mut self) -> &mut ::afrl::cmasi::gimbal_pointing_mode::GimbalPointingMode { &mut self.pointing_mode }
    fn azimuth(&self) -> f32 { self.azimuth }
    fn azimuth_mut(&mut self) -> &mut f32 { &mut self.azimuth }
    fn elevation(&self) -> f32 { self.elevation }
    fn elevation_mut(&mut self) -> &mut f32 { &mut self.elevation }
    fn rotation(&self) -> f32 { self.rotation }
    fn rotation_mut(&mut self) -> &mut f32 { &mut self.rotation }
}
impl CameraStateT for CameraState {
    fn as_afrl_cmasi_camera_state(&self) -> Option<&CameraState> { Some(self) }
    fn as_mut_afrl_cmasi_camera_state(&mut self) -> Option<&mut CameraState> { Some(self) }
    fn horizontal_field_of_view(&self) -> f32 { self.horizontal_field_of_view }
    fn horizontal_field_of_view_mut(&mut self) -> &mut f32 { &mut self.horizontal_field_of_view }
    fn vertical_field_of_view(&self) -> f32 { self.vertical_field_of_view }
    fn vertical_field_of_view_mut(&mut self) -> &mut f32 { &mut self.vertical_field_of_view }
    fn footprint(&self) -> &Vec<Box<::afrl::cmasi::location3d::Location3DT>> { &self.footprint }
    fn footprint_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.footprint }
    fn centerpoint(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>> { &self.centerpoint }
    fn centerpoint_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.centerpoint }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for CameraState {
        fn arbitrary<G: Gen>(_g: &mut G) -> CameraState {
            CameraState {
                payload_id: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                pointing_mode: Arbitrary::arbitrary(_g),
                azimuth: Arbitrary::arbitrary(_g),
                elevation: Arbitrary::arbitrary(_g),
                rotation: Arbitrary::arbitrary(_g),
                horizontal_field_of_view: Arbitrary::arbitrary(_g),
                vertical_field_of_view: Arbitrary::arbitrary(_g),
                footprint: Vec::<::afrl::cmasi::location3d::Location3D>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::location3d::Location3DT>).collect(),
                centerpoint: {
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
        fn serializes(x: CameraState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.footprint.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: CameraState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.footprint.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = CameraState::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
