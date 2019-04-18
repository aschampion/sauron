#![deny(warnings)]
#![feature(proc_macro_hygiene)]

extern crate wasm_bindgen_test;
extern crate web_sys;
use std::rc::Rc;
use wasm_bindgen_test::*;

use sauron::dom::DomUpdater;
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::test_fixtures::simple_program;
use sauron::Node;
use std::cell::RefCell;
use web_sys::*;

wasm_bindgen_test_configure!(run_in_browser);

// Make sure that we successfully attach an event listener and see it work.
#[wasm_bindgen_test]
fn on_input() {
    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "input-element-1";

    let input: Node<()> = input(
        [
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            oninput(move |event: sauron_vdom::Event| match event {
                sauron_vdom::Event::InputEvent(input) => {
                    *text_clone.borrow_mut() = input.value;
                }
                _ => unimplemented!(),
            }),
            value("End Text"),
        ],
        [],
    );

    let input_event = InputEvent::new("input").unwrap();

    let body = sauron::body();
    let _dom_updater = DomUpdater::new_append_to_mount(&simple_program(), input, &body);

    let input_element = sauron::document().get_element_by_id(&elem_id).unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // After dispatching the oninput event our `text` should have a value of the input elements value.
    web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event)
        .unwrap();

    assert_eq!(&*text.borrow(), "End Text");
}

#[wasm_bindgen_test]
fn added_event() {
    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "input-add-event-test";

    let old: Node<()> = input(
        [
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
        ],
        [],
    );

    let new = input(
        [
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
            oninput(move |event: sauron_vdom::Event| match event {
                sauron_vdom::Event::InputEvent(input) => {
                    *text_clone.borrow_mut() = input.value;
                }
                _ => unimplemented!(),
            }),
        ],
        [],
    );

    let input_event = InputEvent::new("input").unwrap();

    let body = sauron::body();
    let simple_program = &simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(simple_program, old, &body);
    // update to new dom with no event attached
    dom_updater.update(simple_program, new);

    let input_element = sauron::document().get_element_by_id(&elem_id).unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // Dispatching the event, after the dom is updated
    web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event)
        .unwrap();

    //Should change the text
    assert_eq!(&*text.borrow(), "End Text");
}

#[wasm_bindgen_test]
fn remove_event() {
    let text = Rc::new(RefCell::new("Start Text".to_string()));
    let text_clone = Rc::clone(&text);

    let elem_id = "input-remove-event-test";

    let old: Node<()> = input(
        [
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
            oninput(move |event: sauron_vdom::Event| match event {
                sauron_vdom::Event::InputEvent(input) => {
                    *text_clone.borrow_mut() = input.value;
                }
                _ => unimplemented!(),
            }),
        ],
        [],
    );

    let new = input(
        [
            // On input we'll set our Rc<RefCell<String>> value to the input elements value
            id(elem_id),
            value("End Text"),
        ],
        [],
    );

    let input_event = InputEvent::new("input").unwrap();

    let body = sauron::body();
    let simple_program = &simple_program();
    let mut dom_updater = DomUpdater::new_append_to_mount(simple_program, old, &body);
    // update to new dom with no event attached
    dom_updater.update(simple_program, new);

    let input_element = sauron::document().get_element_by_id(&elem_id).unwrap();

    assert_eq!(&*text.borrow(), "Start Text");

    // Dispatching the event, after the dom is updated
    web_sys::EventTarget::from(input_element)
        .dispatch_event(&input_event)
        .unwrap();

    //Should never change the text, since it is removed with the dom_updater.update is called with
    //the `new` vdom which has no attached event
    assert_eq!(&*text.borrow(), "Start Text");
}
