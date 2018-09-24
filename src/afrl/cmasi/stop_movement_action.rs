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
pub struct StopMovementAction {
    pub associated_task_list: Vec<i64>,
    pub location: Option<Box<::afrl::cmasi::location3d::Location3DT>>,
}

impl PartialEq for StopMovementAction {
    fn eq(&self, _other: &StopMovementAction) -> bool {
        true
        && &self.location == &_other.location

    }
}

impl LmcpSubscription for StopMovementAction {
    fn subscription() -> &'static str { "afrl.cmasi.StopMovementAction" }
}

impl Struct for StopMovementAction {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 58,
        }
    }
}

impl Lmcp for StopMovementAction {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.associated_task_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.location.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(StopMovementAction, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == StopMovementAction::struct_info() {
            let mut out: StopMovementAction = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_task_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Option<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.location = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.associated_task_list.size();
        size += self.location.size();

        size
    }
}

pub trait StopMovementActionT: Debug + Send + ::afrl::cmasi::vehicle_action::VehicleActionT {
    fn as_afrl_cmasi_stop_movement_action(&self) -> Option<&StopMovementAction> { None }
    fn as_mut_afrl_cmasi_stop_movement_action(&mut self) -> Option<&mut StopMovementAction> { None }
    fn location(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn location_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>>;

}

impl Clone for Box<StopMovementActionT> {
    fn clone(&self) -> Box<StopMovementActionT> {
        if let Some(x) = StopMovementActionT::as_afrl_cmasi_stop_movement_action(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<StopMovementActionT> {
    fn default() -> Box<StopMovementActionT> { Box::new(StopMovementAction::default()) }
}

impl PartialEq for Box<StopMovementActionT> {
    fn eq(&self, other: &Box<StopMovementActionT>) -> bool {
        if let (Some(x), Some(y)) =
            (StopMovementActionT::as_afrl_cmasi_stop_movement_action(self.as_ref()),
             StopMovementActionT::as_afrl_cmasi_stop_movement_action(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<StopMovementActionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = StopMovementActionT::as_afrl_cmasi_stop_movement_action(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<StopMovementActionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == StopMovementAction::struct_info() {
            let (x, readb) = StopMovementAction::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = StopMovementActionT::as_afrl_cmasi_stop_movement_action(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::vehicle_action::VehicleActionT for StopMovementAction {
    fn as_afrl_cmasi_stop_movement_action(&self) -> Option<&StopMovementAction> { Some(self) }
    fn as_mut_afrl_cmasi_stop_movement_action(&mut self) -> Option<&mut StopMovementAction> { Some(self) }
    fn associated_task_list(&self) -> &Vec<i64> { &self.associated_task_list }
    fn associated_task_list_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_task_list }
}
impl StopMovementActionT for StopMovementAction {
    fn as_afrl_cmasi_stop_movement_action(&self) -> Option<&StopMovementAction> { Some(self) }
    fn as_mut_afrl_cmasi_stop_movement_action(&mut self) -> Option<&mut StopMovementAction> { Some(self) }
    fn location(&self) -> &Option<Box<::afrl::cmasi::location3d::Location3DT>> { &self.location }
    fn location_mut(&mut self) -> &mut Option<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.location }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for StopMovementAction {
        fn arbitrary<G: Gen>(_g: &mut G) -> StopMovementAction {
            StopMovementAction {
                associated_task_list: Arbitrary::arbitrary(_g),
                location: {
                    if _g.gen() {
                        Some(Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)))
                    } else {
                        None
                    }
                },

            }
        }
    }

    quickcheck! {
        fn serializes(x: StopMovementAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: StopMovementAction) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_task_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = StopMovementAction::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
