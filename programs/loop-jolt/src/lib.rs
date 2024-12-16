#[jolt::provable]
pub fn func() {
    let iterations = 3000 * 1024;
    for i in 0..iterations {
        memory_barrier(&i);
    }
}

#[allow(unused_variables)]
pub fn memory_barrier<T>(ptr: *const T) {
    #[cfg(target_os = "zkvm")]
    unsafe {
        asm!("/* {0} */", in(reg) (ptr))
    }
    #[cfg(not(target_os = "zkvm"))]
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst)
}
