use async_ffi::{FfiFuture, FutureExt};
use tokio::runtime::Handle;

#[no_mangle]
extern "C" fn something(runtime: Handle) -> FfiFuture<()> {
    async move {
        let _guard = runtime.enter();
        println!("hello");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        println!("yeah from plugin");
    }
    .into_ffi()
}
