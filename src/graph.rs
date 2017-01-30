use std::io::{self, Write};

use node;

#[derive(Debug)]
pub struct Graph {
    name: String,
    properties: GraphProperties,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}
impl Graph {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Graph {
            name: name.into(),
            properties: GraphProperties::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
    pub fn properties_mut(&mut self) -> &mut GraphProperties {
        &mut self.properties
    }
    pub fn write_as_dot<W: Write>(&self, mut writer: W) -> io::Result<()> {
        let graph_type = if self.properties.is_directed {
            "digraph"
        } else {
            "graph"
        };
        writeln!(writer, "{} {:?} {{", graph_type, self.name)?;
        for node in self.nodes.iter() {
            node.write(&mut writer)?;
        }
        for edge in self.edges.iter() {
            edge.write(&mut writer, self.properties.is_directed)?;
        }
        writeln!(writer, "}}")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct GraphProperties {
    pub is_directed: bool,
}
impl Default for GraphProperties {
    fn default() -> Self {
        GraphProperties { is_directed: false }
    }
}

#[derive(Debug)]
pub struct Node {
    id: String,
    label: Option<String>,
    shape: node::NodeShape,
}
impl Node {
    pub fn new<T: Into<String>>(id: T) -> Self {
        Node {
            id: id.into(),
            label: None,
            shape: node::NodeShape::default(),
        }
    }
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
    pub fn shape(mut self, shape: node::NodeShape) -> Self {
        self.shape = shape;
        self
    }
    pub fn set_lable<T: Into<String>>(&mut self, label: T) -> &mut Self {
        self.label = Some(label.into());
        self
    }
    fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        write!(writer, "  {:?}[", self.id)?;
        let mut delim = "";
        if let Some(ref label) = self.label {
            write!(writer, "{}label={:?}", delim, label)?;
            delim = ", ";
        }
        if node::NodeShape::default() != self.shape {
            write!(writer, "{}shape={}", delim, self.shape)?;
            // delim = ", ";
        }
        writeln!(writer, "];")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Edge {
    from_node_id: String,
    to_node_id: String,
}
impl Edge {
    pub fn new<T: Into<String>, U: Into<String>>(from: T, to: U) -> Self {
        Edge {
            from_node_id: from.into(),
            to_node_id: to.into(),
        }
    }
    fn write<W: Write>(&self, writer: &mut W, is_directed: bool) -> io::Result<()> {
        let edge_type = if is_directed { "->" } else { "--" };
        writeln!(writer,
                 "  {:?} {} {:?};",
                 self.from_node_id,
                 edge_type,
                 self.to_node_id)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str;
    use super::*;

    #[test]
    fn empty_graph() {
        let graph = Graph::new("foo");
        let mut buf = Vec::new();
        graph.write_as_dot(&mut buf).unwrap();

        let expected = r#"graph "foo" {
}
"#;
        assert_eq!(str::from_utf8(&buf[..]).unwrap(), expected);
    }
}
