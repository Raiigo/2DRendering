use glutin::event::{Event, StartCause, WindowEvent};

fn main() {

    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Wesh le sang")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024, 768));

    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();

    let window_id = windowed_context.window().id();

    el.run(move |e, wt, cf| {
        if e == Event::MainEventsCleared {

        }
        if e == Event::RedrawEventsCleared {

        }
        if e == Event::NewEvents(StartCause::Poll) {

        }
        if e == (Event::WindowEvent{
            window_id,
            event: WindowEvent::CloseRequested,
        }) {
            println!("Yes");
        }
    });

}
