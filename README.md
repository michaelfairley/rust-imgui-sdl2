# imgui-sdl2

[![Build Status](https://travis-ci.org/michaelfairley/rust-imgui-sdl2.svg?branch=master)](https://travis-ci.org/michaelfairley/rust-imgui-sdl2)
[![Documentation](https://docs.rs/imgui-sdl2/badge.svg)](https://docs.rs/imgui-sdl2)
[![Version](https://img.shields.io/crates/v/imgui-sdl2.svg)](https://crates.io/crates/imgui-sdl2)

[SDL2](https://github.com/Rust-SDL2/rust-sdl2) Input handling for [imgui-rs](https://github.com/Gekkio/imgui-rs)

## Integration guide

1. Construct it.
   ```rust
   let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
   ```
2. At the top of your event handling loop, pass in the input events, and ignore the ones that imgui has captured.
   ```rust
   imgui_sdl2.handle_event(&mut imgui, &event);
   if imgui_sdl2.ignore_event(&event) { continue; }
   ```
3. Call `prepare_frame` before calling `imgui.frame()`.
   ```rust
   imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());
   ```
4. Call `prepare_render` immediately before your UI rendering code.
   ```rust
   imgui_sdl2.prepare_render(&ui, &window);
   ```

Take a look at the [example app](https://github.com/michaelfairley/rust-imgui-sdl2/blob/master/examples/demo.rs) to see it all in context.
