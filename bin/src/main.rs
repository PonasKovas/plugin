use tokio::runtime::{Handle, Runtime};

fn main() {
    let runtime = Runtime::new().unwrap();
    unsafe {
        let lib = libloading::Library::new("../plugin/target/release/libplugin.so").unwrap();
        let func: libloading::Symbol<extern "C" fn(&Handle)> = lib.get(b"something").unwrap();
        func(runtime.handle());
    }
    std::thread::park();
}
