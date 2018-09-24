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
pub struct RadioState {
    pub payload_id: i64,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub enabled: bool,
    pub in_range: bool,
}

impl PartialEq for RadioState {
    fn eq(&self, _other: &RadioState) -> bool {
        true
        && &self.enabled == &_other.enabled
        && &self.in_range == &_other.in_range

    }
}

impl LmcpSubscription for RadioState {
    fn subscription() -> &'static str { "afrl.impact.RadioState" }
}

impl Struct for RadioState {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 4,
        }
    }
}

impl Lmcp for RadioState {
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
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.enabled.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.in_range.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(RadioState, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == RadioState::struct_info() {
            let mut out: RadioState = Default::default();
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
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.enabled = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.in_range = x;
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
        size += self.enabled.size();
        size += self.in_range.size();

        size
    }
}

pub trait RadioStateT: Debug + Send + ::afrl::cmasi::payload_state::PayloadStateT {
    fn as_afrl_impact_radio_state(&self) -> Option<&RadioState> { None }
    fn as_mut_afrl_impact_radio_state(&mut self) -> Option<&mut RadioState> { None }
    fn enabled(&self) -> bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn in_range(&self) -> bool;
    fn in_range_mut(&mut self) -> &mut bool;

}

impl Clone for Box<RadioStateT> {
    fn clone(&self) -> Box<RadioStateT> {
        if let Some(x) = RadioStateT::as_afrl_impact_radio_state(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RadioStateT> {
    fn default() -> Box<RadioStateT> { Box::new(RadioState::default()) }
}

impl PartialEq for Box<RadioStateT> {
    fn eq(&self, other: &Box<RadioStateT>) -> bool {
        if let (Some(x), Some(y)) =
            (RadioStateT::as_afrl_impact_radio_state(self.as_ref()),
             RadioStateT::as_afrl_impact_radio_state(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RadioStateT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RadioStateT::as_afrl_impact_radio_state(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RadioStateT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == RadioState::struct_info() {
            let (x, readb) = RadioState::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RadioStateT::as_afrl_impact_radio_state(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::payload_state::PayloadStateT for RadioState {
    fn as_afrl_impact_radio_state(&self) -> Option<&RadioState> { Some(self) }
    fn as_mut_afrl_impact_radio_state(&mut self) -> Option<&mut RadioState> { Some(self) }
    fn payload_id(&self) -> i64 { self.payload_id }
    fn payload_id_mut(&mut self) -> &mut i64 { &mut self.payload_id }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
}
impl RadioStateT for RadioState {
    fn as_afrl_impact_radio_state(&self) -> Option<&RadioState> { Some(self) }
    fn as_mut_afrl_impact_radio_state(&mut self) -> Option<&mut RadioState> { Some(self) }
    fn enabled(&self) -> bool { self.enabled }
    fn enabled_mut(&mut self) -> &mut bool { &mut self.enabled }
    fn in_range(&self) -> bool { self.in_range }
    fn in_range_mut(&mut self) -> &mut bool { &mut self.in_range }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for RadioState {
        fn arbitrary<G: Gen>(_g: &mut G) -> RadioState {
            RadioState {
                payload_id: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                enabled: Arbitrary::arbitrary(_g),
                in_range: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: RadioState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: RadioState) -> Result<TestResult, Error> {
            use std::u16;
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = RadioState::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
