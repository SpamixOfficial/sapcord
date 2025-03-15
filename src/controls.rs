use embassy_rp::{
    gpio::{Input, Pull},
    peripherals::{PIN_12, PIN_13, PIN_14, PIN_15, PIN_5, PIN_6, PIN_7, PIN_8},
};
use embassy_time::Instant;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Button {
    W,
    A,
    S,
    D,
    I,
    J,
    K,
    L,
    None,
}

pub struct Controls {
    pub pressed_button: Button,
    last_press: Instant,
    last_pressed_button: Button,
    pins: [(Button, Input<'static>); 8],
}

impl Controls {
    pub fn init(
        w: PIN_5,
        a: PIN_6,
        s: PIN_7,
        d: PIN_8,
        i: PIN_12,
        j: PIN_13,
        k: PIN_14,
        l: PIN_15,
    ) -> Self {
        let pins = [
            (Button::W, Input::new(w, Pull::Up)),
            (Button::A, Input::new(a, Pull::Up)),
            (Button::S, Input::new(s, Pull::Up)),
            (Button::D, Input::new(d, Pull::Up)),
            (Button::I, Input::new(i, Pull::Up)),
            (Button::J, Input::new(j, Pull::Up)),
            (Button::K, Input::new(k, Pull::Up)),
            (Button::L, Input::new(l, Pull::Up)),
        ];
        return Self {
            pressed_button: Button::None,
            last_press: Instant::MIN,
            last_pressed_button: Button::None,
            pins,
        };
    }
    pub async fn check_for_input(&mut self) {
        self.pressed_button = Button::None;
        for p in &self.pins {
            if p.1.is_low() {
                if Instant::now().duration_since(self.last_press).as_millis() < 200
                    && self.last_pressed_button == p.0
                    && p.0 != Button::None
                {
                    return;
                }
                self.pressed_button = p.0.clone();
                self.last_pressed_button = self.pressed_button.clone();
                self.last_press = Instant::now();
                break;
            }
        }
    }
}
