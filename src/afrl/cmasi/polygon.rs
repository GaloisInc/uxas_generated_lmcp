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
pub struct Polygon {
    pub boundary_points: Vec<Box<::afrl::cmasi::location3d::Location3DT>>,
}

impl PartialEq for Polygon {
    fn eq(&self, _other: &Polygon) -> bool {
        true
        && &self.boundary_points == &_other.boundary_points

    }
}

impl LmcpSubscription for Polygon {
    fn subscription() -> &'static str { "afrl.cmasi.Polygon" }
}

impl Struct for Polygon {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 42,
        }
    }
}

impl Lmcp for Polygon {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.boundary_points.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(Polygon, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == Polygon::struct_info() {
            let mut out: Polygon = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.boundary_points = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.boundary_points.size();

        size
    }
}

pub trait PolygonT: Debug + Send + ::afrl::cmasi::abstract_geometry::AbstractGeometryT {
    fn as_afrl_cmasi_polygon(&self) -> Option<&Polygon> { None }
    fn as_mut_afrl_cmasi_polygon(&mut self) -> Option<&mut Polygon> { None }
    fn boundary_points(&self) -> &Vec<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn boundary_points_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::location3d::Location3DT>>;

}

impl Clone for Box<PolygonT> {
    fn clone(&self) -> Box<PolygonT> {
        if let Some(x) = PolygonT::as_afrl_cmasi_polygon(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<PolygonT> {
    fn default() -> Box<PolygonT> { Box::new(Polygon::default()) }
}

impl PartialEq for Box<PolygonT> {
    fn eq(&self, other: &Box<PolygonT>) -> bool {
        if let (Some(x), Some(y)) =
            (PolygonT::as_afrl_cmasi_polygon(self.as_ref()),
             PolygonT::as_afrl_cmasi_polygon(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<PolygonT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = PolygonT::as_afrl_cmasi_polygon(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<PolygonT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == Polygon::struct_info() {
            let (x, readb) = Polygon::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = PolygonT::as_afrl_cmasi_polygon(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl ::afrl::cmasi::abstract_geometry::AbstractGeometryT for Polygon {
    fn as_afrl_cmasi_polygon(&self) -> Option<&Polygon> { Some(self) }
    fn as_mut_afrl_cmasi_polygon(&mut self) -> Option<&mut Polygon> { Some(self) }
}
impl PolygonT for Polygon {
    fn as_afrl_cmasi_polygon(&self) -> Option<&Polygon> { Some(self) }
    fn as_mut_afrl_cmasi_polygon(&mut self) -> Option<&mut Polygon> { Some(self) }
    fn boundary_points(&self) -> &Vec<Box<::afrl::cmasi::location3d::Location3DT>> { &self.boundary_points }
    fn boundary_points_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.boundary_points }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for Polygon {
        fn arbitrary<G: Gen>(_g: &mut G) -> Polygon {
            Polygon {
                boundary_points: Vec::<::afrl::cmasi::location3d::Location3D>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::location3d::Location3DT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: Polygon) -> Result<TestResult, Error> {
            use std::u16;
            if x.boundary_points.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: Polygon) -> Result<TestResult, Error> {
            use std::u16;
            if x.boundary_points.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = Polygon::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
