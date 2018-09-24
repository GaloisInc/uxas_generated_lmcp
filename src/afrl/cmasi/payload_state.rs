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
pub struct PayloadState {
    pub payload_id: i64,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
}

impl PartialEq for PayloadState {
    fn eq(&self, _other: &PayloadState) -> bool {
        true
        && &self.payload_id == &_other.payload_id
        && &self.parameters == &_other.parameters

    }
}

impl LmcpSubscription for PayloadState {
    fn subscription() -> &'static str { "afrl.cmasi.PayloadState" }
}

impl Struct for PayloadState {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 6,
        }
    }
}

impl Lmcp for PayloadState {
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

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PayloadState, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PayloadState::struct_info() {
            let mut out: PayloadState = Default::default();
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

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.payload_id.size();
        size += self.parameters.size();

        size
    }
}

pub trait PayloadStateT: Debug + Send  {
    fn as_afrl_cmasi_payload_state(&self) -> Option<&PayloadState> { None }
    fn as_mut_afrl_cmasi_payload_state(&mut self) -> Option<&mut PayloadState> { None }
    fn as_afrl_impact_power_plant_state(&self) -> Option<&::afrl::impact::power_plant_state::PowerPlantState> { None }
    fn as_mut_afrl_impact_power_plant_state(&mut self) -> Option<&mut ::afrl::impact::power_plant_state::PowerPlantState> { None }
    fn as_afrl_impact_radio_state(&self) -> Option<&::afrl::impact::radio_state::RadioState> { None }
    fn as_mut_afrl_impact_radio_state(&mut self) -> Option<&mut ::afrl::impact::radio_state::RadioState> { None }
    fn as_afrl_cmasi_gimbal_state(&self) -> Option<&::afrl::cmasi::gimbal_state::GimbalState> { None }
    fn as_mut_afrl_cmasi_gimbal_state(&mut self) -> Option<&mut ::afrl::cmasi::gimbal_state::GimbalState> { None }
    fn as_afrl_cmasi_video_stream_state(&self) -> Option<&::afrl::cmasi::video_stream_state::VideoStreamState> { None }
    fn as_mut_afrl_cmasi_video_stream_state(&mut self) -> Option<&mut ::afrl::cmasi::video_stream_state::VideoStreamState> { None }
    fn as_afrl_cmasi_gimballed_payload_state(&self) -> Option<&::afrl::cmasi::gimballed_payload_state::GimballedPayloadState> { None }
    fn as_mut_afrl_cmasi_gimballed_payload_state(&mut self) -> Option<&mut ::afrl::cmasi::gimballed_payload_state::GimballedPayloadState> { None }
    fn as_afrl_cmasi_camera_state(&self) -> Option<&::afrl::cmasi::camera_state::CameraState> { None }
    fn as_mut_afrl_cmasi_camera_state(&mut self) -> Option<&mut ::afrl::cmasi::camera_state::CameraState> { None }
    fn payload_id(&self) -> i64;
    fn payload_id_mut(&mut self) -> &mut i64;
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;

}

impl Clone for Box<PayloadStateT> {
    fn clone(&self) -> Box<PayloadStateT> {
        if let Some(x) = PayloadStateT::as_afrl_cmasi_payload_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadStateT::as_afrl_impact_power_plant_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadStateT::as_afrl_impact_radio_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_gimbal_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_video_stream_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_gimballed_payload_state(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_camera_state(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PayloadStateT> {
    fn default() -> Box<PayloadStateT> { Box::new(PayloadState::default()) }
}

impl PartialEq for Box<PayloadStateT> {
    fn eq(&self, other: &Box<PayloadStateT>) -> bool {
        if let (Some(x), Some(y)) =
            (PayloadStateT::as_afrl_cmasi_payload_state(self.as_ref()),
             PayloadStateT::as_afrl_cmasi_payload_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadStateT::as_afrl_impact_power_plant_state(self.as_ref()),
             PayloadStateT::as_afrl_impact_power_plant_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadStateT::as_afrl_impact_radio_state(self.as_ref()),
             PayloadStateT::as_afrl_impact_radio_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadStateT::as_afrl_cmasi_gimbal_state(self.as_ref()),
             PayloadStateT::as_afrl_cmasi_gimbal_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadStateT::as_afrl_cmasi_video_stream_state(self.as_ref()),
             PayloadStateT::as_afrl_cmasi_video_stream_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadStateT::as_afrl_cmasi_gimballed_payload_state(self.as_ref()),
             PayloadStateT::as_afrl_cmasi_gimballed_payload_state(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (PayloadStateT::as_afrl_cmasi_camera_state(self.as_ref()),
             PayloadStateT::as_afrl_cmasi_camera_state(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PayloadStateT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PayloadStateT::as_afrl_cmasi_payload_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadStateT::as_afrl_impact_power_plant_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadStateT::as_afrl_impact_radio_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_gimbal_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_video_stream_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_gimballed_payload_state(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_camera_state(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PayloadStateT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PayloadState::struct_info() {
            let (x, readb) = PayloadState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::power_plant_state::PowerPlantState::struct_info() {
            let (x, readb) = ::afrl::impact::power_plant_state::PowerPlantState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::impact::radio_state::RadioState::struct_info() {
            let (x, readb) = ::afrl::impact::radio_state::RadioState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::gimbal_state::GimbalState::struct_info() {
            let (x, readb) = ::afrl::cmasi::gimbal_state::GimbalState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::video_stream_state::VideoStreamState::struct_info() {
            let (x, readb) = ::afrl::cmasi::video_stream_state::VideoStreamState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::gimballed_payload_state::GimballedPayloadState::struct_info() {
            let (x, readb) = ::afrl::cmasi::gimballed_payload_state::GimballedPayloadState::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::camera_state::CameraState::struct_info() {
            let (x, readb) = ::afrl::cmasi::camera_state::CameraState::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PayloadStateT::as_afrl_cmasi_payload_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadStateT::as_afrl_impact_power_plant_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadStateT::as_afrl_impact_radio_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_gimbal_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_video_stream_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_gimballed_payload_state(self.as_ref()) {
            x.size()
        } else if let Some(x) = PayloadStateT::as_afrl_cmasi_camera_state(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl PayloadStateT for PayloadState {
    fn as_afrl_cmasi_payload_state(&self) -> Option<&PayloadState> { Some(self) }
    fn as_mut_afrl_cmasi_payload_state(&mut self) -> Option<&mut PayloadState> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PayloadState {
        fn arbitrary<G: Gen>(_g: &mut G) -> PayloadState {
            PayloadState {
                payload_id: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PayloadState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PayloadState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PayloadState::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
