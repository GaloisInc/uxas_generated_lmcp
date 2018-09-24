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
pub struct AbstractGeometry {
}

impl PartialEq for AbstractGeometry {
    fn eq(&self, _other: &AbstractGeometry) -> bool {
        true

    }
}

impl LmcpSubscription for AbstractGeometry {
    fn subscription() -> &'static str { "afrl.cmasi.AbstractGeometry" }
}

impl Struct for AbstractGeometry {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 1,
        }
    }
}

impl Lmcp for AbstractGeometry {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(AbstractGeometry, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == AbstractGeometry::struct_info() {
            let out: AbstractGeometry = Default::default();

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let size = 15;

        size
    }
}

pub trait AbstractGeometryT: Debug + Send  {
    fn as_afrl_cmasi_abstract_geometry(&self) -> Option<&AbstractGeometry> { None }
    fn as_mut_afrl_cmasi_abstract_geometry(&mut self) -> Option<&mut AbstractGeometry> { None }
    fn as_afrl_cmasi_rectangle(&self) -> Option<&::afrl::cmasi::rectangle::Rectangle> { None }
    fn as_mut_afrl_cmasi_rectangle(&mut self) -> Option<&mut ::afrl::cmasi::rectangle::Rectangle> { None }
    fn as_afrl_cmasi_circle(&self) -> Option<&::afrl::cmasi::circle::Circle> { None }
    fn as_mut_afrl_cmasi_circle(&mut self) -> Option<&mut ::afrl::cmasi::circle::Circle> { None }
    fn as_afrl_cmasi_polygon(&self) -> Option<&::afrl::cmasi::polygon::Polygon> { None }
    fn as_mut_afrl_cmasi_polygon(&mut self) -> Option<&mut ::afrl::cmasi::polygon::Polygon> { None }

}

impl Clone for Box<AbstractGeometryT> {
    fn clone(&self) -> Box<AbstractGeometryT> {
        if let Some(x) = AbstractGeometryT::as_afrl_cmasi_abstract_geometry(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = AbstractGeometryT::as_afrl_cmasi_rectangle(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = AbstractGeometryT::as_afrl_cmasi_circle(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = AbstractGeometryT::as_afrl_cmasi_polygon(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<AbstractGeometryT> {
    fn default() -> Box<AbstractGeometryT> { Box::new(AbstractGeometry::default()) }
}

impl PartialEq for Box<AbstractGeometryT> {
    fn eq(&self, other: &Box<AbstractGeometryT>) -> bool {
        if let (Some(x), Some(y)) =
            (AbstractGeometryT::as_afrl_cmasi_abstract_geometry(self.as_ref()),
             AbstractGeometryT::as_afrl_cmasi_abstract_geometry(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (AbstractGeometryT::as_afrl_cmasi_rectangle(self.as_ref()),
             AbstractGeometryT::as_afrl_cmasi_rectangle(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (AbstractGeometryT::as_afrl_cmasi_circle(self.as_ref()),
             AbstractGeometryT::as_afrl_cmasi_circle(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (AbstractGeometryT::as_afrl_cmasi_polygon(self.as_ref()),
             AbstractGeometryT::as_afrl_cmasi_polygon(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<AbstractGeometryT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = AbstractGeometryT::as_afrl_cmasi_abstract_geometry(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = AbstractGeometryT::as_afrl_cmasi_rectangle(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = AbstractGeometryT::as_afrl_cmasi_circle(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = AbstractGeometryT::as_afrl_cmasi_polygon(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<AbstractGeometryT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == AbstractGeometry::struct_info() {
            let (x, readb) = AbstractGeometry::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::rectangle::Rectangle::struct_info() {
            let (x, readb) = ::afrl::cmasi::rectangle::Rectangle::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::circle::Circle::struct_info() {
            let (x, readb) = ::afrl::cmasi::circle::Circle::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::polygon::Polygon::struct_info() {
            let (x, readb) = ::afrl::cmasi::polygon::Polygon::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = AbstractGeometryT::as_afrl_cmasi_abstract_geometry(self.as_ref()) {
            x.size()
        } else if let Some(x) = AbstractGeometryT::as_afrl_cmasi_rectangle(self.as_ref()) {
            x.size()
        } else if let Some(x) = AbstractGeometryT::as_afrl_cmasi_circle(self.as_ref()) {
            x.size()
        } else if let Some(x) = AbstractGeometryT::as_afrl_cmasi_polygon(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl AbstractGeometryT for AbstractGeometry {
    fn as_afrl_cmasi_abstract_geometry(&self) -> Option<&AbstractGeometry> { Some(self) }
    fn as_mut_afrl_cmasi_abstract_geometry(&mut self) -> Option<&mut AbstractGeometry> { Some(self) }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for AbstractGeometry {
        fn arbitrary<G: Gen>(_g: &mut G) -> AbstractGeometry {
            AbstractGeometry {

            }
        }
    }

    quickcheck! {
        fn serializes(x: AbstractGeometry) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: AbstractGeometry) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = AbstractGeometry::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
