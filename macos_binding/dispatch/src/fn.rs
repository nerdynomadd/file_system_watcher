use std::ffi::c_void;
use std::mem;
use std::time::Duration;
use crate::ffi::{dispatch_function_t, dispatch_time, DISPATCH_TIME_FOREVER, DISPATCH_TIME_NOW, dispatch_time_t};

extern fn work_fn<F>(context: Box<F>) where F: FnOnce() {
    (*context)();
}

extern fn work_async_fn<F>(context: &mut Option<F>) where F: FnOnce() {
    let closure = context.take().unwrap();
    closure();
}

extern fn work_apply_fn<F>(context: &F, iter: usize) where F: Fn(usize) {
    context(iter);
}

pub fn get_time_after_delay(delay: Duration) -> dispatch_time_t {
    delay.as_secs().checked_mul(1_000_000_000).and_then(|i| {
        i.checked_add(delay.subsec_nanos() as u64)
    }).and_then(|i| {
        if i < (i64::max_value() as u64) { Some(i as i64) } else { None }
    }).map_or(DISPATCH_TIME_FOREVER, |i| unsafe {
        dispatch_time(DISPATCH_TIME_NOW, i)
    })
}

pub fn get_context_and_async_fn<F>(closure: F) -> (*mut c_void, dispatch_function_t)
    where F: FnOnce()
{
    let context = Box::new(closure);
    let func: extern fn(Box<F>) = work_fn::<F>;

    unsafe {
        (Box::into_raw(context) as *mut c_void, mem::transmute(func))
    }
}

pub fn get_context_and_sync_function<F>(closure: &mut Option<F>) -> (*mut c_void, dispatch_function_t)
    where F: FnOnce()
{
    let context: *mut Option<F> = closure;
    let func: extern fn(&mut Option<F>) = work_async_fn::<F>;

    unsafe {
        (context as *mut c_void, mem::transmute(func))
    }
}

pub fn get_context_and_apply_fn<F>(closure: &F) -> (*mut c_void, extern fn(*mut c_void, usize))
    where F: Fn(usize)
{
    let context: *const F = closure;
    let func: extern fn(&F, usize) = work_apply_fn::<F>;

    unsafe {
        (context as *mut c_void, mem::transmute(func))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::r#fn::{get_time_after_delay};

    #[test]
    fn test_get_time_after_delay() {
        let delay = Duration::from_secs(1);
        let time = get_time_after_delay(delay);
        assert_eq!(time, 1_000_000_000);
    }
}