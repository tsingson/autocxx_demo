use autocxx::prelude::*;
use ffi::{Point, Rect};

//include_cpp! {
//    // C++ headers we want to include.
//    #include "cpp.h"
//    // Safety policy. We are marking that this whole C++ inclusion is unsafe
//    // which means the functions themselves do not need to be marked
//    // as unsafe. Other policies are possible.
//    safety!(unsafe)
//    // What types and functions we want to generate
//    generate_pod!("Rect")
//    generate!("print_point")
//}



include_cpp! {
    // C++ headers we want to include.
    #include "cpp.h"
    safety!(unsafe)
    // A non-trivial C++ type
        generate_pod!("Rect")
    generate!("print_point")
    generate!("MessageBuffer")
}
fn main() {
    // Put the non-trivial C++ type on the Rust stack.
    moveit! { let mut msg = ffi::MessageBuffer::new(); }
    // Call methods on it.
    msg.as_mut().add_blurb("Hello");
    msg.as_mut().add_blurb(" world!");

    let binding = msg.get();
    let hello = binding.as_ref().unwrap().to_string_lossy();
    println!("Hello, {}!", hello);


    let r = Rect {
        top_left: Point { x: 3, y: 3 },
        bottom_right: Point { x: 12, y: 15 },
    };
    // r.width() and r.height() return an autocxx::c_int
    // which we need to unpackage. It is hoped that one day cxx will
    // natively support 'int' and friends, and that won't be necessary.
    let center = Point {
        x: r.top_left.x + r.width().0 / 2,
        y: r.top_left.y + r.height().0 / 2,
    };
    ffi::print_point(center);
}
