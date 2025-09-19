mod common;
mod dijkstra;

use crate::dijkstra::build_graph;
use common::geo_json_to_geometry;
use geo::{Area, BoundingRect, Centroid, Haversine};
use geo::{Distance, Point};
use ordered_float::OrderedFloat;
use serde::Serialize;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
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
#[wasm_bindgen]
pub fn polygon_unsigned_area(geo_json: JsValue) -> Result<f64, JsValue> {
    let geo = geo_json_to_geometry(geo_json);
    match geo {
        Ok(geo) => Ok(geo.unsigned_area()),
        Err(_) => f64::try_from(JsValue::from_str(&"数据转化失败".to_string())),
    }
}

//bbox计算
#[wasm_bindgen]
pub fn bbox(geo_json: JsValue) -> Result<JsValue, JsValue> {
    let geom = geo_json_to_geometry(geo_json);
    match geom {
        Ok(geo) => {
            if let Some(rect) = geo.bounding_rect() {
                let arr = js_sys::Array::new();
                arr.push(&JsValue::from_f64(rect.min().x));
                arr.push(&JsValue::from_f64(rect.min().y));
                arr.push(&JsValue::from_f64(rect.max().x));
                arr.push(&JsValue::from_f64(rect.max().y));
                Ok(arr.into())
            } else {
                Err(JsValue::from_str("bbox获取失败"))
            }
        }
        Err(_) => Err(JsValue::from_str("数据转化失败")),
    }
}

#[wasm_bindgen]
pub fn centroid(geo_json: JsValue) -> Result<JsPoint, JsValue> {
    let geo = geo_json_to_geometry(geo_json);
    match geo?.centroid() {
        Some(centroid) => Ok(JsPoint {
            x: centroid.x(),
            y: centroid.y(),
            z: 0.0,
        }),
        None => Err(JsValue::from_str("质心计算失败")),
    }
}

#[derive(Serialize)]
struct PathItem {
    node: u32,
    pred: Option<u32>,
    dist: f64,
}

#[wasm_bindgen]
pub fn dijkstra(edges: JsValue, start: u32, undirected: bool) -> Result<JsValue, JsValue> {
    let inputs: Vec<dijkstra::InputDescription> =
        serde_wasm_bindgen::from_value(edges).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let graph = build_graph(&inputs, undirected);
    let mut dist: BTreeMap<u32, f64> = BTreeMap::new();
    let mut pred: BTreeMap<u32, Option<u32>> = BTreeMap::new();
    let mut priority_queue: BinaryHeap<std::cmp::Reverse<(OrderedFloat<f64>, u32)>> =
        BinaryHeap::new();

    dist.insert(start, 0.0);
    pred.insert(start, None);
    priority_queue.push(Reverse((OrderedFloat(0.0), start)));

    while let Some(Reverse((OrderedFloat(d), u))) = priority_queue.pop() {
        if let Some(&best) = dist.get(&u) {
            if d > best {
                continue;
            }
        }
        if let Some(neighs) = graph.get(&u) {
            for (&v, &w) in neighs.iter() {
                let nd = d + w;
                let update = match dist.get(&v) {
                    None => true,
                    Some(&old) => nd < old,
                };
                if update {
                    dist.insert(v, nd);
                    pred.insert(v, Some(u));
                    priority_queue.push(Reverse((OrderedFloat(nd), v)));
                }
            }
        }
    }
    let mut out: Vec<PathItem> = Vec::with_capacity(dist.len());
    for (node, &d) in dist.iter() {
        let p = pred.get(node).cloned().unwrap_or(None);
        out.push(PathItem {
            node: *node,
            pred: p,
            dist: d,
        });
    }
    out.sort_by_key(|x| x.node);
    serde_wasm_bindgen::to_value(&out)
        .map_err(|e| JsValue::from_str(&format!("Serialize error: {}", e)))
}
