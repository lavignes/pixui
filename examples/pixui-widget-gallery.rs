use std::{
    fs::File,
    io::{self, BufReader},
    time::{Duration, Instant},
};

use pixui::{
    gfx::{
        BdfFont, BlendMode, Color, Point, Rect, Scalar, Size, Surface, U8SliceSurface, WriteSurface,
    },
    ui::{Border, Feedback, HBox, Label, Margin, VBox, UI},
};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::PixelFormatEnum,
};

const SCALE_FACTOR: Scalar = 2;

fn main() -> io::Result<()> {
    let sdl = sdl2::init().unwrap();
    let sdl_video = sdl.video().unwrap();

    let mut window_size = Size::new(640, 480) * SCALE_FACTOR;
    let window = sdl_video
        .window(
            "PIXUI Widget Gallery",
            window_size.width() as u32,
            window_size.height() as u32,
        )
        .resizable()
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture_size = window_size / SCALE_FACTOR;
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA32,
            texture_size.width() as u32,
            texture_size.height() as u32,
        )
        .unwrap();

    let mut frames = 0;
    let mut frame_timer = Instant::now();
    let mut event_timer = Instant::now();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut ui = UI::new();
    let font = BdfFont::new(&mut BufReader::new(File::open(
        "examples/ProggyCleanSZ.bdf",
    )?))?;

    let mut text = String::new();
    let mut fps = String::new();
    let mut cursor = Point::ZERO;
    let mut touch = false;
    let mut offset = Point::ZERO;

    'running: loop {
        if event_timer.elapsed() >= Duration::from_millis(16) {
            event_timer = Instant::now();
            while let Some(event) = event_pump.poll_event() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::TextInput { text: t, .. } => text.push_str(&t),
                    Event::KeyDown {
                        keycode: Some(Keycode::Backspace),
                        ..
                    } => {
                        text.pop();
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Return),
                        ..
                    } => {
                        text.push('\n');
                    }
                    Event::Window {
                        win_event: WindowEvent::Resized(width, height),
                        ..
                    } => {
                        window_size = (width, height).into();
                        texture_size = window_size / SCALE_FACTOR;
                        texture = texture_creator
                            .create_texture_streaming(
                                PixelFormatEnum::RGBA32,
                                texture_size.width() as u32,
                                texture_size.height() as u32,
                            )
                            .unwrap();
                    }
                    Event::MouseMotion { x, y, .. } => cursor = Point::new(x, y) / SCALE_FACTOR,
                    Event::MouseButtonDown { .. } => touch = true,
                    Event::MouseButtonUp { .. } => touch = false,
                    _ => {}
                }
            }
        }

        let mut feedback = Feedback::default();
        texture
            .with_lock(None, |pixels, _| {
                let mut surface =
                    U8SliceSurface::new(pixels, texture_size.width(), Rect::sized(texture_size));

                surface.fill(surface.bounds(), Color::BLACK, BlendMode::None);
                feedback = ui.render(
                    surface.bounds().inset(cursor.y, cursor.x, 0, 0),
                    &mut surface,
                    cursor,
                    &Border {
                        weight: 1,
                        color: Color::opaque(0, 255, 0),
                        child: Some(&Margin {
                            top: 2,
                            left: 2,
                            bottom: 2,
                            right: 2,
                            child: Some(&Border {
                                weight: 1,
                                color: Color::WHITE,
                                child: Some(&Margin {
                                    top: 8,
                                    left: 8,
                                    bottom: 8,
                                    right: 8,
                                    child: Some(&Label {
                                        id: Some("label"),
                                        text: format!(
                                            "({}, {}) :: {fps} :: {}",
                                            cursor.x, cursor.y, text
                                        )
                                        .as_str(),
                                        color: Color::opaque(255, 0, 0),
                                        font: Some(&font),
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                );
            })
            .unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        if let Some(hit) = feedback.hit() {
            if touch {
                dbg!(hit, cursor);
                offset = cursor.relative_to(hit.bounds().origin);
            }
        }

        frames += 1;
        if frame_timer.elapsed() >= Duration::from_secs(1) {
            eprintln!("fps: {frames}");
            fps = format!("fps: {frames}");
            frames = 0;
            frame_timer = Instant::now();
        }
    }

    Ok(())
}
