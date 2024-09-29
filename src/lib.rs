use wasm_bindgen::prelude::*;
use js_sys::Date;
use web_sys::console;

#[wasm_bindgen]
pub struct AnalogWatch {
    hours: f64,
    minutes: f64,
    seconds: f64,
}

#[wasm_bindgen]
impl AnalogWatch {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console::log_1(&"AnalogWatch created".into());
        Self {
            hours: 0.0,
            minutes: 0.0,
            seconds: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = Date::new_0();
        self.hours = now.get_hours() as f64 + now.get_minutes() as f64 / 60.0;
        self.minutes = now.get_minutes() as f64 + now.get_seconds() as f64 / 60.0;
        self.seconds = now.get_seconds() as f64 + now.get_milliseconds() as f64 / 1000.0;
    }

    pub fn hours(&self) -> f64 {
        self.hours
    }

    pub fn minutes(&self) -> f64 {
        self.minutes
    }

    pub fn seconds(&self) -> f64 {
        self.seconds
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    console::log_1(&"WebAssembly module loaded".into());
}