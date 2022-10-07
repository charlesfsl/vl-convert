import{canvas as at}from"/-/vega-canvas@v1.2.6-LOIUlXMv11fR2KwlkAGG/dist=es2020,mode=imports,min/optimized/vega-canvas.js";import{Transform as $,rederive as ot}from"/-/vega-dataflow@v5.7.4-DrCzG6Luqf74SfPN5Hxw/dist=es2020,mode=imports,min/optimized/vega-dataflow.js";import{textMetrics as Y,Marks as ft}from"/-/vega-scenegraph@v4.9.4-Rfd7OGmaS9T7w10Fz4Yx/dist=es2020,mode=imports,min/optimized/vega-scenegraph.js";import{inherits as lt,isFunction as ut,error as st,array as tt}from"/-/vega-util@v1.17.0-uRskU0IBL2vWCP4Va8OC/dist=es2020,mode=imports,min/optimized/vega-util.js";const ct=4278190080,mt=268435456,dt=.0625;function ht(t,a){const o=t.bitmap();return(a||[]).forEach(f=>o.set(t(f.boundary[0]),t(f.boundary[3]))),[o,void 0]}function gt(t,a,o,f){const r=t.width,i=t.height,e=o||f,u=at(r,i).getContext("2d");a.forEach(y=>et(u,y,e));const v=new Uint32Array(u.getImageData(0,0,r,i).data.buffer),c=t.bitmap(),m=e&&t.bitmap();let l,n,s,g,d;for(n=0;n<i;++n)for(l=0;l<r;++l)d=v[n*r+l]&ct,d&&(s=t(l),g=t(n),f||c.set(s,g),e&&d^mt&&m.set(s,g));return[c,m]}function et(t,a,o){if(!a.length)return;const f=a[0].mark.marktype;f==="group"?a.forEach(r=>{r.items.forEach(i=>et(t,i.items,o))}):ft[f].draw(t,{items:o?a.map(yt):a})}function yt(t){const a=ot(t,{});return a.stroke&&(a.strokeOpacity=1),a.fill&&(a.fillOpacity=dt,a.stroke="#000",a.strokeOpacity=1,a.strokeWidth=2),a}const D=5,M=31,H=32,F=new Uint32Array(H+1),B=new Uint32Array(H+1);B[0]=0,F[0]=~B[0];for(let t=1;t<=H;++t)B[t]=B[t-1]<<1|1,F[t]=~B[t];function xt(t,a){const o=new Uint32Array(~~((t*a+H)/H));function f(i,e){o[i]|=e}function r(i,e){o[i]&=e}return{array:o,get:(i,e)=>{const u=e*t+i;return o[u>>>D]&1<<(u&M)},set:(i,e)=>{const u=e*t+i;f(u>>>D,1<<(u&M))},clear:(i,e)=>{const u=e*t+i;r(u>>>D,~(1<<(u&M)))},getRange:(i,e,u,v)=>{let c=v,m,l,n,s;for(;c>=e;--c)if(m=c*t+i,l=c*t+u,n=m>>>D,s=l>>>D,n===s){if(o[n]&F[m&M]&B[(l&M)+1])return!0}else{if(o[n]&F[m&M])return!0;if(o[s]&B[(l&M)+1])return!0;for(let g=n+1;g<s;++g)if(o[g])return!0}return!1},setRange:(i,e,u,v)=>{let c,m,l,n,s;for(;e<=v;++e)if(c=e*t+i,m=e*t+u,l=c>>>D,n=m>>>D,l===n)f(l,F[c&M]&B[(m&M)+1]);else for(f(l,F[c&M]),f(n,B[(m&M)+1]),s=l+1;s<n;++s)f(s,4294967295)},clearRange:(i,e,u,v)=>{let c,m,l,n,s;for(;e<=v;++e)if(c=e*t+i,m=e*t+u,l=c>>>D,n=m>>>D,l===n)r(l,B[c&M]|F[(m&M)+1]);else for(r(l,B[c&M]),r(n,F[(m&M)+1]),s=l+1;s<n;++s)r(s,0)},outOfBounds:(i,e,u,v)=>i<0||e<0||v>=a||u>=t}}function pt(t,a,o){const f=Math.max(1,Math.sqrt(t*a/1e6)),r=~~((t+2*o+f)/f),i=~~((a+2*o+f)/f),e=u=>~~((u+o)/f);return e.invert=u=>u*f-o,e.bitmap=()=>xt(r,i),e.ratio=f,e.padding=o,e.width=t,e.height=a,e}function bt(t,a,o,f){const r=t.width,i=t.height;return function(e){const u=e.datum.datum.items[f].items,v=u.length,c=e.datum.fontSize,m=Y.width(e.datum,e.datum.text);let l=0,n,s,g,d,y,R,w;for(let h=0;h<v;++h)n=u[h].x,g=u[h].y,s=u[h].x2===void 0?n:u[h].x2,d=u[h].y2===void 0?g:u[h].y2,y=(n+s)/2,R=(g+d)/2,w=Math.abs(s-n+d-g),w>=l&&(l=w,e.x=y,e.y=R);return y=m/2,R=c/2,n=e.x-y,s=e.x+y,g=e.y-R,d=e.y+R,e.align="center",n<0&&s<=r?e.align="left":0<=n&&r<s&&(e.align="right"),e.baseline="middle",g<0&&d<=i?e.baseline="top":0<=g&&i<d&&(e.baseline="bottom"),!0}}function vt(t,a,o,f,r,i){let e=o/2;return t-e<0||t+e>r||a-(e=f/2)<0||a+e>i}function wt(){return!1}function At(t,a,o,f,r,i,e,u){const v=r*i/(f*2),c=t(a-v),m=t(a+v),l=t(o-(i=i/2)),n=t(o+i);return e.outOfBounds(c,l,m,n)||e.getRange(c,l,m,n)||u&&u.getRange(c,l,m,n)}function Rt(t,a,o,f,r,i,e,u){const v=r*i/(f*2);let c=t(a-v),m=t(a+v),l=t(o-(i=i/2)),n=t(o+i);return c=c>0?c:0,l=l>0?l:0,m=m<t.width?m:t.width-1,n=n<t.height?n:t.height-1,e.getRange(c,l,m,n)||u&&u.getRange(c,l,m,n)}function nt(t){return t?[Rt,wt]:[At,vt]}function It(t,a,o,f,r){const i=t.width,e=t.height,[u,v]=nt(r),c=a[0],m=a[1];function l(n,s,g,d,y){const R=t.invert(n),w=t.invert(s);let h=g,z=e,I;if(!v(R,w,d,y,i,e)&&!u(t,R,w,y,d,h,c,m)&&!u(t,R,w,y,d,y,c,null)){for(;z-h>=1;)I=(h+z)/2,u(t,R,w,y,d,I,c,m)?z=I:h=I;if(h>g)return[R,w,h,!0]}}return function(n){const s=n.datum.datum.items[f].items,g=s.length,d=n.datum.fontSize,y=Y.width(n.datum,n.datum.text);let R=o?d:0,w=!1,h=!1,z=0,I,E,S,A,x,p,b,O,k,T,N,P,G,L,C,U,j;for(let W=0;W<g;++W){for(I=s[W].x,S=s[W].y,E=s[W].x2===void 0?I:s[W].x2,A=s[W].y2===void 0?S:s[W].y2,I>E&&(j=I,I=E,E=j),S>A&&(j=S,S=A,A=j),k=t(I),N=t(E),T=~~((k+N)/2),P=t(S),L=t(A),G=~~((P+L)/2),b=T;b>=k;--b)for(O=G;O>=P;--O)U=l(b,O,R,y,d),U&&([n.x,n.y,R,w]=U);for(b=T;b<=N;++b)for(O=G;O<=L;++O)U=l(b,O,R,y,d),U&&([n.x,n.y,R,w]=U);!w&&!o&&(C=Math.abs(E-I+A-S),x=(I+E)/2,p=(S+A)/2,C>=z&&!v(x,p,y,d,i,e)&&!u(t,x,p,d,y,d,c,null)&&(z=C,n.x=x,n.y=p,h=!0))}return w||h?(x=y/2,p=d/2,c.setRange(t(n.x-x),t(n.y-p),t(n.x+x),t(n.y+p)),n.align="center",n.baseline="middle",!0):!1}}const Ot=[-1,-1,1,1],Et=[-1,1,-1,1];function St(t,a,o,f,r){const i=t.width,e=t.height,[u,v]=nt(r),c=a[0],m=a[1],l=t.bitmap();return function(n){const s=n.datum.datum.items[f].items,g=s.length,d=n.datum.fontSize,y=Y.width(n.datum,n.datum.text),R=[];let w=o?d:0,h=!1,z=!1,I=0,E,S,A,x,p,b,O,k,T,N,P,G;for(let L=0;L<g;++L){for(E=s[L].x,A=s[L].y,S=s[L].x2===void 0?E:s[L].x2,x=s[L].y2===void 0?A:s[L].y2,R.push([t((E+S)/2),t((A+x)/2)]);R.length;){if([O,k]=R.pop(),c.get(O,k)||m.get(O,k)||l.get(O,k))continue;l.set(O,k);for(let C=0;C<4;++C)p=O+Ot[C],b=k+Et[C],l.outOfBounds(p,b,p,b)||R.push([p,b]);if(p=t.invert(O),b=t.invert(k),T=w,N=e,!v(p,b,y,d,i,e)&&!u(t,p,b,d,y,T,c,m)&&!u(t,p,b,d,y,d,c,null)){for(;N-T>=1;)P=(T+N)/2,u(t,p,b,d,y,P,c,m)?N=P:T=P;T>w&&(n.x=p,n.y=b,w=T,h=!0)}}!h&&!o&&(G=Math.abs(S-E+x-A),p=(E+S)/2,b=(A+x)/2,G>=I&&!v(p,b,y,d,i,e)&&!u(t,p,b,d,y,d,c,null)&&(I=G,n.x=p,n.y=b,z=!0))}return h||z?(p=y/2,b=d/2,c.setRange(t(n.x-p),t(n.y-b),t(n.x+p),t(n.y+b)),n.align="center",n.baseline="middle",!0):!1}}const kt=["right","center","left"],zt=["bottom","middle","top"];function Mt(t,a,o,f,r){const i=t.width,e=t.height,u=a[0],v=a[1],c=f.length;return function(m){const l=m.boundary,n=m.datum.fontSize;if(!r&&(l[2]<0||l[5]<0||l[0]>i||l[3]>e))return!1;let s=0,g,d,y,R,w,h,z,I,E,S,A,x,p,b,O;for(let k=0;k<c;++k){if(g=(o[k]&3)-1,d=(o[k]>>>2&3)-1,y=g===0&&d===0||f[k]<0,R=g&&d?Math.SQRT1_2:1,w=f[k]<0?-1:1,h=l[1+g]+f[k]*g*R,A=l[4+d]+w*n*d/2+f[k]*d*R,I=A-n/2,E=A+n/2,x=t(h),b=t(I),O=t(E),r&&(x=x<0?0:x,b=b<0?0:b,O=O>=t.height?t.height-1:O),!s)if(it(x,x,b,O,u,v,h,h,I,E,l,y))s=Y.width(m.datum,m.datum.text);else continue;if(S=h+w*s*g/2,h=S-s/2,z=S+s/2,x=t(h),p=t(z),r&&(x=x<0?0:x,p=p>=t.width?t.width-1:p),it(x,p,b,O,u,v,h,z,I,E,l,y))return m.x=g?g*w<0?z:h:S,m.y=d?d*w<0?E:I:A,m.align=kt[g*w+1],m.baseline=zt[d*w+1],u.setRange(x,b,p,O),!0}return!1}}function it(t,a,o,f,r,i,e,u,v,c,m,l){return!(r.outOfBounds(t,o,a,f)||(l&&i?i.getRange(t,o,a,f)||!Lt(e,v,u,c,m):r.getRange(t,o,a,f)))}function Lt(t,a,o,f,r){return r[0]<=t&&o<=r[2]&&r[3]<=a&&f<=r[5]}const q=0,K=4,V=8,X=0,Q=1,Z=2,Tt={"top-left":q+X,top:q+Q,"top-right":q+Z,left:K+X,middle:K+Q,right:K+Z,"bottom-left":V+X,bottom:V+Q,"bottom-right":V+Z},Bt={naive:bt,"reduced-search":It,floodfill:St};function Dt(t,a,o,f,r,i,e,u,v,c,m){if(!t.length)return t;const l=Math.max(f.length,r.length),n=Nt(f,l),s=Pt(r,l),g=Ct(t[0].datum),d=g==="group"&&t[0].datum.items[v].marktype,y=d==="area",R=Wt(g,d,u,v),w=c===null||c===Infinity,h=pt(a[0],a[1],w?0:c),z=y&&m==="naive",I=t.map(A=>({datum:A,opacity:0,x:void 0,y:void 0,align:void 0,baseline:void 0,boundary:R(A)}));let E;if(!z){o&&I.sort((x,p)=>o(x.datum,p.datum));let A=!1;for(let x=0;x<s.length&&!A;++x)A=s[x]===5||n[x]<0;g&&(e||y)&&(i=[t.map(x=>x.datum)].concat(i)),E=i.length?gt(h,i,A,y):ht(h,e&&I)}const S=y?Bt[m](h,E,e,v,w):Mt(h,E,s,n,w);return I.forEach(A=>A.opacity=+S(A)),I}function Nt(t,a){const o=new Float64Array(a),f=t.length;for(let r=0;r<f;++r)o[r]=t[r]||0;for(let r=f;r<a;++r)o[r]=o[f-1];return o}function Pt(t,a){const o=new Int8Array(a),f=t.length;for(let r=0;r<f;++r)o[r]|=Tt[t[r]];for(let r=f;r<a;++r)o[r]=o[f-1];return o}function Ct(t){return t&&t.mark&&t.mark.marktype}function Wt(t,a,o,f){const r=i=>[i.x,i.x,i.x,i.y,i.y,i.y];return t?t==="line"||t==="area"?i=>r(i.datum):a==="line"?i=>{const e=i.datum.items[f].items;return r(e.length?e[o==="start"?0:e.length-1]:{x:NaN,y:NaN})}:i=>{const e=i.datum.bounds;return[e.x1,(e.x1+e.x2)/2,e.x2,e.y1,(e.y1+e.y2)/2,e.y2]}:r}const _=["x","y","opacity","align","baseline"],rt=["top-left","left","bottom-left","top","bottom","top-right","right","bottom-right"];function J(t){$.call(this,null,t)}J.Definition={type:"Label",metadata:{modifies:!0},params:[{name:"size",type:"number",array:!0,length:2,required:!0},{name:"sort",type:"compare"},{name:"anchor",type:"string",array:!0,default:rt},{name:"offset",type:"number",array:!0,default:[1]},{name:"padding",type:"number",default:0,null:!0},{name:"lineAnchor",type:"string",values:["start","end"],default:"end"},{name:"markIndex",type:"number",default:0},{name:"avoidBaseMark",type:"boolean",default:!0},{name:"avoidMarks",type:"data",array:!0},{name:"method",type:"string",default:"naive"},{name:"as",type:"string",array:!0,length:_.length,default:_}]},lt(J,$,{transform(t,a){function o(i){const e=t[i];return ut(e)&&a.modified(e.fields)}const f=t.modified();if(!(f||a.changed(a.ADD_REM)||o("sort")))return;(!t.size||t.size.length!==2)&&st("Size parameter should be specified as a [width, height] array.");const r=t.as||_;return Dt(a.materialize(a.SOURCE).source||[],t.size,t.sort,tt(t.offset==null?1:t.offset),tt(t.anchor||rt),t.avoidMarks||[],t.avoidBaseMark!==!1,t.lineAnchor||"end",t.markIndex||0,t.padding===void 0?0:t.padding,t.method||"naive").forEach(i=>{const e=i.datum;e[r[0]]=i.x,e[r[1]]=i.y,e[r[2]]=i.opacity,e[r[3]]=i.align,e[r[4]]=i.baseline}),a.reflow(f).modifies(r)}});export{J as label};export default null;