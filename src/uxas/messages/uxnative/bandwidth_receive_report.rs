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
pub struct BandwidthReceiveReport {
    pub entity_sender: Box<::uxas::messages::uxnative::entity_location::EntityLocationT>,
    pub entity_receiver: Box<::uxas::messages::uxnative::entity_location::EntityLocationT>,
    pub transfer_payload_size: u32,
}

impl PartialEq for BandwidthReceiveReport {
    fn eq(&self, _other: &BandwidthReceiveReport) -> bool {
        true
        && &self.entity_sender == &_other.entity_sender
        && &self.entity_receiver == &_other.entity_receiver
        && &self.transfer_payload_size == &_other.transfer_payload_size

    }
}

impl LmcpSubscription for BandwidthReceiveReport {
    fn subscription() -> &'static str { "uxas.messages.uxnative.BandwidthReceiveReport" }
}

impl Struct for BandwidthReceiveReport {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 6149751333668345413u64,
            version: 8,
            struct_ty: 9,
        }
    }
}

impl Lmcp for BandwidthReceiveReport {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entity_sender.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.entity_receiver.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.transfer_payload_size.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(BandwidthReceiveReport, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == BandwidthReceiveReport::struct_info() {
            let mut out: BandwidthReceiveReport = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::uxas::messages::uxnative::entity_location::EntityLocationT>, usize) = Lmcp::deser(r)?;
                out.entity_sender = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::uxas::messages::uxnative::entity_location::EntityLocationT>, usize) = Lmcp::deser(r)?;
                out.entity_receiver = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (u32, usize) = Lmcp::deser(r)?;
                out.transfer_payload_size = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.entity_sender.size();
        size += self.entity_receiver.size();
        size += self.transfer_payload_size.size();

        size
    }
}

pub trait BandwidthReceiveReportT: Debug + Send  {
    fn as_uxas_messages_uxnative_bandwidth_receive_report(&self) -> Option<&BandwidthReceiveReport> { None }
    fn as_mut_uxas_messages_uxnative_bandwidth_receive_report(&mut self) -> Option<&mut BandwidthReceiveReport> { None }
    fn entity_sender(&self) -> &Box<::uxas::messages::uxnative::entity_location::EntityLocationT>;
    fn entity_sender_mut(&mut self) -> &mut Box<::uxas::messages::uxnative::entity_location::EntityLocationT>;
    fn entity_receiver(&self) -> &Box<::uxas::messages::uxnative::entity_location::EntityLocationT>;
    fn entity_receiver_mut(&mut self) -> &mut Box<::uxas::messages::uxnative::entity_location::EntityLocationT>;
    fn transfer_payload_size(&self) -> u32;
    fn transfer_payload_size_mut(&mut self) -> &mut u32;

}

impl Clone for Box<BandwidthReceiveReportT> {
    fn clone(&self) -> Box<BandwidthReceiveReportT> {
        if let Some(x) = BandwidthReceiveReportT::as_uxas_messages_uxnative_bandwidth_receive_report(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<BandwidthReceiveReportT> {
    fn default() -> Box<BandwidthReceiveReportT> { Box::new(BandwidthReceiveReport::default()) }
}

impl PartialEq for Box<BandwidthReceiveReportT> {
    fn eq(&self, other: &Box<BandwidthReceiveReportT>) -> bool {
        if let (Some(x), Some(y)) =
            (BandwidthReceiveReportT::as_uxas_messages_uxnative_bandwidth_receive_report(self.as_ref()),
             BandwidthReceiveReportT::as_uxas_messages_uxnative_bandwidth_receive_report(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<BandwidthReceiveReportT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = BandwidthReceiveReportT::as_uxas_messages_uxnative_bandwidth_receive_report(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<BandwidthReceiveReportT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == BandwidthReceiveReport::struct_info() {
            let (x, readb) = BandwidthReceiveReport::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = BandwidthReceiveReportT::as_uxas_messages_uxnative_bandwidth_receive_report(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl BandwidthReceiveReportT for BandwidthReceiveReport {
    fn as_uxas_messages_uxnative_bandwidth_receive_report(&self) -> Option<&BandwidthReceiveReport> { Some(self) }
    fn as_mut_uxas_messages_uxnative_bandwidth_receive_report(&mut self) -> Option<&mut BandwidthReceiveReport> { Some(self) }
    fn entity_sender(&self) -> &Box<::uxas::messages::uxnative::entity_location::EntityLocationT> { &self.entity_sender }
    fn entity_sender_mut(&mut self) -> &mut Box<::uxas::messages::uxnative::entity_location::EntityLocationT> { &mut self.entity_sender }
    fn entity_receiver(&self) -> &Box<::uxas::messages::uxnative::entity_location::EntityLocationT> { &self.entity_receiver }
    fn entity_receiver_mut(&mut self) -> &mut Box<::uxas::messages::uxnative::entity_location::EntityLocationT> { &mut self.entity_receiver }
    fn transfer_payload_size(&self) -> u32 { self.transfer_payload_size }
    fn transfer_payload_size_mut(&mut self) -> &mut u32 { &mut self.transfer_payload_size }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for BandwidthReceiveReport {
        fn arbitrary<G: Gen>(_g: &mut G) -> BandwidthReceiveReport {
            BandwidthReceiveReport {
                entity_sender: Box::new(::uxas::messages::uxnative::entity_location::EntityLocation::arbitrary(_g)),
                entity_receiver: Box::new(::uxas::messages::uxnative::entity_location::EntityLocation::arbitrary(_g)),
                transfer_payload_size: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: BandwidthReceiveReport) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: BandwidthReceiveReport) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = BandwidthReceiveReport::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
