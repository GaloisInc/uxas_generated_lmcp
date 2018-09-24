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
pub struct AutopilotKeepAlive {
    pub autopilot_enabled: bool,
    pub speed_authorized: bool,
    pub gimbal_enabled: bool,
    pub time_sent: i64,
    pub speed_override: f32,
    pub alt_override: f32,
}

impl PartialEq for AutopilotKeepAlive {
    fn eq(&self, _other: &AutopilotKeepAlive) -> bool {
        true
        && &self.autopilot_enabled == &_other.autopilot_enabled
        && &self.speed_authorized == &_other.speed_authorized
        && &self.gimbal_enabled == &_other.gimbal_enabled
        && &self.time_sent == &_other.time_sent
        && &self.speed_override == &_other.speed_override
        && &self.alt_override == &_other.alt_override

    }
}

impl LmcpSubscription for AutopilotKeepAlive {
    fn subscription() -> &'static str { "uxas.messages.uxnative.AutopilotKeepAlive" }
}

impl Struct for AutopilotKeepAlive {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 12,
        }
    }
}

impl Lmcp for AutopilotKeepAlive {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.autopilot_enabled.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.speed_authorized.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.gimbal_enabled.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.time_sent.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.speed_override.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.alt_override.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(AutopilotKeepAlive, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == AutopilotKeepAlive::struct_info() {
            let mut out: AutopilotKeepAlive = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.autopilot_enabled = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.speed_authorized = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.gimbal_enabled = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.time_sent = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.speed_override = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.alt_override = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.autopilot_enabled.size();
        size += self.speed_authorized.size();
        size += self.gimbal_enabled.size();
        size += self.time_sent.size();
        size += self.speed_override.size();
        size += self.alt_override.size();

        size
    }
}

pub trait AutopilotKeepAliveT: Debug + Send  {
    fn as_uxas_messages_uxnative_autopilot_keep_alive(&self) -> Option<&AutopilotKeepAlive> { None }
    fn as_mut_uxas_messages_uxnative_autopilot_keep_alive(&mut self) -> Option<&mut AutopilotKeepAlive> { None }
    fn autopilot_enabled(&self) -> bool;
    fn autopilot_enabled_mut(&mut self) -> &mut bool;
    fn speed_authorized(&self) -> bool;
    fn speed_authorized_mut(&mut self) -> &mut bool;
    fn gimbal_enabled(&self) -> bool;
    fn gimbal_enabled_mut(&mut self) -> &mut bool;
    fn time_sent(&self) -> i64;
    fn time_sent_mut(&mut self) -> &mut i64;
    fn speed_override(&self) -> f32;
    fn speed_override_mut(&mut self) -> &mut f32;
    fn alt_override(&self) -> f32;
    fn alt_override_mut(&mut self) -> &mut f32;

}

impl Clone for Box<AutopilotKeepAliveT> {
    fn clone(&self) -> Box<AutopilotKeepAliveT> {
        if let Some(x) = AutopilotKeepAliveT::as_uxas_messages_uxnative_autopilot_keep_alive(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<AutopilotKeepAliveT> {
    fn default() -> Box<AutopilotKeepAliveT> { Box::new(AutopilotKeepAlive::default()) }
}

impl PartialEq for Box<AutopilotKeepAliveT> {
    fn eq(&self, other: &Box<AutopilotKeepAliveT>) -> bool {
        if let (Some(x), Some(y)) =
            (AutopilotKeepAliveT::as_uxas_messages_uxnative_autopilot_keep_alive(self.as_ref()),
             AutopilotKeepAliveT::as_uxas_messages_uxnative_autopilot_keep_alive(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<AutopilotKeepAliveT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = AutopilotKeepAliveT::as_uxas_messages_uxnative_autopilot_keep_alive(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<AutopilotKeepAliveT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == AutopilotKeepAlive::struct_info() {
            let (x, readb) = AutopilotKeepAlive::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = AutopilotKeepAliveT::as_uxas_messages_uxnative_autopilot_keep_alive(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl AutopilotKeepAliveT for AutopilotKeepAlive {
    fn as_uxas_messages_uxnative_autopilot_keep_alive(&self) -> Option<&AutopilotKeepAlive> { Some(self) }
    fn as_mut_uxas_messages_uxnative_autopilot_keep_alive(&mut self) -> Option<&mut AutopilotKeepAlive> { Some(self) }
    fn autopilot_enabled(&self) -> bool { self.autopilot_enabled }
    fn autopilot_enabled_mut(&mut self) -> &mut bool { &mut self.autopilot_enabled }
    fn speed_authorized(&self) -> bool { self.speed_authorized }
    fn speed_authorized_mut(&mut self) -> &mut bool { &mut self.speed_authorized }
    fn gimbal_enabled(&self) -> bool { self.gimbal_enabled }
    fn gimbal_enabled_mut(&mut self) -> &mut bool { &mut self.gimbal_enabled }
    fn time_sent(&self) -> i64 { self.time_sent }
    fn time_sent_mut(&mut self) -> &mut i64 { &mut self.time_sent }
    fn speed_override(&self) -> f32 { self.speed_override }
    fn speed_override_mut(&mut self) -> &mut f32 { &mut self.speed_override }
    fn alt_override(&self) -> f32 { self.alt_override }
    fn alt_override_mut(&mut self) -> &mut f32 { &mut self.alt_override }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AutopilotKeepAlive {
        fn arbitrary<G: Gen>(_g: &mut G) -> AutopilotKeepAlive {
            AutopilotKeepAlive {
                autopilot_enabled: Arbitrary::arbitrary(_g),
                speed_authorized: Arbitrary::arbitrary(_g),
                gimbal_enabled: Arbitrary::arbitrary(_g),
                time_sent: Arbitrary::arbitrary(_g),
                speed_override: Arbitrary::arbitrary(_g),
                alt_override: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: AutopilotKeepAlive) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AutopilotKeepAlive) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AutopilotKeepAlive::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
