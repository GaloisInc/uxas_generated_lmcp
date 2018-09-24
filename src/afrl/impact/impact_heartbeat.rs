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
pub struct ImpactHeartbeat {
    pub component_label: Vec<u8>,
    pub heartbeat_time: i64,
}

impl PartialEq for ImpactHeartbeat {
    fn eq(&self, _other: &ImpactHeartbeat) -> bool {
        true
        && &self.component_label == &_other.component_label
        && &self.heartbeat_time == &_other.heartbeat_time

    }
}

impl LmcpSubscription for ImpactHeartbeat {
    fn subscription() -> &'static str { "afrl.impact.ImpactHeartbeat" }
}

impl Struct for ImpactHeartbeat {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 13,
            struct_ty: 16,
        }
    }
}

impl Lmcp for ImpactHeartbeat {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.component_label.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.heartbeat_time.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(ImpactHeartbeat, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == ImpactHeartbeat::struct_info() {
            let mut out: ImpactHeartbeat = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.component_label = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.heartbeat_time = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.component_label.size();
        size += self.heartbeat_time.size();

        size
    }
}

pub trait ImpactHeartbeatT: Debug + Send  {
    fn as_afrl_impact_impact_heartbeat(&self) -> Option<&ImpactHeartbeat> { None }
    fn as_mut_afrl_impact_impact_heartbeat(&mut self) -> Option<&mut ImpactHeartbeat> { None }
    fn component_label(&self) -> &Vec<u8>;
    fn component_label_mut(&mut self) -> &mut Vec<u8>;
    fn heartbeat_time(&self) -> i64;
    fn heartbeat_time_mut(&mut self) -> &mut i64;

}

impl Clone for Box<ImpactHeartbeatT> {
    fn clone(&self) -> Box<ImpactHeartbeatT> {
        if let Some(x) = ImpactHeartbeatT::as_afrl_impact_impact_heartbeat(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<ImpactHeartbeatT> {
    fn default() -> Box<ImpactHeartbeatT> { Box::new(ImpactHeartbeat::default()) }
}

impl PartialEq for Box<ImpactHeartbeatT> {
    fn eq(&self, other: &Box<ImpactHeartbeatT>) -> bool {
        if let (Some(x), Some(y)) =
            (ImpactHeartbeatT::as_afrl_impact_impact_heartbeat(self.as_ref()),
             ImpactHeartbeatT::as_afrl_impact_impact_heartbeat(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<ImpactHeartbeatT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = ImpactHeartbeatT::as_afrl_impact_impact_heartbeat(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<ImpactHeartbeatT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == ImpactHeartbeat::struct_info() {
            let (x, readb) = ImpactHeartbeat::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = ImpactHeartbeatT::as_afrl_impact_impact_heartbeat(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ImpactHeartbeatT for ImpactHeartbeat {
    fn as_afrl_impact_impact_heartbeat(&self) -> Option<&ImpactHeartbeat> { Some(self) }
    fn as_mut_afrl_impact_impact_heartbeat(&mut self) -> Option<&mut ImpactHeartbeat> { Some(self) }
    fn component_label(&self) -> &Vec<u8> { &self.component_label }
    fn component_label_mut(&mut self) -> &mut Vec<u8> { &mut self.component_label }
    fn heartbeat_time(&self) -> i64 { self.heartbeat_time }
    fn heartbeat_time_mut(&mut self) -> &mut i64 { &mut self.heartbeat_time }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for ImpactHeartbeat {
        fn arbitrary<G: Gen>(_g: &mut G) -> ImpactHeartbeat {
            ImpactHeartbeat {
                component_label: Arbitrary::arbitrary(_g),
                heartbeat_time: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: ImpactHeartbeat) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: ImpactHeartbeat) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = ImpactHeartbeat::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
