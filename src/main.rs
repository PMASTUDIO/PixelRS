use pixel_rs::h;
use pixel_rs::t;

fn main() {
//   let vdom = VNode::append_element("div".to_string(), vec![("class".to_string(), "container".to_string())], vec![
//     VNode::append_element("h1".to_string(), vec![], vec![
//         VNode::append_text("Hello, Rust!".to_string())
//     ]),
//     VNode::append_element("p".to_string(), vec![], vec![
//         VNode::append_text("This is a simple virtual DOM example.".to_string())
//     ]),
// ],);

  let vdom = h("div", vec![
    h("h1", vec![t("Hello, Pixel RS VDom!")]),
    h("p", vec![t("This is a simple virtual DOM example.")]),
  ]);

  let html_output = vdom.render();
  println!("Rendered HTML:\n{}", html_output);
}
