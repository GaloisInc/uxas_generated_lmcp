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
pub struct GraphEdge {
    pub edge_id: i64,
    pub start_node: i64,
    pub end_node: i64,
    pub waypoints: Vec<Box<::afrl::cmasi::location3d::Location3DT>>,
}

impl PartialEq for GraphEdge {
    fn eq(&self, _other: &GraphEdge) -> bool {
        true
        && &self.edge_id == &_other.edge_id
        && &self.start_node == &_other.start_node
        && &self.end_node == &_other.end_node
        && &self.waypoints == &_other.waypoints

    }
}

impl LmcpSubscription for GraphEdge {
    fn subscription() -> &'static str { "uxas.messages.route.GraphEdge" }
}

impl Struct for GraphEdge {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 2,
        }
    }
}

impl Lmcp for GraphEdge {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.edge_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.start_node.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.end_node.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.waypoints.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(GraphEdge, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == GraphEdge::struct_info() {
            let mut out: GraphEdge = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.edge_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.start_node = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.end_node = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::afrl::cmasi::location3d::Location3DT>>, usize) = Lmcp::deser(r)?;
                out.waypoints = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.edge_id.size();
        size += self.start_node.size();
        size += self.end_node.size();
        size += self.waypoints.size();

        size
    }
}

pub trait GraphEdgeT: Debug + Send  {
    fn as_uxas_messages_route_graph_edge(&self) -> Option<&GraphEdge> { None }
    fn as_mut_uxas_messages_route_graph_edge(&mut self) -> Option<&mut GraphEdge> { None }
    fn edge_id(&self) -> i64;
    fn edge_id_mut(&mut self) -> &mut i64;
    fn start_node(&self) -> i64;
    fn start_node_mut(&mut self) -> &mut i64;
    fn end_node(&self) -> i64;
    fn end_node_mut(&mut self) -> &mut i64;
    fn waypoints(&self) -> &Vec<Box<::afrl::cmasi::location3d::Location3DT>>;
    fn waypoints_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::location3d::Location3DT>>;

}

impl Clone for Box<GraphEdgeT> {
    fn clone(&self) -> Box<GraphEdgeT> {
        if let Some(x) = GraphEdgeT::as_uxas_messages_route_graph_edge(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<GraphEdgeT> {
    fn default() -> Box<GraphEdgeT> { Box::new(GraphEdge::default()) }
}

impl PartialEq for Box<GraphEdgeT> {
    fn eq(&self, other: &Box<GraphEdgeT>) -> bool {
        if let (Some(x), Some(y)) =
            (GraphEdgeT::as_uxas_messages_route_graph_edge(self.as_ref()),
             GraphEdgeT::as_uxas_messages_route_graph_edge(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<GraphEdgeT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = GraphEdgeT::as_uxas_messages_route_graph_edge(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<GraphEdgeT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == GraphEdge::struct_info() {
            let (x, readb) = GraphEdge::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = GraphEdgeT::as_uxas_messages_route_graph_edge(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl GraphEdgeT for GraphEdge {
    fn as_uxas_messages_route_graph_edge(&self) -> Option<&GraphEdge> { Some(self) }
    fn as_mut_uxas_messages_route_graph_edge(&mut self) -> Option<&mut GraphEdge> { Some(self) }
    fn edge_id(&self) -> i64 { self.edge_id }
    fn edge_id_mut(&mut self) -> &mut i64 { &mut self.edge_id }
    fn start_node(&self) -> i64 { self.start_node }
    fn start_node_mut(&mut self) -> &mut i64 { &mut self.start_node }
    fn end_node(&self) -> i64 { self.end_node }
    fn end_node_mut(&mut self) -> &mut i64 { &mut self.end_node }
    fn waypoints(&self) -> &Vec<Box<::afrl::cmasi::location3d::Location3DT>> { &self.waypoints }
    fn waypoints_mut(&mut self) -> &mut Vec<Box<::afrl::cmasi::location3d::Location3DT>> { &mut self.waypoints }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for GraphEdge {
        fn arbitrary<G: Gen>(_g: &mut G) -> GraphEdge {
            GraphEdge {
                edge_id: Arbitrary::arbitrary(_g),
                start_node: Arbitrary::arbitrary(_g),
                end_node: Arbitrary::arbitrary(_g),
                waypoints: Vec::<::afrl::cmasi::location3d::Location3D>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::afrl::cmasi::location3d::Location3DT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: GraphEdge) -> Result<TestResult, Error> {
            use std::u16;
            if x.waypoints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: GraphEdge) -> Result<TestResult, Error> {
            use std::u16;
            if x.waypoints.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = GraphEdge::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
