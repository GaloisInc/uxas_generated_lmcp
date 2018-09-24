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
pub struct Location3D {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub altitude_type: ::afrl::cmasi::altitude_type::AltitudeType,
}

impl PartialEq for Location3D {
    fn eq(&self, _other: &Location3D) -> bool {
        true
        && &self.latitude == &_other.latitude
        && &self.longitude == &_other.longitude
        && &self.altitude == &_other.altitude
        && &self.altitude_type == &_other.altitude_type

    }
}

impl LmcpSubscription for Location3D {
    fn subscription() -> &'static str { "afrl.cmasi.Location3D" }
}

impl Struct for Location3D {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 4849604199710720000u64,
            version: 3,
            struct_ty: 3,
        }
    }
}

impl Lmcp for Location3D {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.latitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.longitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.altitude.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.altitude_type.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(Location3D, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == Location3D::struct_info() {
            let mut out: Location3D = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f64, usize) = Lmcp::deser(r)?;
                out.latitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f64, usize) = Lmcp::deser(r)?;
                out.longitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (f32, usize) = Lmcp::deser(r)?;
                out.altitude = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (::afrl::cmasi::altitude_type::AltitudeType, usize) = Lmcp::deser(r)?;
                out.altitude_type = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.latitude.size();
        size += self.longitude.size();
        size += self.altitude.size();
        size += self.altitude_type.size();

        size
    }
}

pub trait Location3DT: Debug + Send  {
    fn as_afrl_cmasi_location3d(&self) -> Option<&Location3D> { None }
    fn as_mut_afrl_cmasi_location3d(&mut self) -> Option<&mut Location3D> { None }
    fn as_afrl_cmasi_path_waypoint(&self) -> Option<&::afrl::cmasi::path_waypoint::PathWaypoint> { None }
    fn as_mut_afrl_cmasi_path_waypoint(&mut self) -> Option<&mut ::afrl::cmasi::path_waypoint::PathWaypoint> { None }
    fn as_afrl_cmasi_waypoint(&self) -> Option<&::afrl::cmasi::waypoint::Waypoint> { None }
    fn as_mut_afrl_cmasi_waypoint(&mut self) -> Option<&mut ::afrl::cmasi::waypoint::Waypoint> { None }
    fn latitude(&self) -> f64;
    fn latitude_mut(&mut self) -> &mut f64;
    fn longitude(&self) -> f64;
    fn longitude_mut(&mut self) -> &mut f64;
    fn altitude(&self) -> f32;
    fn altitude_mut(&mut self) -> &mut f32;
    fn altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType;
    fn altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType;

}

impl Clone for Box<Location3DT> {
    fn clone(&self) -> Box<Location3DT> {
        if let Some(x) = Location3DT::as_afrl_cmasi_location3d(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = Location3DT::as_afrl_cmasi_path_waypoint(self.as_ref()) {
            Box::new(x.clone())
        } else if let Some(x) = Location3DT::as_afrl_cmasi_waypoint(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<Location3DT> {
    fn default() -> Box<Location3DT> { Box::new(Location3D::default()) }
}

impl PartialEq for Box<Location3DT> {
    fn eq(&self, other: &Box<Location3DT>) -> bool {
        if let (Some(x), Some(y)) =
            (Location3DT::as_afrl_cmasi_location3d(self.as_ref()),
             Location3DT::as_afrl_cmasi_location3d(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (Location3DT::as_afrl_cmasi_path_waypoint(self.as_ref()),
             Location3DT::as_afrl_cmasi_path_waypoint(other.as_ref())) {
                x == y
        } else if let (Some(x), Some(y)) =
            (Location3DT::as_afrl_cmasi_waypoint(self.as_ref()),
             Location3DT::as_afrl_cmasi_waypoint(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<Location3DT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = Location3DT::as_afrl_cmasi_location3d(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = Location3DT::as_afrl_cmasi_path_waypoint(self.as_ref()) {
            x.ser(buf)
        } else if let Some(x) = Location3DT::as_afrl_cmasi_waypoint(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<Location3DT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == Location3D::struct_info() {
            let (x, readb) = Location3D::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::path_waypoint::PathWaypoint::struct_info() {
            let (x, readb) = ::afrl::cmasi::path_waypoint::PathWaypoint::deser(buf)?;
            Ok((Box::new(x), readb))
        } else if si == ::afrl::cmasi::waypoint::Waypoint::struct_info() {
            let (x, readb) = ::afrl::cmasi::waypoint::Waypoint::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = Location3DT::as_afrl_cmasi_location3d(self.as_ref()) {
            x.size()
        } else if let Some(x) = Location3DT::as_afrl_cmasi_path_waypoint(self.as_ref()) {
            x.size()
        } else if let Some(x) = Location3DT::as_afrl_cmasi_waypoint(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl Location3DT for Location3D {
    fn as_afrl_cmasi_location3d(&self) -> Option<&Location3D> { Some(self) }
    fn as_mut_afrl_cmasi_location3d(&mut self) -> Option<&mut Location3D> { Some(self) }
    fn latitude(&self) -> f64 { self.latitude }
    fn latitude_mut(&mut self) -> &mut f64 { &mut self.latitude }
    fn longitude(&self) -> f64 { self.longitude }
    fn longitude_mut(&mut self) -> &mut f64 { &mut self.longitude }
    fn altitude(&self) -> f32 { self.altitude }
    fn altitude_mut(&mut self) -> &mut f32 { &mut self.altitude }
    fn altitude_type(&self) -> ::afrl::cmasi::altitude_type::AltitudeType { self.altitude_type }
    fn altitude_type_mut(&mut self) -> &mut ::afrl::cmasi::altitude_type::AltitudeType { &mut self.altitude_type }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for Location3D {
        fn arbitrary<G: Gen>(_g: &mut G) -> Location3D {
            Location3D {
                latitude: Arbitrary::arbitrary(_g),
                longitude: Arbitrary::arbitrary(_g),
                altitude: Arbitrary::arbitrary(_g),
                altitude_type: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: Location3D) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: Location3D) -> Result<TestResult, Error> {

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = Location3D::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
