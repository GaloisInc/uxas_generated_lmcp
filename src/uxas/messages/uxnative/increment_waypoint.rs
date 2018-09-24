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

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct IncrementWaypoint {
    pub vehicle_id: i64,
}

impl PartialEq for IncrementWaypoint {
    fn eq(&self, _other: &IncrementWaypoint) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id

    }
}

impl LmcpSubscription for IncrementWaypoint {
    fn subscription() -> &'static str { "uxas.messages.uxnative.IncrementWaypoint" }
}

impl Struct for IncrementWaypoint {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 5,
        }
    }
}

impl Lmcp for IncrementWaypoint {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.vehicle_id.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(IncrementWaypoint, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == IncrementWaypoint::struct_info() {
            let mut out: IncrementWaypoint = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.vehicle_id.size();

        size
    }
}

pub trait IncrementWaypointT: Debug + Send  {
    fn as_uxas_messages_uxnative_increment_waypoint(&self) -> Option<&IncrementWaypoint> { None }
    fn as_mut_uxas_messages_uxnative_increment_waypoint(&mut self) -> Option<&mut IncrementWaypoint> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;

}

impl Clone for Box<IncrementWaypointT> {
    fn clone(&self) -> Box<IncrementWaypointT> {
        if let Some(x) = IncrementWaypointT::as_uxas_messages_uxnative_increment_waypoint(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<IncrementWaypointT> {
    fn default() -> Box<IncrementWaypointT> { Box::new(IncrementWaypoint::default()) }
}

impl PartialEq for Box<IncrementWaypointT> {
    fn eq(&self, other: &Box<IncrementWaypointT>) -> bool {
        if let (Some(x), Some(y)) =
            (IncrementWaypointT::as_uxas_messages_uxnative_increment_waypoint(self.as_ref()),
             IncrementWaypointT::as_uxas_messages_uxnative_increment_waypoint(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<IncrementWaypointT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = IncrementWaypointT::as_uxas_messages_uxnative_increment_waypoint(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<IncrementWaypointT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == IncrementWaypoint::struct_info() {
            let (x, readb) = IncrementWaypoint::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = IncrementWaypointT::as_uxas_messages_uxnative_increment_waypoint(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl IncrementWaypointT for IncrementWaypoint {
    fn as_uxas_messages_uxnative_increment_waypoint(&self) -> Option<&IncrementWaypoint> { Some(self) }
    fn as_mut_uxas_messages_uxnative_increment_waypoint(&mut self) -> Option<&mut IncrementWaypoint> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for IncrementWaypoint {
        fn arbitrary<G: Gen>(_g: &mut G) -> IncrementWaypoint {
            IncrementWaypoint {
                vehicle_id: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: IncrementWaypoint) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: IncrementWaypoint) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = IncrementWaypoint::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
