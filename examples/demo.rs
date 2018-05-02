extern crate glium;
extern crate glium_sdl2;
extern crate imgui_glium_renderer;
extern crate sdl2;
extern crate imgui;
extern crate imgui_sdl2;

fn main() {
  use glium_sdl2::DisplayBuild;
  use glium::Surface;

  let sdl_context = sdl2::init().unwrap();
  let video = sdl_context.video().unwrap();

  {
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
  }

  let display = video.window("rust-imgui-sdl2 demo", 1000, 1000)
    .position_centered()
    .resizable()
    .opengl()
    .build_glium()
    .unwrap();

  let mut imgui = imgui::ImGui::init();
  imgui.set_ini_filename(None);


  let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(display.window(), &mut imgui);

  let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

  let mut event_pump = sdl_context.event_pump().unwrap();


  'running: loop {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    for event in event_pump.poll_iter() {
      imgui_sdl2.handle_event(&mut imgui, &event);

      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        _ => {}
      }
    }


    let ui = imgui_sdl2.frame(&mut imgui, &event_pump);
    ui.show_test_window(&mut true);

    let mut target = display.draw();
    target.clear_color(0.2, 0.2, 0.2, 1.0);
    renderer.render(&mut target, ui).expect("Rendering failed");
    target.finish().unwrap();


    ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
  }
}
