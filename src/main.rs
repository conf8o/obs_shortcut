extern crate multiinput;

use multiinput::*;
mod router;


const PRESS_W: usize = 1;

fn init_router() -> router::Router {
    let router  = router::Router::init(
        vec![
            (PRESS_W, || 1)
        ]
    );

    router
}

fn main() {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Joysticks(XInputInclude::True));
    manager.register_devices(DeviceType::Keyboards);
    manager.register_devices(DeviceType::Mice);
    'outer: loop{
        if let Some(event) = manager.get_event(){
            match event{
                RawEvent::KeyboardEvent(_,  KeyId::Escape, State::Pressed)
                    => break 'outer,
                _ => (),
            }
            println!("{:?}", event);
        }
    }
    println!("Finishing");
}