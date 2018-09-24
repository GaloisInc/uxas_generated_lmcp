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
pub struct WaypointTransfer {
    pub entity_id: i64,
    pub waypoints: Vec<Box<::afrl::cmasi::waypoint::WaypointT>>,
    pub transfer_mode: ::afrl::cmasi::waypoint_transfer_mode::WaypointTransferMode,
}

impl PartialEq for WaypointTransfer {
    fn eq(&self, _other: &WaypointTransfer) -> bool {
        true
        && &self.entity_id == &_other.entity_id
        && &self.waypoints == &_other.waypoints
        && &self.transfer_mode == &_other.transfer_mode

    }
}

impl LmcpSubscription for WaypointTransfer {
    fn subscription() -> &'static str { "afrl.cmasi.WaypointTransfer" }
}

impl Struct for WaypointTransfer {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 59,
        }
    }
}

impl Lmcp for WaypointTransfer {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entity_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.waypoints.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.transfer_mode.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(WaypointTransfer, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == WaypointTransfer::struct_info() {
            let mut out: WaypointTransfer = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.entity_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::waypoint::WaypointT>>, usize) = Lmcp::deser(r)?;
                out.waypoints = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::waypoint_transfer_mode::WaypointTransferMode, usize) = Lmcp::deser(r)?;
                out.transfer_mode = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.entity_id.size();
        size += self.waypoints.size();
        size += self.transfer_mode.size();

        size
    }
}

pub trait WaypointTransferT: Debug + Send  {
    fn as_afrl_cmasi_waypoint_transfer(&self) -> Option<&WaypointTransfer> { None }
    fn as_mut_afrl_cmasi_waypoint_transfer(&mut self) -> Option<&mut WaypointTransfer> { None }
    fn entity_id(&self) -> i64;
    fn entity_id_mut(&mut self) -> &mut i64;
    fn waypoints(&self) -> &Vec<Box<::afrl::cmasi::waypoint::WaypointT>>;
    fn waypoints_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::waypoint::WaypointT>>;
    fn transfer_mode(&self) -> ::afrl::cmasi::waypoint_transfer_mode::WaypointTransferMode;
    fn transfer_mode_mut(&mut self) -> &mut ::afrl::cmasi::waypoint_transfer_mode::WaypointTransferMode;

}

impl Clone for Box<WaypointTransferT> {
    fn clone(&self) -> Box<WaypointTransferT> {
        if let Some(x) = WaypointTransferT::as_afrl_cmasi_waypoint_transfer(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<WaypointTransferT> {
    fn default() -> Box<WaypointTransferT> { Box::new(WaypointTransfer::default()) }
}

impl PartialEq for Box<WaypointTransferT> {
    fn eq(&self, other: &Box<WaypointTransferT>) -> bool {
        if let (Some(x), Some(y)) =
            (WaypointTransferT::as_afrl_cmasi_waypoint_transfer(self.as_ref()),
             WaypointTransferT::as_afrl_cmasi_waypoint_transfer(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<WaypointTransferT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = WaypointTransferT::as_afrl_cmasi_waypoint_transfer(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<WaypointTransferT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == WaypointTransfer::struct_info() {
            let (x, readb) = WaypointTransfer::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = WaypointTransferT::as_afrl_cmasi_waypoint_transfer(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl WaypointTransferT for WaypointTransfer {
    fn as_afrl_cmasi_waypoint_transfer(&self) -> Option<&WaypointTransfer> { Some(self) }
    fn as_mut_afrl_cmasi_waypoint_transfer(&mut self) -> Option<&mut WaypointTransfer> { Some(self) }
    fn entity_id(&self) -> i64 { self.entity_id }
    fn entity_id_mut(&mut self) -> &mut i64 { &mut self.entity_id }
    fn waypoints(&self) -> &Vec<Box<::afrl::cmasi::waypoint::WaypointT>> { &self.waypoints }
    fn waypoints_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::waypoint::WaypointT>> { &mut self.waypoints }
    fn transfer_mode(&self) -> ::afrl::cmasi::waypoint_transfer_mode::WaypointTransferMode { self.transfer_mode }
    fn transfer_mode_mut(&mut self) -> &mut ::afrl::cmasi::waypoint_transfer_mode::WaypointTransferMode { &mut self.transfer_mode }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for WaypointTransfer {
        fn arbitrary<G: Gen>(_g: &mut G) -> WaypointTransfer {
            WaypointTransfer {
                entity_id: Arbitrary::arbitrary(_g),
                waypoints: Vec::<::afrl::cmasi::waypoint::Waypoint>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::waypoint::WaypointT>).collect(),
                transfer_mode: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: WaypointTransfer) -> Result<TestResult, Error> {
            use std::u16;
            if x.waypoints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: WaypointTransfer) -> Result<TestResult, Error> {
            use std::u16;
            if x.waypoints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = WaypointTransfer::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
