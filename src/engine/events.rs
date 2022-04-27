use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use super::{Engine, WindowPixel, InputsManager};


pub trait GameLoop {
    fn init(&mut self, _engine: &mut Engine, _window: &mut Window) {}

    fn update(&mut self, _engine: &mut Engine, _window: &mut Window, _inputs: &mut InputsManager) {}
    fn update_end(&mut self, _engine: &mut Engine, _window: &mut Window, _inputs: &mut InputsManager) {}

    fn status(&self, control_flow: &mut ControlFlow) { *control_flow = ControlFlow::Poll; }
}


pub fn init<T>(mut game_loop: T)
    where T: GameLoop + 'static {

    let event_loop = EventLoop::new();

    let mut engine = Engine::new();
    let mut window = WindowPixel::new(&event_loop);
    let mut inputs = InputsManager::new();


    game_loop.init(&mut engine, &mut window.window);
    game_loop.update(&mut engine, &mut window.window, &mut inputs);
    game_loop.update_end(&mut engine, &mut window.window, &mut inputs);


    event_loop.run(move |event, _, control_flow| {
        
        game_loop.status(control_flow);


        match event {
            Event::WindowEvent { event, .. } => match event {

                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::Resized(new_size) => {
                    window.resize(new_size);
                },
                WindowEvent::ScaleFactorChanged  { new_inner_size, ..} => {
                    window.resize(*new_inner_size);
                },

                WindowEvent::KeyboardInput { input, ..} => {
                    inputs.on_keyboard_input(input.virtual_keycode, input.state);
                    engine.set_inputs(inputs.clone());
                },
                WindowEvent::CursorMoved { position, .. } => {
                    inputs.on_cursor_move(position);
                    engine.set_inputs(inputs.clone());
                },
                WindowEvent::MouseInput { state, button, ..} => {
                    inputs.on_mouse_input(button, state);
                    engine.set_inputs(inputs.clone());
                },

                _ => (),
            },

            Event::MainEventsCleared => {

                game_loop.update(&mut engine, &mut window.window, &mut inputs);
                engine.set_inputs(inputs.clone());

                engine.update();
                window.render(&engine);

                game_loop.update_end(&mut engine, &mut window.window, &mut inputs);
            },

            _ => ()
        }
    });
}
