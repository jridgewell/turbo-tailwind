use turbo_tasks_build::generate_register;

// The build.rs build script allows us to automatically generate registrations for every turbo
// value and function in this crate. Think of it like registering singletons in C++.
fn main() {
    // This will generate an output file at "$OUT_DIR/register.rs"
    generate_register();
}
