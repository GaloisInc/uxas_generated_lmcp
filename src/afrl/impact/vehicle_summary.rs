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
pub struct VehicleSummary {
    pub vehicle_id: i64,
    pub destination_task_id: i64,
    pub initial_task_id: i64,
    pub initial_task_percentage: f32,
    pub estimate_time_to_task_percentage: i64,
    pub time_to_arrive: i64,
    pub time_on_task: i64,
    pub energy_remaining: f32,
    pub beyond_comm_range: bool,
    pub conflicts_with_roz: bool,
    pub rozids: Vec<i64>,
    pub waypoint_list: Vec<Box<::afrl::cmasi::waypoint::WaypointT>>,
    pub first_waypoint: i64,
}

impl PartialEq for VehicleSummary {
    fn eq(&self, _other: &VehicleSummary) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id
        && &self.destination_task_id == &_other.destination_task_id
        && &self.initial_task_id == &_other.initial_task_id
        && &self.initial_task_percentage == &_other.initial_task_percentage
        && &self.estimate_time_to_task_percentage == &_other.estimate_time_to_task_percentage
        && &self.time_to_arrive == &_other.time_to_arrive
        && &self.time_on_task == &_other.time_on_task
        && &self.energy_remaining == &_other.energy_remaining
        && &self.beyond_comm_range == &_other.beyond_comm_range
        && &self.conflicts_with_roz == &_other.conflicts_with_roz
        && &self.rozids == &_other.rozids
        && &self.waypoint_list == &_other.waypoint_list
        && &self.first_waypoint == &_other.first_waypoint

    }
}

impl LmcpSubscription for VehicleSummary {
    fn subscription() -> &'static str { "afrl.impact.VehicleSummary" }
}

impl Struct for VehicleSummary {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 15,
        }
    }
}

impl Lmcp for VehicleSummary {
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
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.destination_task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.initial_task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.initial_task_percentage.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.estimate_time_to_task_percentage.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_to_arrive.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_on_task.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.energy_remaining.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.beyond_comm_range.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.conflicts_with_roz.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.rozids.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.waypoint_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.first_waypoint.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(VehicleSummary, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == VehicleSummary::struct_info() {
            let mut out: VehicleSummary = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.destination_task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.initial_task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.initial_task_percentage = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.estimate_time_to_task_percentage = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time_to_arrive = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time_on_task = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.energy_remaining = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.beyond_comm_range = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.conflicts_with_roz = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.rozids = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::waypoint::WaypointT>>, usize) = Lmcp::deser(r)?;
                out.waypoint_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.first_waypoint = x;
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
        size += self.destination_task_id.size();
        size += self.initial_task_id.size();
        size += self.initial_task_percentage.size();
        size += self.estimate_time_to_task_percentage.size();
        size += self.time_to_arrive.size();
        size += self.time_on_task.size();
        size += self.energy_remaining.size();
        size += self.beyond_comm_range.size();
        size += self.conflicts_with_roz.size();
        size += self.rozids.size();
        size += self.waypoint_list.size();
        size += self.first_waypoint.size();

        size
    }
}

pub trait VehicleSummaryT: Debug + Send  {
    fn as_afrl_impact_vehicle_summary(&self) -> Option<&VehicleSummary> { None }
    fn as_mut_afrl_impact_vehicle_summary(&mut self) -> Option<&mut VehicleSummary> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn destination_task_id(&self) -> i64;
    fn destination_task_id_mut(&mut self) -> &mut i64;
    fn initial_task_id(&self) -> i64;
    fn initial_task_id_mut(&mut self) -> &mut i64;
    fn initial_task_percentage(&self) -> f32;
    fn initial_task_percentage_mut(&mut self) -> &mut f32;
    fn estimate_time_to_task_percentage(&self) -> i64;
    fn estimate_time_to_task_percentage_mut(&mut self) -> &mut i64;
    fn time_to_arrive(&self) -> i64;
    fn time_to_arrive_mut(&mut self) -> &mut i64;
    fn time_on_task(&self) -> i64;
    fn time_on_task_mut(&mut self) -> &mut i64;
    fn energy_remaining(&self) -> f32;
    fn energy_remaining_mut(&mut self) -> &mut f32;
    fn beyond_comm_range(&self) -> bool;
    fn beyond_comm_range_mut(&mut self) -> &mut bool;
    fn conflicts_with_roz(&self) -> bool;
    fn conflicts_with_roz_mut(&mut self) -> &mut bool;
    fn rozids(&self) -> &Vec<i64>;
    fn rozids_mut(&mut self) -> &mut Vec<i64>;
    fn waypoint_list(&self) -> &Vec<Box<::afrl::cmasi::waypoint::WaypointT>>;
    fn waypoint_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::waypoint::WaypointT>>;
    fn first_waypoint(&self) -> i64;
    fn first_waypoint_mut(&mut self) -> &mut i64;

}

impl Clone for Box<VehicleSummaryT> {
    fn clone(&self) -> Box<VehicleSummaryT> {
        if let Some(x) = VehicleSummaryT::as_afrl_impact_vehicle_summary(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<VehicleSummaryT> {
    fn default() -> Box<VehicleSummaryT> { Box::new(VehicleSummary::default()) }
}

impl PartialEq for Box<VehicleSummaryT> {
    fn eq(&self, other: &Box<VehicleSummaryT>) -> bool {
        if let (Some(x), Some(y)) =
            (VehicleSummaryT::as_afrl_impact_vehicle_summary(self.as_ref()),
             VehicleSummaryT::as_afrl_impact_vehicle_summary(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<VehicleSummaryT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = VehicleSummaryT::as_afrl_impact_vehicle_summary(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<VehicleSummaryT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == VehicleSummary::struct_info() {
            let (x, readb) = VehicleSummary::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = VehicleSummaryT::as_afrl_impact_vehicle_summary(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl VehicleSummaryT for VehicleSummary {
    fn as_afrl_impact_vehicle_summary(&self) -> Option<&VehicleSummary> { Some(self) }
    fn as_mut_afrl_impact_vehicle_summary(&mut self) -> Option<&mut VehicleSummary> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn destination_task_id(&self) -> i64 { self.destination_task_id }
    fn destination_task_id_mut(&mut self) -> &mut i64 { &mut self.destination_task_id }
    fn initial_task_id(&self) -> i64 { self.initial_task_id }
    fn initial_task_id_mut(&mut self) -> &mut i64 { &mut self.initial_task_id }
    fn initial_task_percentage(&self) -> f32 { self.initial_task_percentage }
    fn initial_task_percentage_mut(&mut self) -> &mut f32 { &mut self.initial_task_percentage }
    fn estimate_time_to_task_percentage(&self) -> i64 { self.estimate_time_to_task_percentage }
    fn estimate_time_to_task_percentage_mut(&mut self) -> &mut i64 { &mut self.estimate_time_to_task_percentage }
    fn time_to_arrive(&self) -> i64 { self.time_to_arrive }
    fn time_to_arrive_mut(&mut self) -> &mut i64 { &mut self.time_to_arrive }
    fn time_on_task(&self) -> i64 { self.time_on_task }
    fn time_on_task_mut(&mut self) -> &mut i64 { &mut self.time_on_task }
    fn energy_remaining(&self) -> f32 { self.energy_remaining }
    fn energy_remaining_mut(&mut self) -> &mut f32 { &mut self.energy_remaining }
    fn beyond_comm_range(&self) -> bool { self.beyond_comm_range }
    fn beyond_comm_range_mut(&mut self) -> &mut bool { &mut self.beyond_comm_range }
    fn conflicts_with_roz(&self) -> bool { self.conflicts_with_roz }
    fn conflicts_with_roz_mut(&mut self) -> &mut bool { &mut self.conflicts_with_roz }
    fn rozids(&self) -> &Vec<i64> { &self.rozids }
    fn rozids_mut(&mut self) -> &mut Vec<i64> { &mut self.rozids }
    fn waypoint_list(&self) -> &Vec<Box<::afrl::cmasi::waypoint::WaypointT>> { &self.waypoint_list }
    fn waypoint_list_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::waypoint::WaypointT>> { &mut self.waypoint_list }
    fn first_waypoint(&self) -> i64 { self.first_waypoint }
    fn first_waypoint_mut(&mut self) -> &mut i64 { &mut self.first_waypoint }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for VehicleSummary {
        fn arbitrary<G: Gen>(_g: &mut G) -> VehicleSummary {
            VehicleSummary {
                vehicle_id: Arbitrary::arbitrary(_g),
                destination_task_id: Arbitrary::arbitrary(_g),
                initial_task_id: Arbitrary::arbitrary(_g),
                initial_task_percentage: Arbitrary::arbitrary(_g),
                estimate_time_to_task_percentage: Arbitrary::arbitrary(_g),
                time_to_arrive: Arbitrary::arbitrary(_g),
                time_on_task: Arbitrary::arbitrary(_g),
                energy_remaining: Arbitrary::arbitrary(_g),
                beyond_comm_range: Arbitrary::arbitrary(_g),
                conflicts_with_roz: Arbitrary::arbitrary(_g),
                rozids: Arbitrary::arbitrary(_g),
                waypoint_list: Vec::<::afrl::cmasi::waypoint::Waypoint>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::waypoint::WaypointT>).collect(),
                first_waypoint: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: VehicleSummary) -> Result<TestResult, Error> {
            use std::u16;
            if x.rozids.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.waypoint_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: VehicleSummary) -> Result<TestResult, Error> {
            use std::u16;
            if x.rozids.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.waypoint_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = VehicleSummary::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
