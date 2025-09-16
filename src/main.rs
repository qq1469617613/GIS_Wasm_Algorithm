mod common;
fn main() {
    let result = common::transform_point(116.404, 39.915, 4326, 3857);
    let (x, y, _z) = result.unwrap();
    println!("北京天安门: ({:.2}, {:.2})", x, y);
}