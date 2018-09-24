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
pub struct RoutePlan {
    pub route_id: i64,
    pub waypoints: Vec<Box<::afrl::cmasi::waypoint::WaypointT>>,
    pub route_cost: i64,
    pub route_error: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
}

impl PartialEq for RoutePlan {
    fn eq(&self, _other: &RoutePlan) -> bool {
        true
        && &self.route_id == &_other.route_id
        && &self.waypoints == &_other.waypoints
        && &self.route_cost == &_other.route_cost
        && &self.route_error == &_other.route_error

    }
}

impl LmcpSubscription for RoutePlan {
    fn subscription() -> &'static str { "uxas.messages.route.RoutePlan" }
}

impl Struct for RoutePlan {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 7,
        }
    }
}

impl Lmcp for RoutePlan {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.route_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.waypoints.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.route_cost.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.route_error.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RoutePlan, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RoutePlan::struct_info() {
            let mut out: RoutePlan = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.route_id = x;
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
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.route_cost = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>, usize) = Lmcp::deser(r)?;
                out.route_error = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.route_id.size();
        size += self.waypoints.size();
        size += self.route_cost.size();
        size += self.route_error.size();

        size
    }
}

pub trait RoutePlanT: Debug + Send  {
    fn as_uxas_messages_route_route_plan(&self) -> Option<&RoutePlan> { None }
    fn as_mut_uxas_messages_route_route_plan(&mut self) -> Option<&mut RoutePlan> { None }
    fn route_id(&self) -> i64;
    fn route_id_mut(&mut self) -> &mut i64;
    fn waypoints(&self) -> &Vec<Box<::afrl::cmasi::waypoint::WaypointT>>;
    fn waypoints_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::waypoint::WaypointT>>;
    fn route_cost(&self) -> i64;
    fn route_cost_mut(&mut self) -> &mut i64;
    fn route_error(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn route_error_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;

}

impl Clone for Box<RoutePlanT> {
    fn clone(&self) -> Box<RoutePlanT> {
        if let Some(x) = RoutePlanT::as_uxas_messages_route_route_plan(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RoutePlanT> {
    fn default() -> Box<RoutePlanT> { Box::new(RoutePlan::default()) }
}

impl PartialEq for Box<RoutePlanT> {
    fn eq(&self, other: &Box<RoutePlanT>) -> bool {
        if let (Some(x), Some(y)) =
            (RoutePlanT::as_uxas_messages_route_route_plan(self.as_ref()),
             RoutePlanT::as_uxas_messages_route_route_plan(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RoutePlanT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RoutePlanT::as_uxas_messages_route_route_plan(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RoutePlanT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RoutePlan::struct_info() {
            let (x, readb) = RoutePlan::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RoutePlanT::as_uxas_messages_route_route_plan(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl RoutePlanT for RoutePlan {
    fn as_uxas_messages_route_route_plan(&self) -> Option<&RoutePlan> { Some(self) }
    fn as_mut_uxas_messages_route_route_plan(&mut self) -> Option<&mut RoutePlan> { Some(self) }
    fn route_id(&self) -> i64 { self.route_id }
    fn route_id_mut(&mut self) -> &mut i64 { &mut self.route_id }
    fn waypoints(&self) -> &Vec<Box<::afrl::cmasi::waypoint::WaypointT>> { &self.waypoints }
    fn waypoints_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::waypoint::WaypointT>> { &mut self.waypoints }
    fn route_cost(&self) -> i64 { self.route_cost }
    fn route_cost_mut(&mut self) -> &mut i64 { &mut self.route_cost }
    fn route_error(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.route_error }
    fn route_error_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.route_error }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RoutePlan {
        fn arbitrary<G: Gen>(_g: &mut G) -> RoutePlan {
            RoutePlan {
                route_id: Arbitrary::arbitrary(_g),
                waypoints: Vec::<::afrl::cmasi::waypoint::Waypoint>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::waypoint::WaypointT>).collect(),
                route_cost: Arbitrary::arbitrary(_g),
                route_error: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RoutePlan) -> Result<TestResult, Error> {
            use std::u16;
            if x.waypoints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.route_error.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RoutePlan) -> Result<TestResult, Error> {
            use std::u16;
            if x.waypoints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.route_error.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RoutePlan::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
