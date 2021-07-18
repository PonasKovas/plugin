use tokio::runtime::Handle;

#[no_mangle]
extern "C" fn something(runtime: &Handle) {
    runtime.block_on(async move {
        println!("hello");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        println!("yeah");
    });
}
