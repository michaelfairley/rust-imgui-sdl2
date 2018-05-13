# imgui-sdl2

[![Build Status](https://travis-ci.org/michaelfairley/rust-imgui-sdl2.svg?branch=master)](https://travis-ci.org/michaelfairley/rust-imgui-sdl2)
[![Documentation](https://docs.rs/imgui-sdl2/badge.svg)](https://docs.rs/imgui-sdl2)
[![Version](https://img.shields.io/crates/v/imgui-sdl.svg)](https://crates.io/crates/imgui-sdl2)

[SDL2](https://github.com/Rust-SDL2/rust-sdl2) Input handling for [imgui-rs](https://github.com/Gekkio/imgui-rs)

## Integration guide

1. Construct it.
   ```rust
   let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui);
   ```
2. At the top of your event handling loop, pass in the input events, and ignore the ones that imgui has captured.
   ```rust
   imgui_sdl2.handle_event(&mut imgui, &event);
   if imgui_sdl2.ignore_event(&event) { continue; }
   ```
3. After handling input, call `frame` to start drawing.
   ```rust
   let ui = imgui_sdl2.frame(&window, &mut imgui, &event_pump);
   ```

Take a look at the [example app](https://github.com/michaelfairley/rust-imgui-sdl2/blob/master/examples/demo.rs) to see it all in context.
