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
pub struct Circle {
    pub center_point: Box<::afrl::cmasi::location3d::Location3DT>,
    pub radius: f32,
}

impl PartialEq for Circle {
    fn eq(&self, _other: &Circle) -> bool {
        true
        && &self.center_point == &_other.center_point
        && &self.radius == &_other.radius

    }
}

impl LmcpSubscription for Circle {
    fn subscription() -> &'static str { "afrl.cmasi.Circle" }
}

impl Struct for Circle {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 22,
        }
    }
}

impl Lmcp for Circle {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.center_point.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.radius.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(Circle, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == Circle::struct_info() {
            let mut out: Circle = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.center_point = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.radius = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.center_point.size();
        size += self.radius.size();

        size
    }
}

pub trait CircleT: Debug + Send + ::afrl::cmasi::abstract_geometry::AbstractGeometryT {
    fn as_afrl_cmasi_circle(&self) -> Option<&Circle> { None }
    fn as_mut_afrl_cmasi_circle(&mut self) -> Option<&mut Circle> { None }
    fn center_point(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn center_point_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn radius(&self) -> f32;
    fn radius_mut(&mut self) -> &mut f32;

}

impl Clone for Box<CircleT> {
    fn clone(&self) -> Box<CircleT> {
        if let Some(x) = CircleT::as_afrl_cmasi_circle(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<CircleT> {
    fn default() -> Box<CircleT> { Box::new(Circle::default()) }
}

impl PartialEq for Box<CircleT> {
    fn eq(&self, other: &Box<CircleT>) -> bool {
        if let (Some(x), Some(y)) =
            (CircleT::as_afrl_cmasi_circle(self.as_ref()),
             CircleT::as_afrl_cmasi_circle(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<CircleT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = CircleT::as_afrl_cmasi_circle(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<CircleT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == Circle::struct_info() {
            let (x, readb) = Circle::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = CircleT::as_afrl_cmasi_circle(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::abstract_geometry::AbstractGeometryT for Circle {
    fn as_afrl_cmasi_circle(&self) -> Option<&Circle> { Some(self) }
    fn as_mut_afrl_cmasi_circle(&mut self) -> Option<&mut Circle> { Some(self) }
}
impl CircleT for Circle {
    fn as_afrl_cmasi_circle(&self) -> Option<&Circle> { Some(self) }
    fn as_mut_afrl_cmasi_circle(&mut self) -> Option<&mut Circle> { Some(self) }
    fn center_point(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.center_point }
    fn center_point_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.center_point }
    fn radius(&self) -> f32 { self.radius }
    fn radius_mut(&mut self) -> &mut f32 { &mut self.radius }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for Circle {
        fn arbitrary<G: Gen>(_g: &mut G) -> Circle {
            Circle {
                center_point: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                radius: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: Circle) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: Circle) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = Circle::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
