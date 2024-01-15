use std::fs::read;
use std::path::PathBuf;

use macroquad::prelude::*;

use wasmtime::{component::*, StoreLimits, StoreLimitsBuilder};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{command, Table, WasiCtx, WasiCtxBuilder, WasiView};

use clap::Parser;

bindgen!({
    world: "full",
    path: "./spec",
    async: {
        only_imports: ["next-frame"],
    },
});

struct MyCtx {
    table: Table,
    wasi: WasiCtx,
    limits: StoreLimits,
}

impl WasiView for MyCtx {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

#[allow(clippy::let_unit_value)]
#[async_trait::async_trait]
impl maxiquad::macroquad::color::Host for MyCtx {}

impl From<maxiquad::macroquad::color::Colors> for macroquad::prelude::Color {
    fn from(value: maxiquad::macroquad::color::Colors) -> Self {
        match value {
            maxiquad::macroquad::color::Colors::Lightgray => macroquad::color::colors::LIGHTGRAY,
            maxiquad::macroquad::color::Colors::Gray => macroquad::color::colors::GRAY,
            maxiquad::macroquad::color::Colors::Darkgray => macroquad::color::colors::DARKGRAY,
            maxiquad::macroquad::color::Colors::Yellow => macroquad::color::colors::YELLOW,
            maxiquad::macroquad::color::Colors::Gold => macroquad::color::colors::GOLD,
            maxiquad::macroquad::color::Colors::Orange => macroquad::color::colors::ORANGE,
            maxiquad::macroquad::color::Colors::Pink => macroquad::color::colors::PINK,
            maxiquad::macroquad::color::Colors::Red => macroquad::color::colors::RED,
            maxiquad::macroquad::color::Colors::Maroon => macroquad::color::colors::MAROON,
            maxiquad::macroquad::color::Colors::Green => macroquad::color::colors::GREEN,
            maxiquad::macroquad::color::Colors::Lime => macroquad::color::colors::LIME,
            maxiquad::macroquad::color::Colors::Darkgreen => macroquad::color::colors::DARKGREEN,
            maxiquad::macroquad::color::Colors::Skyblue => macroquad::color::colors::SKYBLUE,
            maxiquad::macroquad::color::Colors::Blue => macroquad::color::colors::BLUE,
            maxiquad::macroquad::color::Colors::Darkblue => macroquad::color::colors::DARKBLUE,
            maxiquad::macroquad::color::Colors::Purple => macroquad::color::colors::PURPLE,
            maxiquad::macroquad::color::Colors::Violet => macroquad::color::colors::VIOLET,
            maxiquad::macroquad::color::Colors::Darkpurple => macroquad::color::colors::DARKPURPLE,
            maxiquad::macroquad::color::Colors::Beige => macroquad::color::colors::BEIGE,
            maxiquad::macroquad::color::Colors::Brown => macroquad::color::colors::BROWN,
            maxiquad::macroquad::color::Colors::Darkbrown => macroquad::color::colors::DARKBROWN,
            maxiquad::macroquad::color::Colors::White => macroquad::color::colors::WHITE,
            maxiquad::macroquad::color::Colors::Black => macroquad::color::colors::BLACK,
            maxiquad::macroquad::color::Colors::Blank => macroquad::color::colors::BLANK,
            maxiquad::macroquad::color::Colors::Magenta => macroquad::color::colors::MAGENTA,
        }
    }
}

#[allow(clippy::let_unit_value)]
#[async_trait::async_trait]
impl maxiquad::macroquad::window::Host for MyCtx {
    fn clear_background(
        &mut self,
        color: maxiquad::macroquad::color::Colors,
    ) -> wasmtime::Result<()> {
        let color = color.into();
        let out = macroquad::window::clear_background(color);
        Ok(out)
    }

    fn screen_width(&mut self) -> wasmtime::Result<f32> {
        let out = macroquad::window::screen_width();
        Ok(out)
    }

    fn screen_height(&mut self) -> wasmtime::Result<f32> {
        let out = macroquad::window::screen_height();
        Ok(out)
    }

    async fn next_frame(&mut self) -> wasmtime::Result<()> {
        let out = macroquad::window::next_frame().await;
        Ok(out)
    }
}

#[allow(clippy::let_unit_value)]
#[async_trait::async_trait]
impl maxiquad::macroquad::shapes::Host for MyCtx {
    fn draw_line(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: maxiquad::macroquad::color::Colors,
    ) -> wasmtime::Result<()> {
        let color = color.into();
        let out = macroquad::shapes::draw_line(start_x, start_y, end_x, end_y, thickness, color);
        Ok(out)
    }

    fn draw_rectangle(
        &mut self,
        pos_x: f32,
        pos_y: f32,
        width: f32,
        height: f32,
        color: maxiquad::macroquad::color::Colors,
    ) -> wasmtime::Result<()> {
        let color = color.into();
        let out = macroquad::shapes::draw_rectangle(pos_x, pos_y, width, height, color);
        Ok(out)
    }

    fn draw_circle(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: maxiquad::macroquad::color::Colors,
    ) -> wasmtime::Result<()> {
        let color = color.into();
        let out = macroquad::shapes::draw_circle(center_x, center_y, radius, color);
        Ok(out)
    }
}

#[allow(clippy::let_unit_value)]
#[async_trait::async_trait]
impl maxiquad::macroquad::text::Host for MyCtx {
    fn draw_text(
        &mut self,
        text: String,
        pos_x: f32,
        pos_y: f32,
        font_size: f32,
        color: maxiquad::macroquad::color::Colors,
    ) -> wasmtime::Result<()> {
        let color = color.into();
        let out = macroquad::text::draw_text(&text, pos_x, pos_y, font_size, color);
        Ok(out)
    }
}

#[allow(clippy::let_unit_value)]
#[async_trait::async_trait]
impl maxiquad::macroquad::extra::Host for MyCtx {
    fn print(&mut self, message: String) -> wasmtime::Result<()> {
        println!("{}", message);
        Ok(())
    }

    fn request_restart(&mut self) -> wasmtime::Result<bool> {
        Ok(false)
    }
}

/// Maxiquad
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the WASM file
    #[arg(short, long)]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut config = Config::new();
    config.wasm_component_model(true).async_support(true);
    let engine = Engine::new(&config)?;

    loop {
        let guest_bytes = read(&args.path)?;
        println!("opening macroquad window");
        macroquad::Window::new("LevoMacroquad", {
            let engine = engine.clone();
            async {
                // <<--- actually this is where we could add the race/select
                app_main(guest_bytes, engine).await.unwrap();
                // use futures::FutureExt;
                // futures::select! {
                //     _ = app_main(guest_bytes, engine).fuse() => {}
                // };
                println!("macroquad window closed because guest finished execution");
            }
        });
    }
    Ok(())
}

async fn app_main(guest_bytes: Vec<u8>, engine: Engine) -> Result<(), Box<dyn std::error::Error>> {
    // Set up Wasmtime linker
    let mut linker = Linker::new(&engine);
    command::add_to_linker(&mut linker)?;

    let table = Table::new();
    let memory_size = 50 << 20; // 50 MB
    let wasi = WasiCtxBuilder::new().build();
    Full::add_to_linker(&mut linker, |state: &mut MyCtx| state)?;
    // Set up Wasmtime store
    let mut store = Store::new(
        &engine,
        MyCtx {
            table,
            wasi,
            limits: StoreLimitsBuilder::new().memory_size(memory_size).build(),
        },
    );
    store.limiter(|state| &mut state.limits);
    let component = Component::new(&engine, guest_bytes)?;
    let (bindings, _) = Full::instantiate_async(&mut store, &component, &linker).await?;
    bindings.call_main(store).await?;
    unreachable!();
    Ok(())
}
