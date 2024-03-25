use vdom::vstructures::VNode;

pub mod vdom;

pub fn h(node_type:&str,children:Vec<VNode>) -> VNode {
  VNode::append_element(String::from(node_type), vec![], children)
}

pub fn t(text:&str)-> VNode {
  VNode::append_text(String::from(text))
}

