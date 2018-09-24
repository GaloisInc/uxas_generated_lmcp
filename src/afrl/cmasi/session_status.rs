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
pub struct SessionStatus {
    pub state: ::afrl::cmasi::simulation_status_type::SimulationStatusType,
    pub start_time: i64,
    pub scenario_time: i64,
    pub real_time_multiple: f32,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
}

impl PartialEq for SessionStatus {
    fn eq(&self, _other: &SessionStatus) -> bool {
        true
        && &self.state == &_other.state
        && &self.start_time == &_other.start_time
        && &self.scenario_time == &_other.scenario_time
        && &self.real_time_multiple == &_other.real_time_multiple
        && &self.parameters == &_other.parameters

    }
}

impl LmcpSubscription for SessionStatus {
    fn subscription() -> &'static str { "afrl.cmasi.SessionStatus" }
}

impl Struct for SessionStatus {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 46,
        }
    }
}

impl Lmcp for SessionStatus {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.state.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_time.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.scenario_time.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.real_time_multiple.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.parameters.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(SessionStatus, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == SessionStatus::struct_info() {
            let mut out: SessionStatus = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::simulation_status_type::SimulationStatusType, usize) = Lmcp::deser(r)?;
                out.state = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.start_time = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.scenario_time = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.real_time_multiple = x;
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
        size += self.state.size();
        size += self.start_time.size();
        size += self.scenario_time.size();
        size += self.real_time_multiple.size();
        size += self.parameters.size();

        size
    }
}

pub trait SessionStatusT: Debug + Send  {
    fn as_afrl_cmasi_session_status(&self) -> Option<&SessionStatus> { None }
    fn as_mut_afrl_cmasi_session_status(&mut self) -> Option<&mut SessionStatus> { None }
    fn state(&self) -> ::afrl::cmasi::simulation_status_type::SimulationStatusType;
    fn state_mut(&mut self) -> &mut ::afrl::cmasi::simulation_status_type::SimulationStatusType;
    fn start_time(&self) -> i64;
    fn start_time_mut(&mut self) -> &mut i64;
    fn scenario_time(&self) -> i64;
    fn scenario_time_mut(&mut self) -> &mut i64;
    fn real_time_multiple(&self) -> f32;
    fn real_time_multiple_mut(&mut self) -> &mut f32;
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>;

}

impl Clone for Box<SessionStatusT> {
    fn clone(&self) -> Box<SessionStatusT> {
        if let Some(x) = SessionStatusT::as_afrl_cmasi_session_status(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<SessionStatusT> {
    fn default() -> Box<SessionStatusT> { Box::new(SessionStatus::default()) }
}

impl PartialEq for Box<SessionStatusT> {
    fn eq(&self, other: &Box<SessionStatusT>) -> bool {
        if let (Some(x), Some(y)) =
            (SessionStatusT::as_afrl_cmasi_session_status(self.as_ref()),
             SessionStatusT::as_afrl_cmasi_session_status(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<SessionStatusT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = SessionStatusT::as_afrl_cmasi_session_status(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<SessionStatusT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == SessionStatus::struct_info() {
            let (x, readb) = SessionStatus::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = SessionStatusT::as_afrl_cmasi_session_status(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl SessionStatusT for SessionStatus {
    fn as_afrl_cmasi_session_status(&self) -> Option<&SessionStatus> { Some(self) }
    fn as_mut_afrl_cmasi_session_status(&mut self) -> Option<&mut SessionStatus> { Some(self) }
    fn state(&self) -> ::afrl::cmasi::simulation_status_type::SimulationStatusType { self.state }
    fn state_mut(&mut self) -> &mut ::afrl::cmasi::simulation_status_type::SimulationStatusType { &mut self.state }
    fn start_time(&self) -> i64 { self.start_time }
    fn start_time_mut(&mut self) -> &mut i64 { &mut self.start_time }
    fn scenario_time(&self) -> i64 { self.scenario_time }
    fn scenario_time_mut(&mut self) -> &mut i64 { &mut self.scenario_time }
    fn real_time_multiple(&self) -> f32 { self.real_time_multiple }
    fn real_time_multiple_mut(&mut self) -> &mut f32 { &mut self.real_time_multiple }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for SessionStatus {
        fn arbitrary<G: Gen>(_g: &mut G) -> SessionStatus {
            SessionStatus {
                state: Arbitrary::arbitrary(_g),
                start_time: Arbitrary::arbitrary(_g),
                scenario_time: Arbitrary::arbitrary(_g),
                real_time_multiple: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: SessionStatus) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: SessionStatus) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = SessionStatus::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
