use async_ffi::{FfiFuture, FutureExt};
use async_io::Timer;

#[no_mangle]
extern "C" fn something() -> FfiFuture<()> {
    async move {
        println!("hello");
        Timer::after(std::time::Duration::from_secs(3)).await;
        println!("yeah from plugin");
    }
    .into_ffi()
}
