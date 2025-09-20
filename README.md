###  安装wasm打包工具
```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
> 注意：编译当前代码请确保已经安装 wasm-pack

#### 代码打包命令
```shell
wasm-pack build --target web --out-dir pkg
```
---
###  距离计算用例
```js
import init, {haversine_distance} from './pkg/GIS_Wasm_Algorithm.js';

let wasm;

async function run() {
    wasm = await init();
    console.log('WASM 模块加载成功')
    // 北京天安门 (116.404, 39.915) 与上海东方明珠 (121.506, 31.245) 单位(米)
    const dist = haversine_distance(116.404, 39.915, 121.506, 31.245);
    console.log(dist)
}
```

### 投影坐标转换
```js
import init, { transform_point } from './pkg/GIS_Wasm_Algorithm.js';

async function demoTransform() {
  await init();
  // WGS84 (4326) → Web Mercator (3857)
  const p3857 = transform_point(116.404, 39.915, 4326, 3857);
  console.log('to 3857:', p3857.x, p3857.y, p3857.z);

  // Web Mercator (3857) → WGS84 (4326)
  const p4326 = transform_point(p3857.x, p3857.y, 3857, 4326);
  console.log('back to 4326:', p4326.x, p4326.y, p4326.z);
}

demoTransform();
```
> 当前支持投影坐标系如下：
```
4326 => "+proj=longlat +datum=WGS84 +no_defs +type=crs",
4979 => "+proj=longlat +datum=WGS84 +no_defs +type=crs",
4978 => "+proj=geocent +datum=WGS84 +units=m +no_defs +type=crs",
//Web Mercator
3857 | 3785 | 900913 | 102113 => {"+proj=merc +a=6378137 +b=6378137 +lat_ts=0 +lon_0=0 +x_0=0 +y_0=0 +k=1 +units=m +nadgrids=@null +no_defs +type=crs"}
//欧洲
4258 => "+proj=longlat +ellps=GRS80 +no_defs +type=crs",
3035 => {"+proj=laea +lat_0=52 +lon_0=10 +x_0=4321000 +y_0=3210000 +ellps=GRS80 +units=m +no_defs +type=crs"}
25832 => "+proj=utm +zone=32 +ellps=GRS80 +units=m +no_defs +type=crs",
25833 => "+proj=utm +zone=33 +ellps=GRS80 +units=m +no_defs +type=crs",
//北美
4269 => "+proj=longlat +datum=NAD83 +no_defs +type=crs",
26910 => "+proj=utm +zone=10 +datum=NAD83 +units=m +no_defs +type=crs",
//东亚
4490 => "+proj=longlat +ellps=GRS80 +no_defs +type=crs",
3395 => "+proj=merc +lon_0=0 +k=1 +x_0=0 +y_0=0 +datum=WGS84 +units=m +no_defs +type=crs",
```
### 简单多边形有向面积
```js
import init, { transform_point, polygon_signed_area } from './pkg/GIS_Wasm_Algorithm.js';
async function demoSignedArea() {
  await init();

  // WGS84 GeoJSON Geometry
  const polygon4326 = { "type": "MultiPolygon", "coordinates": [ [ [ [ 114.124185, 22.416509 ], [ 114.123916, 22.416157 ], [ 114.123603, 22.415363 ], [ 114.123352, 22.413997 ], [ 114.123186, 22.413689 ], [ 114.122726, 22.413451 ], [ 114.120443, 22.41306 ], [ 114.118966, 22.412777 ], [ 114.117282, 22.412311 ], [ 114.115962, 22.411605 ], [ 114.110849, 22.407818 ], [ 114.108782, 22.406812 ], [ 114.107384, 22.406421 ], [ 114.106221, 22.406421 ], [ 114.105356, 22.406568 ], [ 114.105305, 22.406576 ], 

  // 将 Polygon 投影到 3857（米）
  const projectRing = (ring) => ring.map(([lon, lat]) => {
    const p = transform_point(lon, lat, 4326, 3857);
    return [p.x, p.y];
  });

  const polygon3857 = {
    type: 'Polygon',
    coordinates: [ projectRing(polygon4326.coordinates[0]) ]
  };

  const area = polygon_signed_area(polygon3857);
  console.log('有向面积(平方米，外环顺时针为负，逆时针为正):', area)
}
demoSignedArea();
```
### 带洞多边形面积计算
```js
import init, { transform_point, polygon_unsigned_area } from './pkg/GIS_Wasm_Algorithm.js';

// 提供的 MultiPolygon（WGS84，经纬度）
const polygon4326 = { "type": "MultiPolygon", "coordinates": [ [ [ [ 114.124185, 22.416509 ], [ 114.123916, 22.416157 ], [ 114.123603, 22.415363 ], [ 114.123352, 22.413997 ], [ 114.123186, 22.413689 ], [ 114.122726, 22.413451 ], [ 114.120443, 22.41306 ], [ 114.118966, 22.412777 ], [ 114.117282, 22.412311 ], [ 114.115962, 22.411605 ], [ 114.110849, 22.407818 ], [ 114.108782, 22.406812 ], [ 114.107384, 22.406421 ], [ 114.106221, 22.406421 ], [ 114.105356, 22.406568 ], [ 114.105305, 22.406576 ], [ 

async function demoUnsignedArea() {
    await init();

    const projectRing = (ring) => ring.map(([lon, lat]) => {
        const p = transform_point(lon, lat, 4326, 3857);
        return [p.x, p.y];
    });

    const projectPolygon = (poly) => poly.map(projectRing);

    const multiPolygon3857 = {
        type: 'MultiPolygon',
        coordinates: polygon4326.coordinates.map(projectPolygon)
    };

    const area = polygon_unsigned_area(multiPolygon3857);
    console.log('无向面积(平方米):', area);
}
```
### Bbox边界框计算
```js
import init, { bbox } from './pkg/GIS_Wasm_Algorithm.js';

async function demoBBox(geometry) {
  await init();
  const arr = bbox(geometry); // JS Array
  console.log('bbox:', Array.from(arr));
}
```

### 质心计算
```js
import init, { centroid } from './pkg/GIS_Wasm_Algorithm.js';
async function demoCentroidFromFC() {
  await init();
  const resp = await fetch('./test/aaa.geojson');
  const data = await resp.json();

  let geom = null;
  if (data && data.type === 'FeatureCollection' && Array.isArray(data.features) && data.features.length > 0) {
    geom = data.features[0].geometry;
  } else if (data && data.type === 'Feature' && data.geometry) {
    geom = data.geometry;
  } else if (data && data.type && data.coordinates) {
    geom = data;
  } else {
    throw new Error('无有效 Geometry');
  }
  const c = centroid(geom);
  console.log('质心:', c.x, c.y, c.z);
}
demoCentroidFromFC();
```

路径规划（Dijkstra 最短路）
```js
import init, { dijkstra } from './pkg/GIS_Wasm_Algorithm.js';
async function run() {
  await init();
  // 一个简单的图（节点 0..4），边的权重表示成本/长度
  const edges = [
    [0, 1, 2.0],
    [1, 2, 1.0],
    [0, 3, 1.5],
    [3, 2, 2.5],
    [2, 4, 1.2],
    // 也支持对象形式：{ from: 1, to: 3, weight: 0.9 }
  ];
  const start = 0;          // 起点节点 ID
  const undirected = true;  // 是否无向图（true 会自动补反向边）

  // 运行 Dijkstra
  const out = dijkstra(edges, start, undirected);
  // out 是一个数组，每项形如 { node, pred, dist }

  // 还原到目标节点(4)的路径
  const pred = new Map(out.map(o => [Number(o.node), o.pred == null ? null : Number(o.pred)]));
  const dist = new Map(out.map(o => [Number(o.node), Number(o.dist)]));

  const target = 4;
  const path = [];
  let cur = target;
  while (cur != null) {
    path.push(cur);
    const p = pred.get(cur);
    if (p == null) break; // 到达起点
    cur = p;
  }
  path.reverse();

  console.log('最短路径节点序列:', path);        // 例如: [0, 1, 2, 4]
  console.log('总权重/距离:', dist.get(target)); // 例如: 4.2
}
run();
```