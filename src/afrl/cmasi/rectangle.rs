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
pub struct Rectangle {
    pub center_point: Box<::afrl::cmasi::location3d::Location3DT>,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
}

impl PartialEq for Rectangle {
    fn eq(&self, _other: &Rectangle) -> bool {
        true
        && &self.center_point == &_other.center_point
        && &self.width == &_other.width
        && &self.height == &_other.height
        && &self.rotation == &_other.rotation

    }
}

impl LmcpSubscription for Rectangle {
    fn subscription() -> &'static str { "afrl.cmasi.Rectangle" }
}

impl Struct for Rectangle {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 43,
        }
    }
}

impl Lmcp for Rectangle {
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
            let writeb: usize = self.width.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.height.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.rotation.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(Rectangle, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == Rectangle::struct_info() {
            let mut out: Rectangle = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.center_point = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.width = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.height = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.rotation = x;
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
        size += self.width.size();
        size += self.height.size();
        size += self.rotation.size();

        size
    }
}

pub trait RectangleT: Debug + Send + ::afrl::cmasi::abstract_geometry::AbstractGeometryT {
    fn as_afrl_cmasi_rectangle(&self) -> Option<&Rectangle> { None }
    fn as_mut_afrl_cmasi_rectangle(&mut self) -> Option<&mut Rectangle> { None }
    fn center_point(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn center_point_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn width(&self) -> f32;
    fn width_mut(&mut self) -> &mut f32;
    fn height(&self) -> f32;
    fn height_mut(&mut self) -> &mut f32;
    fn rotation(&self) -> f32;
    fn rotation_mut(&mut self) -> &mut f32;

}

impl Clone for Box<RectangleT> {
    fn clone(&self) -> Box<RectangleT> {
        if let Some(x) = RectangleT::as_afrl_cmasi_rectangle(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<RectangleT> {
    fn default() -> Box<RectangleT> { Box::new(Rectangle::default()) }
}

impl PartialEq for Box<RectangleT> {
    fn eq(&self, other: &Box<RectangleT>) -> bool {
        if let (Some(x), Some(y)) =
            (RectangleT::as_afrl_cmasi_rectangle(self.as_ref()),
             RectangleT::as_afrl_cmasi_rectangle(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<RectangleT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = RectangleT::as_afrl_cmasi_rectangle(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<RectangleT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == Rectangle::struct_info() {
            let (x, readb) = Rectangle::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = RectangleT::as_afrl_cmasi_rectangle(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::abstract_geometry::AbstractGeometryT for Rectangle {
    fn as_afrl_cmasi_rectangle(&self) -> Option<&Rectangle> { Some(self) }
    fn as_mut_afrl_cmasi_rectangle(&mut self) -> Option<&mut Rectangle> { Some(self) }
}
impl RectangleT for Rectangle {
    fn as_afrl_cmasi_rectangle(&self) -> Option<&Rectangle> { Some(self) }
    fn as_mut_afrl_cmasi_rectangle(&mut self) -> Option<&mut Rectangle> { Some(self) }
    fn center_point(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.center_point }
    fn center_point_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.center_point }
    fn width(&self) -> f32 { self.width }
    fn width_mut(&mut self) -> &mut f32 { &mut self.width }
    fn height(&self) -> f32 { self.height }
    fn height_mut(&mut self) -> &mut f32 { &mut self.height }
    fn rotation(&self) -> f32 { self.rotation }
    fn rotation_mut(&mut self) -> &mut f32 { &mut self.rotation }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for Rectangle {
        fn arbitrary<G: Gen>(_g: &mut G) -> Rectangle {
            Rectangle {
                center_point: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                width: Arbitrary::arbitrary(_g),
                height: Arbitrary::arbitrary(_g),
                rotation: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: Rectangle) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: Rectangle) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = Rectangle::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
