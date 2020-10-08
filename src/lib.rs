/// # static-instance
///
/// The `static-instance` crate provides a macro used to create safe
/// static instances of objects. This was created in order to replicate
/// the behavior of a static class member used frequently in C++. This
/// allows exporting of rust to C via FFI a whole lot easier when the
/// library needs to keep track of states.
///
/// #### NOTE
/// It is still up to you to use the data inside the instance safely. The
/// data inside the instance is not thread safe unless you make it thread
/// safe.
///
///
/// ```rust
/// extern crate static_instance;
///
/// #[macro_use]
/// use static_instance::{Instance, New};
/// use std::sync::{Arc, Mutex};
///
/// #[derive(Clone)]
/// struct SimpleCounter {
///     counter: Arc<Mutex<Box<i32>>>
/// }
///
/// impl SimpleCounter {
///
///     fn add(&mut self, value: i32) {
///         let mut data = self.count.lock().unwrap();
///         *data += 1;
///     }
///
///     fn get_value(&self) -> i32 {
///         let data: Box<i32> = self.count.lock().unwrap();
///         return (*data).clone();
///     }
///
///     fn print(&self) {
///         let data: Box<i32> = self.count.lock().unwrap();
///         println!("{}", *data);
///     }
/// }
///
/// impl New for SimpleCounter {
///     fn new() -> SimpleCounter {
///         SimpleCounter {
///             counter: Arc::new(Mutex::new(Box::new(0)))
///     }
/// }
/// 
/// CreateInstance!(SimpleCounter);
///
/// fn main() {
///     SimpleCounter::instance().add(30);
///     SimpleCounter::instance().print();
/// }
/// ```

#[allow(unused_imports)]
use std::sync::Once;

pub trait New {
    fn new() -> Self;
}

pub trait Instance: New + Clone {
    fn instance() -> Self;
}

#[allow(unused_macros)]
macro_rules! CreateInstance {
    ($name:ident) => {
        impl Instance for $name {
            fn instance() -> $name {
                static mut INSTANCE: *mut $name = 0 as *mut $name;
                static ONCE: Once = Once::new();

                unsafe {
                    ONCE.call_once(|| {
                        let instance = $name::new();
                        INSTANCE = std::mem::transmute(Box::new(instance));
                    });

                    (*INSTANCE).clone()
                }
            }
        }
    };
}
