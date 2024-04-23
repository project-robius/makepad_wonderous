pub mod app;
mod artifacts;
mod gallery;
mod shared;
mod timeline;
mod wonder;

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn main() {
    app::app_main()
}
