use std::alloc::{GlobalAlloc, System, Layout,alloc,dealloc,};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;


///if we want to use other allocator
///
/// extern crate jemallocator;
/// use jemallocator::Jemalloc;
///
/// #[global_allocator]
/// static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    // This `Vec` will allocate memory through `GLOBAL` above
    let mut v = Vec::new();
    v.push(1);



    unsafe {
        let layout=Layout::new::<i32>();
    }
}