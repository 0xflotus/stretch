#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::Function;
use js_sys::Reflect;
use stretch_js::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn js_value(js: &str) -> JsValue {
    Function::new_no_args(&format!("return {};", js)).call0(&JsValue::UNDEFINED).unwrap()
}

#[wasm_bindgen_test]
fn set_measure() {
    let mut node = Node::new(&js_value("{}"));
    node.set_measure(&js_value(
        "function (w, h) {
        return {width: 100, height: 100};
    }",
    ));

    let layout = node.compute_layout(&JsValue::UNDEFINED);
    assert_eq!(layout.width, 100.0);
    assert_eq!(layout.height, 100.0);
}

#[wasm_bindgen_test]
fn add_child() {
    let mut node = Node::new(&js_value("{}"));
    let child = Node::new(&js_value("{}"));

    node.add_child(&child);

    assert_eq!(node.childCount, 1);
}

#[wasm_bindgen_test]
fn remove_child() {
    let mut node = Node::new(&js_value("{}"));
    let child = Node::new(&js_value("{}"));

    node.add_child(&child);
    node.remove_child(&child);

    assert_eq!(node.childCount, 0);
}

#[wasm_bindgen_test]
fn remove_child_at_index() {
    let mut node = Node::new(&js_value("{}"));
    let child = Node::new(&js_value("{}"));

    node.add_child(&child);
    node.remove_child_at_index(0);

    assert_eq!(node.childCount, 0);
}

#[wasm_bindgen_test]
fn replace_child_at_index() {
    let mut node = Node::new(&js_value("{}"));
    let child1 = Node::new(&js_value("{width: 100, height: 100}"));
    let child2 = Node::new(&js_value("{width: 200, height: 200}"));

    node.add_child(&child1);
    node.replace_child_at_index(0, &child2);
    let layout = node.compute_layout(&JsValue::UNDEFINED);

    assert_eq!(layout.width, 200.0);
    assert_eq!(layout.height, 200.0);
}

#[wasm_bindgen_test]
fn set_style() {
    let mut node = Node::new(&js_value("{}"));
    node.set_style(&js_value("{width: 200, height: 200}"));
    let style = node.get_style();

    assert_eq!(Reflect::get(&style, &"width".into()).unwrap().as_f64(), Some(200.0));
    assert_eq!(Reflect::get(&style, &"height".into()).unwrap().as_f64(), Some(200.0));
}

#[wasm_bindgen_test]
fn mark_dirty() {
    let mut node = Node::new(&js_value("{}"));
    node.compute_layout(&JsValue::UNDEFINED);

    assert_eq!(node.is_dirty(), false);
    node.mark_dirty();
    assert_eq!(node.is_dirty(), true);
}
