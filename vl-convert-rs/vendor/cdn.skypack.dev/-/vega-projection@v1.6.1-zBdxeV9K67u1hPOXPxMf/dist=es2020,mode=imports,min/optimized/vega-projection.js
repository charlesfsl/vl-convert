import{geoPath as n,geoAlbers as u,geoAlbersUsa as g,geoAzimuthalEqualArea as m,geoAzimuthalEquidistant as p,geoConicConformal as d,geoConicEqualArea as h,geoConicEquidistant as f,geoEqualEarth as E,geoEquirectangular as q,geoGnomonic as j,geoIdentity as z,geoMercator as A,geoNaturalEarth1 as v,geoOrthographic as b,geoStereographic as P,geoTransverseMercator as w}from"/-/d3-geo@v3.1.1-IFH102ROpDQE4rIMzW42/dist=es2020,mode=imports,min/optimized/d3-geo.js";import{geoMollweide as C}from"/-/d3-geo-projection@v4.0.0-5Hhxj2zKHEqWYAQIFo3r/dist=es2020,mode=imports,min/optimized/d3-geo-projection.js";import{registerScale as M}from"/-/vega-scale@v7.4.1-M0T9Gn9zHGGuV6XhZsTO/dist=es2020,mode=imports,min/optimized/vega-scale.js";const x=n(),c=["clipAngle","clipExtent","scale","translate","center","rotate","parallels","precision","reflectX","reflectY","coefficient","distance","fraction","lobes","parallel","radius","ratio","spacing","tilt"];function G(e,r){return function l(){const o=r();return o.type=e,o.path=n().projection(o),o.copy=o.copy||function(){const i=l();return c.forEach(a=>{o[a]&&i[a](o[a]())}),i.path.pointRadius(o.path.pointRadius()),i},M(o)}}function s(e,r){if(!e||typeof e!="string")throw new Error("Projection type must be a name string.");return e=e.toLowerCase(),arguments.length>1?(t[e]=G(e,r),this):t[e]||null}function H(e){return e&&e.path||x}const t={albers:u,albersusa:g,azimuthalequalarea:m,azimuthalequidistant:p,conicconformal:d,conicequalarea:h,conicequidistant:f,equalEarth:E,equirectangular:q,gnomonic:j,identity:z,mercator:A,mollweide:C,naturalEarth1:v,orthographic:b,stereographic:P,transversemercator:w};for(const e in t)s(e,t[e]);export{H as getProjectionPath,s as projection,c as projectionProperties};export default null;
