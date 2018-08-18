#[macro_use]
extern crate conrod;
extern crate glium;
extern crate find_folder;
extern crate chrono;

mod support;

use chrono::Local;
use glium::Surface;




fn get_time() -> String{
    let date = Local::now();
    format!("{}", date.format("%H:%M"))
}




// Draw the Ui.
fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids) {
    use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};

    // Construct our main `Canvas` tree.
    widget::Canvas::new().flow_down(&[
        (ids.clock, widget::Canvas::new().color(color::rgba(0.1, 0.1, 0.1, 1.0)).pad_bottom(10.0).length(240.0)),
        (ids.body, widget::Canvas::new().color(color::rgba(0.15, 0.15, 0.15, 1.0)).pad_bottom(20.0))
    ]).set(ids.master, ui);

    // Here we make some canvas `Tabs` in the middle column.
    widget::Tabs::new(&[(ids.tab_radio, "RADIO"), (ids.tab_player, "PLAYLISTS"), (ids.tab_usb, "USB")])
        .wh_of(ids.body)
        .color(color::rgba(0.15, 0.15, 0.15, 1.0))
        .label_color(color::WHITE)
        .middle_of(ids.body)
        .bar_thickness(70.0)
        .set(ids.tabs, ui);

    widget::Text::new(&get_time()[..])
        .color(color::rgba(0.75, 0.75, 0.75, 1.0))
        .font_size(160)
        .middle_of(ids.clock)
        .set(ids.time, ui);

    fn text (text: widget::Text) -> widget::Text { text.color(color::WHITE).font_size(36) }
    text(widget::Text::new("Radio")).middle_of(ids.tab_radio).set(ids.radio_label, ui);
    text(widget::Text::new("Player")).middle_of(ids.tab_player).set(ids.player_label, ui);
    text(widget::Text::new("USB")).middle_of(ids.tab_usb).set(ids.usb_label, ui);
}



// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        clock,
        body,
        tabs,
        tab_radio,
        tab_player,
        tab_usb,

        time,
        usb_label,
        player_label,
        radio_label
    }
}



fn main() {

    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 1024;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Canvas")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").expect("Could not find folder \"assets\" in project");
    // let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    // ui.fonts.insert_from_file(font_path).expect("Didn't find font in assets!");
    let font_path = assets.join("fonts/Din/DIN Light.ttf");
    ui.fonts.insert_from_file(font_path).expect("Didn't find font in assets!");
    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate all widgets in the GUI.
        // This updates all widgets according to the state specified in set_widgets()
        set_widgets(ui.set_widgets(), ids);


        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

