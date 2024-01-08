use std::fs::read;
use std::path::PathBuf;

use levo::portal::my_imports::{self, Host};
use macroquad::prelude::*;

use wasmtime::{component::*, StoreLimits, StoreLimitsBuilder};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::command::add_to_linker;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

use clap::Parser;

bindgen!({
    world: "my-world",
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

#[async_trait::async_trait]
impl Host for MyCtx {
    fn clear_background(&mut self, color: my_imports::Color) -> wasmtime::Result<()> {
        macroquad::prelude::clear_background(color.into());
        Ok(())
    }

    fn draw_line(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: my_imports::Color,
    ) -> wasmtime::Result<()> {
        macroquad::prelude::draw_line(start_x, start_y, end_x, end_y, thickness, color.into());
        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        pos_x: f32,
        pos_y: f32,
        width: f32,
        height: f32,
        color: my_imports::Color,
    ) -> wasmtime::Result<()> {
        macroquad::prelude::draw_rectangle(pos_x, pos_y, width, height, color.into());
        Ok(())
    }

    fn draw_circle(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: my_imports::Color,
    ) -> wasmtime::Result<()> {
        macroquad::prelude::draw_circle(center_x, center_y, radius, color.into());
        Ok(())
    }

    fn draw_text(
        &mut self,
        text: String,
        pos_x: f32,
        pos_y: f32,
        font_size: f32,
        color: my_imports::Color,
    ) -> wasmtime::Result<()> {
        macroquad::prelude::draw_text(&text, pos_x, pos_y, font_size, color.into());
        Ok(())
    }

    fn screen_width(&mut self) -> wasmtime::Result<f32> {
        Ok(macroquad::prelude::screen_width())
    }

    fn screen_height(&mut self) -> wasmtime::Result<f32> {
        Ok(macroquad::prelude::screen_height())
    }

    async fn next_frame(&mut self) -> wasmtime::Result<()> {
        macroquad::prelude::next_frame().await;
        Ok(())
    }
}

impl From<levo::portal::my_imports::Color> for macroquad::prelude::Color {
    fn from(value: levo::portal::my_imports::Color) -> Self {
        match value {
            levo::portal::my_imports::Color::LightGray => LIGHTGRAY,
            levo::portal::my_imports::Color::Gray => GRAY,
            levo::portal::my_imports::Color::DarkGray => DARKGRAY,
            levo::portal::my_imports::Color::Yellow => YELLOW,
            levo::portal::my_imports::Color::Gold => GOLD,
            levo::portal::my_imports::Color::Orange => ORANGE,
            levo::portal::my_imports::Color::Pink => PINK,
            levo::portal::my_imports::Color::Red => RED,
            levo::portal::my_imports::Color::Maroon => MAROON,
            levo::portal::my_imports::Color::Green => GREEN,
            levo::portal::my_imports::Color::Lime => LIME,
            levo::portal::my_imports::Color::DarkGreen => DARKGREEN,
            levo::portal::my_imports::Color::SkyBlue => SKYBLUE,
            levo::portal::my_imports::Color::Blue => BLUE,
            levo::portal::my_imports::Color::DarkBlue => DARKBLUE,
            levo::portal::my_imports::Color::Purple => PURPLE,
            levo::portal::my_imports::Color::Violet => VIOLET,
            levo::portal::my_imports::Color::DarkPurple => DARKPURPLE,
            levo::portal::my_imports::Color::Beige => BEIGE,
            levo::portal::my_imports::Color::Brown => BROWN,
            levo::portal::my_imports::Color::DarkBrown => DARKBROWN,
            levo::portal::my_imports::Color::White => WHITE,
            levo::portal::my_imports::Color::Black => BLACK,
            levo::portal::my_imports::Color::Blank => BLANK,
            levo::portal::my_imports::Color::Magenta => MAGENTA,
        }
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

#[macroquad::main("LevoMacroquad")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let guest_bytes = read(args.path)?;
    let mut config = Config::new();
    config.wasm_component_model(true).async_support(true);
    let engine = Engine::new(&config)?;
    let component = Component::new(&engine, guest_bytes)?;

    // Set up Wasmtime linker
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker)?;
    let table = Table::new();
    let memory_size = 50 << 20; // 50 MB
    let wasi = WasiCtxBuilder::new().build();
    MyWorld::add_to_linker(&mut linker, |state: &mut MyCtx| state)?;
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
    let (bindings, _) = MyWorld::instantiate_async(&mut store, &component, &linker).await?;
    bindings.call_main(store).await?;
    Ok(())
}
