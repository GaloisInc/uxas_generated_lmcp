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
pub struct SimulationTimeStepAcknowledgement {
    pub vehicle_id: i64,
    pub reported_time: i64,
}

impl PartialEq for SimulationTimeStepAcknowledgement {
    fn eq(&self, _other: &SimulationTimeStepAcknowledgement) -> bool {
        true
        && &self.vehicle_id == &_other.vehicle_id
        && &self.reported_time == &_other.reported_time

    }
}

impl LmcpSubscription for SimulationTimeStepAcknowledgement {
    fn subscription() -> &'static str { "uxas.messages.uxnative.SimulationTimeStepAcknowledgement" }
}

impl Struct for SimulationTimeStepAcknowledgement {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 16,
        }
    }
}

impl Lmcp for SimulationTimeStepAcknowledgement {
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
            let writeb: usize = self.reported_time.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SimulationTimeStepAcknowledgement, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SimulationTimeStepAcknowledgement::struct_info() {
            let mut out: SimulationTimeStepAcknowledgement = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.vehicle_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.reported_time = x;
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
        size += self.reported_time.size();

        size
    }
}

pub trait SimulationTimeStepAcknowledgementT: Debug + Send  {
    fn as_uxas_messages_uxnative_simulation_time_step_acknowledgement(&self) -> Option<&SimulationTimeStepAcknowledgement> { None }
    fn as_mut_uxas_messages_uxnative_simulation_time_step_acknowledgement(&mut self) -> Option<&mut SimulationTimeStepAcknowledgement> { None }
    fn vehicle_id(&self) -> i64;
    fn vehicle_id_mut(&mut self) -> &mut i64;
    fn reported_time(&self) -> i64;
    fn reported_time_mut(&mut self) -> &mut i64;

}

impl Clone for Box<SimulationTimeStepAcknowledgementT> {
    fn clone(&self) -> Box<SimulationTimeStepAcknowledgementT> {
        if let Some(x) = SimulationTimeStepAcknowledgementT::as_uxas_messages_uxnative_simulation_time_step_acknowledgement(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SimulationTimeStepAcknowledgementT> {
    fn default() -> Box<SimulationTimeStepAcknowledgementT> { Box::new(SimulationTimeStepAcknowledgement::default()) }
}

impl PartialEq for Box<SimulationTimeStepAcknowledgementT> {
    fn eq(&self, other: &Box<SimulationTimeStepAcknowledgementT>) -> bool {
        if let (Some(x), Some(y)) =
            (SimulationTimeStepAcknowledgementT::as_uxas_messages_uxnative_simulation_time_step_acknowledgement(self.as_ref()),
             SimulationTimeStepAcknowledgementT::as_uxas_messages_uxnative_simulation_time_step_acknowledgement(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SimulationTimeStepAcknowledgementT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SimulationTimeStepAcknowledgementT::as_uxas_messages_uxnative_simulation_time_step_acknowledgement(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SimulationTimeStepAcknowledgementT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SimulationTimeStepAcknowledgement::struct_info() {
            let (x, readb) = SimulationTimeStepAcknowledgement::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SimulationTimeStepAcknowledgementT::as_uxas_messages_uxnative_simulation_time_step_acknowledgement(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl SimulationTimeStepAcknowledgementT for SimulationTimeStepAcknowledgement {
    fn as_uxas_messages_uxnative_simulation_time_step_acknowledgement(&self) -> Option<&SimulationTimeStepAcknowledgement> { Some(self) }
    fn as_mut_uxas_messages_uxnative_simulation_time_step_acknowledgement(&mut self) -> Option<&mut SimulationTimeStepAcknowledgement> { Some(self) }
    fn vehicle_id(&self) -> i64 { self.vehicle_id }
    fn vehicle_id_mut(&mut self) -> &mut i64 { &mut self.vehicle_id }
    fn reported_time(&self) -> i64 { self.reported_time }
    fn reported_time_mut(&mut self) -> &mut i64 { &mut self.reported_time }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SimulationTimeStepAcknowledgement {
        fn arbitrary<G: Gen>(_g: &mut G) -> SimulationTimeStepAcknowledgement {
            SimulationTimeStepAcknowledgement {
                vehicle_id: Arbitrary::arbitrary(_g),
                reported_time: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SimulationTimeStepAcknowledgement) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SimulationTimeStepAcknowledgement) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SimulationTimeStepAcknowledgement::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
