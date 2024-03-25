
pub enum NodeType {
  Text(String),
  Element(ElementData)
}

pub struct ElementData {
  pub tag_name: String,
  pub attributes: Vec<(String, String)>,
  pub children: Vec<VNode>
}

pub struct VNode {
  pub node_type: NodeType
}

impl VNode {
  pub fn append_text(text: String) -> Self {
    VNode {
      node_type: NodeType::Text(text)
    }
  }

  pub fn append_element(tag_name: String, attributes: Vec<(String, String)>, children: Vec<VNode>) -> Self {
    VNode {
      node_type: NodeType::Element(ElementData {
        tag_name,
        attributes,
        children,
      }),
    }
  }
}
