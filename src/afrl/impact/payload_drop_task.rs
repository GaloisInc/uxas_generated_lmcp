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
pub struct PayloadDropTask {
    pub task_id: i64,
    pub label: Vec<u8>,
    pub eligible_entities: Vec<i64>,
    pub revisit_rate: f32,
    pub parameters: Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>>,
    pub priority: u8,
    pub required: bool,
    pub drop_location: Box<::afrl::cmasi::location3d::Location3DT>,
    pub bdalocation: Box<::afrl::cmasi::location3d::Location3DT>,
}

impl PartialEq for PayloadDropTask {
    fn eq(&self, _other: &PayloadDropTask) -> bool {
        true
        && &self.drop_location == &_other.drop_location
        && &self.bdalocation == &_other.bdalocation

    }
}

impl LmcpSubscription for PayloadDropTask {
    fn subscription() -> &'static str { "afrl.impact.PayloadDropTask" }
}

impl Struct for PayloadDropTask {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5281966179208134656u64,
            version: 14,
            struct_ty: 35,
        }
    }
}

impl Lmcp for PayloadDropTask {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.task_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.label.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.eligible_entities.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.revisit_rate.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.parameters.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.priority.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.required.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.drop_location.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.bdalocation.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(PayloadDropTask, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == PayloadDropTask::struct_info() {
            let mut out: PayloadDropTask = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.task_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<u8>, usize) = Lmcp::deser(r)?;
                out.label = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.eligible_entities = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.revisit_rate = x;
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
                let (x, readb): (u8, usize) = Lmcp::deser(r)?;
                out.priority = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (bool, usize) = Lmcp::deser(r)?;
                out.required = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.drop_location = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.bdalocation = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.task_id.size();
        size += self.label.size();
        size += self.eligible_entities.size();
        size += self.revisit_rate.size();
        size += self.parameters.size();
        size += self.priority.size();
        size += self.required.size();
        size += self.drop_location.size();
        size += self.bdalocation.size();

        size
    }
}

pub trait PayloadDropTaskT: Debug + Send + ::afrl::cmasi::task::TaskT {
    fn as_afrl_impact_payload_drop_task(&self) -> Option<&PayloadDropTask> { None }
    fn as_mut_afrl_impact_payload_drop_task(&mut self) -> Option<&mut PayloadDropTask> { None }
    fn drop_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn drop_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn bdalocation(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn bdalocation_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;

}

impl Clone for Box<PayloadDropTaskT> {
    fn clone(&self) -> Box<PayloadDropTaskT> {
        if let Some(x) = PayloadDropTaskT::as_afrl_impact_payload_drop_task(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PayloadDropTaskT> {
    fn default() -> Box<PayloadDropTaskT> { Box::new(PayloadDropTask::default()) }
}

impl PartialEq for Box<PayloadDropTaskT> {
    fn eq(&self, other: &Box<PayloadDropTaskT>) -> bool {
        if let (Some(x), Some(y)) =
            (PayloadDropTaskT::as_afrl_impact_payload_drop_task(self.as_ref()),
             PayloadDropTaskT::as_afrl_impact_payload_drop_task(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PayloadDropTaskT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PayloadDropTaskT::as_afrl_impact_payload_drop_task(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PayloadDropTaskT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == PayloadDropTask::struct_info() {
            let (x, readb) = PayloadDropTask::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PayloadDropTaskT::as_afrl_impact_payload_drop_task(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::task::TaskT for PayloadDropTask {
    fn as_afrl_impact_payload_drop_task(&self) -> Option<&PayloadDropTask> { Some(self) }
    fn as_mut_afrl_impact_payload_drop_task(&mut self) -> Option<&mut PayloadDropTask> { Some(self) }
    fn task_id(&self) -> i64 { self.task_id }
    fn task_id_mut(&mut self) -> &mut i64 { &mut self.task_id }
    fn label(&self) -> &Vec<u8> { &self.label }
    fn label_mut(&mut self) -> &mut Vec<u8> { &mut self.label }
    fn eligible_entities(&self) -> &Vec<i64> { &self.eligible_entities }
    fn eligible_entities_mut(&mut self) -> &mut Vec<i64> { &mut self.eligible_entities }
    fn revisit_rate(&self) -> f32 { self.revisit_rate }
    fn revisit_rate_mut(&mut self) -> &mut f32 { &mut self.revisit_rate }
    fn parameters(&self) -> &Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &self.parameters }
    fn parameters_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::key_value_pair::KeyValuePairT>> { &mut self.parameters }
    fn priority(&self) -> u8 { self.priority }
    fn priority_mut(&mut self) -> &mut u8 { &mut self.priority }
    fn required(&self) -> bool { self.required }
    fn required_mut(&mut self) -> &mut bool { &mut self.required }
}
impl PayloadDropTaskT for PayloadDropTask {
    fn as_afrl_impact_payload_drop_task(&self) -> Option<&PayloadDropTask> { Some(self) }
    fn as_mut_afrl_impact_payload_drop_task(&mut self) -> Option<&mut PayloadDropTask> { Some(self) }
    fn drop_location(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.drop_location }
    fn drop_location_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.drop_location }
    fn bdalocation(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.bdalocation }
    fn bdalocation_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.bdalocation }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for PayloadDropTask {
        fn arbitrary<G: Gen>(_g: &mut G) -> PayloadDropTask {
            PayloadDropTask {
                task_id: Arbitrary::arbitrary(_g),
                label: Arbitrary::arbitrary(_g),
                eligible_entities: Arbitrary::arbitrary(_g),
                revisit_rate: Arbitrary::arbitrary(_g),
                parameters: Vec::<::afrl::cmasi::key_value_pair::KeyValuePair>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::key_value_pair::KeyValuePairT>).collect(),
                priority: Arbitrary::arbitrary(_g),
                required: Arbitrary::arbitrary(_g),
                drop_location: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                bdalocation: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),

            }
        }
    }

    quickcheck! {
        fn serializes(x: PayloadDropTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: PayloadDropTask) -> Result<TestResult, Error> {
            use std::u16;
            if x.eligible_entities.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.parameters.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = PayloadDropTask::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
