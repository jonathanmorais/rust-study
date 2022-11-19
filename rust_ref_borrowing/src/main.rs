// Most of the time, we'd like to access data without taking ownership over it.
// To accomplish this, Rust uses a borrowing mechanism.
// Instead of passing objects by value (T), objects can be passed by reference (&T).

fn destroy_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {

    // create boxed_i32 and stacked_i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;


    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }
}
