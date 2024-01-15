// src/lib.rs

// use lazy_static::lazy_static; // 1.4.0
use maxiquad::macroquad;

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
    path: "../../spec",
    // the name of the world in the `*.wit` input file
    world: "full",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        world: MyWorld,

    },
});

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct MyWorld;

impl Guest for MyWorld {
    fn main() {
        loop {
            macroquad::window::clear_background(macroquad::color::Colors::Lightgray);

            macroquad::shapes::draw_line(
                40.0,
                40.0,
                100.0,
                200.0,
                15.0,
                macroquad::color::Colors::Blue,
            );
            macroquad::shapes::draw_rectangle(
                macroquad::window::screen_width() / 2.0 - 60.0,
                100.0,
                120.0,
                60.0,
                macroquad::color::Colors::Blue,
            );
            macroquad::shapes::draw_circle(
                macroquad::window::screen_width() - 30.0,
                macroquad::window::screen_height() - 30.0,
                15.0,
                macroquad::color::Colors::Yellow,
            );
            macroquad::text::draw_text(
                "HELLO",
                20.0,
                20.0,
                20.0,
                macroquad::color::Colors::Darkgray,
            );

            if macroquad::extra::request_restart() {
                break;
            }
            macroquad::window::next_frame()
        }
    }
}
