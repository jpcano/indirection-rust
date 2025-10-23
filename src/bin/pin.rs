use std::marker::PhantomPinned;
use std::pin::{Pin, pin};

fn main() {
    {
        let mut at = AddressTracker::default();
        at.check_address();
        // This shadowing assignment moves `at' to a new address
        let mut at = at;
        at.check_address();
    }
    {
        let at = AddressTrackerPinned::default();
        let mut at_pin = pin!(at);
        at_pin.as_mut().check_address();
        let mut at_pin = at_pin;
        at_pin.as_mut().check_address();
    }
}

#[derive(Default)]
struct AddressTracker(Option<usize>, PhantomPinned);

impl AddressTracker {
    fn check_address(&mut self) {
        //let current_address = self as *mut Self as usize;
        let current_address = self as *mut Self as usize;
        match self.0 {
            Some(last_address) => {
                println!("last_address: {}", last_address);
                println!(" new_address: {}", current_address);
            }
            None => self.0 = Some(current_address),
        }
    }
}

#[derive(Default)]
struct AddressTrackerPinned(Option<usize>, PhantomPinned);

impl AddressTrackerPinned {
    fn check_address(mut self: Pin<&mut Self>) {
        let current_address = &*self as *const Self as usize;
        match self.0 {
            Some(last_address) => {
                println!("last_address: {}", last_address);
                println!(" new_address: {}", current_address);
            }
            None => {
                // This works when the struct implements Unpin, which means that it is safe to move it.
                // However, since AddressTrackerPinned does not implement Unpin (due to PhantomPinned),
                // we have to use unsafe code to modify its fields.
                //self.0 = Some(current_address);

                // SAFETY: we do not move out of self
                let self_data_mut = unsafe { self.get_unchecked_mut() };
                self_data_mut.0 = Some(current_address);
            }
        }
    }
}
