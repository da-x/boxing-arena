#![deny(missing_docs)]
//! The `boxing-arena` crate provides a very simply reuse of `Box` allocation by
//! keeping a vector of reusable `Box` allocations that can be used when wanting to
//! wrap a value in `Box`.
//!
//! It would be sometimes easier to introduce `boxing-arena` in code bases where
//! `Box` is already used extensively than to use some other arena crate that
//! affects type and life-time semantics more drastically.
//!
//! Basic usage demonstration:
//!
//! ```rust
//! use boxing_arena::BoxingArena;
//!
//! // Prepare a long-lived arena:
//! let mut ba = BoxingArena::new();
//!
//! // ... per allocation ... :
//! let big_value = [0u8; 0x1000];
//!
//! // Instead of using `Box::new` directly, we do:
//! let boxed_big_value = ba.rebox(big_value);
//!
//! // NOTE: Type of boxed_big_value is Box<[0u8; 0x1000]>
//!
//! // Instead of letting Rust drop and deallocate the Box, we do:
//! ba.unbox(boxed_big_value);
//! ```

/// The BoxingArena struct.
pub struct BoxingArena<T> {
    items: Vec<*mut T>,
}

impl<T> BoxingArena<T> {
    /// Create a new BoxingArena. All memory used by empty boxes will be de-allocated when
    /// the BoxingArena is dropped. No allocation is made by this function.
    pub fn new() -> Self {
        Self {
            items: vec![],
        }
    }

    /// Create a new BoxingArena with the given capacity of free boxes.
    pub fn with_capacity(size: usize) -> Self {
        let mut ba = BoxingArena::new();
        ba.resize_capacity(size);
        ba
    }

    /// This function unboxes the value but keeps the allocation for later reuse by the `rebox`
    /// function.
    pub fn unbox(&mut self, v: Box<T>) -> T {
        unsafe {
            let raw = Box::into_raw(v);
            let v = std::ptr::read(raw);
            self.items.push(raw);
            v
        }
    }

    /// When boxing a value, the arena either allocates a new Box or uses an existing empty
    /// allocation from a previous 'unbox` operation. In the latter case, allocation would be very
    /// fast, and the overhead would be mostly the move into the box.
    pub fn rebox(&mut self, v: T) -> Box<T> {
        match self.items.pop() {
            None => Box::new(v),
            Some(raw_ptr) => {
                unsafe {
                    std::ptr::write(raw_ptr, v);
                    Box::from_raw(raw_ptr)
                }
            }
        }
    }

    /// Like `rebox` but only if there are empty boxes. Return `None` if `*v` is `None`.
    /// The stack overhead of this function is guaranteed in the order of pointer-sized.
    pub fn try_rebox(&mut self, v: &mut Option<T>) -> Option<Box<T>> {
        // Test the pre-conditions
        if v.is_none() {
            return None;
        }
        if self.items.len() == 0 {
            return None;
        }

        let raw_ptr = self.items.pop().unwrap();
        let v_ref = v.as_mut().unwrap();

        let boxed = unsafe {
            std::ptr::copy(v_ref, raw_ptr, 1);
            std::ptr::write(v, None);
            Box::from_raw(raw_ptr)
        };

        Some(boxed)
    }

    /// Return the number of free boxes in the BoxingArena.
    pub fn capacity(&self) -> usize {
        self.items.len()
    }

    /// Resize boxes pool to a given capacity.
    pub fn resize_capacity(&mut self, size: usize) {
        let mut n = self.items.len();

        while size < n {
            let p = self.items.pop().unwrap();
            unsafe {
                std::alloc::dealloc(p as *mut u8, std::alloc::Layout::new::<T>());
            }
            n -= 1;
        }

        while size > n {
            let p = unsafe {
                std::alloc::alloc(std::alloc::Layout::new::<T>())
            };
            self.items.push(p as *mut T);
            n += 1;
        }
    }

    /// Trims capacity to the given size if it is larger.
    pub fn trim(&mut self, size: usize) {
        if self.items.len() > size {
            self.resize_capacity(size)
        }
    }
}

impl<T> Drop for BoxingArena<T> {
    fn drop(&mut self) {
        // Deallocate all the free boxes that we kept.
        unsafe {
            for p in &self.items {
                std::alloc::dealloc(*p as *mut u8, std::alloc::Layout::new::<T>());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut ba = BoxingArena::new();
        assert_eq!(ba.capacity(), 0);
        let mut addresses = vec![];
        let a = ba.rebox(0x1234u32);
        assert_eq!(ba.capacity(), 0);
        let b = ba.rebox(0x5678u32);
        assert_eq!(ba.capacity(), 0);

        // Remember the addresses
        addresses.push((&*a as *const _) as usize);
        addresses.push((&*b as *const _) as usize);

        // Unbox both values
        let _ = ba.unbox(a);
        assert_eq!(ba.capacity(), 1);
        let _ = ba.unbox(b);
        assert_eq!(ba.capacity(), 2);

        // Wrap a new box
        let c = ba.rebox(0xffffu32);
        assert_eq!(ba.capacity(), 1);
        let c_addr = &*c as *const _ as usize;

        // Check c_addr exists in addresses
        let v = addresses.iter().position(|x| *x == c_addr);
        assert_eq!(v.is_some(), true);

        ba.resize_capacity(4);
        assert_eq!(ba.capacity(), 4);
        ba.resize_capacity(6);
        assert_eq!(ba.capacity(), 6);
        ba.resize_capacity(2);
        assert_eq!(ba.capacity(), 2);

        // Test `try_rebox`
        let boxed = ba.try_rebox(&mut Some(42));
        assert_eq!(ba.capacity(), 1);
        assert_eq!(*boxed.unwrap(), 42);
        let none = ba.try_rebox(&mut None);
        assert_eq!(ba.capacity(), 1);
        assert_eq!(none.is_none(), true);
        ba.resize_capacity(0);
        let none = ba.try_rebox(&mut Some(42));
        assert_eq!(ba.capacity(), 0);
        assert_eq!(none.is_none(), true);
    }
}
