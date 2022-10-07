import{hasOwnProperty as Se,peek as M,constant as Oe,isNumber as Be,span as qe,isObject as xe,isString as Y,error as Fe,toSet as Ge,array as Ie,toNumber as ke,isArray as Le}from"/-/vega-util@v1.17.0-uRskU0IBL2vWCP4Va8OC/dist=es2020,mode=imports,min/optimized/vega-util.js";import{bisectRight as N,range as ze,bisect as je}from"/-/d3-array@v3.1.1-Ibshj34oOmCw8da1RLSW/dist=es2020,mode=imports,min/optimized/d3-array.js";import{scaleIdentity as Ce,scaleLinear as De,scaleLog as Pe,scalePow as Ae,scaleSqrt as Te,scaleSymlog as Qe,scaleTime as Ue,scaleUtc as Ve,scaleSequential as K,scaleSequentialLog as Ee,scaleSequentialPow as Ye,scaleSequentialSqrt as Ne,scaleSequentialSymlog as Ke,scaleDiverging as We,scaleDivergingLog as He,scaleDivergingPow as Xe,scaleDivergingSqrt as Ze,scaleDivergingSymlog as $e,scaleQuantile as Je,scaleQuantize as _e,scaleThreshold as ea,scaleOrdinal as W,tickFormat as aa}from"/-/d3-scale@v4.0.2-qUv67mnQQKwRMEsPRKcO/dist=es2020,mode=imports,min/optimized/d3-scale.js";export{scaleImplicit}from"/-/d3-scale@v4.0.2-qUv67mnQQKwRMEsPRKcO/dist=es2020,mode=imports,min/optimized/d3-scale.js";import*as fa from"/-/d3-interpolate@v3.0.1-i9AsUdFHwyaukRBWNe8d/dist=es2020,mode=imports,min/optimized/d3-interpolate.js";import{piecewise as na}from"/-/d3-interpolate@v3.0.1-i9AsUdFHwyaukRBWNe8d/dist=es2020,mode=imports,min/optimized/d3-interpolate.js";import{timeInterval as ca,utcInterval as ta}from"/-/vega-time@v2.1.0-LVGYZzfDGZrYRhsP6b35/dist=es2020,mode=imports,min/optimized/vega-time.js";function H(e,a,n){const f=e-a+n*2;return e?f>0?f:1:0}const X="identity",z="linear",v="log",j="pow",C="sqrt",D="symlog",R="time",S="utc",O="sequential",q="diverging",x="quantile",P="quantize",A="threshold",Z="ordinal",$="point",J="band",_="bin-ordinal",s="continuous",G="discrete",I="discretizing",g="interpolating",Q="temporal";function da(e){return function(a){let n=a[0],f=a[1],c;return f<n&&(c=n,n=f,f=c),[e.invert(n),e.invert(f)]}}function ra(e){return function(a){const n=e.range();let f=a[0],c=a[1],t=-1,d,r,o,l;for(c<f&&(r=f,f=c,c=r),o=0,l=n.length;o<l;++o)n[o]>=f&&n[o]<=c&&(t<0&&(t=o),d=o);return t<0?void 0:(f=e.invertExtent(n[t]),c=e.invertExtent(n[d]),[f[0]===void 0?f[1]:f[0],c[1]===void 0?c[0]:c[1]])}}function U(){const e=W().unknown(void 0),a=e.domain,n=e.range;let f=[0,1],c,t,d=!1,r=0,o=0,l=.5;delete e.unknown;function m(){const b=a().length,p=f[1]<f[0],w=f[1-p],k=H(b,r,o);let u=f[p-0];c=(w-u)/(k||1),d&&(c=Math.floor(c)),u+=(w-u-c*(b-r))*l,t=c*(1-r),d&&(u=Math.round(u),t=Math.round(t));const h=ze(b).map(y=>u+c*y);return n(p?h.reverse():h)}return e.domain=function(b){return arguments.length?(a(b),m()):a()},e.range=function(b){return arguments.length?(f=[+b[0],+b[1]],m()):f.slice()},e.rangeRound=function(b){return f=[+b[0],+b[1]],d=!0,m()},e.bandwidth=function(){return t},e.step=function(){return c},e.round=function(b){return arguments.length?(d=!!b,m()):d},e.padding=function(b){return arguments.length?(o=Math.max(0,Math.min(1,b)),r=o,m()):r},e.paddingInner=function(b){return arguments.length?(r=Math.max(0,Math.min(1,b)),m()):r},e.paddingOuter=function(b){return arguments.length?(o=Math.max(0,Math.min(1,b)),m()):o},e.align=function(b){return arguments.length?(l=Math.max(0,Math.min(1,b)),m()):l},e.invertRange=function(b){if(b[0]==null||b[1]==null)return;const p=f[1]<f[0],w=p?n().reverse():n(),k=w.length-1;let u=+b[0],h=+b[1],y,F,L;return u!==u||h!==h?void 0:(h<u&&(L=u,u=h,h=L),h<w[0]||u>f[1-p]?void 0:(y=Math.max(0,N(w,u)-1),F=u===h?y:N(w,h)-1,u-w[y]>t+1e-10&&++y,p&&(L=y,y=k-F,F=k-L),y>F?void 0:a().slice(y,F+1)))},e.invert=function(b){const p=e.invertRange([b,b]);return p&&p[0]},e.copy=function(){return U().domain(a()).range(f).round(d).paddingInner(r).paddingOuter(o).align(l)},m()}function ee(e){const a=e.copy;return e.padding=e.paddingOuter,delete e.paddingInner,e.copy=function(){return ee(a())},e}function ba(){return ee(U().paddingInner(1))}var ia=Array.prototype.map;function oa(e){return ia.call(e,ke)}const sa=Array.prototype.slice;function ae(){let e=[],a=[];function n(f){return f==null||f!==f?void 0:a[(je(e,f)-1)%a.length]}return n.domain=function(f){return arguments.length?(e=oa(f),n):e.slice()},n.range=function(f){return arguments.length?(a=sa.call(f),n):a.slice()},n.tickFormat=function(f,c){return aa(e[0],M(e),f??10,c)},n.copy=function(){return ae().domain(n.domain()).range(n.range())},n}const T={};function ua(e,a,n){const f=function(){const t=a();return t.invertRange||(t.invertRange=t.invert?da(t):t.invertExtent?ra(t):void 0),t.type=e,t};return f.metadata=Ge(Ie(n)),f}function i(e,a,n){return arguments.length>1?(T[e]=ua(e,a,n),this):fe(e)?T[e]:void 0}i(X,Ce),i(z,De,s),i(v,Pe,[s,v]),i(j,Ae,s),i(C,Te,s),i(D,Qe,s),i(R,Ue,[s,Q]),i(S,Ve,[s,Q]),i(O,K,[s,g]),i("".concat(O,"-").concat(z),K,[s,g]),i("".concat(O,"-").concat(v),Ee,[s,g,v]),i("".concat(O,"-").concat(j),Ye,[s,g]),i("".concat(O,"-").concat(C),Ne,[s,g]),i("".concat(O,"-").concat(D),Ke,[s,g]),i("".concat(q,"-").concat(z),We,[s,g]),i("".concat(q,"-").concat(v),He,[s,g,v]),i("".concat(q,"-").concat(j),Xe,[s,g]),i("".concat(q,"-").concat(C),Ze,[s,g]),i("".concat(q,"-").concat(D),$e,[s,g]),i(x,Je,[I,x]),i(P,_e,I),i(A,ea,I),i(_,ae,[G,I]),i(Z,W,G),i(J,U,G),i($,ba,G);function fe(e){return Se(T,e)}function B(e,a){const n=T[e];return n&&n.metadata[a]}function la(e){return B(e,s)}function ne(e){return B(e,G)}function ce(e){return B(e,I)}function te(e){return B(e,v)}function de(e){return B(e,Q)}function ma(e){return B(e,g)}function ga(e){return B(e,x)}const pa=["clamp","base","constant","exponent"];function ha(e,a){const n=a[0],f=M(a)-n;return function(c){return e(n+c*f)}}function re(e,a,n){return na(be(a||"rgb",n),e)}function ya(e,a){const n=new Array(a),f=a+1;for(let c=0;c<a;)n[c]=e(++c/f);return n}function va(e){const a=e.type,n=e.copy();return n.type=a,n}function wa(e,a,n){const f=n-a;let c,t,d;return!f||!Number.isFinite(f)?Oe(.5):(c=(t=e.type).indexOf("-"),t=c<0?t:t.slice(c+1),d=i(t)().domain([a,n]).range([0,1]),pa.forEach(r=>e[r]?d[r](e[r]()):0),d)}function be(e,a){const n=fa[Ma(e)];return a!=null&&n&&n.gamma?n.gamma(a):n}function Ma(e){return"interpolate"+e.toLowerCase().split("-").map(a=>a[0].toUpperCase()+a.slice(1)).join("")}const Ra={blues:"cfe1f2bed8eca8cee58fc1de74b2d75ba3cf4592c63181bd206fb2125ca40a4a90",greens:"d3eecdc0e6baabdda594d3917bc77d60ba6c46ab5e329a512089430e7735036429",greys:"e2e2e2d4d4d4c4c4c4b1b1b19d9d9d8888887575756262624d4d4d3535351e1e1e",oranges:"fdd8b3fdc998fdb87bfda55efc9244f87f2cf06b18e4580bd14904b93d029f3303",purples:"e2e1efd4d4e8c4c5e0b4b3d6a3a0cc928ec3827cb97566ae684ea25c3696501f8c",reds:"fdc9b4fcb49afc9e80fc8767fa7051f6573fec3f2fdc2a25c81b1db21218970b13",blueGreen:"d5efedc1e8e0a7ddd18bd2be70c6a958ba9144ad77319c5d2089460e7736036429",bluePurple:"ccddecbad0e4a8c2dd9ab0d4919cc98d85be8b6db28a55a6873c99822287730f71",greenBlue:"d3eecec5e8c3b1e1bb9bd8bb82cec269c2ca51b2cd3c9fc7288abd1675b10b60a1",orangeRed:"fddcaffdcf9bfdc18afdad77fb9562f67d53ee6545e24932d32d1ebf130da70403",purpleBlue:"dbdaebc8cee4b1c3de97b7d87bacd15b9fc93a90c01e7fb70b70ab056199045281",purpleBlueGreen:"dbd8eac8cee4b0c3de93b7d872acd1549fc83892bb1c88a3097f8702736b016353",purpleRed:"dcc9e2d3b3d7ce9eccd186c0da6bb2e14da0e23189d91e6fc61159ab07498f023a",redPurple:"fccfccfcbec0faa9b8f98faff571a5ec539ddb3695c41b8aa908808d0179700174",yellowGreen:"e4f4acd1eca0b9e2949ed68880c97c62bb6e47aa5e3297502083440e723b036034",yellowOrangeBrown:"feeaa1fedd84fecc63feb746fca031f68921eb7215db5e0bc54c05ab3d038f3204",yellowOrangeRed:"fee087fed16ffebd59fea849fd903efc7335f9522bee3423de1b20ca0b22af0225",blueOrange:"134b852f78b35da2cb9dcae1d2e5eff2f0ebfce0bafbbf74e8932fc5690d994a07",brownBlueGreen:"704108a0651ac79548e3c78af3e6c6eef1eac9e9e48ed1c74da79e187a72025147",purpleGreen:"5b1667834792a67fb6c9aed3e6d6e8eff0efd9efd5aedda971bb75368e490e5e29",purpleOrange:"4114696647968f83b7b9b4d6dadbebf3eeeafce0bafbbf74e8932fc5690d994a07",redBlue:"8c0d25bf363adf745ef4ae91fbdbc9f2efeed2e5ef9dcae15da2cb2f78b3134b85",redGrey:"8c0d25bf363adf745ef4ae91fcdccbfaf4f1e2e2e2c0c0c0969696646464343434",yellowGreenBlue:"eff9bddbf1b4bde5b594d5b969c5be45b4c22c9ec02182b82163aa23479c1c3185",redYellowBlue:"a50026d4322cf16e43fcac64fedd90faf8c1dcf1ecabd6e875abd04a74b4313695",redYellowGreen:"a50026d4322cf16e43fcac63fedd8df9f7aed7ee8ea4d86e64bc6122964f006837",pinkYellowGreen:"8e0152c0267edd72adf0b3d6faddedf5f3efe1f2cab6de8780bb474f9125276419",spectral:"9e0142d13c4bf0704afcac63fedd8dfbf8b0e0f3a1a9dda269bda94288b55e4fa2",viridis:"440154470e61481a6c482575472f7d443a834144873d4e8a39568c35608d31688e2d708e2a788e27818e23888e21918d1f988b1fa08822a8842ab07f35b77943bf7154c56866cc5d7ad1518fd744a5db36bcdf27d2e21be9e51afde725",magma:"0000040404130b0924150e3720114b2c11603b0f704a107957157e651a80721f817f24828c29819a2e80a8327db6377ac43c75d1426fde4968e95462f1605df76f5cfa7f5efc8f65fe9f6dfeaf78febf84fece91fddea0fcedaffcfdbf",inferno:"0000040403130c0826170c3b240c4f330a5f420a68500d6c5d126e6b176e781c6d86216b932667a12b62ae305cbb3755c73e4cd24644dd513ae65c30ed6925f3771af8850ffb9506fca50afcb519fac62df6d645f2e661f3f484fcffa4",plasma:"0d088723069033059742039d5002a25d01a66a00a87801a88405a7900da49c179ea72198b12a90ba3488c33d80cb4779d35171da5a69e16462e76e5bed7953f2834cf68f44fa9a3dfca636fdb32ffec029fcce25f9dc24f5ea27f0f921",cividis:"00205100235800265d002961012b65042e670831690d346b11366c16396d1c3c6e213f6e26426e2c456e31476e374a6e3c4d6e42506e47536d4c566d51586e555b6e5a5e6e5e616e62646f66676f6a6a706e6d717270717573727976737c79747f7c75827f758682768985778c8877908b78938e789691789a94789e9778a19b78a59e77a9a177aea575b2a874b6ab73bbaf71c0b26fc5b66dc9b96acebd68d3c065d8c462ddc85fe2cb5ce7cf58ebd355f0d652f3da4ff7de4cfae249fce647",rainbow:"6e40aa883eb1a43db3bf3cafd83fa4ee4395fe4b83ff576eff6659ff7847ff8c38f3a130e2b72fcfcc36bee044aff05b8ff4576ff65b52f6673af27828ea8d1ddfa319d0b81cbecb23abd82f96e03d82e14c6edb5a5dd0664dbf6e40aa",sinebow:"ff4040fc582af47218e78d0bd5a703bfbf00a7d5038de70b72f41858fc2a40ff402afc5818f4720be78d03d5a700bfbf03a7d50b8de71872f42a58fc4040ff582afc7218f48d0be7a703d5bf00bfd503a7e70b8df41872fc2a58ff4040",turbo:"23171b32204a3e2a71453493493eae4b49c54a53d7485ee44569ee4074f53c7ff8378af93295f72e9ff42ba9ef28b3e926bce125c5d925cdcf27d5c629dcbc2de3b232e9a738ee9d3ff39347f68950f9805afc7765fd6e70fe667cfd5e88fc5795fb51a1f84badf545b9f140c5ec3cd0e637dae034e4d931ecd12ef4c92bfac029ffb626ffad24ffa223ff9821ff8d1fff821dff771cfd6c1af76118f05616e84b14df4111d5380fcb2f0dc0260ab61f07ac1805a313029b0f00950c00910b00",browns:"eedbbdecca96e9b97ae4a865dc9856d18954c7784cc0673fb85536ad44339f3632",tealBlues:"bce4d89dd3d181c3cb65b3c245a2b9368fae347da0306a932c5985",teals:"bbdfdfa2d4d58ac9c975bcbb61b0af4da5a43799982b8b8c1e7f7f127273006667",warmGreys:"dcd4d0cec5c1c0b8b4b3aaa7a59c9998908c8b827f7e7673726866665c5a59504e",goldGreen:"f4d166d5ca60b6c35c98bb597cb25760a6564b9c533f8f4f33834a257740146c36",goldOrange:"f4d166f8be5cf8aa4cf5983bf3852aef701be2621fd65322c54923b142239e3a26",goldRed:"f4d166f6be59f9aa51fc964ef6834bee734ae56249db5247cf4244c43141b71d3e",lightGreyRed:"efe9e6e1dad7d5cbc8c8bdb9bbaea9cd967ddc7b43e15f19df4011dc000b",lightGreyTeal:"e4eaead6dcddc8ced2b7c2c7a6b4bc64b0bf22a6c32295c11f85be1876bc",lightMulti:"e0f1f2c4e9d0b0de9fd0e181f6e072f6c053f3993ef77440ef4a3c",lightOrange:"f2e7daf7d5baf9c499fab184fa9c73f68967ef7860e8645bde515bd43d5b",lightTealBlue:"e3e9e0c0dccf9aceca7abfc859afc0389fb9328dad2f7ca0276b95255988",darkBlue:"3232322d46681a5c930074af008cbf05a7ce25c0dd38daed50f3faffffff",darkGold:"3c3c3c584b37725e348c7631ae8b2bcfa424ecc31ef9de30fff184ffffff",darkGreen:"3a3a3a215748006f4d048942489e4276b340a6c63dd2d836ffeb2cffffaa",darkMulti:"3737371f5287197d8c29a86995ce3fffe800ffffff",darkRed:"3434347036339e3c38cc4037e75d1eec8620eeab29f0ce32ffeb2c"},Sa={category10:"1f77b4ff7f0e2ca02cd627289467bd8c564be377c27f7f7fbcbd2217becf",category20:"1f77b4aec7e8ff7f0effbb782ca02c98df8ad62728ff98969467bdc5b0d58c564bc49c94e377c2f7b6d27f7f7fc7c7c7bcbd22dbdb8d17becf9edae5",category20b:"393b795254a36b6ecf9c9ede6379398ca252b5cf6bcedb9c8c6d31bd9e39e7ba52e7cb94843c39ad494ad6616be7969c7b4173a55194ce6dbdde9ed6",category20c:"3182bd6baed69ecae1c6dbefe6550dfd8d3cfdae6bfdd0a231a35474c476a1d99bc7e9c0756bb19e9ac8bcbddcdadaeb636363969696bdbdbdd9d9d9",tableau10:"4c78a8f58518e4575672b7b254a24beeca3bb279a2ff9da69d755dbab0ac",tableau20:"4c78a89ecae9f58518ffbf7954a24b88d27ab79a20f2cf5b43989483bcb6e45756ff9d9879706ebab0acd67195fcbfd2b279a2d6a5c99e765fd8b5a5",accent:"7fc97fbeaed4fdc086ffff99386cb0f0027fbf5b17666666",dark2:"1b9e77d95f027570b3e7298a66a61ee6ab02a6761d666666",paired:"a6cee31f78b4b2df8a33a02cfb9a99e31a1cfdbf6fff7f00cab2d66a3d9affff99b15928",pastel1:"fbb4aeb3cde3ccebc5decbe4fed9a6ffffcce5d8bdfddaecf2f2f2",pastel2:"b3e2cdfdcdaccbd5e8f4cae4e6f5c9fff2aef1e2cccccccc",set1:"e41a1c377eb84daf4a984ea3ff7f00ffff33a65628f781bf999999",set2:"66c2a5fc8d628da0cbe78ac3a6d854ffd92fe5c494b3b3b3",set3:"8dd3c7ffffb3bebadafb807280b1d3fdb462b3de69fccde5d9d9d9bc80bdccebc5ffed6f"};function ie(e){const a=e.length/6|0,n=new Array(a);for(let f=0;f<a;)n[f]="#"+e.slice(f*6,++f*6);return n}function oe(e,a){for(const n in e)ue(n,a(e[n]))}const se={};oe(Sa,ie),oe(Ra,e=>re(ie(e)));function ue(e,a){return e=e&&e.toLowerCase(),arguments.length>1?(se[e]=a,this):se[e]}const le="symbol",me="discrete",Oa="gradient",Ba=e=>Le(e)?e.map(a=>String(a)):String(e),qa=(e,a)=>e[1]-a[1],xa=(e,a)=>a[1]-e[1];function Fa(e,a,n){let f;return Be(a)&&(e.bins&&(a=Math.max(a,e.bins.length)),n!=null&&(a=Math.min(a,Math.floor(qe(e.domain())/n||1)))),xe(a)&&(f=a.step,a=a.interval),Y(a)&&(a=e.type===R?ca(a):e.type==S?ta(a):Fe("Only time and utc scales accept interval strings."),f&&(a=a.every(f))),a}function ge(e,a,n){let f=e.range(),c=f[0],t=M(f),d=qa;if(c>t&&(f=t,t=c,c=f,d=xa),c=Math.floor(c),t=Math.ceil(t),a=a.map(r=>[r,e(r)]).filter(r=>c<=r[1]&&r[1]<=t).sort(d).map(r=>r[0]),n>0&&a.length>1){const r=[a[0],M(a)];for(;a.length>n&&a.length>=3;)a=a.filter((o,l)=>!(l%2));a.length<3&&(a=r)}return a}function V(e,a){return e.bins?ge(e,e.bins):e.ticks?e.ticks(a):e.domain()}function pe(e,a,n,f,c,t){const d=a.type;let r=Ba;if(d===R||c===R)r=e.timeFormat(f);else if(d===S||c===S)r=e.utcFormat(f);else if(te(d)){const o=e.formatFloat(f);if(t||a.bins)r=o;else{const l=he(a,n,!1);r=m=>l(m)?o(m):""}}else if(a.tickFormat){const o=a.domain();r=e.formatSpan(o[0],o[o.length-1],n,f)}else f&&(r=e.format(f));return r}function he(e,a,n){const f=V(e,a),c=e.base(),t=Math.log(c),d=Math.max(1,c*a/f.length),r=o=>{let l=o/Math.pow(c,Math.round(Math.log(o)/t));return l*c<c-.5&&(l*=c),l<=d};return n?f.filter(r):r}const E={[x]:"quantiles",[P]:"thresholds",[A]:"domain"},ye={[x]:"quantiles",[P]:"domain"};function ve(e,a){return e.bins?ka(e.bins):e.type===v?he(e,a,!0):E[e.type]?Ia(e[E[e.type]]()):V(e,a)}function Ga(e,a,n){const f=a[ye[a.type]](),c=f.length;let t=c>1?f[1]-f[0]:f[0],d;for(d=1;d<c;++d)t=Math.min(t,f[d]-f[d-1]);return e.formatSpan(0,t,3*10,n)}function Ia(e){const a=[-Infinity].concat(e);return a.max=Infinity,a}function ka(e){const a=e.slice(0,-1);return a.max=M(e),a}const La=e=>E[e.type]||e.bins;function we(e,a,n,f,c,t,d){const r=ye[a.type]&&t!==R&&t!==S?Ga(e,a,c):pe(e,a,n,c,t,d);return f===le&&La(a)?za(r):f===me?ja(r):Ca(r)}const za=e=>(a,n,f)=>{const c=Me(f[n+1],Me(f.max,Infinity)),t=Re(a,e),d=Re(c,e);return t&&d?t+" \u2013 "+d:d?"< "+d:"\u2265 "+t},Me=(e,a)=>e??a,ja=e=>(a,n)=>n?e(a):null,Ca=e=>a=>e(a),Re=(e,a)=>Number.isFinite(e)?a(e):null;function Da(e){const a=e.domain(),n=a.length-1;let f=+a[0],c=+M(a),t=c-f;if(e.type===A){const d=n?t/n:.1;f-=d,c+=d,t=c-f}return d=>(d-f)/t}function Pa(e,a,n,f){const c=f||a.type;return Y(n)&&de(c)&&(n=n.replace(/%a/g,"%A").replace(/%b/g,"%B")),!n&&c===R?e.timeFormat("%A, %d %B %Y, %X"):!n&&c===S?e.utcFormat("%A, %d %B %Y, %X UTC"):we(e,a,5,null,n,f,!0)}function Aa(e,a,n){n=n||{};const f=Math.max(3,n.maxlen||7),c=Pa(e,a,n.format,n.formatType);if(ce(a.type)){const t=ve(a).slice(1).map(c),d=t.length;return"".concat(d," boundar").concat(d===1?"y":"ies",": ").concat(t.join(", "))}else if(ne(a.type)){const t=a.domain(),d=t.length,r=d>f?t.slice(0,f-2).map(c).join(", ")+", ending with "+t.slice(-1).map(c):t.map(c).join(", ");return"".concat(d," value").concat(d===1?"":"s",": ").concat(r)}else{const t=a.domain();return"values from ".concat(c(t[0])," to ").concat(c(M(t)))}}export{J as Band,_ as BinOrdinal,me as DiscreteLegend,q as Diverging,Oa as GradientLegend,X as Identity,z as Linear,v as Log,Z as Ordinal,$ as Point,j as Pow,x as Quantile,P as Quantize,O as Sequential,C as Sqrt,le as SymbolLegend,D as Symlog,A as Threshold,R as Time,S as UTC,H as bandSpace,Aa as domainCaption,be as interpolate,re as interpolateColors,ha as interpolateRange,la as isContinuous,ne as isDiscrete,ce as isDiscretizing,ma as isInterpolating,te as isLogarithmic,ga as isQuantile,de as isTemporal,fe as isValidScaleType,we as labelFormat,Da as labelFraction,ve as labelValues,ya as quantizeInterpolator,i as scale,va as scaleCopy,wa as scaleFraction,ue as scheme,Fa as tickCount,pe as tickFormat,V as tickValues,ge as validTicks};export default null;