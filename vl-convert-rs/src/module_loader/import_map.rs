// *************************************************************************
// * This file is generated by vl-convert-vendor/src/main.rs. Do not edit! *
// *************************************************************************
use deno_runtime::deno_core::anyhow::bail;
use deno_runtime::deno_core::error::AnyError;
use std::collections::HashMap;
use std::str::FromStr;

pub const SKYPACK_URL: &str = "https://cdn.skypack.dev";
pub const VEGA_PATH: &str =
    "/pin/vega@v5.30.0-fYDVG3pUN16BiGmbVNdw/mode=imports,min/optimized/vega.js";
pub const VEGA_THEMES_PATH: &str =
    "/pin/vega-themes@v2.14.0-RvUmNETlVH2y3yQM1y36/mode=imports,min/optimized/vega-themes.js";
pub const VEGA_EMBED_PATH: &str =
    "/pin/vega-embed@v6.25.0-3T3K5LYNFrLq0n1oBsLI/mode=imports,min/optimized/vega-embed.js";
pub const DEBOUNCE_PATH: &str = "/pin/lodash.debounce@v4.0.8-aOLIwnE2RethWPrEzTeR/mode=imports,min/optimized/lodash.debounce.js";

pub fn url_for_path(path: &str) -> String {
    format!("{}{}", SKYPACK_URL, path)
}

pub fn vega_url() -> String {
    url_for_path(VEGA_PATH)
}

pub fn vega_themes_url() -> String {
    url_for_path(VEGA_THEMES_PATH)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum VlVersion {
    v5_8,
    v5_12,
    v5_13,
    v5_14,
    v5_15,
    v5_16,
    v5_17,
    v5_18,
    v5_19,
}

impl VlVersion {
    pub fn to_path(self) -> String {
        use VlVersion::*;
        let path = match self {
            v5_8 => "/pin/vega-lite@v5.8.0-4snbURNltT4se5LjMOKF/mode=imports,min/optimized/vega-lite.js",
            v5_12 => "/pin/vega-lite@v5.12.0-ujK64YZaLHcwzRN5lx1E/mode=imports,min/optimized/vega-lite.js",
            v5_13 => "/pin/vega-lite@v5.13.0-GkFo6HVxfKtvVL5RV8aE/mode=imports,min/optimized/vega-lite.js",
            v5_14 => "/pin/vega-lite@v5.14.1-0IRM1VigcIVzRzBRoLFR/mode=imports,min/optimized/vega-lite.js",
            v5_15 => "/pin/vega-lite@v5.15.1-lQeQs8sDPgFa9d7Jm3sd/mode=imports,min/optimized/vega-lite.js",
            v5_16 => "/pin/vega-lite@v5.16.3-Hw7pZxUuaiVgThsNMjY9/mode=imports,min/optimized/vega-lite.js",
            v5_17 => "/pin/vega-lite@v5.17.0-jkfrfJOQ30TsVIlEEbKQ/mode=imports,min/optimized/vega-lite.js",
            v5_18 => "/pin/vega-lite@v5.18.1-CIbWw1F4YnIlhO9UCtHA/mode=imports,min/optimized/vega-lite.js",
            v5_19 => "/pin/vega-lite@v5.19.0-4m5nwXbwdKW9Bc7adV02/mode=imports,min/optimized/vega-lite.js"
        };
        path.to_string()
    }

    pub fn to_url(self) -> String {
        format!("{}{}", SKYPACK_URL, self.to_path())
    }

    pub fn to_semver(self) -> &'static str {
        use VlVersion::*;
        match self {
            v5_8 => "5.8",
            v5_12 => "5.12",
            v5_13 => "5.13",
            v5_14 => "5.14",
            v5_15 => "5.15",
            v5_16 => "5.16",
            v5_17 => "5.17",
            v5_18 => "5.18",
            v5_19 => "5.19",
        }
    }
}

impl Default for VlVersion {
    fn default() -> Self {
        VlVersion::from_str("5.19").unwrap()
    }
}

impl FromStr for VlVersion {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "5.8" | "v5.8" | "5_8" | "v5_8" => Self::v5_8,
            "5.12" | "v5.12" | "5_12" | "v5_12" => Self::v5_12,
            "5.13" | "v5.13" | "5_13" | "v5_13" => Self::v5_13,
            "5.14" | "v5.14" | "5_14" | "v5_14" => Self::v5_14,
            "5.15" | "v5.15" | "5_15" | "v5_15" => Self::v5_15,
            "5.16" | "v5.16" | "5_16" | "v5_16" => Self::v5_16,
            "5.17" | "v5.17" | "5_17" | "v5_17" => Self::v5_17,
            "5.18" | "v5.18" | "5_18" | "v5_18" => Self::v5_18,
            "5.19" | "v5.19" | "5_19" | "v5_19" => Self::v5_19,
            _ => bail!("Unsupported Vega-Lite version string {}", s),
        })
    }
}

pub const VL_VERSIONS: &[VlVersion] = &[
    VlVersion::v5_8,
    VlVersion::v5_12,
    VlVersion::v5_13,
    VlVersion::v5_14,
    VlVersion::v5_15,
    VlVersion::v5_16,
    VlVersion::v5_17,
    VlVersion::v5_18,
    VlVersion::v5_19,
];

pub fn build_import_map() -> HashMap<String, String> {
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert("/-/clone@v2.1.2-inH2VLNzDGiYU9HUWyZM/dist=es2020,mode=imports,min/optimized/clone.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/clone@v2.1.2-inH2VLNzDGiYU9HUWyZM/dist=es2020,mode=imports,min/optimized/clone.js").to_string());
    m.insert("/-/d3-array@v3.2.4-G4hy00bPnjF6FrSYpT32/dist=es2020,mode=imports,min/optimized/d3-array.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-array@v3.2.4-G4hy00bPnjF6FrSYpT32/dist=es2020,mode=imports,min/optimized/d3-array.js").to_string());
    m.insert("/-/d3-color@v3.1.0-MWHDMwd2Pvp3NFjvrHgn/dist=es2020,mode=imports,min/optimized/d3-color.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-color@v3.1.0-MWHDMwd2Pvp3NFjvrHgn/dist=es2020,mode=imports,min/optimized/d3-color.js").to_string());
    m.insert("/-/d3-delaunay@v6.0.4-8vxm7aIldY6XMTxF521F/dist=es2020,mode=imports,min/optimized/d3-delaunay.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-delaunay@v6.0.4-8vxm7aIldY6XMTxF521F/dist=es2020,mode=imports,min/optimized/d3-delaunay.js").to_string());
    m.insert("/-/d3-dispatch@v3.0.1-v6nbfqO2iWOSwp77fYdB/dist=es2020,mode=imports,min/optimized/d3-dispatch.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-dispatch@v3.0.1-v6nbfqO2iWOSwp77fYdB/dist=es2020,mode=imports,min/optimized/d3-dispatch.js").to_string());
    m.insert("/-/d3-dsv@v3.0.1-u1xCRjaLJc0qqv1Z5ERe/dist=es2020,mode=imports,min/optimized/d3-dsv.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-dsv@v3.0.1-u1xCRjaLJc0qqv1Z5ERe/dist=es2020,mode=imports,min/optimized/d3-dsv.js").to_string());
    m.insert("/-/d3-force@v3.0.0-cshj62qMoyIGNIXoil9u/dist=es2020,mode=imports,min/optimized/d3-force.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-force@v3.0.0-cshj62qMoyIGNIXoil9u/dist=es2020,mode=imports,min/optimized/d3-force.js").to_string());
    m.insert("/-/d3-format@v3.1.0-D5wAD2odDPNNWsKloKgL/dist=es2020,mode=imports,min/optimized/d3-format.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-format@v3.1.0-D5wAD2odDPNNWsKloKgL/dist=es2020,mode=imports,min/optimized/d3-format.js").to_string());
    m.insert("/-/d3-geo-projection@v4.0.0-5Hhxj2zKHEqWYAQIFo3r/dist=es2020,mode=imports,min/optimized/d3-geo-projection.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-geo-projection@v4.0.0-5Hhxj2zKHEqWYAQIFo3r/dist=es2020,mode=imports,min/optimized/d3-geo-projection.js").to_string());
    m.insert("/-/d3-geo@v3.1.1-IFH102ROpDQE4rIMzW42/dist=es2020,mode=imports,min/optimized/d3-geo.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-geo@v3.1.1-IFH102ROpDQE4rIMzW42/dist=es2020,mode=imports,min/optimized/d3-geo.js").to_string());
    m.insert("/-/d3-hierarchy@v3.1.2-wx7sW10pU4OkfBLgCDCU/dist=es2020,mode=imports,min/optimized/d3-hierarchy.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-hierarchy@v3.1.2-wx7sW10pU4OkfBLgCDCU/dist=es2020,mode=imports,min/optimized/d3-hierarchy.js").to_string());
    m.insert("/-/d3-interpolate@v3.0.1-i9AsUdFHwyaukRBWNe8d/dist=es2020,mode=imports,min/optimized/d3-interpolate.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-interpolate@v3.0.1-i9AsUdFHwyaukRBWNe8d/dist=es2020,mode=imports,min/optimized/d3-interpolate.js").to_string());
    m.insert("/-/d3-path@v3.1.0-nHaUoYzlRDYONpece9h0/dist=es2020,mode=imports,min/optimized/d3-path.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-path@v3.1.0-nHaUoYzlRDYONpece9h0/dist=es2020,mode=imports,min/optimized/d3-path.js").to_string());
    m.insert("/-/d3-quadtree@v3.0.1-sMtwlDFghZGCTQ3UxKMT/dist=es2020,mode=imports,min/optimized/d3-quadtree.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-quadtree@v3.0.1-sMtwlDFghZGCTQ3UxKMT/dist=es2020,mode=imports,min/optimized/d3-quadtree.js").to_string());
    m.insert("/-/d3-scale-chromatic@v3.1.0-xNM6NGi6M0zu1PK6q2i9/dist=es2020,mode=imports,min/optimized/d3-scale-chromatic.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-scale-chromatic@v3.1.0-xNM6NGi6M0zu1PK6q2i9/dist=es2020,mode=imports,min/optimized/d3-scale-chromatic.js").to_string());
    m.insert("/-/d3-scale@v4.0.2-qUv67mnQQKwRMEsPRKcO/dist=es2020,mode=imports,min/optimized/d3-scale.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-scale@v4.0.2-qUv67mnQQKwRMEsPRKcO/dist=es2020,mode=imports,min/optimized/d3-scale.js").to_string());
    m.insert("/-/d3-shape@v3.2.0-jvLE9CjF3Vp4eEpVme8s/dist=es2020,mode=imports,min/optimized/d3-shape.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-shape@v3.2.0-jvLE9CjF3Vp4eEpVme8s/dist=es2020,mode=imports,min/optimized/d3-shape.js").to_string());
    m.insert("/-/d3-time-format@v4.1.0-f8eZV7eLtGIxvK8uvO3o/dist=es2020,mode=imports,min/optimized/d3-time-format.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-time-format@v4.1.0-f8eZV7eLtGIxvK8uvO3o/dist=es2020,mode=imports,min/optimized/d3-time-format.js").to_string());
    m.insert("/-/d3-time@v3.1.0-hkusO1LcNQpH1ccXwop7/dist=es2020,mode=imports,min/optimized/d3-time.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-time@v3.1.0-hkusO1LcNQpH1ccXwop7/dist=es2020,mode=imports,min/optimized/d3-time.js").to_string());
    m.insert("/-/d3-timer@v3.0.1-O0QpYiI2jhOLEJodLnN1/dist=es2020,mode=imports,min/optimized/d3-timer.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-timer@v3.0.1-O0QpYiI2jhOLEJodLnN1/dist=es2020,mode=imports,min/optimized/d3-timer.js").to_string());
    m.insert("/-/delaunator@v5.0.0-5MwqNhLRHOkBYnNXuoGN/dist=es2020,mode=imports,min/optimized/delaunator.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/delaunator@v5.0.0-5MwqNhLRHOkBYnNXuoGN/dist=es2020,mode=imports,min/optimized/delaunator.js").to_string());
    m.insert("/-/fast-deep-equal@v3.1.3-ysejKs1WDEDPxUJhgGoP/dist=es2020,mode=imports,min/optimized/fast-deep-equal.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/fast-deep-equal@v3.1.3-ysejKs1WDEDPxUJhgGoP/dist=es2020,mode=imports,min/optimized/fast-deep-equal.js").to_string());
    m.insert("/-/fast-json-patch@v3.1.1-IjacxII42OC4A6OXhkDe/dist=es2020,mode=imports,min/optimized/fast-json-patch.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/fast-json-patch@v3.1.1-IjacxII42OC4A6OXhkDe/dist=es2020,mode=imports,min/optimized/fast-json-patch.js").to_string());
    m.insert("/-/fast-json-stable-stringify@v2.1.0-HLgsuOtxPikt0pw16nth/dist=es2020,mode=imports,min/optimized/fast-json-stable-stringify.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/fast-json-stable-stringify@v2.1.0-HLgsuOtxPikt0pw16nth/dist=es2020,mode=imports,min/optimized/fast-json-stable-stringify.js").to_string());
    m.insert("/-/internmap@v2.0.3-GWZlRrRMFcDlELwTQEZq/dist=es2020,mode=imports,min/optimized/internmap.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/internmap@v2.0.3-GWZlRrRMFcDlELwTQEZq/dist=es2020,mode=imports,min/optimized/internmap.js").to_string());
    m.insert("/-/json-stringify-pretty-compact@v3.0.0-RM0i5NMwoiFhg7YNuXef/dist=es2020,mode=imports,min/optimized/json-stringify-pretty-compact.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/json-stringify-pretty-compact@v3.0.0-RM0i5NMwoiFhg7YNuXef/dist=es2020,mode=imports,min/optimized/json-stringify-pretty-compact.js").to_string());
    m.insert("/-/lodash.debounce@v4.0.8-aOLIwnE2RethWPrEzTeR/dist=es2020,mode=imports,min/optimized/lodash.debounce.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/lodash.debounce@v4.0.8-aOLIwnE2RethWPrEzTeR/dist=es2020,mode=imports,min/optimized/lodash.debounce.js").to_string());
    m.insert("/-/robust-predicates@v3.0.1-4p4J15SSeLeNloSXgtZt/dist=es2020,mode=imports,min/optimized/robust-predicates.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/robust-predicates@v3.0.1-4p4J15SSeLeNloSXgtZt/dist=es2020,mode=imports,min/optimized/robust-predicates.js").to_string());
    m.insert("/-/topojson-client@v3.1.0-fyhI24JwGwsqazuuSEoq/dist=es2020,mode=imports,min/optimized/topojson-client.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/topojson-client@v3.1.0-fyhI24JwGwsqazuuSEoq/dist=es2020,mode=imports,min/optimized/topojson-client.js").to_string());
    m.insert("/-/vega-canvas@v1.2.7-hCEcvULuKIOqBVGX1Tn8/dist=es2020,mode=imports,min/optimized/vega-canvas.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-canvas@v1.2.7-hCEcvULuKIOqBVGX1Tn8/dist=es2020,mode=imports,min/optimized/vega-canvas.js").to_string());
    m.insert("/-/vega-crossfilter@v4.1.2-4tsGV0HzWFU39iiKcBjX/dist=es2020,mode=imports,min/optimized/vega-crossfilter.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-crossfilter@v4.1.2-4tsGV0HzWFU39iiKcBjX/dist=es2020,mode=imports,min/optimized/vega-crossfilter.js").to_string());
    m.insert("/-/vega-dataflow@v5.7.6-WDoszrJUyhTwz7sPwzda/dist=es2020,mode=imports,min/optimized/vega-dataflow.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-dataflow@v5.7.6-WDoszrJUyhTwz7sPwzda/dist=es2020,mode=imports,min/optimized/vega-dataflow.js").to_string());
    m.insert("/-/vega-embed@v6.25.0-3T3K5LYNFrLq0n1oBsLI/dist=es2020,mode=imports,min/optimized/vega-embed.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-embed@v6.25.0-3T3K5LYNFrLq0n1oBsLI/dist=es2020,mode=imports,min/optimized/vega-embed.js").to_string());
    m.insert("/-/vega-encode@v4.10.1-Pg43i2sI9rM8xoeOkJ8i/dist=es2020,mode=imports,min/optimized/vega-encode.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-encode@v4.10.1-Pg43i2sI9rM8xoeOkJ8i/dist=es2020,mode=imports,min/optimized/vega-encode.js").to_string());
    m.insert("/-/vega-event-selector@v3.0.1-UgiEAWJA4WQL4DTKnV4R/dist=es2020,mode=imports,min/optimized/vega-event-selector.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-event-selector@v3.0.1-UgiEAWJA4WQL4DTKnV4R/dist=es2020,mode=imports,min/optimized/vega-event-selector.js").to_string());
    m.insert("/-/vega-expression@v5.1.1-K4clrYr2THuj5KncykTn/dist=es2020,mode=imports,min/optimized/vega-expression.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-expression@v5.1.1-K4clrYr2THuj5KncykTn/dist=es2020,mode=imports,min/optimized/vega-expression.js").to_string());
    m.insert("/-/vega-force@v4.2.1-a7V5XmM7au6PlfMbPpDy/dist=es2020,mode=imports,min/optimized/vega-force.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-force@v4.2.1-a7V5XmM7au6PlfMbPpDy/dist=es2020,mode=imports,min/optimized/vega-force.js").to_string());
    m.insert("/-/vega-format@v1.1.2-rBsMjG1MOmb395qvySoI/dist=es2020,mode=imports,min/optimized/vega-format.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-format@v1.1.2-rBsMjG1MOmb395qvySoI/dist=es2020,mode=imports,min/optimized/vega-format.js").to_string());
    m.insert("/-/vega-functions@v5.15.0-Bjrw9nnQutKMtsMi1DSI/dist=es2020,mode=imports,min/optimized/vega-functions.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-functions@v5.15.0-Bjrw9nnQutKMtsMi1DSI/dist=es2020,mode=imports,min/optimized/vega-functions.js").to_string());
    m.insert("/-/vega-geo@v4.4.2-VUooSgw91eB4Vs4BcK3h/dist=es2020,mode=imports,min/optimized/vega-geo.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-geo@v4.4.2-VUooSgw91eB4Vs4BcK3h/dist=es2020,mode=imports,min/optimized/vega-geo.js").to_string());
    m.insert("/-/vega-hierarchy@v4.1.2-3wVtPHq13u3t0KUNwYDf/dist=es2020,mode=imports,min/optimized/vega-hierarchy.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-hierarchy@v4.1.2-3wVtPHq13u3t0KUNwYDf/dist=es2020,mode=imports,min/optimized/vega-hierarchy.js").to_string());
    m.insert("/-/vega-interpreter@v1.0.5-xGayK8haM1EVgaoW7oOi/dist=es2020,mode=imports,min/optimized/vega-interpreter.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-interpreter@v1.0.5-xGayK8haM1EVgaoW7oOi/dist=es2020,mode=imports,min/optimized/vega-interpreter.js").to_string());
    m.insert("/-/vega-label@v1.3.0-9aipnSY6IQFGc3Lm2JSc/dist=es2020,mode=imports,min/optimized/vega-label.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-label@v1.3.0-9aipnSY6IQFGc3Lm2JSc/dist=es2020,mode=imports,min/optimized/vega-label.js").to_string());
    m.insert("/-/vega-lite@v5.12.0-ujK64YZaLHcwzRN5lx1E/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.12.0-ujK64YZaLHcwzRN5lx1E/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.13.0-GkFo6HVxfKtvVL5RV8aE/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.13.0-GkFo6HVxfKtvVL5RV8aE/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.14.1-0IRM1VigcIVzRzBRoLFR/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.14.1-0IRM1VigcIVzRzBRoLFR/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.15.1-lQeQs8sDPgFa9d7Jm3sd/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.15.1-lQeQs8sDPgFa9d7Jm3sd/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.16.3-Hw7pZxUuaiVgThsNMjY9/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.16.3-Hw7pZxUuaiVgThsNMjY9/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.17.0-jkfrfJOQ30TsVIlEEbKQ/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.17.0-jkfrfJOQ30TsVIlEEbKQ/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.18.0-FtLrd9Ks5cFttxoTAph7/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.18.0-FtLrd9Ks5cFttxoTAph7/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.18.1-CIbWw1F4YnIlhO9UCtHA/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.18.1-CIbWw1F4YnIlhO9UCtHA/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.19.0-4m5nwXbwdKW9Bc7adV02/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.19.0-4m5nwXbwdKW9Bc7adV02/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.8.0-4snbURNltT4se5LjMOKF/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.8.0-4snbURNltT4se5LjMOKF/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-loader@v4.5.2-1ImBl2TigOVzvurACWyz/dist=es2020,mode=imports,min/optimized/vega-loader.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-loader@v4.5.2-1ImBl2TigOVzvurACWyz/dist=es2020,mode=imports,min/optimized/vega-loader.js").to_string());
    m.insert("/-/vega-parser@v6.4.0-nwGMLAa2L4N1N7f1iRh9/dist=es2020,mode=imports,min/optimized/vega-parser.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-parser@v6.4.0-nwGMLAa2L4N1N7f1iRh9/dist=es2020,mode=imports,min/optimized/vega-parser.js").to_string());
    m.insert("/-/vega-projection@v1.6.1-zBdxeV9K67u1hPOXPxMf/dist=es2020,mode=imports,min/optimized/vega-projection.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-projection@v1.6.1-zBdxeV9K67u1hPOXPxMf/dist=es2020,mode=imports,min/optimized/vega-projection.js").to_string());
    m.insert("/-/vega-regression@v1.3.0-nz37hjrP5F5zaTiwVMMX/dist=es2020,mode=imports,min/optimized/vega-regression.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-regression@v1.3.0-nz37hjrP5F5zaTiwVMMX/dist=es2020,mode=imports,min/optimized/vega-regression.js").to_string());
    m.insert("/-/vega-runtime@v6.2.0-6BElb2bSMc7jDTsbOGBN/dist=es2020,mode=imports,min/optimized/vega-runtime.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-runtime@v6.2.0-6BElb2bSMc7jDTsbOGBN/dist=es2020,mode=imports,min/optimized/vega-runtime.js").to_string());
    m.insert("/-/vega-scale@v7.4.1-M0T9Gn9zHGGuV6XhZsTO/dist=es2020,mode=imports,min/optimized/vega-scale.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-scale@v7.4.1-M0T9Gn9zHGGuV6XhZsTO/dist=es2020,mode=imports,min/optimized/vega-scale.js").to_string());
    m.insert("/-/vega-scenegraph@v4.13.0-kC2RziuM0eIjiq6SsQ1g/dist=es2020,mode=imports,min/optimized/vega-scenegraph.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-scenegraph@v4.13.0-kC2RziuM0eIjiq6SsQ1g/dist=es2020,mode=imports,min/optimized/vega-scenegraph.js").to_string());
    m.insert("/-/vega-schema-url-parser@v2.2.0-YmXJGRcKOXOac3VG4xfw/dist=es2020,mode=imports,min/optimized/vega-schema-url-parser.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-schema-url-parser@v2.2.0-YmXJGRcKOXOac3VG4xfw/dist=es2020,mode=imports,min/optimized/vega-schema-url-parser.js").to_string());
    m.insert("/-/vega-selections@v5.4.2-kOpqaoZxiZQE5rKxTnvY/dist=es2020,mode=imports,min/optimized/vega-selections.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-selections@v5.4.2-kOpqaoZxiZQE5rKxTnvY/dist=es2020,mode=imports,min/optimized/vega-selections.js").to_string());
    m.insert("/-/vega-statistics@v1.9.0-Qw8CjSQVQOg2M6VMgsme/dist=es2020,mode=imports,min/optimized/vega-statistics.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-statistics@v1.9.0-Qw8CjSQVQOg2M6VMgsme/dist=es2020,mode=imports,min/optimized/vega-statistics.js").to_string());
    m.insert("/-/vega-themes@v2.14.0-RvUmNETlVH2y3yQM1y36/dist=es2020,mode=imports,min/optimized/vega-themes.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-themes@v2.14.0-RvUmNETlVH2y3yQM1y36/dist=es2020,mode=imports,min/optimized/vega-themes.js").to_string());
    m.insert("/-/vega-time@v2.1.2-0zBPNlF5GMoxR6YChBaP/dist=es2020,mode=imports,min/optimized/vega-time.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-time@v2.1.2-0zBPNlF5GMoxR6YChBaP/dist=es2020,mode=imports,min/optimized/vega-time.js").to_string());
    m.insert("/-/vega-tooltip@v0.34.0-YVV3uKnnOnCt2kW7Vclb/dist=es2020,mode=imports,min/optimized/vega-tooltip.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-tooltip@v0.34.0-YVV3uKnnOnCt2kW7Vclb/dist=es2020,mode=imports,min/optimized/vega-tooltip.js").to_string());
    m.insert("/-/vega-transforms@v4.12.0-yproJe1RDXbiUml7qQ0X/dist=es2020,mode=imports,min/optimized/vega-transforms.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-transforms@v4.12.0-yproJe1RDXbiUml7qQ0X/dist=es2020,mode=imports,min/optimized/vega-transforms.js").to_string());
    m.insert("/-/vega-util@v1.17.2-LUfkDhormMyfWqy3Ts6U/dist=es2020,mode=imports,min/optimized/vega-util.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-util@v1.17.2-LUfkDhormMyfWqy3Ts6U/dist=es2020,mode=imports,min/optimized/vega-util.js").to_string());
    m.insert("/-/vega-view-transforms@v4.6.0-xmN4bj51geArzzY2TWCo/dist=es2020,mode=imports,min/optimized/vega-view-transforms.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-view-transforms@v4.6.0-xmN4bj51geArzzY2TWCo/dist=es2020,mode=imports,min/optimized/vega-view-transforms.js").to_string());
    m.insert("/-/vega-view@v5.13.0-kQxd3tMOiOBtyjX1ukMi/dist=es2020,mode=imports,min/optimized/vega-view.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-view@v5.13.0-kQxd3tMOiOBtyjX1ukMi/dist=es2020,mode=imports,min/optimized/vega-view.js").to_string());
    m.insert("/-/vega-voronoi@v4.2.3-8V3D15dKJKAY9dyHESDq/dist=es2020,mode=imports,min/optimized/vega-voronoi.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-voronoi@v4.2.3-8V3D15dKJKAY9dyHESDq/dist=es2020,mode=imports,min/optimized/vega-voronoi.js").to_string());
    m.insert("/-/vega-wordcloud@v4.1.5-5xdRMt86NkNNCDjMSy6z/dist=es2020,mode=imports,min/optimized/vega-wordcloud.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-wordcloud@v4.1.5-5xdRMt86NkNNCDjMSy6z/dist=es2020,mode=imports,min/optimized/vega-wordcloud.js").to_string());
    m.insert("/-/vega@v5.30.0-fYDVG3pUN16BiGmbVNdw/dist=es2020,mode=imports,min/optimized/vega.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega@v5.30.0-fYDVG3pUN16BiGmbVNdw/dist=es2020,mode=imports,min/optimized/vega.js").to_string());
    m.insert("/pin/lodash.debounce@v4.0.8-aOLIwnE2RethWPrEzTeR/mode=imports,min/optimized/lodash.debounce.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/lodash.debounce@v4.0.8-aOLIwnE2RethWPrEzTeR/mode=imports,min/optimized/lodash.debounce.js").to_string());
    m.insert("/pin/vega-embed@v6.25.0-3T3K5LYNFrLq0n1oBsLI/mode=imports,min/optimized/vega-embed.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-embed@v6.25.0-3T3K5LYNFrLq0n1oBsLI/mode=imports,min/optimized/vega-embed.js").to_string());
    m.insert("/pin/vega-lite@v5.12.0-ujK64YZaLHcwzRN5lx1E/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.12.0-ujK64YZaLHcwzRN5lx1E/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.13.0-GkFo6HVxfKtvVL5RV8aE/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.13.0-GkFo6HVxfKtvVL5RV8aE/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.14.1-0IRM1VigcIVzRzBRoLFR/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.14.1-0IRM1VigcIVzRzBRoLFR/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.15.1-lQeQs8sDPgFa9d7Jm3sd/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.15.1-lQeQs8sDPgFa9d7Jm3sd/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.16.3-Hw7pZxUuaiVgThsNMjY9/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.16.3-Hw7pZxUuaiVgThsNMjY9/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.17.0-jkfrfJOQ30TsVIlEEbKQ/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.17.0-jkfrfJOQ30TsVIlEEbKQ/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.18.1-CIbWw1F4YnIlhO9UCtHA/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.18.1-CIbWw1F4YnIlhO9UCtHA/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.19.0-4m5nwXbwdKW9Bc7adV02/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.19.0-4m5nwXbwdKW9Bc7adV02/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.8.0-4snbURNltT4se5LjMOKF/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.8.0-4snbURNltT4se5LjMOKF/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-themes@v2.14.0-RvUmNETlVH2y3yQM1y36/mode=imports,min/optimized/vega-themes.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-themes@v2.14.0-RvUmNETlVH2y3yQM1y36/mode=imports,min/optimized/vega-themes.js").to_string());
    m.insert("/pin/vega@v5.30.0-fYDVG3pUN16BiGmbVNdw/mode=imports,min/optimized/vega.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega@v5.30.0-fYDVG3pUN16BiGmbVNdw/mode=imports,min/optimized/vega.js").to_string());
    m
}

pub fn build_format_locale_map() -> HashMap<String, String> {
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert(
        "ar-001".to_string(),
        include_str!("../../locales/format/ar-001.json").to_string(),
    );
    m.insert(
        "ar-AE".to_string(),
        include_str!("../../locales/format/ar-AE.json").to_string(),
    );
    m.insert(
        "ar-BH".to_string(),
        include_str!("../../locales/format/ar-BH.json").to_string(),
    );
    m.insert(
        "ar-DJ".to_string(),
        include_str!("../../locales/format/ar-DJ.json").to_string(),
    );
    m.insert(
        "ar-DZ".to_string(),
        include_str!("../../locales/format/ar-DZ.json").to_string(),
    );
    m.insert(
        "ar-EG".to_string(),
        include_str!("../../locales/format/ar-EG.json").to_string(),
    );
    m.insert(
        "ar-EH".to_string(),
        include_str!("../../locales/format/ar-EH.json").to_string(),
    );
    m.insert(
        "ar-ER".to_string(),
        include_str!("../../locales/format/ar-ER.json").to_string(),
    );
    m.insert(
        "ar-IL".to_string(),
        include_str!("../../locales/format/ar-IL.json").to_string(),
    );
    m.insert(
        "ar-IQ".to_string(),
        include_str!("../../locales/format/ar-IQ.json").to_string(),
    );
    m.insert(
        "ar-JO".to_string(),
        include_str!("../../locales/format/ar-JO.json").to_string(),
    );
    m.insert(
        "ar-KM".to_string(),
        include_str!("../../locales/format/ar-KM.json").to_string(),
    );
    m.insert(
        "ar-KW".to_string(),
        include_str!("../../locales/format/ar-KW.json").to_string(),
    );
    m.insert(
        "ar-LB".to_string(),
        include_str!("../../locales/format/ar-LB.json").to_string(),
    );
    m.insert(
        "ar-LY".to_string(),
        include_str!("../../locales/format/ar-LY.json").to_string(),
    );
    m.insert(
        "ar-MA".to_string(),
        include_str!("../../locales/format/ar-MA.json").to_string(),
    );
    m.insert(
        "ar-MR".to_string(),
        include_str!("../../locales/format/ar-MR.json").to_string(),
    );
    m.insert(
        "ar-OM".to_string(),
        include_str!("../../locales/format/ar-OM.json").to_string(),
    );
    m.insert(
        "ar-PS".to_string(),
        include_str!("../../locales/format/ar-PS.json").to_string(),
    );
    m.insert(
        "ar-QA".to_string(),
        include_str!("../../locales/format/ar-QA.json").to_string(),
    );
    m.insert(
        "ar-SA".to_string(),
        include_str!("../../locales/format/ar-SA.json").to_string(),
    );
    m.insert(
        "ar-SD".to_string(),
        include_str!("../../locales/format/ar-SD.json").to_string(),
    );
    m.insert(
        "ar-SO".to_string(),
        include_str!("../../locales/format/ar-SO.json").to_string(),
    );
    m.insert(
        "ar-SS".to_string(),
        include_str!("../../locales/format/ar-SS.json").to_string(),
    );
    m.insert(
        "ar-SY".to_string(),
        include_str!("../../locales/format/ar-SY.json").to_string(),
    );
    m.insert(
        "ar-TD".to_string(),
        include_str!("../../locales/format/ar-TD.json").to_string(),
    );
    m.insert(
        "ar-TN".to_string(),
        include_str!("../../locales/format/ar-TN.json").to_string(),
    );
    m.insert(
        "ar-YE".to_string(),
        include_str!("../../locales/format/ar-YE.json").to_string(),
    );
    m.insert(
        "ca-ES".to_string(),
        include_str!("../../locales/format/ca-ES.json").to_string(),
    );
    m.insert(
        "cs-CZ".to_string(),
        include_str!("../../locales/format/cs-CZ.json").to_string(),
    );
    m.insert(
        "da-DK".to_string(),
        include_str!("../../locales/format/da-DK.json").to_string(),
    );
    m.insert(
        "de-CH".to_string(),
        include_str!("../../locales/format/de-CH.json").to_string(),
    );
    m.insert(
        "de-DE".to_string(),
        include_str!("../../locales/format/de-DE.json").to_string(),
    );
    m.insert(
        "en-CA".to_string(),
        include_str!("../../locales/format/en-CA.json").to_string(),
    );
    m.insert(
        "en-GB".to_string(),
        include_str!("../../locales/format/en-GB.json").to_string(),
    );
    m.insert(
        "en-IE".to_string(),
        include_str!("../../locales/format/en-IE.json").to_string(),
    );
    m.insert(
        "en-IN".to_string(),
        include_str!("../../locales/format/en-IN.json").to_string(),
    );
    m.insert(
        "en-US".to_string(),
        include_str!("../../locales/format/en-US.json").to_string(),
    );
    m.insert(
        "es-BO".to_string(),
        include_str!("../../locales/format/es-BO.json").to_string(),
    );
    m.insert(
        "es-ES".to_string(),
        include_str!("../../locales/format/es-ES.json").to_string(),
    );
    m.insert(
        "es-MX".to_string(),
        include_str!("../../locales/format/es-MX.json").to_string(),
    );
    m.insert(
        "fi-FI".to_string(),
        include_str!("../../locales/format/fi-FI.json").to_string(),
    );
    m.insert(
        "fr-CA".to_string(),
        include_str!("../../locales/format/fr-CA.json").to_string(),
    );
    m.insert(
        "fr-FR".to_string(),
        include_str!("../../locales/format/fr-FR.json").to_string(),
    );
    m.insert(
        "he-IL".to_string(),
        include_str!("../../locales/format/he-IL.json").to_string(),
    );
    m.insert(
        "hu-HU".to_string(),
        include_str!("../../locales/format/hu-HU.json").to_string(),
    );
    m.insert(
        "it-IT".to_string(),
        include_str!("../../locales/format/it-IT.json").to_string(),
    );
    m.insert(
        "ja-JP".to_string(),
        include_str!("../../locales/format/ja-JP.json").to_string(),
    );
    m.insert(
        "ko-KR".to_string(),
        include_str!("../../locales/format/ko-KR.json").to_string(),
    );
    m.insert(
        "mk-MK".to_string(),
        include_str!("../../locales/format/mk-MK.json").to_string(),
    );
    m.insert(
        "nl-NL".to_string(),
        include_str!("../../locales/format/nl-NL.json").to_string(),
    );
    m.insert(
        "pl-PL".to_string(),
        include_str!("../../locales/format/pl-PL.json").to_string(),
    );
    m.insert(
        "pt-BR".to_string(),
        include_str!("../../locales/format/pt-BR.json").to_string(),
    );
    m.insert(
        "pt-PT".to_string(),
        include_str!("../../locales/format/pt-PT.json").to_string(),
    );
    m.insert(
        "ru-RU".to_string(),
        include_str!("../../locales/format/ru-RU.json").to_string(),
    );
    m.insert(
        "sl-SI".to_string(),
        include_str!("../../locales/format/sl-SI.json").to_string(),
    );
    m.insert(
        "sv-SE".to_string(),
        include_str!("../../locales/format/sv-SE.json").to_string(),
    );
    m.insert(
        "uk-UA".to_string(),
        include_str!("../../locales/format/uk-UA.json").to_string(),
    );
    m.insert(
        "zh-CN".to_string(),
        include_str!("../../locales/format/zh-CN.json").to_string(),
    );
    m
}

pub fn build_time_format_locale_map() -> HashMap<String, String> {
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert(
        "ar-EG".to_string(),
        include_str!("../../locales/time-format/ar-EG.json").to_string(),
    );
    m.insert(
        "ar-SY".to_string(),
        include_str!("../../locales/time-format/ar-SY.json").to_string(),
    );
    m.insert(
        "ca-ES".to_string(),
        include_str!("../../locales/time-format/ca-ES.json").to_string(),
    );
    m.insert(
        "cs-CZ".to_string(),
        include_str!("../../locales/time-format/cs-CZ.json").to_string(),
    );
    m.insert(
        "da-DK".to_string(),
        include_str!("../../locales/time-format/da-DK.json").to_string(),
    );
    m.insert(
        "de-CH".to_string(),
        include_str!("../../locales/time-format/de-CH.json").to_string(),
    );
    m.insert(
        "de-DE".to_string(),
        include_str!("../../locales/time-format/de-DE.json").to_string(),
    );
    m.insert(
        "en-CA".to_string(),
        include_str!("../../locales/time-format/en-CA.json").to_string(),
    );
    m.insert(
        "en-GB".to_string(),
        include_str!("../../locales/time-format/en-GB.json").to_string(),
    );
    m.insert(
        "en-US".to_string(),
        include_str!("../../locales/time-format/en-US.json").to_string(),
    );
    m.insert(
        "es-ES".to_string(),
        include_str!("../../locales/time-format/es-ES.json").to_string(),
    );
    m.insert(
        "es-MX".to_string(),
        include_str!("../../locales/time-format/es-MX.json").to_string(),
    );
    m.insert(
        "fa-IR".to_string(),
        include_str!("../../locales/time-format/fa-IR.json").to_string(),
    );
    m.insert(
        "fi-FI".to_string(),
        include_str!("../../locales/time-format/fi-FI.json").to_string(),
    );
    m.insert(
        "fr-CA".to_string(),
        include_str!("../../locales/time-format/fr-CA.json").to_string(),
    );
    m.insert(
        "fr-FR".to_string(),
        include_str!("../../locales/time-format/fr-FR.json").to_string(),
    );
    m.insert(
        "he-IL".to_string(),
        include_str!("../../locales/time-format/he-IL.json").to_string(),
    );
    m.insert(
        "hr-HR".to_string(),
        include_str!("../../locales/time-format/hr-HR.json").to_string(),
    );
    m.insert(
        "hu-HU".to_string(),
        include_str!("../../locales/time-format/hu-HU.json").to_string(),
    );
    m.insert(
        "it-IT".to_string(),
        include_str!("../../locales/time-format/it-IT.json").to_string(),
    );
    m.insert(
        "ja-JP".to_string(),
        include_str!("../../locales/time-format/ja-JP.json").to_string(),
    );
    m.insert(
        "ko-KR".to_string(),
        include_str!("../../locales/time-format/ko-KR.json").to_string(),
    );
    m.insert(
        "mk-MK".to_string(),
        include_str!("../../locales/time-format/mk-MK.json").to_string(),
    );
    m.insert(
        "nb-NO".to_string(),
        include_str!("../../locales/time-format/nb-NO.json").to_string(),
    );
    m.insert(
        "nl-BE".to_string(),
        include_str!("../../locales/time-format/nl-BE.json").to_string(),
    );
    m.insert(
        "nl-NL".to_string(),
        include_str!("../../locales/time-format/nl-NL.json").to_string(),
    );
    m.insert(
        "pl-PL".to_string(),
        include_str!("../../locales/time-format/pl-PL.json").to_string(),
    );
    m.insert(
        "pt-BR".to_string(),
        include_str!("../../locales/time-format/pt-BR.json").to_string(),
    );
    m.insert(
        "ru-RU".to_string(),
        include_str!("../../locales/time-format/ru-RU.json").to_string(),
    );
    m.insert(
        "sv-SE".to_string(),
        include_str!("../../locales/time-format/sv-SE.json").to_string(),
    );
    m.insert(
        "tr-TR".to_string(),
        include_str!("../../locales/time-format/tr-TR.json").to_string(),
    );
    m.insert(
        "uk-UA".to_string(),
        include_str!("../../locales/time-format/uk-UA.json").to_string(),
    );
    m.insert(
        "vi-VN".to_string(),
        include_str!("../../locales/time-format/vi-VN.json").to_string(),
    );
    m.insert(
        "zh-CN".to_string(),
        include_str!("../../locales/time-format/zh-CN.json").to_string(),
    );
    m.insert(
        "zh-TW".to_string(),
        include_str!("../../locales/time-format/zh-TW.json").to_string(),
    );
    m
}
