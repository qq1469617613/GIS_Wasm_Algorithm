mod common;
use common::geo_json_to_geometry;
use geo::{Area, Haversine};
use geo::{Distance, Geometry, Point};
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
//投影坐标转换
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

//两点之间距离计算
#[wasm_bindgen]
pub fn haversine_distance(
    point_a_lon: f64,
    point_a_lat: f64,
    point_b_lon: f64,
    point_b_lat: f64,
) -> f64 {
    let point_a = Point::new(point_a_lon, point_a_lat);
    let point_b = Point::new(point_b_lon, point_b_lat);
    Haversine.distance(point_a, point_b)
}

//无洞简单多边形面积计算
#[wasm_bindgen]
pub fn polygon_signed_area(geo_json: JsValue) -> Result<f64, JsValue> {
    let geo = geo_json_to_geometry(geo_json);
    match geo {
        Ok(geo) => Ok(geo.signed_area()),
        Err(_) => f64::try_from(JsValue::from_str(&"数据转化失败".to_string())),
    }
}

//带洞多边形面积计算
pub fn polygon_unsigned_area(geo_json: JsValue) -> Result<f64, JsValue> {
    let geo = geo_json_to_geometry(geo_json);
    match geo {
        Ok(geo) => Ok(geo.unsigned_area()),
        Err(_) => f64::try_from(JsValue::from_str(&"数据转化失败".to_string())),
    }
}
