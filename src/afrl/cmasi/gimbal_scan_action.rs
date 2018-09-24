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
pub struct GimbalScanAction {
    pub associated_task_list: Vec<i64>,
    pub payload_id: i64,
    pub azimuth_slew_rate: f32,
    pub elevation_slew_rate: f32,
    pub start_azimuth: f32,
    pub end_azimuth: f32,
    pub start_elevation: f32,
    pub end_elevation: f32,
    pub cycles: u32,
}

impl PartialEq for GimbalScanAction {
    fn eq(&self, _other: &GimbalScanAction) -> bool {
        true
        && &self.azimuth_slew_rate == &_other.azimuth_slew_rate
        && &self.elevation_slew_rate == &_other.elevation_slew_rate
        && &self.start_azimuth == &_other.start_azimuth
        && &self.end_azimuth == &_other.end_azimuth
        && &self.start_elevation == &_other.start_elevation
        && &self.end_elevation == &_other.end_elevation
        && &self.cycles == &_other.cycles

    }
}

impl LmcpSubscription for GimbalScanAction {
    fn subscription() -> &'static str { "afrl.cmasi.GimbalScanAction" }
}

impl Struct for GimbalScanAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 25,
        }
    }
}

impl Lmcp for GimbalScanAction {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.associated_task_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.payload_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.azimuth_slew_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.elevation_slew_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_azimuth.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.end_azimuth.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_elevation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.end_elevation.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.cycles.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(GimbalScanAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == GimbalScanAction::struct_info() {
            let mut out: GimbalScanAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.payload_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.azimuth_slew_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.elevation_slew_rate = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.start_azimuth = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.end_azimuth = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.start_elevation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.end_elevation = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u32, usize) = Lmcp::deser(r)?;
                out.cycles = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.associated_task_list.size();
        size += self.payload_id.size();
        size += self.azimuth_slew_rate.size();
        size += self.elevation_slew_rate.size();
        size += self.start_azimuth.size();
        size += self.end_azimuth.size();
        size += self.start_elevation.size();
        size += self.end_elevation.size();
        size += self.cycles.size();

        size
    }
}

pub trait GimbalScanActionT: Debug + Send + ::afrl::cmasi::payload_action::PayloadActionT {
    fn as_afrl_cmasi_gimbal_scan_action(&self) -> Option<&GimbalScanAction> { None }
    fn as_mut_afrl_cmasi_gimbal_scan_action(&mut self) -> Option<&mut GimbalScanAction> { None }
    fn azimuth_slew_rate(&self) -> f32;
    fn azimuth_slew_rate_mut(&mut self) -> &mut f32;
    fn elevation_slew_rate(&self) -> f32;
    fn elevation_slew_rate_mut(&mut self) -> &mut f32;
    fn start_azimuth(&self) -> f32;
    fn start_azimuth_mut(&mut self) -> &mut f32;
    fn end_azimuth(&self) -> f32;
    fn end_azimuth_mut(&mut self) -> &mut f32;
    fn start_elevation(&self) -> f32;
    fn start_elevation_mut(&mut self) -> &mut f32;
    fn end_elevation(&self) -> f32;
    fn end_elevation_mut(&mut self) -> &mut f32;
    fn cycles(&self) -> u32;
    fn cycles_mut(&mut self) -> &mut u32;

}

impl Clone for Box<GimbalScanActionT> {
    fn clone(&self) -> Box<GimbalScanActionT> {
        if let Some(x) = GimbalScanActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<GimbalScanActionT> {
    fn default() -> Box<GimbalScanActionT> { Box::new(GimbalScanAction::default()) }
}

impl PartialEq for Box<GimbalScanActionT> {
    fn eq(&self, other: &Box<GimbalScanActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (GimbalScanActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()),
             GimbalScanActionT::as_afrl_cmasi_gimbal_scan_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<GimbalScanActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = GimbalScanActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<GimbalScanActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == GimbalScanAction::struct_info() {
            let (x, readb) = GimbalScanAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = GimbalScanActionT::as_afrl_cmasi_gimbal_scan_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for GimbalScanAction {
    fn as_afrl_cmasi_gimbal_scan_action(&self) -> Option<&GimbalScanAction> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_scan_action(&mut self) -> Option<&mut GimbalScanAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl ::afrl::cmasi::payload_action::PayloadActionT for GimbalScanAction {
    fn as_afrl_cmasi_gimbal_scan_action(&self) -> Option<&GimbalScanAction> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_scan_action(&mut self) -> Option<&mut GimbalScanAction> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
}
impl GimbalScanActionT for GimbalScanAction {
    fn as_afrl_cmasi_gimbal_scan_action(&self) -> Option<&GimbalScanAction> { Some(self) }
    fn as_mut_afrl_cmasi_gimbal_scan_action(&mut self) -> Option<&mut GimbalScanAction> { Some(self) }
    fn azimuth_slew_rate(&self) -> f32 { self.azimuth_slew_rate }
    fn azimuth_slew_rate_mut(&mut self) -> &mut f32 { &mut self.azimuth_slew_rate }
    fn elevation_slew_rate(&self) -> f32 { self.elevation_slew_rate }
    fn elevation_slew_rate_mut(&mut self) -> &mut f32 { &mut self.elevation_slew_rate }
    fn start_azimuth(&self) -> f32 { self.start_azimuth }
    fn start_azimuth_mut(&mut self) -> &mut f32 { &mut self.start_azimuth }
    fn end_azimuth(&self) -> f32 { self.end_azimuth }
    fn end_azimuth_mut(&mut self) -> &mut f32 { &mut self.end_azimuth }
    fn start_elevation(&self) -> f32 { self.start_elevation }
    fn start_elevation_mut(&mut self) -> &mut f32 { &mut self.start_elevation }
    fn end_elevation(&self) -> f32 { self.end_elevation }
    fn end_elevation_mut(&mut self) -> &mut f32 { &mut self.end_elevation }
    fn cycles(&self) -> u32 { self.cycles }
    fn cycles_mut(&mut self) -> &mut u32 { &mut self.cycles }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for GimbalScanAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> GimbalScanAction {
            GimbalScanAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                payload_id: Arbitrary::arbitrary(_g),
                azimuth_slew_rate: Arbitrary::arbitrary(_g),
                elevation_slew_rate: Arbitrary::arbitrary(_g),
                start_azimuth: Arbitrary::arbitrary(_g),
                end_azimuth: Arbitrary::arbitrary(_g),
                start_elevation: Arbitrary::arbitrary(_g),
                end_elevation: Arbitrary::arbitrary(_g),
                cycles: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: GimbalScanAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: GimbalScanAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = GimbalScanAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
