use async_ffi::FfiFuture;
use tokio::runtime::{Handle, Runtime};

fn main() {
    let runtime = Runtime::new().unwrap();
    let handle = runtime.handle();
    runtime.block_on(async move {
        unsafe {
            let lib = libloading::Library::new("../plugin/target/release/libplugin.so").unwrap();
            let func: libloading::Symbol<extern "C" fn(&Handle) -> FfiFuture<()>> =
                lib.get(b"something").unwrap();
            func(handle).await;
        }
    });
}
