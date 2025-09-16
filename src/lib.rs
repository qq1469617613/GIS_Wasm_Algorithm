mod common;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JsPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl JsPoint {
    pub fn new(x: f64, y: f64, z: f64) -> JsPoint {
        JsPoint { x, y, z }
    }
}

#[wasm_bindgen]
pub fn transform_point(
    lon_deg: f64,
    lat_deg: f64,
    from_epsg: u32,
    to_epsg: u32,
) -> Result<JsPoint, JsValue> {
    match common::transform_point(lon_deg, lat_deg, from_epsg, to_epsg) {
        Ok((x, y, z)) => Ok(JsPoint { x, y, z }),
        Err(e) => Err(JsValue::from_str(&format!(
            "坐标系转换失败{}",
            &e.to_string()
        ))),
    }
}
