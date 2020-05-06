extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    winres::WindowsResource::new()
      .set_icon("assets/corona.ico")
      .compile()
      .unwrap();
  }
}
