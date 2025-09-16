use proj4rs::Proj;
use proj4rs::transform::transform;

pub fn proj_string_for_epsg(epsg: u32) -> Option<String> {
    let s = match epsg {
        // 通用坐标系
        4326 => "+proj=longlat +datum=WGS84 +no_defs +type=crs",
        4979 => "+proj=longlat +datum=WGS84 +no_defs +type=crs",
        4978 => "+proj=geocent +datum=WGS84 +units=m +no_defs +type=crs",
        //Web Mercator
        3857 | 3785 | 900913 | 102113 => {
            "+proj=merc +a=6378137 +b=6378137 +lat_ts=0 +lon_0=0 +x_0=0 +y_0=0 +k=1 +units=m +nadgrids=@null +no_defs +type=crs"
        }
        //欧洲
        4258 => "+proj=longlat +ellps=GRS80 +no_defs +type=crs",
        3035 => {
            "+proj=laea +lat_0=52 +lon_0=10 +x_0=4321000 +y_0=3210000 +ellps=GRS80 +units=m +no_defs +type=crs"
        }
        25832 => "+proj=utm +zone=32 +ellps=GRS80 +units=m +no_defs +type=crs",
        25833 => "+proj=utm +zone=33 +ellps=GRS80 +units=m +no_defs +type=crs",
        //北美
        4269 => "+proj=longlat +datum=NAD83 +no_defs +type=crs",
        26910 => "+proj=utm +zone=10 +datum=NAD83 +units=m +no_defs +type=crs",
        //东亚
        4490 => "+proj=longlat +ellps=GRS80 +no_defs +type=crs",
        3395 => "+proj=merc +lon_0=0 +k=1 +x_0=0 +y_0=0 +datum=WGS84 +units=m +no_defs +type=crs",
        _ => return None,
    };
    Some(s.to_string())
}

pub fn transform_point(lon_deg: f64, lat_deg: f64, from_epsg: u32, to_epsg: u32)->Result<(f64,f64,f64),Box<dyn std::error::Error>> {
    let mut point = (lon_deg.to_radians(), lat_deg.to_radians(), 0.0_f64);
    let from_def = proj_string_for_epsg(from_epsg).ok_or("当前坐标系不支持")?;
    let from_proj=Proj::from_proj_string(&from_def)?;

    let to_def=proj_string_for_epsg(to_epsg).ok_or("当前坐标系不支持")?;
    let to_proj=Proj::from_proj_string(&to_def)?;

    transform(&from_proj,&to_proj,&mut point)?;
    // 如果目标是经纬度坐标系，则将弧度转为度
    if matches!(to_epsg, 4326 | 4979 | 4258 | 4269 | 4490) {
        point.0 = point.0.to_degrees();
        point.1 = point.1.to_degrees();
    }
    Ok(point)
}

