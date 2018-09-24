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
pub struct GraphNode {
    pub node_id: i64,
    pub coordinates: Box<::afrl::cmasi::location3d::Location3DT>,
    pub associated_edges: Vec<i64>,
}

impl PartialEq for GraphNode {
    fn eq(&self, _other: &GraphNode) -> bool {
        true
        && &self.node_id == &_other.node_id
        && &self.coordinates == &_other.coordinates
        && &self.associated_edges == &_other.associated_edges

    }
}

impl LmcpSubscription for GraphNode {
    fn subscription() -> &'static str { "uxas.messages.route.GraphNode" }
}

impl Struct for GraphNode {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 1,
        }
    }
}

impl Lmcp for GraphNode {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.node_id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.coordinates.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.associated_edges.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(GraphNode, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == GraphNode::struct_info() {
            let mut out: GraphNode = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.node_id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Box<::afrl::cmasi::location3d::Location3DT>, usize) = Lmcp::deser(r)?;
                out.coordinates = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<i64>, usize) = Lmcp::deser(r)?;
                out.associated_edges = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.node_id.size();
        size += self.coordinates.size();
        size += self.associated_edges.size();

        size
    }
}

pub trait GraphNodeT: Debug + Send  {
    fn as_uxas_messages_route_graph_node(&self) -> Option<&GraphNode> { None }
    fn as_mut_uxas_messages_route_graph_node(&mut self) -> Option<&mut GraphNode> { None }
    fn node_id(&self) -> i64;
    fn node_id_mut(&mut self) -> &mut i64;
    fn coordinates(&self) -> &Box<::afrl::cmasi::location3d::Location3DT>;
    fn coordinates_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT>;
    fn associated_edges(&self) -> &Vec<i64>;
    fn associated_edges_mut(&mut self) -> &mut Vec<i64>;

}

impl Clone for Box<GraphNodeT> {
    fn clone(&self) -> Box<GraphNodeT> {
        if let Some(x) = GraphNodeT::as_uxas_messages_route_graph_node(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<GraphNodeT> {
    fn default() -> Box<GraphNodeT> { Box::new(GraphNode::default()) }
}

impl PartialEq for Box<GraphNodeT> {
    fn eq(&self, other: &Box<GraphNodeT>) -> bool {
        if let (Some(x), Some(y)) =
            (GraphNodeT::as_uxas_messages_route_graph_node(self.as_ref()),
             GraphNodeT::as_uxas_messages_route_graph_node(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<GraphNodeT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = GraphNodeT::as_uxas_messages_route_graph_node(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<GraphNodeT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == GraphNode::struct_info() {
            let (x, readb) = GraphNode::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = GraphNodeT::as_uxas_messages_route_graph_node(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl GraphNodeT for GraphNode {
    fn as_uxas_messages_route_graph_node(&self) -> Option<&GraphNode> { Some(self) }
    fn as_mut_uxas_messages_route_graph_node(&mut self) -> Option<&mut GraphNode> { Some(self) }
    fn node_id(&self) -> i64 { self.node_id }
    fn node_id_mut(&mut self) -> &mut i64 { &mut self.node_id }
    fn coordinates(&self) -> &Box<::afrl::cmasi::location3d::Location3DT> { &self.coordinates }
    fn coordinates_mut(&mut self) -> &mut Box<::afrl::cmasi::location3d::Location3DT> { &mut self.coordinates }
    fn associated_edges(&self) -> &Vec<i64> { &self.associated_edges }
    fn associated_edges_mut(&mut self) -> &mut Vec<i64> { &mut self.associated_edges }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for GraphNode {
        fn arbitrary<G: Gen>(_g: &mut G) -> GraphNode {
            GraphNode {
                node_id: Arbitrary::arbitrary(_g),
                coordinates: Box::new(::afrl::cmasi::location3d::Location3D::arbitrary(_g)),
                associated_edges: Arbitrary::arbitrary(_g),

            }
        }
    }

    quickcheck! {
        fn serializes(x: GraphNode) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_edges.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: GraphNode) -> Result<TestResult, Error> {
            use std::u16;
            if x.associated_edges.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = GraphNode::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
