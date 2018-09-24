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
pub struct CameraConfiguration {
    pub payload_id: i64,
    pub payload_kind: Vec<u8>,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub supported_wavelength_band: ::afrl::cmasi::wavelength_band::WavelengthBand,
    pub field_of_view_mode: ::afrl::cmasi::fovoperation_mode::FOVOperationMode,
    pub min_horizontal_field_of_view: f32,
    pub max_horizontal_field_of_view: f32,
    pub discrete_horizontal_field_of_view_list: Vec<f32>,
    pub video_stream_horizontal_resolution: u32,
    pub video_stream_vertical_resolution: u32,
}

impl PartialEq for CameraConfiguration {
    fn eq(&self, _other: &CameraConfiguration) -> bool {
        true
        && &self.supported_wavelength_band == &_other.supported_wavelength_band
        && &self.field_of_view_mode == &_other.field_of_view_mode
        && &self.min_horizontal_field_of_view == &_other.min_horizontal_field_of_view
        && &self.max_horizontal_field_of_view == &_other.max_horizontal_field_of_view
        && &self.discrete_horizontal_field_of_view_list == &_other.discrete_horizontal_field_of_view_list
        && &self.video_stream_horizontal_resolution == &_other.video_stream_horizontal_resolution
        && &self.video_stream_vertical_resolution == &_other.video_stream_vertical_resolution

    }
}

impl LmcpSubscription for CameraConfiguration {
    fn subscription() -> &'static str { "afrl.cmasi.CameraConfiguration" }
}

impl Struct for CameraConfiguration {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 19,
        }
    }
}

impl Lmcp for CameraConfiguration {
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
            let writeb: usize = self.payload_kind.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.parameters.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.supported_wavelength_band.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.field_of_view_mode.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.min_horizontal_field_of_view.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.max_horizontal_field_of_view.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.discrete_horizontal_field_of_view_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.video_stream_horizontal_resolution.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.video_stream_vertical_resolution.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(CameraConfiguration, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == CameraConfiguration::struct_info() {
            let mut out: CameraConfiguration = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.payload_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.payload_kind = x;
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
                let (x, readb): (::afrl::cmasi::wavelength_band::WavelengthBand, usize) = Lmcp::deser(r)?;
                out.supported_wavelength_band = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::fovoperation_mode::FOVOperationMode, usize) = Lmcp::deser(r)?;
                out.field_of_view_mode = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.min_horizontal_field_of_view = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.max_horizontal_field_of_view = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<f32>, usize) = Lmcp::deser(r)?;
                out.discrete_horizontal_field_of_view_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u32, usize) = Lmcp::deser(r)?;
                out.video_stream_horizontal_resolution = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u32, usize) = Lmcp::deser(r)?;
                out.video_stream_vertical_resolution = x;
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
        size += self.payload_kind.size();
        size += self.parameters.size();
        size += self.supported_wavelength_band.size();
        size += self.field_of_view_mode.size();
        size += self.min_horizontal_field_of_view.size();
        size += self.max_horizontal_field_of_view.size();
        size += self.discrete_horizontal_field_of_view_list.size();
        size += self.video_stream_horizontal_resolution.size();
        size += self.video_stream_vertical_resolution.size();

        size
    }
}

pub trait CameraConfigurationT: Debug + Send + ::afrl::cmasi::payload_configuration::PayloadConfigurationT {
    fn as_afrl_cmasi_camera_configuration(&self) -> Option<&CameraConfiguration> { None }
    fn as_mut_afrl_cmasi_camera_configuration(&mut self) -> Option<&mut CameraConfiguration> { None }
    fn supported_wavelength_band(&self) -> ::afrl::cmasi::wavelength_band::WavelengthBand;
    fn supported_wavelength_band_mut(&mut self) -> &mut ::afrl::cmasi::wavelength_band::WavelengthBand;
    fn field_of_view_mode(&self) -> ::afrl::cmasi::fovoperation_mode::FOVOperationMode;
    fn field_of_view_mode_mut(&mut self) -> &mut ::afrl::cmasi::fovoperation_mode::FOVOperationMode;
    fn min_horizontal_field_of_view(&self) -> f32;
    fn min_horizontal_field_of_view_mut(&mut self) -> &mut f32;
    fn max_horizontal_field_of_view(&self) -> f32;
    fn max_horizontal_field_of_view_mut(&mut self) -> &mut f32;
    fn discrete_horizontal_field_of_view_list(&self) -> &Vec<f32>;
    fn discrete_horizontal_field_of_view_list_mut(&mut self) -> &mut Vec<f32>;
    fn video_stream_horizontal_resolution(&self) -> u32;
    fn video_stream_horizontal_resolution_mut(&mut self) -> &mut u32;
    fn video_stream_vertical_resolution(&self) -> u32;
    fn video_stream_vertical_resolution_mut(&mut self) -> &mut u32;

}

impl Clone for Box<CameraConfigurationT> {
    fn clone(&self) -> Box<CameraConfigurationT> {
        if let Some(x) = CameraConfigurationT::as_afrl_cmasi_camera_configuration(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<CameraConfigurationT> {
    fn default() -> Box<CameraConfigurationT> { Box::new(CameraConfiguration::default()) }
}

impl PartialEq for Box<CameraConfigurationT> {
    fn eq(&self, other: &Box<CameraConfigurationT>) -> bool {
        if let (Some(x), Some(y)) =
            (CameraConfigurationT::as_afrl_cmasi_camera_configuration(self.as_ref()),
             CameraConfigurationT::as_afrl_cmasi_camera_configuration(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<CameraConfigurationT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = CameraConfigurationT::as_afrl_cmasi_camera_configuration(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<CameraConfigurationT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == CameraConfiguration::struct_info() {
            let (x, readb) = CameraConfiguration::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = CameraConfigurationT::as_afrl_cmasi_camera_configuration(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_configuration::PayloadConfigurationT for CameraConfiguration {
    fn as_afrl_cmasi_camera_configuration(&self) -> Option<&CameraConfiguration> { Some(self) }
    fn as_mut_afrl_cmasi_camera_configuration(&mut self) -> Option<&mut CameraConfiguration> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn payload_kind(&self) -> &Vec<u8> { &self.payload_kind }
    fn payload_kind_mut(&mut self) -> &mut Vec<u8> { &mut self.payload_kind }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl CameraConfigurationT for CameraConfiguration {
    fn as_afrl_cmasi_camera_configuration(&self) -> Option<&CameraConfiguration> { Some(self) }
    fn as_mut_afrl_cmasi_camera_configuration(&mut self) -> Option<&mut CameraConfiguration> { Some(self) }
    fn supported_wavelength_band(&self) -> ::afrl::cmasi::wavelength_band::WavelengthBand { self.supported_wavelength_band }
    fn supported_wavelength_band_mut(&mut self) -> &mut ::afrl::cmasi::wavelength_band::WavelengthBand { &mut self.supported_wavelength_band }
    fn field_of_view_mode(&self) -> ::afrl::cmasi::fovoperation_mode::FOVOperationMode { self.field_of_view_mode }
    fn field_of_view_mode_mut(&mut self) -> &mut ::afrl::cmasi::fovoperation_mode::FOVOperationMode { &mut self.field_of_view_mode }
    fn min_horizontal_field_of_view(&self) -> f32 { self.min_horizontal_field_of_view }
    fn min_horizontal_field_of_view_mut(&mut self) -> &mut f32 { &mut self.min_horizontal_field_of_view }
    fn max_horizontal_field_of_view(&self) -> f32 { self.max_horizontal_field_of_view }
    fn max_horizontal_field_of_view_mut(&mut self) -> &mut f32 { &mut self.max_horizontal_field_of_view }
    fn discrete_horizontal_field_of_view_list(&self) -> &Vec<f32> { &self.discrete_horizontal_field_of_view_list }
    fn discrete_horizontal_field_of_view_list_mut(&mut self) -> &mut Vec<f32> { &mut self.discrete_horizontal_field_of_view_list }
    fn video_stream_horizontal_resolution(&self) -> u32 { self.video_stream_horizontal_resolution }
    fn video_stream_horizontal_resolution_mut(&mut self) -> &mut u32 { &mut self.video_stream_horizontal_resolution }
    fn video_stream_vertical_resolution(&self) -> u32 { self.video_stream_vertical_resolution }
    fn video_stream_vertical_resolution_mut(&mut self) -> &mut u32 { &mut self.video_stream_vertical_resolution }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for CameraConfiguration {
        fn arbitrary<G: Gen>(_g: &mut G) -> CameraConfiguration {
            CameraConfiguration {
                payload_id: Arbitrary::arbitrary(_g),
                payload_kind: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                supported_wavelength_band: Arbitrary::arbitrary(_g),
                field_of_view_mode: Arbitrary::arbitrary(_g),
                min_horizontal_field_of_view: Arbitrary::arbitrary(_g),
                max_horizontal_field_of_view: Arbitrary::arbitrary(_g),
                discrete_horizontal_field_of_view_list: Arbitrary::arbitrary(_g),
                video_stream_horizontal_resolution: Arbitrary::arbitrary(_g),
                video_stream_vertical_resolution: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: CameraConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.discrete_horizontal_field_of_view_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: CameraConfiguration) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.discrete_horizontal_field_of_view_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = CameraConfiguration::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
