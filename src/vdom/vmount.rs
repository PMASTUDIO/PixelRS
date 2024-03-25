use super::vstructures;

impl vstructures::VNode {
  pub fn render(&self) -> String {
      match &self.node_type {
        vstructures::NodeType::Text(text) => text.clone(),
          
        vstructures::NodeType::Element(data) => {
              let attrs = data.attributes.iter()
                  .map(|(key, value)| format!("{}=\"{}\"", key, value))
                  .collect::<Vec<_>>()
                  .join(" ");

              let children = data.children.iter()
                  .map(|child| child.render())
                  .collect::<Vec<_>>()
                  .join("");
              format!("<{} {}>{}</{}>", data.tag_name, attrs, children, data.tag_name)
          },
      }
  }
}
