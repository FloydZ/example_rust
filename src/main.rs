use const_fn::const_fn;
pub mod source1;

// function is `const` on specified version and later compiler (including beta and nightly)
#[const_fn("1.36")]
pub const fn version() {}

// function is `const` on nightly compiler (including dev build)
#[const_fn(nightly)]
pub const fn nightly() {}

// function is `const` if `cfg(feature = "intel")` is true
#[const_fn(feature = "intel")]
pub const fn feature() {}


pub fn puffin_example() {
    puffin::set_scopes_on(true); // you may want to control this with a flag
    let mut _puffin_ui = puffin_imgui::ProfilerUi::default();

    // game loop
    loop {
        puffin::GlobalProfiler::lock().new_frame();
        {
            puffin::profile_scope!("slow_code");
            source1::example_bench1(20);
        }
        //_puffin_ui.window(ui);
    }
}


fn main() {
    puffin_example();
}