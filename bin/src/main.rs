use async_ffi::FfiFuture;
use tokio::runtime::Runtime;

fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async move {
        unsafe {
            let lib = libloading::Library::new("../plugin/target/release/libplugin.so").unwrap();
            let func: libloading::Symbol<extern "C" fn() -> FfiFuture<()>> =
                lib.get(b"something").unwrap();
            func().await;
        }
    });
}
