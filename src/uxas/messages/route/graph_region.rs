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
pub struct GraphRegion {
    pub id: i64,
    pub node_list: Vec<Box<::uxas::messages::route::graph_node::GraphNodeT>>,
    pub edge_list: Vec<Box<::uxas::messages::route::graph_edge::GraphEdgeT>>,
}

impl PartialEq for GraphRegion {
    fn eq(&self, _other: &GraphRegion) -> bool {
        true
        && &self.id == &_other.id
        && &self.node_list == &_other.node_list
        && &self.edge_list == &_other.edge_list

    }
}

impl LmcpSubscription for GraphRegion {
    fn subscription() -> &'static str { "uxas.messages.route.GraphRegion" }
}

impl Struct for GraphRegion {
    fn struct_info() -> StructInfo {
        StructInfo {
            exist: 1,
            series: 5931053054693474304u64,
            version: 4,
            struct_ty: 3,
        }
    }
}

impl Lmcp for GraphRegion {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        let mut pos = 0;
        {
            let x = Self::struct_info().ser(buf)?;
            pos += x;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.id.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.node_list.ser(r)?;
            pos += writeb;
        }
        {
            let r = get!(buf.get_mut(pos ..));
            let writeb: usize = self.edge_list.ser(r)?;
            pos += writeb;
        }

        Ok(pos)
    }

    fn deser(buf: &[u8]) -> Result<(GraphRegion, usize), Error> {
        let mut pos = 0;
        let (si, u) = StructInfo::deser(buf)?;
        pos += u;
        if si == GraphRegion::struct_info() {
            let mut out: GraphRegion = Default::default();
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (i64, usize) = Lmcp::deser(r)?;
                out.id = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::route::graph_node::GraphNodeT>>, usize) = Lmcp::deser(r)?;
                out.node_list = x;
                pos += readb;
            }
            {
                let r = get!(buf.get(pos ..));
                let (x, readb): (Vec<Box<::uxas::messages::route::graph_edge::GraphEdgeT>>, usize) = Lmcp::deser(r)?;
                out.edge_list = x;
                pos += readb;
            }

            Ok((out, pos))
        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        let mut size = 15;
        size += self.id.size();
        size += self.node_list.size();
        size += self.edge_list.size();

        size
    }
}

pub trait GraphRegionT: Debug + Send  {
    fn as_uxas_messages_route_graph_region(&self) -> Option<&GraphRegion> { None }
    fn as_mut_uxas_messages_route_graph_region(&mut self) -> Option<&mut GraphRegion> { None }
    fn id(&self) -> i64;
    fn id_mut(&mut self) -> &mut i64;
    fn node_list(&self) -> &Vec<Box<::uxas::messages::route::graph_node::GraphNodeT>>;
    fn node_list_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::graph_node::GraphNodeT>>;
    fn edge_list(&self) -> &Vec<Box<::uxas::messages::route::graph_edge::GraphEdgeT>>;
    fn edge_list_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::graph_edge::GraphEdgeT>>;

}

impl Clone for Box<GraphRegionT> {
    fn clone(&self) -> Box<GraphRegionT> {
        if let Some(x) = GraphRegionT::as_uxas_messages_route_graph_region(self.as_ref()) {
            Box::new(x.clone())

        } else {
            unreachable!()
        }
    }
}

impl Default for Box<GraphRegionT> {
    fn default() -> Box<GraphRegionT> { Box::new(GraphRegion::default()) }
}

impl PartialEq for Box<GraphRegionT> {
    fn eq(&self, other: &Box<GraphRegionT>) -> bool {
        if let (Some(x), Some(y)) =
            (GraphRegionT::as_uxas_messages_route_graph_region(self.as_ref()),
             GraphRegionT::as_uxas_messages_route_graph_region(other.as_ref())) {
                x == y

        } else {
            false
        }
    }
}

impl Lmcp for Box<GraphRegionT> {
    fn ser(&self, buf: &mut[u8]) -> Result<usize, Error> {
        if let Some(x) = GraphRegionT::as_uxas_messages_route_graph_region(self.as_ref()) {
            x.ser(buf)

        } else {
            unreachable!()
        }
    }

    fn deser(buf: &[u8]) -> Result<(Box<GraphRegionT>, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?;
        if si == GraphRegion::struct_info() {
            let (x, readb) = GraphRegion::deser(buf)?;
            Ok((Box::new(x), readb))

        } else {
            Err(error!(ErrorType::InvalidStructInfo))
        }
    }

    fn size(&self) -> usize {
        if let Some(x) = GraphRegionT::as_uxas_messages_route_graph_region(self.as_ref()) {
            x.size()

        } else {
            unreachable!()
        }
    }
}

impl GraphRegionT for GraphRegion {
    fn as_uxas_messages_route_graph_region(&self) -> Option<&GraphRegion> { Some(self) }
    fn as_mut_uxas_messages_route_graph_region(&mut self) -> Option<&mut GraphRegion> { Some(self) }
    fn id(&self) -> i64 { self.id }
    fn id_mut(&mut self) -> &mut i64 { &mut self.id }
    fn node_list(&self) -> &Vec<Box<::uxas::messages::route::graph_node::GraphNodeT>> { &self.node_list }
    fn node_list_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::graph_node::GraphNodeT>> { &mut self.node_list }
    fn edge_list(&self) -> &Vec<Box<::uxas::messages::route::graph_edge::GraphEdgeT>> { &self.edge_list }
    fn edge_list_mut(&mut self) -> &mut Vec<Box<::uxas::messages::route::graph_edge::GraphEdgeT>> { &mut self.edge_list }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for GraphRegion {
        fn arbitrary<G: Gen>(_g: &mut G) -> GraphRegion {
            GraphRegion {
                id: Arbitrary::arbitrary(_g),
                node_list: Vec::<::uxas::messages::route::graph_node::GraphNode>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::route::graph_node::GraphNodeT>).collect(),
                edge_list: Vec::<::uxas::messages::route::graph_edge::GraphEdge>::arbitrary(_g).into_iter().map(|x| Box::new(x) as Box<::uxas::messages::route::graph_edge::GraphEdgeT>).collect(),

            }
        }
    }

    quickcheck! {
        fn serializes(x: GraphRegion) -> Result<TestResult, Error> {
            use std::u16;
            if x.node_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.edge_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            Ok(TestResult::from_bool(sx == x.size()))
        }

        fn roundtrips(x: GraphRegion) -> Result<TestResult, Error> {
            use std::u16;
            if x.node_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }
            if x.edge_list.len() > (u16::MAX as usize) { return Ok(TestResult::discard()); }

            let mut buf: Vec<u8> = vec![0; x.size()];
            let sx = x.ser(&mut buf)?;
            let (y, sy) = GraphRegion::deser(&buf)?;
            Ok(TestResult::from_bool(sx == sy && x == y))
        }
    }
}
