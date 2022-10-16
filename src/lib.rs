use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct DeviceToCloudMessage {
    device_id: String,
    message: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct TelemetryMessage {
    device_id: String,
    message: HashMap<String, f32>,
}

fn parse_input(input: DeviceToCloudMessage) -> TelemetryMessage {
    let mut telemetry = TelemetryMessage {
        device_id: input.device_id,
        message: HashMap::new(),
    };

    match input.message[0] {
        1 => {
            let raw_result: u16 = u16::from(input.message[1]) << 8 | u16::from(input.message[2]);
            let result: f32 = f32::from(raw_result) / 10.0;
            telemetry
                .message
                .insert(String::from("temperature"), result);
        }
        _ => {}
    }

    telemetry.into()
}

#[wasm_bindgen]
pub fn parse_serde_wasm_bindgen(val: JsValue) -> JsValue {
    let input: DeviceToCloudMessage = serde_wasm_bindgen::from_value(val).unwrap();
    let result = parse_input(input);

    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn parse(val: JsValue) -> JsValue {
    let input: DeviceToCloudMessage = val.into_serde().unwrap();
    let result = parse_input(input);

    JsValue::from_serde(&result).unwrap()
}
