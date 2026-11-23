var e=Object.defineProperty,t=(t,n)=>{let r={};for(var i in t)e(r,i,{get:t[i],enumerable:!0});return n||e(r,Symbol.toStringTag,{value:`Module`}),r};function n(){return{progress(){},ready(){},state(){}}}var r={LEFT:0,MIDDLE:1,RIGHT:2,ROTATE:0,DOLLY:1,PAN:2},i={ROTATE:0,PAN:1,DOLLY_PAN:2,DOLLY_ROTATE:3},a=`attached`,o=`detached`,s=1e3,c=1001,l=1002,u=1003,d=1004,f=1004,p=1005,m=1005,h=1006,g=1007,_=1007,v=1008,y=1008,b=1009,x=1010,S=1011,C=1012,w=1013,T=1014,E=1015,D=1016,O=1017,k=1018,A=1020,j=35902,M=35899,N=1021,P=1022,F=1023,I=1026,ee=1027,te=1028,ne=1029,re=1030,ie=1031,ae=1032,oe=1033,L=33776,se=33777,ce=33778,le=33779,ue=35840,de=35841,fe=35842,pe=35843,me=36196,he=37492,R=37496,ge=37488,_e=37489,ve=37490,ye=37491,be=37808,xe=37809,Se=37810,Ce=37811,we=37812,Te=37813,Ee=37814,De=37815,Oe=37816,ke=37817,z=37818,Ae=37819,je=37820,B=37821,V=36492,Me=36494,H=36495,Ne=36283,Pe=36284,Fe=36285,Ie=36286,Le=2200,Re=2201,ze=2202,Be=2300,Ve=2301,He=2302,Ue=2303,We=2400,Ge=2401,Ke=2402,qe=2500,Je=2501,Ye=3200,Xe=3201,Ze=3202,Qe=3203,$e=`srgb`,et=`srgb-linear`,tt=`linear`,nt=`srgb`,rt=7680,it=7681,at=7682,ot=7683,st=34055,ct=34056,lt=5386,ut=35044,dt=35048,ft=35040,pt=35045,mt=35049,ht=35041,gt=35046,_t=35050,vt=35042,yt=`300 es`,bt=2e3,xt=2001,St={COMPUTE:`compute`,RENDER:`render`},Ct={PERSPECTIVE:`perspective`,LINEAR:`linear`,FLAT:`flat`},wt={NORMAL:`normal`,CENTROID:`centroid`,SAMPLE:`sample`,FIRST:`first`,EITHER:`either`},Tt={TEXTURE_COMPARE:`depthTextureCompare`},Et={NONE:0,SHARED:1,FULL:2};function Dt(e){for(let t=e.length-1;t>=0;--t)if(e[t]>=65535)return!0;return!1}var Ot={Int8Array,Uint8Array,Uint8ClampedArray,Int16Array,Uint16Array,Int32Array,Uint32Array,Float32Array,Float64Array};function kt(e,t){return new Ot[e](t)}function At(e){return ArrayBuffer.isView(e)&&!(e instanceof DataView)}function jt(e){return document.createElementNS(`http://www.w3.org/1999/xhtml`,e)}function Mt(){let e=jt(`canvas`);return e.style.display=`block`,e}var Nt={},Pt=null;function Ft(e){Pt=e}function It(){return Pt}function Lt(...e){let t=`THREE.`+e.shift();Pt?Pt(`log`,t,...e):console.log(t,...e)}function Rt(e){let t=e[0];if(typeof t==`string`&&t.startsWith(`TSL:`)){let t=e[1];t&&t.isStackTrace?e[0]+=` `+t.getLocation():e[1]=`Stack trace not available. Enable "THREE.Node.captureStackTrace" to capture stack traces.`}return e}function U(...e){e=Rt(e);let t=`THREE.`+e.shift();if(Pt)Pt(`warn`,t,...e);else{let n=e[0];n&&n.isStackTrace?console.warn(n.getError(t)):console.warn(t,...e)}}function W(...e){e=Rt(e);let t=`THREE.`+e.shift();if(Pt)Pt(`error`,t,...e);else{let n=e[0];n&&n.isStackTrace?console.error(n.getError(t)):console.error(t,...e)}}function zt(...e){let t=e.join(` `);t in Nt||(Nt[t]=!0,U(...e))}function Bt(e,t,n){return new Promise(function(r,i){function a(){switch(e.clientWaitSync(t,e.SYNC_FLUSH_COMMANDS_BIT,0)){case e.WAIT_FAILED:i();break;case e.TIMEOUT_EXPIRED:setTimeout(a,n);break;default:r()}}setTimeout(a,n)})}var Vt={0:1,2:6,4:7,3:5,1:0,6:2,7:4,5:3},Ht=class{addEventListener(e,t){this._listeners===void 0&&(this._listeners={});let n=this._listeners;n[e]===void 0&&(n[e]=[]),n[e].indexOf(t)===-1&&n[e].push(t)}hasEventListener(e,t){let n=this._listeners;return n!==void 0&&n[e]!==void 0&&n[e].indexOf(t)!==-1}removeEventListener(e,t){let n=this._listeners;if(n===void 0)return;let r=n[e];if(r!==void 0){let e=r.indexOf(t);e!==-1&&r.splice(e,1)}}dispatchEvent(e){let t=this._listeners;if(t===void 0)return;let n=t[e.type];if(n!==void 0){e.target=this;let t=n.slice(0);for(let n=0,r=t.length;n<r;n++)t[n].call(this,e);e.target=null}}},Ut=`00.01.02.03.04.05.06.07.08.09.0a.0b.0c.0d.0e.0f.10.11.12.13.14.15.16.17.18.19.1a.1b.1c.1d.1e.1f.20.21.22.23.24.25.26.27.28.29.2a.2b.2c.2d.2e.2f.30.31.32.33.34.35.36.37.38.39.3a.3b.3c.3d.3e.3f.40.41.42.43.44.45.46.47.48.49.4a.4b.4c.4d.4e.4f.50.51.52.53.54.55.56.57.58.59.5a.5b.5c.5d.5e.5f.60.61.62.63.64.65.66.67.68.69.6a.6b.6c.6d.6e.6f.70.71.72.73.74.75.76.77.78.79.7a.7b.7c.7d.7e.7f.80.81.82.83.84.85.86.87.88.89.8a.8b.8c.8d.8e.8f.90.91.92.93.94.95.96.97.98.99.9a.9b.9c.9d.9e.9f.a0.a1.a2.a3.a4.a5.a6.a7.a8.a9.aa.ab.ac.ad.ae.af.b0.b1.b2.b3.b4.b5.b6.b7.b8.b9.ba.bb.bc.bd.be.bf.c0.c1.c2.c3.c4.c5.c6.c7.c8.c9.ca.cb.cc.cd.ce.cf.d0.d1.d2.d3.d4.d5.d6.d7.d8.d9.da.db.dc.dd.de.df.e0.e1.e2.e3.e4.e5.e6.e7.e8.e9.ea.eb.ec.ed.ee.ef.f0.f1.f2.f3.f4.f5.f6.f7.f8.f9.fa.fb.fc.fd.fe.ff`.split(`.`),Wt=1234567,Gt=Math.PI/180,Kt=180/Math.PI;function qt(){let e=Math.random()*4294967295|0,t=Math.random()*4294967295|0,n=Math.random()*4294967295|0,r=Math.random()*4294967295|0;return(Ut[e&255]+Ut[e>>8&255]+Ut[e>>16&255]+Ut[e>>24&255]+`-`+Ut[t&255]+Ut[t>>8&255]+`-`+Ut[t>>16&15|64]+Ut[t>>24&255]+`-`+Ut[n&63|128]+Ut[n>>8&255]+`-`+Ut[n>>16&255]+Ut[n>>24&255]+Ut[r&255]+Ut[r>>8&255]+Ut[r>>16&255]+Ut[r>>24&255]).toLowerCase()}function G(e,t,n){return Math.max(t,Math.min(n,e))}function Jt(e,t){return(e%t+t)%t}function Yt(e,t,n,r,i){return r+(e-t)*(i-r)/(n-t)}function Xt(e,t,n){return e===t?0:(n-e)/(t-e)}function Zt(e,t,n){return(1-n)*e+n*t}function Qt(e,t,n,r){return Zt(e,t,1-Math.exp(-n*r))}function $t(e,t=1){return t-Math.abs(Jt(e,t*2)-t)}function en(e,t,n){return e<=t?0:e>=n?1:(e=(e-t)/(n-t),e*e*(3-2*e))}function tn(e,t,n){return e<=t?0:e>=n?1:(e=(e-t)/(n-t),e*e*e*(e*(e*6-15)+10))}function nn(e,t){return e+Math.floor(Math.random()*(t-e+1))}function rn(e,t){return e+Math.random()*(t-e)}function an(e){return e*(.5-Math.random())}function on(e){e!==void 0&&(Wt=e);let t=Wt+=1831565813;return t=Math.imul(t^t>>>15,t|1),t^=t+Math.imul(t^t>>>7,t|61),((t^t>>>14)>>>0)/4294967296}function sn(e){return e*Gt}function cn(e){return e*Kt}function ln(e){return e>0&&Number.isInteger(e)&&2**Math.round(Math.log2(e))===e}function un(e){return 2**Math.ceil(Math.log(e)/Math.LN2)}function dn(e){return 2**Math.floor(Math.log(e)/Math.LN2)}function fn(e,t,n,r,i){let a=Math.cos,o=Math.sin,s=a(n/2),c=o(n/2),l=a((t+r)/2),u=o((t+r)/2),d=a((t-r)/2),f=o((t-r)/2),p=a((r-t)/2),m=o((r-t)/2);switch(i){case`XYX`:e.set(s*u,c*d,c*f,s*l);break;case`YZY`:e.set(c*f,s*u,c*d,s*l);break;case`ZXZ`:e.set(c*d,c*f,s*u,s*l);break;case`XZX`:e.set(s*u,c*m,c*p,s*l);break;case`YXY`:e.set(c*p,s*u,c*m,s*l);break;case`ZYZ`:e.set(c*m,c*p,s*u,s*l);break;default:U(`MathUtils: .setQuaternionFromProperEuler() encountered an unknown order: `+i)}}function pn(e,t){switch(t.constructor){case Float32Array:return e;case Uint32Array:return e/4294967295;case Uint16Array:return e/65535;case Uint8Array:case Uint8ClampedArray:return e/255;case Int32Array:return Math.max(e/2147483647,-1);case Int16Array:return Math.max(e/32767,-1);case Int8Array:return Math.max(e/127,-1);default:throw Error(`THREE.MathUtils: Invalid component type.`)}}function mn(e,t){switch(t.constructor){case Float32Array:return e;case Uint32Array:return Math.round(e*4294967295);case Uint16Array:return Math.round(e*65535);case Uint8Array:case Uint8ClampedArray:return Math.round(e*255);case Int32Array:return Math.round(e*2147483647);case Int16Array:return Math.round(e*32767);case Int8Array:return Math.round(e*127);default:throw Error(`THREE.MathUtils: Invalid component type.`)}}var hn={DEG2RAD:Gt,RAD2DEG:Kt,generateUUID:qt,clamp:G,euclideanModulo:Jt,mapLinear:Yt,inverseLerp:Xt,lerp:Zt,damp:Qt,pingpong:$t,smoothstep:en,smootherstep:tn,randInt:nn,randFloat:rn,randFloatSpread:an,seededRandom:on,degToRad:sn,radToDeg:cn,isPowerOfTwo:ln,ceilPowerOfTwo:un,floorPowerOfTwo:dn,setQuaternionFromProperEuler:fn,normalize:mn,denormalize:pn},K=class e{static{e.prototype.isVector2=!0}constructor(e=0,t=0){this.x=e,this.y=t}get width(){return this.x}set width(e){this.x=e}get height(){return this.y}set height(e){this.y=e}set(e,t){return this.x=e,this.y=t,this}setScalar(e){return this.x=e,this.y=e,this}setX(e){return this.x=e,this}setY(e){return this.y=e,this}setComponent(e,t){switch(e){case 0:this.x=t;break;case 1:this.y=t;break;default:throw Error(`THREE.Vector2: index is out of range: `+e)}return this}getComponent(e){switch(e){case 0:return this.x;case 1:return this.y;default:throw Error(`THREE.Vector2: index is out of range: `+e)}}clone(){return new this.constructor(this.x,this.y)}copy(e){return this.x=e.x,this.y=e.y,this}add(e){return this.x+=e.x,this.y+=e.y,this}addScalar(e){return this.x+=e,this.y+=e,this}addVectors(e,t){return this.x=e.x+t.x,this.y=e.y+t.y,this}addScaledVector(e,t){return this.x+=e.x*t,this.y+=e.y*t,this}sub(e){return this.x-=e.x,this.y-=e.y,this}subScalar(e){return this.x-=e,this.y-=e,this}subVectors(e,t){return this.x=e.x-t.x,this.y=e.y-t.y,this}multiply(e){return this.x*=e.x,this.y*=e.y,this}multiplyScalar(e){return this.x*=e,this.y*=e,this}divide(e){return this.x/=e.x,this.y/=e.y,this}divideScalar(e){return this.multiplyScalar(1/e)}applyMatrix3(e){let t=this.x,n=this.y,r=e.elements;return this.x=r[0]*t+r[3]*n+r[6],this.y=r[1]*t+r[4]*n+r[7],this}min(e){return this.x=Math.min(this.x,e.x),this.y=Math.min(this.y,e.y),this}max(e){return this.x=Math.max(this.x,e.x),this.y=Math.max(this.y,e.y),this}clamp(e,t){return this.x=G(this.x,e.x,t.x),this.y=G(this.y,e.y,t.y),this}clampScalar(e,t){return this.x=G(this.x,e,t),this.y=G(this.y,e,t),this}clampLength(e,t){let n=this.length();return this.divideScalar(n||1).multiplyScalar(G(n,e,t))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this}negate(){return this.x=-this.x,this.y=-this.y,this}dot(e){return this.x*e.x+this.y*e.y}cross(e){return this.x*e.y-this.y*e.x}lengthSq(){return this.x*this.x+this.y*this.y}length(){return Math.sqrt(this.x*this.x+this.y*this.y)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)}normalize(){return this.divideScalar(this.length()||1)}angle(){return Math.atan2(-this.y,-this.x)+Math.PI}angleTo(e){let t=Math.sqrt(this.lengthSq()*e.lengthSq());if(t===0)return Math.PI/2;let n=this.dot(e)/t;return Math.acos(G(n,-1,1))}distanceTo(e){return Math.sqrt(this.distanceToSquared(e))}distanceToSquared(e){let t=this.x-e.x,n=this.y-e.y;return t*t+n*n}manhattanDistanceTo(e){return Math.abs(this.x-e.x)+Math.abs(this.y-e.y)}setLength(e){return this.normalize().multiplyScalar(e)}lerp(e,t){return this.x+=(e.x-this.x)*t,this.y+=(e.y-this.y)*t,this}lerpVectors(e,t,n){return this.x=e.x+(t.x-e.x)*n,this.y=e.y+(t.y-e.y)*n,this}equals(e){return e.x===this.x&&e.y===this.y}fromArray(e,t=0){return this.x=e[t],this.y=e[t+1],this}toArray(e=[],t=0){return e[t]=this.x,e[t+1]=this.y,e}fromBufferAttribute(e,t){return this.x=e.getX(t),this.y=e.getY(t),this}rotateAround(e,t){let n=Math.cos(t),r=Math.sin(t),i=this.x-e.x,a=this.y-e.y;return this.x=i*n-a*r+e.x,this.y=i*r+a*n+e.y,this}random(){return this.x=Math.random(),this.y=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y}},gn=class{constructor(e=0,t=0,n=0,r=1){this.isQuaternion=!0,this._x=e,this._y=t,this._z=n,this._w=r}static slerpFlat(e,t,n,r,i,a,o){let s=n[r+0],c=n[r+1],l=n[r+2],u=n[r+3],d=i[a+0],f=i[a+1],p=i[a+2],m=i[a+3];if(u!==m||s!==d||c!==f||l!==p){let e=s*d+c*f+l*p+u*m;e<0&&(d=-d,f=-f,p=-p,m=-m,e=-e);let t=1-o;if(e<.9995){let n=Math.acos(e),r=Math.sin(n);t=Math.sin(t*n)/r,o=Math.sin(o*n)/r,s=s*t+d*o,c=c*t+f*o,l=l*t+p*o,u=u*t+m*o}else{s=s*t+d*o,c=c*t+f*o,l=l*t+p*o,u=u*t+m*o;let e=1/Math.sqrt(s*s+c*c+l*l+u*u);s*=e,c*=e,l*=e,u*=e}}e[t]=s,e[t+1]=c,e[t+2]=l,e[t+3]=u}static multiplyQuaternionsFlat(e,t,n,r,i,a){let o=n[r],s=n[r+1],c=n[r+2],l=n[r+3],u=i[a],d=i[a+1],f=i[a+2],p=i[a+3];return e[t]=o*p+l*u+s*f-c*d,e[t+1]=s*p+l*d+c*u-o*f,e[t+2]=c*p+l*f+o*d-s*u,e[t+3]=l*p-o*u-s*d-c*f,e}get x(){return this._x}set x(e){this._x=e,this._onChangeCallback()}get y(){return this._y}set y(e){this._y=e,this._onChangeCallback()}get z(){return this._z}set z(e){this._z=e,this._onChangeCallback()}get w(){return this._w}set w(e){this._w=e,this._onChangeCallback()}set(e,t,n,r){return this._x=e,this._y=t,this._z=n,this._w=r,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._w)}copy(e){return this._x=e.x,this._y=e.y,this._z=e.z,this._w=e.w,this._onChangeCallback(),this}setFromEuler(e,t=!0){let n=e._x,r=e._y,i=e._z,a=e._order,o=Math.cos,s=Math.sin,c=o(n/2),l=o(r/2),u=o(i/2),d=s(n/2),f=s(r/2),p=s(i/2);switch(a){case`XYZ`:this._x=d*l*u+c*f*p,this._y=c*f*u-d*l*p,this._z=c*l*p+d*f*u,this._w=c*l*u-d*f*p;break;case`YXZ`:this._x=d*l*u+c*f*p,this._y=c*f*u-d*l*p,this._z=c*l*p-d*f*u,this._w=c*l*u+d*f*p;break;case`ZXY`:this._x=d*l*u-c*f*p,this._y=c*f*u+d*l*p,this._z=c*l*p+d*f*u,this._w=c*l*u-d*f*p;break;case`ZYX`:this._x=d*l*u-c*f*p,this._y=c*f*u+d*l*p,this._z=c*l*p-d*f*u,this._w=c*l*u+d*f*p;break;case`YZX`:this._x=d*l*u+c*f*p,this._y=c*f*u+d*l*p,this._z=c*l*p-d*f*u,this._w=c*l*u-d*f*p;break;case`XZY`:this._x=d*l*u-c*f*p,this._y=c*f*u-d*l*p,this._z=c*l*p+d*f*u,this._w=c*l*u+d*f*p;break;default:U(`Quaternion: .setFromEuler() encountered an unknown order: `+a)}return t===!0&&this._onChangeCallback(),this}setFromAxisAngle(e,t){let n=t/2,r=Math.sin(n);return this._x=e.x*r,this._y=e.y*r,this._z=e.z*r,this._w=Math.cos(n),this._onChangeCallback(),this}setFromRotationMatrix(e){let t=e.elements,n=t[0],r=t[4],i=t[8],a=t[1],o=t[5],s=t[9],c=t[2],l=t[6],u=t[10],d=n+o+u;if(d>0){let e=.5/Math.sqrt(d+1);this._w=.25/e,this._x=(l-s)*e,this._y=(i-c)*e,this._z=(a-r)*e}else if(n>o&&n>u){let e=2*Math.sqrt(1+n-o-u);this._w=(l-s)/e,this._x=.25*e,this._y=(r+a)/e,this._z=(i+c)/e}else if(o>u){let e=2*Math.sqrt(1+o-n-u);this._w=(i-c)/e,this._x=(r+a)/e,this._y=.25*e,this._z=(s+l)/e}else{let e=2*Math.sqrt(1+u-n-o);this._w=(a-r)/e,this._x=(i+c)/e,this._y=(s+l)/e,this._z=.25*e}return this._onChangeCallback(),this}setFromUnitVectors(e,t){let n=e.dot(t)+1;return n<1e-8?(n=0,Math.abs(e.x)>Math.abs(e.z)?(this._x=-e.y,this._y=e.x,this._z=0,this._w=n):(this._x=0,this._y=-e.z,this._z=e.y,this._w=n)):(this._x=e.y*t.z-e.z*t.y,this._y=e.z*t.x-e.x*t.z,this._z=e.x*t.y-e.y*t.x,this._w=n),this.normalize()}angleTo(e){return 2*Math.acos(Math.abs(G(this.dot(e),-1,1)))}rotateTowards(e,t){let n=this.angleTo(e);if(n===0)return this;let r=Math.min(1,t/n);return this.slerp(e,r),this}identity(){return this.set(0,0,0,1)}invert(){return this.conjugate()}conjugate(){return this._x*=-1,this._y*=-1,this._z*=-1,this._onChangeCallback(),this}dot(e){return this._x*e._x+this._y*e._y+this._z*e._z+this._w*e._w}lengthSq(){return this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w}length(){return Math.sqrt(this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w)}normalize(){let e=this.length();return e===0?(this._x=0,this._y=0,this._z=0,this._w=1):(e=1/e,this._x*=e,this._y*=e,this._z*=e,this._w*=e),this._onChangeCallback(),this}multiply(e){return this.multiplyQuaternions(this,e)}premultiply(e){return this.multiplyQuaternions(e,this)}multiplyQuaternions(e,t){let n=e._x,r=e._y,i=e._z,a=e._w,o=t._x,s=t._y,c=t._z,l=t._w;return this._x=n*l+a*o+r*c-i*s,this._y=r*l+a*s+i*o-n*c,this._z=i*l+a*c+n*s-r*o,this._w=a*l-n*o-r*s-i*c,this._onChangeCallback(),this}slerp(e,t){let n=e._x,r=e._y,i=e._z,a=e._w,o=this.dot(e);o<0&&(n=-n,r=-r,i=-i,a=-a,o=-o);let s=1-t;if(o<.9995){let e=Math.acos(o),c=Math.sin(e);s=Math.sin(s*e)/c,t=Math.sin(t*e)/c,this._x=this._x*s+n*t,this._y=this._y*s+r*t,this._z=this._z*s+i*t,this._w=this._w*s+a*t,this._onChangeCallback()}else this._x=this._x*s+n*t,this._y=this._y*s+r*t,this._z=this._z*s+i*t,this._w=this._w*s+a*t,this.normalize();return this}slerpQuaternions(e,t,n){return this.copy(e).slerp(t,n)}random(){let e=2*Math.PI*Math.random(),t=2*Math.PI*Math.random(),n=Math.random(),r=Math.sqrt(1-n),i=Math.sqrt(n);return this.set(r*Math.sin(e),r*Math.cos(e),i*Math.sin(t),i*Math.cos(t))}equals(e){return e._x===this._x&&e._y===this._y&&e._z===this._z&&e._w===this._w}fromArray(e,t=0){return this._x=e[t],this._y=e[t+1],this._z=e[t+2],this._w=e[t+3],this._onChangeCallback(),this}toArray(e=[],t=0){return e[t]=this._x,e[t+1]=this._y,e[t+2]=this._z,e[t+3]=this._w,e}fromBufferAttribute(e,t){return this._x=e.getX(t),this._y=e.getY(t),this._z=e.getZ(t),this._w=e.getW(t),this._onChangeCallback(),this}toJSON(){return this.toArray()}_onChange(e){return this._onChangeCallback=e,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._w}},q=class e{static{e.prototype.isVector3=!0}constructor(e=0,t=0,n=0){this.x=e,this.y=t,this.z=n}set(e,t,n){return n===void 0&&(n=this.z),this.x=e,this.y=t,this.z=n,this}setScalar(e){return this.x=e,this.y=e,this.z=e,this}setX(e){return this.x=e,this}setY(e){return this.y=e,this}setZ(e){return this.z=e,this}setComponent(e,t){switch(e){case 0:this.x=t;break;case 1:this.y=t;break;case 2:this.z=t;break;default:throw Error(`THREE.Vector3: index is out of range: `+e)}return this}getComponent(e){switch(e){case 0:return this.x;case 1:return this.y;case 2:return this.z;default:throw Error(`THREE.Vector3: index is out of range: `+e)}}clone(){return new this.constructor(this.x,this.y,this.z)}copy(e){return this.x=e.x,this.y=e.y,this.z=e.z,this}add(e){return this.x+=e.x,this.y+=e.y,this.z+=e.z,this}addScalar(e){return this.x+=e,this.y+=e,this.z+=e,this}addVectors(e,t){return this.x=e.x+t.x,this.y=e.y+t.y,this.z=e.z+t.z,this}addScaledVector(e,t){return this.x+=e.x*t,this.y+=e.y*t,this.z+=e.z*t,this}sub(e){return this.x-=e.x,this.y-=e.y,this.z-=e.z,this}subScalar(e){return this.x-=e,this.y-=e,this.z-=e,this}subVectors(e,t){return this.x=e.x-t.x,this.y=e.y-t.y,this.z=e.z-t.z,this}multiply(e){return this.x*=e.x,this.y*=e.y,this.z*=e.z,this}multiplyScalar(e){return this.x*=e,this.y*=e,this.z*=e,this}multiplyVectors(e,t){return this.x=e.x*t.x,this.y=e.y*t.y,this.z=e.z*t.z,this}applyEuler(e){return this.applyQuaternion(vn.setFromEuler(e))}applyAxisAngle(e,t){return this.applyQuaternion(vn.setFromAxisAngle(e,t))}applyMatrix3(e){let t=this.x,n=this.y,r=this.z,i=e.elements;return this.x=i[0]*t+i[3]*n+i[6]*r,this.y=i[1]*t+i[4]*n+i[7]*r,this.z=i[2]*t+i[5]*n+i[8]*r,this}applyNormalMatrix(e){return this.applyMatrix3(e).normalize()}applyMatrix4(e){let t=this.x,n=this.y,r=this.z,i=e.elements,a=1/(i[3]*t+i[7]*n+i[11]*r+i[15]);return this.x=(i[0]*t+i[4]*n+i[8]*r+i[12])*a,this.y=(i[1]*t+i[5]*n+i[9]*r+i[13])*a,this.z=(i[2]*t+i[6]*n+i[10]*r+i[14])*a,this}applyQuaternion(e){let t=this.x,n=this.y,r=this.z,i=e.x,a=e.y,o=e.z,s=e.w,c=2*(a*r-o*n),l=2*(o*t-i*r),u=2*(i*n-a*t);return this.x=t+s*c+a*u-o*l,this.y=n+s*l+o*c-i*u,this.z=r+s*u+i*l-a*c,this}project(e){return this.applyMatrix4(e.matrixWorldInverse).applyMatrix4(e.projectionMatrix)}unproject(e){return this.applyMatrix4(e.projectionMatrixInverse).applyMatrix4(e.matrixWorld)}transformDirection(e){let t=this.x,n=this.y,r=this.z,i=e.elements;return this.x=i[0]*t+i[4]*n+i[8]*r,this.y=i[1]*t+i[5]*n+i[9]*r,this.z=i[2]*t+i[6]*n+i[10]*r,this.normalize()}divide(e){return this.x/=e.x,this.y/=e.y,this.z/=e.z,this}divideScalar(e){return this.multiplyScalar(1/e)}min(e){return this.x=Math.min(this.x,e.x),this.y=Math.min(this.y,e.y),this.z=Math.min(this.z,e.z),this}max(e){return this.x=Math.max(this.x,e.x),this.y=Math.max(this.y,e.y),this.z=Math.max(this.z,e.z),this}clamp(e,t){return this.x=G(this.x,e.x,t.x),this.y=G(this.y,e.y,t.y),this.z=G(this.z,e.z,t.z),this}clampScalar(e,t){return this.x=G(this.x,e,t),this.y=G(this.y,e,t),this.z=G(this.z,e,t),this}clampLength(e,t){let n=this.length();return this.divideScalar(n||1).multiplyScalar(G(n,e,t))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this}dot(e){return this.x*e.x+this.y*e.y+this.z*e.z}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)}normalize(){return this.divideScalar(this.length()||1)}setLength(e){return this.normalize().multiplyScalar(e)}lerp(e,t){return this.x+=(e.x-this.x)*t,this.y+=(e.y-this.y)*t,this.z+=(e.z-this.z)*t,this}lerpVectors(e,t,n){return this.x=e.x+(t.x-e.x)*n,this.y=e.y+(t.y-e.y)*n,this.z=e.z+(t.z-e.z)*n,this}cross(e){return this.crossVectors(this,e)}crossVectors(e,t){let n=e.x,r=e.y,i=e.z,a=t.x,o=t.y,s=t.z;return this.x=r*s-i*o,this.y=i*a-n*s,this.z=n*o-r*a,this}projectOnVector(e){let t=e.lengthSq();if(t===0)return this.set(0,0,0);let n=e.dot(this)/t;return this.copy(e).multiplyScalar(n)}projectOnPlane(e){return _n.copy(this).projectOnVector(e),this.sub(_n)}reflect(e){return this.sub(_n.copy(e).multiplyScalar(2*this.dot(e)))}angleTo(e){let t=Math.sqrt(this.lengthSq()*e.lengthSq());if(t===0)return Math.PI/2;let n=this.dot(e)/t;return Math.acos(G(n,-1,1))}distanceTo(e){return Math.sqrt(this.distanceToSquared(e))}distanceToSquared(e){let t=this.x-e.x,n=this.y-e.y,r=this.z-e.z;return t*t+n*n+r*r}manhattanDistanceTo(e){return Math.abs(this.x-e.x)+Math.abs(this.y-e.y)+Math.abs(this.z-e.z)}setFromSpherical(e){return this.setFromSphericalCoords(e.radius,e.phi,e.theta)}setFromSphericalCoords(e,t,n){let r=Math.sin(t)*e;return this.x=r*Math.sin(n),this.y=Math.cos(t)*e,this.z=r*Math.cos(n),this}setFromCylindrical(e){return this.setFromCylindricalCoords(e.radius,e.theta,e.y)}setFromCylindricalCoords(e,t,n){return this.x=e*Math.sin(t),this.y=n,this.z=e*Math.cos(t),this}setFromMatrixPosition(e){let t=e.elements;return this.x=t[12],this.y=t[13],this.z=t[14],this}setFromMatrixScale(e){let t=this.setFromMatrixColumn(e,0).length(),n=this.setFromMatrixColumn(e,1).length(),r=this.setFromMatrixColumn(e,2).length();return this.x=t,this.y=n,this.z=r,this}setFromMatrixColumn(e,t){return this.fromArray(e.elements,t*4)}setFromMatrix3Column(e,t){return this.fromArray(e.elements,t*3)}setFromEuler(e){return this.x=e._x,this.y=e._y,this.z=e._z,this}setFromColor(e){return this.x=e.r,this.y=e.g,this.z=e.b,this}equals(e){return e.x===this.x&&e.y===this.y&&e.z===this.z}fromArray(e,t=0){return this.x=e[t],this.y=e[t+1],this.z=e[t+2],this}toArray(e=[],t=0){return e[t]=this.x,e[t+1]=this.y,e[t+2]=this.z,e}fromBufferAttribute(e,t){return this.x=e.getX(t),this.y=e.getY(t),this.z=e.getZ(t),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this}randomDirection(){let e=Math.random()*Math.PI*2,t=Math.random()*2-1,n=Math.sqrt(1-t*t);return this.x=n*Math.cos(e),this.y=t,this.z=n*Math.sin(e),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z}},_n=new q,vn=new gn,yn=class e{static{e.prototype.isMatrix3=!0}constructor(e,t,n,r,i,a,o,s,c){this.elements=[1,0,0,0,1,0,0,0,1],e!==void 0&&this.set(e,t,n,r,i,a,o,s,c)}set(e,t,n,r,i,a,o,s,c){let l=this.elements;return l[0]=e,l[1]=r,l[2]=o,l[3]=t,l[4]=i,l[5]=s,l[6]=n,l[7]=a,l[8]=c,this}identity(){return this.set(1,0,0,0,1,0,0,0,1),this}copy(e){let t=this.elements,n=e.elements;return t[0]=n[0],t[1]=n[1],t[2]=n[2],t[3]=n[3],t[4]=n[4],t[5]=n[5],t[6]=n[6],t[7]=n[7],t[8]=n[8],this}extractBasis(e,t,n){return e.setFromMatrix3Column(this,0),t.setFromMatrix3Column(this,1),n.setFromMatrix3Column(this,2),this}setFromMatrix4(e){let t=e.elements;return this.set(t[0],t[4],t[8],t[1],t[5],t[9],t[2],t[6],t[10]),this}multiply(e){return this.multiplyMatrices(this,e)}premultiply(e){return this.multiplyMatrices(e,this)}multiplyMatrices(e,t){let n=e.elements,r=t.elements,i=this.elements,a=n[0],o=n[3],s=n[6],c=n[1],l=n[4],u=n[7],d=n[2],f=n[5],p=n[8],m=r[0],h=r[3],g=r[6],_=r[1],v=r[4],y=r[7],b=r[2],x=r[5],S=r[8];return i[0]=a*m+o*_+s*b,i[3]=a*h+o*v+s*x,i[6]=a*g+o*y+s*S,i[1]=c*m+l*_+u*b,i[4]=c*h+l*v+u*x,i[7]=c*g+l*y+u*S,i[2]=d*m+f*_+p*b,i[5]=d*h+f*v+p*x,i[8]=d*g+f*y+p*S,this}multiplyScalar(e){let t=this.elements;return t[0]*=e,t[3]*=e,t[6]*=e,t[1]*=e,t[4]*=e,t[7]*=e,t[2]*=e,t[5]*=e,t[8]*=e,this}determinant(){let e=this.elements,t=e[0],n=e[1],r=e[2],i=e[3],a=e[4],o=e[5],s=e[6],c=e[7],l=e[8];return t*a*l-t*o*c-n*i*l+n*o*s+r*i*c-r*a*s}invert(){let e=this.elements,t=e[0],n=e[1],r=e[2],i=e[3],a=e[4],o=e[5],s=e[6],c=e[7],l=e[8],u=l*a-o*c,d=o*s-l*i,f=c*i-a*s,p=t*u+n*d+r*f;if(p===0)return this.set(0,0,0,0,0,0,0,0,0);let m=1/p;return e[0]=u*m,e[1]=(r*c-l*n)*m,e[2]=(o*n-r*a)*m,e[3]=d*m,e[4]=(l*t-r*s)*m,e[5]=(r*i-o*t)*m,e[6]=f*m,e[7]=(n*s-c*t)*m,e[8]=(a*t-n*i)*m,this}transpose(){let e,t=this.elements;return e=t[1],t[1]=t[3],t[3]=e,e=t[2],t[2]=t[6],t[6]=e,e=t[5],t[5]=t[7],t[7]=e,this}getNormalMatrix(e){return this.setFromMatrix4(e).invert().transpose()}transposeIntoArray(e){let t=this.elements;return e[0]=t[0],e[1]=t[3],e[2]=t[6],e[3]=t[1],e[4]=t[4],e[5]=t[7],e[6]=t[2],e[7]=t[5],e[8]=t[8],this}setUvTransform(e,t,n,r,i,a,o){let s=Math.cos(i),c=Math.sin(i);return this.set(n*s,n*c,-n*(s*a+c*o)+a+e,-r*c,r*s,-r*(-c*a+s*o)+o+t,0,0,1),this}scale(e,t){return zt(`Matrix3: .scale() is deprecated. Use .makeScale() instead.`),this.premultiply(bn.makeScale(e,t)),this}rotate(e){return zt(`Matrix3: .rotate() is deprecated. Use .makeRotation() instead.`),this.premultiply(bn.makeRotation(-e)),this}translate(e,t){return zt(`Matrix3: .translate() is deprecated. Use .makeTranslation() instead.`),this.premultiply(bn.makeTranslation(e,t)),this}makeTranslation(e,t){return e.isVector2?this.set(1,0,e.x,0,1,e.y,0,0,1):this.set(1,0,e,0,1,t,0,0,1),this}makeRotation(e){let t=Math.cos(e),n=Math.sin(e);return this.set(t,-n,0,n,t,0,0,0,1),this}makeScale(e,t){return this.set(e,0,0,0,t,0,0,0,1),this}equals(e){let t=this.elements,n=e.elements;for(let e=0;e<9;e++)if(t[e]!==n[e])return!1;return!0}fromArray(e,t=0){for(let n=0;n<9;n++)this.elements[n]=e[n+t];return this}toArray(e=[],t=0){let n=this.elements;return e[t]=n[0],e[t+1]=n[1],e[t+2]=n[2],e[t+3]=n[3],e[t+4]=n[4],e[t+5]=n[5],e[t+6]=n[6],e[t+7]=n[7],e[t+8]=n[8],e}clone(){return new this.constructor().fromArray(this.elements)}},bn=new yn,xn=new yn().set(.4123908,.3575843,.1804808,.212639,.7151687,.0721923,.0193308,.1191948,.9505322),Sn=new yn().set(3.2409699,-1.5373832,-.4986108,-.9692436,1.8759675,.0415551,.0556301,-.203977,1.0569715);function Cn(){let e={enabled:!0,workingColorSpace:et,spaces:{},convert:function(e,t,n){return this.enabled===!1||t===n||!t||!n||(this.spaces[t].transfer===`srgb`&&(e.r=Tn(e.r),e.g=Tn(e.g),e.b=Tn(e.b)),this.spaces[t].primaries!==this.spaces[n].primaries&&(e.applyMatrix3(this.spaces[t].toXYZ),e.applyMatrix3(this.spaces[n].fromXYZ)),this.spaces[n].transfer===`srgb`&&(e.r=En(e.r),e.g=En(e.g),e.b=En(e.b))),e},workingToColorSpace:function(e,t){return this.convert(e,this.workingColorSpace,t)},colorSpaceToWorking:function(e,t){return this.convert(e,t,this.workingColorSpace)},getPrimaries:function(e){return this.spaces[e].primaries},getTransfer:function(e){return e===``?tt:this.spaces[e].transfer},getToneMappingMode:function(e){return this.spaces[e].outputColorSpaceConfig.toneMappingMode||`standard`},getLuminanceCoefficients:function(e,t=this.workingColorSpace){return e.fromArray(this.spaces[t].luminanceCoefficients)},define:function(e){Object.assign(this.spaces,e)},_getMatrix:function(e,t,n){return e.copy(this.spaces[t].toXYZ).multiply(this.spaces[n].fromXYZ)},_getDrawingBufferColorSpace:function(e){return this.spaces[e].outputColorSpaceConfig.drawingBufferColorSpace},_getUnpackColorSpace:function(e=this.workingColorSpace){return this.spaces[e].workingColorSpaceConfig.unpackColorSpace},fromWorkingColorSpace:function(t,n){return zt(`ColorManagement: .fromWorkingColorSpace() has been renamed to .workingToColorSpace().`),e.workingToColorSpace(t,n)},toWorkingColorSpace:function(t,n){return zt(`ColorManagement: .toWorkingColorSpace() has been renamed to .colorSpaceToWorking().`),e.colorSpaceToWorking(t,n)}},t=[.64,.33,.3,.6,.15,.06],n=[.2126,.7152,.0722],r=[.3127,.329];return e.define({[et]:{primaries:t,whitePoint:r,transfer:tt,toXYZ:xn,fromXYZ:Sn,luminanceCoefficients:n,workingColorSpaceConfig:{unpackColorSpace:$e},outputColorSpaceConfig:{drawingBufferColorSpace:$e}},[$e]:{primaries:t,whitePoint:r,transfer:nt,toXYZ:xn,fromXYZ:Sn,luminanceCoefficients:n,outputColorSpaceConfig:{drawingBufferColorSpace:$e}}}),e}var wn=Cn();function Tn(e){return e<.04045?e*.0773993808:(e*.9478672986+.0521327014)**2.4}function En(e){return e<.0031308?e*12.92:1.055*e**.41666-.055}var Dn,On=class{static getDataURL(e,t=`image/png`){if(/^data:/i.test(e.src)||typeof HTMLCanvasElement>`u`)return e.src;let n;if(e instanceof HTMLCanvasElement)n=e;else{Dn===void 0&&(Dn=jt(`canvas`)),Dn.width=e.width,Dn.height=e.height;let t=Dn.getContext(`2d`);e instanceof ImageData?t.putImageData(e,0,0):t.drawImage(e,0,0,e.width,e.height),n=Dn}return n.toDataURL(t)}static sRGBToLinear(e){if(typeof HTMLImageElement<`u`&&e instanceof HTMLImageElement||typeof HTMLCanvasElement<`u`&&e instanceof HTMLCanvasElement||typeof ImageBitmap<`u`&&e instanceof ImageBitmap){let t=jt(`canvas`);t.width=e.width,t.height=e.height;let n=t.getContext(`2d`);n.drawImage(e,0,0,e.width,e.height);let r=n.getImageData(0,0,e.width,e.height),i=r.data;for(let e=0;e<i.length;e++)i[e]=Tn(i[e]/255)*255;return n.putImageData(r,0,0),t}if(e.data){let t=e.data.slice(0);for(let e=0;e<t.length;e++)t instanceof Uint8Array||t instanceof Uint8ClampedArray?t[e]=Math.floor(Tn(t[e]/255)*255):t[e]=Tn(t[e]);return{data:t,width:e.width,height:e.height}}return U(`ImageUtils.sRGBToLinear(): Unsupported image type. No color space conversion applied.`),e}},kn=0,An=class{constructor(e=null){this.isTextureSource=!0,Object.defineProperty(this,"id",{value:kn++}),this.uuid=qt(),this.data=e,this.dataReady=!0,this.version=0}getSize(e){let t=this.data;return typeof HTMLVideoElement<`u`&&t instanceof HTMLVideoElement?e.set(t.videoWidth,t.videoHeight,0):typeof VideoFrame<`u`&&t instanceof VideoFrame?e.set(t.displayWidth,t.displayHeight,0):t===null?e.set(0,0,0):e.set(t.width,t.height,t.depth||0),e}set needsUpdate(e){e===!0&&this.version++}toJSON(e){let t=e===void 0||typeof e==`string`;if(!t&&e.images[this.uuid]!==void 0)return e.images[this.uuid];let n={uuid:this.uuid,url:``},r=this.data;if(r!==null){let e;if(Array.isArray(r)){e=[];for(let t=0,n=r.length;t<n;t++)r[t].isDataTexture?e.push(jn(r[t].image)):e.push(jn(r[t]))}else e=jn(r);n.url=e}return t||(e.images[this.uuid]=n),n}};function jn(e){return typeof HTMLImageElement<`u`&&e instanceof HTMLImageElement||typeof HTMLCanvasElement<`u`&&e instanceof HTMLCanvasElement||typeof ImageBitmap<`u`&&e instanceof ImageBitmap?On.getDataURL(e):e.data?{data:Array.from(e.data),width:e.width,height:e.height,type:e.data.constructor.name}:(U(`Texture: Unable to serialize Texture.`),{})}var Mn=class extends An{constructor(e=null){zt(`Source: "Source" has been renamed to "TextureSource". Please update your code to use "THREE.TextureSource" instead.`),super(e),this.isSource=!0}},Nn=0,Pn=new q,Fn=class e extends Ht{constructor(t=e.DEFAULT_IMAGE,n=e.DEFAULT_MAPPING,r=c,i=c,a=h,o=v,s=F,l=b,u=e.DEFAULT_ANISOTROPY,d=``){super(),this.isTexture=!0,Object.defineProperty(this,"id",{value:Nn++}),this.uuid=qt(),this.name=``,this.source=new An(t),this.mipmaps=[],this.mapping=n,this.channel=0,this.wrapS=r,this.wrapT=i,this.magFilter=a,this.minFilter=o,this.anisotropy=u,this.format=s,this.internalFormat=null,this.type=l,this.offset=new K(0,0),this.repeat=new K(1,1),this.center=new K(0,0),this.rotation=0,this.matrixAutoUpdate=!0,this.matrix=new yn,this.generateMipmaps=!0,this.premultiplyAlpha=!1,this.flipY=!0,this.unpackAlignment=4,this.colorSpace=d,this.userData={},this.updateRanges=[],this.version=0,this.onUpdate=null,this.renderTarget=null,this.isRenderTargetTexture=!1,this.isArrayTexture=!!(t&&t.depth&&t.depth>1),this.pmremVersion=0,this.normalized=!1}get width(){return this.source.getSize(Pn).x}get height(){return this.source.getSize(Pn).y}get depth(){return this.source.getSize(Pn).z}get image(){return this.source.data}set image(e){this.source.data=e}updateMatrix(){this.matrix.setUvTransform(this.offset.x,this.offset.y,this.repeat.x,this.repeat.y,this.rotation,this.center.x,this.center.y)}addUpdateRange(e,t){this.updateRanges.push({start:e,count:t})}clearUpdateRanges(){this.updateRanges.length=0}clone(){return new this.constructor().copy(this)}copy(e){return this.name=e.name,this.source=e.source,this.mipmaps=e.mipmaps.slice(0),this.mapping=e.mapping,this.channel=e.channel,this.wrapS=e.wrapS,this.wrapT=e.wrapT,this.magFilter=e.magFilter,this.minFilter=e.minFilter,this.anisotropy=e.anisotropy,this.format=e.format,this.internalFormat=e.internalFormat,this.type=e.type,this.normalized=e.normalized,this.offset.copy(e.offset),this.repeat.copy(e.repeat),this.center.copy(e.center),this.rotation=e.rotation,this.matrixAutoUpdate=e.matrixAutoUpdate,this.matrix.copy(e.matrix),this.generateMipmaps=e.generateMipmaps,this.premultiplyAlpha=e.premultiplyAlpha,this.flipY=e.flipY,this.unpackAlignment=e.unpackAlignment,this.colorSpace=e.colorSpace,this.renderTarget=e.renderTarget,this.isRenderTargetTexture=e.isRenderTargetTexture,this.isArrayTexture=e.isArrayTexture,this.userData=JSON.parse(JSON.stringify(e.userData)),this.needsUpdate=!0,this}setValues(e){for(let t in e){let n=e[t];if(n===void 0){U(`Texture.setValues(): parameter '${t}' has value of undefined.`);continue}let r=this[t];r===void 0?U(`Texture.setValues(): property '${t}' does not exist.`):r&&n&&r.isVector2&&n.isVector2||r&&n&&r.isVector3&&n.isVector3||r&&n&&r.isMatrix3&&n.isMatrix3?r.copy(n):this[t]=n}}toJSON(e){let t=e===void 0||typeof e==`string`;if(!t&&e.textures[this.uuid]!==void 0)return e.textures[this.uuid];let n={metadata:{version:4.7,type:`Texture`,generator:`Texture.toJSON`},uuid:this.uuid,name:this.name,image:this.source.toJSON(e).uuid,mapping:this.mapping,channel:this.channel,repeat:[this.repeat.x,this.repeat.y],offset:[this.offset.x,this.offset.y],center:[this.center.x,this.center.y],rotation:this.rotation,wrap:[this.wrapS,this.wrapT],format:this.format,internalFormat:this.internalFormat,type:this.type,normalized:this.normalized,colorSpace:this.colorSpace,minFilter:this.minFilter,magFilter:this.magFilter,anisotropy:this.anisotropy,flipY:this.flipY,generateMipmaps:this.generateMipmaps,premultiplyAlpha:this.premultiplyAlpha,unpackAlignment:this.unpackAlignment};return Object.keys(this.userData).length>0&&(n.userData=this.userData),t||(e.textures[this.uuid]=n),n}dispose(){this.dispatchEvent({type:`dispose`})}transformUv(e){if(this.mapping!==300)return e;if(e.applyMatrix3(this.matrix),e.x<0||e.x>1)switch(this.wrapS){case s:e.x-=Math.floor(e.x);break;case c:e.x=e.x<0?0:1;break;case l:Math.abs(Math.floor(e.x)%2)===1?e.x=Math.ceil(e.x)-e.x:e.x-=Math.floor(e.x)}if(e.y<0||e.y>1)switch(this.wrapT){case s:e.y-=Math.floor(e.y);break;case c:e.y=e.y<0?0:1;break;case l:Math.abs(Math.floor(e.y)%2)===1?e.y=Math.ceil(e.y)-e.y:e.y-=Math.floor(e.y)}return this.flipY&&(e.y=1-e.y),e}set needsUpdate(e){e===!0&&(this.version++,this.source.needsUpdate=!0)}set needsPMREMUpdate(e){e===!0&&this.pmremVersion++}};Fn.DEFAULT_IMAGE=null,Fn.DEFAULT_MAPPING=300,Fn.DEFAULT_ANISOTROPY=1;var In=class e{static{e.prototype.isVector4=!0}constructor(e=0,t=0,n=0,r=1){this.x=e,this.y=t,this.z=n,this.w=r}get width(){return this.z}set width(e){this.z=e}get height(){return this.w}set height(e){this.w=e}set(e,t,n,r){return this.x=e,this.y=t,this.z=n,this.w=r,this}setScalar(e){return this.x=e,this.y=e,this.z=e,this.w=e,this}setX(e){return this.x=e,this}setY(e){return this.y=e,this}setZ(e){return this.z=e,this}setW(e){return this.w=e,this}setComponent(e,t){switch(e){case 0:this.x=t;break;case 1:this.y=t;break;case 2:this.z=t;break;case 3:this.w=t;break;default:throw Error(`THREE.Vector4: index is out of range: `+e)}return this}getComponent(e){switch(e){case 0:return this.x;case 1:return this.y;case 2:return this.z;case 3:return this.w;default:throw Error(`THREE.Vector4: index is out of range: `+e)}}clone(){return new this.constructor(this.x,this.y,this.z,this.w)}copy(e){return this.x=e.x,this.y=e.y,this.z=e.z,this.w=e.w===void 0?1:e.w,this}add(e){return this.x+=e.x,this.y+=e.y,this.z+=e.z,this.w+=e.w,this}addScalar(e){return this.x+=e,this.y+=e,this.z+=e,this.w+=e,this}addVectors(e,t){return this.x=e.x+t.x,this.y=e.y+t.y,this.z=e.z+t.z,this.w=e.w+t.w,this}addScaledVector(e,t){return this.x+=e.x*t,this.y+=e.y*t,this.z+=e.z*t,this.w+=e.w*t,this}sub(e){return this.x-=e.x,this.y-=e.y,this.z-=e.z,this.w-=e.w,this}subScalar(e){return this.x-=e,this.y-=e,this.z-=e,this.w-=e,this}subVectors(e,t){return this.x=e.x-t.x,this.y=e.y-t.y,this.z=e.z-t.z,this.w=e.w-t.w,this}multiply(e){return this.x*=e.x,this.y*=e.y,this.z*=e.z,this.w*=e.w,this}multiplyScalar(e){return this.x*=e,this.y*=e,this.z*=e,this.w*=e,this}applyMatrix4(e){let t=this.x,n=this.y,r=this.z,i=this.w,a=e.elements;return this.x=a[0]*t+a[4]*n+a[8]*r+a[12]*i,this.y=a[1]*t+a[5]*n+a[9]*r+a[13]*i,this.z=a[2]*t+a[6]*n+a[10]*r+a[14]*i,this.w=a[3]*t+a[7]*n+a[11]*r+a[15]*i,this}divide(e){return this.x/=e.x,this.y/=e.y,this.z/=e.z,this.w/=e.w,this}divideScalar(e){return this.multiplyScalar(1/e)}setAxisAngleFromQuaternion(e){this.w=2*Math.acos(e.w);let t=Math.sqrt(1-e.w*e.w);return t<1e-4?(this.x=1,this.y=0,this.z=0):(this.x=e.x/t,this.y=e.y/t,this.z=e.z/t),this}setAxisAngleFromRotationMatrix(e){let t,n,r,i,a=.01,o=.1,s=e.elements,c=s[0],l=s[4],u=s[8],d=s[1],f=s[5],p=s[9],m=s[2],h=s[6],g=s[10];if(Math.abs(l-d)<a&&Math.abs(u-m)<a&&Math.abs(p-h)<a){if(Math.abs(l+d)<o&&Math.abs(u+m)<o&&Math.abs(p+h)<o&&Math.abs(c+f+g-3)<o)return this.set(1,0,0,0),this;t=Math.PI;let e=(c+1)/2,s=(f+1)/2,_=(g+1)/2,v=(l+d)/4,y=(u+m)/4,b=(p+h)/4;return e>s&&e>_?e<a?(n=0,r=.707106781,i=.707106781):(n=Math.sqrt(e),r=v/n,i=y/n):s>_?s<a?(n=.707106781,r=0,i=.707106781):(r=Math.sqrt(s),n=v/r,i=b/r):_<a?(n=.707106781,r=.707106781,i=0):(i=Math.sqrt(_),n=y/i,r=b/i),this.set(n,r,i,t),this}let _=Math.sqrt((h-p)*(h-p)+(u-m)*(u-m)+(d-l)*(d-l));return Math.abs(_)<.001&&(_=1),this.x=(h-p)/_,this.y=(u-m)/_,this.z=(d-l)/_,this.w=Math.acos((c+f+g-1)/2),this}setFromMatrixPosition(e){let t=e.elements;return this.x=t[12],this.y=t[13],this.z=t[14],this.w=t[15],this}min(e){return this.x=Math.min(this.x,e.x),this.y=Math.min(this.y,e.y),this.z=Math.min(this.z,e.z),this.w=Math.min(this.w,e.w),this}max(e){return this.x=Math.max(this.x,e.x),this.y=Math.max(this.y,e.y),this.z=Math.max(this.z,e.z),this.w=Math.max(this.w,e.w),this}clamp(e,t){return this.x=G(this.x,e.x,t.x),this.y=G(this.y,e.y,t.y),this.z=G(this.z,e.z,t.z),this.w=G(this.w,e.w,t.w),this}clampScalar(e,t){return this.x=G(this.x,e,t),this.y=G(this.y,e,t),this.z=G(this.z,e,t),this.w=G(this.w,e,t),this}clampLength(e,t){let n=this.length();return this.divideScalar(n||1).multiplyScalar(G(n,e,t))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this.w=Math.floor(this.w),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this.w=Math.ceil(this.w),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this.w=Math.round(this.w),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this.w=Math.trunc(this.w),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this.w=-this.w,this}dot(e){return this.x*e.x+this.y*e.y+this.z*e.z+this.w*e.w}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)+Math.abs(this.w)}normalize(){return this.divideScalar(this.length()||1)}setLength(e){return this.normalize().multiplyScalar(e)}lerp(e,t){return this.x+=(e.x-this.x)*t,this.y+=(e.y-this.y)*t,this.z+=(e.z-this.z)*t,this.w+=(e.w-this.w)*t,this}lerpVectors(e,t,n){return this.x=e.x+(t.x-e.x)*n,this.y=e.y+(t.y-e.y)*n,this.z=e.z+(t.z-e.z)*n,this.w=e.w+(t.w-e.w)*n,this}equals(e){return e.x===this.x&&e.y===this.y&&e.z===this.z&&e.w===this.w}fromArray(e,t=0){return this.x=e[t],this.y=e[t+1],this.z=e[t+2],this.w=e[t+3],this}toArray(e=[],t=0){return e[t]=this.x,e[t+1]=this.y,e[t+2]=this.z,e[t+3]=this.w,e}fromBufferAttribute(e,t){return this.x=e.getX(t),this.y=e.getY(t),this.z=e.getZ(t),this.w=e.getW(t),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this.w=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z,yield this.w}},Ln=class extends Ht{constructor(e=1,t=1,n={}){super(),n=Object.assign({generateMipmaps:!1,internalFormat:null,minFilter:h,depthBuffer:!0,stencilBuffer:!1,resolveColorBuffer:!0,resolveDepthBuffer:!0,resolveStencilBuffer:!0,storeMultisampledColorBuffer:!0,storeMultisampledDepthBuffer:!0,storeMultisampledStencilBuffer:!0,depthTexture:null,samples:0,count:1,depth:1,multiview:!1,useArrayDepthTexture:!1},n),this.isRenderTarget=!0,this.width=e,this.height=t,this.depth=n.depth,this.scissor=new In(0,0,e,t),this.scissorTest=!1,this.viewport=new In(0,0,e,t),this.textures=[];let r=new Fn({width:e,height:t,depth:n.depth}),i=n.count;for(let e=0;e<i;e++)this.textures[e]=r.clone(),this.textures[e].isRenderTargetTexture=!0,this.textures[e].renderTarget=this;this._setTextureOptions(n),this.depthBuffer=n.depthBuffer,this.stencilBuffer=n.stencilBuffer,this.resolveColorBuffer=n.resolveColorBuffer,this.resolveDepthBuffer=n.resolveDepthBuffer,this.resolveStencilBuffer=n.resolveStencilBuffer,this.storeMultisampledColorBuffer=n.storeMultisampledColorBuffer,this.storeMultisampledDepthBuffer=n.storeMultisampledDepthBuffer,this.storeMultisampledStencilBuffer=n.storeMultisampledStencilBuffer,this._depthTexture=null,this.depthTexture=n.depthTexture,this.samples=n.samples,this.multiview=n.multiview,this.useArrayDepthTexture=n.useArrayDepthTexture}_setTextureOptions(e={}){let t={minFilter:h,generateMipmaps:!1,flipY:!1,internalFormat:null};e.mapping!==void 0&&(t.mapping=e.mapping),e.wrapS!==void 0&&(t.wrapS=e.wrapS),e.wrapT!==void 0&&(t.wrapT=e.wrapT),e.wrapR!==void 0&&(t.wrapR=e.wrapR),e.magFilter!==void 0&&(t.magFilter=e.magFilter),e.minFilter!==void 0&&(t.minFilter=e.minFilter),e.format!==void 0&&(t.format=e.format),e.type!==void 0&&(t.type=e.type),e.anisotropy!==void 0&&(t.anisotropy=e.anisotropy),e.colorSpace!==void 0&&(t.colorSpace=e.colorSpace),e.flipY!==void 0&&(t.flipY=e.flipY),e.generateMipmaps!==void 0&&(t.generateMipmaps=e.generateMipmaps),e.internalFormat!==void 0&&(t.internalFormat=e.internalFormat);for(let e=0;e<this.textures.length;e++)this.textures[e].setValues(t)}get texture(){return this.textures[0]}set texture(e){this.textures[0]=e}set depthTexture(e){this._depthTexture!==null&&this._depthTexture.renderTarget===this&&(this._depthTexture.renderTarget=null),e!==null&&e.renderTarget===null&&(e.renderTarget=this),this._depthTexture=e}get depthTexture(){return this._depthTexture}setSize(e,t,n=1){if(this.width!==e||this.height!==t||this.depth!==n){this.width=e,this.height=t,this.depth=n;for(let r=0,i=this.textures.length;r<i;r++)this.textures[r].image.width=e,this.textures[r].image.height=t,this.textures[r].image.depth=n,this.textures[r].isData3DTexture!==!0&&(this.textures[r].isArrayTexture=this.textures[r].image.depth>1);this.dispose()}this.viewport.set(0,0,e,t),this.scissor.set(0,0,e,t)}clone(){return new this.constructor().copy(this)}copy(e){this.width=e.width,this.height=e.height,this.depth=e.depth,this.scissor.copy(e.scissor),this.scissorTest=e.scissorTest,this.viewport.copy(e.viewport),this.textures.length=0;for(let t=0,n=e.textures.length;t<n;t++){this.textures[t]=e.textures[t].clone(),this.textures[t].isRenderTargetTexture=!0,this.textures[t].renderTarget=this;let n=Object.assign({},e.textures[t].image);this.textures[t].source=new An(n)}if(this.depthBuffer=e.depthBuffer,this.stencilBuffer=e.stencilBuffer,this.resolveColorBuffer=e.resolveColorBuffer,this.resolveDepthBuffer=e.resolveDepthBuffer,this.resolveStencilBuffer=e.resolveStencilBuffer,this.storeMultisampledColorBuffer=e.storeMultisampledColorBuffer,this.storeMultisampledDepthBuffer=e.storeMultisampledDepthBuffer,this.storeMultisampledStencilBuffer=e.storeMultisampledStencilBuffer,e.depthTexture!==null){if(e.depthTexture.renderTarget===e){let t=e.depthTexture.clone();t.renderTarget=null,this.depthTexture=t}else this.depthTexture=e.depthTexture}return this.samples=e.samples,this.multiview=e.multiview,this.useArrayDepthTexture=e.useArrayDepthTexture,this}dispose(){this.dispatchEvent({type:`dispose`})}},Rn=class extends Ln{constructor(e=1,t=1,n={}){super(e,t,n),this.isWebGLRenderTarget=!0}},zn=class extends Fn{constructor(e=null,t=1,n=1,r=1){super(null),this.isDataArrayTexture=!0,this.image={data:e,width:t,height:n,depth:r},this.magFilter=u,this.minFilter=u,this.wrapR=c,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1,this.layerUpdates=new Set}copy(e){return super.copy(e),this.wrapR=e.wrapR,this}addLayerUpdate(e){this.layerUpdates.add(e)}clearLayerUpdates(){this.layerUpdates.clear()}},Bn=class extends Rn{constructor(e=1,t=1,n=1,r={}){super(e,t,r),this.isWebGLArrayRenderTarget=!0,this.depth=n,this.texture=new zn(null,e,t,n),this._setTextureOptions(r),this.texture.isRenderTargetTexture=!0}},Vn=class extends Fn{constructor(e=null,t=1,n=1,r=1){super(null),this.isData3DTexture=!0,this.image={data:e,width:t,height:n,depth:r},this.magFilter=u,this.minFilter=u,this.wrapR=c,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1}copy(e){return super.copy(e),this.wrapR=e.wrapR,this}},Hn=class extends Rn{constructor(e=1,t=1,n=1,r={}){super(e,t,r),this.isWebGL3DRenderTarget=!0,this.depth=n,this.texture=new Vn(null,e,t,n),this._setTextureOptions(r),this.texture.isRenderTargetTexture=!0}},J=class e{static{e.prototype.isMatrix4=!0}constructor(e,t,n,r,i,a,o,s,c,l,u,d,f,p,m,h){this.elements=[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],e!==void 0&&this.set(e,t,n,r,i,a,o,s,c,l,u,d,f,p,m,h)}set(e,t,n,r,i,a,o,s,c,l,u,d,f,p,m,h){let g=this.elements;return g[0]=e,g[4]=t,g[8]=n,g[12]=r,g[1]=i,g[5]=a,g[9]=o,g[13]=s,g[2]=c,g[6]=l,g[10]=u,g[14]=d,g[3]=f,g[7]=p,g[11]=m,g[15]=h,this}identity(){return this.set(1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1),this}clone(){return new e().fromArray(this.elements)}copy(e){let t=this.elements,n=e.elements;return t[0]=n[0],t[1]=n[1],t[2]=n[2],t[3]=n[3],t[4]=n[4],t[5]=n[5],t[6]=n[6],t[7]=n[7],t[8]=n[8],t[9]=n[9],t[10]=n[10],t[11]=n[11],t[12]=n[12],t[13]=n[13],t[14]=n[14],t[15]=n[15],this}copyPosition(e){let t=this.elements,n=e.elements;return t[12]=n[12],t[13]=n[13],t[14]=n[14],this}setFromMatrix3(e){let t=e.elements;return this.set(t[0],t[3],t[6],0,t[1],t[4],t[7],0,t[2],t[5],t[8],0,0,0,0,1),this}extractBasis(e,t,n){return this.determinantAffine()===0?(e.set(1,0,0),t.set(0,1,0),n.set(0,0,1),this):(e.setFromMatrixColumn(this,0),t.setFromMatrixColumn(this,1),n.setFromMatrixColumn(this,2),this)}makeBasis(e,t,n){return this.set(e.x,t.x,n.x,0,e.y,t.y,n.y,0,e.z,t.z,n.z,0,0,0,0,1),this}extractRotation(e){if(e.determinantAffine()===0)return this.identity();let t=this.elements,n=e.elements,r=1/Un.setFromMatrixColumn(e,0).length(),i=1/Un.setFromMatrixColumn(e,1).length(),a=1/Un.setFromMatrixColumn(e,2).length();return t[0]=n[0]*r,t[1]=n[1]*r,t[2]=n[2]*r,t[3]=0,t[4]=n[4]*i,t[5]=n[5]*i,t[6]=n[6]*i,t[7]=0,t[8]=n[8]*a,t[9]=n[9]*a,t[10]=n[10]*a,t[11]=0,t[12]=0,t[13]=0,t[14]=0,t[15]=1,this}makeRotationFromEuler(e){let t=this.elements,n=e.x,r=e.y,i=e.z,a=Math.cos(n),o=Math.sin(n),s=Math.cos(r),c=Math.sin(r),l=Math.cos(i),u=Math.sin(i);if(e.order===`XYZ`){let e=a*l,n=a*u,r=o*l,i=o*u;t[0]=s*l,t[4]=-s*u,t[8]=c,t[1]=n+r*c,t[5]=e-i*c,t[9]=-o*s,t[2]=i-e*c,t[6]=r+n*c,t[10]=a*s}else if(e.order===`YXZ`){let e=s*l,n=s*u,r=c*l,i=c*u;t[0]=e+i*o,t[4]=r*o-n,t[8]=a*c,t[1]=a*u,t[5]=a*l,t[9]=-o,t[2]=n*o-r,t[6]=i+e*o,t[10]=a*s}else if(e.order===`ZXY`){let e=s*l,n=s*u,r=c*l,i=c*u;t[0]=e-i*o,t[4]=-a*u,t[8]=r+n*o,t[1]=n+r*o,t[5]=a*l,t[9]=i-e*o,t[2]=-a*c,t[6]=o,t[10]=a*s}else if(e.order===`ZYX`){let e=a*l,n=a*u,r=o*l,i=o*u;t[0]=s*l,t[4]=r*c-n,t[8]=e*c+i,t[1]=s*u,t[5]=i*c+e,t[9]=n*c-r,t[2]=-c,t[6]=o*s,t[10]=a*s}else if(e.order===`YZX`){let e=a*s,n=a*c,r=o*s,i=o*c;t[0]=s*l,t[4]=i-e*u,t[8]=r*u+n,t[1]=u,t[5]=a*l,t[9]=-o*l,t[2]=-c*l,t[6]=n*u+r,t[10]=e-i*u}else if(e.order===`XZY`){let e=a*s,n=a*c,r=o*s,i=o*c;t[0]=s*l,t[4]=-u,t[8]=c*l,t[1]=e*u+i,t[5]=a*l,t[9]=n*u-r,t[2]=r*u-n,t[6]=o*l,t[10]=i*u+e}return t[3]=0,t[7]=0,t[11]=0,t[12]=0,t[13]=0,t[14]=0,t[15]=1,this}makeRotationFromQuaternion(e){return this.compose(Gn,e,Kn)}lookAt(e,t,n){let r=this.elements;return Yn.subVectors(e,t),Yn.lengthSq()===0&&(Yn.z=1),Yn.normalize(),qn.crossVectors(n,Yn),qn.lengthSq()===0&&(Math.abs(n.z)===1?Yn.x+=1e-4:Yn.z+=1e-4,Yn.normalize(),qn.crossVectors(n,Yn)),qn.normalize(),Jn.crossVectors(Yn,qn),r[0]=qn.x,r[4]=Jn.x,r[8]=Yn.x,r[1]=qn.y,r[5]=Jn.y,r[9]=Yn.y,r[2]=qn.z,r[6]=Jn.z,r[10]=Yn.z,this}multiply(e){return this.multiplyMatrices(this,e)}premultiply(e){return this.multiplyMatrices(e,this)}multiplyMatrices(e,t){let n=e.elements,r=t.elements,i=this.elements,a=n[0],o=n[4],s=n[8],c=n[12],l=n[1],u=n[5],d=n[9],f=n[13],p=n[2],m=n[6],h=n[10],g=n[14],_=n[3],v=n[7],y=n[11],b=n[15],x=r[0],S=r[4],C=r[8],w=r[12],T=r[1],E=r[5],D=r[9],O=r[13],k=r[2],A=r[6],j=r[10],M=r[14],N=r[3],P=r[7],F=r[11],I=r[15];return i[0]=a*x+o*T+s*k+c*N,i[4]=a*S+o*E+s*A+c*P,i[8]=a*C+o*D+s*j+c*F,i[12]=a*w+o*O+s*M+c*I,i[1]=l*x+u*T+d*k+f*N,i[5]=l*S+u*E+d*A+f*P,i[9]=l*C+u*D+d*j+f*F,i[13]=l*w+u*O+d*M+f*I,i[2]=p*x+m*T+h*k+g*N,i[6]=p*S+m*E+h*A+g*P,i[10]=p*C+m*D+h*j+g*F,i[14]=p*w+m*O+h*M+g*I,i[3]=_*x+v*T+y*k+b*N,i[7]=_*S+v*E+y*A+b*P,i[11]=_*C+v*D+y*j+b*F,i[15]=_*w+v*O+y*M+b*I,this}multiplyScalar(e){let t=this.elements;return t[0]*=e,t[4]*=e,t[8]*=e,t[12]*=e,t[1]*=e,t[5]*=e,t[9]*=e,t[13]*=e,t[2]*=e,t[6]*=e,t[10]*=e,t[14]*=e,t[3]*=e,t[7]*=e,t[11]*=e,t[15]*=e,this}determinant(){let e=this.elements,t=e[0],n=e[4],r=e[8],i=e[12],a=e[1],o=e[5],s=e[9],c=e[13],l=e[2],u=e[6],d=e[10],f=e[14],p=e[3],m=e[7],h=e[11],g=e[15],_=s*f-c*d,v=o*f-c*u,y=o*d-s*u,b=a*f-c*l,x=a*d-s*l,S=a*u-o*l;return t*(m*_-h*v+g*y)-n*(p*_-h*b+g*x)+r*(p*v-m*b+g*S)-i*(p*y-m*x+h*S)}determinantAffine(){let e=this.elements,t=e[0],n=e[4],r=e[8],i=e[1],a=e[5],o=e[9],s=e[2],c=e[6],l=e[10];return t*(a*l-o*c)-n*(i*l-o*s)+r*(i*c-a*s)}transpose(){let e=this.elements,t;return t=e[1],e[1]=e[4],e[4]=t,t=e[2],e[2]=e[8],e[8]=t,t=e[6],e[6]=e[9],e[9]=t,t=e[3],e[3]=e[12],e[12]=t,t=e[7],e[7]=e[13],e[13]=t,t=e[11],e[11]=e[14],e[14]=t,this}setPosition(e,t,n){let r=this.elements;return e.isVector3?(r[12]=e.x,r[13]=e.y,r[14]=e.z):(r[12]=e,r[13]=t,r[14]=n),this}invert(){let e=this.elements,t=e[0],n=e[1],r=e[2],i=e[3],a=e[4],o=e[5],s=e[6],c=e[7],l=e[8],u=e[9],d=e[10],f=e[11],p=e[12],m=e[13],h=e[14],g=e[15],_=t*o-n*a,v=t*s-r*a,y=t*c-i*a,b=n*s-r*o,x=n*c-i*o,S=r*c-i*s,C=l*m-u*p,w=l*h-d*p,T=l*g-f*p,E=u*h-d*m,D=u*g-f*m,O=d*g-f*h,k=_*O-v*D+y*E+b*T-x*w+S*C;if(k===0)return this.set(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);let A=1/k;return e[0]=(o*O-s*D+c*E)*A,e[1]=(r*D-n*O-i*E)*A,e[2]=(m*S-h*x+g*b)*A,e[3]=(d*x-u*S-f*b)*A,e[4]=(s*T-a*O-c*w)*A,e[5]=(t*O-r*T+i*w)*A,e[6]=(h*y-p*S-g*v)*A,e[7]=(l*S-d*y+f*v)*A,e[8]=(a*D-o*T+c*C)*A,e[9]=(n*T-t*D-i*C)*A,e[10]=(p*x-m*y+g*_)*A,e[11]=(u*y-l*x-f*_)*A,e[12]=(o*w-a*E-s*C)*A,e[13]=(t*E-n*w+r*C)*A,e[14]=(m*v-p*b-h*_)*A,e[15]=(l*b-u*v+d*_)*A,this}scale(e){let t=this.elements,n=e.x,r=e.y,i=e.z;return t[0]*=n,t[4]*=r,t[8]*=i,t[1]*=n,t[5]*=r,t[9]*=i,t[2]*=n,t[6]*=r,t[10]*=i,t[3]*=n,t[7]*=r,t[11]*=i,this}getMaxScaleOnAxis(){let e=this.elements,t=e[0]*e[0]+e[1]*e[1]+e[2]*e[2],n=e[4]*e[4]+e[5]*e[5]+e[6]*e[6],r=e[8]*e[8]+e[9]*e[9]+e[10]*e[10];return Math.sqrt(Math.max(t,n,r))}makeTranslation(e,t,n){return e.isVector3?this.set(1,0,0,e.x,0,1,0,e.y,0,0,1,e.z,0,0,0,1):this.set(1,0,0,e,0,1,0,t,0,0,1,n,0,0,0,1),this}makeRotationX(e){let t=Math.cos(e),n=Math.sin(e);return this.set(1,0,0,0,0,t,-n,0,0,n,t,0,0,0,0,1),this}makeRotationY(e){let t=Math.cos(e),n=Math.sin(e);return this.set(t,0,n,0,0,1,0,0,-n,0,t,0,0,0,0,1),this}makeRotationZ(e){let t=Math.cos(e),n=Math.sin(e);return this.set(t,-n,0,0,n,t,0,0,0,0,1,0,0,0,0,1),this}makeRotationAxis(e,t){let n=Math.cos(t),r=Math.sin(t),i=1-n,a=e.x,o=e.y,s=e.z,c=i*a,l=i*o;return this.set(c*a+n,c*o-r*s,c*s+r*o,0,c*o+r*s,l*o+n,l*s-r*a,0,c*s-r*o,l*s+r*a,i*s*s+n,0,0,0,0,1),this}makeScale(e,t,n){return this.set(e,0,0,0,0,t,0,0,0,0,n,0,0,0,0,1),this}makeShear(e,t,n,r,i,a){return this.set(1,n,i,0,e,1,a,0,t,r,1,0,0,0,0,1),this}compose(e,t,n){let r=this.elements,i=t._x,a=t._y,o=t._z,s=t._w,c=i+i,l=a+a,u=o+o,d=i*c,f=i*l,p=i*u,m=a*l,h=a*u,g=o*u,_=s*c,v=s*l,y=s*u,b=n.x,x=n.y,S=n.z;return r[0]=(1-(m+g))*b,r[1]=(f+y)*b,r[2]=(p-v)*b,r[3]=0,r[4]=(f-y)*x,r[5]=(1-(d+g))*x,r[6]=(h+_)*x,r[7]=0,r[8]=(p+v)*S,r[9]=(h-_)*S,r[10]=(1-(d+m))*S,r[11]=0,r[12]=e.x,r[13]=e.y,r[14]=e.z,r[15]=1,this}decompose(e,t,n){let r=this.elements;e.x=r[12],e.y=r[13],e.z=r[14];let i=this.determinantAffine();if(i===0)return n.set(1,1,1),t.identity(),this;let a=Un.set(r[0],r[1],r[2]).length(),o=Un.set(r[4],r[5],r[6]).length(),s=Un.set(r[8],r[9],r[10]).length();i<0&&(a=-a),Wn.copy(this);let c=1/a,l=1/o,u=1/s;return Wn.elements[0]*=c,Wn.elements[1]*=c,Wn.elements[2]*=c,Wn.elements[4]*=l,Wn.elements[5]*=l,Wn.elements[6]*=l,Wn.elements[8]*=u,Wn.elements[9]*=u,Wn.elements[10]*=u,t.setFromRotationMatrix(Wn),n.x=a,n.y=o,n.z=s,this}makePerspective(e,t,n,r,i,a,o=bt,s=!1){let c=this.elements,l=2*i/(t-e),u=2*i/(n-r),d=(t+e)/(t-e),f=(n+r)/(n-r),p,m;if(s)p=i/(a-i),m=a*i/(a-i);else if(o===2e3)p=-(a+i)/(a-i),m=-2*a*i/(a-i);else if(o===2001)p=-a/(a-i),m=-a*i/(a-i);else throw Error(`THREE.Matrix4.makePerspective(): Invalid coordinate system: `+o);return c[0]=l,c[4]=0,c[8]=d,c[12]=0,c[1]=0,c[5]=u,c[9]=f,c[13]=0,c[2]=0,c[6]=0,c[10]=p,c[14]=m,c[3]=0,c[7]=0,c[11]=-1,c[15]=0,this}makeOrthographic(e,t,n,r,i,a,o=bt,s=!1){let c=this.elements,l=2/(t-e),u=2/(n-r),d=-(t+e)/(t-e),f=-(n+r)/(n-r),p,m;if(s)p=1/(a-i),m=a/(a-i);else if(o===2e3)p=-2/(a-i),m=-(a+i)/(a-i);else if(o===2001)p=-1/(a-i),m=-i/(a-i);else throw Error(`THREE.Matrix4.makeOrthographic(): Invalid coordinate system: `+o);return c[0]=l,c[4]=0,c[8]=0,c[12]=d,c[1]=0,c[5]=u,c[9]=0,c[13]=f,c[2]=0,c[6]=0,c[10]=p,c[14]=m,c[3]=0,c[7]=0,c[11]=0,c[15]=1,this}equals(e){let t=this.elements,n=e.elements;for(let e=0;e<16;e++)if(t[e]!==n[e])return!1;return!0}fromArray(e,t=0){for(let n=0;n<16;n++)this.elements[n]=e[n+t];return this}toArray(e=[],t=0){let n=this.elements;return e[t]=n[0],e[t+1]=n[1],e[t+2]=n[2],e[t+3]=n[3],e[t+4]=n[4],e[t+5]=n[5],e[t+6]=n[6],e[t+7]=n[7],e[t+8]=n[8],e[t+9]=n[9],e[t+10]=n[10],e[t+11]=n[11],e[t+12]=n[12],e[t+13]=n[13],e[t+14]=n[14],e[t+15]=n[15],e}},Un=new q,Wn=new J,Gn=new q(0,0,0),Kn=new q(1,1,1),qn=new q,Jn=new q,Yn=new q,Xn=new J,Zn=new gn,Qn=class e{constructor(t=0,n=0,r=0,i=e.DEFAULT_ORDER){this.isEuler=!0,this._x=t,this._y=n,this._z=r,this._order=i}get x(){return this._x}set x(e){this._x=e,this._onChangeCallback()}get y(){return this._y}set y(e){this._y=e,this._onChangeCallback()}get z(){return this._z}set z(e){this._z=e,this._onChangeCallback()}get order(){return this._order}set order(e){this._order=e,this._onChangeCallback()}set(e,t,n,r=this._order){return this._x=e,this._y=t,this._z=n,this._order=r,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._order)}copy(e){return this._x=e._x,this._y=e._y,this._z=e._z,this._order=e._order,this._onChangeCallback(),this}setFromRotationMatrix(e,t=this._order,n=!0){let r=e.elements,i=r[0],a=r[4],o=r[8],s=r[1],c=r[5],l=r[9],u=r[2],d=r[6],f=r[10];switch(t){case`XYZ`:this._y=Math.asin(G(o,-1,1)),Math.abs(o)<.9999999?(this._x=Math.atan2(-l,f),this._z=Math.atan2(-a,i)):(this._x=Math.atan2(d,c),this._z=0);break;case`YXZ`:this._x=Math.asin(-G(l,-1,1)),Math.abs(l)<.9999999?(this._y=Math.atan2(o,f),this._z=Math.atan2(s,c)):(this._y=Math.atan2(-u,i),this._z=0);break;case`ZXY`:this._x=Math.asin(G(d,-1,1)),Math.abs(d)<.9999999?(this._y=Math.atan2(-u,f),this._z=Math.atan2(-a,c)):(this._y=0,this._z=Math.atan2(s,i));break;case`ZYX`:this._y=Math.asin(-G(u,-1,1)),Math.abs(u)<.9999999?(this._x=Math.atan2(d,f),this._z=Math.atan2(s,i)):(this._x=0,this._z=Math.atan2(-a,c));break;case`YZX`:this._z=Math.asin(G(s,-1,1)),Math.abs(s)<.9999999?(this._x=Math.atan2(-l,c),this._y=Math.atan2(-u,i)):(this._x=0,this._y=Math.atan2(o,f));break;case`XZY`:this._z=Math.asin(-G(a,-1,1)),Math.abs(a)<.9999999?(this._x=Math.atan2(d,c),this._y=Math.atan2(o,i)):(this._x=Math.atan2(-l,f),this._y=0);break;default:U(`Euler: .setFromRotationMatrix() encountered an unknown order: `+t)}return this._order=t,n===!0&&this._onChangeCallback(),this}setFromQuaternion(e,t,n){return Xn.makeRotationFromQuaternion(e),this.setFromRotationMatrix(Xn,t,n)}setFromVector3(e,t=this._order){return this.set(e.x,e.y,e.z,t)}reorder(e){return Zn.setFromEuler(this),this.setFromQuaternion(Zn,e)}equals(e){return e._x===this._x&&e._y===this._y&&e._z===this._z&&e._order===this._order}fromArray(e){return this._x=e[0],this._y=e[1],this._z=e[2],e[3]!==void 0&&(this._order=e[3]),this._onChangeCallback(),this}toArray(e=[],t=0){return e[t]=this._x,e[t+1]=this._y,e[t+2]=this._z,e[t+3]=this._order,e}_onChange(e){return this._onChangeCallback=e,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._order}};Qn.DEFAULT_ORDER=`XYZ`;var $n=class{constructor(){this.mask=1}set(e){this.mask=1<<e>>>0}enable(e){this.mask|=1<<e}enableAll(){this.mask=-1}toggle(e){this.mask^=1<<e}disable(e){this.mask&=~(1<<e)}disableAll(){this.mask=0}test(e){return(this.mask&e.mask)!==0}isEnabled(e){return!!(this.mask&1<<e)}},er=0,tr=new q,nr=new gn,rr=new J,ir=new q,ar=new q,or=new q,sr=new gn,cr=new q(1,0,0),lr=new q(0,1,0),ur=new q(0,0,1),dr={type:`added`},fr={type:`removed`},pr={type:`childadded`,child:null},mr={type:`childremoved`,child:null},hr=class e extends Ht{constructor(){super(),this.isObject3D=!0,Object.defineProperty(this,"id",{value:er++}),this.uuid=qt(),this.name=``,this.type=`Object3D`,this.parent=null,this.children=[],this.up=e.DEFAULT_UP.clone();let t=new q,n=new Qn,r=new gn,i=new q(1,1,1);function a(){r.setFromEuler(n,!1)}function o(){n.setFromQuaternion(r,void 0,!1)}n._onChange(a),r._onChange(o),Object.defineProperties(this,{position:{configurable:!0,enumerable:!0,value:t},rotation:{configurable:!0,enumerable:!0,value:n},quaternion:{configurable:!0,enumerable:!0,value:r},scale:{configurable:!0,enumerable:!0,value:i},modelViewMatrix:{value:new J},normalMatrix:{value:new yn}}),this.matrix=new J,this.matrixWorld=new J,this.matrixAutoUpdate=e.DEFAULT_MATRIX_AUTO_UPDATE,this.matrixWorldAutoUpdate=e.DEFAULT_MATRIX_WORLD_AUTO_UPDATE,this.matrixWorldNeedsUpdate=!1,this.layers=new $n,this.visible=!0,this.castShadow=!1,this.receiveShadow=!1,this.frustumCulled=!0,this.renderOrder=0,this.animations=[],this.customDepthMaterial=void 0,this.customDistanceMaterial=void 0,this.static=!1,this.userData={},this.pivot=null}onBeforeShadow(){}onAfterShadow(){}onBeforeRender(){}onAfterRender(){}applyMatrix4(e){this.matrixAutoUpdate&&this.updateMatrix(),this.matrix.premultiply(e),this.matrix.decompose(this.position,this.quaternion,this.scale)}applyQuaternion(e){return this.quaternion.premultiply(e),this}setRotationFromAxisAngle(e,t){this.quaternion.setFromAxisAngle(e,t)}setRotationFromEuler(e){this.quaternion.setFromEuler(e,!0)}setRotationFromMatrix(e){this.quaternion.setFromRotationMatrix(e)}setRotationFromQuaternion(e){this.quaternion.copy(e)}rotateOnAxis(e,t){return nr.setFromAxisAngle(e,t),this.quaternion.multiply(nr),this}rotateOnWorldAxis(e,t){return nr.setFromAxisAngle(e,t),this.quaternion.premultiply(nr),this}rotateX(e){return this.rotateOnAxis(cr,e)}rotateY(e){return this.rotateOnAxis(lr,e)}rotateZ(e){return this.rotateOnAxis(ur,e)}translateOnAxis(e,t){return tr.copy(e).applyQuaternion(this.quaternion),this.position.add(tr.multiplyScalar(t)),this}translateX(e){return this.translateOnAxis(cr,e)}translateY(e){return this.translateOnAxis(lr,e)}translateZ(e){return this.translateOnAxis(ur,e)}localToWorld(e){return this.updateWorldMatrix(!0,!1),e.applyMatrix4(this.matrixWorld)}worldToLocal(e){return this.updateWorldMatrix(!0,!1),e.applyMatrix4(rr.copy(this.matrixWorld).invert())}lookAt(e,t,n){e.isVector3?ir.copy(e):ir.set(e,t,n);let r=this.parent;this.updateWorldMatrix(!0,!1),ar.setFromMatrixPosition(this.matrixWorld),this.isCamera||this.isLight?rr.lookAt(ar,ir,this.up):rr.lookAt(ir,ar,this.up),this.quaternion.setFromRotationMatrix(rr),r&&(rr.extractRotation(r.matrixWorld),nr.setFromRotationMatrix(rr),this.quaternion.premultiply(nr.invert()))}add(e){if(arguments.length>1){for(let e=0;e<arguments.length;e++)this.add(arguments[e]);return this}return e===this?(W(`Object3D.add: object can't be added as a child of itself.`,e),this):(e&&e.isObject3D?(e.removeFromParent(),e.parent=this,this.children.push(e),e.dispatchEvent(dr),pr.child=e,this.dispatchEvent(pr),pr.child=null):W(`Object3D.add: object not an instance of THREE.Object3D.`,e),this)}remove(e){if(arguments.length>1){for(let e=0;e<arguments.length;e++)this.remove(arguments[e]);return this}let t=this.children.indexOf(e);return t!==-1&&(e.parent=null,this.children.splice(t,1),e.dispatchEvent(fr),mr.child=e,this.dispatchEvent(mr),mr.child=null),this}removeFromParent(){let e=this.parent;return e!==null&&e.remove(this),this}clear(){return this.remove(...this.children)}attach(e){return this.updateWorldMatrix(!0,!1),rr.copy(this.matrixWorld).invert(),e.parent!==null&&(e.parent.updateWorldMatrix(!0,!1),rr.multiply(e.parent.matrixWorld)),e.applyMatrix4(rr),e.removeFromParent(),e.parent=this,this.children.push(e),e.updateWorldMatrix(!1,!0),e.dispatchEvent(dr),pr.child=e,this.dispatchEvent(pr),pr.child=null,this}getObjectById(e){return this.getObjectByProperty(`id`,e)}getObjectByName(e){return this.getObjectByProperty(`name`,e)}getObjectByProperty(e,t){if(this[e]===t)return this;for(let n=0,r=this.children.length;n<r;n++){let r=this.children[n].getObjectByProperty(e,t);if(r!==void 0)return r}}getObjectsByProperty(e,t,n=[]){this[e]===t&&n.push(this);let r=this.children;for(let i=0,a=r.length;i<a;i++)r[i].getObjectsByProperty(e,t,n);return n}getWorldPosition(e){return this.updateWorldMatrix(!0,!1),e.setFromMatrixPosition(this.matrixWorld)}getWorldQuaternion(e){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(ar,e,or),e}getWorldScale(e){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(ar,sr,e),e}getWorldDirection(e){this.updateWorldMatrix(!0,!1);let t=this.matrixWorld.elements;return e.set(t[8],t[9],t[10]).normalize()}raycast(){}intersectsFrustum(){}traverse(e){e(this);let t=this.children;for(let n=0,r=t.length;n<r;n++)t[n].traverse(e)}traverseVisible(e){if(this.visible===!1)return;e(this);let t=this.children;for(let n=0,r=t.length;n<r;n++)t[n].traverseVisible(e)}traverseAncestors(e){let t=this.parent;t!==null&&(e(t),t.traverseAncestors(e))}updateMatrix(){this.matrix.compose(this.position,this.quaternion,this.scale);let e=this.pivot;if(e!==null){let t=e.x,n=e.y,r=e.z,i=this.matrix.elements;i[12]+=t-i[0]*t-i[4]*n-i[8]*r,i[13]+=n-i[1]*t-i[5]*n-i[9]*r,i[14]+=r-i[2]*t-i[6]*n-i[10]*r}this.matrixWorldNeedsUpdate=!0}updateMatrixWorld(e){this.matrixAutoUpdate&&this.updateMatrix(),(this.matrixWorldNeedsUpdate||e)&&(this.matrixWorldAutoUpdate===!0&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix)),this.matrixWorldNeedsUpdate=!1,e=!0);let t=this.children;for(let n=0,r=t.length;n<r;n++)t[n].updateMatrixWorld(e)}updateWorldMatrix(e,t,n=!1){let r=this.parent;if(e===!0&&r!==null&&r.updateWorldMatrix(!0,!1),this.matrixAutoUpdate&&this.updateMatrix(),(this.matrixWorldNeedsUpdate||n)&&(this.matrixWorldAutoUpdate===!0&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix)),this.matrixWorldNeedsUpdate=!1,n=!0),t===!0){let e=this.children;for(let t=0,r=e.length;t<r;t++)e[t].updateWorldMatrix(!1,!0,n)}}toJSON(e){let t=e===void 0||typeof e==`string`,n={};t&&(e={geometries:{},materials:{},textures:{},images:{},shapes:{},skeletons:{},animations:{},nodes:{}},n.metadata={version:4.7,type:`Object`,generator:`Object3D.toJSON`});let r={};r.uuid=this.uuid,r.type=this.type,r.name=this.name,r.castShadow=this.castShadow,r.receiveShadow=this.receiveShadow,r.visible=this.visible,r.frustumCulled=this.frustumCulled,r.renderOrder=this.renderOrder,r.static=this.static,r.matrixAutoUpdate=this.matrixAutoUpdate,Object.keys(this.userData).length>0&&(r.userData=this.userData),r.layers=this.layers.mask,r.matrix=this.matrix.toArray(),r.up=this.up.toArray(),this.pivot!==null&&(r.pivot=this.pivot.toArray()),this.morphTargetDictionary!==void 0&&(r.morphTargetDictionary=Object.assign({},this.morphTargetDictionary)),this.morphTargetInfluences!==void 0&&(r.morphTargetInfluences=this.morphTargetInfluences.slice()),this.isInstancedMesh&&(r.type=`InstancedMesh`,r.count=this.count,r.instanceMatrix=this.instanceMatrix.toJSON(),this.instanceColor!==null&&(r.instanceColor=this.instanceColor.toJSON())),this.isBatchedMesh&&(r.type=`BatchedMesh`,r.perObjectFrustumCulled=this.perObjectFrustumCulled,r.sortObjects=this.sortObjects,r.drawRanges=this._drawRanges,r.reservedRanges=this._reservedRanges,r.geometryInfo=this._geometryInfo.map(e=>({...e,boundingBox:e.boundingBox?e.boundingBox.toJSON():void 0,boundingSphere:e.boundingSphere?e.boundingSphere.toJSON():void 0})),r.instanceInfo=this._instanceInfo.map(e=>({...e})),r.availableInstanceIds=this._availableInstanceIds.slice(),r.availableGeometryIds=this._availableGeometryIds.slice(),r.nextIndexStart=this._nextIndexStart,r.nextVertexStart=this._nextVertexStart,r.geometryCount=this._geometryCount,r.maxInstanceCount=this._maxInstanceCount,r.maxVertexCount=this._maxVertexCount,r.maxIndexCount=this._maxIndexCount,r.geometryInitialized=this._geometryInitialized,r.matricesTexture=this._matricesTexture.toJSON(e),r.indirectTexture=this._indirectTexture.toJSON(e),this._colorsTexture!==null&&(r.colorsTexture=this._colorsTexture.toJSON(e)),this.boundingSphere!==null&&(r.boundingSphere=this.boundingSphere.toJSON()),this.boundingBox!==null&&(r.boundingBox=this.boundingBox.toJSON()));function i(t,n){return t[n.uuid]===void 0&&(t[n.uuid]=n.toJSON(e)),n.uuid}if(this.isScene)this.background&&(this.background.isColor?r.background=this.background.toJSON():this.background.isTexture&&(r.background=this.background.toJSON(e).uuid)),this.environment&&this.environment.isTexture&&this.environment.isRenderTargetTexture!==!0&&(r.environment=this.environment.toJSON(e).uuid);else if(this.isMesh||this.isLine||this.isPoints){r.geometry=i(e.geometries,this.geometry);let t=this.geometry.parameters;if(t!==void 0&&t.shapes!==void 0){let n=t.shapes;if(Array.isArray(n))for(let t=0,r=n.length;t<r;t++){let r=n[t];i(e.shapes,r)}else i(e.shapes,n)}}if(this.isSkinnedMesh&&(r.bindMode=this.bindMode,r.bindMatrix=this.bindMatrix.toArray(),this.skeleton!==void 0&&(i(e.skeletons,this.skeleton),r.skeleton=this.skeleton.uuid)),this.material!==void 0){if(Array.isArray(this.material)){let t=[];for(let n=0,r=this.material.length;n<r;n++)t.push(i(e.materials,this.material[n]));r.material=t}else r.material=i(e.materials,this.material)}if(this.children.length>0){r.children=[];for(let t=0;t<this.children.length;t++)r.children.push(this.children[t].toJSON(e).object)}if(this.animations.length>0){r.animations=[];for(let t=0;t<this.animations.length;t++){let n=this.animations[t];r.animations.push(i(e.animations,n))}}if(t){let t=a(e.geometries),r=a(e.materials),i=a(e.textures),o=a(e.images),s=a(e.shapes),c=a(e.skeletons),l=a(e.animations),u=a(e.nodes);t.length>0&&(n.geometries=t),r.length>0&&(n.materials=r),i.length>0&&(n.textures=i),o.length>0&&(n.images=o),s.length>0&&(n.shapes=s),c.length>0&&(n.skeletons=c),l.length>0&&(n.animations=l),u.length>0&&(n.nodes=u)}return n.object=r,n;function a(e){let t=[];for(let n in e){let r=e[n];delete r.metadata,t.push(r)}return t}}clone(e){return new this.constructor().copy(this,e)}copy(e,t=!0){if(this.name=e.name,this.up.copy(e.up),this.position.copy(e.position),this.rotation.order=e.rotation.order,this.quaternion.copy(e.quaternion),this.scale.copy(e.scale),this.pivot=e.pivot===null?null:e.pivot.clone(),this.matrix.copy(e.matrix),this.matrixWorld.copy(e.matrixWorld),this.matrixAutoUpdate=e.matrixAutoUpdate,this.matrixWorldAutoUpdate=e.matrixWorldAutoUpdate,this.matrixWorldNeedsUpdate=e.matrixWorldNeedsUpdate,this.layers.mask=e.layers.mask,this.visible=e.visible,this.castShadow=e.castShadow,this.receiveShadow=e.receiveShadow,this.frustumCulled=e.frustumCulled,this.renderOrder=e.renderOrder,this.static=e.static,this.animations=e.animations.slice(),this.userData=JSON.parse(JSON.stringify(e.userData)),t===!0)for(let t=0;t<e.children.length;t++){let n=e.children[t];this.add(n.clone())}return this}dispose(){this.dispatchEvent({type:`dispose`})}};hr.DEFAULT_UP=new q(0,1,0),hr.DEFAULT_MATRIX_AUTO_UPDATE=!0,hr.DEFAULT_MATRIX_WORLD_AUTO_UPDATE=!0;var gr=class extends hr{constructor(){super(),this.isGroup=!0,this.type=`Group`}},_r={type:`move`},vr=class{constructor(){this._targetRay=null,this._grip=null,this._hand=null}getHandSpace(){return this._hand===null&&(this._hand=new gr,this._hand.matrixAutoUpdate=!1,this._hand.visible=!1,this._hand.joints={},this._hand.inputState={pinching:!1}),this._hand}getTargetRaySpace(){return this._targetRay===null&&(this._targetRay=new gr,this._targetRay.matrixAutoUpdate=!1,this._targetRay.visible=!1,this._targetRay.hasLinearVelocity=!1,this._targetRay.linearVelocity=new q,this._targetRay.hasAngularVelocity=!1,this._targetRay.angularVelocity=new q),this._targetRay}getGripSpace(){return this._grip===null&&(this._grip=new gr,this._grip.matrixAutoUpdate=!1,this._grip.visible=!1,this._grip.hasLinearVelocity=!1,this._grip.linearVelocity=new q,this._grip.hasAngularVelocity=!1,this._grip.angularVelocity=new q,this._grip.eventsEnabled=!1),this._grip}dispatchEvent(e){return this._targetRay!==null&&this._targetRay.dispatchEvent(e),this._grip!==null&&this._grip.dispatchEvent(e),this._hand!==null&&this._hand.dispatchEvent(e),this}connect(e){if(e&&e.hand){let t=this._hand;if(t)for(let n of e.hand.values())this._getHandJoint(t,n)}return this.dispatchEvent({type:`connected`,data:e}),this}disconnect(e){return this.dispatchEvent({type:`disconnected`,data:e}),this._targetRay!==null&&(this._targetRay.visible=!1),this._grip!==null&&(this._grip.visible=!1),this._hand!==null&&(this._hand.visible=!1),this}update(e,t,n){let r=null,i=null,a=null,o=this._targetRay,s=this._grip,c=this._hand;if(e&&t.session.visibilityState!==`visible-blurred`){if(c&&e.hand){a=!0;for(let r of e.hand.values()){let e=t.getJointPose(r,n),i=this._getHandJoint(c,r);e!==null&&(i.matrix.fromArray(e.transform.matrix),i.matrix.decompose(i.position,i.rotation,i.scale),i.matrixWorldNeedsUpdate=!0,i.jointRadius=e.radius),i.visible=e!==null}let r=c.joints[`index-finger-tip`],i=c.joints[`thumb-tip`],o=r.position.distanceTo(i.position);c.inputState.pinching&&o>.025?(c.inputState.pinching=!1,this.dispatchEvent({type:`pinchend`,handedness:e.handedness,target:this})):!c.inputState.pinching&&o<=.015&&(c.inputState.pinching=!0,this.dispatchEvent({type:`pinchstart`,handedness:e.handedness,target:this}))}else s!==null&&e.gripSpace&&(i=t.getPose(e.gripSpace,n),i!==null&&(s.matrix.fromArray(i.transform.matrix),s.matrix.decompose(s.position,s.rotation,s.scale),s.matrixWorldNeedsUpdate=!0,i.linearVelocity?(s.hasLinearVelocity=!0,s.linearVelocity.copy(i.linearVelocity)):s.hasLinearVelocity=!1,i.angularVelocity?(s.hasAngularVelocity=!0,s.angularVelocity.copy(i.angularVelocity)):s.hasAngularVelocity=!1,s.eventsEnabled&&s.dispatchEvent({type:`gripUpdated`,data:e,target:this})));o!==null&&(r=t.getPose(e.targetRaySpace,n),r===null&&i!==null&&(r=i),r!==null&&(o.matrix.fromArray(r.transform.matrix),o.matrix.decompose(o.position,o.rotation,o.scale),o.matrixWorldNeedsUpdate=!0,r.linearVelocity?(o.hasLinearVelocity=!0,o.linearVelocity.copy(r.linearVelocity)):o.hasLinearVelocity=!1,r.angularVelocity?(o.hasAngularVelocity=!0,o.angularVelocity.copy(r.angularVelocity)):o.hasAngularVelocity=!1,this.dispatchEvent(_r)))}return o!==null&&(o.visible=r!==null),s!==null&&(s.visible=i!==null),c!==null&&(c.visible=a!==null),this}_getHandJoint(e,t){if(e.joints[t.jointName]===void 0){let n=new gr;n.matrixAutoUpdate=!1,n.visible=!1,e.joints[t.jointName]=n,e.add(n)}return e.joints[t.jointName]}},yr={aliceblue:15792383,antiquewhite:16444375,aqua:65535,aquamarine:8388564,azure:15794175,beige:16119260,bisque:16770244,black:0,blanchedalmond:16772045,blue:255,blueviolet:9055202,brown:10824234,burlywood:14596231,cadetblue:6266528,chartreuse:8388352,chocolate:13789470,coral:16744272,cornflowerblue:6591981,cornsilk:16775388,crimson:14423100,cyan:65535,darkblue:139,darkcyan:35723,darkgoldenrod:12092939,darkgray:11119017,darkgreen:25600,darkgrey:11119017,darkkhaki:12433259,darkmagenta:9109643,darkolivegreen:5597999,darkorange:16747520,darkorchid:10040012,darkred:9109504,darksalmon:15308410,darkseagreen:9419919,darkslateblue:4734347,darkslategray:3100495,darkslategrey:3100495,darkturquoise:52945,darkviolet:9699539,deeppink:16716947,deepskyblue:49151,dimgray:6908265,dimgrey:6908265,dodgerblue:2003199,firebrick:11674146,floralwhite:16775920,forestgreen:2263842,fuchsia:16711935,gainsboro:14474460,ghostwhite:16316671,gold:16766720,goldenrod:14329120,gray:8421504,green:32768,greenyellow:11403055,grey:8421504,honeydew:15794160,hotpink:16738740,indianred:13458524,indigo:4915330,ivory:16777200,khaki:15787660,lavender:15132410,lavenderblush:16773365,lawngreen:8190976,lemonchiffon:16775885,lightblue:11393254,lightcoral:15761536,lightcyan:14745599,lightgoldenrodyellow:16448210,lightgray:13882323,lightgreen:9498256,lightgrey:13882323,lightpink:16758465,lightsalmon:16752762,lightseagreen:2142890,lightskyblue:8900346,lightslategray:7833753,lightslategrey:7833753,lightsteelblue:11584734,lightyellow:16777184,lime:65280,limegreen:3329330,linen:16445670,magenta:16711935,maroon:8388608,mediumaquamarine:6737322,mediumblue:205,mediumorchid:12211667,mediumpurple:9662683,mediumseagreen:3978097,mediumslateblue:8087790,mediumspringgreen:64154,mediumturquoise:4772300,mediumvioletred:13047173,midnightblue:1644912,mintcream:16121850,mistyrose:16770273,moccasin:16770229,navajowhite:16768685,navy:128,oldlace:16643558,olive:8421376,olivedrab:7048739,orange:16753920,orangered:16729344,orchid:14315734,palegoldenrod:15657130,palegreen:10025880,paleturquoise:11529966,palevioletred:14381203,papayawhip:16773077,peachpuff:16767673,peru:13468991,pink:16761035,plum:14524637,powderblue:11591910,purple:8388736,rebeccapurple:6697881,red:16711680,rosybrown:12357519,royalblue:4286945,saddlebrown:9127187,salmon:16416882,sandybrown:16032864,seagreen:3050327,seashell:16774638,sienna:10506797,silver:12632256,skyblue:8900331,slateblue:6970061,slategray:7372944,slategrey:7372944,snow:16775930,springgreen:65407,steelblue:4620980,tan:13808780,teal:32896,thistle:14204888,tomato:16737095,turquoise:4251856,violet:15631086,wheat:16113331,white:16777215,whitesmoke:16119285,yellow:16776960,yellowgreen:10145074},br={h:0,s:0,l:0},xr={h:0,s:0,l:0};function Sr(e,t,n){return n<0&&(n+=1),n>1&&--n,n<1/6?e+(t-e)*6*n:n<1/2?t:n<2/3?e+(t-e)*6*(2/3-n):e}var Y=class{constructor(e,t,n){return this.isColor=!0,this.r=1,this.g=1,this.b=1,this.set(e,t,n)}set(e,t,n){if(t===void 0&&n===void 0){let t=e;t&&t.isColor?this.copy(t):typeof t==`number`?this.setHex(t):typeof t==`string`&&this.setStyle(t)}else this.setRGB(e,t,n);return this}setScalar(e){return this.r=e,this.g=e,this.b=e,this}setHex(e,t=$e){return e=Math.floor(e),this.r=(e>>16&255)/255,this.g=(e>>8&255)/255,this.b=(e&255)/255,wn.colorSpaceToWorking(this,t),this}setRGB(e,t,n,r=wn.workingColorSpace){return this.r=e,this.g=t,this.b=n,wn.colorSpaceToWorking(this,r),this}setHSL(e,t,n,r=wn.workingColorSpace){if(e=Jt(e,1),t=G(t,0,1),n=G(n,0,1),t===0)this.r=this.g=this.b=n;else{let r=n<=.5?n*(1+t):n+t-n*t,i=2*n-r;this.r=Sr(i,r,e+1/3),this.g=Sr(i,r,e),this.b=Sr(i,r,e-1/3)}return wn.colorSpaceToWorking(this,r),this}setStyle(e,t=$e){function n(t){t!==void 0&&parseFloat(t)<1&&U(`Color: Alpha component of `+e+` will be ignored.`)}let r;if(r=/^(\w+)\(([^\)]*)\)/.exec(e)){let i,a=r[1],o=r[2];switch(a){case`rgb`:case`rgba`:if(i=/^\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return n(i[4]),this.setRGB(Math.min(255,parseInt(i[1],10))/255,Math.min(255,parseInt(i[2],10))/255,Math.min(255,parseInt(i[3],10))/255,t);if(i=/^\s*(\d+)\%\s*,\s*(\d+)\%\s*,\s*(\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return n(i[4]),this.setRGB(Math.min(100,parseInt(i[1],10))/100,Math.min(100,parseInt(i[2],10))/100,Math.min(100,parseInt(i[3],10))/100,t);break;case`hsl`:case`hsla`:if(i=/^\s*(\d*\.?\d+)\s*,\s*(\d*\.?\d+)\%\s*,\s*(\d*\.?\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return n(i[4]),this.setHSL(parseFloat(i[1])/360,parseFloat(i[2])/100,parseFloat(i[3])/100,t);break;default:U(`Color: Unknown color model `+e)}}else if(r=/^\#([A-Fa-f\d]+)$/.exec(e)){let n=r[1],i=n.length;if(i===3)return this.setRGB(parseInt(n.charAt(0),16)/15,parseInt(n.charAt(1),16)/15,parseInt(n.charAt(2),16)/15,t);if(i===6)return this.setHex(parseInt(n,16),t);U(`Color: Invalid hex color `+e)}else if(e&&e.length>0)return this.setColorName(e,t);return this}setColorName(e,t=$e){let n=yr[e.toLowerCase()];return n===void 0?U(`Color: Unknown color `+e):this.setHex(n,t),this}clone(){return new this.constructor(this.r,this.g,this.b)}copy(e){return this.r=e.r,this.g=e.g,this.b=e.b,this}copySRGBToLinear(e){return this.r=Tn(e.r),this.g=Tn(e.g),this.b=Tn(e.b),this}copyLinearToSRGB(e){return this.r=En(e.r),this.g=En(e.g),this.b=En(e.b),this}convertSRGBToLinear(){return this.copySRGBToLinear(this),this}convertLinearToSRGB(){return this.copyLinearToSRGB(this),this}getHex(e=$e){return wn.workingToColorSpace(Cr.copy(this),e),Math.round(G(Cr.r*255,0,255))*65536+Math.round(G(Cr.g*255,0,255))*256+Math.round(G(Cr.b*255,0,255))}getHexString(e=$e){return(`000000`+this.getHex(e).toString(16)).slice(-6)}getHSL(e,t=wn.workingColorSpace){wn.workingToColorSpace(Cr.copy(this),t);let n=Cr.r,r=Cr.g,i=Cr.b,a=Math.max(n,r,i),o=Math.min(n,r,i),s,c,l=(o+a)/2;if(o===a)s=0,c=0;else{let e=a-o;switch(c=l<=.5?e/(a+o):e/(2-a-o),a){case n:s=(r-i)/e+(r<i?6:0);break;case r:s=(i-n)/e+2;break;case i:s=(n-r)/e+4}s/=6}return e.h=s,e.s=c,e.l=l,e}getRGB(e,t=wn.workingColorSpace){return wn.workingToColorSpace(Cr.copy(this),t),e.r=Cr.r,e.g=Cr.g,e.b=Cr.b,e}getStyle(e=$e){wn.workingToColorSpace(Cr.copy(this),e);let t=Cr.r,n=Cr.g,r=Cr.b;return e===`srgb`?`rgb(${Math.round(t*255)},${Math.round(n*255)},${Math.round(r*255)})`:`color(${e} ${t.toFixed(3)} ${n.toFixed(3)} ${r.toFixed(3)})`}offsetHSL(e,t,n){return this.getHSL(br),this.setHSL(br.h+e,br.s+t,br.l+n)}add(e){return this.r+=e.r,this.g+=e.g,this.b+=e.b,this}addColors(e,t){return this.r=e.r+t.r,this.g=e.g+t.g,this.b=e.b+t.b,this}addScalar(e){return this.r+=e,this.g+=e,this.b+=e,this}sub(e){return this.r=Math.max(0,this.r-e.r),this.g=Math.max(0,this.g-e.g),this.b=Math.max(0,this.b-e.b),this}multiply(e){return this.r*=e.r,this.g*=e.g,this.b*=e.b,this}multiplyScalar(e){return this.r*=e,this.g*=e,this.b*=e,this}lerp(e,t){return this.r+=(e.r-this.r)*t,this.g+=(e.g-this.g)*t,this.b+=(e.b-this.b)*t,this}lerpColors(e,t,n){return this.r=e.r+(t.r-e.r)*n,this.g=e.g+(t.g-e.g)*n,this.b=e.b+(t.b-e.b)*n,this}lerpHSL(e,t){this.getHSL(br),e.getHSL(xr);let n=Zt(br.h,xr.h,t),r=Zt(br.s,xr.s,t),i=Zt(br.l,xr.l,t);return this.setHSL(n,r,i),this}setFromVector3(e){return this.r=e.x,this.g=e.y,this.b=e.z,this}applyMatrix3(e){let t=this.r,n=this.g,r=this.b,i=e.elements;return this.r=i[0]*t+i[3]*n+i[6]*r,this.g=i[1]*t+i[4]*n+i[7]*r,this.b=i[2]*t+i[5]*n+i[8]*r,this}equals(e){return e.r===this.r&&e.g===this.g&&e.b===this.b}fromArray(e,t=0){return this.r=e[t],this.g=e[t+1],this.b=e[t+2],this}toArray(e=[],t=0){return e[t]=this.r,e[t+1]=this.g,e[t+2]=this.b,e}fromBufferAttribute(e,t){return this.r=e.getX(t),this.g=e.getY(t),this.b=e.getZ(t),this}toJSON(){return this.getHex()}*[Symbol.iterator](){yield this.r,yield this.g,yield this.b}},Cr=new Y;Y.NAMES=yr;var wr=class e{constructor(e,t=25e-5){this.isFogExp2=!0,this.name=``,this.color=new Y(e),this.density=t}clone(){return new e(this.color,this.density)}toJSON(){return{type:`FogExp2`,name:this.name,color:this.color.getHex(),density:this.density}}},Tr=class e{constructor(e,t=1,n=1e3){this.isFog=!0,this.name=``,this.color=new Y(e),this.near=t,this.far=n}clone(){return new e(this.color,this.near,this.far)}toJSON(){return{type:`Fog`,name:this.name,color:this.color.getHex(),near:this.near,far:this.far}}},Er=class extends hr{constructor(){super(),this.isScene=!0,this.type=`Scene`,this.background=null,this.environment=null,this.fog=null,this.backgroundBlurriness=0,this.backgroundIntensity=1,this.backgroundRotation=new Qn,this.environmentIntensity=1,this.environmentRotation=new Qn,this.overrideMaterial=null,typeof __THREE_DEVTOOLS__<`u`&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent(`observe`,{detail:this}))}copy(e,t){return super.copy(e,t),e.background!==null&&(this.background=e.background.clone()),e.environment!==null&&(this.environment=e.environment.clone()),e.fog!==null&&(this.fog=e.fog.clone()),this.backgroundBlurriness=e.backgroundBlurriness,this.backgroundIntensity=e.backgroundIntensity,this.backgroundRotation.copy(e.backgroundRotation),this.environmentIntensity=e.environmentIntensity,this.environmentRotation.copy(e.environmentRotation),e.overrideMaterial!==null&&(this.overrideMaterial=e.overrideMaterial.clone()),this.matrixAutoUpdate=e.matrixAutoUpdate,this}toJSON(e){let t=super.toJSON(e);return this.fog!==null&&(t.object.fog=this.fog.toJSON()),t.object.backgroundBlurriness=this.backgroundBlurriness,t.object.backgroundIntensity=this.backgroundIntensity,t.object.backgroundRotation=this.backgroundRotation.toArray(),t.object.environmentIntensity=this.environmentIntensity,t.object.environmentRotation=this.environmentRotation.toArray(),t}},Dr=new q,Or=new q,kr=new q,Ar=new q,jr=new q,Mr=new q,Nr=new q,Pr=new q,Fr=new q,Ir=new q,Lr=new In,Rr=new In,zr=new In,Br=class e{constructor(e=new q,t=new q,n=new q){this.a=e,this.b=t,this.c=n}static getNormal(e,t,n,r){r.subVectors(n,t),Dr.subVectors(e,t),r.cross(Dr);let i=r.lengthSq();return i>0?r.multiplyScalar(1/Math.sqrt(i)):r.set(0,0,0)}static getBarycoord(e,t,n,r,i){Dr.subVectors(r,t),Or.subVectors(n,t),kr.subVectors(e,t);let a=Dr.dot(Dr),o=Dr.dot(Or),s=Dr.dot(kr),c=Or.dot(Or),l=Or.dot(kr),u=a*c-o*o;if(u===0)return i.set(0,0,0),null;let d=1/u,f=(c*s-o*l)*d,p=(a*l-o*s)*d;return i.set(1-f-p,p,f)}static containsPoint(e,t,n,r){return this.getBarycoord(e,t,n,r,Ar)!==null&&Ar.x>=0&&Ar.y>=0&&Ar.x+Ar.y<=1}static getInterpolation(e,t,n,r,i,a,o,s){return this.getBarycoord(e,t,n,r,Ar)===null?(s.x=0,s.y=0,`z`in s&&(s.z=0),`w`in s&&(s.w=0),null):(s.setScalar(0),s.addScaledVector(i,Ar.x),s.addScaledVector(a,Ar.y),s.addScaledVector(o,Ar.z),s)}static getInterpolatedAttribute(e,t,n,r,i,a){return Lr.setScalar(0),Rr.setScalar(0),zr.setScalar(0),Lr.fromBufferAttribute(e,t),Rr.fromBufferAttribute(e,n),zr.fromBufferAttribute(e,r),a.setScalar(0),a.addScaledVector(Lr,i.x),a.addScaledVector(Rr,i.y),a.addScaledVector(zr,i.z),a}static isFrontFacing(e,t,n,r){return Dr.subVectors(n,t),Or.subVectors(e,t),Dr.cross(Or).dot(r)<0}set(e,t,n){return this.a.copy(e),this.b.copy(t),this.c.copy(n),this}setFromPointsAndIndices(e,t,n,r){return this.a.copy(e[t]),this.b.copy(e[n]),this.c.copy(e[r]),this}setFromAttributeAndIndices(e,t,n,r){return this.a.fromBufferAttribute(e,t),this.b.fromBufferAttribute(e,n),this.c.fromBufferAttribute(e,r),this}clone(){return new this.constructor().copy(this)}copy(e){return this.a.copy(e.a),this.b.copy(e.b),this.c.copy(e.c),this}getArea(){return Dr.subVectors(this.c,this.b),Or.subVectors(this.a,this.b),Dr.cross(Or).length()*.5}getMidpoint(e){return e.addVectors(this.a,this.b).add(this.c).multiplyScalar(1/3)}getNormal(t){return e.getNormal(this.a,this.b,this.c,t)}getPlane(e){return e.setFromCoplanarPoints(this.a,this.b,this.c)}getBarycoord(t,n){return e.getBarycoord(t,this.a,this.b,this.c,n)}getInterpolation(t,n,r,i,a){return e.getInterpolation(t,this.a,this.b,this.c,n,r,i,a)}containsPoint(t){return e.containsPoint(t,this.a,this.b,this.c)}isFrontFacing(t){return e.isFrontFacing(this.a,this.b,this.c,t)}intersectsBox(e){return e.intersectsTriangle(this)}closestPointToPoint(e,t){let n=this.a,r=this.b,i=this.c,a,o;jr.subVectors(r,n),Mr.subVectors(i,n),Pr.subVectors(e,n);let s=jr.dot(Pr),c=Mr.dot(Pr);if(s<=0&&c<=0)return t.copy(n);Fr.subVectors(e,r);let l=jr.dot(Fr),u=Mr.dot(Fr);if(l>=0&&u<=l)return t.copy(r);let d=s*u-l*c;if(d<=0&&s>=0&&l<=0)return a=s/(s-l),t.copy(n).addScaledVector(jr,a);Ir.subVectors(e,i);let f=jr.dot(Ir),p=Mr.dot(Ir);if(p>=0&&f<=p)return t.copy(i);let m=f*c-s*p;if(m<=0&&c>=0&&p<=0)return o=c/(c-p),t.copy(n).addScaledVector(Mr,o);let h=l*p-f*u;if(h<=0&&u-l>=0&&f-p>=0)return Nr.subVectors(i,r),o=(u-l)/(u-l+(f-p)),t.copy(r).addScaledVector(Nr,o);let g=1/(h+m+d);return a=m*g,o=d*g,t.copy(n).addScaledVector(jr,a).addScaledVector(Mr,o)}equals(e){return e.a.equals(this.a)&&e.b.equals(this.b)&&e.c.equals(this.c)}},Vr=class{constructor(e=new q(1/0,1/0,1/0),t=new q(-1/0,-1/0,-1/0)){this.isBox3=!0,this.min=e,this.max=t}set(e,t){return this.min.copy(e),this.max.copy(t),this}setFromArray(e){this.makeEmpty();for(let t=0,n=e.length;t<n;t+=3)this.expandByPoint(Ur.fromArray(e,t));return this}setFromBufferAttribute(e){this.makeEmpty();for(let t=0,n=e.count;t<n;t++)this.expandByPoint(Ur.fromBufferAttribute(e,t));return this}setFromPoints(e){this.makeEmpty();for(let t=0,n=e.length;t<n;t++)this.expandByPoint(e[t]);return this}setFromCenterAndSize(e,t){let n=Ur.copy(t).multiplyScalar(.5);return this.min.copy(e).sub(n),this.max.copy(e).add(n),this}setFromObject(e,t=!1){return this.makeEmpty(),this.expandByObject(e,t)}clone(){return new this.constructor().copy(this)}copy(e){return this.min.copy(e.min),this.max.copy(e.max),this}makeEmpty(){return this.min.x=this.min.y=this.min.z=1/0,this.max.x=this.max.y=this.max.z=-1/0,this}isEmpty(){return this.max.x<this.min.x||this.max.y<this.min.y||this.max.z<this.min.z}getCenter(e){return this.isEmpty()?e.set(0,0,0):e.addVectors(this.min,this.max).multiplyScalar(.5)}getSize(e){return this.isEmpty()?e.set(0,0,0):e.subVectors(this.max,this.min)}expandByPoint(e){return this.min.min(e),this.max.max(e),this}expandByVector(e){return this.min.sub(e),this.max.add(e),this}expandByScalar(e){return this.min.addScalar(-e),this.max.addScalar(e),this}expandByObject(e,t=!1){e.updateWorldMatrix(!1,!1);let n=e.geometry;if(n!==void 0){let r=n.getAttribute(`position`);if(t===!0&&r!==void 0&&e.isInstancedMesh!==!0)for(let t=0,n=r.count;t<n;t++)e.isMesh===!0?e.getVertexPosition(t,Ur):Ur.fromBufferAttribute(r,t),Ur.applyMatrix4(e.matrixWorld),this.expandByPoint(Ur);else e.boundingBox===void 0?(n.boundingBox===null&&n.computeBoundingBox(),Wr.copy(n.boundingBox)):(e.boundingBox===null&&e.computeBoundingBox(),Wr.copy(e.boundingBox)),Wr.applyMatrix4(e.matrixWorld),this.union(Wr)}let r=e.children;for(let e=0,n=r.length;e<n;e++)this.expandByObject(r[e],t);return this}containsPoint(e){return e.x>=this.min.x&&e.x<=this.max.x&&e.y>=this.min.y&&e.y<=this.max.y&&e.z>=this.min.z&&e.z<=this.max.z}containsBox(e){return this.min.x<=e.min.x&&e.max.x<=this.max.x&&this.min.y<=e.min.y&&e.max.y<=this.max.y&&this.min.z<=e.min.z&&e.max.z<=this.max.z}getParameter(e,t){return t.set((e.x-this.min.x)/(this.max.x-this.min.x),(e.y-this.min.y)/(this.max.y-this.min.y),(e.z-this.min.z)/(this.max.z-this.min.z))}intersectsBox(e){return e.max.x>=this.min.x&&e.min.x<=this.max.x&&e.max.y>=this.min.y&&e.min.y<=this.max.y&&e.max.z>=this.min.z&&e.min.z<=this.max.z}intersectsSphere(e){return this.clampPoint(e.center,Ur),Ur.distanceToSquared(e.center)<=e.radius*e.radius}intersectsPlane(e){let t,n;return e.normal.x>0?(t=e.normal.x*this.min.x,n=e.normal.x*this.max.x):(t=e.normal.x*this.max.x,n=e.normal.x*this.min.x),e.normal.y>0?(t+=e.normal.y*this.min.y,n+=e.normal.y*this.max.y):(t+=e.normal.y*this.max.y,n+=e.normal.y*this.min.y),e.normal.z>0?(t+=e.normal.z*this.min.z,n+=e.normal.z*this.max.z):(t+=e.normal.z*this.max.z,n+=e.normal.z*this.min.z),t<=-e.constant&&n>=-e.constant}intersectsTriangle(e){if(this.isEmpty())return!1;this.getCenter(Zr),Qr.subVectors(this.max,Zr),Gr.subVectors(e.a,Zr),Kr.subVectors(e.b,Zr),qr.subVectors(e.c,Zr),Jr.subVectors(Kr,Gr),Yr.subVectors(qr,Kr),Xr.subVectors(Gr,qr);let t=[0,-Jr.z,Jr.y,0,-Yr.z,Yr.y,0,-Xr.z,Xr.y,Jr.z,0,-Jr.x,Yr.z,0,-Yr.x,Xr.z,0,-Xr.x,-Jr.y,Jr.x,0,-Yr.y,Yr.x,0,-Xr.y,Xr.x,0];return!ti(t,Gr,Kr,qr,Qr)||(t=[1,0,0,0,1,0,0,0,1],!ti(t,Gr,Kr,qr,Qr))?!1:($r.crossVectors(Jr,Yr),t=[$r.x,$r.y,$r.z],ti(t,Gr,Kr,qr,Qr))}clampPoint(e,t){return t.copy(e).clamp(this.min,this.max)}distanceToPoint(e){return this.clampPoint(e,Ur).distanceTo(e)}getBoundingSphere(e){return this.isEmpty()?e.makeEmpty():(this.getCenter(e.center),e.radius=this.getSize(Ur).length()*.5),e}intersect(e){return this.min.max(e.min),this.max.min(e.max),this.isEmpty()&&this.makeEmpty(),this}union(e){return this.min.min(e.min),this.max.max(e.max),this}applyMatrix4(e){return this.isEmpty()||(Hr[0].set(this.min.x,this.min.y,this.min.z).applyMatrix4(e),Hr[1].set(this.min.x,this.min.y,this.max.z).applyMatrix4(e),Hr[2].set(this.min.x,this.max.y,this.min.z).applyMatrix4(e),Hr[3].set(this.min.x,this.max.y,this.max.z).applyMatrix4(e),Hr[4].set(this.max.x,this.min.y,this.min.z).applyMatrix4(e),Hr[5].set(this.max.x,this.min.y,this.max.z).applyMatrix4(e),Hr[6].set(this.max.x,this.max.y,this.min.z).applyMatrix4(e),Hr[7].set(this.max.x,this.max.y,this.max.z).applyMatrix4(e),this.setFromPoints(Hr)),this}translate(e){return this.min.add(e),this.max.add(e),this}equals(e){return e.min.equals(this.min)&&e.max.equals(this.max)}toJSON(){return{min:this.min.toArray(),max:this.max.toArray()}}fromJSON(e){return this.min.fromArray(e.min),this.max.fromArray(e.max),this}},Hr=[new q,new q,new q,new q,new q,new q,new q,new q],Ur=new q,Wr=new Vr,Gr=new q,Kr=new q,qr=new q,Jr=new q,Yr=new q,Xr=new q,Zr=new q,Qr=new q,$r=new q,ei=new q;function ti(e,t,n,r,i){for(let a=0,o=e.length-3;a<=o;a+=3){ei.fromArray(e,a);let o=i.x*Math.abs(ei.x)+i.y*Math.abs(ei.y)+i.z*Math.abs(ei.z),s=t.dot(ei),c=n.dot(ei),l=r.dot(ei);if(Math.max(-Math.max(s,c,l),Math.min(s,c,l))>o)return!1}return!0}var ni=ri();function ri(){let e=new ArrayBuffer(4),t=new Float32Array(e),n=new Uint32Array(e),r=new Uint32Array(512),i=new Uint32Array(512);for(let e=0;e<256;++e){let t=e-127;t<-27?(r[e]=0,r[e|256]=32768,i[e]=24,i[e|256]=24):t<-14?(r[e]=1024>>-t-14,r[e|256]=1024>>-t-14|32768,i[e]=-t-1,i[e|256]=-t-1):t<=15?(r[e]=t+15<<10,r[e|256]=t+15<<10|32768,i[e]=13,i[e|256]=13):t<128?(r[e]=31744,r[e|256]=64512,i[e]=24,i[e|256]=24):(r[e]=31744,r[e|256]=64512,i[e]=13,i[e|256]=13)}let a=new Uint32Array(2048),o=new Uint32Array(64),s=new Uint32Array(64);for(let e=1;e<1024;++e){let t=e<<13,n=0;for(;!(t&8388608);)t<<=1,n-=8388608;t&=-8388609,n+=947912704,a[e]=t|n}for(let e=1024;e<2048;++e)a[e]=939524096+(e-1024<<13);for(let e=1;e<31;++e)o[e]=e<<23;o[31]=1199570944,o[32]=2147483648;for(let e=33;e<63;++e)o[e]=2147483648+(e-32<<23);o[63]=3347054592;for(let e=1;e<64;++e)e!==32&&(s[e]=1024);return{floatView:t,uint32View:n,baseTable:r,shiftTable:i,mantissaTable:a,exponentTable:o,offsetTable:s}}function ii(e){Math.abs(e)>65504&&U(`DataUtils.toHalfFloat(): Value out of range.`),e=G(e,-65504,65504),ni.floatView[0]=e;let t=ni.uint32View[0],n=t>>23&511;return ni.baseTable[n]+((t&8388607)>>ni.shiftTable[n])}function ai(e){let t=e>>10;return ni.uint32View[0]=ni.mantissaTable[ni.offsetTable[t]+(e&1023)]+ni.exponentTable[t],ni.floatView[0]}var oi=class{static toHalfFloat(e){return ii(e)}static fromHalfFloat(e){return ai(e)}},si=new q,ci=new K,li=0,ui=class extends Ht{constructor(e,t,n=!1){if(super(),Array.isArray(e))throw TypeError(`THREE.BufferAttribute: array should be a Typed Array.`);this.isBufferAttribute=!0,Object.defineProperty(this,"id",{value:li++}),this.name=``,this.array=e,this.itemSize=t,this.count=e===void 0?0:e.length/t,this.normalized=n,this.usage=ut,this.updateRanges=[],this.gpuType=E,this.version=0}onUploadCallback(){}set needsUpdate(e){e===!0&&this.version++}setUsage(e){return this.usage=e,this}addUpdateRange(e,t){this.updateRanges.push({start:e,count:t})}clearUpdateRanges(){this.updateRanges.length=0}copy(e){return this.name=e.name,this.array=new e.array.constructor(e.array),this.itemSize=e.itemSize,this.count=e.count,this.normalized=e.normalized,this.usage=e.usage,this.gpuType=e.gpuType,this}copyAt(e,t,n){e*=this.itemSize,n*=t.itemSize;for(let r=0,i=this.itemSize;r<i;r++)this.array[e+r]=t.array[n+r];return this}copyArray(e){return this.array.set(e),this}applyMatrix3(e){if(this.itemSize===2)for(let t=0,n=this.count;t<n;t++)ci.fromBufferAttribute(this,t),ci.applyMatrix3(e),this.setXY(t,ci.x,ci.y);else if(this.itemSize===3)for(let t=0,n=this.count;t<n;t++)si.fromBufferAttribute(this,t),si.applyMatrix3(e),this.setXYZ(t,si.x,si.y,si.z);return this}applyMatrix4(e){for(let t=0,n=this.count;t<n;t++)si.fromBufferAttribute(this,t),si.applyMatrix4(e),this.setXYZ(t,si.x,si.y,si.z);return this}applyNormalMatrix(e){for(let t=0,n=this.count;t<n;t++)si.fromBufferAttribute(this,t),si.applyNormalMatrix(e),this.setXYZ(t,si.x,si.y,si.z);return this}transformDirection(e){for(let t=0,n=this.count;t<n;t++)si.fromBufferAttribute(this,t),si.transformDirection(e),this.setXYZ(t,si.x,si.y,si.z);return this}set(e,t=0){return this.array.set(e,t),this}getComponent(e,t){let n=this.array[e*this.itemSize+t];return this.normalized&&(n=pn(n,this.array)),n}setComponent(e,t,n){return this.normalized&&(n=mn(n,this.array)),this.array[e*this.itemSize+t]=n,this}getX(e){let t=this.array[e*this.itemSize];return this.normalized&&(t=pn(t,this.array)),t}setX(e,t){return this.normalized&&(t=mn(t,this.array)),this.array[e*this.itemSize]=t,this}getY(e){let t=this.array[e*this.itemSize+1];return this.normalized&&(t=pn(t,this.array)),t}setY(e,t){return this.normalized&&(t=mn(t,this.array)),this.array[e*this.itemSize+1]=t,this}getZ(e){let t=this.array[e*this.itemSize+2];return this.normalized&&(t=pn(t,this.array)),t}setZ(e,t){return this.normalized&&(t=mn(t,this.array)),this.array[e*this.itemSize+2]=t,this}getW(e){let t=this.array[e*this.itemSize+3];return this.normalized&&(t=pn(t,this.array)),t}setW(e,t){return this.normalized&&(t=mn(t,this.array)),this.array[e*this.itemSize+3]=t,this}setXY(e,t,n){return e*=this.itemSize,this.normalized&&(t=mn(t,this.array),n=mn(n,this.array)),this.array[e+0]=t,this.array[e+1]=n,this}setXYZ(e,t,n,r){return e*=this.itemSize,this.normalized&&(t=mn(t,this.array),n=mn(n,this.array),r=mn(r,this.array)),this.array[e+0]=t,this.array[e+1]=n,this.array[e+2]=r,this}setXYZW(e,t,n,r,i){return e*=this.itemSize,this.normalized&&(t=mn(t,this.array),n=mn(n,this.array),r=mn(r,this.array),i=mn(i,this.array)),this.array[e+0]=t,this.array[e+1]=n,this.array[e+2]=r,this.array[e+3]=i,this}onUpload(e){return this.onUploadCallback=e,this}clone(){return new this.constructor(this.array,this.itemSize).copy(this)}toJSON(){let e={itemSize:this.itemSize,type:this.array.constructor.name,array:Array.from(this.array),normalized:this.normalized};return e.name=this.name,e.usage=this.usage,e.gpuType=this.gpuType,e}dispose(){this.dispatchEvent({type:`dispose`})}},di=class extends ui{constructor(e,t,n){super(new Int8Array(e),t,n)}},fi=class extends ui{constructor(e,t,n){super(new Uint8Array(e),t,n)}},pi=class extends ui{constructor(e,t,n){super(new Uint8ClampedArray(e),t,n)}},mi=class extends ui{constructor(e,t,n){super(new Int16Array(e),t,n)}},hi=class extends ui{constructor(e,t,n){super(new Uint16Array(e),t,n)}},gi=class extends ui{constructor(e,t,n){super(new Int32Array(e),t,n)}},_i=class extends ui{constructor(e,t,n){super(new Uint32Array(e),t,n)}},vi=class extends ui{constructor(e,t,n){super(new Uint16Array(e),t,n),this.isFloat16BufferAttribute=!0}getX(e){let t=ai(this.array[e*this.itemSize]);return this.normalized&&(t=pn(t,this.array)),t}setX(e,t){return this.normalized&&(t=mn(t,this.array)),this.array[e*this.itemSize]=ii(t),this}getY(e){let t=ai(this.array[e*this.itemSize+1]);return this.normalized&&(t=pn(t,this.array)),t}setY(e,t){return this.normalized&&(t=mn(t,this.array)),this.array[e*this.itemSize+1]=ii(t),this}getZ(e){let t=ai(this.array[e*this.itemSize+2]);return this.normalized&&(t=pn(t,this.array)),t}setZ(e,t){return this.normalized&&(t=mn(t,this.array)),this.array[e*this.itemSize+2]=ii(t),this}getW(e){let t=ai(this.array[e*this.itemSize+3]);return this.normalized&&(t=pn(t,this.array)),t}setW(e,t){return this.normalized&&(t=mn(t,this.array)),this.array[e*this.itemSize+3]=ii(t),this}setXY(e,t,n){return e*=this.itemSize,this.normalized&&(t=mn(t,this.array),n=mn(n,this.array)),this.array[e+0]=ii(t),this.array[e+1]=ii(n),this}setXYZ(e,t,n,r){return e*=this.itemSize,this.normalized&&(t=mn(t,this.array),n=mn(n,this.array),r=mn(r,this.array)),this.array[e+0]=ii(t),this.array[e+1]=ii(n),this.array[e+2]=ii(r),this}setXYZW(e,t,n,r,i){return e*=this.itemSize,this.normalized&&(t=mn(t,this.array),n=mn(n,this.array),r=mn(r,this.array),i=mn(i,this.array)),this.array[e+0]=ii(t),this.array[e+1]=ii(n),this.array[e+2]=ii(r),this.array[e+3]=ii(i),this}},X=class extends ui{constructor(e,t,n){super(new Float32Array(e),t,n)}},yi=new Vr,bi=new q,xi=new q,Si=class{constructor(e=new q,t=-1){this.isSphere=!0,this.center=e,this.radius=t}set(e,t){return this.center.copy(e),this.radius=t,this}setFromPoints(e,t){let n=this.center;t===void 0?yi.setFromPoints(e).getCenter(n):n.copy(t);let r=0;for(let t=0,i=e.length;t<i;t++)r=Math.max(r,n.distanceToSquared(e[t]));return this.radius=Math.sqrt(r),this}copy(e){return this.center.copy(e.center),this.radius=e.radius,this}isEmpty(){return this.radius<0}makeEmpty(){return this.center.set(0,0,0),this.radius=-1,this}containsPoint(e){return e.distanceToSquared(this.center)<=this.radius*this.radius}distanceToPoint(e){return e.distanceTo(this.center)-this.radius}intersectsSphere(e){let t=this.radius+e.radius;return e.center.distanceToSquared(this.center)<=t*t}intersectsBox(e){return e.intersectsSphere(this)}intersectsPlane(e){return Math.abs(e.distanceToPoint(this.center))<=this.radius}clampPoint(e,t){let n=this.center.distanceToSquared(e);return t.copy(e),n>this.radius*this.radius&&(t.sub(this.center).normalize(),t.multiplyScalar(this.radius).add(this.center)),t}getBoundingBox(e){return this.isEmpty()?(e.makeEmpty(),e):(e.set(this.center,this.center),e.expandByScalar(this.radius),e)}applyMatrix4(e){return this.center.applyMatrix4(e),this.radius*=e.getMaxScaleOnAxis(),this}translate(e){return this.center.add(e),this}expandByPoint(e){if(this.isEmpty())return this.center.copy(e),this.radius=0,this;bi.subVectors(e,this.center);let t=bi.lengthSq();if(t>this.radius*this.radius){let e=Math.sqrt(t),n=(e-this.radius)*.5;this.center.addScaledVector(bi,n/e),this.radius+=n}return this}union(e){return e.isEmpty()?this:this.isEmpty()?(this.copy(e),this):(this.center.equals(e.center)===!0?this.radius=Math.max(this.radius,e.radius):(xi.subVectors(e.center,this.center).setLength(e.radius),this.expandByPoint(bi.copy(e.center).add(xi)),this.expandByPoint(bi.copy(e.center).sub(xi))),this)}equals(e){return e.center.equals(this.center)&&e.radius===this.radius}clone(){return new this.constructor().copy(this)}toJSON(){return{radius:this.radius,center:this.center.toArray()}}fromJSON(e){return this.radius=e.radius,this.center.fromArray(e.center),this}},Ci=0,wi=new J,Ti=new hr,Ei=new q,Di=new Vr,Oi=new Vr,ki=new q,Z=class e extends Ht{constructor(){super(),this.isBufferGeometry=!0,Object.defineProperty(this,"id",{value:Ci++}),this.uuid=qt(),this.name=``,this.type=`BufferGeometry`,this.index=null,this.indirect=null,this.indirectOffset=0,this.attributes={},this.morphAttributes={},this.morphTargetsRelative=!1,this.groups=[],this.boundingBox=null,this.boundingSphere=null,this.drawRange={start:0,count:1/0},this.userData={},this._transformed=!1}getIndex(){return this.index}setIndex(e){return this.index=Array.isArray(e)?new(Dt(e)?_i:hi)(e,1):e,this}setIndirect(e,t=0){return this.indirect=e,this.indirectOffset=t,this}getIndirect(){return this.indirect}getAttribute(e){return this.attributes[e]}setAttribute(e,t){return this.attributes[e]=t,this}deleteAttribute(e){return delete this.attributes[e],this}hasAttribute(e){return this.attributes[e]!==void 0}addGroup(e,t,n=0){this.groups.push({start:e,count:t,materialIndex:n})}clearGroups(){this.groups=[]}setDrawRange(e,t){this.drawRange.start=e,this.drawRange.count=t}applyMatrix4(e){let t=this.attributes.position;t!==void 0&&(t.applyMatrix4(e),t.needsUpdate=!0);let n=this.attributes.normal;if(n!==void 0){let t=new yn().getNormalMatrix(e);n.applyNormalMatrix(t),n.needsUpdate=!0}let r=this.attributes.tangent;return r!==void 0&&(r.transformDirection(e),r.needsUpdate=!0),this.boundingBox!==null&&this.computeBoundingBox(),this.boundingSphere!==null&&this.computeBoundingSphere(),this._transformed=!0,this}applyQuaternion(e){return wi.makeRotationFromQuaternion(e),this.applyMatrix4(wi),this}rotateX(e){return wi.makeRotationX(e),this.applyMatrix4(wi),this}rotateY(e){return wi.makeRotationY(e),this.applyMatrix4(wi),this}rotateZ(e){return wi.makeRotationZ(e),this.applyMatrix4(wi),this}translate(e,t,n){return wi.makeTranslation(e,t,n),this.applyMatrix4(wi),this}scale(e,t,n){return wi.makeScale(e,t,n),this.applyMatrix4(wi),this}lookAt(e){return Ti.lookAt(e),Ti.updateMatrix(),this.applyMatrix4(Ti.matrix),this}center(){return this.computeBoundingBox(),this.boundingBox.getCenter(Ei).negate(),this.translate(Ei.x,Ei.y,Ei.z),this}setFromPoints(e){let t=this.getAttribute(`position`);if(t===void 0){let t=[];for(let n=0,r=e.length;n<r;n++){let r=e[n];t.push(r.x,r.y,r.z||0)}this.setAttribute(`position`,new X(t,3))}else{let n=Math.min(e.length,t.count);for(let r=0;r<n;r++){let n=e[r];t.setXYZ(r,n.x,n.y,n.z||0)}e.length>t.count&&U(`BufferGeometry: Buffer size too small for points data. Use .dispose() and create a new geometry.`),t.needsUpdate=!0}return this}computeBoundingBox(){this.boundingBox===null&&(this.boundingBox=new Vr);let e=this.attributes.position,t=this.morphAttributes.position;if(e&&e.isGLBufferAttribute)W(`BufferGeometry.computeBoundingBox(): GLBufferAttribute requires a manual bounding box.`,this),this.boundingBox.set(new q(-1/0,-1/0,-1/0),new q(1/0,1/0,1/0));else{if(e!==void 0){if(this.boundingBox.setFromBufferAttribute(e),t)for(let e=0,n=t.length;e<n;e++){let n=t[e];Di.setFromBufferAttribute(n),this.morphTargetsRelative?(ki.addVectors(this.boundingBox.min,Di.min),this.boundingBox.expandByPoint(ki),ki.addVectors(this.boundingBox.max,Di.max),this.boundingBox.expandByPoint(ki)):(this.boundingBox.expandByPoint(Di.min),this.boundingBox.expandByPoint(Di.max))}}else this.boundingBox.makeEmpty();(isNaN(this.boundingBox.min.x)||isNaN(this.boundingBox.min.y)||isNaN(this.boundingBox.min.z))&&W(`BufferGeometry.computeBoundingBox(): Computed min/max have NaN values. The "position" attribute is likely to have NaN values.`,this)}}computeBoundingSphere(){this.boundingSphere===null&&(this.boundingSphere=new Si);let e=this.attributes.position,t=this.morphAttributes.position;if(e&&e.isGLBufferAttribute)W(`BufferGeometry.computeBoundingSphere(): GLBufferAttribute requires a manual bounding sphere.`,this),this.boundingSphere.set(new q,1/0);else if(e){let n=this.boundingSphere.center;if(Di.setFromBufferAttribute(e),t)for(let e=0,n=t.length;e<n;e++){let n=t[e];Oi.setFromBufferAttribute(n),this.morphTargetsRelative?(ki.addVectors(Di.min,Oi.min),Di.expandByPoint(ki),ki.addVectors(Di.max,Oi.max),Di.expandByPoint(ki)):(Di.expandByPoint(Oi.min),Di.expandByPoint(Oi.max))}Di.getCenter(n);let r=0;for(let t=0,i=e.count;t<i;t++)ki.fromBufferAttribute(e,t),r=Math.max(r,n.distanceToSquared(ki));if(t)for(let i=0,a=t.length;i<a;i++){let a=t[i],o=this.morphTargetsRelative;for(let t=0,i=a.count;t<i;t++)ki.fromBufferAttribute(a,t),o&&(Ei.fromBufferAttribute(e,t),ki.add(Ei)),r=Math.max(r,n.distanceToSquared(ki))}this.boundingSphere.radius=Math.sqrt(r),isNaN(this.boundingSphere.radius)&&W(`BufferGeometry.computeBoundingSphere(): Computed radius is NaN. The "position" attribute is likely to have NaN values.`,this)}}computeTangents(){let e=this.index,t=this.attributes;if(e===null||t.position===void 0||t.normal===void 0||t.uv===void 0){W(`BufferGeometry: .computeTangents() failed. Missing required attributes (index, position, normal or uv)`);return}let n=t.position,r=t.normal,i=t.uv,a=this.getAttribute(`tangent`);(a===void 0||a.count!==n.count)&&(a=new ui(new Float32Array(4*n.count),4),this.setAttribute(`tangent`,a));let o=[],s=[];for(let e=0;e<n.count;e++)o[e]=new q,s[e]=new q;let c=new q,l=new q,u=new q,d=new K,f=new K,p=new K,m=new q,h=new q;function g(e,t,r){c.fromBufferAttribute(n,e),l.fromBufferAttribute(n,t),u.fromBufferAttribute(n,r),d.fromBufferAttribute(i,e),f.fromBufferAttribute(i,t),p.fromBufferAttribute(i,r),l.sub(c),u.sub(c),f.sub(d),p.sub(d);let a=1/(f.x*p.y-p.x*f.y);isFinite(a)&&(m.copy(l).multiplyScalar(p.y).addScaledVector(u,-f.y).multiplyScalar(a),h.copy(u).multiplyScalar(f.x).addScaledVector(l,-p.x).multiplyScalar(a),o[e].add(m),o[t].add(m),o[r].add(m),s[e].add(h),s[t].add(h),s[r].add(h))}let _=this.groups;_.length===0&&(_=[{start:0,count:e.count}]);for(let t=0,n=_.length;t<n;++t){let n=_[t],r=n.start,i=n.count;for(let t=r,n=r+i;t<n;t+=3)g(e.getX(t+0),e.getX(t+1),e.getX(t+2))}let v=new q,y=new q,b=new q,x=new q;function S(e){b.fromBufferAttribute(r,e),x.copy(b);let t=o[e];v.copy(t),v.sub(b.multiplyScalar(b.dot(t))).normalize(),y.crossVectors(x,t);let n=y.dot(s[e])<0?-1:1;a.setXYZW(e,v.x,v.y,v.z,n)}for(let t=0,n=_.length;t<n;++t){let n=_[t],r=n.start,i=n.count;for(let t=r,n=r+i;t<n;t+=3)S(e.getX(t+0)),S(e.getX(t+1)),S(e.getX(t+2))}this._transformed=!0}computeVertexNormals(){let e=this.index,t=this.getAttribute(`position`);if(t!==void 0){let n=this.getAttribute(`normal`);if(n===void 0||n.count!==t.count)n=new ui(new Float32Array(t.count*3),3),this.setAttribute(`normal`,n);else for(let e=0,t=n.count;e<t;e++)n.setXYZ(e,0,0,0);let r=new q,i=new q,a=new q,o=new q,s=new q,c=new q,l=new q,u=new q;if(e)for(let d=0,f=e.count;d<f;d+=3){let f=e.getX(d+0),p=e.getX(d+1),m=e.getX(d+2);r.fromBufferAttribute(t,f),i.fromBufferAttribute(t,p),a.fromBufferAttribute(t,m),l.subVectors(a,i),u.subVectors(r,i),l.cross(u),o.fromBufferAttribute(n,f),s.fromBufferAttribute(n,p),c.fromBufferAttribute(n,m),o.add(l),s.add(l),c.add(l),n.setXYZ(f,o.x,o.y,o.z),n.setXYZ(p,s.x,s.y,s.z),n.setXYZ(m,c.x,c.y,c.z)}else for(let e=0,o=t.count;e<o;e+=3)r.fromBufferAttribute(t,e+0),i.fromBufferAttribute(t,e+1),a.fromBufferAttribute(t,e+2),l.subVectors(a,i),u.subVectors(r,i),l.cross(u),n.setXYZ(e+0,l.x,l.y,l.z),n.setXYZ(e+1,l.x,l.y,l.z),n.setXYZ(e+2,l.x,l.y,l.z);this.normalizeNormals(),n.needsUpdate=!0}}normalizeNormals(){let e=this.attributes.normal;for(let t=0,n=e.count;t<n;t++)ki.fromBufferAttribute(e,t),ki.normalize(),e.setXYZ(t,ki.x,ki.y,ki.z)}toNonIndexed(){function t(e,t){let n=e.array,r=e.itemSize,i=e.normalized,a=new n.constructor(t.length*r),o=0,s=0;for(let i=0,c=t.length;i<c;i++){o=e.isInterleavedBufferAttribute?t[i]*e.data.stride+e.offset:t[i]*r;for(let e=0;e<r;e++)a[s++]=n[o++]}return new ui(a,r,i)}if(this.index===null)return U(`BufferGeometry.toNonIndexed(): BufferGeometry is already non-indexed.`),this;let n=new e,r=this.index.array,i=this.attributes;for(let e in i){let a=i[e],o=t(a,r);n.setAttribute(e,o)}let a=this.morphAttributes;for(let e in a){let i=[],o=a[e];for(let e=0,n=o.length;e<n;e++){let n=o[e],a=t(n,r);i.push(a)}n.morphAttributes[e]=i}n.morphTargetsRelative=this.morphTargetsRelative;let o=this.groups;for(let e=0,t=o.length;e<t;e++){let t=o[e];n.addGroup(t.start,t.count,t.materialIndex)}return n}toJSON(){let e={metadata:{version:4.7,type:`BufferGeometry`,generator:`BufferGeometry.toJSON`}};if(e.uuid=this.uuid,e.type=this.parameters!==void 0&&this._transformed===!0?`BufferGeometry`:this.type,e.name=this.name,Object.keys(this.userData).length>0&&(e.userData=this.userData),this.parameters!==void 0&&this._transformed!==!0){let t=this.parameters;for(let n in t)t[n]!==void 0&&(e[n]=t[n]);return e}e.data={attributes:{}};let t=this.index;t!==null&&(e.data.index={type:t.array.constructor.name,array:Array.prototype.slice.call(t.array)});let n=this.attributes;for(let t in n){let r=n[t];e.data.attributes[t]=r.toJSON(e.data)}let r={},i=!1;for(let t in this.morphAttributes){let n=this.morphAttributes[t],a=[];for(let t=0,r=n.length;t<r;t++){let r=n[t];a.push(r.toJSON(e.data))}a.length>0&&(r[t]=a,i=!0)}i&&(e.data.morphAttributes=r,e.data.morphTargetsRelative=this.morphTargetsRelative);let a=this.groups;a.length>0&&(e.data.groups=JSON.parse(JSON.stringify(a)));let o=this.boundingSphere;return o!==null&&(e.data.boundingSphere=o.toJSON()),e}clone(){return new this.constructor().copy(this)}copy(e){this.index=null,this.attributes={},this.morphAttributes={},this.groups=[],this.boundingBox=null,this.boundingSphere=null;let t={};this.name=e.name;let n=e.index;n!==null&&this.setIndex(n.clone());let r=e.attributes;for(let e in r){let n=r[e];this.setAttribute(e,n.clone(t))}let i=e.morphAttributes;for(let e in i){let n=[],r=i[e];for(let e=0,i=r.length;e<i;e++)n.push(r[e].clone(t));this.morphAttributes[e]=n}this.morphTargetsRelative=e.morphTargetsRelative;let a=e.groups;for(let e=0,t=a.length;e<t;e++){let t=a[e];this.addGroup(t.start,t.count,t.materialIndex)}let o=e.boundingBox;o!==null&&(this.boundingBox=o.clone());let s=e.boundingSphere;return s!==null&&(this.boundingSphere=s.clone()),this.drawRange.start=e.drawRange.start,this.drawRange.count=e.drawRange.count,this.userData=e.userData,this._transformed=e._transformed,this}dispose(){this.dispatchEvent({type:`dispose`})}},Ai=class{constructor(e,t){this.isInterleavedBuffer=!0,this.array=e,this.stride=t,this.count=e===void 0?0:e.length/t,this.usage=ut,this.updateRanges=[],this.version=0,this.uuid=qt()}onUploadCallback(){}set needsUpdate(e){e===!0&&this.version++}setUsage(e){return this.usage=e,this}addUpdateRange(e,t){this.updateRanges.push({start:e,count:t})}clearUpdateRanges(){this.updateRanges.length=0}copy(e){return this.array=new e.array.constructor(e.array),this.count=e.count,this.stride=e.stride,this.usage=e.usage,this}copyAt(e,t,n){e*=this.stride,n*=t.stride;for(let r=0,i=this.stride;r<i;r++)this.array[e+r]=t.array[n+r];return this}set(e,t=0){return this.array.set(e,t),this}clone(e){e.arrayBuffers===void 0&&(e.arrayBuffers={}),this.array.buffer._uuid===void 0&&(this.array.buffer._uuid=qt()),e.arrayBuffers[this.array.buffer._uuid]===void 0&&(e.arrayBuffers[this.array.buffer._uuid]=this.array.slice(0).buffer);let t=new this.array.constructor(e.arrayBuffers[this.array.buffer._uuid]),n=new this.constructor(t,this.stride);return n.setUsage(this.usage),n}onUpload(e){return this.onUploadCallback=e,this}toJSON(e){e.arrayBuffers===void 0&&(e.arrayBuffers={}),this.array.buffer._uuid===void 0&&(this.array.buffer._uuid=qt()),e.arrayBuffers[this.array.buffer._uuid]===void 0&&(e.arrayBuffers[this.array.buffer._uuid]=Array.from(new Uint32Array(this.array.buffer)));let t={uuid:this.uuid,buffer:this.array.buffer._uuid,type:this.array.constructor.name,stride:this.stride};return t.usage=this.usage,t}},ji=new q,Mi=class e{constructor(e,t,n,r=!1){this.isInterleavedBufferAttribute=!0,this.name=``,this.data=e,this.itemSize=t,this.offset=n,this.normalized=r}get count(){return this.data.count}get array(){return this.data.array}set needsUpdate(e){this.data.needsUpdate=e}applyMatrix4(e){for(let t=0,n=this.data.count;t<n;t++)ji.fromBufferAttribute(this,t),ji.applyMatrix4(e),this.setXYZ(t,ji.x,ji.y,ji.z);return this}applyNormalMatrix(e){for(let t=0,n=this.count;t<n;t++)ji.fromBufferAttribute(this,t),ji.applyNormalMatrix(e),this.setXYZ(t,ji.x,ji.y,ji.z);return this}transformDirection(e){for(let t=0,n=this.count;t<n;t++)ji.fromBufferAttribute(this,t),ji.transformDirection(e),this.setXYZ(t,ji.x,ji.y,ji.z);return this}getComponent(e,t){let n=this.array[e*this.data.stride+this.offset+t];return this.normalized&&(n=pn(n,this.array)),n}setComponent(e,t,n){return this.normalized&&(n=mn(n,this.array)),this.data.array[e*this.data.stride+this.offset+t]=n,this}setX(e,t){return this.normalized&&(t=mn(t,this.array)),this.data.array[e*this.data.stride+this.offset]=t,this}setY(e,t){return this.normalized&&(t=mn(t,this.array)),this.data.array[e*this.data.stride+this.offset+1]=t,this}setZ(e,t){return this.normalized&&(t=mn(t,this.array)),this.data.array[e*this.data.stride+this.offset+2]=t,this}setW(e,t){return this.normalized&&(t=mn(t,this.array)),this.data.array[e*this.data.stride+this.offset+3]=t,this}getX(e){let t=this.data.array[e*this.data.stride+this.offset];return this.normalized&&(t=pn(t,this.array)),t}getY(e){let t=this.data.array[e*this.data.stride+this.offset+1];return this.normalized&&(t=pn(t,this.array)),t}getZ(e){let t=this.data.array[e*this.data.stride+this.offset+2];return this.normalized&&(t=pn(t,this.array)),t}getW(e){let t=this.data.array[e*this.data.stride+this.offset+3];return this.normalized&&(t=pn(t,this.array)),t}setXY(e,t,n){return e=e*this.data.stride+this.offset,this.normalized&&(t=mn(t,this.array),n=mn(n,this.array)),this.data.array[e+0]=t,this.data.array[e+1]=n,this}setXYZ(e,t,n,r){return e=e*this.data.stride+this.offset,this.normalized&&(t=mn(t,this.array),n=mn(n,this.array),r=mn(r,this.array)),this.data.array[e+0]=t,this.data.array[e+1]=n,this.data.array[e+2]=r,this}setXYZW(e,t,n,r,i){return e=e*this.data.stride+this.offset,this.normalized&&(t=mn(t,this.array),n=mn(n,this.array),r=mn(r,this.array),i=mn(i,this.array)),this.data.array[e+0]=t,this.data.array[e+1]=n,this.data.array[e+2]=r,this.data.array[e+3]=i,this}clone(t){if(t===void 0){Lt(`InterleavedBufferAttribute.clone(): Cloning an interleaved buffer attribute will de-interleave buffer data.`);let e=[];for(let t=0;t<this.count;t++){let n=t*this.data.stride+this.offset;for(let t=0;t<this.itemSize;t++)e.push(this.data.array[n+t])}return new ui(new this.array.constructor(e),this.itemSize,this.normalized)}return t.interleavedBuffers===void 0&&(t.interleavedBuffers={}),t.interleavedBuffers[this.data.uuid]===void 0&&(t.interleavedBuffers[this.data.uuid]=this.data.clone(t)),new e(t.interleavedBuffers[this.data.uuid],this.itemSize,this.offset,this.normalized)}toJSON(e){if(e===void 0){Lt(`InterleavedBufferAttribute.toJSON(): Serializing an interleaved buffer attribute will de-interleave buffer data.`);let e=[];for(let t=0;t<this.count;t++){let n=t*this.data.stride+this.offset;for(let t=0;t<this.itemSize;t++)e.push(this.data.array[n+t])}return{itemSize:this.itemSize,type:this.array.constructor.name,array:e,normalized:this.normalized}}return e.interleavedBuffers===void 0&&(e.interleavedBuffers={}),e.interleavedBuffers[this.data.uuid]===void 0&&(e.interleavedBuffers[this.data.uuid]=this.data.toJSON(e)),{isInterleavedBufferAttribute:!0,itemSize:this.itemSize,data:this.data.uuid,offset:this.offset,normalized:this.normalized}}},Ni=new q,Pi=new q,Fi=new yn,Ii=class{constructor(e=new q(1,0,0),t=0){this.isPlane=!0,this.normal=e,this.constant=t}set(e,t){return this.normal.copy(e),this.constant=t,this}setComponents(e,t,n,r){return this.normal.set(e,t,n),this.constant=r,this}setFromNormalAndCoplanarPoint(e,t){return this.normal.copy(e),this.constant=-t.dot(this.normal),this}setFromCoplanarPoints(e,t,n){let r=Ni.subVectors(n,t).cross(Pi.subVectors(e,t)).normalize();return this.setFromNormalAndCoplanarPoint(r,e),this}copy(e){return this.normal.copy(e.normal),this.constant=e.constant,this}normalize(){let e=1/this.normal.length();return this.normal.multiplyScalar(e),this.constant*=e,this}negate(){return this.constant*=-1,this.normal.negate(),this}distanceToPoint(e){return this.normal.dot(e)+this.constant}distanceToSphere(e){return this.distanceToPoint(e.center)-e.radius}projectPoint(e,t){return t.copy(e).addScaledVector(this.normal,-this.distanceToPoint(e))}intersectLine(e,t,n=!0){let r=e.delta(Ni),i=this.normal.dot(r);if(i===0)return this.distanceToPoint(e.start)===0?t.copy(e.start):null;let a=-(e.start.dot(this.normal)+this.constant)/i;return n===!0&&(a<0||a>1)?null:t.copy(e.start).addScaledVector(r,a)}intersectsLine(e){let t=this.distanceToPoint(e.start),n=this.distanceToPoint(e.end);return t<0&&n>0||n<0&&t>0}intersectsBox(e){return e.intersectsPlane(this)}intersectsSphere(e){return e.intersectsPlane(this)}coplanarPoint(e){return e.copy(this.normal).multiplyScalar(-this.constant)}applyMatrix4(e,t){let n=t||Fi.getNormalMatrix(e),r=this.coplanarPoint(Ni).applyMatrix4(e),i=this.normal.applyMatrix3(n).normalize();return this.constant=-r.dot(i),this}translate(e){return this.constant-=e.dot(this.normal),this}equals(e){return e.normal.equals(this.normal)&&e.constant===this.constant}clone(){return new this.constructor().copy(this)}toJSON(){return{normal:this.normal.toArray(),constant:this.constant}}fromJSON(e){return this.normal.fromArray(e.normal),this.constant=e.constant,this}},Li=0,Ri=class extends Ht{constructor(){super(),this.isMaterial=!0,Object.defineProperty(this,"id",{value:Li++}),this.uuid=qt(),this.name=``,this.type=`Material`,this.blending=1,this.side=0,this.vertexColors=!1,this.opacity=1,this.transparent=!1,this.alphaHash=!1,this.blendSrc=204,this.blendDst=205,this.blendEquation=100,this.blendSrcAlpha=null,this.blendDstAlpha=null,this.blendEquationAlpha=null,this.blendColor=new Y(0,0,0),this.blendAlpha=0,this.depthFunc=3,this.depthTest=!0,this.depthWrite=!0,this.stencilWriteMask=255,this.stencilFunc=519,this.stencilRef=0,this.stencilFuncMask=255,this.stencilFail=rt,this.stencilZFail=rt,this.stencilZPass=rt,this.stencilWrite=!1,this.clippingPlanes=null,this.clipIntersection=!1,this.clipShadows=!1,this.shadowSide=null,this.colorWrite=!0,this.precision=null,this.polygonOffset=!1,this.polygonOffsetFactor=0,this.polygonOffsetUnits=0,this.dithering=!1,this.alphaToCoverage=!1,this.premultipliedAlpha=!1,this.forceSinglePass=!1,this.allowOverride=!0,this.visible=!0,this.toneMapped=!0,this.userData={},this.version=0,this._alphaTest=0}get alphaTest(){return this._alphaTest}set alphaTest(e){this._alphaTest>0!=e>0&&this.version++,this._alphaTest=e}onBeforeRender(){}onBeforeCompile(){}customProgramCacheKey(){return this.onBeforeCompile.toString()}setValues(e){if(e!==void 0)for(let t in e){let n=e[t];if(n===void 0){U(`Material: parameter '${t}' has value of undefined.`);continue}let r=this[t];r===void 0?U(`Material: '${t}' is not a property of THREE.${this.type}.`):r&&r.isColor?r.set(n):r&&r.isVector2&&n&&n.isVector2||r&&r.isEuler&&n&&n.isEuler||r&&r.isVector3&&n&&n.isVector3?r.copy(n):this[t]=n}}toJSON(e){let t=e===void 0||typeof e==`string`;t&&(e={textures:{},images:{}});let n={metadata:{version:4.7,type:`Material`,generator:`Material.toJSON`}};n.uuid=this.uuid,n.type=this.type,n.blending=this.blending,n.side=this.side,n.shadowSide=this.shadowSide,n.vertexColors=this.vertexColors,n.opacity=this.opacity,n.transparent=this.transparent,n.blendSrc=this.blendSrc,n.blendDst=this.blendDst,n.blendEquation=this.blendEquation,n.blendSrcAlpha=this.blendSrcAlpha,n.blendDstAlpha=this.blendDstAlpha,n.blendEquationAlpha=this.blendEquationAlpha,n.blendColor=this.blendColor.getHex(),n.blendAlpha=this.blendAlpha,n.depthFunc=this.depthFunc,n.depthTest=this.depthTest,n.depthWrite=this.depthWrite,n.colorWrite=this.colorWrite,n.clipIntersection=this.clipIntersection,n.clipShadows=this.clipShadows,n.stencilWriteMask=this.stencilWriteMask,n.stencilFunc=this.stencilFunc,n.stencilRef=this.stencilRef,n.stencilFuncMask=this.stencilFuncMask,n.stencilFail=this.stencilFail,n.stencilZFail=this.stencilZFail,n.stencilZPass=this.stencilZPass,n.stencilWrite=this.stencilWrite,n.polygonOffset=this.polygonOffset,n.polygonOffsetFactor=this.polygonOffsetFactor,n.polygonOffsetUnits=this.polygonOffsetUnits,n.dithering=this.dithering,n.alphaTest=this.alphaTest,n.alphaHash=this.alphaHash,n.alphaToCoverage=this.alphaToCoverage,n.premultipliedAlpha=this.premultipliedAlpha,n.forceSinglePass=this.forceSinglePass,n.allowOverride=this.allowOverride,n.visible=this.visible,n.toneMapped=this.toneMapped,n.name=this.name,this.color&&this.color.isColor&&(n.color=this.color.getHex()),this.roughness!==void 0&&(n.roughness=this.roughness),this.metalness!==void 0&&(n.metalness=this.metalness),this.sheen!==void 0&&(n.sheen=this.sheen),this.sheenColor&&this.sheenColor.isColor&&(n.sheenColor=this.sheenColor.getHex()),this.sheenRoughness!==void 0&&(n.sheenRoughness=this.sheenRoughness),this.emissive&&this.emissive.isColor&&(n.emissive=this.emissive.getHex()),this.emissiveIntensity!==void 0&&(n.emissiveIntensity=this.emissiveIntensity),this.specular&&this.specular.isColor&&(n.specular=this.specular.getHex()),this.specularIntensity!==void 0&&(n.specularIntensity=this.specularIntensity),this.specularColor&&this.specularColor.isColor&&(n.specularColor=this.specularColor.getHex()),this.shininess!==void 0&&(n.shininess=this.shininess),this.clearcoat!==void 0&&(n.clearcoat=this.clearcoat),this.clearcoatRoughness!==void 0&&(n.clearcoatRoughness=this.clearcoatRoughness),this.clearcoatMap&&this.clearcoatMap.isTexture&&(n.clearcoatMap=this.clearcoatMap.toJSON(e).uuid),this.clearcoatRoughnessMap&&this.clearcoatRoughnessMap.isTexture&&(n.clearcoatRoughnessMap=this.clearcoatRoughnessMap.toJSON(e).uuid),this.clearcoatNormalMap&&this.clearcoatNormalMap.isTexture&&(n.clearcoatNormalMap=this.clearcoatNormalMap.toJSON(e).uuid,n.clearcoatNormalScale=this.clearcoatNormalScale.toArray()),this.sheenColorMap&&this.sheenColorMap.isTexture&&(n.sheenColorMap=this.sheenColorMap.toJSON(e).uuid),this.sheenRoughnessMap&&this.sheenRoughnessMap.isTexture&&(n.sheenRoughnessMap=this.sheenRoughnessMap.toJSON(e).uuid),this.dispersion!==void 0&&(n.dispersion=this.dispersion),this.retroreflectivity!==void 0&&(n.retroreflectivity=this.retroreflectivity),this.iridescence!==void 0&&(n.iridescence=this.iridescence),this.iridescenceIOR!==void 0&&(n.iridescenceIOR=this.iridescenceIOR),this.iridescenceThicknessRange!==void 0&&(n.iridescenceThicknessRange=this.iridescenceThicknessRange),this.iridescenceMap&&this.iridescenceMap.isTexture&&(n.iridescenceMap=this.iridescenceMap.toJSON(e).uuid),this.iridescenceThicknessMap&&this.iridescenceThicknessMap.isTexture&&(n.iridescenceThicknessMap=this.iridescenceThicknessMap.toJSON(e).uuid),this.anisotropy!==void 0&&(n.anisotropy=this.anisotropy),this.anisotropyRotation!==void 0&&(n.anisotropyRotation=this.anisotropyRotation),this.anisotropyMap&&this.anisotropyMap.isTexture&&(n.anisotropyMap=this.anisotropyMap.toJSON(e).uuid),this.map&&this.map.isTexture&&(n.map=this.map.toJSON(e).uuid),this.matcap&&this.matcap.isTexture&&(n.matcap=this.matcap.toJSON(e).uuid),this.alphaMap&&this.alphaMap.isTexture&&(n.alphaMap=this.alphaMap.toJSON(e).uuid),this.lightMap&&this.lightMap.isTexture&&(n.lightMap=this.lightMap.toJSON(e).uuid,n.lightMapIntensity=this.lightMapIntensity),this.aoMap&&this.aoMap.isTexture&&(n.aoMap=this.aoMap.toJSON(e).uuid,n.aoMapIntensity=this.aoMapIntensity),this.bumpMap&&this.bumpMap.isTexture&&(n.bumpMap=this.bumpMap.toJSON(e).uuid,n.bumpScale=this.bumpScale),this.normalMap&&this.normalMap.isTexture&&(n.normalMap=this.normalMap.toJSON(e).uuid,n.normalMapType=this.normalMapType,n.normalScale=this.normalScale.toArray()),this.displacementMap&&this.displacementMap.isTexture&&(n.displacementMap=this.displacementMap.toJSON(e).uuid,n.displacementScale=this.displacementScale,n.displacementBias=this.displacementBias),this.roughnessMap&&this.roughnessMap.isTexture&&(n.roughnessMap=this.roughnessMap.toJSON(e).uuid),this.metalnessMap&&this.metalnessMap.isTexture&&(n.metalnessMap=this.metalnessMap.toJSON(e).uuid),this.emissiveMap&&this.emissiveMap.isTexture&&(n.emissiveMap=this.emissiveMap.toJSON(e).uuid),this.specularMap&&this.specularMap.isTexture&&(n.specularMap=this.specularMap.toJSON(e).uuid),this.specularIntensityMap&&this.specularIntensityMap.isTexture&&(n.specularIntensityMap=this.specularIntensityMap.toJSON(e).uuid),this.specularColorMap&&this.specularColorMap.isTexture&&(n.specularColorMap=this.specularColorMap.toJSON(e).uuid),this.envMap&&this.envMap.isTexture&&(n.envMap=this.envMap.toJSON(e).uuid,this.combine!==void 0&&(n.combine=this.combine)),this.envMapRotation!==void 0&&(n.envMapRotation=this.envMapRotation.toArray()),this.envMapIntensity!==void 0&&(n.envMapIntensity=this.envMapIntensity),this.reflectivity!==void 0&&(n.reflectivity=this.reflectivity),this.refractionRatio!==void 0&&(n.refractionRatio=this.refractionRatio),this.gradientMap&&this.gradientMap.isTexture&&(n.gradientMap=this.gradientMap.toJSON(e).uuid),this.transmission!==void 0&&(n.transmission=this.transmission),this.transmissionMap&&this.transmissionMap.isTexture&&(n.transmissionMap=this.transmissionMap.toJSON(e).uuid),this.thickness!==void 0&&(n.thickness=this.thickness),this.thicknessMap&&this.thicknessMap.isTexture&&(n.thicknessMap=this.thicknessMap.toJSON(e).uuid),this.attenuationDistance!==void 0&&(n.attenuationDistance=this.attenuationDistance),this.attenuationColor!==void 0&&(n.attenuationColor=this.attenuationColor.getHex()),this.size!==void 0&&(n.size=this.size),this.sizeAttenuation!==void 0&&(n.sizeAttenuation=this.sizeAttenuation),Array.isArray(this.clippingPlanes)&&this.clippingPlanes.length>0&&(n.clippingPlanes=this.clippingPlanes.map(e=>e.toJSON())),this.rotation!==void 0&&(n.rotation=this.rotation),this.depthPacking!==void 0&&(n.depthPacking=this.depthPacking),this.linewidth!==void 0&&(n.linewidth=this.linewidth),this.linecap!==void 0&&(n.linecap=this.linecap),this.linejoin!==void 0&&(n.linejoin=this.linejoin),this.dashSize!==void 0&&(n.dashSize=this.dashSize),this.gapSize!==void 0&&(n.gapSize=this.gapSize),this.scale!==void 0&&(n.scale=this.scale),this.wireframe!==void 0&&(n.wireframe=this.wireframe),this.wireframeLinewidth!==void 0&&(n.wireframeLinewidth=this.wireframeLinewidth),this.wireframeLinecap!==void 0&&(n.wireframeLinecap=this.wireframeLinecap),this.wireframeLinejoin!==void 0&&(n.wireframeLinejoin=this.wireframeLinejoin),this.flatShading!==void 0&&(n.flatShading=this.flatShading),this.fog!==void 0&&(n.fog=this.fog),Object.keys(this.userData).length>0&&(n.userData=this.userData);function r(e){let t=[];for(let n in e){let r=e[n];delete r.metadata,t.push(r)}return t}if(t){let t=r(e.textures),i=r(e.images);t.length>0&&(n.textures=t),i.length>0&&(n.images=i)}return n}fromJSON(e,t){if(e.uuid!==void 0&&(this.uuid=e.uuid),e.name!==void 0&&(this.name=e.name),e.color!==void 0&&this.color!==void 0&&this.color.setHex(e.color),e.roughness!==void 0&&(this.roughness=e.roughness),e.metalness!==void 0&&(this.metalness=e.metalness),e.sheen!==void 0&&(this.sheen=e.sheen),e.sheenColor!==void 0&&(this.sheenColor=new Y().setHex(e.sheenColor)),e.sheenRoughness!==void 0&&(this.sheenRoughness=e.sheenRoughness),e.emissive!==void 0&&this.emissive!==void 0&&this.emissive.setHex(e.emissive),e.specular!==void 0&&this.specular!==void 0&&this.specular.setHex(e.specular),e.specularIntensity!==void 0&&(this.specularIntensity=e.specularIntensity),e.specularColor!==void 0&&this.specularColor!==void 0&&this.specularColor.setHex(e.specularColor),e.shininess!==void 0&&(this.shininess=e.shininess),e.clearcoat!==void 0&&(this.clearcoat=e.clearcoat),e.clearcoatRoughness!==void 0&&(this.clearcoatRoughness=e.clearcoatRoughness),e.dispersion!==void 0&&(this.dispersion=e.dispersion),e.retroreflectivity!==void 0&&(this.retroreflectivity=e.retroreflectivity),e.iridescence!==void 0&&(this.iridescence=e.iridescence),e.iridescenceIOR!==void 0&&(this.iridescenceIOR=e.iridescenceIOR),e.iridescenceThicknessRange!==void 0&&(this.iridescenceThicknessRange=e.iridescenceThicknessRange),e.transmission!==void 0&&(this.transmission=e.transmission),e.thickness!==void 0&&(this.thickness=e.thickness),e.attenuationDistance!==void 0&&(this.attenuationDistance=e.attenuationDistance),e.attenuationColor!==void 0&&this.attenuationColor!==void 0&&this.attenuationColor.setHex(e.attenuationColor),e.anisotropy!==void 0&&(this.anisotropy=e.anisotropy),e.anisotropyRotation!==void 0&&(this.anisotropyRotation=e.anisotropyRotation),e.fog!==void 0&&(this.fog=e.fog),e.flatShading!==void 0&&(this.flatShading=e.flatShading),e.blending!==void 0&&(this.blending=e.blending),e.combine!==void 0&&(this.combine=e.combine),e.side!==void 0&&(this.side=e.side),e.shadowSide!==void 0&&(this.shadowSide=e.shadowSide),e.opacity!==void 0&&(this.opacity=e.opacity),e.transparent!==void 0&&(this.transparent=e.transparent),e.alphaTest!==void 0&&(this.alphaTest=e.alphaTest),e.alphaHash!==void 0&&(this.alphaHash=e.alphaHash),e.depthFunc!==void 0&&(this.depthFunc=e.depthFunc),e.depthTest!==void 0&&(this.depthTest=e.depthTest),e.depthWrite!==void 0&&(this.depthWrite=e.depthWrite),e.colorWrite!==void 0&&(this.colorWrite=e.colorWrite),e.clippingPlanes!==void 0&&(this.clippingPlanes=e.clippingPlanes.map(e=>new Ii().fromJSON(e))),e.clipIntersection!==void 0&&(this.clipIntersection=e.clipIntersection),e.clipShadows!==void 0&&(this.clipShadows=e.clipShadows),e.depthPacking!==void 0&&(this.depthPacking=e.depthPacking),e.blendSrc!==void 0&&(this.blendSrc=e.blendSrc),e.blendDst!==void 0&&(this.blendDst=e.blendDst),e.blendEquation!==void 0&&(this.blendEquation=e.blendEquation),e.blendSrcAlpha!==void 0&&(this.blendSrcAlpha=e.blendSrcAlpha),e.blendDstAlpha!==void 0&&(this.blendDstAlpha=e.blendDstAlpha),e.blendEquationAlpha!==void 0&&(this.blendEquationAlpha=e.blendEquationAlpha),e.blendColor!==void 0&&this.blendColor!==void 0&&this.blendColor.setHex(e.blendColor),e.blendAlpha!==void 0&&(this.blendAlpha=e.blendAlpha),e.stencilWriteMask!==void 0&&(this.stencilWriteMask=e.stencilWriteMask),e.stencilFunc!==void 0&&(this.stencilFunc=e.stencilFunc),e.stencilRef!==void 0&&(this.stencilRef=e.stencilRef),e.stencilFuncMask!==void 0&&(this.stencilFuncMask=e.stencilFuncMask),e.stencilFail!==void 0&&(this.stencilFail=e.stencilFail),e.stencilZFail!==void 0&&(this.stencilZFail=e.stencilZFail),e.stencilZPass!==void 0&&(this.stencilZPass=e.stencilZPass),e.stencilWrite!==void 0&&(this.stencilWrite=e.stencilWrite),e.wireframe!==void 0&&(this.wireframe=e.wireframe),e.wireframeLinewidth!==void 0&&(this.wireframeLinewidth=e.wireframeLinewidth),e.wireframeLinecap!==void 0&&(this.wireframeLinecap=e.wireframeLinecap),e.wireframeLinejoin!==void 0&&(this.wireframeLinejoin=e.wireframeLinejoin),e.rotation!==void 0&&(this.rotation=e.rotation),e.linewidth!==void 0&&(this.linewidth=e.linewidth),e.linecap!==void 0&&(this.linecap=e.linecap),e.linejoin!==void 0&&(this.linejoin=e.linejoin),e.dashSize!==void 0&&(this.dashSize=e.dashSize),e.gapSize!==void 0&&(this.gapSize=e.gapSize),e.scale!==void 0&&(this.scale=e.scale),e.polygonOffset!==void 0&&(this.polygonOffset=e.polygonOffset),e.polygonOffsetFactor!==void 0&&(this.polygonOffsetFactor=e.polygonOffsetFactor),e.polygonOffsetUnits!==void 0&&(this.polygonOffsetUnits=e.polygonOffsetUnits),e.dithering!==void 0&&(this.dithering=e.dithering),e.alphaToCoverage!==void 0&&(this.alphaToCoverage=e.alphaToCoverage),e.premultipliedAlpha!==void 0&&(this.premultipliedAlpha=e.premultipliedAlpha),e.forceSinglePass!==void 0&&(this.forceSinglePass=e.forceSinglePass),e.allowOverride!==void 0&&(this.allowOverride=e.allowOverride),e.visible!==void 0&&(this.visible=e.visible),e.toneMapped!==void 0&&(this.toneMapped=e.toneMapped),e.userData!==void 0&&(this.userData=e.userData),e.vertexColors!==void 0&&(this.vertexColors=typeof e.vertexColors==`number`?e.vertexColors>0:e.vertexColors),e.size!==void 0&&(this.size=e.size),e.sizeAttenuation!==void 0&&(this.sizeAttenuation=e.sizeAttenuation),e.map!==void 0&&(this.map=t[e.map]||null),e.matcap!==void 0&&(this.matcap=t[e.matcap]||null),e.alphaMap!==void 0&&(this.alphaMap=t[e.alphaMap]||null),e.bumpMap!==void 0&&(this.bumpMap=t[e.bumpMap]||null),e.bumpScale!==void 0&&(this.bumpScale=e.bumpScale),e.normalMap!==void 0&&(this.normalMap=t[e.normalMap]||null),e.normalMapType!==void 0&&(this.normalMapType=e.normalMapType),e.normalScale!==void 0){let t=e.normalScale;Array.isArray(t)===!1&&(t=[t,t]),this.normalScale=new K().fromArray(t)}return e.displacementMap!==void 0&&(this.displacementMap=t[e.displacementMap]||null),e.displacementScale!==void 0&&(this.displacementScale=e.displacementScale),e.displacementBias!==void 0&&(this.displacementBias=e.displacementBias),e.roughnessMap!==void 0&&(this.roughnessMap=t[e.roughnessMap]||null),e.metalnessMap!==void 0&&(this.metalnessMap=t[e.metalnessMap]||null),e.emissiveMap!==void 0&&(this.emissiveMap=t[e.emissiveMap]||null),e.emissiveIntensity!==void 0&&(this.emissiveIntensity=e.emissiveIntensity),e.specularMap!==void 0&&(this.specularMap=t[e.specularMap]||null),e.specularIntensityMap!==void 0&&(this.specularIntensityMap=t[e.specularIntensityMap]||null),e.specularColorMap!==void 0&&(this.specularColorMap=t[e.specularColorMap]||null),e.envMap!==void 0&&(this.envMap=t[e.envMap]||null),e.envMapRotation!==void 0&&this.envMapRotation.fromArray(e.envMapRotation),e.envMapIntensity!==void 0&&(this.envMapIntensity=e.envMapIntensity),e.reflectivity!==void 0&&(this.reflectivity=e.reflectivity),e.refractionRatio!==void 0&&(this.refractionRatio=e.refractionRatio),e.lightMap!==void 0&&(this.lightMap=t[e.lightMap]||null),e.lightMapIntensity!==void 0&&(this.lightMapIntensity=e.lightMapIntensity),e.aoMap!==void 0&&(this.aoMap=t[e.aoMap]||null),e.aoMapIntensity!==void 0&&(this.aoMapIntensity=e.aoMapIntensity),e.gradientMap!==void 0&&(this.gradientMap=t[e.gradientMap]||null),e.clearcoatMap!==void 0&&(this.clearcoatMap=t[e.clearcoatMap]||null),e.clearcoatRoughnessMap!==void 0&&(this.clearcoatRoughnessMap=t[e.clearcoatRoughnessMap]||null),e.clearcoatNormalMap!==void 0&&(this.clearcoatNormalMap=t[e.clearcoatNormalMap]||null),e.clearcoatNormalScale!==void 0&&(this.clearcoatNormalScale=new K().fromArray(e.clearcoatNormalScale)),e.iridescenceMap!==void 0&&(this.iridescenceMap=t[e.iridescenceMap]||null),e.iridescenceThicknessMap!==void 0&&(this.iridescenceThicknessMap=t[e.iridescenceThicknessMap]||null),e.transmissionMap!==void 0&&(this.transmissionMap=t[e.transmissionMap]||null),e.thicknessMap!==void 0&&(this.thicknessMap=t[e.thicknessMap]||null),e.anisotropyMap!==void 0&&(this.anisotropyMap=t[e.anisotropyMap]||null),e.sheenColorMap!==void 0&&(this.sheenColorMap=t[e.sheenColorMap]||null),e.sheenRoughnessMap!==void 0&&(this.sheenRoughnessMap=t[e.sheenRoughnessMap]||null),this}clone(){return new this.constructor().copy(this)}copy(e){this.name=e.name,this.blending=e.blending,this.side=e.side,this.vertexColors=e.vertexColors,this.opacity=e.opacity,this.transparent=e.transparent,this.blendSrc=e.blendSrc,this.blendDst=e.blendDst,this.blendEquation=e.blendEquation,this.blendSrcAlpha=e.blendSrcAlpha,this.blendDstAlpha=e.blendDstAlpha,this.blendEquationAlpha=e.blendEquationAlpha,this.blendColor.copy(e.blendColor),this.blendAlpha=e.blendAlpha,this.depthFunc=e.depthFunc,this.depthTest=e.depthTest,this.depthWrite=e.depthWrite,this.stencilWriteMask=e.stencilWriteMask,this.stencilFunc=e.stencilFunc,this.stencilRef=e.stencilRef,this.stencilFuncMask=e.stencilFuncMask,this.stencilFail=e.stencilFail,this.stencilZFail=e.stencilZFail,this.stencilZPass=e.stencilZPass,this.stencilWrite=e.stencilWrite;let t=e.clippingPlanes,n=null;if(t!==null){let e=t.length;n=Array(e);for(let r=0;r!==e;++r)n[r]=t[r].clone()}return this.clippingPlanes=n,this.clipIntersection=e.clipIntersection,this.clipShadows=e.clipShadows,this.shadowSide=e.shadowSide,this.colorWrite=e.colorWrite,this.precision=e.precision,this.polygonOffset=e.polygonOffset,this.polygonOffsetFactor=e.polygonOffsetFactor,this.polygonOffsetUnits=e.polygonOffsetUnits,this.dithering=e.dithering,this.alphaTest=e.alphaTest,this.alphaHash=e.alphaHash,this.alphaToCoverage=e.alphaToCoverage,this.premultipliedAlpha=e.premultipliedAlpha,this.forceSinglePass=e.forceSinglePass,this.allowOverride=e.allowOverride,this.visible=e.visible,this.toneMapped=e.toneMapped,this.userData=JSON.parse(JSON.stringify(e.userData)),this}dispose(){this.dispatchEvent({type:`dispose`})}set needsUpdate(e){e===!0&&this.version++}},zi=class extends Ri{constructor(e){super(),this.isSpriteMaterial=!0,this.type=`SpriteMaterial`,this.color=new Y(16777215),this.map=null,this.alphaMap=null,this.rotation=0,this.sizeAttenuation=!0,this.transparent=!0,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.map=e.map,this.alphaMap=e.alphaMap,this.rotation=e.rotation,this.sizeAttenuation=e.sizeAttenuation,this.fog=e.fog,this}},Bi,Vi=new q,Hi=new q,Ui=new q,Wi=new K,Gi=new K,Ki=new J,qi=new q,Ji=new q,Yi=new q,Xi=new K,Zi=new K,Qi=new K,$i=class extends hr{constructor(e=new zi){if(super(),this.isSprite=!0,this.type=`Sprite`,Bi===void 0){Bi=new Z;let e=new Ai(new Float32Array([-.5,-.5,0,0,0,.5,-.5,0,1,0,.5,.5,0,1,1,-.5,.5,0,0,1]),5);Bi.setIndex([0,1,2,0,2,3]),Bi.setAttribute(`position`,new Mi(e,3,0,!1)),Bi.setAttribute(`uv`,new Mi(e,2,3,!1))}this.geometry=Bi,this.material=e,this.center=new K(.5,.5),this.count=1}intersectsFrustum(e){return e.intersectsSprite(this)}raycast(e,t){e.camera===null&&W(`Sprite: "Raycaster.camera" needs to be set in order to raycast against sprites.`),Hi.setFromMatrixScale(this.matrixWorld),Ki.copy(e.camera.matrixWorld),this.modelViewMatrix.multiplyMatrices(e.camera.matrixWorldInverse,this.matrixWorld),Ui.setFromMatrixPosition(this.modelViewMatrix),e.camera.isPerspectiveCamera&&this.material.sizeAttenuation===!1&&Hi.multiplyScalar(-Ui.z);let n=this.material.rotation,r,i;n!==0&&(i=Math.cos(n),r=Math.sin(n));let a=this.center;ea(qi.set(-.5,-.5,0),Ui,a,Hi,r,i),ea(Ji.set(.5,-.5,0),Ui,a,Hi,r,i),ea(Yi.set(.5,.5,0),Ui,a,Hi,r,i),Xi.set(0,0),Zi.set(1,0),Qi.set(1,1);let o=e.ray.intersectTriangle(qi,Ji,Yi,!1,Vi);if(o===null&&(ea(Ji.set(-.5,.5,0),Ui,a,Hi,r,i),Zi.set(0,1),o=e.ray.intersectTriangle(qi,Yi,Ji,!1,Vi),o===null))return;let s=e.ray.origin.distanceTo(Vi);s<e.near||s>e.far||t.push({distance:s,point:Vi.clone(),uv:Br.getInterpolation(Vi,qi,Ji,Yi,Xi,Zi,Qi,new K),face:null,object:this})}copy(e,t){return super.copy(e,t),e.center!==void 0&&this.center.copy(e.center),this.material=e.material,this}};function ea(e,t,n,r,i,a){Wi.subVectors(e,n).addScalar(.5).multiply(r),i===void 0?Gi.copy(Wi):(Gi.x=a*Wi.x-i*Wi.y,Gi.y=i*Wi.x+a*Wi.y),e.copy(t),e.x+=Gi.x,e.y+=Gi.y,e.applyMatrix4(Ki)}var ta=new q,na=new q,ra=class extends hr{constructor(){super(),this.isLOD=!0,this._currentLevel=0,this.type=`LOD`,Object.defineProperties(this,{levels:{enumerable:!0,value:[]}}),this.autoUpdate=!0}copy(e){super.copy(e,!1);let t=e.levels;for(let e=0,n=t.length;e<n;e++){let n=t[e];this.addLevel(n.object.clone(),n.distance,n.hysteresis)}return this.autoUpdate=e.autoUpdate,this}addLevel(e,t=0,n=0){t=Math.abs(t);let r=this.levels,i=0;for(;i<r.length&&!(t<r[i].distance);i++);return r.splice(i,0,{distance:t,hysteresis:n,object:e}),this.add(e),this}removeLevel(e){let t=this.levels;for(let n=0;n<t.length;n++)if(t[n].distance===e){let e=t.splice(n,1);return this.remove(e[0].object),!0}return!1}getCurrentLevel(){return this._currentLevel}getObjectForDistance(e){let t=this.levels;if(t.length>0){let n,r;for(n=1,r=t.length;n<r;n++){let r=t[n].distance;if(t[n].object.visible&&(r-=r*t[n].hysteresis),e<r)break}return t[n-1].object}return null}raycast(e,t){if(this.levels.length>0){ta.setFromMatrixPosition(this.matrixWorld);let n=e.ray.origin.distanceTo(ta);this.getObjectForDistance(n).raycast(e,t)}}update(e){let t=this.levels;if(t.length>1){ta.setFromMatrixPosition(e.matrixWorld),na.setFromMatrixPosition(this.matrixWorld);let n=ta.distanceTo(na)/e.zoom;t[0].object.visible=!0;let r,i;for(r=1,i=t.length;r<i;r++){let e=t[r].distance;if(t[r].object.visible&&(e-=e*t[r].hysteresis),n>=e)t[r-1].object.visible=!1,t[r].object.visible=!0;else break}for(this._currentLevel=r-1;r<i;r++)t[r].object.visible=!1}}toJSON(e){let t=super.toJSON(e);t.object.autoUpdate=this.autoUpdate,t.object.levels=[];let n=this.levels;for(let e=0,r=n.length;e<r;e++){let r=n[e];t.object.levels.push({object:r.object.uuid,distance:r.distance,hysteresis:r.hysteresis})}return t}},ia=new q,aa=new q,oa=new q,sa=new q,ca=class{constructor(e=new q,t=new q(0,0,-1)){this.origin=e,this.direction=t}set(e,t){return this.origin.copy(e),this.direction.copy(t),this}copy(e){return this.origin.copy(e.origin),this.direction.copy(e.direction),this}at(e,t){return t.copy(this.origin).addScaledVector(this.direction,e)}lookAt(e){return this.direction.copy(e).sub(this.origin).normalize(),this}recast(e){return this.origin.copy(this.at(e,ia)),this}closestPointToPoint(e,t){t.subVectors(e,this.origin);let n=t.dot(this.direction);return n<0?t.copy(this.origin):t.copy(this.origin).addScaledVector(this.direction,n)}distanceToPoint(e){return Math.sqrt(this.distanceSqToPoint(e))}distanceSqToPoint(e){let t=ia.subVectors(e,this.origin).dot(this.direction);return t<0?this.origin.distanceToSquared(e):(ia.copy(this.origin).addScaledVector(this.direction,t),ia.distanceToSquared(e))}distanceSqToSegment(e,t,n,r){aa.copy(e).add(t).multiplyScalar(.5),oa.copy(t).sub(e).normalize(),sa.copy(this.origin).sub(aa);let i=e.distanceTo(t)*.5,a=-this.direction.dot(oa),o=sa.dot(this.direction),s=-sa.dot(oa),c=sa.lengthSq(),l=Math.abs(1-a*a),u,d,f,p;if(l>0){if(u=a*s-o,d=a*o-s,p=i*l,u>=0){if(d>=-p){if(d<=p){let e=1/l;u*=e,d*=e,f=u*(u+a*d+2*o)+d*(a*u+d+2*s)+c}else d=i,u=Math.max(0,-(a*d+o)),f=-u*u+d*(d+2*s)+c}else d=-i,u=Math.max(0,-(a*d+o)),f=-u*u+d*(d+2*s)+c}else d<=-p?(u=Math.max(0,-(-a*i+o)),d=u>0?-i:Math.min(Math.max(-i,-s),i),f=-u*u+d*(d+2*s)+c):d<=p?(u=0,d=Math.min(Math.max(-i,-s),i),f=d*(d+2*s)+c):(u=Math.max(0,-(a*i+o)),d=u>0?i:Math.min(Math.max(-i,-s),i),f=-u*u+d*(d+2*s)+c)}else d=a>0?-i:i,u=Math.max(0,-(a*d+o)),f=-u*u+d*(d+2*s)+c;return n&&n.copy(this.origin).addScaledVector(this.direction,u),r&&r.copy(aa).addScaledVector(oa,d),f}intersectSphere(e,t){if(e.radius<0)return null;ia.subVectors(e.center,this.origin);let n=ia.dot(this.direction),r=ia.dot(ia)-n*n,i=e.radius*e.radius;if(r>i)return null;let a=Math.sqrt(i-r),o=n-a,s=n+a;return s<0?null:o<0?this.at(s,t):this.at(o,t)}intersectsSphere(e){return e.radius<0?!1:this.distanceSqToPoint(e.center)<=e.radius*e.radius}distanceToPlane(e){let t=e.normal.dot(this.direction);if(t===0)return e.distanceToPoint(this.origin)===0?0:null;let n=-(this.origin.dot(e.normal)+e.constant)/t;return n>=0?n:null}intersectPlane(e,t){let n=this.distanceToPlane(e);return n===null?null:this.at(n,t)}intersectsPlane(e){let t=e.distanceToPoint(this.origin);return t===0||e.normal.dot(this.direction)*t<0}intersectBox(e,t){let n,r,i,a,o,s,c=1/this.direction.x,l=1/this.direction.y,u=1/this.direction.z,d=this.origin;return c>=0?(n=(e.min.x-d.x)*c,r=(e.max.x-d.x)*c):(n=(e.max.x-d.x)*c,r=(e.min.x-d.x)*c),l>=0?(i=(e.min.y-d.y)*l,a=(e.max.y-d.y)*l):(i=(e.max.y-d.y)*l,a=(e.min.y-d.y)*l),n>a||i>r||((i>n||isNaN(n))&&(n=i),(a<r||isNaN(r))&&(r=a),u>=0?(o=(e.min.z-d.z)*u,s=(e.max.z-d.z)*u):(o=(e.max.z-d.z)*u,s=(e.min.z-d.z)*u),n>s||o>r)||((o>n||n!==n)&&(n=o),(s<r||r!==r)&&(r=s),r<0)?null:this.at(n>=0?n:r,t)}intersectsBox(e){return this.intersectBox(e,ia)!==null}intersectTriangle(e,t,n,r,i){let a=this.origin,o=this.direction,s=o.x,c=o.y,l=o.z,u=e.x-a.x,d=e.y-a.y,f=e.z-a.z,p=t.x-a.x,m=t.y-a.y,h=t.z-a.z,g=n.x-a.x,_=n.y-a.y,v=n.z-a.z,y=Math.abs(s),b=Math.abs(c),x=Math.abs(l),S,C,w,T,E,D,O,k,A,j,M,N;if(y>=b&&y>=x?(w=s,D=u,A=p,N=g,s>=0?(S=c,C=l,T=d,E=f,O=m,k=h,j=_,M=v):(S=l,C=c,T=f,E=d,O=h,k=m,j=v,M=_)):b>=x?(w=c,D=d,A=m,N=_,c>=0?(S=l,C=s,T=f,E=u,O=h,k=p,j=v,M=g):(S=s,C=l,T=u,E=f,O=p,k=h,j=g,M=v)):(w=l,D=f,A=h,N=v,l>=0?(S=s,C=c,T=u,E=d,O=p,k=m,j=g,M=_):(S=c,C=s,T=d,E=u,O=m,k=p,j=_,M=g)),w===0)return null;let P=S/w,F=C/w,I=1/w,ee=T-P*D,te=E-F*D,ne=O-P*A,re=k-F*A,ie=j-P*N,ae=M-F*N,oe=ie*re-ae*ne,L=ee*ae-te*ie,se=ne*te-re*ee;if(r){if(oe<0||L<0||se<0)return null}else if((oe<0||L<0||se<0)&&(oe>0||L>0||se>0))return null;let ce=oe+L+se;if(ce===0)return null;let le=I*(oe*D+L*A+se*N);return(ce>0?le<0:le>0)?null:this.at(le/ce,i)}applyMatrix4(e){return this.origin.applyMatrix4(e),this.direction.transformDirection(e),this}equals(e){return e.origin.equals(this.origin)&&e.direction.equals(this.direction)}clone(){return new this.constructor().copy(this)}},la=class extends Ri{constructor(e){super(),this.isMeshBasicMaterial=!0,this.type=`MeshBasicMaterial`,this.color=new Y(16777215),this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.specularMap=null,this.alphaMap=null,this.envMap=null,this.envMapRotation=new Qn,this.combine=0,this.reflectivity=1,this.refractionRatio=.98,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap=`round`,this.wireframeLinejoin=`round`,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.map=e.map,this.lightMap=e.lightMap,this.lightMapIntensity=e.lightMapIntensity,this.aoMap=e.aoMap,this.aoMapIntensity=e.aoMapIntensity,this.specularMap=e.specularMap,this.alphaMap=e.alphaMap,this.envMap=e.envMap,this.envMapRotation.copy(e.envMapRotation),this.combine=e.combine,this.reflectivity=e.reflectivity,this.refractionRatio=e.refractionRatio,this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this.wireframeLinecap=e.wireframeLinecap,this.wireframeLinejoin=e.wireframeLinejoin,this.fog=e.fog,this}},ua=new J,da=new ca,fa=new Si,pa=new q,ma=new q,ha=new q,ga=new q,_a=new q,va=new q,ya=new q,ba=new q,xa=class extends hr{constructor(e=new Z,t=new la){super(),this.isMesh=!0,this.type=`Mesh`,this.geometry=e,this.material=t,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.count=1,this.updateMorphTargets()}copy(e,t){return super.copy(e,t),e.morphTargetInfluences!==void 0&&(this.morphTargetInfluences=e.morphTargetInfluences.slice()),e.morphTargetDictionary!==void 0&&(this.morphTargetDictionary=Object.assign({},e.morphTargetDictionary)),this.material=Array.isArray(e.material)?e.material.slice():e.material,this.geometry=e.geometry,this}updateMorphTargets(){let e=this.geometry.morphAttributes,t=Object.keys(e);if(t.length>0){let n=e[t[0]];if(n!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let e=0,t=n.length;e<t;e++){let t=n[e].name||String(e);this.morphTargetInfluences.push(0),this.morphTargetDictionary[t]=e}}}}getVertexPosition(e,t){let n=this.geometry,r=n.attributes.position,i=n.morphAttributes.position,a=n.morphTargetsRelative;t.fromBufferAttribute(r,e);let o=this.morphTargetInfluences;if(i&&o){va.set(0,0,0);for(let n=0,r=i.length;n<r;n++){let r=o[n],s=i[n];r!==0&&(_a.fromBufferAttribute(s,e),a?va.addScaledVector(_a,r):va.addScaledVector(_a.sub(t),r))}t.add(va)}return t}intersectsFrustum(e){return e.intersectsObject(this)}raycast(e,t){let n=this.geometry,r=this.material,i=this.matrixWorld;r!==void 0&&(n.boundingSphere===null&&n.computeBoundingSphere(),fa.copy(n.boundingSphere),fa.applyMatrix4(i),da.copy(e.ray).recast(e.near),fa.containsPoint(da.origin)===!1&&(da.intersectSphere(fa,pa)===null||da.origin.distanceToSquared(pa)>(e.far-e.near)**2)||(ua.copy(i).invert(),da.copy(e.ray).applyMatrix4(ua),(n.boundingBox===null||da.intersectsBox(n.boundingBox)!==!1)&&this._computeIntersections(e,t,da)))}_computeIntersections(e,t,n){let r,i=this.geometry,a=this.material,o=i.index,s=i.attributes.position,c=i.attributes.uv,l=i.attributes.uv1,u=i.attributes.normal,d=i.groups,f=i.drawRange;if(o!==null){if(Array.isArray(a))for(let i=0,s=d.length;i<s;i++){let s=d[i],p=a[s.materialIndex],m=Math.max(s.start,f.start),h=Math.min(o.count,Math.min(s.start+s.count,f.start+f.count));for(let i=m,a=h;i<a;i+=3){let a=o.getX(i),d=o.getX(i+1),f=o.getX(i+2);r=Ca(this,p,e,n,c,l,u,a,d,f),r&&(r.faceIndex=Math.floor(i/3),r.face.materialIndex=s.materialIndex,t.push(r))}}else{let i=Math.max(0,f.start),s=Math.min(o.count,f.start+f.count);for(let d=i,f=s;d<f;d+=3){let i=o.getX(d),s=o.getX(d+1),f=o.getX(d+2);r=Ca(this,a,e,n,c,l,u,i,s,f),r&&(r.faceIndex=Math.floor(d/3),t.push(r))}}}else if(s!==void 0){if(Array.isArray(a))for(let i=0,o=d.length;i<o;i++){let o=d[i],p=a[o.materialIndex],m=Math.max(o.start,f.start),h=Math.min(s.count,Math.min(o.start+o.count,f.start+f.count));for(let i=m,a=h;i<a;i+=3){let a=i,s=i+1,d=i+2;r=Ca(this,p,e,n,c,l,u,a,s,d),r&&(r.faceIndex=Math.floor(i/3),r.face.materialIndex=o.materialIndex,t.push(r))}}else{let i=Math.max(0,f.start),o=Math.min(s.count,f.start+f.count);for(let s=i,d=o;s<d;s+=3){let i=s,o=s+1,d=s+2;r=Ca(this,a,e,n,c,l,u,i,o,d),r&&(r.faceIndex=Math.floor(s/3),t.push(r))}}}}};function Sa(e,t,n,r,i,a,o,s){let c;if(c=t.side===1?r.intersectTriangle(o,a,i,!0,s):r.intersectTriangle(i,a,o,t.side===0,s),c===null)return null;ba.copy(s),ba.applyMatrix4(e.matrixWorld);let l=n.ray.origin.distanceTo(ba);return l<n.near||l>n.far?null:{distance:l,point:ba.clone(),object:e}}function Ca(e,t,n,r,i,a,o,s,c,l){e.getVertexPosition(s,ma),e.getVertexPosition(c,ha),e.getVertexPosition(l,ga);let u=Sa(e,t,n,r,ma,ha,ga,ya);if(u){let e=new q;Br.getBarycoord(ya,ma,ha,ga,e),i&&(u.uv=Br.getInterpolatedAttribute(i,s,c,l,e,new K)),a&&(u.uv1=Br.getInterpolatedAttribute(a,s,c,l,e,new K)),o&&(u.normal=Br.getInterpolatedAttribute(o,s,c,l,e,new q),u.normal.dot(r.direction)>0&&u.normal.multiplyScalar(-1));let t={a:s,b:c,c:l,normal:new q,materialIndex:0};Br.getNormal(ma,ha,ga,t.normal),u.face=t,u.barycoord=e}return u}var wa=new In,Ta=new In,Ea=new In,Da=new In,Oa=new J,ka=new q,Aa=new Si,ja=new J,Ma=new ca,Na=class extends xa{constructor(e,t){super(e,t),this.isSkinnedMesh=!0,this.type=`SkinnedMesh`,this.bindMode=a,this.bindMatrix=new J,this.bindMatrixInverse=new J,this.boundingBox=null,this.boundingSphere=null}computeBoundingBox(){let e=this.geometry;this.boundingBox===null&&(this.boundingBox=new Vr),this.boundingBox.makeEmpty();let t=e.getAttribute(`position`);for(let e=0;e<t.count;e++)this.getVertexPosition(e,ka),this.boundingBox.expandByPoint(ka)}computeBoundingSphere(){let e=this.geometry;this.boundingSphere===null&&(this.boundingSphere=new Si),this.boundingSphere.makeEmpty();let t=e.getAttribute(`position`);for(let e=0;e<t.count;e++)this.getVertexPosition(e,ka),this.boundingSphere.expandByPoint(ka)}copy(e,t){return super.copy(e,t),this.bindMode=e.bindMode,this.bindMatrix.copy(e.bindMatrix),this.bindMatrixInverse.copy(e.bindMatrixInverse),this.skeleton=e.skeleton,e.boundingBox!==null&&(this.boundingBox=e.boundingBox.clone()),e.boundingSphere!==null&&(this.boundingSphere=e.boundingSphere.clone()),this}raycast(e,t){let n=this.material,r=this.matrixWorld;n!==void 0&&(this.boundingSphere===null&&this.computeBoundingSphere(),Aa.copy(this.boundingSphere),Aa.applyMatrix4(r),e.ray.intersectsSphere(Aa)!==!1&&(ja.copy(r).invert(),Ma.copy(e.ray).applyMatrix4(ja),(this.boundingBox===null||Ma.intersectsBox(this.boundingBox)!==!1)&&this._computeIntersections(e,t,Ma)))}getVertexPosition(e,t){return super.getVertexPosition(e,t),this.applyBoneTransform(e,t),t}bind(e,t){this.skeleton=e,t===void 0&&(this.updateMatrixWorld(!0),this.skeleton.calculateInverses(),t=this.matrixWorld),this.bindMatrix.copy(t),this.bindMatrixInverse.copy(t).invert()}pose(){this.skeleton.pose()}normalizeSkinWeights(){let e=new In,t=this.geometry.attributes.skinWeight;for(let n=0,r=t.count;n<r;n++){e.fromBufferAttribute(t,n);let r=1/e.manhattanLength();r===1/0?e.set(1,0,0,0):e.multiplyScalar(r),t.setXYZW(n,e.x,e.y,e.z,e.w)}}updateMatrixWorld(e){super.updateMatrixWorld(e),this.bindMode===`attached`?this.bindMatrixInverse.copy(this.matrixWorld).invert():this.bindMode===`detached`?this.bindMatrixInverse.copy(this.bindMatrix).invert():U(`SkinnedMesh: Unrecognized bindMode: `+this.bindMode)}applyBoneTransform(e,t){let n=this.skeleton,r=this.geometry;Ta.fromBufferAttribute(r.attributes.skinIndex,e),Ea.fromBufferAttribute(r.attributes.skinWeight,e),t.isVector4?(wa.copy(t),t.set(0,0,0,0)):(wa.set(...t,1),t.set(0,0,0)),wa.applyMatrix4(this.bindMatrix);for(let e=0;e<4;e++){let r=Ea.getComponent(e);if(r!==0){let i=Ta.getComponent(e);Oa.multiplyMatrices(n.bones[i].matrixWorld,n.boneInverses[i]),t.addScaledVector(Da.copy(wa).applyMatrix4(Oa),r)}}return t.isVector4&&(t.w=wa.w),t.applyMatrix4(this.bindMatrixInverse)}},Pa=class extends hr{constructor(){super(),this.isBone=!0,this.type=`Bone`}},Fa=class extends Fn{constructor(e=null,t=1,n=1,r,i,a,o,s,c=u,l=u,d,f){super(null,a,o,s,c,l,r,i,d,f),this.isDataTexture=!0,this.image={data:e,width:t,height:n},this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1}},Ia=new J,La=new J,Ra=class e{constructor(e=[],t=[]){this.uuid=qt(),this.bones=e.slice(0),this.boneInverses=t,this.boneMatrices=null,this.boneTexture=null,this.init()}init(){let e=this.bones,t=this.boneInverses;if(this.boneMatrices=new Float32Array(e.length*16),t.length===0)this.calculateInverses();else if(e.length!==t.length){U(`Skeleton: Number of inverse bone matrices does not match amount of bones.`),this.boneInverses=[];for(let e=0,t=this.bones.length;e<t;e++)this.boneInverses.push(new J)}}calculateInverses(){this.boneInverses.length=0;for(let e=0,t=this.bones.length;e<t;e++){let t=new J;this.bones[e]&&t.copy(this.bones[e].matrixWorld).invert(),this.boneInverses.push(t)}}pose(){for(let e=0,t=this.bones.length;e<t;e++){let t=this.bones[e];t&&t.matrixWorld.copy(this.boneInverses[e]).invert()}for(let e=0,t=this.bones.length;e<t;e++){let t=this.bones[e];t&&(t.parent&&t.parent.isBone?(t.matrix.copy(t.parent.matrixWorld).invert(),t.matrix.multiply(t.matrixWorld)):t.matrix.copy(t.matrixWorld),t.matrix.decompose(t.position,t.quaternion,t.scale))}}update(){let e=this.bones,t=this.boneInverses,n=this.boneMatrices,r=this.boneTexture;for(let r=0,i=e.length;r<i;r++){let i=e[r]?e[r].matrixWorld:La;Ia.multiplyMatrices(i,t[r]),Ia.toArray(n,r*16)}r!==null&&(r.needsUpdate=!0)}clone(){return new e(this.bones,this.boneInverses)}computeBoneTexture(){let e=Math.sqrt(this.bones.length*4);e=Math.ceil(e/4)*4,e=Math.max(e,4);let t=new Float32Array(e*e*4);t.set(this.boneMatrices);let n=new Fa(t,e,e,F,E);return n.needsUpdate=!0,this.boneMatrices=t,this.boneTexture=n,this}getBoneByName(e){for(let t=0,n=this.bones.length;t<n;t++){let n=this.bones[t];if(n.name===e)return n}}dispose(){this.boneTexture!==null&&(this.boneTexture.dispose(),this.boneTexture=null)}fromJSON(e,t){this.uuid=e.uuid;for(let n=0,r=e.bones.length;n<r;n++){let r=e.bones[n],i=t[r];i===void 0&&(U(`Skeleton: No bone found with UUID:`,r),i=new Pa),this.bones.push(i),this.boneInverses.push(new J().fromArray(e.boneInverses[n]))}return this.init(),this}toJSON(){let e={metadata:{version:4.7,type:`Skeleton`,generator:`Skeleton.toJSON`},bones:[],boneInverses:[]};e.uuid=this.uuid;let t=this.bones,n=this.boneInverses;for(let r=0,i=t.length;r<i;r++){let i=t[r];e.bones.push(i.uuid);let a=n[r];e.boneInverses.push(a.toArray())}return e}},za=class extends ui{constructor(e,t,n,r=1){super(e,t,n),this.isInstancedBufferAttribute=!0,this.meshPerAttribute=r}copy(e){return super.copy(e),this.meshPerAttribute=e.meshPerAttribute,this}toJSON(){let e=super.toJSON();return e.meshPerAttribute=this.meshPerAttribute,e.isInstancedBufferAttribute=!0,e}},Ba=new J,Va=new J,Ha=[],Ua=new Vr,Wa=new J,Ga=new xa,Ka=new Si,qa=class extends xa{constructor(e,t,n){super(e,t),this.isInstancedMesh=!0,this.instanceMatrix=new za(new Float32Array(n*16),16),this.instanceColor=null,this.morphTexture=null,this.count=n,this.boundingBox=null,this.boundingSphere=null;for(let e=0;e<n;e++)this.setMatrixAt(e,Wa)}computeBoundingBox(){let e=this.geometry,t=this.count;this.boundingBox===null&&(this.boundingBox=new Vr),e.boundingBox===null&&e.computeBoundingBox(),this.boundingBox.makeEmpty();for(let n=0;n<t;n++)this.getMatrixAt(n,Ba),Ua.copy(e.boundingBox).applyMatrix4(Ba),this.boundingBox.union(Ua)}computeBoundingSphere(){let e=this.geometry,t=this.count;this.boundingSphere===null&&(this.boundingSphere=new Si),e.boundingSphere===null&&e.computeBoundingSphere(),this.boundingSphere.makeEmpty();for(let n=0;n<t;n++)this.getMatrixAt(n,Ba),Ka.copy(e.boundingSphere).applyMatrix4(Ba),this.boundingSphere.union(Ka)}copy(e,t){return super.copy(e,t),this.instanceMatrix.copy(e.instanceMatrix),e.morphTexture!==null&&(this.morphTexture=e.morphTexture.clone()),e.instanceColor!==null&&(this.instanceColor=e.instanceColor.clone()),this.count=e.count,e.boundingBox!==null&&(this.boundingBox=e.boundingBox.clone()),e.boundingSphere!==null&&(this.boundingSphere=e.boundingSphere.clone()),this}getColorAt(e,t){return this.instanceColor===null?t.setRGB(1,1,1):t.fromArray(this.instanceColor.array,e*3)}getMatrixAt(e,t){return t.fromArray(this.instanceMatrix.array,e*16)}getMorphAt(e,t){let n=t.morphTargetInfluences,r=this.morphTexture.source.data.data,i=e*(n.length+1)+1;for(let e=0;e<n.length;e++)n[e]=r[i+e]}raycast(e,t){let n=this.matrixWorld,r=this.count;if(Ga.geometry=this.geometry,Ga.material=this.material,Ga.material!==void 0&&(this.boundingSphere===null&&this.computeBoundingSphere(),Ka.copy(this.boundingSphere),Ka.applyMatrix4(n),e.ray.intersectsSphere(Ka)!==!1))for(let i=0;i<r;i++){this.getMatrixAt(i,Ba),Va.multiplyMatrices(n,Ba),Ga.matrixWorld=Va,Ga.raycast(e,Ha);for(let e=0,n=Ha.length;e<n;e++){let n=Ha[e];n.instanceId=i,n.object=this,t.push(n)}Ha.length=0}}setColorAt(e,t){return this.instanceColor===null&&(this.instanceColor=new za(new Float32Array(this.instanceMatrix.count*3).fill(1),3)),t.toArray(this.instanceColor.array,e*3),this}setMatrixAt(e,t){return t.toArray(this.instanceMatrix.array,e*16),this}setMorphAt(e,t){let n=t.morphTargetInfluences,r=n.length+1;this.morphTexture===null&&(this.morphTexture=new Fa(new Float32Array(r*this.count),r,this.count,te,E));let i=this.morphTexture.source.data.data,a=0;for(let e=0;e<n.length;e++)a+=n[e];let o=this.geometry.morphTargetsRelative?1:1-a,s=r*e;return i[s]=o,i.set(n,s+1),this}updateMorphTargets(){}dispose(){super.dispose(),this.morphTexture!==null&&(this.morphTexture.dispose(),this.morphTexture=null)}},Ja=new Si,Ya=new K(.5,.5),Xa=new q,Za=class{constructor(e=new Ii,t=new Ii,n=new Ii,r=new Ii,i=new Ii,a=new Ii){this.planes=[e,t,n,r,i,a]}set(e,t,n,r,i,a){let o=this.planes;return o[0].copy(e),o[1].copy(t),o[2].copy(n),o[3].copy(r),o[4].copy(i),o[5].copy(a),this}copy(e){let t=this.planes;for(let n=0;n<6;n++)t[n].copy(e.planes[n]);return this}setFromProjectionMatrix(e,t=bt,n=!1){let r=this.planes,i=e.elements,a=i[0],o=i[1],s=i[2],c=i[3],l=i[4],u=i[5],d=i[6],f=i[7],p=i[8],m=i[9],h=i[10],g=i[11],_=i[12],v=i[13],y=i[14],b=i[15];if(r[0].setComponents(c-a,f-l,g-p,b-_).normalize(),r[1].setComponents(c+a,f+l,g+p,b+_).normalize(),r[2].setComponents(c+o,f+u,g+m,b+v).normalize(),r[3].setComponents(c-o,f-u,g-m,b-v).normalize(),n)r[4].setComponents(s,d,h,y).normalize(),r[5].setComponents(c-s,f-d,g-h,b-y).normalize();else if(r[4].setComponents(c-s,f-d,g-h,b-y).normalize(),t===2e3)r[5].setComponents(c+s,f+d,g+h,b+y).normalize();else if(t===2001)r[5].setComponents(s,d,h,y).normalize();else throw Error(`THREE.Frustum.setFromProjectionMatrix(): Invalid coordinate system: `+t);return this}intersectsObject(e){if(e.boundingSphere!==void 0)e.boundingSphere===null&&e.computeBoundingSphere(),Ja.copy(e.boundingSphere).applyMatrix4(e.matrixWorld);else{let t=e.geometry;t.boundingSphere===null&&t.computeBoundingSphere(),Ja.copy(t.boundingSphere).applyMatrix4(e.matrixWorld)}return this.intersectsSphere(Ja)}intersectsSprite(e){return Ja.center.set(0,0,0),Ja.radius=.7071067811865476+Ya.distanceTo(e.center),Ja.applyMatrix4(e.matrixWorld),this.intersectsSphere(Ja)}intersectsSphere(e){let t=this.planes,n=e.center,r=-e.radius;for(let e=0;e<6;e++)if(t[e].distanceToPoint(n)<r)return!1;return!0}intersectsBox(e){let t=this.planes;for(let n=0;n<6;n++){let r=t[n];if(Xa.x=r.normal.x>0?e.max.x:e.min.x,Xa.y=r.normal.y>0?e.max.y:e.min.y,Xa.z=r.normal.z>0?e.max.z:e.min.z,r.distanceToPoint(Xa)<0)return!1}return!0}containsPoint(e){let t=this.planes;for(let n=0;n<6;n++)if(t[n].distanceToPoint(e)<0)return!1;return!0}clone(){return new this.constructor().copy(this)}},Qa=new J,$a=class e{constructor(){this.coordinateSystem=bt,this._frustums=[],this._count=0}setFromArrayCamera(e){let t=e.cameras,n=this._frustums;for(let e=0;e<t.length;e++){let r=t[e];Qa.multiplyMatrices(r.projectionMatrix,r.matrixWorldInverse),n[e]===void 0&&(n[e]=new Za),n[e].setFromProjectionMatrix(Qa,r.coordinateSystem,r.reversedDepth)}return this._count=t.length,this}intersectsObject(e){let t=this._frustums;for(let n=0;n<this._count;n++)if(t[n].intersectsObject(e))return!0;return!1}intersectsSprite(e){let t=this._frustums;for(let n=0;n<this._count;n++)if(t[n].intersectsSprite(e))return!0;return!1}intersectsSphere(e){let t=this._frustums;for(let n=0;n<this._count;n++)if(t[n].intersectsSphere(e))return!0;return!1}intersectsBox(e){let t=this._frustums;for(let n=0;n<this._count;n++)if(t[n].intersectsBox(e))return!0;return!1}containsPoint(e){let t=this._frustums;for(let n=0;n<this._count;n++)if(t[n].containsPoint(e))return!0;return!1}copy(e){this.coordinateSystem=e.coordinateSystem;let t=this._frustums,n=e._frustums;for(let r=0;r<e._count;r++)t[r]===void 0&&(t[r]=new Za),t[r].copy(n[r]);return this._count=e._count,this}clone(){return new e().copy(this)}};function eo(e,t){return e-t}function to(e,t){return e.z-t.z}function no(e,t){return t.z-e.z}var ro=class{constructor(){this.index=0,this.pool=[],this.list=[]}push(e,t,n,r){let i=this.pool,a=this.list;this.index>=i.length&&i.push({start:-1,count:-1,z:-1,index:-1});let o=i[this.index];a.push(o),this.index++,o.start=e,o.count=t,o.z=n,o.index=r}reset(){this.list.length=0,this.index=0}},io=new J,ao=new Y(1,1,1),oo=new Za,so=new $a,co=new Vr,lo=new Si,uo=new q,fo=new q,po=new q,mo=new ro,ho=new xa,go=[];function _o(e,t,n=0){let r=t.itemSize;if(e.isInterleavedBufferAttribute||e.array.constructor!==t.array.constructor){let i=e.count;for(let a=0;a<i;a++)for(let i=0;i<r;i++)t.setComponent(a+n,i,e.getComponent(a,i))}else t.array.set(e.array,n*r);t.needsUpdate=!0}function vo(e,t){if(e.constructor!==t.constructor){let n=Math.min(e.length,t.length);for(let r=0;r<n;r++)t[r]=e[r]}else{let n=Math.min(e.length,t.length);t.set(new e.constructor(e.buffer,0,n))}}var yo=class extends xa{constructor(e,t,n=t*2,r){super(new Z,r),this.isBatchedMesh=!0,this.perObjectFrustumCulled=!0,this.sortObjects=!0,this.boundingBox=null,this.boundingSphere=null,this.customSort=null,this._instanceInfo=[],this._geometryInfo=[],this._availableInstanceIds=[],this._availableGeometryIds=[],this._nextIndexStart=0,this._nextVertexStart=0,this._geometryCount=0,this._visibilityChanged=!0,this._geometryInitialized=!1,this._maxInstanceCount=e,this._maxVertexCount=t,this._maxIndexCount=n,this._multiDrawCounts=new Int32Array(e),this._multiDrawStarts=new Int32Array(e),this._multiDrawCount=0,this._multiDrawBytesPerElement=1,this._matricesTexture=null,this._indirectTexture=null,this._colorsTexture=null,this._initMatricesTexture(),this._initIndirectTexture()}get maxInstanceCount(){return this._maxInstanceCount}get instanceCount(){return this._instanceInfo.length-this._availableInstanceIds.length}get unusedVertexCount(){return this._maxVertexCount-this._nextVertexStart}get unusedIndexCount(){return this._maxIndexCount-this._nextIndexStart}_initMatricesTexture(){let e=Math.sqrt(this._maxInstanceCount*4);e=Math.ceil(e/4)*4,e=Math.max(e,4);let t=new Fa(new Float32Array(e*e*4),e,e,F,E);this._matricesTexture=t}_initIndirectTexture(){let e=Math.sqrt(this._maxInstanceCount);e=Math.ceil(e);let t=new Fa(new Uint32Array(e*e),e,e,ne,T);this._indirectTexture=t}_initColorsTexture(){let e=Math.sqrt(this._maxInstanceCount);e=Math.ceil(e);let t=new Fa(new Float32Array(e*e*4).fill(1),e,e,F,E);t.colorSpace=wn.workingColorSpace,this._colorsTexture=t}_initializeGeometry(e){let t=this.geometry,n=this._maxVertexCount,r=this._maxIndexCount;if(this._geometryInitialized===!1){for(let r in e.attributes){let{array:i,itemSize:a,normalized:o}=e.getAttribute(r),s=new ui(new i.constructor(n*a),a,o);t.setAttribute(r,s)}if(e.getIndex()!==null){let e=n>65535?new Uint32Array(r):new Uint16Array(r);t.setIndex(new ui(e,1))}this._geometryInitialized=!0}}_validateGeometry(e){let t=this.geometry;if(!!e.getIndex()!=!!t.getIndex())throw Error(`THREE.BatchedMesh: All geometries must consistently have "index".`);for(let n in t.attributes){if(!e.hasAttribute(n))throw Error(`THREE.BatchedMesh: Added geometry missing "${n}". All geometries must have consistent attributes.`);let r=e.getAttribute(n),i=t.getAttribute(n);if(r.itemSize!==i.itemSize||r.normalized!==i.normalized)throw Error(`THREE.BatchedMesh: All attributes must have a consistent itemSize and normalized value.`)}}validateInstanceId(e){let t=this._instanceInfo;if(e<0||e>=t.length||t[e].active===!1)throw Error(`THREE.BatchedMesh: Invalid instanceId ${e}. Instance is either out of range or has been deleted.`)}validateGeometryId(e){let t=this._geometryInfo;if(e<0||e>=t.length||t[e].active===!1)throw Error(`THREE.BatchedMesh: Invalid geometryId ${e}. Geometry is either out of range or has been deleted.`)}setCustomSort(e){return this.customSort=e,this}computeBoundingBox(){this.boundingBox===null&&(this.boundingBox=new Vr);let e=this.boundingBox,t=this._instanceInfo;e.makeEmpty();for(let n=0,r=t.length;n<r;n++){if(t[n].active===!1)continue;let r=t[n].geometryIndex;this.getMatrixAt(n,io),this.getBoundingBoxAt(r,co).applyMatrix4(io),e.union(co)}}computeBoundingSphere(){this.boundingSphere===null&&(this.boundingSphere=new Si);let e=this.boundingSphere,t=this._instanceInfo;e.makeEmpty();for(let n=0,r=t.length;n<r;n++){if(t[n].active===!1)continue;let r=t[n].geometryIndex;this.getMatrixAt(n,io),this.getBoundingSphereAt(r,lo).applyMatrix4(io),e.union(lo)}}addInstance(e){if(this._instanceInfo.length>=this.maxInstanceCount&&this._availableInstanceIds.length===0)throw Error(`THREE.BatchedMesh: Maximum item count reached.`);let t={visible:!0,active:!0,geometryIndex:e},n=null;this._availableInstanceIds.length>0?(this._availableInstanceIds.sort(eo),n=this._availableInstanceIds.shift(),this._instanceInfo[n]=t):(n=this._instanceInfo.length,this._instanceInfo.push(t));let r=this._matricesTexture;io.identity().toArray(r.image.data,n*16),r.needsUpdate=!0;let i=this._colorsTexture;return i&&(ao.toArray(i.image.data,n*4),i.needsUpdate=!0),this._visibilityChanged=!0,n}addGeometry(e,t=-1,n=-1){this._initializeGeometry(e),this._validateGeometry(e);let r={vertexStart:-1,vertexCount:-1,reservedVertexCount:-1,indexStart:-1,indexCount:-1,reservedIndexCount:-1,start:-1,count:-1,boundingBox:null,boundingSphere:null,active:!0},i=this._geometryInfo;r.vertexStart=this._nextVertexStart,r.reservedVertexCount=t===-1?e.getAttribute(`position`).count:t;let a=e.getIndex();if(a!==null&&(r.indexStart=this._nextIndexStart,r.reservedIndexCount=n===-1?a.count:n),r.indexStart!==-1&&r.indexStart+r.reservedIndexCount>this._maxIndexCount||r.vertexStart+r.reservedVertexCount>this._maxVertexCount)throw Error(`THREE.BatchedMesh: Reserved space request exceeds the maximum buffer size.`);let o;return this._availableGeometryIds.length>0?(this._availableGeometryIds.sort(eo),o=this._availableGeometryIds.shift(),i[o]=r):(o=this._geometryCount,this._geometryCount++,i.push(r)),this.setGeometryAt(o,e),this._nextIndexStart=r.indexStart+r.reservedIndexCount,this._nextVertexStart=r.vertexStart+r.reservedVertexCount,o}setGeometryAt(e,t){if(e>=this._geometryCount)throw Error(`THREE.BatchedMesh: Maximum geometry count reached.`);this._validateGeometry(t);let n=this.geometry,r=n.getIndex()!==null,i=n.getIndex(),a=t.getIndex(),o=this._geometryInfo[e];if(r&&a.count>o.reservedIndexCount||t.attributes.position.count>o.reservedVertexCount)throw Error(`THREE.BatchedMesh: Reserved space not large enough for provided geometry.`);let s=o.vertexStart,c=o.reservedVertexCount;o.vertexCount=t.getAttribute(`position`).count;for(let e in n.attributes){let r=t.getAttribute(e),i=n.getAttribute(e);_o(r,i,s);let a=r.itemSize;for(let e=r.count,t=c;e<t;e++){let t=s+e;for(let e=0;e<a;e++)i.setComponent(t,e,0)}i.needsUpdate=!0,i.addUpdateRange(s*a,c*a)}if(r){let e=o.indexStart,n=o.reservedIndexCount;o.indexCount=t.getIndex().count;for(let t=0;t<a.count;t++)i.setX(e+t,s+a.getX(t));for(let t=a.count,r=n;t<r;t++)i.setX(e+t,s);i.needsUpdate=!0,i.addUpdateRange(e,o.reservedIndexCount)}return o.start=r?o.indexStart:o.vertexStart,o.count=r?o.indexCount:o.vertexCount,o.boundingBox=null,t.boundingBox!==null&&(o.boundingBox=t.boundingBox.clone()),o.boundingSphere=null,t.boundingSphere!==null&&(o.boundingSphere=t.boundingSphere.clone()),this._visibilityChanged=!0,e}deleteGeometry(e){let t=this._geometryInfo;if(e>=t.length||t[e].active===!1)return this;let n=this._instanceInfo;for(let t=0,r=n.length;t<r;t++)n[t].active&&n[t].geometryIndex===e&&this.deleteInstance(t);return t[e].active=!1,this._availableGeometryIds.push(e),this._visibilityChanged=!0,this}deleteInstance(e){return this.validateInstanceId(e),this._instanceInfo[e].active=!1,this._availableInstanceIds.push(e),this._visibilityChanged=!0,this}optimize(){let e=0,t=0,n=this._geometryInfo,r=n.map((e,t)=>t).sort((e,t)=>n[e].vertexStart-n[t].vertexStart),i=this.geometry;for(let a=0,o=n.length;a<o;a++){let o=n[r[a]];if(o.active!==!1){if(i.index!==null){if(o.indexStart!==t){let{indexStart:n,vertexStart:r,reservedIndexCount:a}=o,s=i.index,c=s.array,l=e-r;for(let e=n;e<n+a;e++)c[e]=c[e]+l;s.array.copyWithin(t,n,n+a),s.addUpdateRange(t,a),s.needsUpdate=!0,o.indexStart=t}t+=o.reservedIndexCount}if(o.vertexStart!==e){let{vertexStart:t,reservedVertexCount:n}=o,r=i.attributes;for(let i in r){let a=r[i],{array:o,itemSize:s}=a;o.copyWithin(e*s,t*s,(t+n)*s),a.addUpdateRange(e*s,n*s),a.needsUpdate=!0}o.vertexStart=e}e+=o.reservedVertexCount,o.start=i.index?o.indexStart:o.vertexStart}}return this._nextIndexStart=t,this._nextVertexStart=e,this._visibilityChanged=!0,this}getBoundingBoxAt(e,t){if(e>=this._geometryCount)return null;let n=this.geometry,r=this._geometryInfo[e];if(r.boundingBox===null){let e=new Vr,t=n.index,i=n.attributes.position;for(let n=r.start,a=r.start+r.count;n<a;n++){let r=n;t&&(r=t.getX(r)),e.expandByPoint(uo.fromBufferAttribute(i,r))}r.boundingBox=e}return t.copy(r.boundingBox),t}getBoundingSphereAt(e,t){if(e>=this._geometryCount)return null;let n=this.geometry,r=this._geometryInfo[e];if(r.boundingSphere===null){let t=new Si;this.getBoundingBoxAt(e,co),co.getCenter(t.center);let i=n.index,a=n.attributes.position,o=0;for(let e=r.start,n=r.start+r.count;e<n;e++){let n=e;i&&(n=i.getX(n)),uo.fromBufferAttribute(a,n),o=Math.max(o,t.center.distanceToSquared(uo))}t.radius=Math.sqrt(o),r.boundingSphere=t}return t.copy(r.boundingSphere),t}setMatrixAt(e,t){this.validateInstanceId(e);let n=this._matricesTexture,r=this._matricesTexture.image.data;return t.toArray(r,e*16),n.needsUpdate=!0,this}getMatrixAt(e,t){return this.validateInstanceId(e),t.fromArray(this._matricesTexture.image.data,e*16)}setColorAt(e,t){return this.validateInstanceId(e),this._colorsTexture===null&&this._initColorsTexture(),t.toArray(this._colorsTexture.image.data,e*4),this._colorsTexture.needsUpdate=!0,this}getColorAt(e,t){return this.validateInstanceId(e),this._colorsTexture===null?t.isVector4?t.set(1,1,1,1):t.setRGB(1,1,1):t.fromArray(this._colorsTexture.image.data,e*4)}setVisibleAt(e,t){return this.validateInstanceId(e),this._instanceInfo[e].visible===t||(this._instanceInfo[e].visible=t,this._visibilityChanged=!0),this}getVisibleAt(e){return this.validateInstanceId(e),this._instanceInfo[e].visible}setGeometryIdAt(e,t){return this.validateInstanceId(e),this.validateGeometryId(t),this._instanceInfo[e].geometryIndex=t,this._visibilityChanged=!0,this}getGeometryIdAt(e){return this.validateInstanceId(e),this._instanceInfo[e].geometryIndex}getGeometryRangeAt(e,t={}){this.validateGeometryId(e);let n=this._geometryInfo[e];return t.vertexStart=n.vertexStart,t.vertexCount=n.vertexCount,t.reservedVertexCount=n.reservedVertexCount,t.indexStart=n.indexStart,t.indexCount=n.indexCount,t.reservedIndexCount=n.reservedIndexCount,t.start=n.start,t.count=n.count,t}setInstanceCount(e){let t=this._availableInstanceIds,n=this._instanceInfo;for(t.sort(eo);t[t.length-1]===n.length-1;)n.pop(),t.pop();if(e<n.length)throw Error(`THREE.BatchedMesh: Instance ids outside the range ${e} are being used. Cannot shrink instance count.`);let r=new Int32Array(e),i=new Int32Array(e);vo(this._multiDrawCounts,r),vo(this._multiDrawStarts,i),this._multiDrawCounts=r,this._multiDrawStarts=i,this._maxInstanceCount=e;let a=this._indirectTexture,o=this._matricesTexture,s=this._colorsTexture;a.dispose(),this._initIndirectTexture(),vo(a.image.data,this._indirectTexture.image.data),o.dispose(),this._initMatricesTexture(),vo(o.image.data,this._matricesTexture.image.data),s&&(s.dispose(),this._initColorsTexture(),vo(s.image.data,this._colorsTexture.image.data))}setGeometrySize(e,t){let n=[...this._geometryInfo].filter(e=>e.active);if(Math.max(...n.map(e=>e.vertexStart+e.reservedVertexCount))>e)throw Error(`THREE.BatchedMesh: Geometry vertex values are being used outside the range ${t}. Cannot shrink further.`);if(this.geometry.index&&Math.max(...n.map(e=>e.indexStart+e.reservedIndexCount))>t)throw Error(`THREE.BatchedMesh: Geometry index values are being used outside the range ${t}. Cannot shrink further.`);let r=this.geometry;r.dispose(),this._maxVertexCount=e,this._maxIndexCount=t,this._geometryInitialized&&(this._geometryInitialized=!1,this.geometry=new Z,this._initializeGeometry(r));let i=this.geometry;r.index&&vo(r.index.array,i.index.array);for(let e in r.attributes)vo(r.attributes[e].array,i.attributes[e].array)}raycast(e,t){let n=this._instanceInfo,r=this._geometryInfo,i=this.matrixWorld,a=this.geometry;ho.material=this.material,ho.geometry.index=a.index,ho.geometry.attributes=a.attributes,ho.geometry.boundingBox===null&&(ho.geometry.boundingBox=new Vr),ho.geometry.boundingSphere===null&&(ho.geometry.boundingSphere=new Si);for(let a=0,o=n.length;a<o;a++){if(!n[a].visible||!n[a].active)continue;let o=n[a].geometryIndex,s=r[o];ho.geometry.setDrawRange(s.start,s.count),this.getMatrixAt(a,ho.matrixWorld).premultiply(i),this.getBoundingBoxAt(o,ho.geometry.boundingBox),this.getBoundingSphereAt(o,ho.geometry.boundingSphere),ho.raycast(e,go);for(let e=0,n=go.length;e<n;e++){let n=go[e];n.object=this,n.batchId=a,t.push(n)}go.length=0}ho.material=null,ho.geometry.index=null,ho.geometry.attributes={},ho.geometry.setDrawRange(0,1/0)}copy(e){return super.copy(e),this.geometry=e.geometry.clone(),this.perObjectFrustumCulled=e.perObjectFrustumCulled,this.sortObjects=e.sortObjects,this.boundingBox=e.boundingBox===null?null:e.boundingBox.clone(),this.boundingSphere=e.boundingSphere===null?null:e.boundingSphere.clone(),this._geometryInfo=e._geometryInfo.map(e=>({...e,boundingBox:e.boundingBox===null?null:e.boundingBox.clone(),boundingSphere:e.boundingSphere===null?null:e.boundingSphere.clone()})),this._instanceInfo=e._instanceInfo.map(e=>({...e})),this._availableInstanceIds=e._availableInstanceIds.slice(),this._availableGeometryIds=e._availableGeometryIds.slice(),this._nextIndexStart=e._nextIndexStart,this._nextVertexStart=e._nextVertexStart,this._geometryCount=e._geometryCount,this._maxInstanceCount=e._maxInstanceCount,this._maxVertexCount=e._maxVertexCount,this._maxIndexCount=e._maxIndexCount,this._geometryInitialized=e._geometryInitialized,this._multiDrawCounts=e._multiDrawCounts.slice(),this._multiDrawStarts=e._multiDrawStarts.slice(),this._multiDrawBytesPerElement=e._multiDrawBytesPerElement,this._indirectTexture=e._indirectTexture.clone(),this._indirectTexture.image.data=this._indirectTexture.image.data.slice(),this._matricesTexture=e._matricesTexture.clone(),this._matricesTexture.image.data=this._matricesTexture.image.data.slice(),this._colorsTexture!==null&&(this._colorsTexture=e._colorsTexture.clone(),this._colorsTexture.image.data=this._colorsTexture.image.data.slice()),this}dispose(){super.dispose(),this.geometry.dispose(),this._matricesTexture.dispose(),this._matricesTexture=null,this._indirectTexture.dispose(),this._indirectTexture=null,this._colorsTexture!==null&&(this._colorsTexture.dispose(),this._colorsTexture=null)}onBeforeRender(e,t,n,r,i){if(!this._visibilityChanged&&!this.perObjectFrustumCulled&&!this.sortObjects)return;let a=r.getIndex(),o=a===null?1:a.array.BYTES_PER_ELEMENT,s=1;i.wireframe&&(s=2,o=r.attributes.position.count>65535?4:2);let c=this._instanceInfo,l=this._multiDrawStarts,u=this._multiDrawCounts,d=this._geometryInfo,f=this.perObjectFrustumCulled,p=this._indirectTexture,m=p.image.data,h=n.isArrayCamera?so:oo;f&&(n.isArrayCamera?h.setFromArrayCamera(n):(io.multiplyMatrices(n.projectionMatrix,n.matrixWorldInverse).multiply(this.matrixWorld),h.setFromProjectionMatrix(io,n.coordinateSystem,n.reversedDepth)));let g=0;if(this.sortObjects){io.copy(this.matrixWorld).invert(),uo.setFromMatrixPosition(n.matrixWorld).applyMatrix4(io),fo.set(0,0,-1).transformDirection(n.matrixWorld).transformDirection(io);for(let e=0,t=c.length;e<t;e++)if(c[e].visible&&c[e].active){let t=c[e].geometryIndex;this.getMatrixAt(e,io),this.getBoundingSphereAt(t,lo).applyMatrix4(io);let n=!1;if(f&&(n=!h.intersectsSphere(lo)),!n){let n=d[t],r=po.subVectors(lo.center,uo).dot(fo);mo.push(n.start,n.count,r,e)}}let e=mo.list,t=this.customSort;t===null?e.sort(i.transparent?no:to):t.call(this,e,n);for(let t=0,n=e.length;t<n;t++){let n=e[t];l[g]=n.start*o*s,u[g]=n.count*s,m[g]=n.index,g++}mo.reset()}else for(let e=0,t=c.length;e<t;e++)if(c[e].visible&&c[e].active){let t=c[e].geometryIndex,n=!1;if(f&&(this.getMatrixAt(e,io),this.getBoundingSphereAt(t,lo).applyMatrix4(io),n=!h.intersectsSphere(lo)),!n){let n=d[t];l[g]=n.start*o*s,u[g]=n.count*s,m[g]=e,g++}}p.needsUpdate=!0,this._multiDrawCount=g,this._multiDrawBytesPerElement=o,this._visibilityChanged=!1}onBeforeShadow(e,t,n,r,i,a){this.onBeforeRender(e,null,r,i,a)}},bo=class extends Ri{constructor(e){super(),this.isLineBasicMaterial=!0,this.type=`LineBasicMaterial`,this.color=new Y(16777215),this.map=null,this.linewidth=1,this.linecap=`round`,this.linejoin=`round`,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.map=e.map,this.linewidth=e.linewidth,this.linecap=e.linecap,this.linejoin=e.linejoin,this.fog=e.fog,this}},xo=new q,So=new q,Co=new J,wo=new ca,To=new Si,Eo=new q,Do=new q,Oo=class extends hr{constructor(e=new Z,t=new bo){super(),this.isLine=!0,this.type=`Line`,this.geometry=e,this.material=t,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.updateMorphTargets()}copy(e,t){return super.copy(e,t),this.material=Array.isArray(e.material)?e.material.slice():e.material,this.geometry=e.geometry,this}computeLineDistances(){let e=this.geometry;if(e.index===null){let t=e.attributes.position,n=[0];for(let e=1,r=t.count;e<r;e++)xo.fromBufferAttribute(t,e-1),So.fromBufferAttribute(t,e),n[e]=n[e-1],n[e]+=xo.distanceTo(So);e.setAttribute(`lineDistance`,new X(n,1))}else U(`Line.computeLineDistances(): Computation only possible with non-indexed BufferGeometry.`);return this}intersectsFrustum(e){return e.intersectsObject(this)}raycast(e,t){let n=this.geometry,r=this.matrixWorld,i=e.params.Line.threshold,a=n.drawRange;if(n.boundingSphere===null&&n.computeBoundingSphere(),To.copy(n.boundingSphere),To.applyMatrix4(r),To.radius+=i,e.ray.intersectsSphere(To)===!1)return;Co.copy(r).invert(),wo.copy(e.ray).applyMatrix4(Co);let o=i/((this.scale.x+this.scale.y+this.scale.z)/3),s=o*o,c=this.isLineSegments?2:1,l=n.index,u=n.attributes.position;if(l!==null){let n=Math.max(0,a.start),r=Math.min(l.count,a.start+a.count);for(let i=n,a=r-1;i<a;i+=c){let n=l.getX(i),r=l.getX(i+1),a=ko(this,e,wo,s,n,r,i);a&&t.push(a)}if(this.isLineLoop){let i=l.getX(r-1),a=l.getX(n),o=ko(this,e,wo,s,i,a,r-1);o&&t.push(o)}}else{let n=Math.max(0,a.start),r=Math.min(u.count,a.start+a.count);for(let i=n,a=r-1;i<a;i+=c){let n=ko(this,e,wo,s,i,i+1,i);n&&t.push(n)}if(this.isLineLoop){let i=ko(this,e,wo,s,r-1,n,r-1);i&&t.push(i)}}}updateMorphTargets(){let e=this.geometry.morphAttributes,t=Object.keys(e);if(t.length>0){let n=e[t[0]];if(n!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let e=0,t=n.length;e<t;e++){let t=n[e].name||String(e);this.morphTargetInfluences.push(0),this.morphTargetDictionary[t]=e}}}}};function ko(e,t,n,r,i,a,o){let s=e.geometry.attributes.position;if(xo.fromBufferAttribute(s,i),So.fromBufferAttribute(s,a),n.distanceSqToSegment(xo,So,Eo,Do)>r)return;Eo.applyMatrix4(e.matrixWorld);let c=t.ray.origin.distanceTo(Eo);if(!(c<t.near||c>t.far))return{distance:c,point:Do.clone().applyMatrix4(e.matrixWorld),index:o,face:null,faceIndex:null,barycoord:null,object:e}}var Ao=new q,jo=new q,Mo=class extends Oo{constructor(e,t){super(e,t),this.isLineSegments=!0,this.type=`LineSegments`}computeLineDistances(){let e=this.geometry;if(e.index===null){let t=e.attributes.position,n=[];for(let e=0,r=t.count;e<r;e+=2)Ao.fromBufferAttribute(t,e),jo.fromBufferAttribute(t,e+1),n[e]=e===0?0:n[e-1],n[e+1]=n[e]+Ao.distanceTo(jo);e.setAttribute(`lineDistance`,new X(n,1))}else U(`LineSegments.computeLineDistances(): Computation only possible with non-indexed BufferGeometry.`);return this}},No=class extends Oo{constructor(e,t){super(e,t),this.isLineLoop=!0,this.type=`LineLoop`}},Po=class extends Ri{constructor(e){super(),this.isPointsMaterial=!0,this.type=`PointsMaterial`,this.color=new Y(16777215),this.map=null,this.alphaMap=null,this.size=1,this.sizeAttenuation=!0,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.map=e.map,this.alphaMap=e.alphaMap,this.size=e.size,this.sizeAttenuation=e.sizeAttenuation,this.fog=e.fog,this}},Fo=new J,Io=new ca,Lo=new Si,Ro=new q,zo=class extends hr{constructor(e=new Z,t=new Po){super(),this.isPoints=!0,this.type=`Points`,this.geometry=e,this.material=t,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.updateMorphTargets()}copy(e,t){return super.copy(e,t),this.material=Array.isArray(e.material)?e.material.slice():e.material,this.geometry=e.geometry,this}intersectsFrustum(e){return e.intersectsObject(this)}raycast(e,t){let n=this.geometry,r=this.matrixWorld,i=e.params.Points.threshold,a=n.drawRange;if(n.boundingSphere===null&&n.computeBoundingSphere(),Lo.copy(n.boundingSphere),Lo.applyMatrix4(r),Lo.radius+=i,e.ray.intersectsSphere(Lo)===!1)return;Fo.copy(r).invert(),Io.copy(e.ray).applyMatrix4(Fo);let o=i/((this.scale.x+this.scale.y+this.scale.z)/3),s=o*o,c=n.index,l=n.attributes.position;if(c!==null){let n=Math.max(0,a.start),i=Math.min(c.count,a.start+a.count);for(let a=n,o=i;a<o;a++){let n=c.getX(a);Ro.fromBufferAttribute(l,n),Bo(Ro,n,s,r,e,t,this)}}else{let n=Math.max(0,a.start),i=Math.min(l.count,a.start+a.count);for(let a=n,o=i;a<o;a++)Ro.fromBufferAttribute(l,a),Bo(Ro,a,s,r,e,t,this)}}updateMorphTargets(){let e=this.geometry.morphAttributes,t=Object.keys(e);if(t.length>0){let n=e[t[0]];if(n!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let e=0,t=n.length;e<t;e++){let t=n[e].name||String(e);this.morphTargetInfluences.push(0),this.morphTargetDictionary[t]=e}}}}};function Bo(e,t,n,r,i,a,o){let s=Io.distanceSqToPoint(e);if(s<n){let n=new q;Io.closestPointToPoint(e,n),n.applyMatrix4(r);let c=i.ray.origin.distanceTo(n);if(c<i.near||c>i.far)return;a.push({distance:c,distanceToRay:Math.sqrt(s),point:n,index:t,face:null,faceIndex:null,barycoord:null,object:o})}}var Vo=class extends Fn{constructor(e,t,n,r,i=h,a=h,o,s,c){super(e,t,n,r,i,a,o,s,c),this.isVideoTexture=!0,this.generateMipmaps=!1,this._requestVideoFrameCallbackId=0;let l=this;function u(){l.needsUpdate=!0,l._requestVideoFrameCallbackId=e.requestVideoFrameCallback(u)}`requestVideoFrameCallback`in e&&(this._requestVideoFrameCallbackId=e.requestVideoFrameCallback(u))}clone(){return new this.constructor(this.image).copy(this)}update(){let e=this.image;!(`requestVideoFrameCallback`in e)&&e.readyState>=e.HAVE_CURRENT_DATA&&(this.needsUpdate=!0)}dispose(){this._requestVideoFrameCallbackId!==0&&(this.source.data.cancelVideoFrameCallback(this._requestVideoFrameCallbackId),this._requestVideoFrameCallbackId=0),super.dispose()}},Ho=class extends Vo{constructor(e,t,n,r,i,a,o,s){super({},e,t,n,r,i,a,o,s),this.isVideoFrameTexture=!0}update(){}clone(){return new this.constructor().copy(this)}setFrame(e){this.image=e,this.needsUpdate=!0}},Uo=class extends Fn{constructor(e,t){super({width:e,height:t}),this.isFramebufferTexture=!0,this.magFilter=u,this.minFilter=u,this.generateMipmaps=!1,this.needsUpdate=!0}},Wo=class extends Fn{constructor(e,t,n,r,i,a,o,s,c,l,u,d){super(null,a,o,s,c,l,r,i,u,d),this.isCompressedTexture=!0,this.image={width:t,height:n},this.mipmaps=e,this.flipY=!1,this.generateMipmaps=!1}},Go=class extends Wo{constructor(e,t,n,r,i,a){super(e,t,n,i,a),this.isCompressedArrayTexture=!0,this.image.depth=r,this.wrapR=c,this.layerUpdates=new Set}copy(e){return super.copy(e),this.wrapR=e.wrapR,this}addLayerUpdate(e){this.layerUpdates.add(e)}clearLayerUpdates(){this.layerUpdates.clear()}},Ko=class extends Wo{constructor(e,t,n){super(void 0,e[0].width,e[0].height,t,n,301),this.isCompressedCubeTexture=!0,this.isCubeTexture=!0,this.image=e}},qo=class extends Fn{constructor(e=[],t=301,n,r,i,a,o,s,c,l){super(e,t,n,r,i,a,o,s,c,l),this.isCubeTexture=!0,this.flipY=!1}get images(){return this.image}set images(e){this.image=e}},Jo=class extends Fn{constructor(e,t,n,r,i,a,o,s,c){super(e,t,n,r,i,a,o,s,c),this.isCanvasTexture=!0,this.needsUpdate=!0}},Yo=class extends Fn{constructor(e,t,n,r,i,a,o,s,c){super(e,t,n,r,i,a,o,s,c),this.isHTMLTexture=!0,this.generateMipmaps=!1,this.needsUpdate=!0;let l=e?e.parentNode:null;l!==null&&`requestPaint`in l&&(l.onpaint=()=>{this.needsUpdate=!0},l.requestPaint())}dispose(){let e=this.image?this.image.parentNode:null;e!==null&&`onpaint`in e&&(e.onpaint=null),super.dispose()}},Xo=class extends Fn{constructor(e,t,n=T,r,i,a,o=u,s=u,c,l=I,d=1){if(l!==1026&&l!==1027)throw Error(`THREE.DepthTexture: format must be either THREE.DepthFormat or THREE.DepthStencilFormat`);super({width:e,height:t,depth:d},r,i,a,o,s,l,n,c),this.isDepthTexture=!0,this.flipY=!1,this.generateMipmaps=!1,this.compareFunction=null}copy(e){return super.copy(e),this.source=new An(Object.assign({},e.image)),this.compareFunction=e.compareFunction,this}toJSON(e){let t=super.toJSON(e);return t.compareFunction=this.compareFunction,t}},Zo=class extends Xo{constructor(e,t=T,n=301,r,i,a=u,o=u,s,c=I){let l={width:e,height:e,depth:1},d=[l,l,l,l,l,l];super(e,e,t,n,r,i,a,o,s,c),this.image=d,this.isCubeDepthTexture=!0,this.isCubeTexture=!0}get images(){return this.image}set images(e){this.image=e}},Qo=class extends Fn{constructor(e=null){super(),this.sourceTexture=e,this.isExternalTexture=!0}copy(e){return super.copy(e),this.sourceTexture=e.sourceTexture,this}},$o=class e extends Z{constructor(e=1,t=1,n=1,r=1,i=1,a=1){super(),this.type=`BoxGeometry`,this.parameters={width:e,height:t,depth:n,widthSegments:r,heightSegments:i,depthSegments:a};let o=this;r=Math.floor(r),i=Math.floor(i),a=Math.floor(a);let s=[],c=[],l=[],u=[],d=0,f=0;p(`z`,`y`,`x`,-1,-1,n,t,e,a,i,0),p(`z`,`y`,`x`,1,-1,n,t,-e,a,i,1),p(`x`,`z`,`y`,1,1,e,n,t,r,a,2),p(`x`,`z`,`y`,1,-1,e,n,-t,r,a,3),p(`x`,`y`,`z`,1,-1,e,t,n,r,i,4),p(`x`,`y`,`z`,-1,-1,e,t,-n,r,i,5),this.setIndex(s),this.setAttribute(`position`,new X(c,3)),this.setAttribute(`normal`,new X(l,3)),this.setAttribute(`uv`,new X(u,2));function p(e,t,n,r,i,a,p,m,h,g,_){let v=a/h,y=p/g,b=a/2,x=p/2,S=m/2,C=h+1,w=g+1,T=0,E=0,D=new q;for(let a=0;a<w;a++){let o=a*y-x;for(let s=0;s<C;s++)D[e]=(s*v-b)*r,D[t]=o*i,D[n]=S,c.push(D.x,D.y,D.z),D[e]=0,D[t]=0,D[n]=m>0?1:-1,l.push(D.x,D.y,D.z),u.push(s/h),u.push(1-a/g),T+=1}for(let e=0;e<g;e++)for(let t=0;t<h;t++){let n=d+t+C*e,r=d+t+C*(e+1),i=d+(t+1)+C*(e+1),a=d+(t+1)+C*e;s.push(n,r,a),s.push(r,i,a),E+=6}o.addGroup(f,E,_),f+=E,d+=T}}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.width,t.height,t.depth,t.widthSegments,t.heightSegments,t.depthSegments)}},es=class e extends Z{constructor(e=1,t=1,n=4,r=8,i=1){super(),this.type=`CapsuleGeometry`,this.parameters={radius:e,height:t,capSegments:n,radialSegments:r,heightSegments:i},t=Math.max(0,t),n=Math.max(1,Math.floor(n)),r=Math.max(3,Math.floor(r)),i=Math.max(1,Math.floor(i));let a=[],o=[],s=[],c=[],l=t/2,u=Math.PI/2*e,d=t,f=2*u+d,p=n*2+i,m=r+1,h=new q,g=new q;for(let _=0;_<=p;_++){let v=0,y=0,b=0,x=0;if(_<=n){let t=_/n,r=t*Math.PI/2;y=-l-e*Math.cos(r),b=e*Math.sin(r),x=-e*Math.cos(r),v=t*u}else if(_<=n+i){let r=(_-n)/i;y=-l+r*t,b=e,x=0,v=u+r*d}else{let t=(_-n-i)/n,r=t*Math.PI/2;y=l+e*Math.sin(r),b=e*Math.cos(r),x=e*Math.sin(r),v=u+d+t*u}let S=Math.max(0,Math.min(1,v/f)),C=0;_===0?C=.5/r:_===p&&(C=-.5/r);for(let e=0;e<=r;e++){let t=e/r,n=t*Math.PI*2,i=Math.sin(n),a=Math.cos(n);g.x=-b*a,g.y=y,g.z=b*i,o.push(g.x,g.y,g.z),h.set(-b*a,x,b*i),h.normalize(),s.push(h.x,h.y,h.z),c.push(t+C,S)}if(_>0){let e=(_-1)*m;for(let t=0;t<r;t++){let n=e+t,r=e+t+1,i=_*m+t,o=_*m+t+1;a.push(n,r,i),a.push(r,o,i)}}}this.setIndex(a),this.setAttribute(`position`,new X(o,3)),this.setAttribute(`normal`,new X(s,3)),this.setAttribute(`uv`,new X(c,2))}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.radius,t.height,t.capSegments,t.radialSegments,t.heightSegments)}},ts=class e extends Z{constructor(e=1,t=32,n=0,r=Math.PI*2){super(),this.type=`CircleGeometry`,this.parameters={radius:e,segments:t,thetaStart:n,thetaLength:r},t=Math.max(3,t);let i=[],a=[],o=[],s=[],c=new q,l=new K;a.push(0,0,0),o.push(0,0,1),s.push(.5,.5);for(let i=0,u=3;i<=t;i++,u+=3){let d=n+i/t*r;c.x=e*Math.cos(d),c.y=e*Math.sin(d),a.push(c.x,c.y,c.z),o.push(0,0,1),l.x=(a[u]/e+1)/2,l.y=(a[u+1]/e+1)/2,s.push(l.x,l.y)}for(let e=1;e<=t;e++)i.push(e,e+1,0);this.setIndex(i),this.setAttribute(`position`,new X(a,3)),this.setAttribute(`normal`,new X(o,3)),this.setAttribute(`uv`,new X(s,2))}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.radius,t.segments,t.thetaStart,t.thetaLength)}},ns=class e extends Z{constructor(e=1,t=1,n=1,r=32,i=1,a=!1,o=0,s=Math.PI*2){super(),this.type=`CylinderGeometry`,this.parameters={radiusTop:e,radiusBottom:t,height:n,radialSegments:r,heightSegments:i,openEnded:a,thetaStart:o,thetaLength:s};let c=this;r=Math.floor(r),i=Math.floor(i);let l=[],u=[],d=[],f=[],p=0,m=[],h=n/2,g=0;_(),a===!1&&(e>0&&v(!0),t>0&&v(!1)),this.setIndex(l),this.setAttribute(`position`,new X(u,3)),this.setAttribute(`normal`,new X(d,3)),this.setAttribute(`uv`,new X(f,2));function _(){let a=new q,_=new q,v=0,y=(t-e)/n;for(let c=0;c<=i;c++){let l=[],g=c/i,v=g*(t-e)+e;for(let e=0;e<=r;e++){let t=e/r,i=t*s+o,c=Math.sin(i),m=Math.cos(i);_.x=v*c,_.y=-g*n+h,_.z=v*m,u.push(_.x,_.y,_.z),a.set(c,y,m).normalize(),d.push(a.x,a.y,a.z),f.push(t,1-g),l.push(p++)}m.push(l)}for(let n=0;n<r;n++)for(let r=0;r<i;r++){let a=m[r][n],o=m[r+1][n],s=m[r+1][n+1],c=m[r][n+1];(e>0||r!==0)&&(l.push(a,o,c),v+=3),(t>0||r!==i-1)&&(l.push(o,s,c),v+=3)}c.addGroup(g,v,0),g+=v}function v(n){let i=p,a=new K,m=new q,_=0,v=n===!0?e:t,y=n===!0?1:-1;for(let e=1;e<=r;e++)u.push(0,h*y,0),d.push(0,y,0),f.push(.5,.5),p++;let b=p;for(let e=0;e<=r;e++){let t=e/r*s+o,n=Math.cos(t),i=Math.sin(t);m.x=v*i,m.y=h*y,m.z=v*n,u.push(m.x,m.y,m.z),d.push(0,y,0),a.x=n*.5+.5,a.y=i*.5*y+.5,f.push(a.x,a.y),p++}for(let e=0;e<r;e++){let t=i+e,r=b+e;n===!0?l.push(r,r+1,t):l.push(r+1,r,t),_+=3}c.addGroup(g,_,n===!0?1:2),g+=_}}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.radiusTop,t.radiusBottom,t.height,t.radialSegments,t.heightSegments,t.openEnded,t.thetaStart,t.thetaLength)}},rs=class e extends ns{constructor(e=1,t=1,n=32,r=1,i=!1,a=0,o=Math.PI*2){super(0,e,t,n,r,i,a,o),this.type=`ConeGeometry`,this.parameters={radius:e,height:t,radialSegments:n,heightSegments:r,openEnded:i,thetaStart:a,thetaLength:o}}static fromJSON(t){return new e(t.radius,t.height,t.radialSegments,t.heightSegments,t.openEnded,t.thetaStart,t.thetaLength)}},is=class e extends Z{constructor(e=[],t=[],n=1,r=0){super(),this.type=`PolyhedronGeometry`,this.parameters={vertices:e,indices:t,radius:n,detail:r};let i=[],a=[];o(r),c(n),l(),this.setAttribute(`position`,new X(i,3)),this.setAttribute(`normal`,new X(i.slice(),3)),this.setAttribute(`uv`,new X(a,2)),r===0?this.computeVertexNormals():this.normalizeNormals();function o(e){let n=new q,r=new q,i=new q;for(let a=0;a<t.length;a+=3)f(t[a+0],n),f(t[a+1],r),f(t[a+2],i),s(n,r,i,e)}function s(e,t,n,r){let i=r+1,a=[];for(let r=0;r<=i;r++){a[r]=[];let o=e.clone().lerp(n,r/i),s=t.clone().lerp(n,r/i),c=i-r;for(let e=0;e<=c;e++)e===0&&r===i?a[r][e]=o:a[r][e]=o.clone().lerp(s,e/c)}for(let e=0;e<i;e++)for(let t=0;t<2*(i-e)-1;t++){let n=Math.floor(t/2);t%2==0?(d(a[e][n+1]),d(a[e+1][n]),d(a[e][n])):(d(a[e][n+1]),d(a[e+1][n+1]),d(a[e+1][n]))}}function c(e){let t=new q;for(let n=0;n<i.length;n+=3)t.x=i[n+0],t.y=i[n+1],t.z=i[n+2],t.normalize().multiplyScalar(e),i[n+0]=t.x,i[n+1]=t.y,i[n+2]=t.z}function l(){let e=new q;for(let t=0;t<i.length;t+=3){e.x=i[t+0],e.y=i[t+1],e.z=i[t+2];let n=h(e)/2/Math.PI+.5,r=g(e)/Math.PI+.5;a.push(n,1-r)}p(),u()}function u(){for(let e=0;e<a.length;e+=6){let t=a[e+0],n=a[e+2],r=a[e+4];Math.max(t,n,r)>.9&&Math.min(t,n,r)<.1&&(t<.2&&(a[e+0]+=1),n<.2&&(a[e+2]+=1),r<.2&&(a[e+4]+=1))}}function d(e){i.push(e.x,e.y,e.z)}function f(t,n){let r=t*3;n.x=e[r+0],n.y=e[r+1],n.z=e[r+2]}function p(){let e=new q,t=new q,n=new q,r=new q,o=new K,s=new K,c=new K;for(let l=0,u=0;l<i.length;l+=9,u+=6){e.set(i[l+0],i[l+1],i[l+2]),t.set(i[l+3],i[l+4],i[l+5]),n.set(i[l+6],i[l+7],i[l+8]),o.set(a[u+0],a[u+1]),s.set(a[u+2],a[u+3]),c.set(a[u+4],a[u+5]),r.copy(e).add(t).add(n).divideScalar(3);let d=h(r);m(o,u+0,e,d),m(s,u+2,t,d),m(c,u+4,n,d)}}function m(e,t,n,r){r<0&&e.x===1&&(a[t]=e.x-1),n.x===0&&n.z===0&&(a[t]=r/2/Math.PI+.5)}function h(e){return Math.atan2(e.z,-e.x)}function g(e){return Math.atan2(-e.y,Math.sqrt(e.x*e.x+e.z*e.z))}}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.vertices,t.indices,t.radius,t.detail)}},as=class e extends is{constructor(e=1,t=0){let n=(1+Math.sqrt(5))/2,r=1/n,i=[-1,-1,-1,-1,-1,1,-1,1,-1,-1,1,1,1,-1,-1,1,-1,1,1,1,-1,1,1,1,0,-r,-n,0,-r,n,0,r,-n,0,r,n,-r,-n,0,-r,n,0,r,-n,0,r,n,0,-n,0,-r,n,0,-r,-n,0,r,n,0,r];super(i,[3,11,7,3,7,15,3,15,13,7,19,17,7,17,6,7,6,15,17,4,8,17,8,10,17,10,6,8,0,16,8,16,2,8,2,10,0,12,1,0,1,18,0,18,16,6,10,2,6,2,13,6,13,15,2,16,18,2,18,3,2,3,13,18,1,9,18,9,11,18,11,3,4,14,12,4,12,0,4,0,8,11,9,5,11,5,19,11,19,7,19,5,14,19,14,4,19,4,17,1,12,14,1,14,5,1,5,9],e,t),this.type=`DodecahedronGeometry`,this.parameters={radius:e,detail:t}}static fromJSON(t){return new e(t.radius,t.detail)}},os=new q,ss=new q,cs=new q,ls=new Br,us=class extends Z{constructor(e=null,t=1){if(super(),this.type=`EdgesGeometry`,this.parameters={geometry:e,thresholdAngle:t},e!==null){let n=1e4,r=Math.cos(Gt*t),i=e.getIndex(),a=e.getAttribute(`position`),o=i?i.count:a.count,s=[0,0,0],c=[`a`,`b`,`c`],l=[,,,],u={},d=[];for(let e=0;e<o;e+=3){i?(s[0]=i.getX(e),s[1]=i.getX(e+1),s[2]=i.getX(e+2)):(s[0]=e,s[1]=e+1,s[2]=e+2);let{a:t,b:o,c:f}=ls;if(t.fromBufferAttribute(a,s[0]),o.fromBufferAttribute(a,s[1]),f.fromBufferAttribute(a,s[2]),ls.getNormal(cs),l[0]=`${Math.round(t.x*n)},${Math.round(t.y*n)},${Math.round(t.z*n)}`,l[1]=`${Math.round(o.x*n)},${Math.round(o.y*n)},${Math.round(o.z*n)}`,l[2]=`${Math.round(f.x*n)},${Math.round(f.y*n)},${Math.round(f.z*n)}`,l[0]!==l[1]&&l[1]!==l[2]&&l[2]!==l[0])for(let e=0;e<3;e++){let t=(e+1)%3,n=l[e],i=l[t],a=ls[c[e]],o=ls[c[t]],f=`${n}_${i}`,p=`${i}_${n}`;p in u&&u[p]?(cs.dot(u[p].normal)<=r&&(d.push(a.x,a.y,a.z),d.push(o.x,o.y,o.z)),u[p]=null):f in u||(u[f]={index0:s[e],index1:s[t],normal:cs.clone()})}}for(let e in u)if(u[e]){let{index0:t,index1:n}=u[e];os.fromBufferAttribute(a,t),ss.fromBufferAttribute(a,n),d.push(os.x,os.y,os.z),d.push(ss.x,ss.y,ss.z)}this.setAttribute(`position`,new X(d,3))}}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}},ds=class{constructor(){this.type=`Curve`,this.arcLengthDivisions=200,this.needsUpdate=!1,this.cacheArcLengths=null}getPoint(){U(`Curve: .getPoint() not implemented.`)}getPointAt(e,t){let n=this.getUtoTmapping(e);return this.getPoint(n,t)}getPoints(e=5){let t=[];for(let n=0;n<=e;n++)t.push(this.getPoint(n/e));return t}getSpacedPoints(e=5){let t=[];for(let n=0;n<=e;n++)t.push(this.getPointAt(n/e));return t}getLength(){let e=this.getLengths();return e[e.length-1]}getLengths(e=this.arcLengthDivisions){if(this.cacheArcLengths&&this.cacheArcLengths.length===e+1&&!this.needsUpdate)return this.cacheArcLengths;this.needsUpdate=!1;let t=[],n,r=this.getPoint(0),i=0;t.push(0);for(let a=1;a<=e;a++)n=this.getPoint(a/e),i+=n.distanceTo(r),t.push(i),r=n;return this.cacheArcLengths=t,t}updateArcLengths(){this.needsUpdate=!0,this.getLengths()}getUtoTmapping(e,t=null){let n=this.getLengths(),r=0,i=n.length,a;a=t||e*n[i-1];let o=0,s=i-1,c;for(;o<=s;)if(r=Math.floor(o+(s-o)/2),c=n[r]-a,c<0)o=r+1;else if(c>0)s=r-1;else{s=r;break}if(r=s,n[r]===a)return r/(i-1);let l=n[r],u=n[r+1]-l,d=(a-l)/u;return(r+d)/(i-1)}getTangent(e,t){let n=1e-4,r=e-n,i=e+n;r<0&&(r=0),i>1&&(i=1);let a=this.getPoint(r),o=this.getPoint(i),s=t||(a.isVector2?new K:new q);return s.copy(o).sub(a).normalize(),s}getTangentAt(e,t){let n=this.getUtoTmapping(e);return this.getTangent(n,t)}computeFrenetFrames(e,t=!1){let n=new q,r=[],i=[],a=[],o=new q,s=new J;for(let t=0;t<=e;t++){let n=t/e;r[t]=this.getTangentAt(n,new q)}i[0]=new q,a[0]=new q;let c=Number.MAX_VALUE,l=Math.abs(r[0].x),u=Math.abs(r[0].y),d=Math.abs(r[0].z);l<=c&&(c=l,n.set(1,0,0)),u<=c&&(c=u,n.set(0,1,0)),d<=c&&n.set(0,0,1),o.crossVectors(r[0],n).normalize(),i[0].crossVectors(r[0],o),a[0].crossVectors(r[0],i[0]);for(let t=1;t<=e;t++){if(i[t]=i[t-1].clone(),a[t]=a[t-1].clone(),o.crossVectors(r[t-1],r[t]),o.length()>2**-52){o.normalize();let e=Math.acos(G(r[t-1].dot(r[t]),-1,1));i[t].applyMatrix4(s.makeRotationAxis(o,e))}a[t].crossVectors(r[t],i[t])}if(t===!0){let t=Math.acos(G(i[0].dot(i[e]),-1,1));t/=e,r[0].dot(o.crossVectors(i[0],i[e]))>0&&(t=-t);for(let n=1;n<=e;n++)i[n].applyMatrix4(s.makeRotationAxis(r[n],t*n)),a[n].crossVectors(r[n],i[n])}return{tangents:r,normals:i,binormals:a}}clone(){return new this.constructor().copy(this)}copy(e){return this.arcLengthDivisions=e.arcLengthDivisions,this}toJSON(){let e={metadata:{version:4.7,type:`Curve`,generator:`Curve.toJSON`}};return e.arcLengthDivisions=this.arcLengthDivisions,e.type=this.type,e}fromJSON(e){return this.arcLengthDivisions=e.arcLengthDivisions,this}},fs=class extends ds{constructor(e=0,t=0,n=1,r=1,i=0,a=Math.PI*2,o=!1,s=0){super(),this.isEllipseCurve=!0,this.type=`EllipseCurve`,this.aX=e,this.aY=t,this.xRadius=n,this.yRadius=r,this.aStartAngle=i,this.aEndAngle=a,this.aClockwise=o,this.aRotation=s}getPoint(e,t=new K){let n=t,r=Math.PI*2,i=this.aEndAngle-this.aStartAngle,a=Math.abs(i)<2**-52;for(;i<0;)i+=r;for(;i>r;)i-=r;i<2**-52&&(i=a?0:r),this.aClockwise===!0&&!a&&(i===r?i=-r:i-=r);let o=this.aStartAngle+e*i,s=this.aX+this.xRadius*Math.cos(o),c=this.aY+this.yRadius*Math.sin(o);if(this.aRotation!==0){let e=Math.cos(this.aRotation),t=Math.sin(this.aRotation),n=s-this.aX,r=c-this.aY;s=n*e-r*t+this.aX,c=n*t+r*e+this.aY}return n.set(s,c)}copy(e){return super.copy(e),this.aX=e.aX,this.aY=e.aY,this.xRadius=e.xRadius,this.yRadius=e.yRadius,this.aStartAngle=e.aStartAngle,this.aEndAngle=e.aEndAngle,this.aClockwise=e.aClockwise,this.aRotation=e.aRotation,this}toJSON(){let e=super.toJSON();return e.aX=this.aX,e.aY=this.aY,e.xRadius=this.xRadius,e.yRadius=this.yRadius,e.aStartAngle=this.aStartAngle,e.aEndAngle=this.aEndAngle,e.aClockwise=this.aClockwise,e.aRotation=this.aRotation,e}fromJSON(e){return super.fromJSON(e),this.aX=e.aX,this.aY=e.aY,this.xRadius=e.xRadius,this.yRadius=e.yRadius,this.aStartAngle=e.aStartAngle,this.aEndAngle=e.aEndAngle,this.aClockwise=e.aClockwise,this.aRotation=e.aRotation,this}},ps=class extends fs{constructor(e,t,n,r,i,a){super(e,t,n,n,r,i,a),this.isArcCurve=!0,this.type=`ArcCurve`}};function ms(){let e=0,t=0,n=0,r=0;function i(i,a,o,s){e=i,t=o,n=-3*i+3*a-2*o-s,r=2*i-2*a+o+s}return{initCatmullRom:function(e,t,n,r,a){i(t,n,a*(n-e),a*(r-t))},initNonuniformCatmullRom:function(e,t,n,r,a,o,s){let c=(t-e)/a-(n-e)/(a+o)+(n-t)/o,l=(n-t)/o-(r-t)/(o+s)+(r-n)/s;c*=o,l*=o,i(t,n,c,l)},calc:function(i){let a=i*i,o=a*i;return e+t*i+n*a+r*o}}}var hs=new q,gs=new q,_s=new ms,vs=new ms,ys=new ms,bs=class extends ds{constructor(e=[],t=!1,n=`centripetal`,r=.5){super(),this.isCatmullRomCurve3=!0,this.type=`CatmullRomCurve3`,this.points=e,this.closed=t,this.curveType=n,this.tension=r}getPoint(e,t=new q){let n=t,r=this.points,i=r.length,a=(i-+!this.closed)*e,o=Math.floor(a),s=a-o;this.closed?o+=o>0?0:(Math.floor(Math.abs(o)/i)+1)*i:s===0&&o===i-1&&(o=i-2,s=1);let c,l;this.closed||o>0?c=r[(o-1)%i]:(gs.subVectors(r[0],r[1]).add(r[0]),c=gs);let u=r[o%i],d=r[(o+1)%i];if(this.closed||o+2<i?l=r[(o+2)%i]:(hs.subVectors(r[i-1],r[i-2]).add(r[i-1]),l=hs),this.curveType===`centripetal`||this.curveType===`chordal`){let e=this.curveType===`chordal`?.5:.25,t=c.distanceToSquared(u)**+e,n=u.distanceToSquared(d)**+e,r=d.distanceToSquared(l)**+e;n<1e-4&&(n=1),t<1e-4&&(t=n),r<1e-4&&(r=n),_s.initNonuniformCatmullRom(c.x,u.x,d.x,l.x,t,n,r),vs.initNonuniformCatmullRom(c.y,u.y,d.y,l.y,t,n,r),ys.initNonuniformCatmullRom(c.z,u.z,d.z,l.z,t,n,r)}else this.curveType===`catmullrom`&&(_s.initCatmullRom(c.x,u.x,d.x,l.x,this.tension),vs.initCatmullRom(c.y,u.y,d.y,l.y,this.tension),ys.initCatmullRom(c.z,u.z,d.z,l.z,this.tension));return n.set(_s.calc(s),vs.calc(s),ys.calc(s)),n}copy(e){super.copy(e),this.points=[];for(let t=0,n=e.points.length;t<n;t++){let n=e.points[t];this.points.push(n.clone())}return this.closed=e.closed,this.curveType=e.curveType,this.tension=e.tension,this}toJSON(){let e=super.toJSON();e.points=[];for(let t=0,n=this.points.length;t<n;t++){let n=this.points[t];e.points.push(n.toArray())}return e.closed=this.closed,e.curveType=this.curveType,e.tension=this.tension,e}fromJSON(e){super.fromJSON(e),this.points=[];for(let t=0,n=e.points.length;t<n;t++){let n=e.points[t];this.points.push(new q().fromArray(n))}return this.closed=e.closed,this.curveType=e.curveType,this.tension=e.tension,this}};function xs(e,t,n,r,i){let a=(r-t)*.5,o=(i-n)*.5,s=e*e,c=e*s;return(2*n-2*r+a+o)*c+(-3*n+3*r-2*a-o)*s+a*e+n}function Ss(e,t){let n=1-e;return n*n*t}function Cs(e,t){return 2*(1-e)*e*t}function ws(e,t){return e*e*t}function Ts(e,t,n,r){return Ss(e,t)+Cs(e,n)+ws(e,r)}function Es(e,t){let n=1-e;return n*n*n*t}function Ds(e,t){let n=1-e;return 3*n*n*e*t}function Os(e,t){return 3*(1-e)*e*e*t}function ks(e,t){return e*e*e*t}function As(e,t,n,r,i){return Es(e,t)+Ds(e,n)+Os(e,r)+ks(e,i)}var js=class extends ds{constructor(e=new K,t=new K,n=new K,r=new K){super(),this.isCubicBezierCurve=!0,this.type=`CubicBezierCurve`,this.v0=e,this.v1=t,this.v2=n,this.v3=r}getPoint(e,t=new K){let n=t,r=this.v0,i=this.v1,a=this.v2,o=this.v3;return n.set(As(e,r.x,i.x,a.x,o.x),As(e,r.y,i.y,a.y,o.y)),n}copy(e){return super.copy(e),this.v0.copy(e.v0),this.v1.copy(e.v1),this.v2.copy(e.v2),this.v3.copy(e.v3),this}toJSON(){let e=super.toJSON();return e.v0=this.v0.toArray(),e.v1=this.v1.toArray(),e.v2=this.v2.toArray(),e.v3=this.v3.toArray(),e}fromJSON(e){return super.fromJSON(e),this.v0.fromArray(e.v0),this.v1.fromArray(e.v1),this.v2.fromArray(e.v2),this.v3.fromArray(e.v3),this}},Ms=class extends ds{constructor(e=new q,t=new q,n=new q,r=new q){super(),this.isCubicBezierCurve3=!0,this.type=`CubicBezierCurve3`,this.v0=e,this.v1=t,this.v2=n,this.v3=r}getPoint(e,t=new q){let n=t,r=this.v0,i=this.v1,a=this.v2,o=this.v3;return n.set(As(e,r.x,i.x,a.x,o.x),As(e,r.y,i.y,a.y,o.y),As(e,r.z,i.z,a.z,o.z)),n}copy(e){return super.copy(e),this.v0.copy(e.v0),this.v1.copy(e.v1),this.v2.copy(e.v2),this.v3.copy(e.v3),this}toJSON(){let e=super.toJSON();return e.v0=this.v0.toArray(),e.v1=this.v1.toArray(),e.v2=this.v2.toArray(),e.v3=this.v3.toArray(),e}fromJSON(e){return super.fromJSON(e),this.v0.fromArray(e.v0),this.v1.fromArray(e.v1),this.v2.fromArray(e.v2),this.v3.fromArray(e.v3),this}},Ns=class extends ds{constructor(e=new K,t=new K){super(),this.isLineCurve=!0,this.type=`LineCurve`,this.v1=e,this.v2=t}getPoint(e,t=new K){let n=t;return e===1?n.copy(this.v2):(n.copy(this.v2).sub(this.v1),n.multiplyScalar(e).add(this.v1)),n}getPointAt(e,t){return this.getPoint(e,t)}getTangent(e,t=new K){return t.subVectors(this.v2,this.v1).normalize()}getTangentAt(e,t){return this.getTangent(e,t)}copy(e){return super.copy(e),this.v1.copy(e.v1),this.v2.copy(e.v2),this}toJSON(){let e=super.toJSON();return e.v1=this.v1.toArray(),e.v2=this.v2.toArray(),e}fromJSON(e){return super.fromJSON(e),this.v1.fromArray(e.v1),this.v2.fromArray(e.v2),this}},Ps=class extends ds{constructor(e=new q,t=new q){super(),this.isLineCurve3=!0,this.type=`LineCurve3`,this.v1=e,this.v2=t}getPoint(e,t=new q){let n=t;return e===1?n.copy(this.v2):(n.copy(this.v2).sub(this.v1),n.multiplyScalar(e).add(this.v1)),n}getPointAt(e,t){return this.getPoint(e,t)}getTangent(e,t=new q){return t.subVectors(this.v2,this.v1).normalize()}getTangentAt(e,t){return this.getTangent(e,t)}copy(e){return super.copy(e),this.v1.copy(e.v1),this.v2.copy(e.v2),this}toJSON(){let e=super.toJSON();return e.v1=this.v1.toArray(),e.v2=this.v2.toArray(),e}fromJSON(e){return super.fromJSON(e),this.v1.fromArray(e.v1),this.v2.fromArray(e.v2),this}},Fs=class extends ds{constructor(e=new K,t=new K,n=new K){super(),this.isQuadraticBezierCurve=!0,this.type=`QuadraticBezierCurve`,this.v0=e,this.v1=t,this.v2=n}getPoint(e,t=new K){let n=t,r=this.v0,i=this.v1,a=this.v2;return n.set(Ts(e,r.x,i.x,a.x),Ts(e,r.y,i.y,a.y)),n}copy(e){return super.copy(e),this.v0.copy(e.v0),this.v1.copy(e.v1),this.v2.copy(e.v2),this}toJSON(){let e=super.toJSON();return e.v0=this.v0.toArray(),e.v1=this.v1.toArray(),e.v2=this.v2.toArray(),e}fromJSON(e){return super.fromJSON(e),this.v0.fromArray(e.v0),this.v1.fromArray(e.v1),this.v2.fromArray(e.v2),this}},Is=class extends ds{constructor(e=new q,t=new q,n=new q){super(),this.isQuadraticBezierCurve3=!0,this.type=`QuadraticBezierCurve3`,this.v0=e,this.v1=t,this.v2=n}getPoint(e,t=new q){let n=t,r=this.v0,i=this.v1,a=this.v2;return n.set(Ts(e,r.x,i.x,a.x),Ts(e,r.y,i.y,a.y),Ts(e,r.z,i.z,a.z)),n}copy(e){return super.copy(e),this.v0.copy(e.v0),this.v1.copy(e.v1),this.v2.copy(e.v2),this}toJSON(){let e=super.toJSON();return e.v0=this.v0.toArray(),e.v1=this.v1.toArray(),e.v2=this.v2.toArray(),e}fromJSON(e){return super.fromJSON(e),this.v0.fromArray(e.v0),this.v1.fromArray(e.v1),this.v2.fromArray(e.v2),this}},Ls=class extends ds{constructor(e=[]){super(),this.isSplineCurve=!0,this.type=`SplineCurve`,this.points=e}getPoint(e,t=new K){let n=t,r=this.points,i=(r.length-1)*e,a=Math.floor(i),o=i-a,s=r[a===0?a:a-1],c=r[a],l=r[a>r.length-2?r.length-1:a+1],u=r[a>r.length-3?r.length-1:a+2];return n.set(xs(o,s.x,c.x,l.x,u.x),xs(o,s.y,c.y,l.y,u.y)),n}copy(e){super.copy(e),this.points=[];for(let t=0,n=e.points.length;t<n;t++){let n=e.points[t];this.points.push(n.clone())}return this}toJSON(){let e=super.toJSON();e.points=[];for(let t=0,n=this.points.length;t<n;t++){let n=this.points[t];e.points.push(n.toArray())}return e}fromJSON(e){super.fromJSON(e),this.points=[];for(let t=0,n=e.points.length;t<n;t++){let n=e.points[t];this.points.push(new K().fromArray(n))}return this}},Rs=Object.freeze({__proto__:null,ArcCurve:ps,CatmullRomCurve3:bs,CubicBezierCurve:js,CubicBezierCurve3:Ms,EllipseCurve:fs,LineCurve:Ns,LineCurve3:Ps,QuadraticBezierCurve:Fs,QuadraticBezierCurve3:Is,SplineCurve:Ls}),zs=class extends ds{constructor(){super(),this.type=`CurvePath`,this.curves=[],this.autoClose=!1}add(e){this.curves.push(e)}closePath(){let e=this.curves[0].getPoint(0),t=this.curves[this.curves.length-1].getPoint(1);if(!e.equals(t)){let n=e.isVector2===!0?`LineCurve`:`LineCurve3`;this.curves.push(new Rs[n](t,e))}return this}getPoint(e,t){let n=e*this.getLength(),r=this.getCurveLengths(),i=0;for(;i<r.length;){if(r[i]>=n){let e=r[i]-n,a=this.curves[i],o=a.getLength(),s=o===0?0:1-e/o;return a.getPointAt(s,t)}i++}return null}getLength(){let e=this.getCurveLengths();return e[e.length-1]}updateArcLengths(){this.needsUpdate=!0,this.cacheLengths=null,this.getCurveLengths()}getCurveLengths(){if(this.cacheLengths&&this.cacheLengths.length===this.curves.length)return this.cacheLengths;let e=[],t=0;for(let n=0,r=this.curves.length;n<r;n++)t+=this.curves[n].getLength(),e.push(t);return this.cacheLengths=e,e}getSpacedPoints(e=40){let t=[];for(let n=0;n<=e;n++)t.push(this.getPoint(n/e));return this.autoClose&&t.push(t[0]),t}getPoints(e=12){let t=[],n;for(let r=0,i=this.curves;r<i.length;r++){let a=i[r],o=a.isEllipseCurve?e*2:a.isLineCurve||a.isLineCurve3?1:a.isSplineCurve?e*a.points.length:e,s=a.getPoints(o);for(let e=0;e<s.length;e++){let r=s[e];n&&n.equals(r)||(t.push(r),n=r)}}return this.autoClose&&t.length>1&&!t[t.length-1].equals(t[0])&&t.push(t[0]),t}copy(e){super.copy(e),this.curves=[];for(let t=0,n=e.curves.length;t<n;t++){let n=e.curves[t];this.curves.push(n.clone())}return this.autoClose=e.autoClose,this}toJSON(){let e=super.toJSON();e.autoClose=this.autoClose,e.curves=[];for(let t=0,n=this.curves.length;t<n;t++){let n=this.curves[t];e.curves.push(n.toJSON())}return e}fromJSON(e){super.fromJSON(e),this.autoClose=e.autoClose,this.curves=[];for(let t=0,n=e.curves.length;t<n;t++){let n=e.curves[t];this.curves.push(new Rs[n.type]().fromJSON(n))}return this}},Bs=class extends zs{constructor(e){super(),this.type=`Path`,this.currentPoint=new K,e&&this.setFromPoints(e)}setFromPoints(e){this.moveTo(e[0].x,e[0].y);for(let t=1,n=e.length;t<n;t++)this.lineTo(e[t].x,e[t].y);return this}moveTo(e,t){return this.currentPoint.set(e,t),this}lineTo(e,t){let n=new Ns(this.currentPoint.clone(),new K(e,t));return this.curves.push(n),this.currentPoint.set(e,t),this}quadraticCurveTo(e,t,n,r){let i=new Fs(this.currentPoint.clone(),new K(e,t),new K(n,r));return this.curves.push(i),this.currentPoint.set(n,r),this}bezierCurveTo(e,t,n,r,i,a){let o=new js(this.currentPoint.clone(),new K(e,t),new K(n,r),new K(i,a));return this.curves.push(o),this.currentPoint.set(i,a),this}splineThru(e){let t=new Ls([this.currentPoint.clone()].concat(e));return this.curves.push(t),this.currentPoint.copy(e[e.length-1]),this}arc(e,t,n,r,i,a){let o=this.currentPoint.x,s=this.currentPoint.y;return this.absarc(e+o,t+s,n,r,i,a),this}absarc(e,t,n,r,i,a){return this.absellipse(e,t,n,n,r,i,a),this}ellipse(e,t,n,r,i,a,o,s){let c=this.currentPoint.x,l=this.currentPoint.y;return this.absellipse(e+c,t+l,n,r,i,a,o,s),this}absellipse(e,t,n,r,i,a,o,s){let c=new fs(e,t,n,r,i,a,o,s);if(this.curves.length>0){let e=c.getPoint(0);e.equals(this.currentPoint)||this.lineTo(e.x,e.y)}this.curves.push(c);let l=c.getPoint(1);return this.currentPoint.copy(l),this}copy(e){return super.copy(e),this.currentPoint.copy(e.currentPoint),this}toJSON(){let e=super.toJSON();return e.currentPoint=this.currentPoint.toArray(),e}fromJSON(e){return super.fromJSON(e),this.currentPoint.fromArray(e.currentPoint),this}},Vs=class extends Bs{constructor(e){super(e),this.uuid=qt(),this.type=`Shape`,this.holes=[]}getPointsHoles(e){let t=[];for(let n=0,r=this.holes.length;n<r;n++)t[n]=this.holes[n].getPoints(e);return t}extractPoints(e){return{shape:this.getPoints(e),holes:this.getPointsHoles(e)}}copy(e){super.copy(e),this.holes=[];for(let t=0,n=e.holes.length;t<n;t++){let n=e.holes[t];this.holes.push(n.clone())}return this}toJSON(){let e=super.toJSON();e.uuid=this.uuid,e.holes=[];for(let t=0,n=this.holes.length;t<n;t++){let n=this.holes[t];e.holes.push(n.toJSON())}return e}fromJSON(e){super.fromJSON(e),this.uuid=e.uuid,this.holes=[];for(let t=0,n=e.holes.length;t<n;t++){let n=e.holes[t];this.holes.push(new Bs().fromJSON(n))}return this}};function Hs(e,t,n=2){let r=t&&t.length,i=r?t[0]*n:e.length,a=Us(e,0,i,n,!0),o=[];if(!a||a.next===a.prev)return o;let s,c,l;if(r&&(a=Xs(e,t,a,n)),e.length>80*n){s=e[0],c=e[1];let t=s,r=c;for(let a=n;a<i;a+=n){let n=e[a],i=e[a+1];n<s&&(s=n),i<c&&(c=i),n>t&&(t=n),i>r&&(r=i)}l=Math.max(t-s,r-c),l=l===0?0:32767/l}return Gs(a,o,n,s,c,l,0),o}function Us(e,t,n,r,i){let a;if(i===bc(e,t,n,r)>0)for(let i=t;i<n;i+=r)a=_c(i/r|0,e[i],e[i+1],a);else for(let i=n-r;i>=t;i-=r)a=_c(i/r|0,e[i],e[i+1],a);return a&&lc(a,a.next)&&(vc(a),a=a.next),a}function Ws(e,t){if(!e)return e;t||=e;let n=e,r;do if(r=!1,!n.steiner&&(lc(n,n.next)||cc(n.prev,n,n.next)===0)){if(vc(n),n=t=n.prev,n===n.next)break;r=!0}else n=n.next;while(r||n!==t);return t}function Gs(e,t,n,r,i,a,o){if(!e)return;!o&&a&&tc(e,r,i,a);let s=e;for(;e.prev!==e.next;){let c=e.prev,l=e.next;if(a?qs(e,r,i,a):Ks(e))t.push(c.i,e.i,l.i),vc(e),e=l.next,s=l.next;else if(e=l,e===s){o?o===1?(e=Js(Ws(e),t),Gs(e,t,n,r,i,a,2)):o===2&&Ys(e,t,n,r,i,a):Gs(Ws(e),t,n,r,i,a,1);break}}}function Ks(e){let t=e.prev,n=e,r=e.next;if(cc(t,n,r)>=0)return!1;let i=t.x,a=n.x,o=r.x,s=t.y,c=n.y,l=r.y,u=Math.min(i,a,o),d=Math.min(s,c,l),f=Math.max(i,a,o),p=Math.max(s,c,l),m=r.next;for(;m!==t;){if(m.x>=u&&m.x<=f&&m.y>=d&&m.y<=p&&oc(i,s,a,c,o,l,m.x,m.y)&&cc(m.prev,m,m.next)>=0)return!1;m=m.next}return!0}function qs(e,t,n,r){let i=e.prev,a=e,o=e.next;if(cc(i,a,o)>=0)return!1;let s=i.x,c=a.x,l=o.x,u=i.y,d=a.y,f=o.y,p=Math.min(s,c,l),m=Math.min(u,d,f),h=Math.max(s,c,l),g=Math.max(u,d,f),_=rc(p,m,t,n,r),v=rc(h,g,t,n,r),y=e.prevZ,b=e.nextZ;for(;y&&y.z>=_&&b&&b.z<=v;){if(y.x>=p&&y.x<=h&&y.y>=m&&y.y<=g&&y!==i&&y!==o&&oc(s,u,c,d,l,f,y.x,y.y)&&cc(y.prev,y,y.next)>=0||(y=y.prevZ,b.x>=p&&b.x<=h&&b.y>=m&&b.y<=g&&b!==i&&b!==o&&oc(s,u,c,d,l,f,b.x,b.y)&&cc(b.prev,b,b.next)>=0))return!1;b=b.nextZ}for(;y&&y.z>=_;){if(y.x>=p&&y.x<=h&&y.y>=m&&y.y<=g&&y!==i&&y!==o&&oc(s,u,c,d,l,f,y.x,y.y)&&cc(y.prev,y,y.next)>=0)return!1;y=y.prevZ}for(;b&&b.z<=v;){if(b.x>=p&&b.x<=h&&b.y>=m&&b.y<=g&&b!==i&&b!==o&&oc(s,u,c,d,l,f,b.x,b.y)&&cc(b.prev,b,b.next)>=0)return!1;b=b.nextZ}return!0}function Js(e,t){let n=e;do{let r=n.prev,i=n.next.next;!lc(r,i)&&uc(r,n,n.next,i)&&mc(r,i)&&mc(i,r)&&(t.push(r.i,n.i,i.i),vc(n),vc(n.next),n=e=i),n=n.next}while(n!==e);return Ws(n)}function Ys(e,t,n,r,i,a){let o=e;do{let e=o.next.next;for(;e!==o.prev;){if(o.i!==e.i&&sc(o,e)){let s=gc(o,e);o=Ws(o,o.next),s=Ws(s,s.next),Gs(o,t,n,r,i,a,0),Gs(s,t,n,r,i,a,0);return}e=e.next}o=o.next}while(o!==e)}function Xs(e,t,n,r){let i=[];for(let n=0,a=t.length;n<a;n++){let o=Us(e,t[n]*r,n<a-1?t[n+1]*r:e.length,r,!1);o===o.next&&(o.steiner=!0),i.push(ic(o))}i.sort(Zs);for(let e=0;e<i.length;e++)n=Qs(i[e],n);return n}function Zs(e,t){let n=e.x-t.x;return n===0&&(n=e.y-t.y,n===0&&(n=(e.next.y-e.y)/(e.next.x-e.x)-(t.next.y-t.y)/(t.next.x-t.x))),n}function Qs(e,t){let n=$s(e,t);if(!n)return t;let r=gc(n,e);return Ws(r,r.next),Ws(n,n.next)}function $s(e,t){let n=t,r=e.x,i=e.y,a=-1/0,o;if(lc(e,n))return n;do{if(lc(e,n.next))return n.next;if(i<=n.y&&i>=n.next.y&&n.next.y!==n.y){let e=n.x+(i-n.y)*(n.next.x-n.x)/(n.next.y-n.y);if(e<=r&&e>a&&(a=e,o=n.x<n.next.x?n:n.next,e===r))return o}n=n.next}while(n!==t);if(!o)return null;let s=o,c=o.x,l=o.y,u=1/0;n=o;do{if(r>=n.x&&n.x>=c&&r!==n.x&&ac(i<l?r:a,i,c,l,i<l?a:r,i,n.x,n.y)){let t=Math.abs(i-n.y)/(r-n.x);mc(n,e)&&(t<u||t===u&&(n.x>o.x||n.x===o.x&&ec(o,n)))&&(o=n,u=t)}n=n.next}while(n!==s);return o}function ec(e,t){return cc(e.prev,e,t.prev)<0&&cc(t.next,e,e.next)<0}function tc(e,t,n,r){let i=e;do i.z===0&&(i.z=rc(i.x,i.y,t,n,r)),i.prevZ=i.prev,i.nextZ=i.next,i=i.next;while(i!==e);i.prevZ.nextZ=null,i.prevZ=null,nc(i)}function nc(e){let t,n=1;do{let r=e,i;e=null;let a=null;for(t=0;r;){t++;let o=r,s=0;for(let e=0;e<n&&(s++,o=o.nextZ,o);e++);let c=n;for(;s>0||c>0&&o;)s!==0&&(c===0||!o||r.z<=o.z)?(i=r,r=r.nextZ,s--):(i=o,o=o.nextZ,c--),a?a.nextZ=i:e=i,i.prevZ=a,a=i;r=o}a.nextZ=null,n*=2}while(t>1);return e}function rc(e,t,n,r,i){return e=(e-n)*i|0,t=(t-r)*i|0,e=(e|e<<8)&16711935,e=(e|e<<4)&252645135,e=(e|e<<2)&858993459,e=(e|e<<1)&1431655765,t=(t|t<<8)&16711935,t=(t|t<<4)&252645135,t=(t|t<<2)&858993459,t=(t|t<<1)&1431655765,e|t<<1}function ic(e){let t=e,n=e;do(t.x<n.x||t.x===n.x&&t.y<n.y)&&(n=t),t=t.next;while(t!==e);return n}function ac(e,t,n,r,i,a,o,s){return(i-o)*(t-s)>=(e-o)*(a-s)&&(e-o)*(r-s)>=(n-o)*(t-s)&&(n-o)*(a-s)>=(i-o)*(r-s)}function oc(e,t,n,r,i,a,o,s){return(e!==o||t!==s)&&ac(e,t,n,r,i,a,o,s)}function sc(e,t){return e.next.i!==t.i&&e.prev.i!==t.i&&!pc(e,t)&&(mc(e,t)&&mc(t,e)&&hc(e,t)&&(cc(e.prev,e,t.prev)||cc(e,t.prev,t))||lc(e,t)&&cc(e.prev,e,e.next)>0&&cc(t.prev,t,t.next)>0)}function cc(e,t,n){return(t.y-e.y)*(n.x-t.x)-(t.x-e.x)*(n.y-t.y)}function lc(e,t){return e.x===t.x&&e.y===t.y}function uc(e,t,n,r){let i=fc(cc(e,t,n)),a=fc(cc(e,t,r)),o=fc(cc(n,r,e)),s=fc(cc(n,r,t));return!!(i!==a&&o!==s||i===0&&dc(e,n,t)||a===0&&dc(e,r,t)||o===0&&dc(n,e,r)||s===0&&dc(n,t,r))}function dc(e,t,n){return t.x<=Math.max(e.x,n.x)&&t.x>=Math.min(e.x,n.x)&&t.y<=Math.max(e.y,n.y)&&t.y>=Math.min(e.y,n.y)}function fc(e){return e>0?1:e<0?-1:0}function pc(e,t){let n=e;do{if(n.i!==e.i&&n.next.i!==e.i&&n.i!==t.i&&n.next.i!==t.i&&uc(n,n.next,e,t))return!0;n=n.next}while(n!==e);return!1}function mc(e,t){return cc(e.prev,e,e.next)<0?cc(e,t,e.next)>=0&&cc(e,e.prev,t)>=0:cc(e,t,e.prev)<0||cc(e,e.next,t)<0}function hc(e,t){let n=e,r=!1,i=(e.x+t.x)/2,a=(e.y+t.y)/2;do n.y>a!=n.next.y>a&&n.next.y!==n.y&&i<(n.next.x-n.x)*(a-n.y)/(n.next.y-n.y)+n.x&&(r=!r),n=n.next;while(n!==e);return r}function gc(e,t){let n=yc(e.i,e.x,e.y),r=yc(t.i,t.x,t.y),i=e.next,a=t.prev;return e.next=t,t.prev=e,n.next=i,i.prev=n,r.next=n,n.prev=r,a.next=r,r.prev=a,r}function _c(e,t,n,r){let i=yc(e,t,n);return r?(i.next=r.next,i.prev=r,r.next.prev=i,r.next=i):(i.prev=i,i.next=i),i}function vc(e){e.next.prev=e.prev,e.prev.next=e.next,e.prevZ&&(e.prevZ.nextZ=e.nextZ),e.nextZ&&(e.nextZ.prevZ=e.prevZ)}function yc(e,t,n){return{i:e,x:t,y:n,prev:null,next:null,z:0,prevZ:null,nextZ:null,steiner:!1}}function bc(e,t,n,r){let i=0;for(let a=t,o=n-r;a<n;a+=r)i+=(e[o]-e[a])*(e[a+1]+e[o+1]),o=a;return i}var xc=class{static triangulate(e,t,n=2){return Hs(e,t,n)}},Sc=class e{static area(e){let t=e.length,n=0;for(let r=t-1,i=0;i<t;r=i++)n+=e[r].x*e[i].y-e[i].x*e[r].y;return n*.5}static isClockWise(t){return e.area(t)<0}static triangulateShape(e,t){let n=[],r=[],i=[];Cc(e),wc(n,e);let a=e.length;t.forEach(Cc);for(let e=0;e<t.length;e++)r.push(a),a+=t[e].length,wc(n,t[e]);let o=xc.triangulate(n,r);for(let e=0;e<o.length;e+=3)i.push(o.slice(e,e+3));return i}};function Cc(e){let t=e.length;t>2&&e[t-1].equals(e[0])&&e.pop()}function wc(e,t){for(let n=0;n<t.length;n++)e.push(t[n].x),e.push(t[n].y)}var Tc=class e extends Z{constructor(e=new Vs([new K(.5,.5),new K(-.5,.5),new K(-.5,-.5),new K(.5,-.5)]),t={}){super(),this.type=`ExtrudeGeometry`,this.parameters={shapes:e,options:t},e=Array.isArray(e)?e:[e];let n=this,r=[],i=[];for(let t=0,n=e.length;t<n;t++){let n=e[t];a(n)}this.setAttribute(`position`,new X(r,3)),this.setAttribute(`uv`,new X(i,2)),this.computeVertexNormals();function a(e){let a=[],o=t.curveSegments===void 0?12:t.curveSegments,s=t.steps===void 0?1:t.steps,c=t.depth===void 0?1:t.depth,l=t.bevelEnabled===void 0||t.bevelEnabled,u=t.bevelThickness===void 0?.2:t.bevelThickness,d=t.bevelSize===void 0?u-.1:t.bevelSize,f=t.bevelOffset===void 0?0:t.bevelOffset,p=t.bevelSegments===void 0?3:t.bevelSegments,m=t.extrudePath,h=t.UVGenerator===void 0?Ec:t.UVGenerator,g,_=!1,v,y,b,x;if(m){g=m.getSpacedPoints(s),_=!0,l=!1;let e=m.isCatmullRomCurve3?m.closed:!1;v=m.computeFrenetFrames(s,e),y=new q,b=new q,x=new q}l||(p=0,u=0,d=0,f=0);let S=e.extractPoints(o),C=S.shape,w=S.holes;if(!Sc.isClockWise(C)){C=C.reverse();for(let e=0,t=w.length;e<t;e++){let t=w[e];Sc.isClockWise(t)&&(w[e]=t.reverse())}}function T(e){let t=e[0];for(let n=1;n<=e.length;n++){let r=n%e.length,i=e[r],a=i.x-t.x,o=i.y-t.y,s=a*a+o*o,c=Math.max(Math.abs(i.x),Math.abs(i.y),Math.abs(t.x),Math.abs(t.y));s<=10000000000000001e-36*c*c?(e.splice(r,1),n--):t=i}}T(C),w.forEach(T);let E=w.length,D=C;for(let e=0;e<E;e++){let t=w[e];C=C.concat(t)}function O(e,t,n){return t||W(`ExtrudeGeometry: vec does not exist`),e.clone().addScaledVector(t,n)}let k=C.length;function A(e,t,n){let r,i,a,o=e.x-t.x,s=e.y-t.y,c=n.x-e.x,l=n.y-e.y,u=o*o+s*s,d=o*l-s*c;if(Math.abs(d)>2**-52){let d=Math.sqrt(u),f=Math.sqrt(c*c+l*l),p=t.x-s/d,m=t.y+o/d,h=n.x-l/f,g=n.y+c/f,_=((h-p)*l-(g-m)*c)/(o*l-s*c);r=p+o*_-e.x,i=m+s*_-e.y;let v=r*r+i*i;if(v<=2)return new K(r,i);a=Math.sqrt(v/2)}else{let e=!1;o>2**-52?c>2**-52&&(e=!0):o<-(2**-52)?c<-(2**-52)&&(e=!0):Math.sign(s)===Math.sign(l)&&(e=!0),e?(r=-s,i=o,a=Math.sqrt(u)):(r=o,i=s,a=Math.sqrt(u/2))}return new K(r/a,i/a)}let j=[];for(let e=0,t=D.length,n=t-1,r=e+1;e<t;e++,n++,r++)n===t&&(n=0),r===t&&(r=0),j[e]=A(D[e],D[n],D[r]);let M=[],N,P=j.concat();for(let e=0,t=E;e<t;e++){let t=w[e];N=[];for(let e=0,n=t.length,r=n-1,i=e+1;e<n;e++,r++,i++)r===n&&(r=0),i===n&&(i=0),N[e]=A(t[e],t[r],t[i]);M.push(N),P=P.concat(N)}let F;if(p===0)F=Sc.triangulateShape(D,w);else{let e=[],t=[];for(let n=0;n<p;n++){let r=n/p,i=u*Math.cos(r*Math.PI/2),a=d*Math.sin(r*Math.PI/2)+f;for(let t=0,n=D.length;t<n;t++){let n=O(D[t],j[t],a);ie(n.x,n.y,-i),r===0&&e.push(n)}for(let e=0,n=E;e<n;e++){let n=w[e];N=M[e];let o=[];for(let e=0,t=n.length;e<t;e++){let t=O(n[e],N[e],a);ie(t.x,t.y,-i),r===0&&o.push(t)}r===0&&t.push(o)}}F=Sc.triangulateShape(e,t)}let I=F.length,ee=d+f;for(let e=0;e<k;e++){let t=l?O(C[e],P[e],ee):C[e];_?(b.copy(v.normals[0]).multiplyScalar(t.x),y.copy(v.binormals[0]).multiplyScalar(t.y),x.copy(g[0]).add(b).add(y),ie(x.x,x.y,x.z)):ie(t.x,t.y,0)}for(let e=1;e<=s;e++)for(let t=0;t<k;t++){let n=l?O(C[t],P[t],ee):C[t];_?(b.copy(v.normals[e]).multiplyScalar(n.x),y.copy(v.binormals[e]).multiplyScalar(n.y),x.copy(g[e]).add(b).add(y),ie(x.x,x.y,x.z)):ie(n.x,n.y,c/s*e)}for(let e=p-1;e>=0;e--){let t=e/p,n=u*Math.cos(t*Math.PI/2),r=d*Math.sin(t*Math.PI/2)+f;for(let e=0,t=D.length;e<t;e++){let t=O(D[e],j[e],r);ie(t.x,t.y,c+n)}for(let e=0,t=w.length;e<t;e++){let t=w[e];N=M[e];for(let e=0,i=t.length;e<i;e++){let i=O(t[e],N[e],r);_?ie(i.x,i.y+g[s-1].y,g[s-1].x+n):ie(i.x,i.y,c+n)}}}te(),ne();function te(){let e=r.length/3;if(l){let e=0,t=k*e;for(let e=0;e<I;e++){let n=F[e];ae(n[2]+t,n[1]+t,n[0]+t)}e=s+p*2,t=k*e;for(let e=0;e<I;e++){let n=F[e];ae(n[0]+t,n[1]+t,n[2]+t)}}else{for(let e=0;e<I;e++){let t=F[e];ae(t[2],t[1],t[0])}for(let e=0;e<I;e++){let t=F[e];ae(t[0]+k*s,t[1]+k*s,t[2]+k*s)}}n.addGroup(e,r.length/3-e,0)}function ne(){let e=r.length/3,t=0;re(D,t),t+=D.length;for(let e=0,n=w.length;e<n;e++){let n=w[e];re(n,t),t+=n.length}n.addGroup(e,r.length/3-e,1)}function re(e,t){let n=e.length;for(;--n>=0;){let r=n,i=n-1;i<0&&(i=e.length-1);for(let e=0,n=s+p*2;e<n;e++){let n=k*e,a=k*(e+1);oe(t+r+n,t+i+n,t+i+a,t+r+a)}}}function ie(e,t,n){a.push(e),a.push(t),a.push(n)}function ae(e,t,i){L(e),L(t),L(i);let a=r.length/3,o=h.generateTopUV(n,r,a-3,a-2,a-1);se(o[0]),se(o[1]),se(o[2])}function oe(e,t,i,a){L(e),L(t),L(a),L(t),L(i),L(a);let o=r.length/3,s=h.generateSideWallUV(n,r,o-6,o-3,o-2,o-1);se(s[0]),se(s[1]),se(s[3]),se(s[1]),se(s[2]),se(s[3])}function L(e){r.push(a[e*3+0]),r.push(a[e*3+1]),r.push(a[e*3+2])}function se(e){i.push(e.x),i.push(e.y)}}}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}toJSON(){let e=super.toJSON(),t=this.parameters.shapes,n=this.parameters.options;return Dc(t,n,e)}static fromJSON(t,n){let r=[];for(let e=0,i=t.shapes.length;e<i;e++){let i=n[t.shapes[e]];r.push(i)}let i=t.options.extrudePath;return i!==void 0&&(t.options.extrudePath=new Rs[i.type]().fromJSON(i)),new e(r,t.options)}},Ec={generateTopUV:function(e,t,n,r,i){let a=t[n*3],o=t[n*3+1],s=t[r*3],c=t[r*3+1],l=t[i*3],u=t[i*3+1];return[new K(a,o),new K(s,c),new K(l,u)]},generateSideWallUV:function(e,t,n,r,i,a){let o=t[n*3],s=t[n*3+1],c=t[n*3+2],l=t[r*3],u=t[r*3+1],d=t[r*3+2],f=t[i*3],p=t[i*3+1],m=t[i*3+2],h=t[a*3],g=t[a*3+1],_=t[a*3+2];return Math.abs(s-u)<Math.abs(o-l)?[new K(o,1-c),new K(l,1-d),new K(f,1-m),new K(h,1-_)]:[new K(s,1-c),new K(u,1-d),new K(p,1-m),new K(g,1-_)]}};function Dc(e,t,n){if(n.shapes=[],Array.isArray(e))for(let t=0,r=e.length;t<r;t++){let r=e[t];n.shapes.push(r.uuid)}else n.shapes.push(e.uuid);return n.options=Object.assign({},t),t.extrudePath!==void 0&&(n.options.extrudePath=t.extrudePath.toJSON()),n}var Oc=class e extends is{constructor(e=1,t=0){let n=(1+Math.sqrt(5))/2,r=[-1,n,0,1,n,0,-1,-n,0,1,-n,0,0,-1,n,0,1,n,0,-1,-n,0,1,-n,n,0,-1,n,0,1,-n,0,-1,-n,0,1];super(r,[0,11,5,0,5,1,0,1,7,0,7,10,0,10,11,1,5,9,5,11,4,11,10,2,10,7,6,7,1,8,3,9,4,3,4,2,3,2,6,3,6,8,3,8,9,4,9,5,2,4,11,6,2,10,8,6,7,9,8,1],e,t),this.type=`IcosahedronGeometry`,this.parameters={radius:e,detail:t}}static fromJSON(t){return new e(t.radius,t.detail)}},kc=class e extends Z{constructor(e=[new K(0,-.5),new K(.5,0),new K(0,.5)],t=12,n=0,r=Math.PI*2){super(),this.type=`LatheGeometry`,this.parameters={points:e,segments:t,phiStart:n,phiLength:r},t=Math.floor(t),r=G(r,0,Math.PI*2);let i=[],a=[],o=[],s=[],c=[],l=1/t,u=new q,d=new K,f=new q,p=new q,m=new q,h=0,g=0;for(let t=0;t<=e.length-1;t++)switch(t){case 0:h=e[t+1].x-e[t].x,g=e[t+1].y-e[t].y,f.x=g*1,f.y=-h,f.z=g*0,m.copy(f),f.normalize(),s.push(f.x,f.y,f.z);break;case e.length-1:s.push(m.x,m.y,m.z);break;default:h=e[t+1].x-e[t].x,g=e[t+1].y-e[t].y,f.x=g*1,f.y=-h,f.z=g*0,p.copy(f),f.x+=m.x,f.y+=m.y,f.z+=m.z,f.normalize(),s.push(f.x,f.y,f.z),m.copy(p)}for(let i=0;i<=t;i++){let f=n+i*l*r,p=Math.sin(f),m=Math.cos(f);for(let n=0;n<=e.length-1;n++){u.x=e[n].x*p,u.y=e[n].y,u.z=e[n].x*m,a.push(u.x,u.y,u.z),d.x=i/t,d.y=n/(e.length-1),o.push(d.x,d.y);let r=s[3*n+0]*p,l=s[3*n+1],f=s[3*n+0]*m;c.push(r,l,f)}}for(let n=0;n<t;n++)for(let t=0;t<e.length-1;t++){let r=t+n*e.length,a=r,o=r+e.length,s=r+e.length+1,c=r+1;i.push(a,o,c),i.push(s,c,o)}this.setIndex(i),this.setAttribute(`position`,new X(a,3)),this.setAttribute(`uv`,new X(o,2)),this.setAttribute(`normal`,new X(c,3))}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.points,t.segments,t.phiStart,t.phiLength)}},Ac=class e extends is{constructor(e=1,t=0){super([1,0,0,-1,0,0,0,1,0,0,-1,0,0,0,1,0,0,-1],[0,2,4,0,4,3,0,3,5,0,5,2,1,2,5,1,5,3,1,3,4,1,4,2],e,t),this.type=`OctahedronGeometry`,this.parameters={radius:e,detail:t}}static fromJSON(t){return new e(t.radius,t.detail)}},jc=class e extends Z{constructor(e=1,t=1,n=1,r=1){super(),this.type=`PlaneGeometry`,this.parameters={width:e,height:t,widthSegments:n,heightSegments:r};let i=e/2,a=t/2,o=Math.floor(n),s=Math.floor(r),c=o+1,l=s+1,u=e/o,d=t/s,f=[],p=[],m=[],h=[];for(let e=0;e<l;e++){let t=e*d-a;for(let n=0;n<c;n++){let r=n*u-i;p.push(r,-t,0),m.push(0,0,1),h.push(n/o),h.push(1-e/s)}}for(let e=0;e<s;e++)for(let t=0;t<o;t++){let n=t+c*e,r=t+c*(e+1),i=t+1+c*(e+1),a=t+1+c*e;f.push(n,r,a),f.push(r,i,a)}this.setIndex(f),this.setAttribute(`position`,new X(p,3)),this.setAttribute(`normal`,new X(m,3)),this.setAttribute(`uv`,new X(h,2))}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.width,t.height,t.widthSegments,t.heightSegments)}},Mc=class e extends Z{constructor(e=.5,t=1,n=32,r=1,i=0,a=Math.PI*2){super(),this.type=`RingGeometry`,this.parameters={innerRadius:e,outerRadius:t,thetaSegments:n,phiSegments:r,thetaStart:i,thetaLength:a},n=Math.max(3,n),r=Math.max(1,r);let o=[],s=[],c=[],l=[],u=e,d=(t-e)/r,f=new q,p=new K;for(let e=0;e<=r;e++){for(let e=0;e<=n;e++){let r=i+e/n*a;f.x=u*Math.cos(r),f.y=u*Math.sin(r),s.push(f.x,f.y,f.z),c.push(0,0,1),p.x=(f.x/t+1)/2,p.y=(f.y/t+1)/2,l.push(p.x,p.y)}u+=d}for(let e=0;e<r;e++){let t=e*(n+1);for(let e=0;e<n;e++){let r=e+t,i=r,a=r+n+1,s=r+n+2,c=r+1;o.push(i,a,c),o.push(a,s,c)}}this.setIndex(o),this.setAttribute(`position`,new X(s,3)),this.setAttribute(`normal`,new X(c,3)),this.setAttribute(`uv`,new X(l,2))}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.innerRadius,t.outerRadius,t.thetaSegments,t.phiSegments,t.thetaStart,t.thetaLength)}},Nc=class e extends Z{constructor(e=new Vs([new K(0,.5),new K(-.5,-.5),new K(.5,-.5)]),t=12){super(),this.type=`ShapeGeometry`,this.parameters={shapes:e,curveSegments:t};let n=[],r=[],i=[],a=[],o=0,s=0;if(Array.isArray(e)===!1)c(e);else for(let t=0;t<e.length;t++)c(e[t]),this.addGroup(o,s,t),o+=s,s=0;this.setIndex(n),this.setAttribute(`position`,new X(r,3)),this.setAttribute(`normal`,new X(i,3)),this.setAttribute(`uv`,new X(a,2));function c(e){let o=r.length/3,c=e.extractPoints(t),l=c.shape,u=c.holes;Sc.isClockWise(l)===!1&&(l=l.reverse());for(let e=0,t=u.length;e<t;e++){let t=u[e];Sc.isClockWise(t)===!0&&(u[e]=t.reverse())}let d=Sc.triangulateShape(l,u);for(let e=0,t=u.length;e<t;e++){let t=u[e];l=l.concat(t)}for(let e=0,t=l.length;e<t;e++){let t=l[e];r.push(t.x,t.y,0),i.push(0,0,1),a.push(t.x,t.y)}for(let e=0,t=d.length;e<t;e++){let t=d[e],r=t[0]+o,i=t[1]+o,a=t[2]+o;n.push(r,i,a),s+=3}}}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}toJSON(){let e=super.toJSON(),t=this.parameters.shapes;return Pc(t,e)}static fromJSON(t,n){let r=[];for(let e=0,i=t.shapes.length;e<i;e++){let i=n[t.shapes[e]];r.push(i)}return new e(r,t.curveSegments)}};function Pc(e,t){if(t.shapes=[],Array.isArray(e))for(let n=0,r=e.length;n<r;n++){let r=e[n];t.shapes.push(r.uuid)}else t.shapes.push(e.uuid);return t}var Fc=class e extends Z{constructor(e=1,t=32,n=16,r=0,i=Math.PI*2,a=0,o=Math.PI){super(),this.type=`SphereGeometry`,this.parameters={radius:e,widthSegments:t,heightSegments:n,phiStart:r,phiLength:i,thetaStart:a,thetaLength:o},t=Math.max(3,Math.floor(t)),n=Math.max(2,Math.floor(n));let s=Math.min(a+o,Math.PI),c=0,l=[],u=new q,d=new q,f=[],p=[],m=[],h=[];for(let f=0;f<=n;f++){let g=[],_=f/n,v=a+_*o,y=e*Math.cos(v),b=Math.sqrt(e*e-y*y),x=0;f===0&&a===0?x=.5/t:f===n&&s===Math.PI&&(x=-.5/t);for(let e=0;e<=t;e++){let n=e/t,a=r+n*i;u.x=-b*Math.cos(a),u.y=y,u.z=b*Math.sin(a),p.push(u.x,u.y,u.z),d.copy(u).normalize(),m.push(d.x,d.y,d.z),h.push(n+x,1-_),g.push(c++)}l.push(g)}for(let e=0;e<n;e++)for(let r=0;r<t;r++){let t=l[e][r+1],i=l[e][r],o=l[e+1][r],c=l[e+1][r+1];(e!==0||a>0)&&f.push(t,i,c),(e!==n-1||s<Math.PI)&&f.push(i,o,c)}this.setIndex(f),this.setAttribute(`position`,new X(p,3)),this.setAttribute(`normal`,new X(m,3)),this.setAttribute(`uv`,new X(h,2))}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.radius,t.widthSegments,t.heightSegments,t.phiStart,t.phiLength,t.thetaStart,t.thetaLength)}},Ic=class e extends is{constructor(e=1,t=0){super([1,1,1,-1,-1,1,-1,1,-1,1,-1,-1],[2,1,0,0,3,2,1,3,0,2,3,1],e,t),this.type=`TetrahedronGeometry`,this.parameters={radius:e,detail:t}}static fromJSON(t){return new e(t.radius,t.detail)}},Lc=class e extends Z{constructor(e=1,t=.4,n=12,r=48,i=Math.PI*2,a=0,o=Math.PI*2){super(),this.type=`TorusGeometry`,this.parameters={radius:e,tube:t,radialSegments:n,tubularSegments:r,arc:i,thetaStart:a,thetaLength:o},n=Math.floor(n),r=Math.floor(r);let s=[],c=[],l=[],u=[],d=new q,f=new q,p=new q;for(let s=0;s<=n;s++){let m=a+s/n*o;for(let a=0;a<=r;a++){let o=a/r*i;f.x=(e+t*Math.cos(m))*Math.cos(o),f.y=(e+t*Math.cos(m))*Math.sin(o),f.z=t*Math.sin(m),c.push(f.x,f.y,f.z),d.x=e*Math.cos(o),d.y=e*Math.sin(o),p.subVectors(f,d).normalize(),l.push(p.x,p.y,p.z),u.push(a/r),u.push(s/n)}}for(let e=1;e<=n;e++)for(let t=1;t<=r;t++){let n=(r+1)*e+t-1,i=(r+1)*(e-1)+t-1,a=(r+1)*(e-1)+t,o=(r+1)*e+t;s.push(n,i,o),s.push(i,a,o)}this.setIndex(s),this.setAttribute(`position`,new X(c,3)),this.setAttribute(`normal`,new X(l,3)),this.setAttribute(`uv`,new X(u,2))}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.radius,t.tube,t.radialSegments,t.tubularSegments,t.arc,t.thetaStart,t.thetaLength)}},Rc=class e extends Z{constructor(e=1,t=.4,n=64,r=8,i=2,a=3){super(),this.type=`TorusKnotGeometry`,this.parameters={radius:e,tube:t,tubularSegments:n,radialSegments:r,p:i,q:a},n=Math.floor(n),r=Math.floor(r);let o=[],s=[],c=[],l=[],u=new q,d=new q,f=new q,p=new q,m=new q,h=new q,g=new q;for(let o=0;o<=n;++o){let v=o/n*i*Math.PI*2;_(v,i,a,e,f),_(v+.01,i,a,e,p),h.subVectors(p,f),g.addVectors(p,f),m.crossVectors(h,g),g.crossVectors(m,h),m.normalize(),g.normalize();for(let e=0;e<=r;++e){let i=e/r*Math.PI*2,a=-t*Math.cos(i),p=t*Math.sin(i);u.x=f.x+(a*g.x+p*m.x),u.y=f.y+(a*g.y+p*m.y),u.z=f.z+(a*g.z+p*m.z),s.push(u.x,u.y,u.z),d.subVectors(u,f).normalize(),c.push(d.x,d.y,d.z),l.push(o/n),l.push(e/r)}}for(let e=1;e<=n;e++)for(let t=1;t<=r;t++){let n=(r+1)*(e-1)+(t-1),i=(r+1)*e+(t-1),a=(r+1)*e+t,s=(r+1)*(e-1)+t;o.push(n,i,s),o.push(i,a,s)}this.setIndex(o),this.setAttribute(`position`,new X(s,3)),this.setAttribute(`normal`,new X(c,3)),this.setAttribute(`uv`,new X(l,2));function _(e,t,n,r,i){let a=Math.cos(e),o=Math.sin(e),s=n/t*e,c=Math.cos(s);i.x=r*(2+c)*.5*a,i.y=r*(2+c)*o*.5,i.z=r*Math.sin(s)*.5}}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(t){return new e(t.radius,t.tube,t.tubularSegments,t.radialSegments,t.p,t.q)}},zc=class e extends Z{constructor(e=new Is(new q(-1,-1,0),new q(-1,1,0),new q(1,1,0)),t=64,n=1,r=8,i=!1){super(),this.type=`TubeGeometry`,this.parameters={path:e,tubularSegments:t,radius:n,radialSegments:r,closed:i};let a=e.computeFrenetFrames(t,i);this.tangents=a.tangents,this.normals=a.normals,this.binormals=a.binormals;let o=new q,s=new q,c=new K,l=new q,u=[],d=[],f=[],p=[];m(),this.setIndex(p),this.setAttribute(`position`,new X(u,3)),this.setAttribute(`normal`,new X(d,3)),this.setAttribute(`uv`,new X(f,2));function m(){for(let e=0;e<t;e++)h(e);h(i===!1?t:0),_(),g()}function h(i){l=e.getPointAt(i/t,l);let c=a.normals[i],f=a.binormals[i];for(let e=0;e<=r;e++){let t=e/r*Math.PI*2,i=Math.sin(t),a=-Math.cos(t);s.x=a*c.x+i*f.x,s.y=a*c.y+i*f.y,s.z=a*c.z+i*f.z,s.normalize(),d.push(s.x,s.y,s.z),o.x=l.x+n*s.x,o.y=l.y+n*s.y,o.z=l.z+n*s.z,u.push(o.x,o.y,o.z)}}function g(){for(let e=1;e<=t;e++)for(let t=1;t<=r;t++){let n=(r+1)*(e-1)+(t-1),i=(r+1)*e+(t-1),a=(r+1)*e+t,o=(r+1)*(e-1)+t;p.push(n,i,o),p.push(i,a,o)}}function _(){for(let e=0;e<=t;e++)for(let n=0;n<=r;n++)c.x=e/t,c.y=n/r,f.push(c.x,c.y)}}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}toJSON(){let e=super.toJSON();return e.path=this.parameters.path.toJSON(),e}static fromJSON(t){return new e(new Rs[t.path.type]().fromJSON(t.path),t.tubularSegments,t.radius,t.radialSegments,t.closed)}},Bc=class extends Z{constructor(e=null){if(super(),this.type=`WireframeGeometry`,this.parameters={geometry:e},e!==null){let t=[],n=new Set,r=new q,i=new q;if(e.index!==null){let a=e.attributes.position,o=e.index,s=e.groups;s.length===0&&(s=[{start:0,count:o.count,materialIndex:0}]);for(let e=0,c=s.length;e<c;++e){let c=s[e],l=c.start,u=c.count;for(let e=l,s=l+u;e<s;e+=3)for(let s=0;s<3;s++){let c=o.getX(e+s),l=o.getX(e+(s+1)%3);r.fromBufferAttribute(a,c),i.fromBufferAttribute(a,l),Vc(r,i,n)===!0&&(t.push(r.x,r.y,r.z),t.push(i.x,i.y,i.z))}}}else{let a=e.attributes.position;for(let e=0,o=a.count/3;e<o;e++)for(let o=0;o<3;o++){let s=3*e+o,c=3*e+(o+1)%3;r.fromBufferAttribute(a,s),i.fromBufferAttribute(a,c),Vc(r,i,n)===!0&&(t.push(r.x,r.y,r.z),t.push(i.x,i.y,i.z))}}this.setAttribute(`position`,new X(t,3))}}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}};function Vc(e,t,n){let r=`${e.x},${e.y},${e.z}-${t.x},${t.y},${t.z}`,i=`${t.x},${t.y},${t.z}-${e.x},${e.y},${e.z}`;return n.has(r)===!0||n.has(i)===!0?!1:(n.add(r),n.add(i),!0)}var Hc=Object.freeze({__proto__:null,BoxGeometry:$o,CapsuleGeometry:es,CircleGeometry:ts,ConeGeometry:rs,CylinderGeometry:ns,DodecahedronGeometry:as,EdgesGeometry:us,ExtrudeGeometry:Tc,IcosahedronGeometry:Oc,LatheGeometry:kc,OctahedronGeometry:Ac,PlaneGeometry:jc,PolyhedronGeometry:is,RingGeometry:Mc,ShapeGeometry:Nc,SphereGeometry:Fc,TetrahedronGeometry:Ic,TorusGeometry:Lc,TorusKnotGeometry:Rc,TubeGeometry:zc,WireframeGeometry:Bc}),Uc=class extends Ri{constructor(e){super(),this.isShadowMaterial=!0,this.type=`ShadowMaterial`,this.color=new Y(0),this.transparent=!0,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.fog=e.fog,this}};function Wc(e){let t={};for(let n in e){t[n]={};for(let r in e[n]){let i=e[n][r];if(Kc(i))i.isRenderTargetTexture?(U(`UniformsUtils: Textures of render targets cannot be cloned via cloneUniforms() or mergeUniforms().`),t[n][r]=null):t[n][r]=i.clone();else if(Array.isArray(i)){if(Kc(i[0])){let e=[];for(let t=0,n=i.length;t<n;t++)e[t]=i[t].clone();t[n][r]=e}else t[n][r]=i.slice()}else t[n][r]=i}}return t}function Gc(e){let t={};for(let n=0;n<e.length;n++){let r=Wc(e[n]);for(let e in r)t[e]=r[e]}return t}function Kc(e){return e&&(e.isColor||e.isMatrix3||e.isMatrix4||e.isVector2||e.isVector3||e.isVector4||e.isTexture||e.isQuaternion)}function qc(e){let t=[];for(let n=0;n<e.length;n++)t.push(e[n].clone());return t}function Jc(e){let t=e.getRenderTarget();return t===null?e.outputColorSpace:t.isXRRenderTarget===!0?t.texture.colorSpace:wn.workingColorSpace}var Yc={clone:Wc,merge:Gc},Xc=`void main() {
	gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
}`,Zc=`void main() {
	gl_FragColor = vec4( 1.0, 0.0, 0.0, 1.0 );
}`,Qc=class extends Ri{constructor(e){super(),this.isShaderMaterial=!0,this.type=`ShaderMaterial`,this.defines={},this.uniforms={},this.uniformsGroups=[],this.vertexShader=Xc,this.fragmentShader=Zc,this.linewidth=1,this.wireframe=!1,this.wireframeLinewidth=1,this.fog=!1,this.lights=!1,this.clipping=!1,this.forceSinglePass=!0,this.extensions={clipCullDistance:!1,multiDraw:!1},this.defaultAttributeValues={color:[1,1,1],uv:[0,0],uv1:[0,0]},this.index0AttributeName=void 0,this.uniformsNeedUpdate=!1,this.glslVersion=null,e!==void 0&&this.setValues(e)}copy(e){return super.copy(e),this.fragmentShader=e.fragmentShader,this.vertexShader=e.vertexShader,this.uniforms=Wc(e.uniforms),this.uniformsGroups=qc(e.uniformsGroups),this.defines=Object.assign({},e.defines),this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this.fog=e.fog,this.lights=e.lights,this.clipping=e.clipping,this.extensions=Object.assign({},e.extensions),this.glslVersion=e.glslVersion,this.defaultAttributeValues=Object.assign({},e.defaultAttributeValues),this.index0AttributeName=e.index0AttributeName,this.uniformsNeedUpdate=e.uniformsNeedUpdate,this}toJSON(e){let t=super.toJSON(e);t.glslVersion=this.glslVersion,t.uniforms={};for(let n in this.uniforms){let r=this.uniforms[n].value;r&&r.isTexture?t.uniforms[n]={type:`t`,value:r.toJSON(e).uuid}:r&&r.isColor?t.uniforms[n]={type:`c`,value:r.getHex()}:r&&r.isVector2?t.uniforms[n]={type:`v2`,value:r.toArray()}:r&&r.isVector3?t.uniforms[n]={type:`v3`,value:r.toArray()}:r&&r.isVector4?t.uniforms[n]={type:`v4`,value:r.toArray()}:r&&r.isMatrix3?t.uniforms[n]={type:`m3`,value:r.toArray()}:r&&r.isMatrix4?t.uniforms[n]={type:`m4`,value:r.toArray()}:t.uniforms[n]={value:r}}Object.keys(this.defines).length>0&&(t.defines=this.defines),t.vertexShader=this.vertexShader,t.fragmentShader=this.fragmentShader,t.lights=this.lights,t.clipping=this.clipping;let n={};for(let e in this.extensions)this.extensions[e]===!0&&(n[e]=!0);return Object.keys(n).length>0&&(t.extensions=n),t}fromJSON(e,t){if(super.fromJSON(e,t),e.uniforms!==void 0)for(let n in e.uniforms){let r=e.uniforms[n];switch(this.uniforms[n]={},r.type){case`t`:this.uniforms[n].value=t[r.value]||null;break;case`c`:this.uniforms[n].value=new Y().setHex(r.value);break;case`v2`:this.uniforms[n].value=new K().fromArray(r.value);break;case`v3`:this.uniforms[n].value=new q().fromArray(r.value);break;case`v4`:this.uniforms[n].value=new In().fromArray(r.value);break;case`m3`:this.uniforms[n].value=new yn().fromArray(r.value);break;case`m4`:this.uniforms[n].value=new J().fromArray(r.value);break;default:this.uniforms[n].value=r.value}}if(e.defines!==void 0&&(this.defines=e.defines),e.vertexShader!==void 0&&(this.vertexShader=e.vertexShader),e.fragmentShader!==void 0&&(this.fragmentShader=e.fragmentShader),e.glslVersion!==void 0&&(this.glslVersion=e.glslVersion),e.extensions!==void 0)for(let t in e.extensions)this.extensions[t]=e.extensions[t];return e.lights!==void 0&&(this.lights=e.lights),e.clipping!==void 0&&(this.clipping=e.clipping),this}},$c=class extends Qc{constructor(e){super(e),this.isRawShaderMaterial=!0,this.type=`RawShaderMaterial`}},el=class extends Ri{constructor(e){super(),this.isMeshStandardMaterial=!0,this.type=`MeshStandardMaterial`,this.defines={STANDARD:``},this.color=new Y(16777215),this.roughness=1,this.metalness=0,this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.emissive=new Y(0),this.emissiveIntensity=1,this.emissiveMap=null,this.bumpMap=null,this.bumpScale=1,this.normalMap=null,this.normalMapType=0,this.normalScale=new K(1,1),this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.roughnessMap=null,this.metalnessMap=null,this.alphaMap=null,this.envMap=null,this.envMapRotation=new Qn,this.envMapIntensity=1,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap=`round`,this.wireframeLinejoin=`round`,this.flatShading=!1,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.defines={STANDARD:``},this.color.copy(e.color),this.roughness=e.roughness,this.metalness=e.metalness,this.map=e.map,this.lightMap=e.lightMap,this.lightMapIntensity=e.lightMapIntensity,this.aoMap=e.aoMap,this.aoMapIntensity=e.aoMapIntensity,this.emissive.copy(e.emissive),this.emissiveMap=e.emissiveMap,this.emissiveIntensity=e.emissiveIntensity,this.bumpMap=e.bumpMap,this.bumpScale=e.bumpScale,this.normalMap=e.normalMap,this.normalMapType=e.normalMapType,this.normalScale.copy(e.normalScale),this.displacementMap=e.displacementMap,this.displacementScale=e.displacementScale,this.displacementBias=e.displacementBias,this.roughnessMap=e.roughnessMap,this.metalnessMap=e.metalnessMap,this.alphaMap=e.alphaMap,this.envMap=e.envMap,this.envMapRotation.copy(e.envMapRotation),this.envMapIntensity=e.envMapIntensity,this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this.wireframeLinecap=e.wireframeLinecap,this.wireframeLinejoin=e.wireframeLinejoin,this.flatShading=e.flatShading,this.fog=e.fog,this}},tl=class extends el{constructor(e){super(),this.isMeshPhysicalMaterial=!0,this.defines={STANDARD:``,PHYSICAL:``},this.type=`MeshPhysicalMaterial`,this.anisotropyRotation=0,this.anisotropyMap=null,this.clearcoatMap=null,this.clearcoatRoughness=0,this.clearcoatRoughnessMap=null,this.clearcoatNormalScale=new K(1,1),this.clearcoatNormalMap=null,this.ior=1.5,Object.defineProperty(this,"reflectivity",{get:function(){return G(2.5*(this.ior-1)/(this.ior+1),0,1)},set:function(e){this.ior=(1+.4*e)/(1-.4*e)}}),this.iridescenceMap=null,this.iridescenceIOR=1.3,this.iridescenceThicknessRange=[100,400],this.iridescenceThicknessMap=null,this.sheenColor=new Y(0),this.sheenColorMap=null,this.sheenRoughness=1,this.sheenRoughnessMap=null,this.transmissionMap=null,this.thickness=0,this.thicknessMap=null,this.attenuationDistance=1/0,this.attenuationColor=new Y(1,1,1),this.specularIntensity=1,this.specularIntensityMap=null,this.specularColor=new Y(1,1,1),this.specularColorMap=null,this._anisotropy=0,this._clearcoat=0,this._dispersion=0,this._iridescence=0,this._retroreflectivity=0,this._sheen=0,this._transmission=0,this.setValues(e)}get anisotropy(){return this._anisotropy}set anisotropy(e){this._anisotropy>0!=e>0&&this.version++,this._anisotropy=e}get clearcoat(){return this._clearcoat}set clearcoat(e){this._clearcoat>0!=e>0&&this.version++,this._clearcoat=e}get iridescence(){return this._iridescence}set iridescence(e){this._iridescence>0!=e>0&&this.version++,this._iridescence=e}get dispersion(){return this._dispersion}set dispersion(e){this._dispersion>0!=e>0&&this.version++,this._dispersion=e}get retroreflectivity(){return this._retroreflectivity}set retroreflectivity(e){this._retroreflectivity>0!=e>0&&this.version++,this._retroreflectivity=e}get sheen(){return this._sheen}set sheen(e){this._sheen>0!=e>0&&this.version++,this._sheen=e}get transmission(){return this._transmission}set transmission(e){this._transmission>0!=e>0&&this.version++,this._transmission=e}copy(e){return super.copy(e),this.defines={STANDARD:``,PHYSICAL:``},this.anisotropy=e.anisotropy,this.anisotropyRotation=e.anisotropyRotation,this.anisotropyMap=e.anisotropyMap,this.clearcoat=e.clearcoat,this.clearcoatMap=e.clearcoatMap,this.clearcoatRoughness=e.clearcoatRoughness,this.clearcoatRoughnessMap=e.clearcoatRoughnessMap,this.clearcoatNormalMap=e.clearcoatNormalMap,this.clearcoatNormalScale.copy(e.clearcoatNormalScale),this.dispersion=e.dispersion,this.ior=e.ior,this.iridescence=e.iridescence,this.iridescenceMap=e.iridescenceMap,this.iridescenceIOR=e.iridescenceIOR,this.iridescenceThicknessRange=[...e.iridescenceThicknessRange],this.iridescenceThicknessMap=e.iridescenceThicknessMap,this.retroreflectivity=e.retroreflectivity,this.sheen=e.sheen,this.sheenColor.copy(e.sheenColor),this.sheenColorMap=e.sheenColorMap,this.sheenRoughness=e.sheenRoughness,this.sheenRoughnessMap=e.sheenRoughnessMap,this.transmission=e.transmission,this.transmissionMap=e.transmissionMap,this.thickness=e.thickness,this.thicknessMap=e.thicknessMap,this.attenuationDistance=e.attenuationDistance,this.attenuationColor.copy(e.attenuationColor),this.specularIntensity=e.specularIntensity,this.specularIntensityMap=e.specularIntensityMap,this.specularColor.copy(e.specularColor),this.specularColorMap=e.specularColorMap,this}},nl=class extends Ri{constructor(e){super(),this.isMeshPhongMaterial=!0,this.type=`MeshPhongMaterial`,this.color=new Y(16777215),this.specular=new Y(1118481),this.shininess=30,this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.emissive=new Y(0),this.emissiveIntensity=1,this.emissiveMap=null,this.bumpMap=null,this.bumpScale=1,this.normalMap=null,this.normalMapType=0,this.normalScale=new K(1,1),this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.specularMap=null,this.alphaMap=null,this.envMap=null,this.envMapRotation=new Qn,this.combine=0,this.reflectivity=1,this.envMapIntensity=1,this.refractionRatio=.98,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap=`round`,this.wireframeLinejoin=`round`,this.flatShading=!1,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.specular.copy(e.specular),this.shininess=e.shininess,this.map=e.map,this.lightMap=e.lightMap,this.lightMapIntensity=e.lightMapIntensity,this.aoMap=e.aoMap,this.aoMapIntensity=e.aoMapIntensity,this.emissive.copy(e.emissive),this.emissiveMap=e.emissiveMap,this.emissiveIntensity=e.emissiveIntensity,this.bumpMap=e.bumpMap,this.bumpScale=e.bumpScale,this.normalMap=e.normalMap,this.normalMapType=e.normalMapType,this.normalScale.copy(e.normalScale),this.displacementMap=e.displacementMap,this.displacementScale=e.displacementScale,this.displacementBias=e.displacementBias,this.specularMap=e.specularMap,this.alphaMap=e.alphaMap,this.envMap=e.envMap,this.envMapRotation.copy(e.envMapRotation),this.combine=e.combine,this.reflectivity=e.reflectivity,this.envMapIntensity=e.envMapIntensity,this.refractionRatio=e.refractionRatio,this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this.wireframeLinecap=e.wireframeLinecap,this.wireframeLinejoin=e.wireframeLinejoin,this.flatShading=e.flatShading,this.fog=e.fog,this}},rl=class extends Ri{constructor(e){super(),this.isMeshToonMaterial=!0,this.defines={TOON:``},this.type=`MeshToonMaterial`,this.color=new Y(16777215),this.map=null,this.gradientMap=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.emissive=new Y(0),this.emissiveIntensity=1,this.emissiveMap=null,this.bumpMap=null,this.bumpScale=1,this.normalMap=null,this.normalMapType=0,this.normalScale=new K(1,1),this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.alphaMap=null,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap=`round`,this.wireframeLinejoin=`round`,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.map=e.map,this.gradientMap=e.gradientMap,this.lightMap=e.lightMap,this.lightMapIntensity=e.lightMapIntensity,this.aoMap=e.aoMap,this.aoMapIntensity=e.aoMapIntensity,this.emissive.copy(e.emissive),this.emissiveMap=e.emissiveMap,this.emissiveIntensity=e.emissiveIntensity,this.bumpMap=e.bumpMap,this.bumpScale=e.bumpScale,this.normalMap=e.normalMap,this.normalMapType=e.normalMapType,this.normalScale.copy(e.normalScale),this.displacementMap=e.displacementMap,this.displacementScale=e.displacementScale,this.displacementBias=e.displacementBias,this.alphaMap=e.alphaMap,this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this.wireframeLinecap=e.wireframeLinecap,this.wireframeLinejoin=e.wireframeLinejoin,this.fog=e.fog,this}},il=class extends Ri{constructor(e){super(),this.isMeshNormalMaterial=!0,this.type=`MeshNormalMaterial`,this.bumpMap=null,this.bumpScale=1,this.normalMap=null,this.normalMapType=0,this.normalScale=new K(1,1),this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.wireframe=!1,this.wireframeLinewidth=1,this.flatShading=!1,this.setValues(e)}copy(e){return super.copy(e),this.bumpMap=e.bumpMap,this.bumpScale=e.bumpScale,this.normalMap=e.normalMap,this.normalMapType=e.normalMapType,this.normalScale.copy(e.normalScale),this.displacementMap=e.displacementMap,this.displacementScale=e.displacementScale,this.displacementBias=e.displacementBias,this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this.flatShading=e.flatShading,this}},al=class extends Ri{constructor(e){super(),this.isMeshLambertMaterial=!0,this.type=`MeshLambertMaterial`,this.color=new Y(16777215),this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.emissive=new Y(0),this.emissiveIntensity=1,this.emissiveMap=null,this.bumpMap=null,this.bumpScale=1,this.normalMap=null,this.normalMapType=0,this.normalScale=new K(1,1),this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.specularMap=null,this.alphaMap=null,this.envMap=null,this.envMapRotation=new Qn,this.combine=0,this.reflectivity=1,this.envMapIntensity=1,this.refractionRatio=.98,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap=`round`,this.wireframeLinejoin=`round`,this.flatShading=!1,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.map=e.map,this.lightMap=e.lightMap,this.lightMapIntensity=e.lightMapIntensity,this.aoMap=e.aoMap,this.aoMapIntensity=e.aoMapIntensity,this.emissive.copy(e.emissive),this.emissiveMap=e.emissiveMap,this.emissiveIntensity=e.emissiveIntensity,this.bumpMap=e.bumpMap,this.bumpScale=e.bumpScale,this.normalMap=e.normalMap,this.normalMapType=e.normalMapType,this.normalScale.copy(e.normalScale),this.displacementMap=e.displacementMap,this.displacementScale=e.displacementScale,this.displacementBias=e.displacementBias,this.specularMap=e.specularMap,this.alphaMap=e.alphaMap,this.envMap=e.envMap,this.envMapRotation.copy(e.envMapRotation),this.combine=e.combine,this.reflectivity=e.reflectivity,this.envMapIntensity=e.envMapIntensity,this.refractionRatio=e.refractionRatio,this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this.wireframeLinecap=e.wireframeLinecap,this.wireframeLinejoin=e.wireframeLinejoin,this.flatShading=e.flatShading,this.fog=e.fog,this}},ol=class extends Ri{constructor(e){super(),this.isMeshDepthMaterial=!0,this.type=`MeshDepthMaterial`,this.depthPacking=Ye,this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.wireframe=!1,this.wireframeLinewidth=1,this.setValues(e)}copy(e){return super.copy(e),this.depthPacking=e.depthPacking,this.map=e.map,this.alphaMap=e.alphaMap,this.displacementMap=e.displacementMap,this.displacementScale=e.displacementScale,this.displacementBias=e.displacementBias,this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this}},sl=class extends Ri{constructor(e){super(),this.isMeshDistanceMaterial=!0,this.type=`MeshDistanceMaterial`,this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.setValues(e)}copy(e){return super.copy(e),this.map=e.map,this.alphaMap=e.alphaMap,this.displacementMap=e.displacementMap,this.displacementScale=e.displacementScale,this.displacementBias=e.displacementBias,this}},cl=class extends Ri{constructor(e){super(),this.isMeshMatcapMaterial=!0,this.defines={MATCAP:``},this.type=`MeshMatcapMaterial`,this.color=new Y(16777215),this.matcap=null,this.map=null,this.bumpMap=null,this.bumpScale=1,this.normalMap=null,this.normalMapType=0,this.normalScale=new K(1,1),this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.alphaMap=null,this.wireframe=!1,this.wireframeLinewidth=1,this.flatShading=!1,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.defines={MATCAP:``},this.color.copy(e.color),this.matcap=e.matcap,this.map=e.map,this.bumpMap=e.bumpMap,this.bumpScale=e.bumpScale,this.normalMap=e.normalMap,this.normalMapType=e.normalMapType,this.normalScale.copy(e.normalScale),this.displacementMap=e.displacementMap,this.displacementScale=e.displacementScale,this.displacementBias=e.displacementBias,this.alphaMap=e.alphaMap,this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this.flatShading=e.flatShading,this.fog=e.fog,this}},ll=class extends bo{constructor(e){super(),this.isLineDashedMaterial=!0,this.type=`LineDashedMaterial`,this.scale=1,this.dashSize=3,this.gapSize=1,this.setValues(e)}copy(e){return super.copy(e),this.scale=e.scale,this.dashSize=e.dashSize,this.gapSize=e.gapSize,this}};function ul(e,t){return!e||e.constructor===t?e:typeof t.BYTES_PER_ELEMENT==`number`?new t(e):Array.prototype.slice.call(e)}function dl(e){return e!==void 0&&e.inTangents!==void 0&&e.outTangents!==void 0}function fl(e){function t(t,n){return e[t]-e[n]}let n=e.length,r=Array(n);for(let e=0;e!==n;++e)r[e]=e;return r.sort(t),r}function pl(e,t,n){let r=e.length,i=new e.constructor(r);for(let a=0,o=0;o!==r;++a){let r=n[a]*t;for(let n=0;n!==t;++n)i[o++]=e[r+n]}return i}function ml(e,t,n,r){let i=1,a=e[0];for(;a!==void 0&&a[r]===void 0;)a=e[i++];if(a===void 0)return;let o=a[r];if(o!==void 0){if(Array.isArray(o))do o=a[r],o!==void 0&&(t.push(a.time),n.push(...o)),a=e[i++];while(a!==void 0);else if(o.toArray!==void 0)do o=a[r],o!==void 0&&(t.push(a.time),o.toArray(n,n.length)),a=e[i++];while(a!==void 0);else do o=a[r],o!==void 0&&(t.push(a.time),n.push(o)),a=e[i++];while(a!==void 0)}}function hl(e,t,n,r,i=30){let a=e.clone();a.name=t;let o=[];for(let e=0;e<a.tracks.length;++e){let t=a.tracks[e],s=t.getValueSize(),c=[],l=[];for(let e=0;e<t.times.length;++e){let a=t.times[e]*i;if(!(a<n||a>=r)){c.push(t.times[e]);for(let n=0;n<s;++n)l.push(t.values[e*s+n])}}c.length!==0&&(t.times=ul(c,t.times.constructor),t.values=ul(l,t.values.constructor),o.push(t))}a.tracks=o;let s=1/0;for(let e=0;e<a.tracks.length;++e)s>a.tracks[e].times[0]&&(s=a.tracks[e].times[0]);for(let e=0;e<a.tracks.length;++e)a.tracks[e].shift(-1*s);return a.resetDuration(),a}function gl(e,t=0,n=e,r=30){r<=0&&(r=30);let i=n.tracks.length,a=t/r;for(let t=0;t<i;++t){let r=n.tracks[t],i=r.ValueTypeName;if(i===`bool`||i===`string`)continue;let o=e.tracks.find(function(e){return e.name===r.name&&e.ValueTypeName===i});if(o===void 0)continue;let s=0,c=r.getValueSize();r.createInterpolant.isInterpolantFactoryMethodGLTFCubicSpline&&(s=c/3);let l=0,u=o.getValueSize();o.createInterpolant.isInterpolantFactoryMethodGLTFCubicSpline&&(l=u/3);let d=r.times.length-1,f;if(a<=r.times[0]){let e=s,t=c-s;f=r.values.slice(e,t)}else if(a>=r.times[d]){let e=d*c+s,t=e+c-s;f=r.values.slice(e,t)}else{let e=r.createInterpolant(),t=s,n=c-s;e.evaluate(a),f=e.resultBuffer.slice(t,n)}i===`quaternion`&&new gn().fromArray(f).normalize().conjugate().toArray(f);let p=o.times.length;for(let e=0;e<p;++e){let t=e*u+l;if(i===`quaternion`)gn.multiplyQuaternionsFlat(o.values,t,f,0,o.values,t);else{let e=u-l*2;for(let n=0;n<e;++n)o.values[t+n]-=f[n]}}}return e.blendMode=Je,e}var _l=class{static convertArray(e,t){return ul(e,t)}static isTypedArray(e){return At(e)}static hasTangents(e){return dl(e)}static getKeyframeOrder(e){return fl(e)}static sortedArray(e,t,n){return pl(e,t,n)}static flattenJSON(e,t,n,r){ml(e,t,n,r)}static subclip(e,t,n,r,i=30){return hl(e,t,n,r,i)}static makeClipAdditive(e,t=0,n=e,r=30){return gl(e,t,n,r)}},vl=class{constructor(e,t,n,r){this.parameterPositions=e,this._cachedIndex=0,this.resultBuffer=r===void 0?new t.constructor(n):r,this.sampleValues=t,this.valueSize=n,this.settings=null,this.DefaultSettings_={}}evaluate(e){let t=this.parameterPositions,n=this._cachedIndex,r=t[n],i=t[n-1];validate_interval:{seek:{let a;linear_scan:{forward_scan:if(!(e<r)){for(let a=n+2;;){if(r===void 0){if(e<i)break forward_scan;return n=t.length,this._cachedIndex=n,this.copySampleValue_(n-1)}if(n===a)break;if(i=r,r=t[++n],e<r)break seek}a=t.length;break linear_scan}if(!(e>=i)){let o=t[1];e<o&&(n=2,i=o);for(let a=n-2;;){if(i===void 0)return this._cachedIndex=0,this.copySampleValue_(0);if(n===a)break;if(r=i,i=t[--n-1],e>=i)break seek}a=n,n=0;break linear_scan}break validate_interval}for(;n<a;){let r=n+a>>>1;e<t[r]?a=r:n=r+1}if(r=t[n],i=t[n-1],i===void 0)return this._cachedIndex=0,this.copySampleValue_(0);if(r===void 0)return n=t.length,this._cachedIndex=n,this.copySampleValue_(n-1)}this._cachedIndex=n,this.intervalChanged_(n,i,r)}return this.interpolate_(n,i,e,r)}getSettings_(){return this.settings||this.DefaultSettings_}copySampleValue_(e){let t=this.resultBuffer,n=this.sampleValues,r=this.valueSize,i=e*r;for(let e=0;e!==r;++e)t[e]=n[i+e];return t}interpolate_(){throw Error(`THREE.Interpolant: Call to abstract method.`)}intervalChanged_(){}},yl=class extends vl{constructor(e,t,n,r){super(e,t,n,r),this._weightPrev=-0,this._offsetPrev=-0,this._weightNext=-0,this._offsetNext=-0,this.DefaultSettings_={endingStart:We,endingEnd:We}}intervalChanged_(e,t,n){let r=this.parameterPositions,i=e-2,a=e+1,o=r[i],s=r[a];if(o===void 0)switch(this.getSettings_().endingStart){case Ge:i=e,o=2*t-n;break;case Ke:i=r.length-2,o=t+r[i]-r[i+1];break;default:i=e,o=n}if(s===void 0)switch(this.getSettings_().endingEnd){case Ge:a=e,s=2*n-t;break;case Ke:a=1,s=n+r[1]-r[0];break;default:a=e-1,s=t}let c=(n-t)*.5,l=this.valueSize;this._weightPrev=c/(t-o),this._weightNext=c/(s-n),this._offsetPrev=i*l,this._offsetNext=a*l}interpolate_(e,t,n,r){let i=this.resultBuffer,a=this.sampleValues,o=this.valueSize,s=e*o,c=s-o,l=this._offsetPrev,u=this._offsetNext,d=this._weightPrev,f=this._weightNext,p=(n-t)/(r-t),m=p*p,h=m*p,g=-d*h+2*d*m-d*p,_=(1+d)*h+(-1.5-2*d)*m+(-.5+d)*p+1,v=(-1-f)*h+(1.5+f)*m+.5*p,y=f*h-f*m;for(let e=0;e!==o;++e)i[e]=g*a[l+e]+_*a[c+e]+v*a[s+e]+y*a[u+e];return i}},bl=class extends vl{constructor(e,t,n,r){super(e,t,n,r)}interpolate_(e,t,n,r){let i=this.resultBuffer,a=this.sampleValues,o=this.valueSize,s=e*o,c=s-o,l=(n-t)/(r-t),u=1-l;for(let e=0;e!==o;++e)i[e]=a[c+e]*u+a[s+e]*l;return i}},xl=class extends vl{constructor(e,t,n,r){super(e,t,n,r)}interpolate_(e){return this.copySampleValue_(e-1)}},Sl=class extends vl{interpolate_(e,t,n,r){let i=this.resultBuffer,a=this.sampleValues,o=this.valueSize,s=e*o,c=s-o,l=this.inTangents,u=this.outTangents;if(!l||!u){let e=(n-t)/(r-t),l=1-e;for(let t=0;t!==o;++t)i[t]=a[c+t]*l+a[s+t]*e;return i}let d=o*2,f=e-1;for(let p=0;p!==o;++p){let o=a[c+p],m=a[s+p],h=f*d+p*2,g=u[h],_=u[h+1],v=e*d+p*2,y=l[v],b=l[v+1],x=Tl(n,t,g,y,r);i[p]=Cl(x,o,_,b,m)}return i}};function Cl(e,t,n,r,i){let a=1-e;return a*a*a*t+3*a*a*e*n+3*a*e*e*r+e*e*e*i}function wl(e,t,n,r,i){let a=1-e;return 3*a*a*(n-t)+6*a*e*(r-n)+3*e*e*(i-r)}function Tl(e,t,n,r,i){let a=(e-t)/(i-t);for(let o=0;o<8;o++){let o=Cl(a,t,n,r,i)-e;if(Math.abs(o)<1e-10)break;let s=wl(a,t,n,r,i);if(Math.abs(s)<1e-10)break;a=Math.max(0,Math.min(1,a-o/s))}return a}var El=class{constructor(e,t,n,r){if(e===void 0)throw Error(`THREE.KeyframeTrack: track name is undefined`);if(t===void 0||t.length===0)throw Error(`THREE.KeyframeTrack: no keyframes in track named `+e);this.name=e,this.times=ul(t,this.TimeBufferType),this.values=ul(n,this.ValueBufferType),this.setInterpolation(r||this.DefaultInterpolation)}static toJSON(e){let t=e.constructor,n;if(t.toJSON!==this.toJSON)n=t.toJSON(e);else{n={name:e.name,times:ul(e.times,Array),values:ul(e.values,Array)};let t=e.getInterpolation();t!==e.DefaultInterpolation&&(n.interpolation=t),dl(e.settings)&&(n.settings={inTangents:ul(e.settings.inTangents,Array),outTangents:ul(e.settings.outTangents,Array)})}return n.type=e.ValueTypeName,n}InterpolantFactoryMethodDiscrete(e){return new xl(this.times,this.values,this.getValueSize(),e)}InterpolantFactoryMethodLinear(e){return new bl(this.times,this.values,this.getValueSize(),e)}InterpolantFactoryMethodSmooth(e){return new yl(this.times,this.values,this.getValueSize(),e)}InterpolantFactoryMethodBezier(e){let t=new Sl(this.times,this.values,this.getValueSize(),e);return this.settings&&(t.inTangents=this.settings.inTangents,t.outTangents=this.settings.outTangents),t}setInterpolation(e){let t;switch(e){case Be:t=this.InterpolantFactoryMethodDiscrete;break;case Ve:t=this.InterpolantFactoryMethodLinear;break;case He:t=this.InterpolantFactoryMethodSmooth;break;case Ue:t=this.InterpolantFactoryMethodBezier}if(t===void 0){let t=`unsupported interpolation for `+this.ValueTypeName+` keyframe track named `+this.name;if(this.createInterpolant===void 0){if(e!==this.DefaultInterpolation)this.setInterpolation(this.DefaultInterpolation);else throw Error(t)}return U(`KeyframeTrack:`,t),this}return this.createInterpolant=t,this}getInterpolation(){switch(this.createInterpolant){case this.InterpolantFactoryMethodDiscrete:return Be;case this.InterpolantFactoryMethodLinear:return Ve;case this.InterpolantFactoryMethodSmooth:return He;case this.InterpolantFactoryMethodBezier:return Ue}}getValueSize(){return this.values.length/this.times.length}shift(e){if(e!==0){let t=this.times;for(let n=0,r=t.length;n!==r;++n)t[n]+=e}return this}scale(e){if(e!==1){let t=this.times;for(let n=0,r=t.length;n!==r;++n)t[n]*=e;dl(this.settings)&&(Dl(this.settings.inTangents,e),Dl(this.settings.outTangents,e))}return this}trim(e,t){let n=this.times,r=n.length,i=0,a=r-1;for(;i!==r&&n[i]<e;)++i;for(;a!==-1&&n[a]>t;)--a;if(++a,i!==0||a!==r){i>=a&&(a=Math.max(a,1),i=a-1);let e=this.getValueSize();this.times=n.slice(i,a),this.values=this.values.slice(i*e,a*e)}return this}validate(){let e=!0,t=this.getValueSize();t-Math.floor(t)!==0&&(W(`KeyframeTrack: Invalid value size in track.`,this),e=!1);let n=this.times,r=this.values,i=n.length;i===0&&(W(`KeyframeTrack: Track is empty.`,this),e=!1);let a=null;for(let t=0;t!==i;t++){let r=n[t];if(typeof r==`number`&&isNaN(r)){W(`KeyframeTrack: Time is not a valid number.`,this,t,r),e=!1;break}if(a!==null&&a>r){W(`KeyframeTrack: Out of order keys.`,this,t,r,a),e=!1;break}a=r}if(r!==void 0&&At(r))for(let t=0,n=r.length;t!==n;++t){let n=r[t];if(isNaN(n)){W(`KeyframeTrack: Value is not a valid number.`,this,t,n),e=!1;break}}return e}optimize(){let e=this.times.slice(),t=this.values.slice(),n=this.getValueSize(),r=this.getInterpolation()===He,i=e.length-1,a=1;for(let o=1;o<i;++o){let i=!1,s=e[o];if(s!==e[o+1]&&(o!==1||s!==e[0])){if(r)i=!0;else{let e=o*n,r=e-n,a=e+n;for(let o=0;o!==n;++o){let n=t[e+o];if(n!==t[r+o]||n!==t[a+o]){i=!0;break}}}}if(i){if(o!==a){e[a]=e[o];let r=o*n,i=a*n;for(let e=0;e!==n;++e)t[i+e]=t[r+e]}++a}}if(i>0){e[a]=e[i];for(let e=i*n,r=a*n,o=0;o!==n;++o)t[r+o]=t[e+o];++a}return a===e.length?(this.times=e,this.values=t):(this.times=e.slice(0,a),this.values=t.slice(0,a*n)),this}clone(){let e=this.times.slice(),t=this.values.slice(),n=this.constructor,r=new n(this.name,e,t);return r.createInterpolant=this.createInterpolant,dl(this.settings)&&(r.settings={inTangents:this.settings.inTangents.slice(),outTangents:this.settings.outTangents.slice()}),r}};function Dl(e,t){for(let n=0,r=e.length;n!==r;n+=2)e[n]*=t}El.prototype.ValueTypeName=``,El.prototype.TimeBufferType=Float32Array,El.prototype.ValueBufferType=Float32Array,El.prototype.DefaultInterpolation=Ve;var Ol=class extends El{constructor(e,t,n){super(e,t,n)}};Ol.prototype.ValueTypeName=`bool`,Ol.prototype.ValueBufferType=Array,Ol.prototype.DefaultInterpolation=Be,Ol.prototype.InterpolantFactoryMethodLinear=void 0,Ol.prototype.InterpolantFactoryMethodSmooth=void 0;var kl=class extends El{constructor(e,t,n,r){super(e,t,n,r)}};kl.prototype.ValueTypeName=`color`;var Al=class extends El{constructor(e,t,n,r){super(e,t,n,r)}};Al.prototype.ValueTypeName=`number`;var jl=class extends vl{constructor(e,t,n,r){super(e,t,n,r)}interpolate_(e,t,n,r){let i=this.resultBuffer,a=this.sampleValues,o=this.valueSize,s=(n-t)/(r-t),c=e*o;for(let e=c+o;c!==e;c+=4)gn.slerpFlat(i,0,a,c-o,a,c,s);return i}},Ml=class extends El{constructor(e,t,n,r){super(e,t,n,r)}InterpolantFactoryMethodLinear(e){return new jl(this.times,this.values,this.getValueSize(),e)}};Ml.prototype.ValueTypeName=`quaternion`,Ml.prototype.InterpolantFactoryMethodSmooth=void 0;var Nl=class extends El{constructor(e,t,n){super(e,t,n)}};Nl.prototype.ValueTypeName=`string`,Nl.prototype.ValueBufferType=Array,Nl.prototype.DefaultInterpolation=Be,Nl.prototype.InterpolantFactoryMethodLinear=void 0,Nl.prototype.InterpolantFactoryMethodSmooth=void 0;var Pl=class extends El{constructor(e,t,n,r){super(e,t,n,r)}};Pl.prototype.ValueTypeName=`vector`;var Fl=class{constructor(e=``,t=-1,n=[],r=qe){this.name=e,this.tracks=n,this.duration=t,this.blendMode=r,this.uuid=qt(),this.userData={},this.duration<0&&this.resetDuration()}static parse(e){let t=[],n=e.tracks,r=1/(e.fps||1);for(let e=0,i=n.length;e!==i;++e)t.push(Ll(n[e]).scale(r));let i=new this(e.name,e.duration,t,e.blendMode);return i.uuid=e.uuid,i.userData=JSON.parse(e.userData||`{}`),i}static toJSON(e){let t=[],n=e.tracks,r={name:e.name,duration:e.duration,tracks:t,uuid:e.uuid,blendMode:e.blendMode,userData:JSON.stringify(e.userData)};for(let e=0,r=n.length;e!==r;++e)t.push(El.toJSON(n[e]));return r}static CreateFromMorphTargetSequence(e,t,n,r){let i=t.length,a=[];for(let e=0;e<i;e++){let o=[],s=[];o.push((e+i-1)%i,e,(e+1)%i),s.push(0,1,0);let c=fl(o);o=pl(o,1,c),s=pl(s,1,c),!r&&o[0]===0&&(o.push(i),s.push(s[0])),a.push(new Al(`.morphTargetInfluences[`+t[e].name+`]`,o,s).scale(1/n))}return new this(e,-1,a)}static findByName(e,t){let n=e;if(!Array.isArray(e)){let t=e;n=t.geometry&&t.geometry.animations||t.animations}for(let e=0;e<n.length;e++)if(n[e].name===t)return n[e];return null}static CreateClipsFromMorphTargetSequences(e,t,n){let r={},i=/^([\w-]*?)([\d]+)$/;for(let t=0,n=e.length;t<n;t++){let n=e[t],a=n.name.match(i);if(a&&a.length>1){let e=a[1],t=r[e];t||(r[e]=t=[]),t.push(n)}}let a=[];for(let e in r)a.push(this.CreateFromMorphTargetSequence(e,r[e],t,n));return a}resetDuration(){let e=this.tracks,t=0;for(let n=0,r=e.length;n!==r;++n){let e=this.tracks[n];t=Math.max(t,e.times[e.times.length-1])}return this.duration=t,this}trim(){for(let e=0;e<this.tracks.length;e++)this.tracks[e].trim(0,this.duration);return this}validate(){let e=!0;for(let t=0;t<this.tracks.length;t++)e&&=this.tracks[t].validate();return e}optimize(){for(let e=0;e<this.tracks.length;e++)this.tracks[e].optimize();return this}clone(){let e=[];for(let t=0;t<this.tracks.length;t++)e.push(this.tracks[t].clone());let t=new this.constructor(this.name,this.duration,e,this.blendMode);return t.userData=JSON.parse(JSON.stringify(this.userData)),t}toJSON(){return this.constructor.toJSON(this)}};function Il(e){switch(e.toLowerCase()){case`scalar`:case`double`:case`float`:case`number`:case`integer`:return Al;case`vector`:case`vector2`:case`vector3`:case`vector4`:return Pl;case`color`:return kl;case`quaternion`:return Ml;case`bool`:case`boolean`:return Ol;case`string`:return Nl}throw Error(`THREE.KeyframeTrack: Unsupported typeName: `+e)}function Ll(e){if(e.type===void 0)throw Error(`THREE.KeyframeTrack: track type undefined, can not parse`);let t=Il(e.type);if(e.times===void 0){let t=[],n=[];ml(e.keys,t,n,`value`),e.times=t,e.values=n}let n;return n=t.parse===void 0?new t(e.name,e.times,e.values,e.interpolation):t.parse(e),dl(e.settings)&&(n.settings={inTangents:ul(e.settings.inTangents,Float32Array),outTangents:ul(e.settings.outTangents,Float32Array)}),n}var Rl={enabled:!1,files:{},add:function(e,t){this.enabled!==!1&&(zl(e)||(this.files[e]=t))},get:function(e){if(this.enabled!==!1&&!zl(e))return this.files[e]},remove:function(e){delete this.files[e]},clear:function(){this.files={}}};function zl(e){try{let t=e.slice(e.indexOf(`:`)+1);return new URL(t).protocol===`blob:`}catch{return!1}}var Bl=class{constructor(e,t,n){let r=this,i=!1,a=0,o=0,s,c=[];this.onStart=void 0,this.onLoad=e,this.onProgress=t,this.onError=n,this._abortController=null,this.itemStart=function(e){o++,i===!1&&r.onStart!==void 0&&r.onStart(e,a,o),i=!0},this.itemEnd=function(e){a++,r.onProgress!==void 0&&r.onProgress(e,a,o),a===o&&(i=!1,r.onLoad!==void 0&&r.onLoad())},this.itemError=function(e){r.onError!==void 0&&r.onError(e)},this.resolveURL=function(e){return e=e.normalize(`NFC`),s?s(e):e},this.setURLModifier=function(e){return s=e,this},this.addHandler=function(e,t){return c.push(e,t),this},this.removeHandler=function(e){let t=c.indexOf(e);return t!==-1&&c.splice(t,2),this},this.getHandler=function(e){for(let t=0,n=c.length;t<n;t+=2){let n=c[t],r=c[t+1];if(n.global&&(n.lastIndex=0),n.test(e))return r}return null},this.abort=function(){return this.abortController.abort(),this._abortController=null,this}}get abortController(){return this._abortController||=new AbortController,this._abortController}},Vl=new Bl,Hl=class{constructor(e){this.manager=e===void 0?Vl:e,this.crossOrigin=`anonymous`,this.withCredentials=!1,this.path=``,this.resourcePath=``,this.requestHeader={},typeof __THREE_DEVTOOLS__<`u`&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent(`observe`,{detail:this}))}load(){}loadAsync(e,t){let n=this;return new Promise(function(r,i){n.load(e,r,t,i)})}parse(){}setCrossOrigin(e){return this.crossOrigin=e,this}setWithCredentials(e){return this.withCredentials=e,this}setPath(e){return this.path=e,this}setResourcePath(e){return this.resourcePath=e,this}setRequestHeader(e){return this.requestHeader=e,this}abort(){return this}};Hl.DEFAULT_MATERIAL_NAME=`__DEFAULT`;var Ul={},Wl=class extends Error{constructor(e,t){super(e),this.response=t}},Gl=class extends Hl{constructor(e){super(e),this.mimeType=``,this.responseType=``,this._abortController=new AbortController}load(e,t,n,r){e===void 0&&(e=``),this.path!==void 0&&(e=this.path+e),e=this.manager.resolveURL(e);let i=Rl.get(`file:${e}`);if(i!==void 0){this.manager.itemStart(e),setTimeout(()=>{t&&t(i),this.manager.itemEnd(e)},0);return}if(Ul[e]!==void 0){Ul[e].push({onLoad:t,onProgress:n,onError:r});return}Ul[e]=[],Ul[e].push({onLoad:t,onProgress:n,onError:r});let a=new Request(e,{headers:new Headers(this.requestHeader),credentials:this.withCredentials?`include`:`same-origin`,signal:typeof AbortSignal.any==`function`?AbortSignal.any([this._abortController.signal,this.manager.abortController.signal]):this._abortController.signal}),o=this.mimeType,s=this.responseType;fetch(a).then(t=>{if(t.status===200||t.status===0){if(t.status===0&&U(`FileLoader: HTTP Status 0 received.`),typeof ReadableStream>`u`||t.body===void 0||t.body.getReader===void 0)return t;let n=Ul[e],r=t.body.getReader(),i=t.headers.get(`X-File-Size`)||t.headers.get(`Content-Length`),a=i?parseInt(i):0,o=a!==0,s=0,c=new ReadableStream({start(e){t();function t(){r.read().then(({done:r,value:i})=>{if(r)e.close();else{s+=i.byteLength;let r=new ProgressEvent(`progress`,{lengthComputable:o,loaded:s,total:a});for(let e=0,t=n.length;e<t;e++){let t=n[e];t.onProgress&&t.onProgress(r)}e.enqueue(i),t()}},t=>{e.error(t)})}}});return new Response(c)}throw new Wl(`fetch for "${t.url}" responded with ${t.status}: ${t.statusText}`,t)}).then(e=>{switch(s){case`arraybuffer`:return e.arrayBuffer();case`blob`:return e.blob();case`document`:return e.text().then(e=>new DOMParser().parseFromString(e,o));case`json`:return e.json();default:if(o===``)return e.text();{let t=/charset="?([^;"\s]*)"?/i.exec(o),n=t&&t[1]?t[1].toLowerCase():void 0,r=new TextDecoder(n);return e.arrayBuffer().then(e=>r.decode(e))}}}).then(t=>{Rl.add(`file:${e}`,t);let n=Ul[e];delete Ul[e];for(let e=0,r=n.length;e<r;e++){let r=n[e];r.onLoad&&r.onLoad(t)}}).catch(t=>{let n=Ul[e];if(n===void 0)throw this.manager.itemError(e),t;delete Ul[e];for(let e=0,r=n.length;e<r;e++){let r=n[e];r.onError&&r.onError(t)}this.manager.itemError(e)}).finally(()=>{this.manager.itemEnd(e)}),this.manager.itemStart(e)}setResponseType(e){return this.responseType=e,this}setMimeType(e){return this.mimeType=e,this}abort(){return this._abortController.abort(),this._abortController=new AbortController,this}},Kl=class extends Hl{constructor(e){super(e)}load(e,t,n,r){let i=this,a=new Gl(this.manager);a.setPath(this.path),a.setRequestHeader(this.requestHeader),a.setWithCredentials(this.withCredentials),a.load(e,function(n){try{t(i.parse(JSON.parse(n)))}catch(t){r?r(t):W(t),i.manager.itemError(e)}},n,r)}parse(e){let t=[];for(let n=0;n<e.length;n++){let r=Fl.parse(e[n]);t.push(r)}return t}},ql=class extends Hl{constructor(e){super(e)}load(e,t,n,r){let i=this,a=[],o=new Wo,s=new Gl(this.manager);s.setPath(this.path),s.setResponseType(`arraybuffer`),s.setRequestHeader(this.requestHeader),s.setWithCredentials(i.withCredentials);let c=0;function l(l){s.load(e[l],function(e){let n=i.parse(e,!0);a[l]={width:n.width,height:n.height,format:n.format,mipmaps:n.mipmaps},c+=1,c===6&&(n.mipmapCount===1&&(o.minFilter=h),o.image=a,o.format=n.format,o.needsUpdate=!0,t&&t(o))},n,r)}if(Array.isArray(e))for(let t=0,n=e.length;t<n;++t)l(t);else s.load(e,function(e){let n=i.parse(e,!0);if(n.isCubemap){let e=n.mipmaps.length/n.mipmapCount;for(let t=0;t<e;t++){a[t]={mipmaps:[]};for(let e=0;e<n.mipmapCount;e++)a[t].mipmaps.push(n.mipmaps[t*n.mipmapCount+e]),a[t].format=n.format,a[t].width=n.width,a[t].height=n.height}o.image=a}else o.image.width=n.width,o.image.height=n.height,o.mipmaps=n.mipmaps;n.mipmapCount===1&&(o.minFilter=h),o.format=n.format,o.needsUpdate=!0,t&&t(o)},n,r);return o}},Jl=new WeakMap,Yl=class extends Hl{constructor(e){super(e)}load(e,t,n,r){this.path!==void 0&&(e=this.path+e),e=this.manager.resolveURL(e);let i=this,a=Rl.get(`image:${e}`);if(a!==void 0){if(a.complete===!0)i.manager.itemStart(e),setTimeout(function(){t&&t(a),i.manager.itemEnd(e)},0);else{let e=Jl.get(a);e===void 0&&(e=[],Jl.set(a,e)),e.push({onLoad:t,onError:r})}return a}let o=jt(`img`);function s(){l(),t&&t(this);let n=Jl.get(this)||[];for(let e=0;e<n.length;e++){let t=n[e];t.onLoad&&t.onLoad(this)}Jl.delete(this),i.manager.itemEnd(e)}function c(t){l(),r&&r(t),Rl.remove(`image:${e}`);let n=Jl.get(this)||[];for(let e=0;e<n.length;e++){let r=n[e];r.onError&&r.onError(t)}Jl.delete(this),i.manager.itemError(e),i.manager.itemEnd(e)}function l(){o.removeEventListener(`load`,s,!1),o.removeEventListener(`error`,c,!1)}return o.addEventListener(`load`,s,!1),o.addEventListener(`error`,c,!1),e.slice(0,5)!==`data:`&&this.crossOrigin!==void 0&&(o.crossOrigin=this.crossOrigin),Rl.add(`image:${e}`,o),i.manager.itemStart(e),o.src=e,o}},Xl=class extends Hl{constructor(e){super(e)}load(e,t,n,r){let i=new qo;i.colorSpace=$e;let a=new Yl(this.manager);a.setCrossOrigin(this.crossOrigin),a.setPath(this.path);let o=0;function s(n){a.load(e[n],function(e){i.images[n]=e,o++,o===6&&(i.needsUpdate=!0,t&&t(i))},void 0,r)}for(let t=0;t<e.length;++t)s(t);return i}},Zl=class extends Hl{constructor(e){super(e)}load(e,t,n,r){let i=this,a=new Fa,o=new Gl(this.manager);return o.setResponseType(`arraybuffer`),o.setRequestHeader(this.requestHeader),o.setPath(this.path),o.setWithCredentials(i.withCredentials),o.load(e,function(e){let n;try{n=i.parse(e)}catch(e){r===void 0?W(e):r(e);return}i._applyTexData(a,n),t&&t(a,n)},n,r),a}createDataTexture(e){let t=new Fa;return this._applyTexData(t,this.parse(e)),t}_applyTexData(e,t){t.image===void 0?t.data!==void 0&&(e.image.width=t.width,e.image.height=t.height,e.image.data=t.data):e.image=t.image,e.wrapS=t.wrapS===void 0?c:t.wrapS,e.wrapT=t.wrapT===void 0?c:t.wrapT,e.magFilter=t.magFilter===void 0?h:t.magFilter,e.minFilter=t.minFilter===void 0?h:t.minFilter,e.anisotropy=t.anisotropy===void 0?1:t.anisotropy,t.colorSpace!==void 0&&(e.colorSpace=t.colorSpace),t.flipY!==void 0&&(e.flipY=t.flipY),t.format!==void 0&&(e.format=t.format),t.type!==void 0&&(e.type=t.type),t.mipmaps!==void 0&&(e.mipmaps=t.mipmaps,e.minFilter=v),t.mipmapCount===1&&(e.minFilter=h),t.generateMipmaps!==void 0&&(e.generateMipmaps=t.generateMipmaps),e.needsUpdate=!0}},Ql=class extends Hl{constructor(e){super(e)}load(e,t,n,r){let i=new Fn,a=new Yl(this.manager);return a.setCrossOrigin(this.crossOrigin),a.setPath(this.path),a.load(e,function(e){i.image=e,i.needsUpdate=!0,t!==void 0&&t(i)},n,r),i}},$l=class extends hr{constructor(e,t=1){super(),this.isLight=!0,this.type=`Light`,this.color=new Y(e),this.intensity=t}copy(e,t){return super.copy(e,t),this.color.copy(e.color),this.intensity=e.intensity,this}toJSON(e){let t=super.toJSON(e);return t.object.color=this.color.getHex(),t.object.intensity=this.intensity,t}},eu=class extends $l{constructor(e,t,n){super(e,n),this.isHemisphereLight=!0,this.type=`HemisphereLight`,this.position.copy(hr.DEFAULT_UP),this.updateMatrix(),this.groundColor=new Y(t)}copy(e,t){return super.copy(e,t),this.groundColor.copy(e.groundColor),this}toJSON(e){let t=super.toJSON(e);return t.object.groundColor=this.groundColor.getHex(),t}},tu=new J,nu=new q,ru=new q,iu=class{constructor(e){this.camera=e,this.intensity=1,this.bias=0,this.biasNode=null,this.normalBias=0,this.radius=1,this.blurSamples=8,this.mapSize=new K(512,512),this.mapType=b,this.map=null,this.mapPass=null,this.matrix=new J,this.autoUpdate=!0,this.needsUpdate=!1,this._frustum=new Za,this._frameExtents=new K(1,1),this._viewportCount=1,this._viewports=[new In(0,0,1,1)]}getViewportCount(){return this._viewportCount}getCamera(){return this.camera}getFrustum(){return this._frustum}updateMatrices(e){let t=this.camera;nu.setFromMatrixPosition(e.matrixWorld),t.position.copy(nu),ru.setFromMatrixPosition(e.target.matrixWorld),t.lookAt(ru),t.updateMatrixWorld(),this._updateMatrix(t,this.matrix,this._frustum)}_updateMatrix(e,t,n,r){tu.multiplyMatrices(e.projectionMatrix,e.matrixWorldInverse),n.setFromProjectionMatrix(tu,e.coordinateSystem,e.reversedDepth);let i=this._frameExtents,a=r?r.z/i.x:1,o=r?r.w/i.y:1,s=r?r.x/i.x:0,c=r?r.y/i.y:0;e.coordinateSystem===2001||e.reversedDepth?t.set(.5*a,0,0,.5*a+s,0,.5*o,0,.5*o+c,0,0,1,0,0,0,0,1):t.set(.5*a,0,0,.5*a+s,0,.5*o,0,.5*o+c,0,0,.5,.5,0,0,0,1),t.multiply(tu)}getViewport(e){return this._viewports[e]}getFrameExtents(){return this._frameExtents}dispose(){this.map&&this.map.dispose(),this.mapPass&&this.mapPass.dispose()}copy(e){return this.camera=e.camera.clone(),this.intensity=e.intensity,this.bias=e.bias,this.radius=e.radius,this.autoUpdate=e.autoUpdate,this.needsUpdate=e.needsUpdate,this.normalBias=e.normalBias,this.blurSamples=e.blurSamples,this.mapSize.copy(e.mapSize),this.biasNode=e.biasNode,this}clone(){return new this.constructor().copy(this)}toJSON(){let e={};return e.intensity=this.intensity,e.bias=this.bias,e.normalBias=this.normalBias,e.radius=this.radius,e.blurSamples=this.blurSamples,e.mapSize=this.mapSize.toArray(),e.camera=this.camera.toJSON(!1).object,delete e.camera.matrix,e}},au=new q,ou=new gn,su=new q,cu=class extends hr{constructor(){super(),this.isCamera=!0,this.type=`Camera`,this.matrixWorldInverse=new J,this.projectionMatrix=new J,this.projectionMatrixInverse=new J,this.coordinateSystem=bt,this._reversedDepth=!1}get reversedDepth(){return this._reversedDepth}copy(e,t){return super.copy(e,t),this.matrixWorldInverse.copy(e.matrixWorldInverse),this.projectionMatrix.copy(e.projectionMatrix),this.projectionMatrixInverse.copy(e.projectionMatrixInverse),this.coordinateSystem=e.coordinateSystem,this}getWorldDirection(e){return super.getWorldDirection(e).negate()}updateMatrixWorld(e){super.updateMatrixWorld(e),this.matrixWorld.decompose(au,ou,su),su.x===1&&su.y===1&&su.z===1?this.matrixWorldInverse.copy(this.matrixWorld).invert():this.matrixWorldInverse.compose(au,ou,su.set(1,1,1)).invert()}updateWorldMatrix(e,t,n=!1){super.updateWorldMatrix(e,t,n),this.matrixWorld.decompose(au,ou,su),su.x===1&&su.y===1&&su.z===1?this.matrixWorldInverse.copy(this.matrixWorld).invert():this.matrixWorldInverse.compose(au,ou,su.set(1,1,1)).invert()}clone(){return new this.constructor().copy(this)}},lu=new q,uu=new K,du=new K,fu=class extends cu{constructor(e=50,t=1,n=.1,r=2e3){super(),this.isPerspectiveCamera=!0,this.type=`PerspectiveCamera`,this.fov=e,this.zoom=1,this.near=n,this.far=r,this.focus=10,this.aspect=t,this.view=null,this.filmGauge=35,this.filmOffset=0,this.updateProjectionMatrix()}copy(e,t){return super.copy(e,t),this.fov=e.fov,this.zoom=e.zoom,this.near=e.near,this.far=e.far,this.focus=e.focus,this.aspect=e.aspect,this.view=e.view===null?null:Object.assign({},e.view),this.filmGauge=e.filmGauge,this.filmOffset=e.filmOffset,this}setFocalLength(e){let t=.5*this.getFilmHeight()/e;this.fov=Kt*2*Math.atan(t),this.updateProjectionMatrix()}getFocalLength(){let e=Math.tan(Gt*.5*this.fov);return .5*this.getFilmHeight()/e}getEffectiveFOV(){return Kt*2*Math.atan(Math.tan(Gt*.5*this.fov)/this.zoom)}getFilmWidth(){return this.filmGauge*Math.min(this.aspect,1)}getFilmHeight(){return this.filmGauge/Math.max(this.aspect,1)}getViewBounds(e,t,n){lu.set(-1,-1,.5).applyMatrix4(this.projectionMatrixInverse),t.set(lu.x,lu.y).multiplyScalar(-e/lu.z),lu.set(1,1,.5).applyMatrix4(this.projectionMatrixInverse),n.set(lu.x,lu.y).multiplyScalar(-e/lu.z)}getViewSize(e,t){return this.getViewBounds(e,uu,du),t.subVectors(du,uu)}setViewOffset(e,t,n,r,i,a){this.aspect=e/t,this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=e,this.view.fullHeight=t,this.view.offsetX=n,this.view.offsetY=r,this.view.width=i,this.view.height=a,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){let e=this.near,t=e*Math.tan(Gt*.5*this.fov)/this.zoom,n=2*t,r=this.aspect*n,i=-.5*r,a=this.view;if(this.view!==null&&this.view.enabled){let e=a.fullWidth,o=a.fullHeight;i+=a.offsetX*r/e,t-=a.offsetY*n/o,r*=a.width/e,n*=a.height/o}let o=this.filmOffset;o!==0&&(i+=e*o/this.getFilmWidth()),this.projectionMatrix.makePerspective(i,i+r,t,t-n,e,this.far,this.coordinateSystem,this.reversedDepth),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(e){let t=super.toJSON(e);return t.object.fov=this.fov,t.object.zoom=this.zoom,t.object.near=this.near,t.object.far=this.far,t.object.focus=this.focus,t.object.aspect=this.aspect,this.view!==null&&(t.object.view=Object.assign({},this.view)),t.object.filmGauge=this.filmGauge,t.object.filmOffset=this.filmOffset,t}},pu=class extends iu{constructor(){super(new fu(50,1,.5,500)),this.isSpotLightShadow=!0,this.focus=1,this.aspect=1}updateMatrices(e){let t=this.camera,n=Kt*2*e.angle*this.focus,r=this.mapSize.width/this.mapSize.height*this.aspect,i=e.distance||t.far;(n!==t.fov||r!==t.aspect||i!==t.far)&&(t.fov=n,t.aspect=r,t.far=i,t.updateProjectionMatrix()),super.updateMatrices(e)}copy(e){return super.copy(e),this.focus=e.focus,this.aspect=e.aspect,this}toJSON(){let e=super.toJSON();return e.focus=this.focus,e.aspect=this.aspect,e}},mu=class extends $l{constructor(e,t,n=0,r=Math.PI/3,i=0,a=2){super(e,t),this.isSpotLight=!0,this.type=`SpotLight`,this.position.copy(hr.DEFAULT_UP),this.updateMatrix(),this.target=new hr,this.distance=n,this.angle=r,this.penumbra=i,this.decay=a,this.map=null,this.shadow=new pu}get power(){return this.intensity*Math.PI}set power(e){this.intensity=e/Math.PI}dispose(){super.dispose(),this.shadow.dispose()}copy(e,t){return super.copy(e,t),this.distance=e.distance,this.angle=e.angle,this.penumbra=e.penumbra,this.decay=e.decay,this.target=e.target.clone(),this.map=e.map,this.shadow=e.shadow.clone(),this}toJSON(e){let t=super.toJSON(e);return t.object.distance=this.distance,t.object.angle=this.angle,t.object.decay=this.decay,t.object.penumbra=this.penumbra,t.object.target=this.target.uuid,this.map&&this.map.isTexture&&(t.object.map=this.map.toJSON(e).uuid),t.object.shadow=this.shadow.toJSON(),t}},hu=class extends iu{constructor(){super(new fu(90,1,.5,500)),this.isPointLightShadow=!0}},gu=class extends $l{constructor(e,t,n=0,r=2){super(e,t),this.isPointLight=!0,this.type=`PointLight`,this.distance=n,this.decay=r,this.shadow=new hu}get power(){return this.intensity*4*Math.PI}set power(e){this.intensity=e/(4*Math.PI)}dispose(){super.dispose(),this.shadow.dispose()}copy(e,t){return super.copy(e,t),this.distance=e.distance,this.decay=e.decay,this.shadow=e.shadow.clone(),this}toJSON(e){let t=super.toJSON(e);return t.object.distance=this.distance,t.object.decay=this.decay,t.object.shadow=this.shadow.toJSON(),t}},_u=class extends cu{constructor(e=-1,t=1,n=1,r=-1,i=.1,a=2e3){super(),this.isOrthographicCamera=!0,this.type=`OrthographicCamera`,this.zoom=1,this.view=null,this.left=e,this.right=t,this.top=n,this.bottom=r,this.near=i,this.far=a,this.updateProjectionMatrix()}copy(e,t){return super.copy(e,t),this.left=e.left,this.right=e.right,this.top=e.top,this.bottom=e.bottom,this.near=e.near,this.far=e.far,this.zoom=e.zoom,this.view=e.view===null?null:Object.assign({},e.view),this}setViewOffset(e,t,n,r,i,a){this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=e,this.view.fullHeight=t,this.view.offsetX=n,this.view.offsetY=r,this.view.width=i,this.view.height=a,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){let e=(this.right-this.left)/(2*this.zoom),t=(this.top-this.bottom)/(2*this.zoom),n=(this.right+this.left)/2,r=(this.top+this.bottom)/2,i=n-e,a=n+e,o=r+t,s=r-t;if(this.view!==null&&this.view.enabled){let e=(this.right-this.left)/this.view.fullWidth/this.zoom,t=(this.top-this.bottom)/this.view.fullHeight/this.zoom;i+=e*this.view.offsetX,a=i+e*this.view.width,o-=t*this.view.offsetY,s=o-t*this.view.height}this.projectionMatrix.makeOrthographic(i,a,o,s,this.near,this.far,this.coordinateSystem,this.reversedDepth),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(e){let t=super.toJSON(e);return t.object.zoom=this.zoom,t.object.left=this.left,t.object.right=this.right,t.object.top=this.top,t.object.bottom=this.bottom,t.object.near=this.near,t.object.far=this.far,this.view!==null&&(t.object.view=Object.assign({},this.view)),t}},vu=class extends iu{constructor(){super(new _u(-5,5,5,-5,.5,500)),this.isDirectionalLightShadow=!0}},yu=class extends $l{constructor(e,t){super(e,t),this.isDirectionalLight=!0,this.type=`DirectionalLight`,this.position.copy(hr.DEFAULT_UP),this.updateMatrix(),this.target=new hr,this.shadow=new vu}dispose(){super.dispose(),this.shadow.dispose()}copy(e){return super.copy(e),this.target=e.target.clone(),this.shadow=e.shadow.clone(),this}toJSON(e){let t=super.toJSON(e);return t.object.shadow=this.shadow.toJSON(),t.object.target=this.target.uuid,t}},bu=class extends $l{constructor(e,t){super(e,t),this.isAmbientLight=!0,this.type=`AmbientLight`}},xu=class extends $l{constructor(e,t,n=10,r=10){super(e,t),this.isRectAreaLight=!0,this.type=`RectAreaLight`,this.width=n,this.height=r}get power(){return this.intensity*this.width*this.height*Math.PI}set power(e){this.intensity=e/(this.width*this.height*Math.PI)}copy(e){return super.copy(e),this.width=e.width,this.height=e.height,this}toJSON(e){let t=super.toJSON(e);return t.object.width=this.width,t.object.height=this.height,t}},Su=class{constructor(){this.isSphericalHarmonics3=!0,this.coefficients=[];for(let e=0;e<9;e++)this.coefficients.push(new q)}set(e){for(let t=0;t<9;t++)this.coefficients[t].copy(e[t]);return this}zero(){for(let e=0;e<9;e++)this.coefficients[e].set(0,0,0);return this}getAt(e,t){let n=e.x,r=e.y,i=e.z,a=this.coefficients;return t.copy(a[0]).multiplyScalar(.282095),t.addScaledVector(a[1],.488603*r),t.addScaledVector(a[2],.488603*i),t.addScaledVector(a[3],.488603*n),t.addScaledVector(a[4],n*r*1.092548),t.addScaledVector(a[5],r*i*1.092548),t.addScaledVector(a[6],.315392*(3*i*i-1)),t.addScaledVector(a[7],n*i*1.092548),t.addScaledVector(a[8],.546274*(n*n-r*r)),t}getIrradianceAt(e,t){let n=e.x,r=e.y,i=e.z,a=this.coefficients;return t.copy(a[0]).multiplyScalar(.886227),t.addScaledVector(a[1],1.023328*r),t.addScaledVector(a[2],1.023328*i),t.addScaledVector(a[3],1.023328*n),t.addScaledVector(a[4],.858086*n*r),t.addScaledVector(a[5],.858086*r*i),t.addScaledVector(a[6],.743125*i*i-.247708),t.addScaledVector(a[7],.858086*n*i),t.addScaledVector(a[8],.429043*(n*n-r*r)),t}add(e){for(let t=0;t<9;t++)this.coefficients[t].add(e.coefficients[t]);return this}addScaledSH(e,t){for(let n=0;n<9;n++)this.coefficients[n].addScaledVector(e.coefficients[n],t);return this}scale(e){for(let t=0;t<9;t++)this.coefficients[t].multiplyScalar(e);return this}lerp(e,t){for(let n=0;n<9;n++)this.coefficients[n].lerp(e.coefficients[n],t);return this}equals(e){for(let t=0;t<9;t++)if(!this.coefficients[t].equals(e.coefficients[t]))return!1;return!0}copy(e){return this.set(e.coefficients)}clone(){return new this.constructor().copy(this)}fromArray(e,t=0){let n=this.coefficients;for(let r=0;r<9;r++)n[r].fromArray(e,t+r*3);return this}toArray(e=[],t=0){let n=this.coefficients;for(let r=0;r<9;r++)n[r].toArray(e,t+r*3);return e}static getBasisAt(e,t){let n=e.x,r=e.y,i=e.z;t[0]=.282095,t[1]=.488603*r,t[2]=.488603*i,t[3]=.488603*n,t[4]=1.092548*n*r,t[5]=1.092548*r*i,t[6]=.315392*(3*i*i-1),t[7]=1.092548*n*i,t[8]=.546274*(n*n-r*r)}},Cu=class extends $l{constructor(e=new Su,t=1){super(void 0,t),this.isLightProbe=!0,this.sh=e}copy(e){return super.copy(e),this.sh.copy(e.sh),this}toJSON(e){let t=super.toJSON(e);return t.object.sh=this.sh.toArray(),t}},wu={},Tu=class e extends Hl{constructor(e){super(e),this.textures={}}load(e,t,n,r){let i=this,a=new Gl(i.manager);a.setPath(i.path),a.setRequestHeader(i.requestHeader),a.setWithCredentials(i.withCredentials),a.load(e,function(n){try{t(i.parse(JSON.parse(n)))}catch(t){r?r(t):W(t),i.manager.itemError(e)}},n,r)}parse(e){let t=this.createMaterialFromType(e.type);return t.fromJSON(e,this.textures),t}setTextures(e){return this.textures=e,this}createMaterialFromType(t){return e.createMaterialFromType(t)}static createMaterialFromType(e){let t={ShadowMaterial:Uc,SpriteMaterial:zi,RawShaderMaterial:$c,ShaderMaterial:Qc,PointsMaterial:Po,MeshPhysicalMaterial:tl,MeshStandardMaterial:el,MeshPhongMaterial:nl,MeshToonMaterial:rl,MeshNormalMaterial:il,MeshLambertMaterial:al,MeshDepthMaterial:ol,MeshDistanceMaterial:sl,MeshBasicMaterial:la,MeshMatcapMaterial:cl,LineDashedMaterial:ll,LineBasicMaterial:bo,Material:Ri,...wu}[e],n;return t===void 0?(zt(`MaterialLoader: Unknown material type "${e}". Use .registerMaterial() before starting the deserialization process.`),n=new Ri):n=new t,n}static registerMaterial(e,t){wu[e]=t}},Eu=class{static extractUrlBase(e){let t=e.lastIndexOf(`/`);return t===-1?`./`:e.slice(0,t+1)}static resolveURL(e,t){return typeof e!=`string`||e===``?``:(/^https?:\/\//i.test(t)&&/^\//.test(e)&&(t=t.replace(/(^https?:\/\/[^\/]+).*/i,`$1`)),/^(https?:)?\/\//i.test(e)||/^data:.*,.*$/i.test(e)||/^blob:.*$/i.test(e)?e:t+e)}},Du=class extends Z{constructor(){super(),this.isInstancedBufferGeometry=!0,this.type=`InstancedBufferGeometry`,this.instanceCount=1/0}copy(e){return super.copy(e),this.instanceCount=e.instanceCount,this}toJSON(){let e=super.toJSON();return e.instanceCount=this.instanceCount,e.isInstancedBufferGeometry=!0,e}},Ou=class extends Hl{constructor(e){super(e)}load(e,t,n,r){let i=this,a=new Gl(i.manager);a.setPath(i.path),a.setRequestHeader(i.requestHeader),a.setWithCredentials(i.withCredentials),a.load(e,function(n){try{t(i.parse(JSON.parse(n)))}catch(t){r?r(t):W(t),i.manager.itemError(e)}},n,r)}parse(e){let t={},n={};function r(e,n){if(t[n]!==void 0)return t[n];let r=e.interleavedBuffers[n],a=i(e,r.buffer),o=new Ai(kt(r.type,a),r.stride);return o.uuid=r.uuid,r.usage!==void 0&&o.setUsage(r.usage),t[n]=o,o}function i(e,t){if(n[t]!==void 0)return n[t];let r=e.arrayBuffers[t],i=new Uint32Array(r).buffer;return n[t]=i,i}let a=e.isInstancedBufferGeometry?new Du:new Z,o=e.data.index;if(o!==void 0){let e=kt(o.type,o.array);a.setIndex(new ui(e,1))}let s=e.data.attributes;for(let t in s){let n=s[t],i;if(n.isInterleavedBufferAttribute)i=new Mi(r(e.data,n.data),n.itemSize,n.offset,n.normalized);else{let e=kt(n.type,n.array);i=new(n.isInstancedBufferAttribute?za:ui)(e,n.itemSize,n.normalized)}n.name!==void 0&&(i.name=n.name),n.usage!==void 0&&i.setUsage(n.usage),n.gpuType!==void 0&&(i.gpuType=n.gpuType),a.setAttribute(t,i)}let c=e.data.morphAttributes;if(c)for(let t in c){let n=c[t],i=[];for(let t=0,a=n.length;t<a;t++){let a=n[t],o;o=a.isInterleavedBufferAttribute?new Mi(r(e.data,a.data),a.itemSize,a.offset,a.normalized):new ui(kt(a.type,a.array),a.itemSize,a.normalized),a.name!==void 0&&(o.name=a.name),a.usage!==void 0&&o.setUsage(a.usage),a.gpuType!==void 0&&(o.gpuType=a.gpuType),i.push(o)}a.morphAttributes[t]=i}e.data.morphTargetsRelative&&(a.morphTargetsRelative=!0);let l=e.data.groups||e.data.drawcalls||e.data.offsets;if(l!==void 0)for(let e=0,t=l.length;e!==t;++e){let t=l[e];a.addGroup(t.start,t.count,t.materialIndex)}let u=e.data.boundingSphere;return u!==void 0&&(a.boundingSphere=new Si().fromJSON(u)),e.name&&(a.name=e.name),e.userData&&(a.userData=e.userData),a}},ku={},Au=class extends Hl{constructor(e){super(e)}load(e,t,n,r){let i=this,a=this.path===``?Eu.extractUrlBase(e):this.path;this.resourcePath=this.resourcePath||a;let o=new Gl(this.manager);o.setPath(this.path),o.setRequestHeader(this.requestHeader),o.setWithCredentials(this.withCredentials),o.load(e,function(n){let a=null;try{a=JSON.parse(n)}catch(t){r!==void 0&&r(t),W(`ObjectLoader: Can't parse `+e+`.`,t.message);return}let o=a.metadata;o===void 0||o.type===void 0||o.type.toLowerCase()===`geometry`?(r!==void 0&&r(Error(`THREE.ObjectLoader: Can't load `+e)),W(`ObjectLoader: Can't load `+e)):i.parse(a,t)},n,r)}async loadAsync(e,t){let n=this,r=this.path===``?Eu.extractUrlBase(e):this.path;this.resourcePath=this.resourcePath||r;let i=new Gl(this.manager);i.setPath(this.path),i.setRequestHeader(this.requestHeader),i.setWithCredentials(this.withCredentials);let a=await i.loadAsync(e,t),o;try{o=JSON.parse(a)}catch(t){throw Error(`THREE.ObjectLoader: Can't parse `+e+`. `+t.message)}let s=o.metadata;if(s===void 0||s.type===void 0||s.type.toLowerCase()===`geometry`)throw Error(`THREE.ObjectLoader: Can't load `+e);return await n.parseAsync(o)}parse(e,t){let n=this.parseAnimations(e.animations),r=this.parseShapes(e.shapes),i=this.parseGeometries(e.geometries,r),a=this.parseImages(e.images,function(){t!==void 0&&t(c)}),o=this.parseTextures(e.textures,a),s=this.parseMaterials(e.materials,o),c=this.parseObject(e.object,i,s,o,n),l=this.parseSkeletons(e.skeletons,c);if(this.bindSkeletons(c,l),this.bindLightTargets(c),t!==void 0){let e=!1;for(let t in a)if(a[t].data instanceof HTMLImageElement){e=!0;break}e===!1&&t(c)}return c}async parseAsync(e){let t=this.parseAnimations(e.animations),n=this.parseShapes(e.shapes),r=this.parseGeometries(e.geometries,n),i=await this.parseImagesAsync(e.images),a=this.parseTextures(e.textures,i),o=this.parseMaterials(e.materials,a),s=this.parseObject(e.object,r,o,a,t),c=this.parseSkeletons(e.skeletons,s);return this.bindSkeletons(s,c),this.bindLightTargets(s),s}static registerGeometry(e,t){ku[e]=t}parseShapes(e){let t={};if(e!==void 0)for(let n=0,r=e.length;n<r;n++){let r=new Vs().fromJSON(e[n]);t[r.uuid]=r}return t}parseSkeletons(e,t){let n={},r={};if(t.traverse(function(e){e.isBone&&(r[e.uuid]=e)}),e!==void 0)for(let t=0,i=e.length;t<i;t++){let i=new Ra().fromJSON(e[t],r);n[i.uuid]=i}return n}parseGeometries(e,t){let n={};if(e!==void 0){let r=new Ou;for(let i=0,a=e.length;i<a;i++){let a,o=e[i];switch(o.type){case`BufferGeometry`:case`InstancedBufferGeometry`:a=r.parse(o);break;default:o.type in Hc?a=Hc[o.type].fromJSON(o,t):o.type in ku?a=ku[o.type].fromJSON(o,t):U(`ObjectLoader: Unknown geometry type "${o.type}". Use .registerGeometry() before starting the deserialization process.`)}a.uuid=o.uuid,o.name!==void 0&&(a.name=o.name),o.userData!==void 0&&(a.userData=o.userData),n[o.uuid]=a}}return n}parseMaterials(e,t){let n={},r={};if(e!==void 0){let i=new Tu;i.setTextures(t);for(let t=0,a=e.length;t<a;t++){let a=e[t];n[a.uuid]===void 0&&(n[a.uuid]=i.parse(a)),r[a.uuid]=n[a.uuid]}}return r}parseAnimations(e){let t={};if(e!==void 0)for(let n=0;n<e.length;n++){let r=e[n],i=Fl.parse(r);t[i.uuid]=i}return t}parseImages(e,t){let n=this,r={},i;function a(e){return e=n.manager.resolveURL(e),n.manager.itemStart(e),i.load(e,function(){n.manager.itemEnd(e)},void 0,function(){n.manager.itemError(e),n.manager.itemEnd(e)})}function o(e){if(typeof e==`string`){let t=e;return a(/^(\/\/)|([a-z]+:(\/\/)?)/i.test(t)?t:n.resourcePath+t)}return e.data?{data:kt(e.type,e.data),width:e.width,height:e.height}:null}if(e!==void 0&&e.length>0){i=new Yl(new Bl(t)),i.setCrossOrigin(this.crossOrigin);for(let t=0,n=e.length;t<n;t++){let n=e[t],i=n.url;if(Array.isArray(i)){let e=[];for(let t=0,n=i.length;t<n;t++){let n=i[t],r=o(n);r!==null&&(r instanceof HTMLImageElement?e.push(r):e.push(new Fa(r.data,r.width,r.height)))}r[n.uuid]=new An(e)}else{let e=o(n.url);r[n.uuid]=new An(e)}}}return r}async parseImagesAsync(e){let t=this,n={},r;async function i(e){if(typeof e==`string`){let n=e,i=/^(\/\/)|([a-z]+:(\/\/)?)/i.test(n)?n:t.resourcePath+n;return await r.loadAsync(i)}return e.data?{data:kt(e.type,e.data),width:e.width,height:e.height}:null}if(e!==void 0&&e.length>0){r=new Yl(this.manager),r.setCrossOrigin(this.crossOrigin);for(let t=0,r=e.length;t<r;t++){let r=e[t],a=r.url;if(Array.isArray(a)){let e=[];for(let t=0,n=a.length;t<n;t++){let n=a[t],r=await i(n);r!==null&&(r instanceof HTMLImageElement?e.push(r):e.push(new Fa(r.data,r.width,r.height)))}n[r.uuid]=new An(e)}else{let e=await i(r.url);n[r.uuid]=new An(e)}}}return n}parseTextures(e,t){function n(e,t){return typeof e==`number`?e:(U(`ObjectLoader.parseTexture: Constant should be in numeric form.`,e),t[e])}let r={};if(e!==void 0)for(let i=0,a=e.length;i<a;i++){let a=e[i];a.image===void 0&&U(`ObjectLoader: No "image" specified for`,a.uuid),t[a.image]===void 0&&U(`ObjectLoader: Undefined image`,a.image);let o=t[a.image],s=o.data,c;Array.isArray(s)?(c=new qo,s.length===6&&(c.needsUpdate=!0)):(c=s&&s.data?new Fa:new Fn,s&&(c.needsUpdate=!0)),c.source=o,c.uuid=a.uuid,a.name!==void 0&&(c.name=a.name),a.mapping!==void 0&&(c.mapping=n(a.mapping,ju)),a.channel!==void 0&&(c.channel=a.channel),a.offset!==void 0&&c.offset.fromArray(a.offset),a.repeat!==void 0&&c.repeat.fromArray(a.repeat),a.center!==void 0&&c.center.fromArray(a.center),a.rotation!==void 0&&(c.rotation=a.rotation),a.wrap!==void 0&&(c.wrapS=n(a.wrap[0],Mu),c.wrapT=n(a.wrap[1],Mu)),a.format!==void 0&&(c.format=a.format),a.internalFormat!==void 0&&(c.internalFormat=a.internalFormat),a.type!==void 0&&(c.type=a.type),a.colorSpace!==void 0&&(c.colorSpace=a.colorSpace),a.minFilter!==void 0&&(c.minFilter=n(a.minFilter,Nu)),a.magFilter!==void 0&&(c.magFilter=n(a.magFilter,Nu)),a.anisotropy!==void 0&&(c.anisotropy=a.anisotropy),a.flipY!==void 0&&(c.flipY=a.flipY),a.generateMipmaps!==void 0&&(c.generateMipmaps=a.generateMipmaps),a.premultiplyAlpha!==void 0&&(c.premultiplyAlpha=a.premultiplyAlpha),a.unpackAlignment!==void 0&&(c.unpackAlignment=a.unpackAlignment),a.compareFunction!==void 0&&(c.compareFunction=a.compareFunction),a.normalized!==void 0&&(c.normalized=a.normalized),a.userData!==void 0&&(c.userData=a.userData),r[a.uuid]=c}return r}parseObject(e,t,n,r,i){let a;function o(e){return t[e]===void 0&&U(`ObjectLoader: Undefined geometry`,e),t[e]}function s(e){if(e!==void 0){if(Array.isArray(e)){let t=[];for(let r=0,i=e.length;r<i;r++){let i=e[r];n[i]===void 0&&U(`ObjectLoader: Undefined material`,i),t.push(n[i])}return t}return n[e]===void 0&&U(`ObjectLoader: Undefined material`,e),n[e]}}function c(e){return r[e]===void 0&&U(`ObjectLoader: Undefined texture`,e),r[e]}let l,u;switch(e.type){case`Scene`:a=new Er,e.background!==void 0&&(Number.isInteger(e.background)?a.background=new Y(e.background):a.background=c(e.background)),e.environment!==void 0&&(a.environment=c(e.environment)),e.fog!==void 0&&(e.fog.type===`Fog`?a.fog=new Tr(e.fog.color,e.fog.near,e.fog.far):e.fog.type===`FogExp2`&&(a.fog=new wr(e.fog.color,e.fog.density)),e.fog.name!==``&&(a.fog.name=e.fog.name)),e.backgroundBlurriness!==void 0&&(a.backgroundBlurriness=e.backgroundBlurriness),e.backgroundIntensity!==void 0&&(a.backgroundIntensity=e.backgroundIntensity),e.backgroundRotation!==void 0&&a.backgroundRotation.fromArray(e.backgroundRotation),e.environmentIntensity!==void 0&&(a.environmentIntensity=e.environmentIntensity),e.environmentRotation!==void 0&&a.environmentRotation.fromArray(e.environmentRotation);break;case`PerspectiveCamera`:a=new fu(e.fov,e.aspect,e.near,e.far),e.focus!==void 0&&(a.focus=e.focus),e.zoom!==void 0&&(a.zoom=e.zoom),e.filmGauge!==void 0&&(a.filmGauge=e.filmGauge),e.filmOffset!==void 0&&(a.filmOffset=e.filmOffset),e.view!==void 0&&(a.view=Object.assign({},e.view));break;case`OrthographicCamera`:a=new _u(e.left,e.right,e.top,e.bottom,e.near,e.far),e.zoom!==void 0&&(a.zoom=e.zoom),e.view!==void 0&&(a.view=Object.assign({},e.view));break;case`AmbientLight`:a=new bu(e.color,e.intensity);break;case`DirectionalLight`:a=new yu(e.color,e.intensity),a.target=e.target||``;break;case`PointLight`:a=new gu(e.color,e.intensity,e.distance,e.decay);break;case`RectAreaLight`:a=new xu(e.color,e.intensity,e.width,e.height);break;case`SpotLight`:a=new mu(e.color,e.intensity,e.distance,e.angle,e.penumbra,e.decay),a.target=e.target||``;break;case`HemisphereLight`:a=new eu(e.color,e.groundColor,e.intensity);break;case`LightProbe`:a=new Cu(new Su().fromArray(e.sh),e.intensity);break;case`SkinnedMesh`:l=o(e.geometry),u=s(e.material),a=new Na(l,u),e.bindMode!==void 0&&(a.bindMode=e.bindMode),e.bindMatrix!==void 0&&a.bindMatrix.fromArray(e.bindMatrix),e.skeleton!==void 0&&(a.skeleton=e.skeleton);break;case`Mesh`:l=o(e.geometry),u=s(e.material),a=new xa(l,u);break;case`InstancedMesh`:l=o(e.geometry),u=s(e.material);let t=e.count,n=e.instanceMatrix,r=e.instanceColor;a=new qa(l,u,t),a.instanceMatrix=new za(new Float32Array(n.array),16),r!==void 0&&(a.instanceColor=new za(new Float32Array(r.array),r.itemSize));break;case`BatchedMesh`:l=o(e.geometry),u=s(e.material),a=new yo(e.maxInstanceCount,e.maxVertexCount,e.maxIndexCount,u),a.geometry=l,a.perObjectFrustumCulled=e.perObjectFrustumCulled,a.sortObjects=e.sortObjects,a._drawRanges=e.drawRanges,a._reservedRanges=e.reservedRanges,a._geometryInfo=e.geometryInfo.map(e=>{let t=null,n=null;return e.boundingBox!==void 0&&(t=new Vr().fromJSON(e.boundingBox)),e.boundingSphere!==void 0&&(n=new Si().fromJSON(e.boundingSphere)),{...e,boundingBox:t,boundingSphere:n}}),a._instanceInfo=e.instanceInfo,a._availableInstanceIds=e._availableInstanceIds,a._availableGeometryIds=e._availableGeometryIds,a._nextIndexStart=e.nextIndexStart,a._nextVertexStart=e.nextVertexStart,a._geometryCount=e.geometryCount,a._maxInstanceCount=e.maxInstanceCount,a._maxVertexCount=e.maxVertexCount,a._maxIndexCount=e.maxIndexCount,a._geometryInitialized=e.geometryInitialized,a._matricesTexture=c(e.matricesTexture.uuid),a._indirectTexture=c(e.indirectTexture.uuid),e.colorsTexture!==void 0&&(a._colorsTexture=c(e.colorsTexture.uuid)),e.boundingSphere!==void 0&&(a.boundingSphere=new Si().fromJSON(e.boundingSphere)),e.boundingBox!==void 0&&(a.boundingBox=new Vr().fromJSON(e.boundingBox));break;case`LOD`:a=new ra;break;case`Line`:a=new Oo(o(e.geometry),s(e.material));break;case`LineLoop`:a=new No(o(e.geometry),s(e.material));break;case`LineSegments`:a=new Mo(o(e.geometry),s(e.material));break;case`PointCloud`:case`Points`:a=new zo(o(e.geometry),s(e.material));break;case`Sprite`:a=new $i(s(e.material));break;case`Group`:a=new gr;break;case`Bone`:a=new Pa;break;default:a=new hr}if(a.uuid=e.uuid,e.name!==void 0&&(a.name=e.name),e.matrix===void 0?(e.position!==void 0&&a.position.fromArray(e.position),e.rotation!==void 0&&a.rotation.fromArray(e.rotation),e.quaternion!==void 0&&a.quaternion.fromArray(e.quaternion),e.scale!==void 0&&a.scale.fromArray(e.scale)):(a.matrix.fromArray(e.matrix),e.matrixAutoUpdate!==void 0&&(a.matrixAutoUpdate=e.matrixAutoUpdate),a.matrixAutoUpdate&&a.matrix.decompose(a.position,a.quaternion,a.scale)),e.up!==void 0&&a.up.fromArray(e.up),e.pivot!==void 0&&(a.pivot=new q().fromArray(e.pivot)),e.morphTargetDictionary!==void 0&&(a.morphTargetDictionary=Object.assign({},e.morphTargetDictionary)),e.morphTargetInfluences!==void 0&&(a.morphTargetInfluences=e.morphTargetInfluences.slice()),e.castShadow!==void 0&&(a.castShadow=e.castShadow),e.receiveShadow!==void 0&&(a.receiveShadow=e.receiveShadow),e.shadow&&(e.shadow.intensity!==void 0&&(a.shadow.intensity=e.shadow.intensity),e.shadow.bias!==void 0&&(a.shadow.bias=e.shadow.bias),e.shadow.normalBias!==void 0&&(a.shadow.normalBias=e.shadow.normalBias),e.shadow.radius!==void 0&&(a.shadow.radius=e.shadow.radius),e.shadow.blurSamples!==void 0&&(a.shadow.blurSamples=e.shadow.blurSamples),e.shadow.focus!==void 0&&(a.shadow.focus=e.shadow.focus),e.shadow.aspect!==void 0&&(a.shadow.aspect=e.shadow.aspect),e.shadow.mapSize!==void 0&&a.shadow.mapSize.fromArray(e.shadow.mapSize),e.shadow.camera!==void 0&&(a.shadow.camera=this.parseObject(e.shadow.camera))),e.visible!==void 0&&(a.visible=e.visible),e.frustumCulled!==void 0&&(a.frustumCulled=e.frustumCulled),e.renderOrder!==void 0&&(a.renderOrder=e.renderOrder),e.static!==void 0&&(a.static=e.static),e.userData!==void 0&&(a.userData=e.userData),e.layers!==void 0&&(a.layers.mask=e.layers),e.children!==void 0){let o=e.children;for(let e=0;e<o.length;e++)a.add(this.parseObject(o[e],t,n,r,i))}if(e.animations!==void 0){let t=e.animations;for(let e=0;e<t.length;e++){let n=t[e];a.animations.push(i[n])}}if(e.type===`LOD`){e.autoUpdate!==void 0&&(a.autoUpdate=e.autoUpdate);let t=e.levels;for(let e=0;e<t.length;e++){let n=t[e],r=a.getObjectByProperty(`uuid`,n.object);r!==void 0&&a.addLevel(r,n.distance,n.hysteresis)}}return a}bindSkeletons(e,t){Object.keys(t).length!==0&&e.traverse(function(e){if(e.isSkinnedMesh===!0&&e.skeleton!==void 0){let n=t[e.skeleton];n===void 0?U(`ObjectLoader: No skeleton found with UUID:`,e.skeleton):e.bind(n,e.bindMatrix)}})}bindLightTargets(e){e.traverse(function(t){if(t.isDirectionalLight||t.isSpotLight){let n=t.target,r=e.getObjectByProperty(`uuid`,n);t.target=r===void 0?new hr:r}})}},ju={UVMapping:300,CubeReflectionMapping:301,CubeRefractionMapping:302,EquirectangularReflectionMapping:303,EquirectangularRefractionMapping:304,CubeUVReflectionMapping:306},Mu={RepeatWrapping:s,ClampToEdgeWrapping:c,MirroredRepeatWrapping:l},Nu={NearestFilter:u,NearestMipmapNearestFilter:d,NearestMipmapLinearFilter:p,LinearFilter:h,LinearMipmapNearestFilter:g,LinearMipmapLinearFilter:v},Pu=new WeakMap,Fu=class extends Hl{constructor(e){super(e),this.isImageBitmapLoader=!0,typeof createImageBitmap>`u`&&U(`ImageBitmapLoader: createImageBitmap() not supported.`),typeof fetch>`u`&&U(`ImageBitmapLoader: fetch() not supported.`),this.options={premultiplyAlpha:`none`},this._abortController=new AbortController}setOptions(e){return this.options=e,this}load(e,t,n,r){e===void 0&&(e=``),this.path!==void 0&&(e=this.path+e),e=this.manager.resolveURL(e);let i=this,a=Rl.get(`image-bitmap:${e}`);if(a!==void 0){if(i.manager.itemStart(e),a.then){a.then(n=>{Pu.has(a)===!0?(r&&r(Pu.get(a)),i.manager.itemError(e),i.manager.itemEnd(e)):(t&&t(n),i.manager.itemEnd(e))});return}setTimeout(function(){t&&t(a),i.manager.itemEnd(e)},0);return}let o={};o.credentials=this.crossOrigin===`anonymous`?`same-origin`:`include`,o.headers=this.requestHeader,o.signal=typeof AbortSignal.any==`function`?AbortSignal.any([this._abortController.signal,this.manager.abortController.signal]):this._abortController.signal;let s=fetch(e,o).then(function(e){return e.blob()}).then(function(e){return createImageBitmap(e,Object.assign({},i.options,{colorSpaceConversion:`none`}))}).then(function(n){return Rl.add(`image-bitmap:${e}`,n),t&&t(n),i.manager.itemEnd(e),n}).catch(function(t){r&&r(t),Pu.set(s,t),Rl.remove(`image-bitmap:${e}`),i.manager.itemError(e),i.manager.itemEnd(e)});Rl.add(`image-bitmap:${e}`,s),i.manager.itemStart(e)}abort(){return this._abortController.abort(),this._abortController=new AbortController,this}},Iu,Lu=class{static getContext(){return Iu===void 0&&(Iu=new(window.AudioContext||window.webkitAudioContext)),Iu}static setContext(e){Iu=e}},Ru=class extends Hl{constructor(e){super(e)}load(e,t,n,r){let i=this,a=new Gl(this.manager);a.setResponseType(`arraybuffer`),a.setPath(this.path),a.setRequestHeader(this.requestHeader),a.setWithCredentials(this.withCredentials),a.load(e,function(n){try{let r=n.slice(0),a=Lu.getContext(),s=e+`#decode`;i.manager.itemStart(s),a.decodeAudioData(r,function(e){t(e),i.manager.itemEnd(s)}).catch(function(e){o(e),i.manager.itemEnd(s)})}catch(e){o(e)}},n,r);function o(t){r?r(t):W(t),i.manager.itemError(e)}}},zu=new J,Bu=new J,Vu=new J,Hu=class{constructor(){this.type=`StereoCamera`,this.aspect=1,this.eyeSep=.064,this.cameraL=new fu,this.cameraL.layers.enable(1),this.cameraL.matrixAutoUpdate=!1,this.cameraR=new fu,this.cameraR.layers.enable(2),this.cameraR.matrixAutoUpdate=!1,this._cache={focus:null,fov:null,aspect:null,near:null,far:null,zoom:null,eyeSep:null}}update(e){let t=this._cache;if(t.focus!==e.focus||t.fov!==e.fov||t.aspect!==e.aspect*this.aspect||t.near!==e.near||t.far!==e.far||t.zoom!==e.zoom||t.eyeSep!==this.eyeSep){t.focus=e.focus,t.fov=e.fov,t.aspect=e.aspect*this.aspect,t.near=e.near,t.far=e.far,t.zoom=e.zoom,t.eyeSep=this.eyeSep,Vu.copy(e.projectionMatrix);let n=t.eyeSep/2,r=n*t.near/t.focus,i=t.near*Math.tan(Gt*t.fov*.5)/t.zoom,a,o;Bu.elements[12]=-n,zu.elements[12]=n,a=-i*t.aspect+r,o=i*t.aspect+r,Vu.elements[0]=2*t.near/(o-a),Vu.elements[8]=(o+a)/(o-a),this.cameraL.projectionMatrix.copy(Vu),a=-i*t.aspect-r,o=i*t.aspect-r,Vu.elements[0]=2*t.near/(o-a),Vu.elements[8]=(o+a)/(o-a),this.cameraR.projectionMatrix.copy(Vu)}this.cameraL.matrix.copy(e.matrixWorld).multiply(Bu),this.cameraL.matrixWorldNeedsUpdate=!0,this.cameraR.matrix.copy(e.matrixWorld).multiply(zu),this.cameraR.matrixWorldNeedsUpdate=!0}},Uu=-90,Wu=1,Gu=class extends hr{constructor(e,t,n){super(),this.type=`CubeCamera`,this.renderTarget=n,this.coordinateSystem=null,this.activeMipmapLevel=0;let r=new fu(Uu,Wu,e,t);r.layers=this.layers,this.add(r);let i=new fu(Uu,Wu,e,t);i.layers=this.layers,this.add(i);let a=new fu(Uu,Wu,e,t);a.layers=this.layers,this.add(a);let o=new fu(Uu,Wu,e,t);o.layers=this.layers,this.add(o);let s=new fu(Uu,Wu,e,t);s.layers=this.layers,this.add(s);let c=new fu(Uu,Wu,e,t);c.layers=this.layers,this.add(c)}updateCoordinateSystem(){let e=this.coordinateSystem,t=this.children.concat(),[n,r,i,a,o,s]=t;for(let e of t)this.remove(e);if(e===2e3)n.up.set(0,1,0),n.lookAt(1,0,0),r.up.set(0,1,0),r.lookAt(-1,0,0),i.up.set(0,0,-1),i.lookAt(0,1,0),a.up.set(0,0,1),a.lookAt(0,-1,0),o.up.set(0,1,0),o.lookAt(0,0,1),s.up.set(0,1,0),s.lookAt(0,0,-1);else if(e===2001)n.up.set(0,-1,0),n.lookAt(-1,0,0),r.up.set(0,-1,0),r.lookAt(1,0,0),i.up.set(0,0,1),i.lookAt(0,1,0),a.up.set(0,0,-1),a.lookAt(0,-1,0),o.up.set(0,-1,0),o.lookAt(0,0,1),s.up.set(0,-1,0),s.lookAt(0,0,-1);else throw Error(`THREE.CubeCamera.updateCoordinateSystem(): Invalid coordinate system: `+e);for(let e of t)this.add(e),e.updateMatrixWorld()}update(e,t){this.parent===null&&this.updateMatrixWorld();let{renderTarget:n,activeMipmapLevel:r}=this;this.coordinateSystem!==e.coordinateSystem&&(this.coordinateSystem=e.coordinateSystem,this.updateCoordinateSystem());let[i,a,o,s,c,l]=this.children,u=e.getRenderTarget(),d=e.getActiveCubeFace(),f=e.getActiveMipmapLevel(),p=e.xr.enabled;e.xr.enabled=!1;let m=n.texture.generateMipmaps;n.texture.generateMipmaps=!1;let h=!1;h=e.isWebGLRenderer===!0?e.state.buffers.depth.getReversed():e.reversedDepthBuffer,e.setRenderTarget(n,0,r),h&&e.autoClear===!1&&e.clearDepth(),e.render(t,i),e.setRenderTarget(n,1,r),h&&e.autoClear===!1&&e.clearDepth(),e.render(t,a),e.setRenderTarget(n,2,r),h&&e.autoClear===!1&&e.clearDepth(),e.render(t,o),e.setRenderTarget(n,3,r),h&&e.autoClear===!1&&e.clearDepth(),e.render(t,s),e.setRenderTarget(n,4,r),h&&e.autoClear===!1&&e.clearDepth(),e.render(t,c),n.texture.generateMipmaps=m,e.setRenderTarget(n,5,r),h&&e.autoClear===!1&&e.clearDepth(),e.render(t,l),e.setRenderTarget(u,d,f),e.xr.enabled=p,n.texture.needsPMREMUpdate=!0}},Ku=class extends fu{constructor(e=[]){super(),this.isArrayCamera=!0,this.isMultiViewCamera=!1,this.cameras=e}},qu=class{constructor(){this._previousTime=0,this._currentTime=0,this._startTime=performance.now(),this._delta=0,this._elapsed=0,this._timescale=1,this._document=null,this._pageVisibilityHandler=null}connect(e){this._document=e,e.hidden!==void 0&&(this._pageVisibilityHandler=Ju.bind(this),e.addEventListener(`visibilitychange`,this._pageVisibilityHandler,!1))}disconnect(){this._pageVisibilityHandler!==null&&(this._document.removeEventListener(`visibilitychange`,this._pageVisibilityHandler),this._pageVisibilityHandler=null),this._document=null}getDelta(){return this._delta/1e3}getElapsed(){return this._elapsed/1e3}getTimescale(){return this._timescale}setTimescale(e){return this._timescale=e,this}reset(){return this._currentTime=performance.now()-this._startTime,this}dispose(){this.disconnect()}update(e){return this._pageVisibilityHandler!==null&&this._document.hidden===!0?this._delta=0:(this._previousTime=this._currentTime,this._currentTime=(e===void 0?performance.now():e)-this._startTime,this._delta=(this._currentTime-this._previousTime)*this._timescale,this._elapsed+=this._delta),this}};function Ju(){this._document.hidden===!1&&this.reset()}var Yu=new q,Xu=new gn,Zu=new q,Qu=new q,$u=new q,ed=class extends hr{constructor(){super(),this.type=`AudioListener`,this.context=Lu.getContext(),this.gain=this.context.createGain(),this.gain.connect(this.context.destination),this.filter=null,this.timeDelta=0,this._timer=new qu}getInput(){return this.gain}removeFilter(){return this.filter!==null&&(this.gain.disconnect(this.filter),this.filter.disconnect(this.context.destination),this.gain.connect(this.context.destination),this.filter=null),this}getFilter(){return this.filter}setFilter(e){return this.filter===null?this.gain.disconnect(this.context.destination):(this.gain.disconnect(this.filter),this.filter.disconnect(this.context.destination)),this.filter=e,this.gain.connect(this.filter),this.filter.connect(this.context.destination),this}getMasterVolume(){return this.gain.gain.value}setMasterVolume(e){return this.gain.gain.setTargetAtTime(e,this.context.currentTime,.01),this}updateMatrixWorld(e){super.updateMatrixWorld(e),this._timer.update();let t=this.context.listener;if(this.timeDelta=this._timer.getDelta(),this.matrixWorld.decompose(Yu,Xu,Zu),Qu.set(0,0,-1).applyQuaternion(Xu),$u.set(0,1,0).applyQuaternion(Xu),t.positionX){let e=this.context.currentTime+this.timeDelta;t.positionX.linearRampToValueAtTime(Yu.x,e),t.positionY.linearRampToValueAtTime(Yu.y,e),t.positionZ.linearRampToValueAtTime(Yu.z,e),t.forwardX.linearRampToValueAtTime(Qu.x,e),t.forwardY.linearRampToValueAtTime(Qu.y,e),t.forwardZ.linearRampToValueAtTime(Qu.z,e),t.upX.linearRampToValueAtTime($u.x,e),t.upY.linearRampToValueAtTime($u.y,e),t.upZ.linearRampToValueAtTime($u.z,e)}else t.setPosition(Yu.x,Yu.y,Yu.z),t.setOrientation(Qu.x,Qu.y,Qu.z,$u.x,$u.y,$u.z)}},td=class extends hr{constructor(e){super(),this.type=`Audio`,this.listener=e,this.context=e.context,this.gain=this.context.createGain(),this.gain.connect(e.getInput()),this.autoplay=!1,this.buffer=null,this.detune=0,this.loop=!1,this.loopStart=0,this.loopEnd=0,this.offset=0,this.duration=void 0,this.playbackRate=1,this.isPlaying=!1,this.hasPlaybackControl=!0,this.source=null,this.sourceType=`empty`,this._startedAt=0,this._progress=0,this._connected=!1,this.filters=[]}getOutput(){return this.gain}setNodeSource(e){return this.hasPlaybackControl=!1,this.sourceType=`audioNode`,this.source=e,this.connect(),this}setMediaElementSource(e){return this.hasPlaybackControl=!1,this.sourceType=`mediaNode`,this.source=this.context.createMediaElementSource(e),this.connect(),this}setMediaStreamSource(e){return this.hasPlaybackControl=!1,this.sourceType=`mediaStreamNode`,this.source=this.context.createMediaStreamSource(e),this.connect(),this}setBuffer(e){return this.buffer=e,this.sourceType=`buffer`,this.autoplay&&this.play(),this}play(e=0){if(this.isPlaying===!0){U(`Audio: Audio is already playing.`);return}if(this.hasPlaybackControl===!1){U(`Audio: this Audio has no playback control.`);return}this._startedAt=this.context.currentTime+e;let t=this.context.createBufferSource();return t.buffer=this.buffer,t.loop=this.loop,t.loopStart=this.loopStart,t.loopEnd=this.loopEnd,t.onended=this.onEnded.bind(this),t.start(this._startedAt,this._progress+this.offset,this.duration),this.isPlaying=!0,this.source=t,this.setDetune(this.detune),this.setPlaybackRate(this.playbackRate),this.connect()}pause(){if(this.hasPlaybackControl===!1)U(`Audio: this Audio has no playback control.`);else return this.isPlaying===!0&&(this._progress+=Math.max(this.context.currentTime-this._startedAt,0)*this.playbackRate,this.loop===!0&&(this._progress%=this.duration||this.buffer.duration),this.source.stop(),this.source.onended=null,this.isPlaying=!1),this}stop(e=0){if(this.hasPlaybackControl===!1)U(`Audio: this Audio has no playback control.`);else return this._progress=0,this.source!==null&&(this.source.stop(this.context.currentTime+e),this.source.onended=null),this.isPlaying=!1,this}connect(){if(this.filters.length>0){this.source.connect(this.filters[0]);for(let e=1,t=this.filters.length;e<t;e++)this.filters[e-1].connect(this.filters[e]);this.filters[this.filters.length-1].connect(this.getOutput())}else this.source.connect(this.getOutput());return this._connected=!0,this}disconnect(){if(this._connected!==!1){if(this.filters.length>0){this.source.disconnect(this.filters[0]);for(let e=1,t=this.filters.length;e<t;e++)this.filters[e-1].disconnect(this.filters[e]);this.filters[this.filters.length-1].disconnect(this.getOutput())}else this.source.disconnect(this.getOutput());return this._connected=!1,this}}getFilters(){return this.filters}setFilters(e){return e||=[],this._connected===!0?(this.disconnect(),this.filters=e.slice(),this.connect()):this.filters=e.slice(),this}setDetune(e){return this.detune=e,this.isPlaying===!0&&this.source.detune!==void 0&&this.source.detune.setTargetAtTime(this.detune,this.context.currentTime,.01),this}getDetune(){return this.detune}getFilter(){return this.getFilters()[0]}setFilter(e){return this.setFilters(e?[e]:[])}setPlaybackRate(e){if(this.hasPlaybackControl===!1)U(`Audio: this Audio has no playback control.`);else return this.playbackRate=e,this.isPlaying===!0&&this.source.playbackRate.setTargetAtTime(this.playbackRate,this.context.currentTime,.01),this}getPlaybackRate(){return this.playbackRate}onEnded(){this.isPlaying=!1,this._progress=0}getLoop(){return this.hasPlaybackControl===!1?(U(`Audio: this Audio has no playback control.`),!1):this.loop}setLoop(e){if(this.hasPlaybackControl===!1)U(`Audio: this Audio has no playback control.`);else return this.loop=e,this.isPlaying===!0&&(this.source.loop=this.loop),this}setLoopStart(e){return this.loopStart=e,this}setLoopEnd(e){return this.loopEnd=e,this}getVolume(){return this.gain.gain.value}setVolume(e){return this.gain.gain.setTargetAtTime(e,this.context.currentTime,.01),this}copy(e,t){return super.copy(e,t),e.sourceType===`buffer`?(this.autoplay=e.autoplay,this.buffer=e.buffer,this.detune=e.detune,this.loop=e.loop,this.loopStart=e.loopStart,this.loopEnd=e.loopEnd,this.offset=e.offset,this.duration=e.duration,this.playbackRate=e.playbackRate,this.hasPlaybackControl=e.hasPlaybackControl,this.sourceType=e.sourceType,this.filters=e.filters.slice(),this):(U(`Audio: Audio source type cannot be copied.`),this)}clone(e){return new this.constructor(this.listener).copy(this,e)}},nd=new q,rd=new gn,id=new q,ad=new q,od=class extends td{constructor(e){super(e),this.panner=this.context.createPanner(),this.panner.panningModel=`HRTF`,this.panner.connect(this.gain)}connect(){return super.connect(),this.panner.connect(this.gain),this}disconnect(){return super.disconnect(),this.panner.disconnect(this.gain),this}getOutput(){return this.panner}getRefDistance(){return this.panner.refDistance}setRefDistance(e){return this.panner.refDistance=e,this}getRolloffFactor(){return this.panner.rolloffFactor}setRolloffFactor(e){return this.panner.rolloffFactor=e,this}getDistanceModel(){return this.panner.distanceModel}setDistanceModel(e){return this.panner.distanceModel=e,this}getMaxDistance(){return this.panner.maxDistance}setMaxDistance(e){return this.panner.maxDistance=e,this}setDirectionalCone(e,t,n){return this.panner.coneInnerAngle=e,this.panner.coneOuterAngle=t,this.panner.coneOuterGain=n,this}updateMatrixWorld(e){if(super.updateMatrixWorld(e),this.hasPlaybackControl===!0&&this.isPlaying===!1)return;this.matrixWorld.decompose(nd,rd,id),ad.set(0,0,1).applyQuaternion(rd);let t=this.panner;if(t.positionX){let e=this.context.currentTime+this.listener.timeDelta;t.positionX.linearRampToValueAtTime(nd.x,e),t.positionY.linearRampToValueAtTime(nd.y,e),t.positionZ.linearRampToValueAtTime(nd.z,e),t.orientationX.linearRampToValueAtTime(ad.x,e),t.orientationY.linearRampToValueAtTime(ad.y,e),t.orientationZ.linearRampToValueAtTime(ad.z,e)}else t.setPosition(nd.x,nd.y,nd.z),t.setOrientation(ad.x,ad.y,ad.z)}},sd=class{constructor(e,t=2048){this.analyser=e.context.createAnalyser(),this.analyser.fftSize=t,this.data=new Uint8Array(this.analyser.frequencyBinCount),e.getOutput().connect(this.analyser)}getFrequencyData(){return this.analyser.getByteFrequencyData(this.data),this.data}getAverageFrequency(){let e=0,t=this.getFrequencyData();for(let n=0;n<t.length;n++)e+=t[n];return e/t.length}},cd=class{constructor(e,t,n){this.binding=e,this.valueSize=n;let r,i,a;switch(t){case`quaternion`:r=this._slerp,i=this._slerpAdditive,a=this._setAdditiveIdentityQuaternion,this.buffer=new Float64Array(n*6),this._workIndex=5;break;case`string`:case`bool`:r=this._select,i=this._select,a=this._setAdditiveIdentityOther,this.buffer=Array(n*5);break;default:r=this._lerp,i=this._lerpAdditive,a=this._setAdditiveIdentityNumeric,this.buffer=new Float64Array(n*5)}this._mixBufferRegion=r,this._mixBufferRegionAdditive=i,this._setIdentity=a,this._origIndex=3,this._addIndex=4,this.cumulativeWeight=0,this.cumulativeWeightAdditive=0,this.useCount=0,this.referenceCount=0}accumulate(e,t){let n=this.buffer,r=this.valueSize,i=e*r+r,a=this.cumulativeWeight;if(a===0){for(let e=0;e!==r;++e)n[i+e]=n[e];a=t}else{a+=t;let e=t/a;this._mixBufferRegion(n,i,0,e,r)}this.cumulativeWeight=a}accumulateAdditive(e){let t=this.buffer,n=this.valueSize,r=n*this._addIndex;this.cumulativeWeightAdditive===0&&this._setIdentity(),this._mixBufferRegionAdditive(t,r,0,e,n),this.cumulativeWeightAdditive+=e}apply(e){let t=this.valueSize,n=this.buffer,r=e*t+t,i=this.cumulativeWeight,a=this.cumulativeWeightAdditive,o=this.binding;if(this.cumulativeWeight=0,this.cumulativeWeightAdditive=0,i<1){let e=t*this._origIndex;this._mixBufferRegion(n,r,e,1-i,t)}a>0&&this._mixBufferRegionAdditive(n,r,this._addIndex*t,1,t);for(let e=t,i=t+t;e!==i;++e)if(n[e]!==n[e+t]){o.setValue(n,r);break}}saveOriginalState(){let e=this.binding,t=this.buffer,n=this.valueSize,r=n*this._origIndex;e.getValue(t,r);for(let e=n,i=r;e!==i;++e)t[e]=t[r+e%n];this._setIdentity(),this.cumulativeWeight=0,this.cumulativeWeightAdditive=0}restoreOriginalState(){let e=this.valueSize*3;this.binding.setValue(this.buffer,e)}_setAdditiveIdentityNumeric(){let e=this._addIndex*this.valueSize,t=e+this.valueSize;for(let n=e;n<t;n++)this.buffer[n]=0}_setAdditiveIdentityQuaternion(){this._setAdditiveIdentityNumeric(),this.buffer[this._addIndex*this.valueSize+3]=1}_setAdditiveIdentityOther(){let e=this._origIndex*this.valueSize,t=this._addIndex*this.valueSize;for(let n=0;n<this.valueSize;n++)this.buffer[t+n]=this.buffer[e+n]}_select(e,t,n,r,i){if(r>=.5)for(let r=0;r!==i;++r)e[t+r]=e[n+r]}_slerp(e,t,n,r){gn.slerpFlat(e,t,e,t,e,n,r)}_slerpAdditive(e,t,n,r,i){let a=this._workIndex*i;gn.multiplyQuaternionsFlat(e,a,e,t,e,n),gn.slerpFlat(e,t,e,t,e,a,r)}_lerp(e,t,n,r,i){let a=1-r;for(let o=0;o!==i;++o){let i=t+o;e[i]=e[i]*a+e[n+o]*r}}_lerpAdditive(e,t,n,r,i){for(let a=0;a!==i;++a){let i=t+a;e[i]=e[i]+e[n+a]*r}}},ld=`\\[\\]\\.:\\/`,ud=RegExp(`[\\[\\]\\.:\\/]`,`g`),dd=`[^\\[\\]\\.:\\/]`,fd=`[^`+ld.replace(`\\.`,``)+`]`,pd=`((?:WC+[\\/:])*)`.replace(`WC`,dd),md=`(WCOD+)?`.replace(`WCOD`,fd),hd=`(?:\\.(WC+)(?:\\[(.+)\\])?)?`.replace(`WC`,dd),gd=`\\.(WC+)(?:\\[(.+)\\])?`.replace(`WC`,dd),_d=RegExp(`^`+pd+md+hd+gd+`$`),vd=[`material`,`materials`,`bones`,`map`],yd=class{constructor(e,t,n){let r=n||bd.parseTrackName(t);this._targetGroup=e,this._bindings=e.subscribe_(t,r)}getValue(e,t){this.bind();let n=this._targetGroup.nCachedObjects_,r=this._bindings[n];r!==void 0&&r.getValue(e,t)}setValue(e,t){let n=this._bindings;for(let r=this._targetGroup.nCachedObjects_,i=n.length;r!==i;++r)n[r].setValue(e,t)}bind(){let e=this._bindings;for(let t=this._targetGroup.nCachedObjects_,n=e.length;t!==n;++t)e[t].bind()}unbind(){let e=this._bindings;for(let t=this._targetGroup.nCachedObjects_,n=e.length;t!==n;++t)e[t].unbind()}},bd=class e{constructor(t,n,r){this.path=n,this.parsedPath=r||e.parseTrackName(n),this.node=e.findNode(t,this.parsedPath.nodeName),this.rootNode=t,this.getValue=this._getValue_unbound,this.setValue=this._setValue_unbound}static create(t,n,r){return t&&t.isAnimationObjectGroup?new e.Composite(t,n,r):new e(t,n,r)}static sanitizeNodeName(e){return e.replace(/\s/g,`_`).replace(ud,``)}static parseTrackName(e){let t=_d.exec(e);if(t===null)throw Error(`THREE.PropertyBinding: Cannot parse trackName: `+e);let n={nodeName:t[2],objectName:t[3],objectIndex:t[4],propertyName:t[5],propertyIndex:t[6]},r=n.nodeName&&n.nodeName.lastIndexOf(`.`);if(r!==void 0&&r!==-1){let e=n.nodeName.substring(r+1);vd.indexOf(e)!==-1&&(n.nodeName=n.nodeName.substring(0,r),n.objectName=e)}if(n.propertyName===null||n.propertyName.length===0)throw Error(`THREE.PropertyBinding: can not parse propertyName from trackName: `+e);return n}static findNode(e,t){if(t===void 0||t===``||t===`.`||t===-1||t===e.name||t===e.uuid)return e;if(e.skeleton){let n=e.skeleton.getBoneByName(t);if(n!==void 0)return n}if(e.children){let n=function(e){for(let r=0;r<e.length;r++){let i=e[r];if(i.name===t||i.uuid===t)return i;let a=n(i.children);if(a)return a}return null},r=n(e.children);if(r)return r}return null}_getValue_unavailable(){}_setValue_unavailable(){}_getValue_direct(e,t){e[t]=this.targetObject[this.propertyName]}_getValue_array(e,t){let n=this.resolvedProperty;for(let r=0,i=n.length;r!==i;++r)e[t++]=n[r]}_getValue_arrayElement(e,t){e[t]=this.resolvedProperty[this.propertyIndex]}_getValue_toArray(e,t){this.resolvedProperty.toArray(e,t)}_setValue_direct(e,t){this.targetObject[this.propertyName]=e[t]}_setValue_direct_setNeedsUpdate(e,t){this.targetObject[this.propertyName]=e[t],this.targetObject.needsUpdate=!0}_setValue_direct_setMatrixWorldNeedsUpdate(e,t){this.targetObject[this.propertyName]=e[t],this.targetObject.matrixWorldNeedsUpdate=!0}_setValue_array(e,t){let n=this.resolvedProperty;for(let r=0,i=n.length;r!==i;++r)n[r]=e[t++]}_setValue_array_setNeedsUpdate(e,t){let n=this.resolvedProperty;for(let r=0,i=n.length;r!==i;++r)n[r]=e[t++];this.targetObject.needsUpdate=!0}_setValue_array_setMatrixWorldNeedsUpdate(e,t){let n=this.resolvedProperty;for(let r=0,i=n.length;r!==i;++r)n[r]=e[t++];this.targetObject.matrixWorldNeedsUpdate=!0}_setValue_arrayElement(e,t){this.resolvedProperty[this.propertyIndex]=e[t]}_setValue_arrayElement_setNeedsUpdate(e,t){this.resolvedProperty[this.propertyIndex]=e[t],this.targetObject.needsUpdate=!0}_setValue_arrayElement_setMatrixWorldNeedsUpdate(e,t){this.resolvedProperty[this.propertyIndex]=e[t],this.targetObject.matrixWorldNeedsUpdate=!0}_setValue_fromArray(e,t){this.resolvedProperty.fromArray(e,t)}_setValue_fromArray_setNeedsUpdate(e,t){this.resolvedProperty.fromArray(e,t),this.targetObject.needsUpdate=!0}_setValue_fromArray_setMatrixWorldNeedsUpdate(e,t){this.resolvedProperty.fromArray(e,t),this.targetObject.matrixWorldNeedsUpdate=!0}_getValue_unbound(e,t){this.bind(),this.getValue(e,t)}_setValue_unbound(e,t){this.bind(),this.setValue(e,t)}bind(){let t=this.node,n=this.parsedPath,r=n.objectName,i=n.propertyName,a=n.propertyIndex;if(t||(t=e.findNode(this.rootNode,n.nodeName),this.node=t),this.getValue=this._getValue_unavailable,this.setValue=this._setValue_unavailable,!t){U(`PropertyBinding: No target node found for track: `+this.path+`.`);return}if(r){let e=n.objectIndex;switch(r){case`materials`:if(!t.material){W(`PropertyBinding: Can not bind to material as node does not have a material.`,this);return}if(!t.material.materials){W(`PropertyBinding: Can not bind to material.materials as node.material does not have a materials array.`,this);return}t=t.material.materials;break;case`bones`:if(!t.skeleton){W(`PropertyBinding: Can not bind to bones as node does not have a skeleton.`,this);return}t=t.skeleton.bones;for(let n=0;n<t.length;n++)if(t[n].name===e){e=n;break}break;case`map`:if(`map`in t){t=t.map;break}if(!t.material){W(`PropertyBinding: Can not bind to material as node does not have a material.`,this);return}if(!t.material.map){W(`PropertyBinding: Can not bind to material.map as node.material does not have a map.`,this);return}t=t.material.map;break;default:if(t[r]===void 0){W(`PropertyBinding: Can not bind to objectName of node undefined.`,this);return}t=t[r]}if(e!==void 0){if(t[e]===void 0){W(`PropertyBinding: Trying to bind to objectIndex of objectName, but is undefined.`,this,t);return}t=t[e]}}let o=t[i];if(o===void 0){let e=n.nodeName;W(`PropertyBinding: Trying to update property for track: `+e+`.`+i+` but it wasn't found.`,t);return}let s=this.Versioning.None;this.targetObject=t,t.isMaterial===!0?s=this.Versioning.NeedsUpdate:t.isObject3D===!0&&(s=this.Versioning.MatrixWorldNeedsUpdate);let c=this.BindingType.Direct;if(a!==void 0){if(i===`morphTargetInfluences`){if(!t.geometry){W(`PropertyBinding: Can not bind to morphTargetInfluences because node does not have a geometry.`,this);return}if(!t.geometry.morphAttributes){W(`PropertyBinding: Can not bind to morphTargetInfluences because node does not have a geometry.morphAttributes.`,this);return}t.morphTargetDictionary[a]!==void 0&&(a=t.morphTargetDictionary[a])}c=this.BindingType.ArrayElement,this.resolvedProperty=o,this.propertyIndex=a}else o.fromArray!==void 0&&o.toArray!==void 0?(c=this.BindingType.HasFromToArray,this.resolvedProperty=o):Array.isArray(o)?(c=this.BindingType.EntireArray,this.resolvedProperty=o):this.propertyName=i;this.getValue=this.GetterByBindingType[c],this.setValue=this.SetterByBindingTypeAndVersioning[c][s]}unbind(){this.node=null,this.getValue=this._getValue_unbound,this.setValue=this._setValue_unbound}};bd.Composite=yd,bd.prototype.BindingType={Direct:0,EntireArray:1,ArrayElement:2,HasFromToArray:3},bd.prototype.Versioning={None:0,NeedsUpdate:1,MatrixWorldNeedsUpdate:2},bd.prototype.GetterByBindingType=[bd.prototype._getValue_direct,bd.prototype._getValue_array,bd.prototype._getValue_arrayElement,bd.prototype._getValue_toArray],bd.prototype.SetterByBindingTypeAndVersioning=[[bd.prototype._setValue_direct,bd.prototype._setValue_direct_setNeedsUpdate,bd.prototype._setValue_direct_setMatrixWorldNeedsUpdate],[bd.prototype._setValue_array,bd.prototype._setValue_array_setNeedsUpdate,bd.prototype._setValue_array_setMatrixWorldNeedsUpdate],[bd.prototype._setValue_arrayElement,bd.prototype._setValue_arrayElement_setNeedsUpdate,bd.prototype._setValue_arrayElement_setMatrixWorldNeedsUpdate],[bd.prototype._setValue_fromArray,bd.prototype._setValue_fromArray_setNeedsUpdate,bd.prototype._setValue_fromArray_setMatrixWorldNeedsUpdate]];var xd=class{constructor(){this.isAnimationObjectGroup=!0,this.uuid=qt(),this._objects=Array.prototype.slice.call(arguments),this.nCachedObjects_=0;let e={};this._indicesByUUID=e;for(let t=0,n=arguments.length;t!==n;++t)e[arguments[t].uuid]=t;this._paths=[],this._parsedPaths=[],this._bindings=[],this._bindingsIndicesByPath={};let t=this;this.stats={objects:{get total(){return t._objects.length},get inUse(){return this.total-t.nCachedObjects_}},get bindingsPerObject(){return t._bindings.length}}}add(){let e=this._objects,t=this._indicesByUUID,n=this._paths,r=this._parsedPaths,i=this._bindings,a=i.length,o,s=e.length,c=this.nCachedObjects_;for(let l=0,u=arguments.length;l!==u;++l){let u=arguments[l],d=u.uuid,f=t[d];if(f===void 0){f=s++,t[d]=f,e.push(u);for(let e=0,t=a;e!==t;++e)i[e].push(new bd(u,n[e],r[e]))}else if(f<c){o=e[f];let s=--c,l=e[s];t[l.uuid]=f,e[f]=l,t[d]=s,e[s]=u;for(let e=0,t=a;e!==t;++e){let t=i[e],a=t[s],o=t[f];t[f]=a,o===void 0&&(o=new bd(u,n[e],r[e])),t[s]=o}}else e[f]!==o&&W(`AnimationObjectGroup: Different objects with the same UUID detected. Clean the caches or recreate your infrastructure when reloading scenes.`)}this.nCachedObjects_=c}remove(){let e=this._objects,t=this._indicesByUUID,n=this._bindings,r=n.length,i=this.nCachedObjects_;for(let a=0,o=arguments.length;a!==o;++a){let o=arguments[a],s=o.uuid,c=t[s];if(c!==void 0&&c>=i){let a=i++,l=e[a];t[l.uuid]=c,e[c]=l,t[s]=a,e[a]=o;for(let e=0,t=r;e!==t;++e){let t=n[e],r=t[a],i=t[c];t[c]=r,t[a]=i}}}this.nCachedObjects_=i}uncache(){let e=this._objects,t=this._indicesByUUID,n=this._bindings,r=n.length,i=this.nCachedObjects_,a=e.length;for(let o=0,s=arguments.length;o!==s;++o){let s=arguments[o].uuid,c=t[s];if(c!==void 0){if(delete t[s],c<i){let o=--i,s=e[o],l=--a,u=e[l];c!==o&&(t[s.uuid]=c),e[c]=s,o!==l&&(t[u.uuid]=o),e[o]=u,e.pop();for(let e=0,t=r;e!==t;++e){let t=n[e],r=t[o],i=t[l];t[c]=r,t[o]=i,t.pop()}}else{let i=--a,o=e[i];c!==i&&(t[o.uuid]=c),e[c]=o,e.pop();for(let e=0,t=r;e!==t;++e){let t=n[e];t[c]=t[i],t.pop()}}}}this.nCachedObjects_=i}subscribe_(e,t){let n=this._bindingsIndicesByPath,r=n[e],i=this._bindings;if(r!==void 0)return i[r];let a=this._paths,o=this._parsedPaths,s=this._objects,c=s.length,l=this.nCachedObjects_,u=Array(c);r=i.length,n[e]=r,a.push(e),o.push(t),i.push(u);for(let n=l,r=s.length;n!==r;++n){let r=s[n];u[n]=new bd(r,e,t)}return u}unsubscribe_(e){let t=this._bindingsIndicesByPath,n=t[e];if(n!==void 0){let e=this._paths,r=this._parsedPaths,i=this._bindings,a=i.length-1,o=i[a],s=e[a];t[s]=n,i[n]=o,i.pop(),r[n]=r[a],r.pop(),e[n]=e[a],e.pop()}}},Sd=class{constructor(e,t,n=null,r=t.blendMode){this._mixer=e,this._clip=t,this._localRoot=n,this.blendMode=r;let i=t.tracks,a=i.length,o=Array(a),s={endingStart:We,endingEnd:We};for(let e=0;e!==a;++e){let t=i[e].createInterpolant(null);o[e]=t,t.settings=s}this._interpolantSettings=s,this._interpolants=o,this._propertyBindings=Array(a),this._cacheIndex=null,this._byClipCacheIndex=null,this._timeScaleInterpolant=null,this._restoreTimeScale=null,this._weightInterpolant=null,this.loop=Re,this._loopCount=-1,this._startTime=null,this.time=0,this.timeScale=1,this._effectiveTimeScale=1,this.weight=1,this._effectiveWeight=1,this.repetitions=1/0,this.paused=!1,this.enabled=!0,this.clampWhenFinished=!1,this.zeroSlopeAtStart=!0,this.zeroSlopeAtEnd=!0}play(){return this._mixer._activateAction(this),this}stop(){return this._mixer._deactivateAction(this),this.reset()}reset(){return this.paused=!1,this.enabled=!0,this.time=0,this._loopCount=-1,this._startTime=null,this.stopFading().stopWarping()}isRunning(){return this.enabled&&!this.paused&&this.timeScale!==0&&this._startTime===null&&this._mixer._isActiveAction(this)}isScheduled(){return this._mixer._isActiveAction(this)}startAt(e){return this._startTime=e,this}setLoop(e,t){return this.loop=e,this.repetitions=t,this}setEffectiveWeight(e){return this.weight=e,this._effectiveWeight=this.enabled?e:0,this.stopFading()}getEffectiveWeight(){return this._effectiveWeight}fadeIn(e){return this._scheduleFading(e,0,1)}fadeOut(e){return this._scheduleFading(e,1,0)}crossFadeFrom(e,t,n=!1){if(e.fadeOut(t),this.fadeIn(t),n===!0){let n=this._clip.duration,r=e._clip.duration,i=r/n,a=n/r;e._restoreTimeScale=e.timeScale,this._restoreTimeScale=this.timeScale,e.warp(1,i,t),this.warp(a,1,t)}return this}crossFadeTo(e,t,n=!1){return e.crossFadeFrom(this,t,n)}stopFading(){let e=this._weightInterpolant;return e!==null&&(this._weightInterpolant=null,this._mixer._takeBackControlInterpolant(e)),this}setEffectiveTimeScale(e){return this.timeScale=e,this._effectiveTimeScale=this.paused?0:e,this.stopWarping()}getEffectiveTimeScale(){return this._effectiveTimeScale}setDuration(e){return this.timeScale=this._clip.duration/e,this.stopWarping()}syncWith(e){return this.time=e.time,this.timeScale=e.timeScale,this.stopWarping()}halt(e){return this.warp(this._effectiveTimeScale,0,e)}warp(e,t,n){let r=this._mixer,i=r.time,a=this.timeScale,o=this._timeScaleInterpolant;o===null&&(o=r._lendControlInterpolant(),this._timeScaleInterpolant=o);let s=o.parameterPositions,c=o.sampleValues;return s[0]=i,s[1]=i+n,c[0]=e/a,c[1]=t/a,this}stopWarping(){let e=this._timeScaleInterpolant;return e!==null&&(this._timeScaleInterpolant=null,this._mixer._takeBackControlInterpolant(e)),this._restoreTimeScale=null,this}getMixer(){return this._mixer}getClip(){return this._clip}getRoot(){return this._localRoot||this._mixer._root}_update(e,t,n,r){if(!this.enabled){this._updateWeight(e);return}let i=this._startTime;if(i!==null){let r=(e-i)*n;r<0||n===0?t=0:(this._startTime=null,t=n*r)}t*=this._updateTimeScale(e);let a=this._updateTime(t),o=this._updateWeight(e);if(o>0){let e=this._interpolants,t=this._propertyBindings;switch(this.blendMode){case Je:for(let n=0,r=e.length;n!==r;++n)e[n].evaluate(a),t[n].accumulateAdditive(o);break;case qe:default:for(let n=0,i=e.length;n!==i;++n)e[n].evaluate(a),t[n].accumulate(r,o)}}}_updateWeight(e){let t=0;if(this.enabled){t=this.weight;let n=this._weightInterpolant;if(n!==null){let r=n.evaluate(e)[0];t*=r,e>n.parameterPositions[1]&&(this.stopFading(),r===0&&(this.enabled=!1))}}return this._effectiveWeight=t,t}_updateTimeScale(e){let t=0;if(!this.paused){t=this.timeScale;let n=this._timeScaleInterpolant;if(n!==null){let r=n.evaluate(e)[0];t*=r,e>n.parameterPositions[1]&&(t===0?this.paused=!0:(this._restoreTimeScale!==null&&(t=this._restoreTimeScale),this.timeScale=t),this.stopWarping())}}return this._effectiveTimeScale=t,t}_updateTime(e){let t=this._clip.duration,n=this.loop,r=this.time+e,i=this._loopCount,a=n===ze;if(e===0)return i===-1?r:a&&(i&1)==1?t-r:r;if(n===2200){i===-1&&(this._loopCount=0,this._setEndings(!0,!0,!1));handle_stop:{if(r>=t)r=t;else if(r<0)r=0;else{this.time=r;break handle_stop}this.clampWhenFinished?this.paused=!0:this.enabled=!1,this.time=r,this._mixer.dispatchEvent({type:`finished`,action:this,direction:e<0?-1:1})}}else{if(i===-1&&(e>=0?(i=0,this._setEndings(!0,this.repetitions===0,a)):this._setEndings(this.repetitions===0,!0,a)),r>=t||r<0){let n=Math.floor(r/t);r-=t*n,i+=Math.abs(n);let o=this.repetitions-i;if(o<=0)this.clampWhenFinished?this.paused=!0:this.enabled=!1,r=e>0?t:0,this.time=r,this._mixer.dispatchEvent({type:`finished`,action:this,direction:e>0?1:-1});else{if(o===1){let t=e<0;this._setEndings(t,!t,a)}else this._setEndings(!1,!1,a);this._loopCount=i,this.time=r,this._mixer.dispatchEvent({type:`loop`,action:this,loopDelta:n})}}else this._loopCount=i,this.time=r;if(a&&(i&1)==1)return t-r}return r}_setEndings(e,t,n){let r=this._interpolantSettings;n?(r.endingStart=Ge,r.endingEnd=Ge):(r.endingStart=e?this.zeroSlopeAtStart?Ge:We:Ke,r.endingEnd=t?this.zeroSlopeAtEnd?Ge:We:Ke)}_scheduleFading(e,t,n){let r=this._mixer,i=r.time,a=this._weightInterpolant;a===null&&(a=r._lendControlInterpolant(),this._weightInterpolant=a);let o=a.parameterPositions,s=a.sampleValues;return o[0]=i,s[0]=t,o[1]=i+e,s[1]=n,this}},Cd=new Float32Array(1),wd=class extends Ht{constructor(e){super(),this._root=e,this._initMemoryManager(),this._accuIndex=0,this.time=0,this.timeScale=1,typeof __THREE_DEVTOOLS__<`u`&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent(`observe`,{detail:this}))}_bindAction(e,t){let n=e._localRoot||this._root,r=e._clip.tracks,i=r.length,a=e._propertyBindings,o=e._interpolants,s=n.uuid,c=this._bindingsByRootAndName,l=c[s];l===void 0&&(l={},c[s]=l);for(let e=0;e!==i;++e){let i=r[e],c=i.name,u=l[c];if(u!==void 0)++u.referenceCount,a[e]=u;else{if(u=a[e],u!==void 0){u._cacheIndex===null&&(++u.referenceCount,this._addInactiveBinding(u,s,c));continue}let r=t&&t._propertyBindings[e].binding.parsedPath;u=new cd(bd.create(n,c,r),i.ValueTypeName,i.getValueSize()),++u.referenceCount,this._addInactiveBinding(u,s,c),a[e]=u}o[e].resultBuffer=u.buffer}}_activateAction(e){if(!this._isActiveAction(e)){if(e._cacheIndex===null){let t=(e._localRoot||this._root).uuid,n=e._clip.uuid,r=this._actionsByClip[n];this._bindAction(e,r&&r.knownActions[0]),this._addInactiveAction(e,n,t)}let t=e._propertyBindings;for(let e=0,n=t.length;e!==n;++e){let n=t[e];n.useCount++===0&&(this._lendBinding(n),n.saveOriginalState())}this._lendAction(e)}}_deactivateAction(e){if(this._isActiveAction(e)){let t=e._propertyBindings;for(let e=0,n=t.length;e!==n;++e){let n=t[e];--n.useCount===0&&(n.restoreOriginalState(),this._takeBackBinding(n))}this._takeBackAction(e)}}_initMemoryManager(){this._actions=[],this._nActiveActions=0,this._actionsByClip={},this._bindings=[],this._nActiveBindings=0,this._bindingsByRootAndName={},this._controlInterpolants=[],this._nActiveControlInterpolants=0;let e=this;this.stats={actions:{get total(){return e._actions.length},get inUse(){return e._nActiveActions}},bindings:{get total(){return e._bindings.length},get inUse(){return e._nActiveBindings}},controlInterpolants:{get total(){return e._controlInterpolants.length},get inUse(){return e._nActiveControlInterpolants}}}}_isActiveAction(e){let t=e._cacheIndex;return t!==null&&t<this._nActiveActions}_addInactiveAction(e,t,n){let r=this._actions,i=this._actionsByClip,a=i[t];if(a===void 0)a={knownActions:[e],actionByRoot:{}},e._byClipCacheIndex=0,i[t]=a;else{let t=a.knownActions;e._byClipCacheIndex=t.length,t.push(e)}e._cacheIndex=r.length,r.push(e),a.actionByRoot[n]=e}_removeInactiveAction(e){let t=this._actions,n=t[t.length-1],r=e._cacheIndex;n._cacheIndex=r,t[r]=n,t.pop(),e._cacheIndex=null;let i=e._clip.uuid,a=this._actionsByClip,o=a[i],s=o.knownActions,c=s[s.length-1],l=e._byClipCacheIndex;c._byClipCacheIndex=l,s[l]=c,s.pop(),e._byClipCacheIndex=null;let u=o.actionByRoot,d=(e._localRoot||this._root).uuid;delete u[d],s.length===0&&delete a[i],this._removeInactiveBindingsForAction(e)}_removeInactiveBindingsForAction(e){let t=e._propertyBindings;for(let e=0,n=t.length;e!==n;++e){let n=t[e];--n.referenceCount===0&&this._removeInactiveBinding(n)}}_lendAction(e){let t=this._actions,n=e._cacheIndex,r=this._nActiveActions++,i=t[r];e._cacheIndex=r,t[r]=e,i._cacheIndex=n,t[n]=i}_takeBackAction(e){let t=this._actions,n=e._cacheIndex,r=--this._nActiveActions,i=t[r];e._cacheIndex=r,t[r]=e,i._cacheIndex=n,t[n]=i}_addInactiveBinding(e,t,n){let r=this._bindingsByRootAndName,i=this._bindings,a=r[t];a===void 0&&(a={},r[t]=a),a[n]=e,e._cacheIndex=i.length,i.push(e)}_removeInactiveBinding(e){let t=this._bindings,n=e.binding,r=n.rootNode.uuid,i=n.path,a=this._bindingsByRootAndName,o=a[r],s=t[t.length-1],c=e._cacheIndex;s._cacheIndex=c,t[c]=s,t.pop(),delete o[i],Object.keys(o).length===0&&delete a[r]}_lendBinding(e){let t=this._bindings,n=e._cacheIndex,r=this._nActiveBindings++,i=t[r];e._cacheIndex=r,t[r]=e,i._cacheIndex=n,t[n]=i}_takeBackBinding(e){let t=this._bindings,n=e._cacheIndex,r=--this._nActiveBindings,i=t[r];e._cacheIndex=r,t[r]=e,i._cacheIndex=n,t[n]=i}_lendControlInterpolant(){let e=this._controlInterpolants,t=this._nActiveControlInterpolants++,n=e[t];return n===void 0&&(n=new bl(new Float32Array(2),new Float32Array(2),1,Cd),n.__cacheIndex=t,e[t]=n),n}_takeBackControlInterpolant(e){let t=this._controlInterpolants,n=e.__cacheIndex,r=--this._nActiveControlInterpolants,i=t[r];e.__cacheIndex=r,t[r]=e,i.__cacheIndex=n,t[n]=i}clipAction(e,t,n){let r=t||this._root,i=r.uuid,a=typeof e==`string`?Fl.findByName(r,e):e,o=a===null?e:a.uuid,s=this._actionsByClip[o],c=null;if(n===void 0&&(n=a===null?qe:a.blendMode),s!==void 0){let e=s.actionByRoot[i];if(e!==void 0&&e.blendMode===n)return e;c=s.knownActions[0],a===null&&(a=c._clip)}if(a===null)return null;let l=new Sd(this,a,t,n);return this._bindAction(l,c),this._addInactiveAction(l,o,i),l}existingAction(e,t){let n=t||this._root,r=n.uuid,i=typeof e==`string`?Fl.findByName(n,e):e,a=i?i.uuid:e,o=this._actionsByClip[a];return o===void 0?null:o.actionByRoot[r]||null}stopAllAction(){let e=this._actions,t=this._nActiveActions;for(let n=t-1;n>=0;--n)e[n].stop();return this}update(e){e*=this.timeScale;let t=this._actions,n=this._nActiveActions,r=this.time+=e,i=Math.sign(e),a=this._accuIndex^=1;for(let o=0;o!==n;++o)t[o]._update(r,e,i,a);let o=this._bindings,s=this._nActiveBindings;for(let e=0;e!==s;++e)o[e].apply(a);return this}setTime(e){this.time=0;for(let e=0;e<this._actions.length;e++)this._actions[e].time=0;return this.update(e)}getRoot(){return this._root}uncacheClip(e){let t=this._actions,n=e.uuid,r=this._actionsByClip,i=r[n];if(i!==void 0){let e=i.knownActions;for(let n=0,r=e.length;n!==r;++n){let r=e[n];this._deactivateAction(r);let i=r._cacheIndex,a=t[t.length-1];r._cacheIndex=null,r._byClipCacheIndex=null,a._cacheIndex=i,t[i]=a,t.pop(),this._removeInactiveBindingsForAction(r)}delete r[n]}}uncacheRoot(e){let t=e.uuid,n=this._actionsByClip;for(let e in n){let r=n[e].actionByRoot[t];r!==void 0&&(this._deactivateAction(r),this._removeInactiveAction(r))}let r=this._bindingsByRootAndName[t];if(r!==void 0)for(let e in r){let t=r[e];t.restoreOriginalState(),this._removeInactiveBinding(t)}}uncacheAction(e,t){let n=this.existingAction(e,t);n!==null&&(this._deactivateAction(n),this._removeInactiveAction(n))}},Td=class extends Ln{constructor(e=1,t=1,n=1,r={}){super(e,t,r),this.isRenderTarget3D=!0,this.depth=n;for(let r=0;r<this.textures.length;r++){let i=new Vn(null,e,t,n);i.isRenderTargetTexture=!0,i.renderTarget=this,this.textures[r]=i}this._setTextureOptions(r)}},Ed=class e{constructor(e){this.value=e}clone(){return new e(this.value.clone===void 0?this.value:this.value.clone())}},Dd=0,Od=class extends Ht{constructor(){super(),this.isUniformsGroup=!0,Object.defineProperty(this,"id",{value:Dd++}),this.name=``,this.usage=ut,this.uniforms=[]}add(e){return this.uniforms.push(e),this}remove(e){let t=this.uniforms.indexOf(e);return t!==-1&&this.uniforms.splice(t,1),this}setName(e){return this.name=e,this}setUsage(e){return this.usage=e,this}dispose(){this.dispatchEvent({type:`dispose`})}copy(e){this.name=e.name,this.usage=e.usage;let t=e.uniforms;this.uniforms.length=0;for(let e=0,n=t.length;e<n;e++){let n=Array.isArray(t[e])?t[e]:[t[e]];for(let e=0;e<n.length;e++)this.uniforms.push(n[e].clone())}return this}clone(){return new this.constructor().copy(this)}},kd=class extends Ai{constructor(e,t,n=1){super(e,t),this.isInstancedInterleavedBuffer=!0,this.meshPerAttribute=n}copy(e){return super.copy(e),this.meshPerAttribute=e.meshPerAttribute,this}clone(e){let t=super.clone(e);return t.meshPerAttribute=this.meshPerAttribute,t}toJSON(e){let t=super.toJSON(e);return t.isInstancedInterleavedBuffer=!0,t.meshPerAttribute=this.meshPerAttribute,t}},Ad=class{constructor(e,t,n,r,i,a=!1){this.isGLBufferAttribute=!0,this.name=``,this.buffer=e,this.type=t,this.itemSize=n,this.elementSize=r,this.count=i,this.normalized=a,this.version=0}set needsUpdate(e){e===!0&&this.version++}setBuffer(e){return this.buffer=e,this}setType(e,t){return this.type=e,this.elementSize=t,this}setItemSize(e){return this.itemSize=e,this}setCount(e){return this.count=e,this}},jd=new J,Md=class{constructor(e,t,n=0,r=1/0){this.ray=new ca(e,t),this.near=n,this.far=r,this.camera=null,this.layers=new $n,this.params={Mesh:{},Line:{threshold:1},LOD:{},Points:{threshold:1},Sprite:{}}}set(e,t){this.ray.set(e,t)}setFromCamera(e,t){t.isPerspectiveCamera?(this.ray.origin.setFromMatrixPosition(t.matrixWorld),this.ray.direction.set(e.x,e.y,.5).unproject(t).sub(this.ray.origin).normalize(),this.camera=t):t.isOrthographicCamera?(this.ray.origin.set(e.x,e.y,t.projectionMatrix.elements[14]).unproject(t),this.ray.direction.set(0,0,-1).transformDirection(t.matrixWorld),this.camera=t):W(`Raycaster: Unsupported camera type: `+t.type)}setFromXRController(e){return jd.identity().extractRotation(e.matrixWorld),this.ray.origin.setFromMatrixPosition(e.matrixWorld),this.ray.direction.set(0,0,-1).applyMatrix4(jd),this}intersectObject(e,t=!0,n=[]){return Pd(e,this,n,t),n.sort(Nd),n}intersectObjects(e,t=!0,n=[]){for(let r=0,i=e.length;r<i;r++)Pd(e[r],this,n,t);return n.sort(Nd),n}};function Nd(e,t){return e.distance-t.distance}function Pd(e,t,n,r){let i=!0;if(e.layers.test(t.layers)&&e.raycast(t,n)===!1&&(i=!1),i===!0&&r===!0){let r=e.children;for(let e=0,i=r.length;e<i;e++)Pd(r[e],t,n,!0)}}var Fd=class{constructor(e=!0){this.autoStart=e,this.startTime=0,this.oldTime=0,this.elapsedTime=0,this.running=!1,U(`Clock: This module has been deprecated. Please use THREE.Timer instead.`)}start(){this.startTime=performance.now(),this.oldTime=this.startTime,this.elapsedTime=0,this.running=!0}stop(){this.getElapsedTime(),this.running=!1,this.autoStart=!1}getElapsedTime(){return this.getDelta(),this.elapsedTime}getDelta(){let e=0;if(this.autoStart&&!this.running)return this.start(),0;if(this.running){let t=performance.now();e=(t-this.oldTime)/1e3,this.oldTime=t,this.elapsedTime+=e}return e}},Id=class{constructor(e=1,t=0,n=0){this.radius=e,this.phi=t,this.theta=n}set(e,t,n){return this.radius=e,this.phi=t,this.theta=n,this}copy(e){return this.radius=e.radius,this.phi=e.phi,this.theta=e.theta,this}makeSafe(){let e=1e-6;return this.phi=G(this.phi,e,Math.PI-e),this}setFromVector3(e){return this.setFromCartesianCoords(e.x,e.y,e.z)}setFromCartesianCoords(e,t,n){return this.radius=Math.sqrt(e*e+t*t+n*n),this.radius===0?(this.theta=0,this.phi=0):(this.theta=Math.atan2(e,n),this.phi=Math.acos(G(t/this.radius,-1,1))),this}clone(){return new this.constructor().copy(this)}},Ld=class{constructor(e=1,t=0,n=0){this.radius=e,this.theta=t,this.y=n}set(e,t,n){return this.radius=e,this.theta=t,this.y=n,this}copy(e){return this.radius=e.radius,this.theta=e.theta,this.y=e.y,this}setFromVector3(e){return this.setFromCartesianCoords(e.x,e.y,e.z)}setFromCartesianCoords(e,t,n){return this.radius=Math.sqrt(e*e+n*n),this.theta=Math.atan2(e,n),this.y=t,this}clone(){return new this.constructor().copy(this)}},Rd=class e{static{e.prototype.isMatrix2=!0}constructor(e,t,n,r){this.elements=[1,0,0,1],e!==void 0&&this.set(e,t,n,r)}identity(){return this.set(1,0,0,1),this}fromArray(e,t=0){for(let n=0;n<4;n++)this.elements[n]=e[n+t];return this}set(e,t,n,r){let i=this.elements;return i[0]=e,i[2]=t,i[1]=n,i[3]=r,this}},zd=new K,Bd=class{constructor(e=new K(1/0,1/0),t=new K(-1/0,-1/0)){this.isBox2=!0,this.min=e,this.max=t}set(e,t){return this.min.copy(e),this.max.copy(t),this}setFromPoints(e){this.makeEmpty();for(let t=0,n=e.length;t<n;t++)this.expandByPoint(e[t]);return this}setFromCenterAndSize(e,t){let n=zd.copy(t).multiplyScalar(.5);return this.min.copy(e).sub(n),this.max.copy(e).add(n),this}clone(){return new this.constructor().copy(this)}copy(e){return this.min.copy(e.min),this.max.copy(e.max),this}makeEmpty(){return this.min.x=this.min.y=1/0,this.max.x=this.max.y=-1/0,this}isEmpty(){return this.max.x<this.min.x||this.max.y<this.min.y}getCenter(e){return this.isEmpty()?e.set(0,0):e.addVectors(this.min,this.max).multiplyScalar(.5)}getSize(e){return this.isEmpty()?e.set(0,0):e.subVectors(this.max,this.min)}expandByPoint(e){return this.min.min(e),this.max.max(e),this}expandByVector(e){return this.min.sub(e),this.max.add(e),this}expandByScalar(e){return this.min.addScalar(-e),this.max.addScalar(e),this}containsPoint(e){return e.x>=this.min.x&&e.x<=this.max.x&&e.y>=this.min.y&&e.y<=this.max.y}containsBox(e){return this.min.x<=e.min.x&&e.max.x<=this.max.x&&this.min.y<=e.min.y&&e.max.y<=this.max.y}getParameter(e,t){return t.set((e.x-this.min.x)/(this.max.x-this.min.x),(e.y-this.min.y)/(this.max.y-this.min.y))}intersectsBox(e){return e.max.x>=this.min.x&&e.min.x<=this.max.x&&e.max.y>=this.min.y&&e.min.y<=this.max.y}clampPoint(e,t){return t.copy(e).clamp(this.min,this.max)}distanceToPoint(e){return this.clampPoint(e,zd).distanceTo(e)}intersect(e){return this.min.max(e.min),this.max.min(e.max),this.isEmpty()&&this.makeEmpty(),this}union(e){return this.min.min(e.min),this.max.max(e.max),this}translate(e){return this.min.add(e),this.max.add(e),this}equals(e){return e.min.equals(this.min)&&e.max.equals(this.max)}},Vd=new q,Hd=new q,Ud=new q,Wd=new q,Gd=new q,Kd=new q,qd=new q,Jd=class{constructor(e=new q,t=new q){this.start=e,this.end=t}set(e,t){return this.start.copy(e),this.end.copy(t),this}copy(e){return this.start.copy(e.start),this.end.copy(e.end),this}getCenter(e){return e.addVectors(this.start,this.end).multiplyScalar(.5)}delta(e){return e.subVectors(this.end,this.start)}distanceSq(){return this.start.distanceToSquared(this.end)}distance(){return this.start.distanceTo(this.end)}at(e,t){return this.delta(t).multiplyScalar(e).add(this.start)}closestPointToPointParameter(e,t){Vd.subVectors(e,this.start),Hd.subVectors(this.end,this.start);let n=Hd.dot(Hd);if(n===0)return 0;let r=Hd.dot(Vd)/n;return t&&(r=G(r,0,1)),r}closestPointToPoint(e,t,n){let r=this.closestPointToPointParameter(e,t);return this.delta(n).multiplyScalar(r).add(this.start)}distanceSqToLine3(e,t=Kd,n=qd){let r=1e-8*1e-8,i,a,o=this.start,s=e.start,c=this.end,l=e.end;Ud.subVectors(c,o),Wd.subVectors(l,s),Gd.subVectors(o,s);let u=Ud.dot(Ud),d=Wd.dot(Wd),f=Wd.dot(Gd);if(u<=r&&d<=r)return t.copy(o),n.copy(s),t.sub(n),t.dot(t);if(u<=r)i=0,a=f/d,a=G(a,0,1);else{let e=Ud.dot(Gd);if(d<=r)a=0,i=G(-e/u,0,1);else{let t=Ud.dot(Wd),n=u*d-t*t;i=n===0?0:G((t*f-e*d)/n,0,1),a=(t*i+f)/d,a<0?(a=0,i=G(-e/u,0,1)):a>1&&(a=1,i=G((t-e)/u,0,1))}}return t.copy(o).addScaledVector(Ud,i),n.copy(s).addScaledVector(Wd,a),t.distanceToSquared(n)}applyMatrix4(e){return this.start.applyMatrix4(e),this.end.applyMatrix4(e),this}equals(e){return e.start.equals(this.start)&&e.end.equals(this.end)}clone(){return new this.constructor().copy(this)}},Yd=new q,Xd=class extends hr{constructor(e,t){super(),this.light=e,this.matrixAutoUpdate=!1,this.color=t,this.type=`SpotLightHelper`;let n=new Z,r=[0,0,0,0,0,1,0,0,0,1,0,1,0,0,0,-1,0,1,0,0,0,0,1,1,0,0,0,0,-1,1];for(let e=0,t=1;e<32;e++,t++){let n=e/32*Math.PI*2,i=t/32*Math.PI*2;r.push(Math.cos(n),Math.sin(n),1,Math.cos(i),Math.sin(i),1)}n.setAttribute(`position`,new X(r,3));let i=new bo({fog:!1,toneMapped:!1});this.cone=new Mo(n,i),this.add(this.cone),this.update()}dispose(){super.dispose(),this.cone.geometry.dispose(),this.cone.material.dispose()}update(){this.light.updateWorldMatrix(!0,!1),this.light.target.updateWorldMatrix(!0,!1),this.parent?(this.parent.updateWorldMatrix(!0),this.matrix.copy(this.parent.matrixWorld).invert().multiply(this.light.matrixWorld)):this.matrix.copy(this.light.matrixWorld),this.matrixWorldNeedsUpdate=!0;let e=this.light.distance?this.light.distance:1e3,t=e*Math.tan(this.light.angle);this.cone.scale.set(t,t,e),Yd.setFromMatrixPosition(this.light.target.matrixWorld),this.cone.lookAt(Yd),this.color===void 0?this.cone.material.color.copy(this.light.color):this.cone.material.color.set(this.color)}},Zd=new q,Qd=new J,$d=new J,ef=class extends Mo{constructor(e){let t=tf(e),n=new Z,r=[],i=[];for(let e=0;e<t.length;e++){let n=t[e];n.parent&&n.parent.isBone&&(r.push(0,0,0),r.push(0,0,0),i.push(0,0,0),i.push(0,0,0))}n.setAttribute(`position`,new X(r,3)),n.setAttribute(`color`,new X(i,3));let a=new bo({vertexColors:!0,depthTest:!1,depthWrite:!1,toneMapped:!1,transparent:!0});super(n,a),this.isSkeletonHelper=!0,this.type=`SkeletonHelper`,this.root=e,this.bones=t,this.matrix=e.matrixWorld,this.matrixAutoUpdate=!1;let o=new Y(255),s=new Y(65280);this.setColors(o,s)}updateMatrixWorld(e){let t=this.bones,n=this.geometry,r=n.getAttribute(`position`);$d.copy(this.root.matrixWorld).invert();for(let e=0,n=0;e<t.length;e++){let i=t[e];i.parent&&i.parent.isBone&&(Qd.multiplyMatrices($d,i.matrixWorld),Zd.setFromMatrixPosition(Qd),r.setXYZ(n,Zd.x,Zd.y,Zd.z),Qd.multiplyMatrices($d,i.parent.matrixWorld),Zd.setFromMatrixPosition(Qd),r.setXYZ(n+1,Zd.x,Zd.y,Zd.z),n+=2)}n.getAttribute(`position`).needsUpdate=!0,super.updateMatrixWorld(e)}setColors(e,t){let n=this.geometry.getAttribute(`color`);for(let r=0;r<n.count;r+=2)n.setXYZ(r,e.r,e.g,e.b),n.setXYZ(r+1,t.r,t.g,t.b);return n.needsUpdate=!0,this}dispose(){super.dispose(),this.geometry.dispose(),this.material.dispose()}};function tf(e){let t=[];e.isBone===!0&&t.push(e);for(let n=0;n<e.children.length;n++)t.push(...tf(e.children[n]));return t}var nf=class extends xa{constructor(e,t,n){let r=new Fc(t,4,2),i=new la({wireframe:!0,fog:!1,toneMapped:!1});super(r,i),this.light=e,this.color=n,this.type=`PointLightHelper`,this.matrix=this.light.matrixWorld,this.matrixAutoUpdate=!1,this.update()}dispose(){super.dispose(),this.geometry.dispose(),this.material.dispose()}update(){this.matrixWorldNeedsUpdate=!0,this.light.updateWorldMatrix(!0,!1),this.color===void 0?this.material.color.copy(this.light.color):this.material.color.set(this.color)}},rf=new q,af=new Y,of=new Y,sf=class extends hr{constructor(e,t,n){super(),this.light=e,this.matrix=e.matrixWorld,this.matrixAutoUpdate=!1,this.color=n,this.type=`HemisphereLightHelper`;let r=new Ac(t);r.rotateY(Math.PI*.5),this.material=new la({wireframe:!0,fog:!1,toneMapped:!1}),this.color===void 0&&(this.material.vertexColors=!0);let i=r.getAttribute(`position`),a=new Float32Array(i.count*3);r.setAttribute(`color`,new ui(a,3)),this.add(new xa(r,this.material)),this.update()}dispose(){super.dispose(),this.children[0].geometry.dispose(),this.children[0].material.dispose()}update(){let e=this.children[0];if(this.color!==void 0)this.material.color.set(this.color);else{let t=e.geometry.getAttribute(`color`);af.copy(this.light.color),of.copy(this.light.groundColor);for(let e=0,n=t.count;e<n;e++){let r=e<n/2?af:of;t.setXYZ(e,r.r,r.g,r.b)}t.needsUpdate=!0}this.matrixWorldNeedsUpdate=!0,this.light.updateWorldMatrix(!0,!1),e.lookAt(rf.setFromMatrixPosition(this.light.matrixWorld).negate())}},cf=class extends Mo{constructor(e=10,t=10,n=4473924,r=8947848){n=new Y(n),r=new Y(r);let i=t/2,a=e/t,o=e/2,s=[],c=[];for(let e=0,l=0,u=-o;e<=t;e++,u+=a){s.push(-o,0,u,o,0,u),s.push(u,0,-o,u,0,o);let t=e===i?n:r;t.toArray(c,l),l+=3,t.toArray(c,l),l+=3,t.toArray(c,l),l+=3,t.toArray(c,l),l+=3}let l=new Z;l.setAttribute(`position`,new X(s,3)),l.setAttribute(`color`,new X(c,3));let u=new bo({vertexColors:!0,toneMapped:!1});super(l,u),this.type=`GridHelper`}dispose(){super.dispose(),this.geometry.dispose(),this.material.dispose()}},lf=class extends Mo{constructor(e=10,t=16,n=8,r=64,i=4473924,a=8947848){i=new Y(i),a=new Y(a);let o=[],s=[];if(t>1)for(let n=0;n<t;n++){let r=n/t*(Math.PI*2),c=Math.sin(r)*e,l=Math.cos(r)*e;o.push(0,0,0),o.push(c,0,l);let u=n&1?i:a;s.push(u.r,u.g,u.b),s.push(u.r,u.g,u.b)}for(let t=0;t<n;t++){let c=t&1?i:a,l=e-e/n*t;for(let e=0;e<r;e++){let t=e/r*(Math.PI*2),n=Math.sin(t)*l,i=Math.cos(t)*l;o.push(n,0,i),s.push(c.r,c.g,c.b),t=(e+1)/r*(Math.PI*2),n=Math.sin(t)*l,i=Math.cos(t)*l,o.push(n,0,i),s.push(c.r,c.g,c.b)}}let c=new Z;c.setAttribute(`position`,new X(o,3)),c.setAttribute(`color`,new X(s,3));let l=new bo({vertexColors:!0,toneMapped:!1});super(c,l),this.type=`PolarGridHelper`}dispose(){super.dispose(),this.geometry.dispose(),this.material.dispose()}},uf=new q,df=new q,ff=new q,pf=class extends hr{constructor(e,t,n){super(),this.light=e,this.matrix=e.matrixWorld,this.matrixAutoUpdate=!1,this.color=n,this.type=`DirectionalLightHelper`,t===void 0&&(t=1);let r=new Z;r.setAttribute(`position`,new X([-t,t,0,t,t,0,t,-t,0,-t,-t,0,-t,t,0],3));let i=new bo({fog:!1,toneMapped:!1});this.lightPlane=new Oo(r,i),this.add(this.lightPlane),r=new Z,r.setAttribute(`position`,new X([0,0,0,0,0,1],3)),this.targetLine=new Oo(r,i),this.add(this.targetLine),this.update()}dispose(){super.dispose(),this.lightPlane.geometry.dispose(),this.lightPlane.material.dispose(),this.targetLine.geometry.dispose(),this.targetLine.material.dispose()}update(){this.matrixWorldNeedsUpdate=!0,this.light.updateWorldMatrix(!0,!1),this.light.target.updateWorldMatrix(!0,!1),uf.setFromMatrixPosition(this.light.matrixWorld),df.setFromMatrixPosition(this.light.target.matrixWorld),ff.subVectors(df,uf),this.lightPlane.lookAt(df),this.color===void 0?(this.lightPlane.material.color.copy(this.light.color),this.targetLine.material.color.copy(this.light.color)):(this.lightPlane.material.color.set(this.color),this.targetLine.material.color.set(this.color)),this.targetLine.lookAt(df),this.targetLine.scale.z=ff.length()}},mf=new q,hf=new cu,gf=class extends Mo{constructor(e){let t=new Z,n=new bo({color:16777215,vertexColors:!0,toneMapped:!1}),r=[],i=[],a={};o(`n1`,`n2`),o(`n2`,`n4`),o(`n4`,`n3`),o(`n3`,`n1`),o(`f1`,`f2`),o(`f2`,`f4`),o(`f4`,`f3`),o(`f3`,`f1`),o(`n1`,`f1`),o(`n2`,`f2`),o(`n3`,`f3`),o(`n4`,`f4`),o(`p`,`n1`),o(`p`,`n2`),o(`p`,`n3`),o(`p`,`n4`),o(`u1`,`u2`),o(`u2`,`u3`),o(`u3`,`u1`),o(`c`,`t`),o(`p`,`c`),o(`cn1`,`cn2`),o(`cn3`,`cn4`),o(`cf1`,`cf2`),o(`cf3`,`cf4`);function o(e,t){s(e),s(t)}function s(e){r.push(0,0,0),i.push(0,0,0),a[e]===void 0&&(a[e]=[]),a[e].push(r.length/3-1)}t.setAttribute(`position`,new X(r,3)),t.setAttribute(`color`,new X(i,3)),super(t,n),this.type=`CameraHelper`,this.camera=e,this.camera.updateProjectionMatrix&&this.camera.updateProjectionMatrix(),this.matrix=e.matrixWorld,this.matrixAutoUpdate=!1,this.pointMap=a,this.update();let c=new Y(16755200),l=new Y(16711680),u=new Y(43775),d=new Y(16777215),f=new Y(3355443);this.setColors(c,l,u,d,f)}setColors(e,t,n,r,i){let a=this.geometry.getAttribute(`color`);return a.setXYZ(0,e.r,e.g,e.b),a.setXYZ(1,e.r,e.g,e.b),a.setXYZ(2,e.r,e.g,e.b),a.setXYZ(3,e.r,e.g,e.b),a.setXYZ(4,e.r,e.g,e.b),a.setXYZ(5,e.r,e.g,e.b),a.setXYZ(6,e.r,e.g,e.b),a.setXYZ(7,e.r,e.g,e.b),a.setXYZ(8,e.r,e.g,e.b),a.setXYZ(9,e.r,e.g,e.b),a.setXYZ(10,e.r,e.g,e.b),a.setXYZ(11,e.r,e.g,e.b),a.setXYZ(12,e.r,e.g,e.b),a.setXYZ(13,e.r,e.g,e.b),a.setXYZ(14,e.r,e.g,e.b),a.setXYZ(15,e.r,e.g,e.b),a.setXYZ(16,e.r,e.g,e.b),a.setXYZ(17,e.r,e.g,e.b),a.setXYZ(18,e.r,e.g,e.b),a.setXYZ(19,e.r,e.g,e.b),a.setXYZ(20,e.r,e.g,e.b),a.setXYZ(21,e.r,e.g,e.b),a.setXYZ(22,e.r,e.g,e.b),a.setXYZ(23,e.r,e.g,e.b),a.setXYZ(24,t.r,t.g,t.b),a.setXYZ(25,t.r,t.g,t.b),a.setXYZ(26,t.r,t.g,t.b),a.setXYZ(27,t.r,t.g,t.b),a.setXYZ(28,t.r,t.g,t.b),a.setXYZ(29,t.r,t.g,t.b),a.setXYZ(30,t.r,t.g,t.b),a.setXYZ(31,t.r,t.g,t.b),a.setXYZ(32,n.r,n.g,n.b),a.setXYZ(33,n.r,n.g,n.b),a.setXYZ(34,n.r,n.g,n.b),a.setXYZ(35,n.r,n.g,n.b),a.setXYZ(36,n.r,n.g,n.b),a.setXYZ(37,n.r,n.g,n.b),a.setXYZ(38,r.r,r.g,r.b),a.setXYZ(39,r.r,r.g,r.b),a.setXYZ(40,i.r,i.g,i.b),a.setXYZ(41,i.r,i.g,i.b),a.setXYZ(42,i.r,i.g,i.b),a.setXYZ(43,i.r,i.g,i.b),a.setXYZ(44,i.r,i.g,i.b),a.setXYZ(45,i.r,i.g,i.b),a.setXYZ(46,i.r,i.g,i.b),a.setXYZ(47,i.r,i.g,i.b),a.setXYZ(48,i.r,i.g,i.b),a.setXYZ(49,i.r,i.g,i.b),a.needsUpdate=!0,this}update(){let e=this.geometry,t=this.pointMap,n,r;if(hf.projectionMatrixInverse.copy(this.camera.projectionMatrixInverse),this.camera.reversedDepth===!0)n=1,r=0;else if(this.camera.coordinateSystem===2e3)n=-1,r=1;else if(this.camera.coordinateSystem===2001)n=0,r=1;else throw Error(`THREE.CameraHelper.update(): Invalid coordinate system: `+this.camera.coordinateSystem);_f(`c`,t,e,hf,0,0,n),_f(`t`,t,e,hf,0,0,r),_f(`n1`,t,e,hf,-1,-1,n),_f(`n2`,t,e,hf,1,-1,n),_f(`n3`,t,e,hf,-1,1,n),_f(`n4`,t,e,hf,1,1,n),_f(`f1`,t,e,hf,-1,-1,r),_f(`f2`,t,e,hf,1,-1,r),_f(`f3`,t,e,hf,-1,1,r),_f(`f4`,t,e,hf,1,1,r),_f(`u1`,t,e,hf,.7,1.1,n),_f(`u2`,t,e,hf,-.7,1.1,n),_f(`u3`,t,e,hf,0,2,n),_f(`cf1`,t,e,hf,-1,0,r),_f(`cf2`,t,e,hf,1,0,r),_f(`cf3`,t,e,hf,0,-1,r),_f(`cf4`,t,e,hf,0,1,r),_f(`cn1`,t,e,hf,-1,0,n),_f(`cn2`,t,e,hf,1,0,n),_f(`cn3`,t,e,hf,0,-1,n),_f(`cn4`,t,e,hf,0,1,n),e.getAttribute(`position`).needsUpdate=!0}dispose(){super.dispose(),this.geometry.dispose(),this.material.dispose()}};function _f(e,t,n,r,i,a,o){mf.set(i,a,o).unproject(r);let s=t[e];if(s!==void 0){let e=n.getAttribute(`position`);for(let t=0,n=s.length;t<n;t++)e.setXYZ(s[t],mf.x,mf.y,mf.z)}}var vf=new Vr,yf=class extends Mo{constructor(e,t=16776960){let n=new Uint16Array([0,1,1,2,2,3,3,0,4,5,5,6,6,7,7,4,0,4,1,5,2,6,3,7]),r=new Float32Array(24),i=new Z;i.setIndex(new ui(n,1)),i.setAttribute(`position`,new ui(r,3)),super(i,new bo({color:t,toneMapped:!1})),this.object=e,this.type=`BoxHelper`,this.matrixAutoUpdate=!1,this.update()}update(){if(this.object!==void 0&&vf.setFromObject(this.object),vf.isEmpty())return;let e=vf.min,t=vf.max,n=this.geometry.attributes.position,r=n.array;r[0]=t.x,r[1]=t.y,r[2]=t.z,r[3]=e.x,r[4]=t.y,r[5]=t.z,r[6]=e.x,r[7]=e.y,r[8]=t.z,r[9]=t.x,r[10]=e.y,r[11]=t.z,r[12]=t.x,r[13]=t.y,r[14]=e.z,r[15]=e.x,r[16]=t.y,r[17]=e.z,r[18]=e.x,r[19]=e.y,r[20]=e.z,r[21]=t.x,r[22]=e.y,r[23]=e.z,n.needsUpdate=!0,this.geometry.computeBoundingSphere()}setFromObject(e){return this.object=e,this.update(),this}copy(e,t){return super.copy(e,t),this.object=e.object,this}dispose(){super.dispose(),this.geometry.dispose(),this.material.dispose()}},bf=class extends Mo{constructor(e,t=16776960){let n=new Uint16Array([0,1,1,2,2,3,3,0,4,5,5,6,6,7,7,4,0,4,1,5,2,6,3,7]),r=[1,1,1,-1,1,1,-1,-1,1,1,-1,1,1,1,-1,-1,1,-1,-1,-1,-1,1,-1,-1],i=new Z;i.setIndex(new ui(n,1)),i.setAttribute(`position`,new X(r,3)),super(i,new bo({color:t,toneMapped:!1})),this.box=e,this.type=`Box3Helper`,this.geometry.computeBoundingSphere()}updateMatrixWorld(e){let t=this.box;t.isEmpty()||(t.getCenter(this.position),t.getSize(this.scale),this.scale.multiplyScalar(.5),super.updateMatrixWorld(e))}dispose(){super.dispose(),this.geometry.dispose(),this.material.dispose()}},xf=class extends Oo{constructor(e,t=1,n=16776960){let r=n,i=[1,-1,0,-1,1,0,-1,-1,0,1,1,0,-1,1,0,-1,-1,0,1,-1,0,1,1,0],a=new Z;a.setAttribute(`position`,new X(i,3)),a.computeBoundingSphere(),super(a,new bo({color:r,toneMapped:!1})),this.type=`PlaneHelper`,this.plane=e,this.size=t;let o=[1,1,0,-1,1,0,-1,-1,0,1,1,0,-1,-1,0,1,-1,0],s=new Z;s.setAttribute(`position`,new X(o,3)),s.computeBoundingSphere(),this.add(new xa(s,new la({color:r,opacity:.2,transparent:!0,depthWrite:!1,toneMapped:!1})))}updateMatrixWorld(e){this.position.set(0,0,0),this.scale.set(.5*this.size,.5*this.size,1),this.lookAt(this.plane.normal),this.translateZ(-this.plane.constant),super.updateMatrixWorld(e)}dispose(){super.dispose(),this.geometry.dispose(),this.material.dispose(),this.children[0].geometry.dispose(),this.children[0].material.dispose()}},Sf=new q,Cf,wf,Tf=class extends hr{constructor(e=new q(0,0,1),t=new q(0,0,0),n=1,r=16776960,i=n*.2,a=i*.2){super(),this.type=`ArrowHelper`,Cf===void 0&&(Cf=new Z,Cf.setAttribute(`position`,new X([0,0,0,0,1,0],3)),wf=new rs(.5,1,5,1),wf.translate(0,-.5,0)),this.position.copy(t),this.line=new Oo(Cf,new bo({color:r,toneMapped:!1})),this.line.matrixAutoUpdate=!1,this.add(this.line),this.cone=new xa(wf,new la({color:r,toneMapped:!1})),this.cone.matrixAutoUpdate=!1,this.add(this.cone),this.setDirection(e),this.setLength(n,i,a)}setDirection(e){if(e.y>.99999)this.quaternion.set(0,0,0,1);else if(e.y<-.99999)this.quaternion.set(1,0,0,0);else{Sf.set(e.z,0,-e.x).normalize();let t=Math.acos(e.y);this.quaternion.setFromAxisAngle(Sf,t)}}setLength(e,t=e*.2,n=t*.2){this.line.scale.set(1,Math.max(1e-4,e-t),1),this.line.updateMatrix(),this.cone.scale.set(n,t,n),this.cone.position.y=e,this.cone.updateMatrix()}setColor(e){this.line.material.color.set(e),this.cone.material.color.set(e)}copy(e){return super.copy(e,!1),this.line.copy(e.line),this.cone.copy(e.cone),this}dispose(){super.dispose(),this.line.geometry.dispose(),this.line.material.dispose(),this.cone.geometry.dispose(),this.cone.material.dispose()}},Ef=class extends Mo{constructor(e=1){let t=[0,0,0,e,0,0,0,0,0,0,e,0,0,0,0,0,0,e],n=[1,0,0,1,.6,0,0,1,0,.6,1,0,0,0,1,0,.6,1],r=new Z;r.setAttribute(`position`,new X(t,3)),r.setAttribute(`color`,new X(n,3));let i=new bo({vertexColors:!0,toneMapped:!1});super(r,i),this.type=`AxesHelper`}setColors(e,t,n){let r=new Y,i=this.geometry.attributes.color.array;return r.set(e),r.toArray(i,0),r.toArray(i,3),r.set(t),r.toArray(i,6),r.toArray(i,9),r.set(n),r.toArray(i,12),r.toArray(i,15),this.geometry.attributes.color.needsUpdate=!0,this}dispose(){super.dispose(),this.geometry.dispose(),this.material.dispose()}},Df=class{constructor(){this.type=`ShapePath`,this.color=new Y,this.subPaths=[],this.currentPath=null,this.userData={}}moveTo(e,t){return this.currentPath=new Bs,this.subPaths.push(this.currentPath),this.currentPath.moveTo(e,t),this}lineTo(e,t){return this.currentPath.lineTo(e,t),this}quadraticCurveTo(e,t,n,r){return this.currentPath.quadraticCurveTo(e,t,n,r),this}bezierCurveTo(e,t,n,r,i,a){return this.currentPath.bezierCurveTo(e,t,n,r,i,a),this}splineThru(e){return this.currentPath.splineThru(e),this}toShapes(){function e(e,t){let n=!1,r=t.length;for(let i=0,a=r-1;i<r;a=i++){let r=t[i],o=t[a];r.y>e.y!=o.y>e.y&&e.x<(o.x-r.x)*(e.y-r.y)/(o.y-r.y)+r.x&&(n=!n)}return n}function t(t,n){let r=n.getCenter(new K);if(e(r,t))return r;let i=r.y,a=[],o=t.length;for(let e=0;e<o;e++){let n=t[e],r=t[(e+1)%o];if(n.y>i!=r.y>i){let e=n.x+(i-n.y)*(r.x-n.x)/(r.y-n.y);a.push(e)}}return a.length>1&&(a.sort((e,t)=>e-t),r.x=(a[0]+a[1])/2),r}let n=this.userData.style&&this.userData.style.fillRule||`nonzero`;n!==`nonzero`&&n!==`evenodd`&&(U(`Fill-rule "`+n+`" is not supported, falling back to "nonzero".`),n=`nonzero`);let r=n===`nonzero`?(e=>e!==0):(e=>!!(e&1)),i=[];for(let e of this.subPaths){let n=e.getPoints();if(n.length<3)continue;let r=Sc.area(n);if(r===0)continue;let a=new Bd;for(let e=0;e<n.length;e++)a.expandByPoint(n[e]);i.push({subPath:e,points:n,boundingBox:a,interiorPoint:t(n,a),absArea:Math.abs(r),winding:r<0?-1:1,container:null,exclude:!1,role:null})}i.sort((e,t)=>t.absArea-e.absArea);for(let t=0;t<i.length;t++){let n=i[t],a=0;for(let r=t-1;r>=0;r--){let t=i[r];if(t.boundingBox.containsBox(n.boundingBox)&&e(n.interiorPoint,t.points)){n.container=t.exclude?t.container:t,a=t.winding,n.winding+=a;break}}r(n.winding)===r(a)&&(n.exclude=!0)}for(let e of i)e.exclude||(e.role=e.container===null||e.container.role===`hole`?`outer`:`hole`);let a=[],o=new Map;for(let e of i){if(e.exclude||e.role!==`outer`)continue;let t=new Vs;t.curves=e.subPath.curves,a.push(t),o.set(e,t)}for(let e of i){if(e.exclude||e.role!==`hole`)continue;let t=o.get(e.container);if(!t)continue;let n=new Bs;n.curves=e.subPath.curves,t.holes.push(n)}return a}},Of=class extends Ht{constructor(e,t=null){super(),this.object=e,this.domElement=t,this.enabled=!0,this.state=-1,this.keys={},this.mouseButtons={LEFT:null,MIDDLE:null,RIGHT:null},this.touches={ONE:null,TWO:null}}connect(e){this.domElement!==null&&this.disconnect(),this.domElement=e}disconnect(){}dispose(){}update(){}};function kf(e,t){let n=e.image&&e.image.width?e.image.width/e.image.height:1;return n>t?(e.repeat.x=1,e.repeat.y=n/t,e.offset.x=0,e.offset.y=(1-e.repeat.y)/2):(e.repeat.x=t/n,e.repeat.y=1,e.offset.x=(1-e.repeat.x)/2,e.offset.y=0),e}function Af(e,t){let n=e.image&&e.image.width?e.image.width/e.image.height:1;return n>t?(e.repeat.x=t/n,e.repeat.y=1,e.offset.x=(1-e.repeat.x)/2,e.offset.y=0):(e.repeat.x=1,e.repeat.y=n/t,e.offset.x=0,e.offset.y=(1-e.repeat.y)/2),e}function jf(e){return e.repeat.x=1,e.repeat.y=1,e.offset.x=0,e.offset.y=0,e}function Mf(e,t,n,r){let i=Nf(r);switch(n){case N:return e*t;case te:return e*t/i.components*i.byteLength;case ne:return e*t/i.components*i.byteLength;case re:return e*t*2/i.components*i.byteLength;case ie:return e*t*2/i.components*i.byteLength;case P:return e*t*3/i.components*i.byteLength;case F:return e*t*4/i.components*i.byteLength;case oe:return e*t*4/i.components*i.byteLength;case L:case se:return Math.floor((e+3)/4)*Math.floor((t+3)/4)*8;case ce:case le:return Math.floor((e+3)/4)*Math.floor((t+3)/4)*16;case de:case pe:return Math.max(e,16)*Math.max(t,8)/4;case ue:case fe:return Math.max(e,8)*Math.max(t,8)/2;case me:case he:case ge:case _e:return Math.floor((e+3)/4)*Math.floor((t+3)/4)*8;case R:case ve:case ye:return Math.floor((e+3)/4)*Math.floor((t+3)/4)*16;case be:return Math.floor((e+3)/4)*Math.floor((t+3)/4)*16;case xe:return Math.floor((e+4)/5)*Math.floor((t+3)/4)*16;case Se:return Math.floor((e+4)/5)*Math.floor((t+4)/5)*16;case Ce:return Math.floor((e+5)/6)*Math.floor((t+4)/5)*16;case we:return Math.floor((e+5)/6)*Math.floor((t+5)/6)*16;case Te:return Math.floor((e+7)/8)*Math.floor((t+4)/5)*16;case Ee:return Math.floor((e+7)/8)*Math.floor((t+5)/6)*16;case De:return Math.floor((e+7)/8)*Math.floor((t+7)/8)*16;case Oe:return Math.floor((e+9)/10)*Math.floor((t+4)/5)*16;case ke:return Math.floor((e+9)/10)*Math.floor((t+5)/6)*16;case z:return Math.floor((e+9)/10)*Math.floor((t+7)/8)*16;case Ae:return Math.floor((e+9)/10)*Math.floor((t+9)/10)*16;case je:return Math.floor((e+11)/12)*Math.floor((t+9)/10)*16;case B:return Math.floor((e+11)/12)*Math.floor((t+11)/12)*16;case V:case Me:case H:return Math.ceil(e/4)*Math.ceil(t/4)*16;case Ne:case Pe:return Math.ceil(e/4)*Math.ceil(t/4)*8;case Fe:case Ie:return Math.ceil(e/4)*Math.ceil(t/4)*16}throw Error(`Unable to determine texture byte length for ${n} format.`)}function Nf(e){switch(e){case b:case x:return{byteLength:1,components:1};case C:case S:case D:return{byteLength:2,components:1};case O:case k:return{byteLength:2,components:4};case T:case w:case E:return{byteLength:4,components:1};case j:case M:return{byteLength:4,components:3}}throw Error(`THREE.TextureUtils: Unknown texture type ${e}.`)}var Pf=class{static contain(e,t){return kf(e,t)}static cover(e,t){return Af(e,t)}static fill(e){return jf(e)}static getByteLength(e,t,n,r){return Mf(e,t,n,r)}};typeof __THREE_DEVTOOLS__<`u`&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent(`register`,{detail:{revision:`186`}})),typeof window<`u`&&(window.__THREE__?U(`WARNING: Multiple instances of Three.js being imported.`):window.__THREE__=`186`);var Ff=t({ACESFilmicToneMapping:()=>4,AddEquation:()=>100,AddOperation:()=>2,AdditiveAnimationBlendMode:()=>Je,AdditiveBlending:()=>2,AgXToneMapping:()=>6,AlphaFormat:()=>N,AlwaysCompare:()=>519,AlwaysDepth:()=>1,AlwaysStencilFunc:()=>519,AmbientLight:()=>bu,AnimationAction:()=>Sd,AnimationClip:()=>Fl,AnimationLoader:()=>Kl,AnimationMixer:()=>wd,AnimationObjectGroup:()=>xd,AnimationUtils:()=>_l,ArcCurve:()=>ps,ArrayCamera:()=>Ku,ArrowHelper:()=>Tf,AttachedBindMode:()=>a,Audio:()=>td,AudioAnalyser:()=>sd,AudioContext:()=>Lu,AudioListener:()=>ed,AudioLoader:()=>Ru,AxesHelper:()=>Ef,BackSide:()=>1,BasicDepthPacking:()=>Ye,BasicShadowMap:()=>0,BatchedMesh:()=>yo,BezierInterpolant:()=>Sl,Bone:()=>Pa,BooleanKeyframeTrack:()=>Ol,Box2:()=>Bd,Box3:()=>Vr,Box3Helper:()=>bf,BoxGeometry:()=>$o,BoxHelper:()=>yf,BufferAttribute:()=>ui,BufferGeometry:()=>Z,BufferGeometryLoader:()=>Ou,ByteType:()=>x,Cache:()=>Rl,Camera:()=>cu,CameraHelper:()=>gf,CanvasTexture:()=>Jo,CapsuleGeometry:()=>es,CatmullRomCurve3:()=>bs,CineonToneMapping:()=>3,CircleGeometry:()=>ts,ClampToEdgeWrapping:()=>c,Clock:()=>Fd,Color:()=>Y,ColorKeyframeTrack:()=>kl,ColorManagement:()=>wn,Compatibility:()=>Tt,CompressedArrayTexture:()=>Go,CompressedCubeTexture:()=>Ko,CompressedTexture:()=>Wo,CompressedTextureLoader:()=>ql,ConeGeometry:()=>rs,ConstantAlphaFactor:()=>213,ConstantColorFactor:()=>211,Controls:()=>Of,CubeCamera:()=>Gu,CubeDepthTexture:()=>Zo,CubeReflectionMapping:()=>301,CubeRefractionMapping:()=>302,CubeTexture:()=>qo,CubeTextureLoader:()=>Xl,CubeUVReflectionMapping:()=>306,CubicBezierCurve:()=>js,CubicBezierCurve3:()=>Ms,CubicInterpolant:()=>yl,CullFaceBack:()=>1,CullFaceFront:()=>2,CullFaceFrontBack:()=>3,CullFaceNone:()=>0,Curve:()=>ds,CurvePath:()=>zs,CustomBlending:()=>5,CustomToneMapping:()=>5,CylinderGeometry:()=>ns,Cylindrical:()=>Ld,Data3DTexture:()=>Vn,DataArrayTexture:()=>zn,DataTexture:()=>Fa,DataTextureLoader:()=>Zl,DataUtils:()=>oi,DecrementStencilOp:()=>ot,DecrementWrapStencilOp:()=>ct,DefaultLoadingManager:()=>Vl,DepthFormat:()=>I,DepthStencilFormat:()=>ee,DepthTexture:()=>Xo,DetachedBindMode:()=>o,DirectionalLight:()=>yu,DirectionalLightHelper:()=>pf,DiscreteInterpolant:()=>xl,DodecahedronGeometry:()=>as,DoubleSide:()=>2,DstAlphaFactor:()=>206,DstColorFactor:()=>208,DynamicCopyUsage:()=>_t,DynamicDrawUsage:()=>dt,DynamicReadUsage:()=>mt,EdgesGeometry:()=>us,EllipseCurve:()=>fs,EqualCompare:()=>514,EqualDepth:()=>4,EqualStencilFunc:()=>514,EquirectangularReflectionMapping:()=>303,EquirectangularRefractionMapping:()=>304,Euler:()=>Qn,EventDispatcher:()=>Ht,ExternalTexture:()=>Qo,ExtrudeGeometry:()=>Tc,FileLoader:()=>Gl,Float16BufferAttribute:()=>vi,Float32BufferAttribute:()=>X,FloatType:()=>E,Fog:()=>Tr,FogExp2:()=>wr,FramebufferTexture:()=>Uo,FrontSide:()=>0,Frustum:()=>Za,FrustumArray:()=>$a,GLBufferAttribute:()=>Ad,GLSL1:()=>`100`,GLSL3:()=>yt,GreaterCompare:()=>516,GreaterDepth:()=>6,GreaterEqualCompare:()=>518,GreaterEqualDepth:()=>5,GreaterEqualStencilFunc:()=>518,GreaterStencilFunc:()=>516,GridHelper:()=>cf,Group:()=>gr,HTMLTexture:()=>Yo,HalfFloatType:()=>D,HemisphereLight:()=>eu,HemisphereLightHelper:()=>sf,IcosahedronGeometry:()=>Oc,ImageBitmapLoader:()=>Fu,ImageLoader:()=>Yl,ImageUtils:()=>On,IncrementStencilOp:()=>at,IncrementWrapStencilOp:()=>st,InstancedBufferAttribute:()=>za,InstancedBufferGeometry:()=>Du,InstancedInterleavedBuffer:()=>kd,InstancedMesh:()=>qa,Int16BufferAttribute:()=>mi,Int32BufferAttribute:()=>gi,Int8BufferAttribute:()=>di,IntType:()=>w,InterleavedBuffer:()=>Ai,InterleavedBufferAttribute:()=>Mi,Interpolant:()=>vl,InterpolateBezier:()=>Ue,InterpolateDiscrete:()=>Be,InterpolateLinear:()=>Ve,InterpolateSmooth:()=>He,InterpolationSamplingMode:()=>wt,InterpolationSamplingType:()=>Ct,InvertStencilOp:()=>lt,KeepStencilOp:()=>rt,KeyframeTrack:()=>El,LOD:()=>ra,LatheGeometry:()=>kc,Layers:()=>$n,LessCompare:()=>513,LessDepth:()=>2,LessEqualCompare:()=>515,LessEqualDepth:()=>3,LessEqualStencilFunc:()=>515,LessStencilFunc:()=>513,Light:()=>$l,LightProbe:()=>Cu,LightShadow:()=>iu,Line:()=>Oo,Line3:()=>Jd,LineBasicMaterial:()=>bo,LineCurve:()=>Ns,LineCurve3:()=>Ps,LineDashedMaterial:()=>ll,LineLoop:()=>No,LineSegments:()=>Mo,LinearFilter:()=>h,LinearInterpolant:()=>bl,LinearMipMapLinearFilter:()=>y,LinearMipMapNearestFilter:()=>_,LinearMipmapLinearFilter:()=>v,LinearMipmapNearestFilter:()=>g,LinearSRGBColorSpace:()=>et,LinearToneMapping:()=>1,LinearTransfer:()=>tt,Loader:()=>Hl,LoaderUtils:()=>Eu,LoadingManager:()=>Bl,LoopOnce:()=>Le,LoopPingPong:()=>ze,LoopRepeat:()=>Re,MOUSE:()=>r,Material:()=>Ri,MaterialBlending:()=>6,MaterialLoader:()=>Tu,MathUtils:()=>hn,Matrix2:()=>Rd,Matrix3:()=>yn,Matrix4:()=>J,MaxEquation:()=>104,Mesh:()=>xa,MeshBasicMaterial:()=>la,MeshDepthMaterial:()=>ol,MeshDistanceMaterial:()=>sl,MeshLambertMaterial:()=>al,MeshMatcapMaterial:()=>cl,MeshNormalMaterial:()=>il,MeshPhongMaterial:()=>nl,MeshPhysicalMaterial:()=>tl,MeshStandardMaterial:()=>el,MeshToonMaterial:()=>rl,MinEquation:()=>103,MirroredRepeatWrapping:()=>l,MixOperation:()=>1,MultiplyBlending:()=>4,MultiplyOperation:()=>0,NearestFilter:()=>u,NearestMipMapLinearFilter:()=>m,NearestMipMapNearestFilter:()=>f,NearestMipmapLinearFilter:()=>p,NearestMipmapNearestFilter:()=>d,NeutralToneMapping:()=>7,NeverCompare:()=>512,NeverDepth:()=>0,NeverStencilFunc:()=>512,NoBlending:()=>0,NoColorSpace:()=>``,NoNormalPacking:()=>``,NoToneMapping:()=>0,NormalAnimationBlendMode:()=>qe,NormalBlending:()=>1,NormalGAPacking:()=>`ga`,NormalRGPacking:()=>`rg`,NotEqualCompare:()=>517,NotEqualDepth:()=>7,NotEqualStencilFunc:()=>517,NumberKeyframeTrack:()=>Al,Object3D:()=>hr,ObjectLoader:()=>Au,ObjectSpaceNormalMap:()=>1,OctahedronGeometry:()=>Ac,OneFactor:()=>201,OneMinusConstantAlphaFactor:()=>214,OneMinusConstantColorFactor:()=>212,OneMinusDstAlphaFactor:()=>207,OneMinusDstColorFactor:()=>209,OneMinusSrcAlphaFactor:()=>205,OneMinusSrcColorFactor:()=>203,OrthographicCamera:()=>_u,PCFShadowMap:()=>1,PCFSoftShadowMap:()=>2,PMREMGenerator:()=>op,Path:()=>Bs,PerspectiveCamera:()=>fu,Plane:()=>Ii,PlaneGeometry:()=>jc,PlaneHelper:()=>xf,PointLight:()=>gu,PointLightHelper:()=>nf,Points:()=>zo,PointsMaterial:()=>Po,PolarGridHelper:()=>lf,PolyhedronGeometry:()=>is,PositionalAudio:()=>od,PropertyBinding:()=>bd,PropertyMixer:()=>cd,QuadraticBezierCurve:()=>Fs,QuadraticBezierCurve3:()=>Is,Quaternion:()=>gn,QuaternionKeyframeTrack:()=>Ml,QuaternionLinearInterpolant:()=>jl,R11_EAC_Format:()=>ge,RED_GREEN_RGTC2_Format:()=>Fe,RED_RGTC1_Format:()=>Ne,REVISION:()=>`186`,RG11_EAC_Format:()=>ve,RGBADepthPacking:()=>Xe,RGBAFormat:()=>F,RGBAIntegerFormat:()=>oe,RGBA_ASTC_10x10_Format:()=>Ae,RGBA_ASTC_10x5_Format:()=>Oe,RGBA_ASTC_10x6_Format:()=>ke,RGBA_ASTC_10x8_Format:()=>z,RGBA_ASTC_12x10_Format:()=>je,RGBA_ASTC_12x12_Format:()=>B,RGBA_ASTC_4x4_Format:()=>be,RGBA_ASTC_5x4_Format:()=>xe,RGBA_ASTC_5x5_Format:()=>Se,RGBA_ASTC_6x5_Format:()=>Ce,RGBA_ASTC_6x6_Format:()=>we,RGBA_ASTC_8x5_Format:()=>Te,RGBA_ASTC_8x6_Format:()=>Ee,RGBA_ASTC_8x8_Format:()=>De,RGBA_BPTC_Format:()=>V,RGBA_ETC2_EAC_Format:()=>R,RGBA_PVRTC_2BPPV1_Format:()=>pe,RGBA_PVRTC_4BPPV1_Format:()=>fe,RGBA_S3TC_DXT1_Format:()=>se,RGBA_S3TC_DXT3_Format:()=>ce,RGBA_S3TC_DXT5_Format:()=>le,RGBDepthPacking:()=>Ze,RGBFormat:()=>P,RGBIntegerFormat:()=>ae,RGB_BPTC_SIGNED_Format:()=>Me,RGB_BPTC_UNSIGNED_Format:()=>H,RGB_ETC1_Format:()=>me,RGB_ETC2_Format:()=>he,RGB_PVRTC_2BPPV1_Format:()=>de,RGB_PVRTC_4BPPV1_Format:()=>ue,RGB_S3TC_DXT1_Format:()=>L,RGDepthPacking:()=>Qe,RGFormat:()=>re,RGIntegerFormat:()=>ie,RawShaderMaterial:()=>$c,Ray:()=>ca,Raycaster:()=>Md,RectAreaLight:()=>xu,RedFormat:()=>te,RedIntegerFormat:()=>ne,ReinhardToneMapping:()=>2,RenderObjectRefreshType:()=>Et,RenderTarget:()=>Ln,RenderTarget3D:()=>Td,RepeatWrapping:()=>s,ReplaceStencilOp:()=>it,ReverseSubtractEquation:()=>102,RingGeometry:()=>Mc,SIGNED_R11_EAC_Format:()=>_e,SIGNED_RED_GREEN_RGTC2_Format:()=>Ie,SIGNED_RED_RGTC1_Format:()=>Pe,SIGNED_RG11_EAC_Format:()=>ye,SRGBColorSpace:()=>$e,SRGBTransfer:()=>nt,Scene:()=>Er,ShaderChunk:()=>Rf,ShaderLib:()=>zf,ShaderMaterial:()=>Qc,ShadowMaterial:()=>Uc,Shape:()=>Vs,ShapeGeometry:()=>Nc,ShapePath:()=>Df,ShapeUtils:()=>Sc,ShortType:()=>S,Skeleton:()=>Ra,SkeletonHelper:()=>ef,SkinnedMesh:()=>Na,Source:()=>Mn,Sphere:()=>Si,SphereGeometry:()=>Fc,Spherical:()=>Id,SphericalHarmonics3:()=>Su,SplineCurve:()=>Ls,SpotLight:()=>mu,SpotLightHelper:()=>Xd,Sprite:()=>$i,SpriteMaterial:()=>zi,SrcAlphaFactor:()=>204,SrcAlphaSaturateFactor:()=>210,SrcColorFactor:()=>202,StaticCopyUsage:()=>gt,StaticDrawUsage:()=>ut,StaticReadUsage:()=>pt,StereoCamera:()=>Hu,StreamCopyUsage:()=>vt,StreamDrawUsage:()=>ft,StreamReadUsage:()=>ht,StringKeyframeTrack:()=>Nl,SubtractEquation:()=>101,SubtractiveBlending:()=>3,TOUCH:()=>i,TangentSpaceNormalMap:()=>0,TetrahedronGeometry:()=>Ic,Texture:()=>Fn,TextureLoader:()=>Ql,TextureSource:()=>An,TextureUtils:()=>Pf,Timer:()=>qu,TimestampQuery:()=>St,TorusGeometry:()=>Lc,TorusKnotGeometry:()=>Rc,Triangle:()=>Br,TriangleFanDrawMode:()=>2,TriangleStripDrawMode:()=>1,TrianglesDrawMode:()=>0,TubeGeometry:()=>zc,UVMapping:()=>300,Uint16BufferAttribute:()=>hi,Uint32BufferAttribute:()=>_i,Uint8BufferAttribute:()=>fi,Uint8ClampedBufferAttribute:()=>pi,Uniform:()=>Ed,UniformsGroup:()=>Od,UniformsLib:()=>Q,UniformsUtils:()=>Yc,UnsignedByteType:()=>b,UnsignedInt101111Type:()=>M,UnsignedInt248Type:()=>A,UnsignedInt5999Type:()=>j,UnsignedIntType:()=>T,UnsignedShort4444Type:()=>O,UnsignedShort5551Type:()=>k,UnsignedShortType:()=>C,VSMShadowMap:()=>3,Vector2:()=>K,Vector3:()=>q,Vector4:()=>In,VectorKeyframeTrack:()=>Pl,VideoFrameTexture:()=>Ho,VideoTexture:()=>Vo,WebGL3DRenderTarget:()=>Hn,WebGLArrayRenderTarget:()=>Bn,WebGLCoordinateSystem:()=>bt,WebGLCubeRenderTarget:()=>hp,WebGLRenderTarget:()=>Rn,WebGLRenderer:()=>$h,WebGLUtils:()=>Vh,WebGPUCoordinateSystem:()=>xt,WebXRController:()=>vr,WireframeGeometry:()=>Bc,WrapAroundEnding:()=>Ke,ZeroCurvatureEnding:()=>We,ZeroFactor:()=>200,ZeroSlopeEnding:()=>Ge,ZeroStencilOp:()=>0,createCanvasElement:()=>Mt,error:()=>W,getConsoleFunction:()=>It,log:()=>Lt,setConsoleFunction:()=>Ft,warn:()=>U,warnOnce:()=>zt});function If(){let e=null,t=!1,n=null,r=null;function i(t,a){r=e.requestAnimationFrame(i),n(t,a)}return{start:function(){t!==!0&&n!==null&&e!==null&&(r=e.requestAnimationFrame(i),t=!0)},stop:function(){e!==null&&e.cancelAnimationFrame(r),t=!1},setAnimationLoop:function(e){n=e},setContext:function(t){e=t}}}function Lf(e){let t=new WeakMap;function n(t,n){let r=t.array,i=t.usage,a=r.byteLength,o=e.createBuffer();e.bindBuffer(n,o),e.bufferData(n,r,i),t.onUploadCallback();let s;if(r instanceof Float32Array)s=e.FLOAT;else if(typeof Float16Array<`u`&&r instanceof Float16Array)s=e.HALF_FLOAT;else if(r instanceof Uint16Array)s=t.isFloat16BufferAttribute?e.HALF_FLOAT:e.UNSIGNED_SHORT;else if(r instanceof Int16Array)s=e.SHORT;else if(r instanceof Uint32Array)s=e.UNSIGNED_INT;else if(r instanceof Int32Array)s=e.INT;else if(r instanceof Int8Array)s=e.BYTE;else if(r instanceof Uint8Array)s=e.UNSIGNED_BYTE;else if(r instanceof Uint8ClampedArray)s=e.UNSIGNED_BYTE;else throw Error(`THREE.WebGLAttributes: Unsupported buffer data format: `+r);return{buffer:o,type:s,bytesPerElement:r.BYTES_PER_ELEMENT,version:t.version,size:a}}function r(t,n,r){let i=n.array,a=n.updateRanges;if(e.bindBuffer(r,t),a.length===0)e.bufferSubData(r,0,i);else{a.sort((e,t)=>e.start-t.start);let t=0;for(let e=1;e<a.length;e++){let n=a[t],r=a[e];r.start<=n.start+n.count+1?n.count=Math.max(n.count,r.start+r.count-n.start):(++t,a[t]=r)}a.length=t+1;for(let t=0,n=a.length;t<n;t++){let n=a[t];e.bufferSubData(r,n.start*i.BYTES_PER_ELEMENT,i,n.start,n.count)}n.clearUpdateRanges()}n.onUploadCallback()}function i(e){return e.isInterleavedBufferAttribute&&(e=e.data),t.get(e)}function a(n){n.isInterleavedBufferAttribute&&(n=n.data);let r=t.get(n);r&&(e.deleteBuffer(r.buffer),t.delete(n))}function o(e,i){if(e.isInterleavedBufferAttribute&&(e=e.data),e.isGLBufferAttribute){let n=t.get(e);(!n||n.version<e.version)&&t.set(e,{buffer:e.buffer,type:e.type,bytesPerElement:e.elementSize,version:e.version});return}let a=t.get(e);if(a===void 0)t.set(e,n(e,i));else if(a.version<e.version){if(a.size!==e.array.byteLength)throw Error(`THREE.WebGLAttributes: The size of the buffer attribute's array buffer does not match the original size. Resizing buffer attributes is not supported.`);r(a.buffer,e,i),a.version=e.version}}return{get:i,remove:a,update:o}}var Rf={alphahash_fragment:`#ifdef USE_ALPHAHASH
	if ( diffuseColor.a < getAlphaHashThreshold( vPosition ) ) discard;
#endif`,alphahash_pars_fragment:`#ifdef USE_ALPHAHASH
	const float ALPHA_HASH_SCALE = 0.05;
	float hash2D( vec2 value ) {
		return fract( 1.0e4 * sin( 17.0 * value.x + 0.1 * value.y ) * ( 0.1 + abs( sin( 13.0 * value.y + value.x ) ) ) );
	}
	float hash3D( vec3 value ) {
		return hash2D( vec2( hash2D( value.xy ), value.z ) );
	}
	float getAlphaHashThreshold( vec3 position ) {
		float maxDeriv = max(
			length( dFdx( position.xyz ) ),
			length( dFdy( position.xyz ) )
		);
		float pixScale = 1.0 / ( ALPHA_HASH_SCALE * maxDeriv );
		vec2 pixScales = vec2(
			exp2( floor( log2( pixScale ) ) ),
			exp2( ceil( log2( pixScale ) ) )
		);
		vec2 alpha = vec2(
			hash3D( floor( pixScales.x * position.xyz ) ),
			hash3D( floor( pixScales.y * position.xyz ) )
		);
		float lerpFactor = fract( log2( pixScale ) );
		float x = ( 1.0 - lerpFactor ) * alpha.x + lerpFactor * alpha.y;
		float a = min( lerpFactor, 1.0 - lerpFactor );
		vec3 cases = vec3(
			x * x / ( 2.0 * a * ( 1.0 - a ) ),
			( x - 0.5 * a ) / ( 1.0 - a ),
			1.0 - ( ( 1.0 - x ) * ( 1.0 - x ) / ( 2.0 * a * ( 1.0 - a ) ) )
		);
		float threshold = ( x < ( 1.0 - a ) )
			? ( ( x < a ) ? cases.x : cases.y )
			: cases.z;
		return clamp( threshold , 1.0e-6, 1.0 );
	}
#endif`,alphamap_fragment:`#ifdef USE_ALPHAMAP
	diffuseColor.a *= texture2D( alphaMap, vAlphaMapUv ).g;
#endif`,alphamap_pars_fragment:`#ifdef USE_ALPHAMAP
	uniform sampler2D alphaMap;
#endif`,alphatest_fragment:`#ifdef USE_ALPHATEST
	#ifdef ALPHA_TO_COVERAGE
	diffuseColor.a = smoothstep( alphaTest, alphaTest + fwidth( diffuseColor.a ), diffuseColor.a );
	if ( diffuseColor.a == 0.0 ) discard;
	#else
	if ( diffuseColor.a < alphaTest ) discard;
	#endif
#endif`,alphatest_pars_fragment:`#ifdef USE_ALPHATEST
	uniform float alphaTest;
#endif`,aomap_fragment:`#ifdef USE_AOMAP
	float ambientOcclusion = ( texture2D( aoMap, vAoMapUv ).r - 1.0 ) * aoMapIntensity + 1.0;
	reflectedLight.indirectDiffuse *= ambientOcclusion;
	#if defined( USE_CLEARCOAT ) 
		clearcoatSpecularIndirect *= ambientOcclusion;
	#endif
	#if defined( USE_SHEEN ) 
		sheenSpecularIndirect *= ambientOcclusion;
	#endif
	#if defined( USE_ENVMAP ) && defined( STANDARD )
		float dotNV = saturate( dot( geometryNormal, geometryViewDir ) );
		reflectedLight.indirectSpecular *= computeSpecularOcclusion( dotNV, ambientOcclusion, material.roughness );
	#endif
#endif`,aomap_pars_fragment:`#ifdef USE_AOMAP
	uniform sampler2D aoMap;
	uniform float aoMapIntensity;
#endif`,batching_pars_vertex:`#ifdef USE_BATCHING
	#if ! defined( GL_ANGLE_multi_draw )
	#define gl_DrawID _gl_DrawID
	uniform int _gl_DrawID;
	#endif
	uniform highp sampler2D batchingTexture;
	uniform highp usampler2D batchingIdTexture;
	mat4 getBatchingMatrix( const in float i ) {
		int size = textureSize( batchingTexture, 0 ).x;
		int j = int( i ) * 4;
		int x = j % size;
		int y = j / size;
		vec4 v1 = texelFetch( batchingTexture, ivec2( x, y ), 0 );
		vec4 v2 = texelFetch( batchingTexture, ivec2( x + 1, y ), 0 );
		vec4 v3 = texelFetch( batchingTexture, ivec2( x + 2, y ), 0 );
		vec4 v4 = texelFetch( batchingTexture, ivec2( x + 3, y ), 0 );
		return mat4( v1, v2, v3, v4 );
	}
	float getIndirectIndex( const in int i ) {
		int size = textureSize( batchingIdTexture, 0 ).x;
		int x = i % size;
		int y = i / size;
		return float( texelFetch( batchingIdTexture, ivec2( x, y ), 0 ).r );
	}
#endif
#ifdef USE_BATCHING_COLOR
	uniform sampler2D batchingColorTexture;
	vec4 getBatchingColor( const in float i ) {
		int size = textureSize( batchingColorTexture, 0 ).x;
		int j = int( i );
		int x = j % size;
		int y = j / size;
		return texelFetch( batchingColorTexture, ivec2( x, y ), 0 );
	}
#endif`,batching_vertex:`#ifdef USE_BATCHING
	mat4 batchingMatrix = getBatchingMatrix( getIndirectIndex( gl_DrawID ) );
#endif`,begin_vertex:`vec3 transformed = vec3( position );
#ifdef USE_ALPHAHASH
	vPosition = vec3( position );
#endif`,beginnormal_vertex:`vec3 objectNormal = vec3( normal );
#ifdef USE_TANGENT
	vec3 objectTangent = vec3( tangent.xyz );
#endif`,bsdfs:`float G_BlinnPhong_Implicit( ) {
	return 0.25;
}
float D_BlinnPhong( const in float shininess, const in float dotNH ) {
	return RECIPROCAL_PI * ( shininess * 0.5 + 1.0 ) * pow( dotNH, shininess );
}
vec3 BRDF_BlinnPhong( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in vec3 specularColor, const in float shininess ) {
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNH = saturate( dot( normal, halfDir ) );
	float dotVH = saturate( dot( viewDir, halfDir ) );
	vec3 F = F_Schlick( specularColor, 1.0, dotVH );
	float G = G_BlinnPhong_Implicit( );
	float D = D_BlinnPhong( shininess, dotNH );
	return F * ( G * D );
} // validated`,iridescence_fragment:`#ifdef USE_IRIDESCENCE
	const mat3 XYZ_TO_REC709 = mat3(
		 3.2404542, -0.9692660,  0.0556434,
		-1.5371385,  1.8760108, -0.2040259,
		-0.4985314,  0.0415560,  1.0572252
	);
	vec3 Fresnel0ToIor( vec3 fresnel0 ) {
		vec3 sqrtF0 = sqrt( fresnel0 );
		return ( vec3( 1.0 ) + sqrtF0 ) / ( vec3( 1.0 ) - sqrtF0 );
	}
	vec3 IorToFresnel0( vec3 transmittedIor, float incidentIor ) {
		return pow2( ( transmittedIor - vec3( incidentIor ) ) / ( transmittedIor + vec3( incidentIor ) ) );
	}
	float IorToFresnel0( float transmittedIor, float incidentIor ) {
		return pow2( ( transmittedIor - incidentIor ) / ( transmittedIor + incidentIor ));
	}
	vec3 evalSensitivity( float OPD, vec3 shift ) {
		float phase = 2.0 * PI * OPD * 1.0e-9;
		vec3 val = vec3( 5.4856e-13, 4.4201e-13, 5.2481e-13 );
		vec3 pos = vec3( 1.6810e+06, 1.7953e+06, 2.2084e+06 );
		vec3 var = vec3( 4.3278e+09, 9.3046e+09, 6.6121e+09 );
		vec3 xyz = val * sqrt( 2.0 * PI * var ) * cos( pos * phase + shift ) * exp( - pow2( phase ) * var );
		xyz.x += 9.7470e-14 * sqrt( 2.0 * PI * 4.5282e+09 ) * cos( 2.2399e+06 * phase + shift[ 0 ] ) * exp( - 4.5282e+09 * pow2( phase ) );
		xyz /= 1.0685e-7;
		vec3 rgb = XYZ_TO_REC709 * xyz;
		return rgb;
	}
	vec3 evalIridescence( float outsideIOR, float eta2, float cosTheta1, float thinFilmThickness, vec3 baseF0 ) {
		vec3 I;
		float iridescenceIOR = mix( outsideIOR, eta2, smoothstep( 0.0, 0.03, thinFilmThickness ) );
		float sinTheta2Sq = pow2( outsideIOR / iridescenceIOR ) * ( 1.0 - pow2( cosTheta1 ) );
		float cosTheta2Sq = 1.0 - sinTheta2Sq;
		if ( cosTheta2Sq < 0.0 ) {
			return vec3( 1.0 );
		}
		float cosTheta2 = sqrt( cosTheta2Sq );
		float R0 = IorToFresnel0( iridescenceIOR, outsideIOR );
		float R12 = F_Schlick( R0, 1.0, cosTheta1 );
		float T121 = 1.0 - R12;
		float phi12 = 0.0;
		if ( iridescenceIOR < outsideIOR ) phi12 = PI;
		float phi21 = PI - phi12;
		vec3 baseIOR = Fresnel0ToIor( clamp( baseF0, 0.0, 0.9999 ) );		vec3 R1 = IorToFresnel0( baseIOR, iridescenceIOR );
		vec3 R23 = F_Schlick( R1, 1.0, cosTheta2 );
		vec3 phi23 = vec3( 0.0 );
		if ( baseIOR[ 0 ] < iridescenceIOR ) phi23[ 0 ] = PI;
		if ( baseIOR[ 1 ] < iridescenceIOR ) phi23[ 1 ] = PI;
		if ( baseIOR[ 2 ] < iridescenceIOR ) phi23[ 2 ] = PI;
		float OPD = 2.0 * iridescenceIOR * thinFilmThickness * cosTheta2;
		vec3 phi = vec3( phi21 ) + phi23;
		vec3 R123 = clamp( R12 * R23, 1e-5, 0.9999 );
		vec3 r123 = sqrt( R123 );
		vec3 Rs = pow2( T121 ) * R23 / ( vec3( 1.0 ) - R123 );
		vec3 C0 = R12 + Rs;
		I = C0;
		vec3 Cm = Rs - T121;
		for ( int m = 1; m <= 2; ++ m ) {
			Cm *= r123;
			vec3 Sm = 2.0 * evalSensitivity( float( m ) * OPD, float( m ) * phi );
			I += Cm * Sm;
		}
		return max( I, vec3( 0.0 ) );
	}
#endif`,bumpmap_pars_fragment:`#ifdef USE_BUMPMAP
	uniform sampler2D bumpMap;
	uniform float bumpScale;
	vec2 dHdxy_fwd() {
		vec2 dSTdx = dFdx( vBumpMapUv );
		vec2 dSTdy = dFdy( vBumpMapUv );
		float Hll = bumpScale * texture2D( bumpMap, vBumpMapUv ).x;
		float dBx = bumpScale * texture2D( bumpMap, vBumpMapUv + dSTdx ).x - Hll;
		float dBy = bumpScale * texture2D( bumpMap, vBumpMapUv + dSTdy ).x - Hll;
		return vec2( dBx, dBy );
	}
	vec3 perturbNormalArb( vec3 surf_pos, vec3 surf_norm, vec2 dHdxy, float faceDirection ) {
		vec3 vSigmaX = normalize( dFdx( surf_pos.xyz ) );
		vec3 vSigmaY = normalize( dFdy( surf_pos.xyz ) );
		vec3 vN = surf_norm;
		vec3 R1 = cross( vSigmaY, vN );
		vec3 R2 = cross( vN, vSigmaX );
		float fDet = dot( vSigmaX, R1 ) * faceDirection;
		vec3 vGrad = sign( fDet ) * ( dHdxy.x * R1 + dHdxy.y * R2 );
		return normalize( abs( fDet ) * surf_norm - vGrad );
	}
#endif`,clipping_planes_fragment:`#if NUM_CLIPPING_PLANES > 0
	vec4 plane;
	#ifdef ALPHA_TO_COVERAGE
		float distanceToPlane, distanceGradient;
		float clipOpacity = 1.0;
		#pragma unroll_loop_start
		for ( int i = 0; i < UNION_CLIPPING_PLANES; i ++ ) {
			plane = clippingPlanes[ i ];
			distanceToPlane = - dot( vClipPosition, plane.xyz ) + plane.w;
			distanceGradient = fwidth( distanceToPlane ) / 2.0;
			clipOpacity *= smoothstep( - distanceGradient, distanceGradient, distanceToPlane );
			if ( clipOpacity == 0.0 ) discard;
		}
		#pragma unroll_loop_end
		#if UNION_CLIPPING_PLANES < NUM_CLIPPING_PLANES
			float unionClipOpacity = 1.0;
			#pragma unroll_loop_start
			for ( int i = UNION_CLIPPING_PLANES; i < NUM_CLIPPING_PLANES; i ++ ) {
				plane = clippingPlanes[ i ];
				distanceToPlane = - dot( vClipPosition, plane.xyz ) + plane.w;
				distanceGradient = fwidth( distanceToPlane ) / 2.0;
				unionClipOpacity *= 1.0 - smoothstep( - distanceGradient, distanceGradient, distanceToPlane );
			}
			#pragma unroll_loop_end
			clipOpacity *= 1.0 - unionClipOpacity;
		#endif
		diffuseColor.a *= clipOpacity;
		if ( diffuseColor.a == 0.0 ) discard;
	#else
		#pragma unroll_loop_start
		for ( int i = 0; i < UNION_CLIPPING_PLANES; i ++ ) {
			plane = clippingPlanes[ i ];
			if ( dot( vClipPosition, plane.xyz ) > plane.w ) discard;
		}
		#pragma unroll_loop_end
		#if UNION_CLIPPING_PLANES < NUM_CLIPPING_PLANES
			bool clipped = true;
			#pragma unroll_loop_start
			for ( int i = UNION_CLIPPING_PLANES; i < NUM_CLIPPING_PLANES; i ++ ) {
				plane = clippingPlanes[ i ];
				clipped = ( dot( vClipPosition, plane.xyz ) > plane.w ) && clipped;
			}
			#pragma unroll_loop_end
			if ( clipped ) discard;
		#endif
	#endif
#endif`,clipping_planes_pars_fragment:`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
	uniform vec4 clippingPlanes[ NUM_CLIPPING_PLANES ];
#endif`,clipping_planes_pars_vertex:`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
#endif`,clipping_planes_vertex:`#if NUM_CLIPPING_PLANES > 0
	vClipPosition = - mvPosition.xyz;
#endif`,color_fragment:`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA )
	diffuseColor *= vColor;
#endif`,color_pars_fragment:`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA )
	varying vec4 vColor;
#endif`,color_pars_vertex:`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA ) || defined( USE_INSTANCING_COLOR ) || defined( USE_BATCHING_COLOR )
	varying vec4 vColor;
#endif`,color_vertex:`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA ) || defined( USE_INSTANCING_COLOR ) || defined( USE_BATCHING_COLOR )
	vColor = vec4( 1.0 );
#endif
#ifdef USE_COLOR_ALPHA
	vColor *= color;
#elif defined( USE_COLOR )
	vColor.rgb *= color;
#endif
#ifdef USE_INSTANCING_COLOR
	vColor.rgb *= instanceColor.rgb;
#endif
#ifdef USE_BATCHING_COLOR
	vColor *= getBatchingColor( getIndirectIndex( gl_DrawID ) );
#endif`,common:`#define PI 3.141592653589793
#define PI2 6.283185307179586
#define PI_HALF 1.5707963267948966
#define RECIPROCAL_PI 0.3183098861837907
#define RECIPROCAL_PI2 0.15915494309189535
#define EPSILON 1e-6
#ifndef saturate
#define saturate( a ) clamp( a, 0.0, 1.0 )
#endif
#define whiteComplement( a ) ( 1.0 - saturate( a ) )
float pow2( const in float x ) { return x*x; }
vec3 pow2( const in vec3 x ) { return x*x; }
float pow3( const in float x ) { return x*x*x; }
float pow4( const in float x ) { float x2 = x*x; return x2*x2; }
float max3( const in vec3 v ) { return max( max( v.x, v.y ), v.z ); }
float average( const in vec3 v ) { return dot( v, vec3( 0.3333333 ) ); }
highp float rand( const in vec2 uv ) {
	const highp float a = 12.9898, b = 78.233, c = 43758.5453;
	highp float dt = dot( uv.xy, vec2( a,b ) ), sn = mod( dt, PI );
	return fract( sin( sn ) * c );
}
#ifdef HIGH_PRECISION
	float precisionSafeLength( vec3 v ) { return length( v ); }
#else
	float precisionSafeLength( vec3 v ) {
		float maxComponent = max3( abs( v ) );
		return length( v / maxComponent ) * maxComponent;
	}
#endif
struct IncidentLight {
	vec3 color;
	vec3 direction;
	bool visible;
};
struct ReflectedLight {
	vec3 directDiffuse;
	vec3 directSpecular;
	vec3 indirectDiffuse;
	vec3 indirectSpecular;
};
#ifdef USE_ALPHAHASH
	varying vec3 vPosition;
#endif
vec3 transformDirection( in vec3 dir, in mat4 matrix ) {
	return normalize( ( matrix * vec4( dir, 0.0 ) ).xyz );
}
#define inverseTransformDirection transformDirectionByInverseViewMatrix
vec3 transformNormalByInverseViewMatrix( in vec3 normal, in mat4 viewMatrix ) {
	return normalize( ( vec4( normal, 0.0 ) * viewMatrix ).xyz );
}
vec3 transformDirectionByInverseViewMatrix( in vec3 dir, in mat4 viewMatrix ) {
	return normalize( ( vec4( dir, 0.0 ) * viewMatrix ).xyz );
}
bool isPerspectiveMatrix( mat4 m ) {
	return m[ 2 ][ 3 ] == - 1.0;
}
vec2 equirectUv( in vec3 dir ) {
	float u = atan( dir.z, dir.x ) * RECIPROCAL_PI2 + 0.5;
	float v = asin( clamp( dir.y, - 1.0, 1.0 ) ) * RECIPROCAL_PI + 0.5;
	return vec2( u, v );
}
vec3 BRDF_Lambert( const in vec3 diffuseColor ) {
	return RECIPROCAL_PI * diffuseColor;
}
vec3 F_Schlick( const in vec3 f0, const in float f90, const in float dotVH ) {
	float fresnel = exp2( ( - 5.55473 * dotVH - 6.98316 ) * dotVH );
	return f0 * ( 1.0 - fresnel ) + ( f90 * fresnel );
}
float F_Schlick( const in float f0, const in float f90, const in float dotVH ) {
	float fresnel = exp2( ( - 5.55473 * dotVH - 6.98316 ) * dotVH );
	return f0 * ( 1.0 - fresnel ) + ( f90 * fresnel );
} // validated`,cube_uv_reflection_fragment:`#ifdef ENVMAP_TYPE_CUBE_UV
	#define cubeUV_minMipLevel 4.0
	#define cubeUV_minTileSize 16.0
	float getFace( vec3 direction ) {
		vec3 absDirection = abs( direction );
		float face = - 1.0;
		if ( absDirection.x > absDirection.z ) {
			if ( absDirection.x > absDirection.y )
				face = direction.x > 0.0 ? 0.0 : 3.0;
			else
				face = direction.y > 0.0 ? 1.0 : 4.0;
		} else {
			if ( absDirection.z > absDirection.y )
				face = direction.z > 0.0 ? 2.0 : 5.0;
			else
				face = direction.y > 0.0 ? 1.0 : 4.0;
		}
		return face;
	}
	vec2 getUV( vec3 direction, float face ) {
		vec2 uv;
		if ( face == 0.0 ) {
			uv = vec2( direction.z, direction.y ) / abs( direction.x );
		} else if ( face == 1.0 ) {
			uv = vec2( - direction.x, - direction.z ) / abs( direction.y );
		} else if ( face == 2.0 ) {
			uv = vec2( - direction.x, direction.y ) / abs( direction.z );
		} else if ( face == 3.0 ) {
			uv = vec2( - direction.z, direction.y ) / abs( direction.x );
		} else if ( face == 4.0 ) {
			uv = vec2( - direction.x, direction.z ) / abs( direction.y );
		} else {
			uv = vec2( direction.x, direction.y ) / abs( direction.z );
		}
		return 0.5 * ( uv + 1.0 );
	}
	vec3 bilinearCubeUV( sampler2D envMap, vec3 direction, float mipInt ) {
		float face = getFace( direction );
		float filterInt = max( cubeUV_minMipLevel - mipInt, 0.0 );
		mipInt = max( mipInt, cubeUV_minMipLevel );
		float faceSize = exp2( mipInt );
		highp vec2 uv = getUV( direction, face ) * ( faceSize - 2.0 ) + 1.0;
		if ( face > 2.0 ) {
			uv.y += faceSize;
			face -= 3.0;
		}
		uv.x += face * faceSize;
		uv.x += filterInt * 3.0 * cubeUV_minTileSize;
		uv.y += 4.0 * ( exp2( CUBEUV_MAX_MIP ) - faceSize );
		uv.x *= CUBEUV_TEXEL_WIDTH;
		uv.y *= CUBEUV_TEXEL_HEIGHT;
		#ifdef texture2DGradEXT
			return texture2DGradEXT( envMap, uv, vec2( 0.0 ), vec2( 0.0 ) ).rgb;
		#else
			return texture2D( envMap, uv ).rgb;
		#endif
	}
	#define cubeUV_r0 1.0
	#define cubeUV_m0 - 2.0
	#define cubeUV_r1 0.8
	#define cubeUV_m1 - 1.0
	#define cubeUV_r4 0.4
	#define cubeUV_m4 2.0
	#define cubeUV_r5 0.305
	#define cubeUV_m5 3.0
	#define cubeUV_r6 0.21
	#define cubeUV_m6 4.0
	float roughnessToMip( float roughness ) {
		float mip = 0.0;
		if ( roughness >= cubeUV_r1 ) {
			mip = ( cubeUV_r0 - roughness ) * ( cubeUV_m1 - cubeUV_m0 ) / ( cubeUV_r0 - cubeUV_r1 ) + cubeUV_m0;
		} else if ( roughness >= cubeUV_r4 ) {
			mip = ( cubeUV_r1 - roughness ) * ( cubeUV_m4 - cubeUV_m1 ) / ( cubeUV_r1 - cubeUV_r4 ) + cubeUV_m1;
		} else if ( roughness >= cubeUV_r5 ) {
			mip = ( cubeUV_r4 - roughness ) * ( cubeUV_m5 - cubeUV_m4 ) / ( cubeUV_r4 - cubeUV_r5 ) + cubeUV_m4;
		} else if ( roughness >= cubeUV_r6 ) {
			mip = ( cubeUV_r5 - roughness ) * ( cubeUV_m6 - cubeUV_m5 ) / ( cubeUV_r5 - cubeUV_r6 ) + cubeUV_m5;
		} else {
			mip = - 2.0 * log2( 1.16 * roughness );		}
		return mip;
	}
	vec4 textureCubeUV( sampler2D envMap, vec3 sampleDir, float roughness ) {
		float mip = clamp( roughnessToMip( roughness ), cubeUV_m0, CUBEUV_MAX_MIP );
		float mipF = fract( mip );
		float mipInt = floor( mip );
		vec3 color0 = bilinearCubeUV( envMap, sampleDir, mipInt );
		if ( mipF == 0.0 ) {
			return vec4( color0, 1.0 );
		} else {
			vec3 color1 = bilinearCubeUV( envMap, sampleDir, mipInt + 1.0 );
			return vec4( mix( color0, color1, mipF ), 1.0 );
		}
	}
#endif`,defaultnormal_vertex:`vec3 transformedNormal = objectNormal;
#ifdef USE_TANGENT
	vec3 transformedTangent = objectTangent;
#endif
#ifdef USE_BATCHING
	mat3 bm = mat3( batchingMatrix );
	transformedNormal /= vec3( dot( bm[ 0 ], bm[ 0 ] ), dot( bm[ 1 ], bm[ 1 ] ), dot( bm[ 2 ], bm[ 2 ] ) );
	transformedNormal = bm * transformedNormal;
	#ifdef USE_TANGENT
		transformedTangent = bm * transformedTangent;
	#endif
#endif
#ifdef USE_INSTANCING
	mat3 im = mat3( instanceMatrix );
	transformedNormal /= vec3( dot( im[ 0 ], im[ 0 ] ), dot( im[ 1 ], im[ 1 ] ), dot( im[ 2 ], im[ 2 ] ) );
	transformedNormal = im * transformedNormal;
	#ifdef USE_TANGENT
		transformedTangent = im * transformedTangent;
	#endif
#endif
transformedNormal = normalMatrix * transformedNormal;
#ifdef FLIP_SIDED
	transformedNormal = - transformedNormal;
#endif
#ifdef USE_TANGENT
	transformedTangent = ( modelViewMatrix * vec4( transformedTangent, 0.0 ) ).xyz;
#endif`,displacementmap_pars_vertex:`#ifdef USE_DISPLACEMENTMAP
	uniform sampler2D displacementMap;
	uniform float displacementScale;
	uniform float displacementBias;
#endif`,displacementmap_vertex:`#ifdef USE_DISPLACEMENTMAP
	transformed += normalize( objectNormal ) * ( texture2D( displacementMap, vDisplacementMapUv ).x * displacementScale + displacementBias );
#endif`,emissivemap_fragment:`#ifdef USE_EMISSIVEMAP
	vec4 emissiveColor = texture2D( emissiveMap, vEmissiveMapUv );
	#ifdef DECODE_VIDEO_TEXTURE_EMISSIVE
		emissiveColor = sRGBTransferEOTF( emissiveColor );
	#endif
	totalEmissiveRadiance *= emissiveColor.rgb;
#endif`,emissivemap_pars_fragment:`#ifdef USE_EMISSIVEMAP
	uniform sampler2D emissiveMap;
#endif`,colorspace_fragment:`gl_FragColor = linearToOutputTexel( gl_FragColor );`,colorspace_pars_fragment:`vec4 LinearTransferOETF( in vec4 value ) {
	return value;
}
vec4 sRGBTransferEOTF( in vec4 value ) {
	return vec4( mix( pow( value.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), value.rgb * 0.0773993808, vec3( lessThanEqual( value.rgb, vec3( 0.04045 ) ) ) ), value.a );
}
vec4 sRGBTransferOETF( in vec4 value ) {
	return vec4( mix( pow( value.rgb, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), value.rgb * 12.92, vec3( lessThanEqual( value.rgb, vec3( 0.0031308 ) ) ) ), value.a );
}`,envmap_fragment:`#ifdef USE_ENVMAP
	#ifdef ENV_WORLDPOS
		vec3 cameraToFrag;
		if ( isOrthographic ) {
			cameraToFrag = normalize( vec3( - viewMatrix[ 0 ][ 2 ], - viewMatrix[ 1 ][ 2 ], - viewMatrix[ 2 ][ 2 ] ) );
		} else {
			cameraToFrag = normalize( vWorldPosition - cameraPosition );
		}
		vec3 worldNormal = transformNormalByInverseViewMatrix( normal, viewMatrix );
		#ifdef ENVMAP_MODE_REFLECTION
			vec3 reflectVec = reflect( cameraToFrag, worldNormal );
		#else
			vec3 reflectVec = refract( cameraToFrag, worldNormal, refractionRatio );
		#endif
	#else
		vec3 reflectVec = vReflect;
	#endif
	#ifdef ENVMAP_TYPE_CUBE
		vec4 envColor = textureCube( envMap, envMapRotation * reflectVec );
		#ifdef ENVMAP_BLENDING_MULTIPLY
			outgoingLight = mix( outgoingLight, outgoingLight * envColor.xyz, specularStrength * reflectivity );
		#elif defined( ENVMAP_BLENDING_MIX )
			outgoingLight = mix( outgoingLight, envColor.xyz, specularStrength * reflectivity );
		#elif defined( ENVMAP_BLENDING_ADD )
			outgoingLight += envColor.xyz * specularStrength * reflectivity;
		#endif
	#endif
#endif`,envmap_common_pars_fragment:`#ifdef USE_ENVMAP
	uniform float envMapIntensity;
	uniform mat3 envMapRotation;
	#ifdef ENVMAP_TYPE_CUBE
		uniform samplerCube envMap;
	#else
		uniform sampler2D envMap;
	#endif
#endif`,envmap_pars_fragment:`#ifdef USE_ENVMAP
	uniform float reflectivity;
	#if defined( USE_BUMPMAP ) || defined( USE_NORMALMAP ) || defined( PHONG ) || defined( LAMBERT )
		#define ENV_WORLDPOS
	#endif
	#ifdef ENV_WORLDPOS
		varying vec3 vWorldPosition;
		uniform float refractionRatio;
	#else
		varying vec3 vReflect;
	#endif
#endif`,envmap_pars_vertex:`#ifdef USE_ENVMAP
	#if defined( USE_BUMPMAP ) || defined( USE_NORMALMAP ) || defined( PHONG ) || defined( LAMBERT )
		#define ENV_WORLDPOS
	#endif
	#ifdef ENV_WORLDPOS
		
		varying vec3 vWorldPosition;
	#else
		varying vec3 vReflect;
		uniform float refractionRatio;
	#endif
#endif`,envmap_physical_pars_fragment:`#ifdef USE_ENVMAP
	vec3 getIBLIrradiance( const in vec3 normal ) {
		#ifdef ENVMAP_TYPE_CUBE_UV
			vec3 worldNormal = transformNormalByInverseViewMatrix( normal, viewMatrix );
			vec4 envMapColor = textureCubeUV( envMap, envMapRotation * worldNormal, 1.0 );
			return PI * envMapColor.rgb * envMapIntensity;
		#else
			return vec3( 0.0 );
		#endif
	}
	vec3 getIBLRadiance( const in vec3 viewDir, const in vec3 normal, const in float roughness ) {
		#ifdef ENVMAP_TYPE_CUBE_UV
			vec3 reflectVec = reflect( - viewDir, normal );
			reflectVec = normalize( mix( reflectVec, normal, pow4( roughness ) ) );
			reflectVec = transformDirectionByInverseViewMatrix( reflectVec, viewMatrix );
			vec4 envMapColor = textureCubeUV( envMap, envMapRotation * reflectVec, roughness );
			return envMapColor.rgb * envMapIntensity;
		#else
			return vec3( 0.0 );
		#endif
	}
	#ifdef USE_RETROREFLECTION
		vec3 getIBLRetroRadiance( const in vec3 viewDir, const in vec3 normal, const in float roughness ) {
			#ifdef ENVMAP_TYPE_CUBE_UV
				vec3 retroVec = normalize( mix( viewDir, normal, pow4( roughness ) ) );
				retroVec = transformDirectionByInverseViewMatrix( retroVec, viewMatrix );
				vec4 envMapColor = textureCubeUV( envMap, envMapRotation * retroVec, roughness );
				return envMapColor.rgb * envMapIntensity;
			#else
				return vec3( 0.0 );
			#endif
		}
	#endif
	#ifdef USE_ANISOTROPY
		vec3 getIBLAnisotropyRadiance( const in vec3 viewDir, const in vec3 normal, const in float roughness, const in vec3 bitangent, const in float anisotropy ) {
			#ifdef ENVMAP_TYPE_CUBE_UV
				vec3 bentNormal = cross( bitangent, viewDir );
				bentNormal = normalize( cross( bentNormal, bitangent ) );
				bentNormal = normalize( mix( bentNormal, normal, pow2( pow2( 1.0 - anisotropy * ( 1.0 - roughness ) ) ) ) );
				return getIBLRadiance( viewDir, bentNormal, roughness );
			#else
				return vec3( 0.0 );
			#endif
		}
		#ifdef USE_RETROREFLECTION
			vec3 getIBLAnisotropyRetroRadiance( const in vec3 viewDir, const in vec3 normal, const in float roughness, const in vec3 bitangent, const in float anisotropy ) {
				#ifdef ENVMAP_TYPE_CUBE_UV
					vec3 bentNormal = cross( bitangent, viewDir );
					bentNormal = normalize( cross( bentNormal, bitangent ) );
					bentNormal = normalize( mix( bentNormal, normal, pow2( pow2( 1.0 - anisotropy * ( 1.0 - roughness ) ) ) ) );
					return getIBLRetroRadiance( viewDir, bentNormal, roughness );
				#else
					return vec3( 0.0 );
				#endif
			}
		#endif
	#endif
#endif`,envmap_vertex:`#ifdef USE_ENVMAP
	#ifdef ENV_WORLDPOS
		vWorldPosition = worldPosition.xyz;
	#else
		vec3 cameraToVertex;
		if ( isOrthographic ) {
			cameraToVertex = normalize( vec3( - viewMatrix[ 0 ][ 2 ], - viewMatrix[ 1 ][ 2 ], - viewMatrix[ 2 ][ 2 ] ) );
		} else {
			cameraToVertex = normalize( worldPosition.xyz - cameraPosition );
		}
		vec3 worldNormal = transformNormalByInverseViewMatrix( transformedNormal, viewMatrix );
		#ifdef ENVMAP_MODE_REFLECTION
			vReflect = reflect( cameraToVertex, worldNormal );
		#else
			vReflect = refract( cameraToVertex, worldNormal, refractionRatio );
		#endif
	#endif
#endif`,fog_vertex:`#ifdef USE_FOG
	vFogDepth = - mvPosition.z;
#endif`,fog_pars_vertex:`#ifdef USE_FOG
	varying float vFogDepth;
#endif`,fog_fragment:`#ifdef USE_FOG
	#ifdef FOG_EXP2
		float fogFactor = 1.0 - exp( - fogDensity * fogDensity * vFogDepth * vFogDepth );
	#else
		float fogFactor = smoothstep( fogNear, fogFar, vFogDepth );
	#endif
	gl_FragColor.rgb = mix( gl_FragColor.rgb, fogColor, fogFactor );
#endif`,fog_pars_fragment:`#ifdef USE_FOG
	uniform vec3 fogColor;
	varying float vFogDepth;
	#ifdef FOG_EXP2
		uniform float fogDensity;
	#else
		uniform float fogNear;
		uniform float fogFar;
	#endif
#endif`,gradientmap_pars_fragment:`#ifdef USE_GRADIENTMAP
	uniform sampler2D gradientMap;
#endif
vec3 getGradientIrradiance( vec3 normal, vec3 lightDirection ) {
	float dotNL = dot( normal, lightDirection );
	vec2 coord = vec2( dotNL * 0.5 + 0.5, 0.0 );
	#ifdef USE_GRADIENTMAP
		return vec3( texture2D( gradientMap, coord ).r );
	#else
		vec2 fw = fwidth( coord ) * 0.5;
		return mix( vec3( 0.7 ), vec3( 1.0 ), smoothstep( 0.7 - fw.x, 0.7 + fw.x, coord.x ) );
	#endif
}`,lightmap_pars_fragment:`#ifdef USE_LIGHTMAP
	uniform sampler2D lightMap;
	uniform float lightMapIntensity;
#endif`,lights_lambert_fragment:`LambertMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularStrength = specularStrength;`,lights_lambert_pars_fragment:`varying vec3 vViewPosition;
struct LambertMaterial {
	vec3 diffuseColor;
	float specularStrength;
};
void RE_Direct_Lambert( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in LambertMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectDiffuse_Lambert( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in LambertMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_Lambert
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Lambert`,lights_pars_begin:`uniform bool receiveShadow;
uniform vec3 ambientLightColor;
#if defined( USE_LIGHT_PROBES )
	uniform vec3 lightProbe[ 9 ];
#endif
vec3 shGetIrradianceAt( in vec3 normal, in vec3 shCoefficients[ 9 ] ) {
	float x = normal.x, y = normal.y, z = normal.z;
	vec3 result = shCoefficients[ 0 ] * 0.886227;
	result += shCoefficients[ 1 ] * 2.0 * 0.511664 * y;
	result += shCoefficients[ 2 ] * 2.0 * 0.511664 * z;
	result += shCoefficients[ 3 ] * 2.0 * 0.511664 * x;
	result += shCoefficients[ 4 ] * 2.0 * 0.429043 * x * y;
	result += shCoefficients[ 5 ] * 2.0 * 0.429043 * y * z;
	result += shCoefficients[ 6 ] * ( 0.743125 * z * z - 0.247708 );
	result += shCoefficients[ 7 ] * 2.0 * 0.429043 * x * z;
	result += shCoefficients[ 8 ] * 0.429043 * ( x * x - y * y );
	return result;
}
vec3 getLightProbeIrradiance( const in vec3 lightProbe[ 9 ], const in vec3 normal ) {
	vec3 worldNormal = transformNormalByInverseViewMatrix( normal, viewMatrix );
	vec3 irradiance = shGetIrradianceAt( worldNormal, lightProbe );
	return irradiance;
}
vec3 getAmbientLightIrradiance( const in vec3 ambientLightColor ) {
	vec3 irradiance = ambientLightColor;
	return irradiance;
}
float getDistanceAttenuation( const in float lightDistance, const in float cutoffDistance, const in float decayExponent ) {
	float distanceFalloff = 1.0 / max( pow( lightDistance, decayExponent ), 0.01 );
	if ( cutoffDistance > 0.0 ) {
		distanceFalloff *= pow2( saturate( 1.0 - pow4( lightDistance / cutoffDistance ) ) );
	}
	return distanceFalloff;
}
float getSpotAttenuation( const in float coneCosine, const in float penumbraCosine, const in float angleCosine ) {
	return smoothstep( coneCosine, penumbraCosine, angleCosine );
}
#if NUM_SUN_LIGHTS > 0
	struct SunLight {
		vec3 direction;
		vec3 color;
	};
	uniform SunLight sunLights[ NUM_SUN_LIGHTS ];
	void getSunLightInfo( const in SunLight sunLight, out IncidentLight light ) {
		light.color = sunLight.color;
		light.direction = sunLight.direction;
		light.visible = true;
	}
#endif
#if NUM_DIR_LIGHTS > 0
	struct DirectionalLight {
		vec3 direction;
		vec3 color;
	};
	uniform DirectionalLight directionalLights[ NUM_DIR_LIGHTS ];
	void getDirectionalLightInfo( const in DirectionalLight directionalLight, out IncidentLight light ) {
		light.color = directionalLight.color;
		light.direction = directionalLight.direction;
		light.visible = true;
	}
#endif
#if NUM_POINT_LIGHTS > 0
	struct PointLight {
		vec3 position;
		vec3 color;
		float distance;
		float decay;
	};
	uniform PointLight pointLights[ NUM_POINT_LIGHTS ];
	void getPointLightInfo( const in PointLight pointLight, const in vec3 geometryPosition, out IncidentLight light ) {
		vec3 lVector = pointLight.position - geometryPosition;
		light.direction = normalize( lVector );
		float lightDistance = length( lVector );
		light.color = pointLight.color;
		light.color *= getDistanceAttenuation( lightDistance, pointLight.distance, pointLight.decay );
		light.visible = ( light.color != vec3( 0.0 ) );
	}
#endif
#if NUM_SPOT_LIGHTS > 0
	struct SpotLight {
		vec3 position;
		vec3 direction;
		vec3 color;
		float distance;
		float decay;
		float coneCos;
		float penumbraCos;
	};
	uniform SpotLight spotLights[ NUM_SPOT_LIGHTS ];
	void getSpotLightInfo( const in SpotLight spotLight, const in vec3 geometryPosition, out IncidentLight light ) {
		vec3 lVector = spotLight.position - geometryPosition;
		light.direction = normalize( lVector );
		float angleCos = dot( light.direction, spotLight.direction );
		float spotAttenuation = getSpotAttenuation( spotLight.coneCos, spotLight.penumbraCos, angleCos );
		if ( spotAttenuation > 0.0 ) {
			float lightDistance = length( lVector );
			light.color = spotLight.color * spotAttenuation;
			light.color *= getDistanceAttenuation( lightDistance, spotLight.distance, spotLight.decay );
			light.visible = ( light.color != vec3( 0.0 ) );
		} else {
			light.color = vec3( 0.0 );
			light.visible = false;
		}
	}
#endif
#if NUM_RECT_AREA_LIGHTS > 0
	struct RectAreaLight {
		vec3 color;
		vec3 position;
		vec3 halfWidth;
		vec3 halfHeight;
	};
	uniform sampler2D ltc_1;	uniform sampler2D ltc_2;
	uniform RectAreaLight rectAreaLights[ NUM_RECT_AREA_LIGHTS ];
#endif
#if NUM_HEMI_LIGHTS > 0
	struct HemisphereLight {
		vec3 direction;
		vec3 skyColor;
		vec3 groundColor;
	};
	uniform HemisphereLight hemisphereLights[ NUM_HEMI_LIGHTS ];
	vec3 getHemisphereLightIrradiance( const in HemisphereLight hemiLight, const in vec3 normal ) {
		float dotNL = dot( normal, hemiLight.direction );
		float hemiDiffuseWeight = 0.5 * dotNL + 0.5;
		vec3 irradiance = mix( hemiLight.groundColor, hemiLight.skyColor, hemiDiffuseWeight );
		return irradiance;
	}
#endif
#include <lightprobes_pars_fragment>`,lights_toon_fragment:`ToonMaterial material;
material.diffuseColor = diffuseColor.rgb;`,lights_toon_pars_fragment:`varying vec3 vViewPosition;
struct ToonMaterial {
	vec3 diffuseColor;
};
void RE_Direct_Toon( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in ToonMaterial material, inout ReflectedLight reflectedLight ) {
	vec3 irradiance = getGradientIrradiance( geometryNormal, directLight.direction ) * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectDiffuse_Toon( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in ToonMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_Toon
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Toon`,lights_phong_fragment:`BlinnPhongMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularColor = specular;
material.specularShininess = shininess;
material.specularStrength = specularStrength;`,lights_phong_pars_fragment:`varying vec3 vViewPosition;
struct BlinnPhongMaterial {
	vec3 diffuseColor;
	vec3 specularColor;
	float specularShininess;
	float specularStrength;
};
void RE_Direct_BlinnPhong( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in BlinnPhongMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
	reflectedLight.directSpecular += irradiance * BRDF_BlinnPhong( directLight.direction, geometryViewDir, geometryNormal, material.specularColor, material.specularShininess ) * material.specularStrength;
}
void RE_IndirectDiffuse_BlinnPhong( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in BlinnPhongMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_BlinnPhong
#define RE_IndirectDiffuse		RE_IndirectDiffuse_BlinnPhong`,lights_physical_fragment:`PhysicalMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.diffuseContribution = diffuseColor.rgb * ( 1.0 - metalnessFactor );
material.metalness = metalnessFactor;
vec3 dxy = max( abs( dFdx( nonPerturbedNormal ) ), abs( dFdy( nonPerturbedNormal ) ) );
float geometryRoughness = max( max( dxy.x, dxy.y ), dxy.z );
material.roughness = max( roughnessFactor, 0.0525 );material.roughness += geometryRoughness;
material.roughness = min( material.roughness, 1.0 );
#ifdef IOR
	material.ior = ior;
	#ifdef USE_SPECULAR
		float specularIntensityFactor = specularIntensity;
		vec3 specularColorFactor = specularColor;
		#ifdef USE_SPECULAR_COLORMAP
			specularColorFactor *= texture2D( specularColorMap, vSpecularColorMapUv ).rgb;
		#endif
		#ifdef USE_SPECULAR_INTENSITYMAP
			specularIntensityFactor *= texture2D( specularIntensityMap, vSpecularIntensityMapUv ).a;
		#endif
		material.specularF90 = mix( specularIntensityFactor, 1.0, metalnessFactor );
	#else
		float specularIntensityFactor = 1.0;
		vec3 specularColorFactor = vec3( 1.0 );
		material.specularF90 = 1.0;
	#endif
	material.specularColor = min( pow2( ( material.ior - 1.0 ) / ( material.ior + 1.0 ) ) * specularColorFactor, vec3( 1.0 ) ) * specularIntensityFactor;
	material.specularColorBlended = mix( material.specularColor, diffuseColor.rgb, metalnessFactor );
#else
	material.specularColor = vec3( 0.04 );
	material.specularColorBlended = mix( material.specularColor, diffuseColor.rgb, metalnessFactor );
	material.specularF90 = 1.0;
#endif
#ifdef USE_CLEARCOAT
	material.clearcoat = clearcoat;
	material.clearcoatRoughness = clearcoatRoughness;
	material.clearcoatF0 = vec3( 0.04 );
	material.clearcoatF90 = 1.0;
	#ifdef USE_CLEARCOATMAP
		material.clearcoat *= texture2D( clearcoatMap, vClearcoatMapUv ).x;
	#endif
	#ifdef USE_CLEARCOAT_ROUGHNESSMAP
		material.clearcoatRoughness *= texture2D( clearcoatRoughnessMap, vClearcoatRoughnessMapUv ).y;
	#endif
	material.clearcoat = saturate( material.clearcoat );	material.clearcoatRoughness = max( material.clearcoatRoughness, 0.0525 );
	material.clearcoatRoughness += geometryRoughness;
	material.clearcoatRoughness = min( material.clearcoatRoughness, 1.0 );
#endif
#ifdef USE_DISPERSION
	material.dispersion = dispersion;
#endif
#ifdef USE_RETROREFLECTION
	material.retroreflectivity = retroreflectivity;
#endif
#ifdef USE_IRIDESCENCE
	material.iridescence = iridescence;
	material.iridescenceIOR = iridescenceIOR;
	#ifdef USE_IRIDESCENCEMAP
		material.iridescence *= texture2D( iridescenceMap, vIridescenceMapUv ).r;
	#endif
	#ifdef USE_IRIDESCENCE_THICKNESSMAP
		material.iridescenceThickness = (iridescenceThicknessMaximum - iridescenceThicknessMinimum) * texture2D( iridescenceThicknessMap, vIridescenceThicknessMapUv ).g + iridescenceThicknessMinimum;
	#else
		material.iridescenceThickness = iridescenceThicknessMaximum;
	#endif
#endif
#ifdef USE_SHEEN
	material.sheenColor = sheenColor;
	#ifdef USE_SHEEN_COLORMAP
		material.sheenColor *= texture2D( sheenColorMap, vSheenColorMapUv ).rgb;
	#endif
	material.sheenRoughness = clamp( sheenRoughness, 0.0001, 1.0 );
	#ifdef USE_SHEEN_ROUGHNESSMAP
		material.sheenRoughness *= texture2D( sheenRoughnessMap, vSheenRoughnessMapUv ).a;
	#endif
#endif
#ifdef USE_ANISOTROPY
	#ifdef USE_ANISOTROPYMAP
		mat2 anisotropyMat = mat2( anisotropyVector.x, anisotropyVector.y, - anisotropyVector.y, anisotropyVector.x );
		vec3 anisotropyPolar = texture2D( anisotropyMap, vAnisotropyMapUv ).rgb;
		vec2 anisotropyV = anisotropyMat * normalize( 2.0 * anisotropyPolar.rg - vec2( 1.0 ) ) * anisotropyPolar.b;
	#else
		vec2 anisotropyV = anisotropyVector;
	#endif
	material.anisotropy = length( anisotropyV );
	if( material.anisotropy == 0.0 ) {
		anisotropyV = vec2( 1.0, 0.0 );
	} else {
		anisotropyV /= material.anisotropy;
		material.anisotropy = saturate( material.anisotropy );
	}
	material.alphaT = mix( pow2( material.roughness ), 1.0, pow2( material.anisotropy ) );
	material.anisotropyT = tbn[ 0 ] * anisotropyV.x + tbn[ 1 ] * anisotropyV.y;
	material.anisotropyB = tbn[ 1 ] * anisotropyV.x - tbn[ 0 ] * anisotropyV.y;
#endif`,lights_physical_pars_fragment:`uniform sampler2D dfgLUT;
struct PhysicalMaterial {
	vec3 diffuseColor;
	vec3 diffuseContribution;
	vec3 specularColor;
	vec3 specularColorBlended;
	float roughness;
	float metalness;
	float specularF90;
	float dispersion;
	vec2 dfg;
	vec3 multiScatteringCompensation;
	#ifdef USE_RETROREFLECTION
		float retroreflectivity;
	#endif
	#ifdef USE_CLEARCOAT
		float clearcoat;
		float clearcoatRoughness;
		vec3 clearcoatF0;
		float clearcoatF90;
	#endif
	#ifdef USE_IRIDESCENCE
		float iridescence;
		float iridescenceIOR;
		float iridescenceThickness;
		vec3 iridescenceFresnel;
		vec3 iridescenceF0Dielectric;
		vec3 iridescenceF0Metallic;
	#endif
	#ifdef USE_SHEEN
		vec3 sheenColor;
		float sheenRoughness;
	#endif
	#ifdef IOR
		float ior;
	#endif
	#ifdef USE_TRANSMISSION
		float transmission;
		float transmissionAlpha;
		float thickness;
		float attenuationDistance;
		vec3 attenuationColor;
	#endif
	#ifdef USE_ANISOTROPY
		float anisotropy;
		float alphaT;
		vec3 anisotropyT;
		vec3 anisotropyB;
	#endif
};
vec3 clearcoatSpecularDirect = vec3( 0.0 );
vec3 clearcoatSpecularIndirect = vec3( 0.0 );
vec3 sheenSpecularDirect = vec3( 0.0 );
vec3 sheenSpecularIndirect = vec3(0.0 );
vec3 Schlick_to_F0( const in vec3 f, const in float f90, const in float dotVH ) {
    float x = clamp( 1.0 - dotVH, 0.0, 1.0 );
    float x2 = x * x;
    float x5 = clamp( x * x2 * x2, 0.0, 0.9999 );
    return ( f - vec3( f90 ) * x5 ) / ( 1.0 - x5 );
}
float V_GGX_SmithCorrelated( const in float alpha, const in float dotNL, const in float dotNV ) {
	float a2 = pow2( alpha );
	float gv = dotNL * sqrt( a2 + ( 1.0 - a2 ) * pow2( dotNV ) );
	float gl = dotNV * sqrt( a2 + ( 1.0 - a2 ) * pow2( dotNL ) );
	return 0.5 / max( gv + gl, EPSILON );
}
float D_GGX( const in float alpha, const in float dotNH ) {
	float a2 = pow2( alpha );
	float denom = pow2( dotNH ) * ( a2 - 1.0 ) + 1.0;
	return RECIPROCAL_PI * a2 / pow2( denom );
}
#ifdef USE_ANISOTROPY
	float V_GGX_SmithCorrelated_Anisotropic( const in float alphaT, const in float alphaB, const in float dotTV, const in float dotBV, const in float dotTL, const in float dotBL, const in float dotNV, const in float dotNL ) {
		float gv = dotNL * length( vec3( alphaT * dotTV, alphaB * dotBV, dotNV ) );
		float gl = dotNV * length( vec3( alphaT * dotTL, alphaB * dotBL, dotNL ) );
		return 0.5 / max( gv + gl, EPSILON );
	}
	float D_GGX_Anisotropic( const in float alphaT, const in float alphaB, const in float dotNH, const in float dotTH, const in float dotBH ) {
		float a2 = alphaT * alphaB;
		highp vec3 v = vec3( alphaB * dotTH, alphaT * dotBH, a2 * dotNH );
		highp float v2 = dot( v, v );
		float w2 = a2 / v2;
		return RECIPROCAL_PI * a2 * pow2 ( w2 );
	}
#endif
#ifdef USE_CLEARCOAT
	vec3 BRDF_GGX_Clearcoat( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in PhysicalMaterial material) {
		vec3 f0 = material.clearcoatF0;
		float f90 = material.clearcoatF90;
		float roughness = material.clearcoatRoughness;
		float alpha = pow2( roughness );
		vec3 halfDir = normalize( lightDir + viewDir );
		float dotNL = saturate( dot( normal, lightDir ) );
		float dotNV = saturate( dot( normal, viewDir ) );
		float dotNH = saturate( dot( normal, halfDir ) );
		float dotVH = saturate( dot( viewDir, halfDir ) );
		vec3 F = F_Schlick( f0, f90, dotVH );
		float V = V_GGX_SmithCorrelated( alpha, dotNL, dotNV );
		float D = D_GGX( alpha, dotNH );
		return F * ( V * D );
	}
#endif
vec3 BRDF_GGX( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in PhysicalMaterial material ) {
	vec3 f0 = material.specularColorBlended;
	float f90 = material.specularF90;
	float roughness = material.roughness;
	float alpha = pow2( roughness );
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNL = saturate( dot( normal, lightDir ) );
	float dotNV = saturate( dot( normal, viewDir ) );
	float dotNH = saturate( dot( normal, halfDir ) );
	float dotVH = saturate( dot( viewDir, halfDir ) );
	vec3 F = F_Schlick( f0, f90, dotVH );
	#ifdef USE_IRIDESCENCE
		F = mix( F, material.iridescenceFresnel, material.iridescence );
	#endif
	#ifdef USE_ANISOTROPY
		float dotTL = dot( material.anisotropyT, lightDir );
		float dotTV = dot( material.anisotropyT, viewDir );
		float dotTH = dot( material.anisotropyT, halfDir );
		float dotBL = dot( material.anisotropyB, lightDir );
		float dotBV = dot( material.anisotropyB, viewDir );
		float dotBH = dot( material.anisotropyB, halfDir );
		float V = V_GGX_SmithCorrelated_Anisotropic( material.alphaT, alpha, dotTV, dotBV, dotTL, dotBL, dotNV, dotNL );
		float D = D_GGX_Anisotropic( material.alphaT, alpha, dotNH, dotTH, dotBH );
	#else
		float V = V_GGX_SmithCorrelated( alpha, dotNL, dotNV );
		float D = D_GGX( alpha, dotNH );
	#endif
	return F * ( V * D );
}
vec2 LTC_Uv( const in vec3 N, const in vec3 V, const in float roughness ) {
	const float LUT_SIZE = 64.0;
	const float LUT_SCALE = ( LUT_SIZE - 1.0 ) / LUT_SIZE;
	const float LUT_BIAS = 0.5 / LUT_SIZE;
	float dotNV = saturate( dot( N, V ) );
	vec2 uv = vec2( roughness, sqrt( 1.0 - dotNV ) );
	uv = uv * LUT_SCALE + LUT_BIAS;
	return uv;
}
float LTC_ClippedSphereFormFactor( const in vec3 f ) {
	float l = length( f );
	return max( ( l * l + f.z ) / ( l + 1.0 ), 0.0 );
}
vec3 LTC_EdgeVectorFormFactor( const in vec3 v1, const in vec3 v2 ) {
	float x = dot( v1, v2 );
	float y = abs( x );
	float a = 0.8543985 + ( 0.4965155 + 0.0145206 * y ) * y;
	float b = 3.4175940 + ( 4.1616724 + y ) * y;
	float v = a / b;
	float theta_sintheta = ( x > 0.0 ) ? v : 0.5 * inversesqrt( max( 1.0 - x * x, 1e-7 ) ) - v;
	return cross( v1, v2 ) * theta_sintheta;
}
vec3 LTC_Evaluate( const in vec3 N, const in vec3 V, const in vec3 P, const in mat3 mInv, const in vec3 rectCoords[ 4 ] ) {
	vec3 v1 = rectCoords[ 1 ] - rectCoords[ 0 ];
	vec3 v2 = rectCoords[ 3 ] - rectCoords[ 0 ];
	vec3 lightNormal = cross( v1, v2 );
	if( dot( lightNormal, P - rectCoords[ 0 ] ) < 0.0 ) return vec3( 0.0 );
	vec3 T1, T2;
	T1 = normalize( V - N * dot( V, N ) );
	T2 = - cross( N, T1 );
	mat3 mat = mInv * transpose( mat3( T1, T2, N ) );
	vec3 coords[ 4 ];
	coords[ 0 ] = mat * ( rectCoords[ 0 ] - P );
	coords[ 1 ] = mat * ( rectCoords[ 1 ] - P );
	coords[ 2 ] = mat * ( rectCoords[ 2 ] - P );
	coords[ 3 ] = mat * ( rectCoords[ 3 ] - P );
	coords[ 0 ] = normalize( coords[ 0 ] );
	coords[ 1 ] = normalize( coords[ 1 ] );
	coords[ 2 ] = normalize( coords[ 2 ] );
	coords[ 3 ] = normalize( coords[ 3 ] );
	vec3 vectorFormFactor = vec3( 0.0 );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 0 ], coords[ 1 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 1 ], coords[ 2 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 2 ], coords[ 3 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 3 ], coords[ 0 ] );
	float result = LTC_ClippedSphereFormFactor( vectorFormFactor );
	return vec3( result );
}
#if defined( USE_SHEEN )
float D_Charlie( float roughness, float dotNH ) {
	float alpha = pow2( roughness );
	float invAlpha = 1.0 / alpha;
	float cos2h = dotNH * dotNH;
	float sin2h = max( 1.0 - cos2h, 0.0078125 );
	return ( 2.0 + invAlpha ) * pow( sin2h, invAlpha * 0.5 ) / ( 2.0 * PI );
}
float V_Neubelt( float dotNV, float dotNL ) {
	return saturate( 1.0 / ( 4.0 * ( dotNL + dotNV - dotNL * dotNV ) ) );
}
vec3 BRDF_Sheen( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, vec3 sheenColor, const in float sheenRoughness ) {
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNL = saturate( dot( normal, lightDir ) );
	float dotNV = saturate( dot( normal, viewDir ) );
	float dotNH = saturate( dot( normal, halfDir ) );
	float D = D_Charlie( sheenRoughness, dotNH );
	float V = V_Neubelt( dotNV, dotNL );
	return sheenColor * ( D * V );
}
#endif
float IBLSheenBRDF( const in vec3 normal, const in vec3 viewDir, const in float roughness ) {
	float dotNV = saturate( dot( normal, viewDir ) );
	float r2 = roughness * roughness;
	float rInv = 1.0 / ( roughness + 0.1 );
	float a = -1.9362 + 1.0678 * roughness + 0.4573 * r2 - 0.8469 * rInv;
	float b = -0.6014 + 0.5538 * roughness - 0.4670 * r2 - 0.1255 * rInv;
	float DG = exp( a * dotNV + b );
	return saturate( DG );
}
vec3 EnvironmentBRDF( const in vec3 normal, const in vec3 viewDir, const in vec3 specularColor, const in float specularF90, const in float roughness ) {
	float dotNV = saturate( dot( normal, viewDir ) );
	vec2 fab = texture2D( dfgLUT, vec2( roughness, dotNV ) ).rg;
	return specularColor * fab.x + specularF90 * fab.y;
}
#ifdef USE_IRIDESCENCE
void computeMultiscatteringIridescence( const in vec2 fab, const in vec3 specularColor, const in float specularF90, const in float iridescence, const in vec3 iridescenceF0, inout vec3 singleScatter, inout vec3 multiScatter ) {
#else
void computeMultiscattering( const in vec2 fab, const in vec3 specularColor, const in float specularF90, inout vec3 singleScatter, inout vec3 multiScatter ) {
#endif
	#ifdef USE_IRIDESCENCE
		vec3 Fr = mix( specularColor, iridescenceF0, iridescence );
	#else
		vec3 Fr = specularColor;
	#endif
	vec3 FssEss = Fr * fab.x + specularF90 * fab.y;
	float Ess = fab.x + fab.y;
	float Ems = 1.0 - Ess;
	vec3 Favg = Fr + ( 1.0 - Fr ) * 0.047619;	vec3 Fms = FssEss * Favg / ( 1.0 - Ems * Favg );
	singleScatter += FssEss;
	multiScatter += Fms * Ems;
}
#if NUM_RECT_AREA_LIGHTS > 0
	void RE_Direct_RectArea_Physical( const in RectAreaLight rectAreaLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
		vec3 normal = geometryNormal;
		vec3 viewDir = geometryViewDir;
		vec3 position = geometryPosition;
		vec3 lightPos = rectAreaLight.position;
		vec3 halfWidth = rectAreaLight.halfWidth;
		vec3 halfHeight = rectAreaLight.halfHeight;
		vec3 lightColor = rectAreaLight.color;
		float roughness = material.roughness;
		vec3 rectCoords[ 4 ];
		rectCoords[ 0 ] = lightPos + halfWidth - halfHeight;		rectCoords[ 1 ] = lightPos - halfWidth - halfHeight;
		rectCoords[ 2 ] = lightPos - halfWidth + halfHeight;
		rectCoords[ 3 ] = lightPos + halfWidth + halfHeight;
		vec2 uv = LTC_Uv( normal, viewDir, roughness );
		vec4 t1 = texture2D( ltc_1, uv );
		vec4 t2 = texture2D( ltc_2, uv );
		mat3 mInv = mat3(
			vec3( t1.x, 0, t1.y ),
			vec3(    0, 1,    0 ),
			vec3( t1.z, 0, t1.w )
		);
		vec3 fresnel = ( material.specularColorBlended * t2.x + ( material.specularF90 - material.specularColorBlended ) * t2.y );
		reflectedLight.directSpecular += lightColor * fresnel * LTC_Evaluate( normal, viewDir, position, mInv, rectCoords );
		reflectedLight.directDiffuse += lightColor * material.diffuseContribution * LTC_Evaluate( normal, viewDir, position, mat3( 1.0 ), rectCoords );
		#ifdef USE_CLEARCOAT
			vec3 Ncc = geometryClearcoatNormal;
			vec2 uvClearcoat = LTC_Uv( Ncc, viewDir, material.clearcoatRoughness );
			vec4 t1Clearcoat = texture2D( ltc_1, uvClearcoat );
			vec4 t2Clearcoat = texture2D( ltc_2, uvClearcoat );
			mat3 mInvClearcoat = mat3(
				vec3( t1Clearcoat.x, 0, t1Clearcoat.y ),
				vec3(             0, 1,             0 ),
				vec3( t1Clearcoat.z, 0, t1Clearcoat.w )
			);
			vec3 fresnelClearcoat = material.clearcoatF0 * t2Clearcoat.x + ( material.clearcoatF90 - material.clearcoatF0 ) * t2Clearcoat.y;
			clearcoatSpecularDirect += lightColor * fresnelClearcoat * LTC_Evaluate( Ncc, viewDir, position, mInvClearcoat, rectCoords );
		#endif
	}
#endif
void RE_Direct_Physical( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	#ifdef USE_CLEARCOAT
		float dotNLcc = saturate( dot( geometryClearcoatNormal, directLight.direction ) );
		vec3 ccIrradiance = dotNLcc * directLight.color;
		clearcoatSpecularDirect += ccIrradiance * BRDF_GGX_Clearcoat( directLight.direction, geometryViewDir, geometryClearcoatNormal, material );
	#endif
	#ifdef USE_SHEEN
 
 		sheenSpecularDirect += irradiance * BRDF_Sheen( directLight.direction, geometryViewDir, geometryNormal, material.sheenColor, material.sheenRoughness );
 
 		float sheenAlbedoV = IBLSheenBRDF( geometryNormal, geometryViewDir, material.sheenRoughness );
 		float sheenAlbedoL = IBLSheenBRDF( geometryNormal, directLight.direction, material.sheenRoughness );
 
 		float sheenEnergyComp = 1.0 - max3( material.sheenColor ) * max( sheenAlbedoV, sheenAlbedoL );
 
 		irradiance *= sheenEnergyComp;
 
 	#endif
	vec3 specularBRDF = BRDF_GGX( directLight.direction, geometryViewDir, geometryNormal, material );
	#ifdef USE_RETROREFLECTION
		vec3 retroViewDir = reflect( - geometryViewDir, geometryNormal );
		vec3 retroSpecularBRDF = BRDF_GGX( directLight.direction, retroViewDir, geometryNormal, material );
		specularBRDF = mix( specularBRDF, retroSpecularBRDF, saturate( material.retroreflectivity ) );
	#endif
	reflectedLight.directSpecular += irradiance * specularBRDF * material.multiScatteringCompensation;
	vec3 halfDir = normalize( directLight.direction + geometryViewDir );
	float dotVH = saturate( dot( geometryViewDir, halfDir ) );
	vec3 F = F_Schlick( material.specularColor, material.specularF90, dotVH );
	#ifdef USE_RETROREFLECTION
		vec3 retroHalfDir = normalize( directLight.direction + retroViewDir );
		float dotRetroVH = saturate( dot( retroViewDir, retroHalfDir ) );
		vec3 retroF = F_Schlick( material.specularColor, material.specularF90, dotRetroVH );
		F = mix( F, retroF, saturate( material.retroreflectivity ) );
	#endif
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseContribution ) * ( 1.0 - F );
}
void RE_IndirectDiffuse_Physical( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
	vec3 singleScattering = vec3( 0.0 );
	vec3 multiScattering = vec3( 0.0 );
	#ifdef USE_IRIDESCENCE
		computeMultiscatteringIridescence( material.dfg, material.specularColor, material.specularF90, material.iridescence, material.iridescenceF0Dielectric, singleScattering, multiScattering );
	#else
		computeMultiscattering( material.dfg, material.specularColor, material.specularF90, singleScattering, multiScattering );
	#endif
	vec3 diffuse = irradiance * BRDF_Lambert( material.diffuseContribution ) * ( 1.0 - singleScattering - multiScattering );
	#ifdef USE_SHEEN
		float sheenAlbedo = IBLSheenBRDF( geometryNormal, geometryViewDir, material.sheenRoughness );
		sheenSpecularIndirect += irradiance * material.sheenColor * sheenAlbedo * RECIPROCAL_PI;
		float sheenEnergyComp = 1.0 - max3( material.sheenColor ) * sheenAlbedo;
		diffuse *= sheenEnergyComp;
	#endif
	reflectedLight.indirectDiffuse += diffuse;
}
void RE_IndirectSpecular_Physical( const in vec3 radiance, const in vec3 irradiance, const in vec3 clearcoatRadiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight) {
	#ifdef USE_CLEARCOAT
		clearcoatSpecularIndirect += clearcoatRadiance * EnvironmentBRDF( geometryClearcoatNormal, geometryViewDir, material.clearcoatF0, material.clearcoatF90, material.clearcoatRoughness );
	#endif
	#ifdef USE_SHEEN
		sheenSpecularIndirect += irradiance * material.sheenColor * IBLSheenBRDF( geometryNormal, geometryViewDir, material.sheenRoughness ) * RECIPROCAL_PI;
 	#endif
	vec3 singleScatteringDielectric = vec3( 0.0 );
	vec3 multiScatteringDielectric = vec3( 0.0 );
	vec3 singleScatteringMetallic = vec3( 0.0 );
	vec3 multiScatteringMetallic = vec3( 0.0 );
	#ifdef USE_IRIDESCENCE
		computeMultiscatteringIridescence( material.dfg, material.specularColor, material.specularF90, material.iridescence, material.iridescenceF0Dielectric, singleScatteringDielectric, multiScatteringDielectric );
		computeMultiscatteringIridescence( material.dfg, material.diffuseColor, material.specularF90, material.iridescence, material.iridescenceF0Metallic, singleScatteringMetallic, multiScatteringMetallic );
	#else
		computeMultiscattering( material.dfg, material.specularColor, material.specularF90, singleScatteringDielectric, multiScatteringDielectric );
		computeMultiscattering( material.dfg, material.diffuseColor, material.specularF90, singleScatteringMetallic, multiScatteringMetallic );
	#endif
	vec3 singleScattering = mix( singleScatteringDielectric, singleScatteringMetallic, material.metalness );
	vec3 multiScattering = mix( multiScatteringDielectric, multiScatteringMetallic, material.metalness );
	vec3 totalScatteringDielectric = singleScatteringDielectric + multiScatteringDielectric;
	vec3 diffuse = material.diffuseContribution * ( 1.0 - totalScatteringDielectric );
	vec3 cosineWeightedIrradiance = irradiance * RECIPROCAL_PI;
	vec3 indirectSpecular = radiance * singleScattering;
	indirectSpecular += multiScattering * cosineWeightedIrradiance;
	vec3 indirectDiffuse = diffuse * cosineWeightedIrradiance;
	#ifdef USE_SHEEN
		float sheenAlbedo = IBLSheenBRDF( geometryNormal, geometryViewDir, material.sheenRoughness );
		float sheenEnergyComp = 1.0 - max3( material.sheenColor ) * sheenAlbedo;
		indirectSpecular *= sheenEnergyComp;
		indirectDiffuse *= sheenEnergyComp;
	#endif
	reflectedLight.indirectSpecular += indirectSpecular;
	reflectedLight.indirectDiffuse += indirectDiffuse;
}
#define RE_Direct				RE_Direct_Physical
#define RE_Direct_RectArea		RE_Direct_RectArea_Physical
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Physical
#define RE_IndirectSpecular		RE_IndirectSpecular_Physical
float computeSpecularOcclusion( const in float dotNV, const in float ambientOcclusion, const in float roughness ) {
	return saturate( pow( dotNV + ambientOcclusion, exp2( - 16.0 * roughness - 1.0 ) ) - 1.0 + ambientOcclusion );
}`,lights_fragment_begin:`
vec3 geometryPosition = - vViewPosition;
vec3 geometryNormal = normal;
vec3 geometryViewDir = ( isOrthographic ) ? vec3( 0, 0, 1 ) : normalize( vViewPosition );
vec3 geometryClearcoatNormal = vec3( 0.0 );
#ifdef USE_CLEARCOAT
	geometryClearcoatNormal = clearcoatNormal;
#endif
#ifdef USE_IRIDESCENCE
	float dotNVi = saturate( dot( normal, geometryViewDir ) );
	if ( material.iridescenceThickness == 0.0 ) {
		material.iridescence = 0.0;
	} else {
		material.iridescence = saturate( material.iridescence );
	}
	if ( material.iridescence > 0.0 ) {
		vec3 iridescenceFresnelDielectric = evalIridescence( 1.0, material.iridescenceIOR, dotNVi, material.iridescenceThickness, material.specularColor );
		vec3 iridescenceFresnelMetallic = evalIridescence( 1.0, material.iridescenceIOR, dotNVi, material.iridescenceThickness, material.diffuseColor );
		material.iridescenceFresnel = mix( iridescenceFresnelDielectric, iridescenceFresnelMetallic, material.metalness );
		material.iridescenceF0Dielectric = Schlick_to_F0( iridescenceFresnelDielectric, 1.0, dotNVi );
		material.iridescenceF0Metallic = Schlick_to_F0( iridescenceFresnelMetallic, 1.0, dotNVi );
	}
#endif
#ifdef STANDARD
	float dotNVms = saturate( dot( geometryNormal, geometryViewDir ) );
	material.dfg = texture2D( dfgLUT, vec2( material.roughness, dotNVms ) ).rg;
	#if ( NUM_SUN_LIGHTS > 0 || NUM_DIR_LIGHTS > 0 || NUM_POINT_LIGHTS > 0 || NUM_SPOT_LIGHTS > 0 )
		float EssMs = material.dfg.x + material.dfg.y;
		material.multiScatteringCompensation = 1.0 + material.specularColorBlended * ( 1.0 / EssMs - 1.0 );
	#endif
#endif
IncidentLight directLight;
#if ( NUM_POINT_LIGHTS > 0 ) && defined( RE_Direct )
	PointLight pointLight;
	#if defined( USE_SHADOWMAP ) && NUM_POINT_LIGHT_SHADOWS > 0
	PointLightShadow pointLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_POINT_LIGHTS; i ++ ) {
		pointLight = pointLights[ i ];
		getPointLightInfo( pointLight, geometryPosition, directLight );
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_POINT_LIGHT_SHADOWS ) && ( defined( SHADOWMAP_TYPE_PCF ) || defined( SHADOWMAP_TYPE_BASIC ) )
		pointLightShadow = pointLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getPointShadow( pointShadowMap[ i ], pointLightShadow.shadowMapSize, pointLightShadow.shadowIntensity, pointLightShadow.shadowBias, pointLightShadow.shadowRadius, vPointShadowCoord[ i ], pointLightShadow.shadowCameraNear, pointLightShadow.shadowCameraFar ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_SPOT_LIGHTS > 0 ) && defined( RE_Direct )
	SpotLight spotLight;
	vec4 spotColor;
	vec3 spotLightCoord;
	bool inSpotLightMap;
	#if defined( USE_SHADOWMAP ) && NUM_SPOT_LIGHT_SHADOWS > 0
	SpotLightShadow spotLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHTS; i ++ ) {
		spotLight = spotLights[ i ];
		getSpotLightInfo( spotLight, geometryPosition, directLight );
		#if ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS )
		#define SPOT_LIGHT_MAP_INDEX UNROLLED_LOOP_INDEX
		#elif ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
		#define SPOT_LIGHT_MAP_INDEX NUM_SPOT_LIGHT_MAPS
		#else
		#define SPOT_LIGHT_MAP_INDEX ( UNROLLED_LOOP_INDEX - NUM_SPOT_LIGHT_SHADOWS + NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS )
		#endif
		#if ( SPOT_LIGHT_MAP_INDEX < NUM_SPOT_LIGHT_MAPS )
			spotLightCoord = vSpotLightCoord[ i ].xyz / vSpotLightCoord[ i ].w;
			inSpotLightMap = all( lessThan( abs( spotLightCoord * 2. - 1. ), vec3( 1.0 ) ) );
			spotColor = texture2D( spotLightMap[ SPOT_LIGHT_MAP_INDEX ], spotLightCoord.xy );
			directLight.color = inSpotLightMap ? directLight.color * spotColor.rgb : directLight.color;
		#endif
		#undef SPOT_LIGHT_MAP_INDEX
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
		spotLightShadow = spotLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getShadow( spotShadowMap[ i ], spotLightShadow.shadowMapSize, spotLightShadow.shadowIntensity, spotLightShadow.shadowBias, spotLightShadow.shadowRadius, vSpotLightCoord[ i ] ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_SUN_LIGHTS > 0 ) && defined( RE_Direct )
	SunLight sunLight;
	#if defined( USE_SHADOWMAP ) && NUM_SUN_LIGHT_SHADOWS > 0
	SunLightShadow sunLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SUN_LIGHTS; i ++ ) {
		sunLight = sunLights[ i ];
		getSunLightInfo( sunLight, directLight );
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_SUN_LIGHT_SHADOWS )
		sunLightShadow = sunLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getSunShadow( sunShadowMap[ i ], sunLightShadow, UNROLLED_LOOP_INDEX ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_DIR_LIGHTS > 0 ) && defined( RE_Direct )
	DirectionalLight directionalLight;
	#if defined( USE_SHADOWMAP ) && NUM_DIR_LIGHT_SHADOWS > 0
	DirectionalLightShadow directionalLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_DIR_LIGHTS; i ++ ) {
		directionalLight = directionalLights[ i ];
		getDirectionalLightInfo( directionalLight, directLight );
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_DIR_LIGHT_SHADOWS )
		directionalLightShadow = directionalLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getShadow( directionalShadowMap[ i ], directionalLightShadow.shadowMapSize, directionalLightShadow.shadowIntensity, directionalLightShadow.shadowBias, directionalLightShadow.shadowRadius, vDirectionalShadowCoord[ i ] ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_RECT_AREA_LIGHTS > 0 ) && defined( RE_Direct_RectArea )
	RectAreaLight rectAreaLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_RECT_AREA_LIGHTS; i ++ ) {
		rectAreaLight = rectAreaLights[ i ];
		RE_Direct_RectArea( rectAreaLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if defined( RE_IndirectDiffuse )
	vec3 iblIrradiance = vec3( 0.0 );
	vec3 irradiance = getAmbientLightIrradiance( ambientLightColor );
	#if defined( USE_LIGHT_PROBES )
		irradiance += getLightProbeIrradiance( lightProbe, geometryNormal );
	#endif
	#if ( NUM_HEMI_LIGHTS > 0 )
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_HEMI_LIGHTS; i ++ ) {
			irradiance += getHemisphereLightIrradiance( hemisphereLights[ i ], geometryNormal );
		}
		#pragma unroll_loop_end
	#endif
	#ifdef USE_LIGHT_PROBES_GRID
		vec3 probeWorldPos = ( ( vec4( geometryPosition, 1.0 ) - viewMatrix[ 3 ] ) * viewMatrix ).xyz;
		vec3 probeWorldNormal = transformNormalByInverseViewMatrix( geometryNormal, viewMatrix );
		irradiance += getLightProbeGridIrradiance( probeWorldPos, probeWorldNormal );
	#endif
#endif
#if defined( RE_IndirectSpecular )
	vec3 radiance = vec3( 0.0 );
	vec3 clearcoatRadiance = vec3( 0.0 );
#endif`,lights_fragment_maps:`#if defined( RE_IndirectDiffuse )
	#ifdef USE_LIGHTMAP
		vec4 lightMapTexel = texture2D( lightMap, vLightMapUv );
		vec3 lightMapIrradiance = lightMapTexel.rgb * lightMapIntensity;
		irradiance += lightMapIrradiance;
	#endif
	#if defined( USE_ENVMAP ) && defined( ENVMAP_TYPE_CUBE_UV )
		#if defined( STANDARD ) || defined( LAMBERT ) || defined( PHONG )
			iblIrradiance += getIBLIrradiance( geometryNormal );
		#endif
	#endif
#endif
#if defined( USE_ENVMAP ) && defined( RE_IndirectSpecular )
	#ifdef USE_ANISOTROPY
		vec3 iblRadiance = getIBLAnisotropyRadiance( geometryViewDir, geometryNormal, material.roughness, material.anisotropyB, material.anisotropy );
	#else
		vec3 iblRadiance = getIBLRadiance( geometryViewDir, geometryNormal, material.roughness );
	#endif
	#ifdef USE_RETROREFLECTION
		#ifdef USE_ANISOTROPY
			vec3 retroIBLRadiance = getIBLAnisotropyRetroRadiance( geometryViewDir, geometryNormal, material.roughness, material.anisotropyB, material.anisotropy );
		#else
			vec3 retroIBLRadiance = getIBLRetroRadiance( geometryViewDir, geometryNormal, material.roughness );
		#endif
		iblRadiance = mix( iblRadiance, retroIBLRadiance, saturate( material.retroreflectivity ) );
	#endif
	radiance += iblRadiance;
	#ifdef USE_CLEARCOAT
		clearcoatRadiance += getIBLRadiance( geometryViewDir, geometryClearcoatNormal, material.clearcoatRoughness );
	#endif
#endif`,lights_fragment_end:`#if defined( RE_IndirectDiffuse )
	#if defined( LAMBERT ) || defined( PHONG )
		irradiance += iblIrradiance;
	#endif
	RE_IndirectDiffuse( irradiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif
#if defined( RE_IndirectSpecular )
	RE_IndirectSpecular( radiance, iblIrradiance, clearcoatRadiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif`,lightprobes_pars_fragment:`#ifdef USE_LIGHT_PROBES_GRID
uniform highp sampler3D probesSH;
uniform vec3 probesMin;
uniform vec3 probesMax;
uniform vec3 probesResolution;
vec3 getLightProbeGridIrradiance( vec3 worldPos, vec3 worldNormal ) {
	vec3 res = probesResolution;
	vec3 gridRange = probesMax - probesMin;
	vec3 resMinusOne = res - 1.0;
	vec3 probeSpacing = gridRange / resMinusOne;
	vec3 samplePos = worldPos + worldNormal * probeSpacing * 0.5;
	vec3 uvw = clamp( ( samplePos - probesMin ) / gridRange, 0.0, 1.0 );
	uvw = uvw * resMinusOne / res + 0.5 / res;
	float nz          = res.z;
	float paddedSlices = nz + 2.0;
	float atlasDepth  = 7.0 * paddedSlices;
	float uvZBase     = uvw.z * nz + 1.0;
	vec4 s0 = texture( probesSH, vec3( uvw.xy, ( uvZBase                       ) / atlasDepth ) );
	vec4 s1 = texture( probesSH, vec3( uvw.xy, ( uvZBase +       paddedSlices   ) / atlasDepth ) );
	vec4 s2 = texture( probesSH, vec3( uvw.xy, ( uvZBase + 2.0 * paddedSlices   ) / atlasDepth ) );
	vec4 s3 = texture( probesSH, vec3( uvw.xy, ( uvZBase + 3.0 * paddedSlices   ) / atlasDepth ) );
	vec4 s4 = texture( probesSH, vec3( uvw.xy, ( uvZBase + 4.0 * paddedSlices   ) / atlasDepth ) );
	vec4 s5 = texture( probesSH, vec3( uvw.xy, ( uvZBase + 5.0 * paddedSlices   ) / atlasDepth ) );
	vec4 s6 = texture( probesSH, vec3( uvw.xy, ( uvZBase + 6.0 * paddedSlices   ) / atlasDepth ) );
	vec3 c0 = s0.xyz;
	vec3 c1 = vec3( s0.w, s1.xy );
	vec3 c2 = vec3( s1.zw, s2.x );
	vec3 c3 = s2.yzw;
	vec3 c4 = s3.xyz;
	vec3 c5 = vec3( s3.w, s4.xy );
	vec3 c6 = vec3( s4.zw, s5.x );
	vec3 c7 = s5.yzw;
	vec3 c8 = s6.xyz;
	float x = worldNormal.x, y = worldNormal.y, z = worldNormal.z;
	vec3 result = c0 * 0.886227;
	result += c1 * 2.0 * 0.511664 * y;
	result += c2 * 2.0 * 0.511664 * z;
	result += c3 * 2.0 * 0.511664 * x;
	result += c4 * 2.0 * 0.429043 * x * y;
	result += c5 * 2.0 * 0.429043 * y * z;
	result += c6 * ( 0.743125 * z * z - 0.247708 );
	result += c7 * 2.0 * 0.429043 * x * z;
	result += c8 * 0.429043 * ( x * x - y * y );
	return max( result, vec3( 0.0 ) );
}
#endif`,logdepthbuf_fragment:`#if defined( USE_LOGARITHMIC_DEPTH_BUFFER )
	gl_FragDepth = vIsPerspective == 0.0 ? gl_FragCoord.z : log2( vFragDepth ) * logDepthBufFC * 0.5;
#endif`,logdepthbuf_pars_fragment:`#if defined( USE_LOGARITHMIC_DEPTH_BUFFER )
	uniform float logDepthBufFC;
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,logdepthbuf_pars_vertex:`#ifdef USE_LOGARITHMIC_DEPTH_BUFFER
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,logdepthbuf_vertex:`#ifdef USE_LOGARITHMIC_DEPTH_BUFFER
	vFragDepth = 1.0 + gl_Position.w;
	vIsPerspective = float( isPerspectiveMatrix( projectionMatrix ) );
#endif`,map_fragment:`#ifdef USE_MAP
	vec4 sampledDiffuseColor = texture2D( map, vMapUv );
	#ifdef DECODE_VIDEO_TEXTURE
		sampledDiffuseColor = sRGBTransferEOTF( sampledDiffuseColor );
	#endif
	diffuseColor *= sampledDiffuseColor;
#endif`,map_pars_fragment:`#ifdef USE_MAP
	uniform sampler2D map;
#endif`,map_particle_fragment:`#if defined( USE_MAP ) || defined( USE_ALPHAMAP )
	#if defined( USE_POINTS_UV )
		vec2 uv = vUv;
	#else
		vec2 uv = ( uvTransform * vec3( gl_PointCoord.x, 1.0 - gl_PointCoord.y, 1 ) ).xy;
	#endif
#endif
#ifdef USE_MAP
	diffuseColor *= texture2D( map, uv );
#endif
#ifdef USE_ALPHAMAP
	diffuseColor.a *= texture2D( alphaMap, uv ).g;
#endif`,map_particle_pars_fragment:`#if defined( USE_POINTS_UV )
	varying vec2 vUv;
#else
	#if defined( USE_MAP ) || defined( USE_ALPHAMAP )
		uniform mat3 uvTransform;
	#endif
#endif
#ifdef USE_MAP
	uniform sampler2D map;
#endif
#ifdef USE_ALPHAMAP
	uniform sampler2D alphaMap;
#endif`,metalnessmap_fragment:`float metalnessFactor = metalness;
#ifdef USE_METALNESSMAP
	vec4 texelMetalness = texture2D( metalnessMap, vMetalnessMapUv );
	metalnessFactor *= texelMetalness.b;
#endif`,metalnessmap_pars_fragment:`#ifdef USE_METALNESSMAP
	uniform sampler2D metalnessMap;
#endif`,morphinstance_vertex:`#ifdef USE_INSTANCING_MORPH
	float morphTargetInfluences[ MORPHTARGETS_COUNT ];
	float morphTargetBaseInfluence = texelFetch( morphTexture, ivec2( 0, gl_InstanceID ), 0 ).r;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		morphTargetInfluences[i] =  texelFetch( morphTexture, ivec2( i + 1, gl_InstanceID ), 0 ).r;
	}
#endif`,morphcolor_vertex:`#if defined( USE_MORPHCOLORS )
	vColor *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		#if defined( USE_COLOR_ALPHA )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ) * morphTargetInfluences[ i ];
		#elif defined( USE_COLOR )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ).rgb * morphTargetInfluences[ i ];
		#endif
	}
#endif`,morphnormal_vertex:`#ifdef USE_MORPHNORMALS
	objectNormal *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		if ( morphTargetInfluences[ i ] != 0.0 ) objectNormal += getMorph( gl_VertexID, i, 1 ).xyz * morphTargetInfluences[ i ];
	}
#endif`,morphtarget_pars_vertex:`#ifdef USE_MORPHTARGETS
	#ifndef USE_INSTANCING_MORPH
		uniform float morphTargetBaseInfluence;
		uniform float morphTargetInfluences[ MORPHTARGETS_COUNT ];
	#endif
	uniform sampler2DArray morphTargetsTexture;
	uniform ivec2 morphTargetsTextureSize;
	vec4 getMorph( const in int vertexIndex, const in int morphTargetIndex, const in int offset ) {
		int texelIndex = vertexIndex * MORPHTARGETS_TEXTURE_STRIDE + offset;
		int y = texelIndex / morphTargetsTextureSize.x;
		int x = texelIndex - y * morphTargetsTextureSize.x;
		ivec3 morphUV = ivec3( x, y, morphTargetIndex );
		return texelFetch( morphTargetsTexture, morphUV, 0 );
	}
#endif`,morphtarget_vertex:`#ifdef USE_MORPHTARGETS
	transformed *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		if ( morphTargetInfluences[ i ] != 0.0 ) transformed += getMorph( gl_VertexID, i, 0 ).xyz * morphTargetInfluences[ i ];
	}
#endif`,normal_fragment_begin:`float faceDirection = gl_FrontFacing ? 1.0 : - 1.0;
#ifdef FLAT_SHADED
	vec3 fdx = dFdx( vViewPosition );
	vec3 fdy = dFdy( vViewPosition );
	vec3 normal = normalize( cross( fdx, fdy ) );
#else
	vec3 normal = normalize( vNormal );
	#ifdef DOUBLE_SIDED
		normal *= faceDirection;
	#endif
#endif
#if defined( USE_NORMALMAP_TANGENTSPACE ) || defined( USE_CLEARCOAT_NORMALMAP ) || defined( USE_ANISOTROPY )
	#ifdef USE_TANGENT
		mat3 tbn = mat3( normalize( vTangent ), normalize( vBitangent ), normal );
	#else
		mat3 tbn = getTangentFrame( - vViewPosition, normal,
		#if defined( USE_NORMALMAP )
			vNormalMapUv
		#elif defined( USE_CLEARCOAT_NORMALMAP )
			vClearcoatNormalMapUv
		#else
			vUv
		#endif
		);
	#endif
	#ifdef DOUBLE_SIDED
		tbn[0] *= faceDirection;
		tbn[1] *= faceDirection;
	#endif
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	#ifdef USE_TANGENT
		mat3 tbn2 = mat3( normalize( vTangent ), normalize( vBitangent ), normal );
	#else
		mat3 tbn2 = getTangentFrame( - vViewPosition, normal, vClearcoatNormalMapUv );
	#endif
	#ifdef DOUBLE_SIDED
		tbn2[0] *= faceDirection;
		tbn2[1] *= faceDirection;
	#endif
#endif
vec3 nonPerturbedNormal = normal;`,normal_fragment_maps:`#ifdef USE_NORMALMAP_OBJECTSPACE
	normal = texture2D( normalMap, vNormalMapUv ).xyz * 2.0 - 1.0;
	#ifdef FLIP_SIDED
		normal = - normal;
	#endif
	#ifdef DOUBLE_SIDED
		normal = normal * faceDirection;
	#endif
	normal = normalize( normalMatrix * normal );
#elif defined( USE_NORMALMAP_TANGENTSPACE )
	vec3 mapN = texture2D( normalMap, vNormalMapUv ).xyz * 2.0 - 1.0;
	#if defined( USE_PACKED_NORMALMAP )
		mapN = vec3( mapN.xy, sqrt( saturate( 1.0 - dot( mapN.xy, mapN.xy ) ) ) );
	#endif
	mapN.xy *= normalScale;
	normal = normalize( tbn * mapN );
#elif defined( USE_BUMPMAP )
	normal = perturbNormalArb( - vViewPosition, normal, dHdxy_fwd(), faceDirection );
#endif`,normal_pars_fragment:`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,normal_pars_vertex:`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,normal_vertex:`#ifndef FLAT_SHADED
	vNormal = normalize( transformedNormal );
	#ifdef USE_TANGENT
		vTangent = normalize( transformedTangent );
		vBitangent = normalize( cross( vNormal, vTangent ) * tangent.w );
		#ifdef FLIP_SIDED
			vBitangent = - vBitangent;
		#endif
	#endif
#endif`,normalmap_pars_fragment:`#ifdef USE_NORMALMAP
	uniform sampler2D normalMap;
	uniform vec2 normalScale;
#endif
#ifdef USE_NORMALMAP_OBJECTSPACE
	uniform mat3 normalMatrix;
#endif
#if ! defined ( USE_TANGENT ) && ( defined ( USE_NORMALMAP_TANGENTSPACE ) || defined ( USE_CLEARCOAT_NORMALMAP ) || defined( USE_ANISOTROPY ) )
	mat3 getTangentFrame( vec3 eye_pos, vec3 surf_norm, vec2 uv ) {
		vec3 q0 = dFdx( eye_pos.xyz );
		vec3 q1 = dFdy( eye_pos.xyz );
		vec2 st0 = dFdx( uv.st );
		vec2 st1 = dFdy( uv.st );
		vec3 N = surf_norm;
		vec3 q1perp = cross( q1, N );
		vec3 q0perp = cross( N, q0 );
		vec3 T = q1perp * st0.x + q0perp * st1.x;
		vec3 B = q1perp * st0.y + q0perp * st1.y;
		float det = max( dot( T, T ), dot( B, B ) );
		float scale = ( det == 0.0 ) ? 0.0 : inversesqrt( det );
		return mat3( T * scale, B * scale, N );
	}
#endif`,clearcoat_normal_fragment_begin:`#ifdef USE_CLEARCOAT
	vec3 clearcoatNormal = nonPerturbedNormal;
#endif`,clearcoat_normal_fragment_maps:`#ifdef USE_CLEARCOAT_NORMALMAP
	vec3 clearcoatMapN = texture2D( clearcoatNormalMap, vClearcoatNormalMapUv ).xyz * 2.0 - 1.0;
	clearcoatMapN.xy *= clearcoatNormalScale;
	clearcoatNormal = normalize( tbn2 * clearcoatMapN );
#endif`,clearcoat_pars_fragment:`#ifdef USE_CLEARCOATMAP
	uniform sampler2D clearcoatMap;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	uniform sampler2D clearcoatNormalMap;
	uniform vec2 clearcoatNormalScale;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	uniform sampler2D clearcoatRoughnessMap;
#endif`,iridescence_pars_fragment:`#ifdef USE_IRIDESCENCEMAP
	uniform sampler2D iridescenceMap;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	uniform sampler2D iridescenceThicknessMap;
#endif`,opaque_fragment:`#ifdef OPAQUE
diffuseColor.a = 1.0;
#endif
#ifdef USE_TRANSMISSION
diffuseColor.a *= material.transmissionAlpha;
#endif
gl_FragColor = vec4( outgoingLight, diffuseColor.a );`,packing:`vec3 packNormalToRGB( const in vec3 normal ) {
	return normalize( normal ) * 0.5 + 0.5;
}
vec3 unpackRGBToNormal( const in vec3 rgb ) {
	return 2.0 * rgb.xyz - 1.0;
}
const float PackUpscale = 256. / 255.;const float UnpackDownscale = 255. / 256.;const float ShiftRight8 = 1. / 256.;
const float Inv255 = 1. / 255.;
const vec4 PackFactors = vec4( 1.0, 256.0, 256.0 * 256.0, 256.0 * 256.0 * 256.0 );
const vec2 UnpackFactors2 = vec2( UnpackDownscale, 1.0 / PackFactors.g );
const vec3 UnpackFactors3 = vec3( UnpackDownscale / PackFactors.rg, 1.0 / PackFactors.b );
const vec4 UnpackFactors4 = vec4( UnpackDownscale / PackFactors.rgb, 1.0 / PackFactors.a );
vec4 packDepthToRGBA( const in float v ) {
	if( v <= 0.0 )
		return vec4( 0., 0., 0., 0. );
	if( v >= 1.0 )
		return vec4( 1., 1., 1., 1. );
	float vuf;
	float af = modf( v * PackFactors.a, vuf );
	float bf = modf( vuf * ShiftRight8, vuf );
	float gf = modf( vuf * ShiftRight8, vuf );
	return vec4( vuf * Inv255, gf * PackUpscale, bf * PackUpscale, af );
}
vec3 packDepthToRGB( const in float v ) {
	if( v <= 0.0 )
		return vec3( 0., 0., 0. );
	if( v >= 1.0 )
		return vec3( 1., 1., 1. );
	float vuf;
	float bf = modf( v * PackFactors.b, vuf );
	float gf = modf( vuf * ShiftRight8, vuf );
	return vec3( vuf * Inv255, gf * PackUpscale, bf );
}
vec2 packDepthToRG( const in float v ) {
	if( v <= 0.0 )
		return vec2( 0., 0. );
	if( v >= 1.0 )
		return vec2( 1., 1. );
	float vuf;
	float gf = modf( v * 256., vuf );
	return vec2( vuf * Inv255, gf );
}
float unpackRGBAToDepth( const in vec4 v ) {
	return dot( v, UnpackFactors4 );
}
float unpackRGBToDepth( const in vec3 v ) {
	return dot( v, UnpackFactors3 );
}
float unpackRGToDepth( const in vec2 v ) {
	return v.r * UnpackFactors2.r + v.g * UnpackFactors2.g;
}
vec4 pack2HalfToRGBA( const in vec2 v ) {
	vec4 r = vec4( v.x, fract( v.x * 255.0 ), v.y, fract( v.y * 255.0 ) );
	return vec4( r.x - r.y / 255.0, r.y, r.z - r.w / 255.0, r.w );
}
vec2 unpackRGBATo2Half( const in vec4 v ) {
	return vec2( v.x + ( v.y / 255.0 ), v.z + ( v.w / 255.0 ) );
}
float viewZToOrthographicDepth( const in float viewZ, const in float near, const in float far ) {
	return ( viewZ + near ) / ( near - far );
}
float orthographicDepthToViewZ( const in float depth, const in float near, const in float far ) {
	#ifdef USE_REVERSED_DEPTH_BUFFER
	
		return depth * ( far - near ) - far;
	#else
		return depth * ( near - far ) - near;
	#endif
}
float viewZToPerspectiveDepth( const in float viewZ, const in float near, const in float far ) {
	return ( ( near + viewZ ) * far ) / ( ( far - near ) * viewZ );
}
float perspectiveDepthToViewZ( const in float depth, const in float near, const in float far ) {
	
	#ifdef USE_REVERSED_DEPTH_BUFFER
		return ( near * far ) / ( ( near - far ) * depth - near );
	#else
		return ( near * far ) / ( ( far - near ) * depth - far );
	#endif
}`,premultiplied_alpha_fragment:`#ifdef PREMULTIPLIED_ALPHA
	gl_FragColor.rgb *= gl_FragColor.a;
#endif`,project_vertex:`vec4 mvPosition = vec4( transformed, 1.0 );
#ifdef USE_BATCHING
	mvPosition = batchingMatrix * mvPosition;
#endif
#ifdef USE_INSTANCING
	mvPosition = instanceMatrix * mvPosition;
#endif
mvPosition = modelViewMatrix * mvPosition;
gl_Position = projectionMatrix * mvPosition;`,dithering_fragment:`#ifdef DITHERING
	gl_FragColor.rgb = dithering( gl_FragColor.rgb );
#endif`,dithering_pars_fragment:`#ifdef DITHERING
	vec3 dithering( vec3 color ) {
		float grid_position = rand( gl_FragCoord.xy );
		vec3 dither_shift_RGB = vec3( 0.25 / 255.0, -0.25 / 255.0, 0.25 / 255.0 );
		dither_shift_RGB = mix( 2.0 * dither_shift_RGB, -2.0 * dither_shift_RGB, grid_position );
		return color + dither_shift_RGB;
	}
#endif`,roughnessmap_fragment:`float roughnessFactor = roughness;
#ifdef USE_ROUGHNESSMAP
	vec4 texelRoughness = texture2D( roughnessMap, vRoughnessMapUv );
	roughnessFactor *= texelRoughness.g;
#endif`,roughnessmap_pars_fragment:`#ifdef USE_ROUGHNESSMAP
	uniform sampler2D roughnessMap;
#endif`,shadowmap_pars_fragment:`#if NUM_SPOT_LIGHT_COORDS > 0
	varying vec4 vSpotLightCoord[ NUM_SPOT_LIGHT_COORDS ];
#endif
#if NUM_SPOT_LIGHT_MAPS > 0
	uniform sampler2D spotLightMap[ NUM_SPOT_LIGHT_MAPS ];
#endif
#ifdef USE_SHADOWMAP
	#if NUM_SUN_LIGHT_SHADOWS > 0
		#define SUN_LIGHT_CASCADES 2
		#if defined( SHADOWMAP_TYPE_PCF )
			uniform sampler2DShadow sunShadowMap[ NUM_SUN_LIGHT_SHADOWS ];
		#else
			uniform sampler2D sunShadowMap[ NUM_SUN_LIGHT_SHADOWS ];
		#endif
		uniform mat4 sunShadowMatrix[ NUM_SUN_LIGHT_SHADOWS * SUN_LIGHT_CASCADES ];
		uniform vec4 sunShadowCascade[ NUM_SUN_LIGHT_SHADOWS * SUN_LIGHT_CASCADES ];
		varying vec4 vSunShadowWorldPosition;
		varying vec3 vSunShadowWorldNormal;
		struct SunLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform SunLightShadow sunLightShadows[ NUM_SUN_LIGHT_SHADOWS ];
	#endif
	#if NUM_DIR_LIGHT_SHADOWS > 0
		#if defined( SHADOWMAP_TYPE_PCF )
			uniform sampler2DShadow directionalShadowMap[ NUM_DIR_LIGHT_SHADOWS ];
		#else
			uniform sampler2D directionalShadowMap[ NUM_DIR_LIGHT_SHADOWS ];
		#endif
		varying vec4 vDirectionalShadowCoord[ NUM_DIR_LIGHT_SHADOWS ];
		struct DirectionalLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform DirectionalLightShadow directionalLightShadows[ NUM_DIR_LIGHT_SHADOWS ];
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
		#if defined( SHADOWMAP_TYPE_PCF )
			uniform sampler2DShadow spotShadowMap[ NUM_SPOT_LIGHT_SHADOWS ];
		#else
			uniform sampler2D spotShadowMap[ NUM_SPOT_LIGHT_SHADOWS ];
		#endif
		struct SpotLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform SpotLightShadow spotLightShadows[ NUM_SPOT_LIGHT_SHADOWS ];
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		#if defined( SHADOWMAP_TYPE_PCF )
			uniform samplerCubeShadow pointShadowMap[ NUM_POINT_LIGHT_SHADOWS ];
		#elif defined( SHADOWMAP_TYPE_BASIC )
			uniform samplerCube pointShadowMap[ NUM_POINT_LIGHT_SHADOWS ];
		#endif
		varying vec4 vPointShadowCoord[ NUM_POINT_LIGHT_SHADOWS ];
		struct PointLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
			float shadowCameraNear;
			float shadowCameraFar;
		};
		uniform PointLightShadow pointLightShadows[ NUM_POINT_LIGHT_SHADOWS ];
	#endif
	#if defined( SHADOWMAP_TYPE_PCF )
		float interleavedGradientNoise( vec2 position ) {
			return fract( 52.9829189 * fract( dot( position, vec2( 0.06711056, 0.00583715 ) ) ) );
		}
		vec2 vogelDiskSample( int sampleIndex, int samplesCount, float phi ) {
			const float goldenAngle = 2.399963229728653;
			float r = sqrt( ( float( sampleIndex ) + 0.5 ) / float( samplesCount ) );
			float theta = float( sampleIndex ) * goldenAngle + phi;
			return vec2( cos( theta ), sin( theta ) ) * r;
		}
	#endif
	#if defined( SHADOWMAP_TYPE_PCF )
		float getShadow( sampler2DShadow shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord ) {
			float shadow = 1.0;
			shadowCoord.xyz /= shadowCoord.w;
			shadowCoord.z += shadowBias;
			bool inFrustum = shadowCoord.x >= 0.0 && shadowCoord.x <= 1.0 && shadowCoord.y >= 0.0 && shadowCoord.y <= 1.0;
			bool frustumTest = inFrustum && shadowCoord.z <= 1.0;
			if ( frustumTest ) {
				vec2 texelSize = vec2( 1.0 ) / shadowMapSize;
				float radius = shadowRadius * texelSize.x;
				float phi = interleavedGradientNoise( gl_FragCoord.xy ) * PI2;
				shadow = (
					texture( shadowMap, vec3( shadowCoord.xy + vogelDiskSample( 0, 5, phi ) * radius, shadowCoord.z ) ) +
					texture( shadowMap, vec3( shadowCoord.xy + vogelDiskSample( 1, 5, phi ) * radius, shadowCoord.z ) ) +
					texture( shadowMap, vec3( shadowCoord.xy + vogelDiskSample( 2, 5, phi ) * radius, shadowCoord.z ) ) +
					texture( shadowMap, vec3( shadowCoord.xy + vogelDiskSample( 3, 5, phi ) * radius, shadowCoord.z ) ) +
					texture( shadowMap, vec3( shadowCoord.xy + vogelDiskSample( 4, 5, phi ) * radius, shadowCoord.z ) )
				) * 0.2;
			}
			return mix( 1.0, shadow, shadowIntensity );
		}
	#elif defined( SHADOWMAP_TYPE_VSM )
		float getShadow( sampler2D shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord ) {
			float shadow = 1.0;
			shadowCoord.xyz /= shadowCoord.w;
			#ifdef USE_REVERSED_DEPTH_BUFFER
				shadowCoord.z -= shadowBias;
			#else
				shadowCoord.z += shadowBias;
			#endif
			bool inFrustum = shadowCoord.x >= 0.0 && shadowCoord.x <= 1.0 && shadowCoord.y >= 0.0 && shadowCoord.y <= 1.0;
			bool frustumTest = inFrustum && shadowCoord.z <= 1.0;
			if ( frustumTest ) {
				vec2 distribution = texture2D( shadowMap, shadowCoord.xy ).rg;
				float mean = distribution.x;
				float variance = distribution.y * distribution.y;
				#ifdef USE_REVERSED_DEPTH_BUFFER
					float hard_shadow = step( mean, shadowCoord.z );
				#else
					float hard_shadow = step( shadowCoord.z, mean );
				#endif
				
				if ( hard_shadow == 1.0 ) {
					shadow = 1.0;
				} else {
					variance = max( variance, 0.0000001 );
					float d = shadowCoord.z - mean;
					float p_max = variance / ( variance + d * d );
					p_max = clamp( ( p_max - 0.3 ) / 0.65, 0.0, 1.0 );
					shadow = max( hard_shadow, p_max );
				}
			}
			return mix( 1.0, shadow, shadowIntensity );
		}
	#else
		float getShadow( sampler2D shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord ) {
			float shadow = 1.0;
			shadowCoord.xyz /= shadowCoord.w;
			#ifdef USE_REVERSED_DEPTH_BUFFER
				shadowCoord.z -= shadowBias;
			#else
				shadowCoord.z += shadowBias;
			#endif
			bool inFrustum = shadowCoord.x >= 0.0 && shadowCoord.x <= 1.0 && shadowCoord.y >= 0.0 && shadowCoord.y <= 1.0;
			bool frustumTest = inFrustum && shadowCoord.z <= 1.0;
			if ( frustumTest ) {
				float depth = texture2D( shadowMap, shadowCoord.xy ).r;
				#ifdef USE_REVERSED_DEPTH_BUFFER
					shadow = step( depth, shadowCoord.z );
				#else
					shadow = step( shadowCoord.z, depth );
				#endif
			}
			return mix( 1.0, shadow, shadowIntensity );
		}
	#endif
	#if NUM_SUN_LIGHT_SHADOWS > 0
		float getSunShadow(
			#if defined( SHADOWMAP_TYPE_PCF )
				sampler2DShadow shadowMap,
			#else
				sampler2D shadowMap,
			#endif
			SunLightShadow sunLightShadow,
			int shadowIndex
		) {
			vec4 shadowWorldPosition = vec4( vSunShadowWorldPosition.xyz + vSunShadowWorldNormal * sunLightShadow.shadowNormalBias, 1.0 );
			float viewDepth = vSunShadowWorldPosition.w;
			int cascadeOffset = shadowIndex * SUN_LIGHT_CASCADES;
			float shadow = 1.0;
			for ( int i = SUN_LIGHT_CASCADES - 1; i >= 0; i -- ) {
				vec4 cascade = sunShadowCascade[ cascadeOffset + i ];
				if ( viewDepth >= cascade.x && viewDepth < cascade.y ) {
					float cascadeShadow = getShadow(
						shadowMap,
						sunLightShadow.shadowMapSize,
						sunLightShadow.shadowIntensity,
						sunLightShadow.shadowBias,
						sunLightShadow.shadowRadius,
						sunShadowMatrix[ cascadeOffset + i ] * shadowWorldPosition
					);
					shadow = mix( cascadeShadow, shadow, smoothstep( cascade.z, cascade.y, viewDepth ) );
				}
			}
			return shadow;
		}
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
	#if defined( SHADOWMAP_TYPE_PCF )
	float getPointShadow( samplerCubeShadow shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord, float shadowCameraNear, float shadowCameraFar ) {
		float shadow = 1.0;
		vec3 lightToPosition = shadowCoord.xyz;
		vec3 bd3D = normalize( lightToPosition );
		vec3 absVec = abs( lightToPosition );
		float viewSpaceZ = max( max( absVec.x, absVec.y ), absVec.z );
		if ( viewSpaceZ - shadowCameraFar <= 0.0 && viewSpaceZ - shadowCameraNear >= 0.0 ) {
			#ifdef USE_REVERSED_DEPTH_BUFFER
				float dp = ( shadowCameraNear * ( shadowCameraFar - viewSpaceZ ) ) / ( viewSpaceZ * ( shadowCameraFar - shadowCameraNear ) );
				dp -= shadowBias;
			#else
				float dp = ( shadowCameraFar * ( viewSpaceZ - shadowCameraNear ) ) / ( viewSpaceZ * ( shadowCameraFar - shadowCameraNear ) );
				dp += shadowBias;
			#endif
			float texelSize = shadowRadius / shadowMapSize.x;
			vec3 absDir = abs( bd3D );
			vec3 tangent = absDir.x > absDir.z ? vec3( 0.0, 1.0, 0.0 ) : vec3( 1.0, 0.0, 0.0 );
			tangent = normalize( cross( bd3D, tangent ) );
			vec3 bitangent = cross( bd3D, tangent );
			float phi = interleavedGradientNoise( gl_FragCoord.xy ) * PI2;
			vec2 sample0 = vogelDiskSample( 0, 5, phi );
			vec2 sample1 = vogelDiskSample( 1, 5, phi );
			vec2 sample2 = vogelDiskSample( 2, 5, phi );
			vec2 sample3 = vogelDiskSample( 3, 5, phi );
			vec2 sample4 = vogelDiskSample( 4, 5, phi );
			shadow = (
				texture( shadowMap, vec4( bd3D + ( tangent * sample0.x + bitangent * sample0.y ) * texelSize, dp ) ) +
				texture( shadowMap, vec4( bd3D + ( tangent * sample1.x + bitangent * sample1.y ) * texelSize, dp ) ) +
				texture( shadowMap, vec4( bd3D + ( tangent * sample2.x + bitangent * sample2.y ) * texelSize, dp ) ) +
				texture( shadowMap, vec4( bd3D + ( tangent * sample3.x + bitangent * sample3.y ) * texelSize, dp ) ) +
				texture( shadowMap, vec4( bd3D + ( tangent * sample4.x + bitangent * sample4.y ) * texelSize, dp ) )
			) * 0.2;
		}
		return mix( 1.0, shadow, shadowIntensity );
	}
	#elif defined( SHADOWMAP_TYPE_BASIC )
	float getPointShadow( samplerCube shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord, float shadowCameraNear, float shadowCameraFar ) {
		float shadow = 1.0;
		vec3 lightToPosition = shadowCoord.xyz;
		vec3 absVec = abs( lightToPosition );
		float viewSpaceZ = max( max( absVec.x, absVec.y ), absVec.z );
		if ( viewSpaceZ - shadowCameraFar <= 0.0 && viewSpaceZ - shadowCameraNear >= 0.0 ) {
			float dp = ( shadowCameraFar * ( viewSpaceZ - shadowCameraNear ) ) / ( viewSpaceZ * ( shadowCameraFar - shadowCameraNear ) );
			dp += shadowBias;
			vec3 bd3D = normalize( lightToPosition );
			float depth = textureCube( shadowMap, bd3D ).r;
			#ifdef USE_REVERSED_DEPTH_BUFFER
				depth = 1.0 - depth;
			#endif
			shadow = step( dp, depth );
		}
		return mix( 1.0, shadow, shadowIntensity );
	}
	#endif
	#endif
#endif`,shadowmap_pars_vertex:`#if NUM_SPOT_LIGHT_COORDS > 0
	uniform mat4 spotLightMatrix[ NUM_SPOT_LIGHT_COORDS ];
	varying vec4 vSpotLightCoord[ NUM_SPOT_LIGHT_COORDS ];
#endif
#ifdef USE_SHADOWMAP
	#if NUM_SUN_LIGHT_SHADOWS > 0
		varying vec4 vSunShadowWorldPosition;
		varying vec3 vSunShadowWorldNormal;
	#endif
	#if NUM_DIR_LIGHT_SHADOWS > 0
		uniform mat4 directionalShadowMatrix[ NUM_DIR_LIGHT_SHADOWS ];
		varying vec4 vDirectionalShadowCoord[ NUM_DIR_LIGHT_SHADOWS ];
		struct DirectionalLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform DirectionalLightShadow directionalLightShadows[ NUM_DIR_LIGHT_SHADOWS ];
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
		struct SpotLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform SpotLightShadow spotLightShadows[ NUM_SPOT_LIGHT_SHADOWS ];
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		uniform mat4 pointShadowMatrix[ NUM_POINT_LIGHT_SHADOWS ];
		varying vec4 vPointShadowCoord[ NUM_POINT_LIGHT_SHADOWS ];
		struct PointLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
			float shadowCameraNear;
			float shadowCameraFar;
		};
		uniform PointLightShadow pointLightShadows[ NUM_POINT_LIGHT_SHADOWS ];
	#endif
#endif`,shadowmap_vertex:`#if ( defined( USE_SHADOWMAP ) && ( NUM_DIR_LIGHT_SHADOWS > 0 || NUM_SUN_LIGHT_SHADOWS > 0 || NUM_POINT_LIGHT_SHADOWS > 0 ) ) || ( NUM_SPOT_LIGHT_COORDS > 0 )
	#ifdef HAS_NORMAL
		vec3 shadowWorldNormal = transformNormalByInverseViewMatrix( transformedNormal, viewMatrix );
	#else
		vec3 shadowWorldNormal = vec3( 0.0 );
	#endif
	vec4 shadowWorldPosition;
#endif
#if defined( USE_SHADOWMAP )
	#if NUM_SUN_LIGHT_SHADOWS > 0
		vSunShadowWorldPosition = vec4( worldPosition.xyz, - mvPosition.z );
		vSunShadowWorldNormal = shadowWorldNormal;
	#endif
	#if NUM_DIR_LIGHT_SHADOWS > 0
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_DIR_LIGHT_SHADOWS; i ++ ) {
			shadowWorldPosition = worldPosition + vec4( shadowWorldNormal * directionalLightShadows[ i ].shadowNormalBias, 0 );
			vDirectionalShadowCoord[ i ] = directionalShadowMatrix[ i ] * shadowWorldPosition;
		}
		#pragma unroll_loop_end
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_POINT_LIGHT_SHADOWS; i ++ ) {
			shadowWorldPosition = worldPosition + vec4( shadowWorldNormal * pointLightShadows[ i ].shadowNormalBias, 0 );
			vPointShadowCoord[ i ] = pointShadowMatrix[ i ] * shadowWorldPosition;
		}
		#pragma unroll_loop_end
	#endif
#endif
#if NUM_SPOT_LIGHT_COORDS > 0
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHT_COORDS; i ++ ) {
		shadowWorldPosition = worldPosition;
		#if ( defined( USE_SHADOWMAP ) && UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
			shadowWorldPosition.xyz += shadowWorldNormal * spotLightShadows[ i ].shadowNormalBias;
		#endif
		vSpotLightCoord[ i ] = spotLightMatrix[ i ] * shadowWorldPosition;
	}
	#pragma unroll_loop_end
#endif`,shadowmask_pars_fragment:`float getShadowMask() {
	float shadow = 1.0;
	#ifdef USE_SHADOWMAP
	#if NUM_SUN_LIGHT_SHADOWS > 0
	SunLightShadow sunLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SUN_LIGHT_SHADOWS; i ++ ) {
		sunLight = sunLightShadows[ i ];
		shadow *= receiveShadow ? getSunShadow( sunShadowMap[ i ], sunLight, UNROLLED_LOOP_INDEX ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#if NUM_DIR_LIGHT_SHADOWS > 0
	DirectionalLightShadow directionalLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_DIR_LIGHT_SHADOWS; i ++ ) {
		directionalLight = directionalLightShadows[ i ];
		shadow *= receiveShadow ? getShadow( directionalShadowMap[ i ], directionalLight.shadowMapSize, directionalLight.shadowIntensity, directionalLight.shadowBias, directionalLight.shadowRadius, vDirectionalShadowCoord[ i ] ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
	SpotLightShadow spotLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHT_SHADOWS; i ++ ) {
		spotLight = spotLightShadows[ i ];
		shadow *= receiveShadow ? getShadow( spotShadowMap[ i ], spotLight.shadowMapSize, spotLight.shadowIntensity, spotLight.shadowBias, spotLight.shadowRadius, vSpotLightCoord[ i ] ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0 && ( defined( SHADOWMAP_TYPE_PCF ) || defined( SHADOWMAP_TYPE_BASIC ) )
	PointLightShadow pointLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_POINT_LIGHT_SHADOWS; i ++ ) {
		pointLight = pointLightShadows[ i ];
		shadow *= receiveShadow ? getPointShadow( pointShadowMap[ i ], pointLight.shadowMapSize, pointLight.shadowIntensity, pointLight.shadowBias, pointLight.shadowRadius, vPointShadowCoord[ i ], pointLight.shadowCameraNear, pointLight.shadowCameraFar ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#endif
	return shadow;
}`,skinbase_vertex:`#ifdef USE_SKINNING
	mat4 boneMatX = getBoneMatrix( skinIndex.x );
	mat4 boneMatY = getBoneMatrix( skinIndex.y );
	mat4 boneMatZ = getBoneMatrix( skinIndex.z );
	mat4 boneMatW = getBoneMatrix( skinIndex.w );
#endif`,skinning_pars_vertex:`#ifdef USE_SKINNING
	uniform mat4 bindMatrix;
	uniform mat4 bindMatrixInverse;
	uniform highp sampler2D boneTexture;
	mat4 getBoneMatrix( const in float i ) {
		int size = textureSize( boneTexture, 0 ).x;
		int j = int( i ) * 4;
		int x = j % size;
		int y = j / size;
		vec4 v1 = texelFetch( boneTexture, ivec2( x, y ), 0 );
		vec4 v2 = texelFetch( boneTexture, ivec2( x + 1, y ), 0 );
		vec4 v3 = texelFetch( boneTexture, ivec2( x + 2, y ), 0 );
		vec4 v4 = texelFetch( boneTexture, ivec2( x + 3, y ), 0 );
		return mat4( v1, v2, v3, v4 );
	}
#endif`,skinning_vertex:`#ifdef USE_SKINNING
	vec4 skinVertex = bindMatrix * vec4( transformed, 1.0 );
	vec4 skinned = vec4( 0.0 );
	skinned += boneMatX * skinVertex * skinWeight.x;
	skinned += boneMatY * skinVertex * skinWeight.y;
	skinned += boneMatZ * skinVertex * skinWeight.z;
	skinned += boneMatW * skinVertex * skinWeight.w;
	transformed = ( bindMatrixInverse * skinned ).xyz;
#endif`,skinnormal_vertex:`#ifdef USE_SKINNING
	mat4 skinMatrix = mat4( 0.0 );
	skinMatrix += skinWeight.x * boneMatX;
	skinMatrix += skinWeight.y * boneMatY;
	skinMatrix += skinWeight.z * boneMatZ;
	skinMatrix += skinWeight.w * boneMatW;
	skinMatrix = bindMatrixInverse * skinMatrix * bindMatrix;
	objectNormal = vec4( skinMatrix * vec4( objectNormal, 0.0 ) ).xyz;
	#ifdef USE_TANGENT
		objectTangent = vec4( skinMatrix * vec4( objectTangent, 0.0 ) ).xyz;
	#endif
#endif`,specularmap_fragment:`float specularStrength;
#ifdef USE_SPECULARMAP
	vec4 texelSpecular = texture2D( specularMap, vSpecularMapUv );
	specularStrength = texelSpecular.r;
#else
	specularStrength = 1.0;
#endif`,specularmap_pars_fragment:`#ifdef USE_SPECULARMAP
	uniform sampler2D specularMap;
#endif`,tonemapping_fragment:`#if defined( TONE_MAPPING )
	gl_FragColor.rgb = toneMapping( gl_FragColor.rgb );
#endif`,tonemapping_pars_fragment:`#ifndef saturate
#define saturate( a ) clamp( a, 0.0, 1.0 )
#endif
uniform float toneMappingExposure;
vec3 LinearToneMapping( vec3 color ) {
	return saturate( toneMappingExposure * color );
}
vec3 ReinhardToneMapping( vec3 color ) {
	color *= toneMappingExposure;
	return saturate( color / ( vec3( 1.0 ) + color ) );
}
vec3 CineonToneMapping( vec3 color ) {
	color *= toneMappingExposure;
	color = max( vec3( 0.0 ), color - 0.004 );
	return pow( ( color * ( 6.2 * color + 0.5 ) ) / ( color * ( 6.2 * color + 1.7 ) + 0.06 ), vec3( 2.2 ) );
}
vec3 RRTAndODTFit( vec3 v ) {
	vec3 a = v * ( v + 0.0245786 ) - 0.000090537;
	vec3 b = v * ( 0.983729 * v + 0.4329510 ) + 0.238081;
	return a / b;
}
vec3 ACESFilmicToneMapping( vec3 color ) {
	const mat3 ACESInputMat = mat3(
		vec3( 0.59719, 0.07600, 0.02840 ),		vec3( 0.35458, 0.90834, 0.13383 ),
		vec3( 0.04823, 0.01566, 0.83777 )
	);
	const mat3 ACESOutputMat = mat3(
		vec3(  1.60475, -0.10208, -0.00327 ),		vec3( -0.53108,  1.10813, -0.07276 ),
		vec3( -0.07367, -0.00605,  1.07602 )
	);
	color *= toneMappingExposure / 0.6;
	color = ACESInputMat * color;
	color = RRTAndODTFit( color );
	color = ACESOutputMat * color;
	return saturate( color );
}
const mat3 LINEAR_REC2020_TO_LINEAR_SRGB = mat3(
	vec3( 1.6605, - 0.1246, - 0.0182 ),
	vec3( - 0.5876, 1.1329, - 0.1006 ),
	vec3( - 0.0728, - 0.0083, 1.1187 )
);
const mat3 LINEAR_SRGB_TO_LINEAR_REC2020 = mat3(
	vec3( 0.6274, 0.0691, 0.0164 ),
	vec3( 0.3293, 0.9195, 0.0880 ),
	vec3( 0.0433, 0.0113, 0.8956 )
);
vec3 agxDefaultContrastApprox( vec3 x ) {
	vec3 x2 = x * x;
	vec3 x4 = x2 * x2;
	return + 15.5 * x4 * x2
		- 40.14 * x4 * x
		+ 31.96 * x4
		- 6.868 * x2 * x
		+ 0.4298 * x2
		+ 0.1191 * x
		- 0.00232;
}
vec3 AgXToneMapping( vec3 color ) {
	const mat3 AgXInsetMatrix = mat3(
		vec3( 0.856627153315983, 0.137318972929847, 0.11189821299995 ),
		vec3( 0.0951212405381588, 0.761241990602591, 0.0767994186031903 ),
		vec3( 0.0482516061458583, 0.101439036467562, 0.811302368396859 )
	);
	const mat3 AgXOutsetMatrix = mat3(
		vec3( 1.1271005818144368, - 0.1413297634984383, - 0.14132976349843826 ),
		vec3( - 0.11060664309660323, 1.157823702216272, - 0.11060664309660294 ),
		vec3( - 0.016493938717834573, - 0.016493938717834257, 1.2519364065950405 )
	);
	const float AgxMinEv = - 12.47393;	const float AgxMaxEv = 4.026069;
	color *= toneMappingExposure;
	color = LINEAR_SRGB_TO_LINEAR_REC2020 * color;
	color = AgXInsetMatrix * color;
	color = max( color, 1e-10 );	color = log2( color );
	color = ( color - AgxMinEv ) / ( AgxMaxEv - AgxMinEv );
	color = clamp( color, 0.0, 1.0 );
	color = agxDefaultContrastApprox( color );
	color = AgXOutsetMatrix * color;
	color = pow( max( vec3( 0.0 ), color ), vec3( 2.2 ) );
	color = LINEAR_REC2020_TO_LINEAR_SRGB * color;
	color = clamp( color, 0.0, 1.0 );
	return color;
}
vec3 NeutralToneMapping( vec3 color ) {
	const float StartCompression = 0.8 - 0.04;
	const float Desaturation = 0.15;
	color *= toneMappingExposure;
	float x = min( color.r, min( color.g, color.b ) );
	float offset = x < 0.08 ? x - 6.25 * x * x : 0.04;
	color -= offset;
	float peak = max( color.r, max( color.g, color.b ) );
	if ( peak < StartCompression ) return color;
	float d = 1. - StartCompression;
	float newPeak = 1. - d * d / ( peak + d - StartCompression );
	color *= newPeak / peak;
	float g = 1. - 1. / ( Desaturation * ( peak - newPeak ) + 1. );
	return mix( color, vec3( newPeak ), g );
}
vec3 CustomToneMapping( vec3 color ) { return color; }`,transmission_fragment:`#ifdef USE_TRANSMISSION
	material.transmission = transmission;
	material.transmissionAlpha = 1.0;
	material.thickness = thickness;
	material.attenuationDistance = attenuationDistance;
	material.attenuationColor = attenuationColor;
	#ifdef USE_TRANSMISSIONMAP
		material.transmission *= texture2D( transmissionMap, vTransmissionMapUv ).r;
	#endif
	#ifdef USE_THICKNESSMAP
		material.thickness *= texture2D( thicknessMap, vThicknessMapUv ).g;
	#endif
	vec3 pos = vWorldPosition;
	vec3 v = normalize( cameraPosition - pos );
	vec3 n = transformNormalByInverseViewMatrix( normal, viewMatrix );
	vec4 transmitted = getIBLVolumeRefraction(
		n, v, material.roughness, material.diffuseContribution, material.specularColorBlended, material.specularF90,
		pos, modelMatrix, viewMatrix, projectionMatrix, material.dispersion, material.ior, material.thickness,
		material.attenuationColor, material.attenuationDistance );
	material.transmissionAlpha = mix( material.transmissionAlpha, transmitted.a, material.transmission );
	totalDiffuse = mix( totalDiffuse, transmitted.rgb, material.transmission );
#endif`,transmission_pars_fragment:`#ifdef USE_TRANSMISSION
	uniform float transmission;
	uniform float thickness;
	uniform float attenuationDistance;
	uniform vec3 attenuationColor;
	#ifdef USE_TRANSMISSIONMAP
		uniform sampler2D transmissionMap;
	#endif
	#ifdef USE_THICKNESSMAP
		uniform sampler2D thicknessMap;
	#endif
	uniform vec2 transmissionSamplerSize;
	uniform sampler2D transmissionSamplerMap;
	uniform mat4 modelMatrix;
	uniform mat4 projectionMatrix;
	varying vec3 vWorldPosition;
	float w0( float a ) {
		return ( 1.0 / 6.0 ) * ( a * ( a * ( - a + 3.0 ) - 3.0 ) + 1.0 );
	}
	float w1( float a ) {
		return ( 1.0 / 6.0 ) * ( a *  a * ( 3.0 * a - 6.0 ) + 4.0 );
	}
	float w2( float a ){
		return ( 1.0 / 6.0 ) * ( a * ( a * ( - 3.0 * a + 3.0 ) + 3.0 ) + 1.0 );
	}
	float w3( float a ) {
		return ( 1.0 / 6.0 ) * ( a * a * a );
	}
	float g0( float a ) {
		return w0( a ) + w1( a );
	}
	float g1( float a ) {
		return w2( a ) + w3( a );
	}
	float h0( float a ) {
		return - 1.0 + w1( a ) / ( w0( a ) + w1( a ) );
	}
	float h1( float a ) {
		return 1.0 + w3( a ) / ( w2( a ) + w3( a ) );
	}
	vec4 bicubic( sampler2D tex, vec2 uv, vec4 texelSize, float lod ) {
		uv = uv * texelSize.zw + 0.5;
		vec2 iuv = floor( uv );
		vec2 fuv = fract( uv );
		float g0x = g0( fuv.x );
		float g1x = g1( fuv.x );
		float h0x = h0( fuv.x );
		float h1x = h1( fuv.x );
		float h0y = h0( fuv.y );
		float h1y = h1( fuv.y );
		vec2 p0 = ( vec2( iuv.x + h0x, iuv.y + h0y ) - 0.5 ) * texelSize.xy;
		vec2 p1 = ( vec2( iuv.x + h1x, iuv.y + h0y ) - 0.5 ) * texelSize.xy;
		vec2 p2 = ( vec2( iuv.x + h0x, iuv.y + h1y ) - 0.5 ) * texelSize.xy;
		vec2 p3 = ( vec2( iuv.x + h1x, iuv.y + h1y ) - 0.5 ) * texelSize.xy;
		return g0( fuv.y ) * ( g0x * textureLod( tex, p0, lod ) + g1x * textureLod( tex, p1, lod ) ) +
			g1( fuv.y ) * ( g0x * textureLod( tex, p2, lod ) + g1x * textureLod( tex, p3, lod ) );
	}
	vec4 textureBicubic( sampler2D sampler, vec2 uv, float lod ) {
		vec2 fLodSize = vec2( textureSize( sampler, int( lod ) ) );
		vec2 cLodSize = vec2( textureSize( sampler, int( lod + 1.0 ) ) );
		vec2 fLodSizeInv = 1.0 / fLodSize;
		vec2 cLodSizeInv = 1.0 / cLodSize;
		vec4 fSample = bicubic( sampler, uv, vec4( fLodSizeInv, fLodSize ), floor( lod ) );
		vec4 cSample = bicubic( sampler, uv, vec4( cLodSizeInv, cLodSize ), ceil( lod ) );
		return mix( fSample, cSample, fract( lod ) );
	}
	vec3 getVolumeTransmissionRay( const in vec3 n, const in vec3 v, const in float thickness, const in float ior, const in mat4 modelMatrix ) {
		vec3 refractionVector = refract( - v, normalize( n ), 1.0 / ior );
		vec3 modelScale;
		modelScale.x = length( vec3( modelMatrix[ 0 ].xyz ) );
		modelScale.y = length( vec3( modelMatrix[ 1 ].xyz ) );
		modelScale.z = length( vec3( modelMatrix[ 2 ].xyz ) );
		return normalize( refractionVector ) * thickness * modelScale;
	}
	float applyIorToRoughness( const in float roughness, const in float ior ) {
		return roughness * clamp( ior * 2.0 - 2.0, 0.0, 1.0 );
	}
	vec4 getTransmissionSample( const in vec2 fragCoord, const in float roughness, const in float ior ) {
		float lod = log2( transmissionSamplerSize.x ) * applyIorToRoughness( roughness, ior );
		return textureBicubic( transmissionSamplerMap, fragCoord.xy, lod );
	}
	vec3 volumeAttenuation( const in float transmissionDistance, const in vec3 attenuationColor, const in float attenuationDistance ) {
		if ( isinf( attenuationDistance ) ) {
			return vec3( 1.0 );
		} else {
			vec3 attenuationCoefficient = -log( attenuationColor ) / attenuationDistance;
			vec3 transmittance = exp( - attenuationCoefficient * transmissionDistance );			return transmittance;
		}
	}
	vec4 getIBLVolumeRefraction( const in vec3 n, const in vec3 v, const in float roughness, const in vec3 diffuseColor,
		const in vec3 specularColor, const in float specularF90, const in vec3 position, const in mat4 modelMatrix,
		const in mat4 viewMatrix, const in mat4 projMatrix, const in float dispersion, const in float ior, const in float thickness,
		const in vec3 attenuationColor, const in float attenuationDistance ) {
		vec4 transmittedLight;
		vec3 transmittance;
		#ifdef USE_DISPERSION
			float halfSpread = ( ior - 1.0 ) * 0.025 * dispersion;
			vec3 iors = vec3( ior - halfSpread, ior, ior + halfSpread );
			for ( int i = 0; i < 3; i ++ ) {
				vec3 transmissionRay = getVolumeTransmissionRay( n, v, thickness, iors[ i ], modelMatrix );
				vec3 refractedRayExit = position + transmissionRay;
				vec4 ndcPos = projMatrix * viewMatrix * vec4( refractedRayExit, 1.0 );
				vec2 refractionCoords = ndcPos.xy / ndcPos.w;
				refractionCoords += 1.0;
				refractionCoords /= 2.0;
				vec4 transmissionSample = getTransmissionSample( refractionCoords, roughness, iors[ i ] );
				transmittedLight[ i ] = transmissionSample[ i ];
				transmittedLight.a += transmissionSample.a;
				transmittance[ i ] = diffuseColor[ i ] * volumeAttenuation( length( transmissionRay ), attenuationColor, attenuationDistance )[ i ];
			}
			transmittedLight.a /= 3.0;
		#else
			vec3 transmissionRay = getVolumeTransmissionRay( n, v, thickness, ior, modelMatrix );
			vec3 refractedRayExit = position + transmissionRay;
			vec4 ndcPos = projMatrix * viewMatrix * vec4( refractedRayExit, 1.0 );
			vec2 refractionCoords = ndcPos.xy / ndcPos.w;
			refractionCoords += 1.0;
			refractionCoords /= 2.0;
			transmittedLight = getTransmissionSample( refractionCoords, roughness, ior );
			transmittance = diffuseColor * volumeAttenuation( length( transmissionRay ), attenuationColor, attenuationDistance );
		#endif
		vec3 attenuatedColor = transmittance * transmittedLight.rgb;
		vec3 F = EnvironmentBRDF( n, v, specularColor, specularF90, roughness );
		float transmittanceFactor = ( transmittance.r + transmittance.g + transmittance.b ) / 3.0;
		return vec4( ( 1.0 - F ) * attenuatedColor, 1.0 - ( 1.0 - transmittedLight.a ) * transmittanceFactor );
	}
#endif`,uv_pars_fragment:`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	varying vec2 vUv;
#endif
#ifdef USE_MAP
	varying vec2 vMapUv;
#endif
#ifdef USE_ALPHAMAP
	varying vec2 vAlphaMapUv;
#endif
#ifdef USE_LIGHTMAP
	varying vec2 vLightMapUv;
#endif
#ifdef USE_AOMAP
	varying vec2 vAoMapUv;
#endif
#ifdef USE_BUMPMAP
	varying vec2 vBumpMapUv;
#endif
#ifdef USE_NORMALMAP
	varying vec2 vNormalMapUv;
#endif
#ifdef USE_EMISSIVEMAP
	varying vec2 vEmissiveMapUv;
#endif
#ifdef USE_METALNESSMAP
	varying vec2 vMetalnessMapUv;
#endif
#ifdef USE_ROUGHNESSMAP
	varying vec2 vRoughnessMapUv;
#endif
#ifdef USE_ANISOTROPYMAP
	varying vec2 vAnisotropyMapUv;
#endif
#ifdef USE_CLEARCOATMAP
	varying vec2 vClearcoatMapUv;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	varying vec2 vClearcoatNormalMapUv;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	varying vec2 vClearcoatRoughnessMapUv;
#endif
#ifdef USE_IRIDESCENCEMAP
	varying vec2 vIridescenceMapUv;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	varying vec2 vIridescenceThicknessMapUv;
#endif
#ifdef USE_SHEEN_COLORMAP
	varying vec2 vSheenColorMapUv;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	varying vec2 vSheenRoughnessMapUv;
#endif
#ifdef USE_SPECULARMAP
	varying vec2 vSpecularMapUv;
#endif
#ifdef USE_SPECULAR_COLORMAP
	varying vec2 vSpecularColorMapUv;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	varying vec2 vSpecularIntensityMapUv;
#endif
#ifdef USE_TRANSMISSIONMAP
	uniform mat3 transmissionMapTransform;
	varying vec2 vTransmissionMapUv;
#endif
#ifdef USE_THICKNESSMAP
	uniform mat3 thicknessMapTransform;
	varying vec2 vThicknessMapUv;
#endif`,uv_pars_vertex:`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	varying vec2 vUv;
#endif
#ifdef USE_MAP
	uniform mat3 mapTransform;
	varying vec2 vMapUv;
#endif
#ifdef USE_ALPHAMAP
	uniform mat3 alphaMapTransform;
	varying vec2 vAlphaMapUv;
#endif
#ifdef USE_LIGHTMAP
	uniform mat3 lightMapTransform;
	varying vec2 vLightMapUv;
#endif
#ifdef USE_AOMAP
	uniform mat3 aoMapTransform;
	varying vec2 vAoMapUv;
#endif
#ifdef USE_BUMPMAP
	uniform mat3 bumpMapTransform;
	varying vec2 vBumpMapUv;
#endif
#ifdef USE_NORMALMAP
	uniform mat3 normalMapTransform;
	varying vec2 vNormalMapUv;
#endif
#ifdef USE_DISPLACEMENTMAP
	uniform mat3 displacementMapTransform;
	varying vec2 vDisplacementMapUv;
#endif
#ifdef USE_EMISSIVEMAP
	uniform mat3 emissiveMapTransform;
	varying vec2 vEmissiveMapUv;
#endif
#ifdef USE_METALNESSMAP
	uniform mat3 metalnessMapTransform;
	varying vec2 vMetalnessMapUv;
#endif
#ifdef USE_ROUGHNESSMAP
	uniform mat3 roughnessMapTransform;
	varying vec2 vRoughnessMapUv;
#endif
#ifdef USE_ANISOTROPYMAP
	uniform mat3 anisotropyMapTransform;
	varying vec2 vAnisotropyMapUv;
#endif
#ifdef USE_CLEARCOATMAP
	uniform mat3 clearcoatMapTransform;
	varying vec2 vClearcoatMapUv;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	uniform mat3 clearcoatNormalMapTransform;
	varying vec2 vClearcoatNormalMapUv;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	uniform mat3 clearcoatRoughnessMapTransform;
	varying vec2 vClearcoatRoughnessMapUv;
#endif
#ifdef USE_SHEEN_COLORMAP
	uniform mat3 sheenColorMapTransform;
	varying vec2 vSheenColorMapUv;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	uniform mat3 sheenRoughnessMapTransform;
	varying vec2 vSheenRoughnessMapUv;
#endif
#ifdef USE_IRIDESCENCEMAP
	uniform mat3 iridescenceMapTransform;
	varying vec2 vIridescenceMapUv;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	uniform mat3 iridescenceThicknessMapTransform;
	varying vec2 vIridescenceThicknessMapUv;
#endif
#ifdef USE_SPECULARMAP
	uniform mat3 specularMapTransform;
	varying vec2 vSpecularMapUv;
#endif
#ifdef USE_SPECULAR_COLORMAP
	uniform mat3 specularColorMapTransform;
	varying vec2 vSpecularColorMapUv;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	uniform mat3 specularIntensityMapTransform;
	varying vec2 vSpecularIntensityMapUv;
#endif
#ifdef USE_TRANSMISSIONMAP
	uniform mat3 transmissionMapTransform;
	varying vec2 vTransmissionMapUv;
#endif
#ifdef USE_THICKNESSMAP
	uniform mat3 thicknessMapTransform;
	varying vec2 vThicknessMapUv;
#endif`,uv_vertex:`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	vUv = vec3( uv, 1 ).xy;
#endif
#ifdef USE_MAP
	vMapUv = ( mapTransform * vec3( MAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ALPHAMAP
	vAlphaMapUv = ( alphaMapTransform * vec3( ALPHAMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_LIGHTMAP
	vLightMapUv = ( lightMapTransform * vec3( LIGHTMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_AOMAP
	vAoMapUv = ( aoMapTransform * vec3( AOMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_BUMPMAP
	vBumpMapUv = ( bumpMapTransform * vec3( BUMPMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_NORMALMAP
	vNormalMapUv = ( normalMapTransform * vec3( NORMALMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_DISPLACEMENTMAP
	vDisplacementMapUv = ( displacementMapTransform * vec3( DISPLACEMENTMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_EMISSIVEMAP
	vEmissiveMapUv = ( emissiveMapTransform * vec3( EMISSIVEMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_METALNESSMAP
	vMetalnessMapUv = ( metalnessMapTransform * vec3( METALNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ROUGHNESSMAP
	vRoughnessMapUv = ( roughnessMapTransform * vec3( ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ANISOTROPYMAP
	vAnisotropyMapUv = ( anisotropyMapTransform * vec3( ANISOTROPYMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOATMAP
	vClearcoatMapUv = ( clearcoatMapTransform * vec3( CLEARCOATMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	vClearcoatNormalMapUv = ( clearcoatNormalMapTransform * vec3( CLEARCOAT_NORMALMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	vClearcoatRoughnessMapUv = ( clearcoatRoughnessMapTransform * vec3( CLEARCOAT_ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_IRIDESCENCEMAP
	vIridescenceMapUv = ( iridescenceMapTransform * vec3( IRIDESCENCEMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	vIridescenceThicknessMapUv = ( iridescenceThicknessMapTransform * vec3( IRIDESCENCE_THICKNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SHEEN_COLORMAP
	vSheenColorMapUv = ( sheenColorMapTransform * vec3( SHEEN_COLORMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	vSheenRoughnessMapUv = ( sheenRoughnessMapTransform * vec3( SHEEN_ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULARMAP
	vSpecularMapUv = ( specularMapTransform * vec3( SPECULARMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULAR_COLORMAP
	vSpecularColorMapUv = ( specularColorMapTransform * vec3( SPECULAR_COLORMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	vSpecularIntensityMapUv = ( specularIntensityMapTransform * vec3( SPECULAR_INTENSITYMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_TRANSMISSIONMAP
	vTransmissionMapUv = ( transmissionMapTransform * vec3( TRANSMISSIONMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_THICKNESSMAP
	vThicknessMapUv = ( thicknessMapTransform * vec3( THICKNESSMAP_UV, 1 ) ).xy;
#endif`,worldpos_vertex:`#if defined( USE_ENVMAP ) || defined( DISTANCE ) || defined ( USE_SHADOWMAP ) || defined ( USE_TRANSMISSION ) || NUM_SPOT_LIGHT_COORDS > 0
	vec4 worldPosition = vec4( transformed, 1.0 );
	#ifdef USE_BATCHING
		worldPosition = batchingMatrix * worldPosition;
	#endif
	#ifdef USE_INSTANCING
		worldPosition = instanceMatrix * worldPosition;
	#endif
	worldPosition = modelMatrix * worldPosition;
#endif`,background_vert:`varying vec2 vUv;
uniform mat3 uvTransform;
void main() {
	vUv = ( uvTransform * vec3( uv, 1 ) ).xy;
	gl_Position = vec4( position.xy, 1.0, 1.0 );
}`,background_frag:`uniform sampler2D t2D;
uniform float backgroundIntensity;
varying vec2 vUv;
void main() {
	vec4 texColor = texture2D( t2D, vUv );
	#ifdef DECODE_VIDEO_TEXTURE
		texColor = vec4( mix( pow( texColor.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), texColor.rgb * 0.0773993808, vec3( lessThanEqual( texColor.rgb, vec3( 0.04045 ) ) ) ), texColor.w );
	#endif
	texColor.rgb *= backgroundIntensity;
	gl_FragColor = texColor;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,backgroundCube_vert:`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,backgroundCube_frag:`#ifdef ENVMAP_TYPE_CUBE
	uniform samplerCube envMap;
#elif defined( ENVMAP_TYPE_CUBE_UV )
	uniform sampler2D envMap;
#endif
uniform float backgroundBlurriness;
uniform float backgroundIntensity;
uniform mat3 backgroundRotation;
varying vec3 vWorldDirection;
#include <cube_uv_reflection_fragment>
void main() {
	#ifdef ENVMAP_TYPE_CUBE
		vec4 texColor = textureCube( envMap, backgroundRotation * vWorldDirection );
	#elif defined( ENVMAP_TYPE_CUBE_UV )
		vec4 texColor = textureCubeUV( envMap, backgroundRotation * vWorldDirection, backgroundBlurriness );
	#else
		vec4 texColor = vec4( 0.0, 0.0, 0.0, 1.0 );
	#endif
	texColor.rgb *= backgroundIntensity;
	gl_FragColor = texColor;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,cube_vert:`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,cube_frag:`uniform samplerCube tCube;
uniform float tFlip;
uniform float opacity;
varying vec3 vWorldDirection;
void main() {
	vec4 texColor = textureCube( tCube, vec3( tFlip * vWorldDirection.x, vWorldDirection.yz ) );
	gl_FragColor = texColor;
	gl_FragColor.a *= opacity;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,depth_vert:`#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
varying vec2 vHighPrecisionZW;
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <skinbase_vertex>
	#include <morphinstance_vertex>
	#ifdef USE_DISPLACEMENTMAP
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vHighPrecisionZW = gl_Position.zw;
}`,depth_frag:`#if DEPTH_PACKING == 3200
	uniform float opacity;
#endif
#include <common>
#include <packing>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
varying vec2 vHighPrecisionZW;
void main() {
	vec4 diffuseColor = vec4( 1.0 );
	#include <clipping_planes_fragment>
	#if DEPTH_PACKING == 3200
		diffuseColor.a = opacity;
	#endif
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <logdepthbuf_fragment>
	#ifdef USE_REVERSED_DEPTH_BUFFER
		float fragCoordZ = vHighPrecisionZW[ 0 ] / vHighPrecisionZW[ 1 ];
	#else
		float fragCoordZ = 0.5 * vHighPrecisionZW[ 0 ] / vHighPrecisionZW[ 1 ] + 0.5;
	#endif
	#if DEPTH_PACKING == 3200
		gl_FragColor = vec4( vec3( 1.0 - fragCoordZ ), opacity );
	#elif DEPTH_PACKING == 3201
		gl_FragColor = packDepthToRGBA( fragCoordZ );
	#elif DEPTH_PACKING == 3202
		gl_FragColor = vec4( packDepthToRGB( fragCoordZ ), 1.0 );
	#elif DEPTH_PACKING == 3203
		gl_FragColor = vec4( packDepthToRG( fragCoordZ ), 0.0, 1.0 );
	#endif
}`,distance_vert:`#define DISTANCE
varying vec3 vWorldPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <skinbase_vertex>
	#include <morphinstance_vertex>
	#ifdef USE_DISPLACEMENTMAP
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <worldpos_vertex>
	#include <clipping_planes_vertex>
	vWorldPosition = worldPosition.xyz;
}`,distance_frag:`#define DISTANCE
uniform vec3 referencePosition;
uniform float nearDistance;
uniform float farDistance;
varying vec3 vWorldPosition;
#include <common>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( 1.0 );
	#include <clipping_planes_fragment>
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	float dist = length( vWorldPosition - referencePosition );
	dist = ( dist - nearDistance ) / ( farDistance - nearDistance );
	dist = saturate( dist );
	gl_FragColor = vec4( dist, 0.0, 0.0, 1.0 );
}`,equirect_vert:`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
}`,equirect_frag:`uniform sampler2D tEquirect;
varying vec3 vWorldDirection;
#include <common>
void main() {
	vec3 direction = normalize( vWorldDirection );
	vec2 sampleUV = equirectUv( direction );
	gl_FragColor = texture2D( tEquirect, sampleUV );
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,linedashed_vert:`uniform float scale;
attribute float lineDistance;
varying float vLineDistance;
#include <common>
#include <uv_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	vLineDistance = scale * lineDistance;
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
}`,linedashed_frag:`uniform vec3 diffuse;
uniform float opacity;
uniform float dashSize;
uniform float totalSize;
varying float vLineDistance;
#include <common>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	if ( mod( vLineDistance, totalSize ) > dashSize ) {
		discard;
	}
	vec3 outgoingLight = vec3( 0.0 );
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
}`,meshbasic_vert:`#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#if defined ( USE_ENVMAP ) || defined ( USE_SKINNING )
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinbase_vertex>
		#include <skinnormal_vertex>
		#include <defaultnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <fog_vertex>
}`,meshbasic_frag:`uniform vec3 diffuse;
uniform float opacity;
#ifndef FLAT_SHADED
	varying vec3 vNormal;
#endif
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <fog_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	#ifdef USE_LIGHTMAP
		vec4 lightMapTexel = texture2D( lightMap, vLightMapUv );
		reflectedLight.indirectDiffuse += lightMapTexel.rgb * lightMapIntensity * RECIPROCAL_PI;
	#else
		reflectedLight.indirectDiffuse += vec3( 1.0 );
	#endif
	#include <aomap_fragment>
	reflectedLight.indirectDiffuse *= diffuseColor.rgb;
	vec3 outgoingLight = reflectedLight.indirectDiffuse;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,meshlambert_vert:`#define LAMBERT
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,meshlambert_frag:`#define LAMBERT
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float opacity;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <cube_uv_reflection_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <envmap_physical_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_lambert_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_lambert_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + totalEmissiveRadiance;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,meshmatcap_vert:`#define MATCAP
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <color_pars_vertex>
#include <displacementmap_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
	vViewPosition = - mvPosition.xyz;
}`,meshmatcap_frag:`#define MATCAP
uniform vec3 diffuse;
uniform float opacity;
uniform sampler2D matcap;
varying vec3 vViewPosition;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <normal_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	vec3 viewDir = normalize( vViewPosition );
	vec3 x = normalize( vec3( viewDir.z, 0.0, - viewDir.x ) );
	vec3 y = cross( viewDir, x );
	vec2 uv = vec2( dot( x, normal ), dot( y, normal ) ) * 0.495 + 0.5;
	#ifdef USE_MATCAP
		vec4 matcapColor = texture2D( matcap, uv );
	#else
		vec4 matcapColor = vec4( vec3( mix( 0.2, 0.8, uv.y ) ), 1.0 );
	#endif
	vec3 outgoingLight = diffuseColor.rgb * matcapColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,meshnormal_vert:`#define NORMAL
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	varying vec3 vViewPosition;
#endif
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphinstance_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	vViewPosition = - mvPosition.xyz;
#endif
}`,meshnormal_frag:`#define NORMAL
uniform float opacity;
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	varying vec3 vViewPosition;
#endif
#include <uv_pars_fragment>
#include <normal_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( 0.0, 0.0, 0.0, opacity );
	#include <clipping_planes_fragment>
	#include <logdepthbuf_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	gl_FragColor = vec4( normalize( normal ) * 0.5 + 0.5, diffuseColor.a );
	#ifdef OPAQUE
		gl_FragColor.a = 1.0;
	#endif
}`,meshphong_vert:`#define PHONG
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphinstance_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,meshphong_frag:`#define PHONG
uniform vec3 diffuse;
uniform vec3 emissive;
uniform vec3 specular;
uniform float shininess;
uniform float opacity;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <cube_uv_reflection_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <envmap_physical_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_phong_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_phong_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + reflectedLight.directSpecular + reflectedLight.indirectSpecular + totalEmissiveRadiance;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,meshphysical_vert:`#define STANDARD
varying vec3 vViewPosition;
#ifdef USE_TRANSMISSION
	varying vec3 vWorldPosition;
#endif
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
#ifdef USE_TRANSMISSION
	vWorldPosition = worldPosition.xyz;
#endif
}`,meshphysical_frag:`#define STANDARD
#ifdef PHYSICAL
	#define IOR
	#define USE_SPECULAR
#endif
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float roughness;
uniform float metalness;
uniform float opacity;
#ifdef IOR
	uniform float ior;
#endif
#ifdef USE_SPECULAR
	uniform float specularIntensity;
	uniform vec3 specularColor;
	#ifdef USE_SPECULAR_COLORMAP
		uniform sampler2D specularColorMap;
	#endif
	#ifdef USE_SPECULAR_INTENSITYMAP
		uniform sampler2D specularIntensityMap;
	#endif
#endif
#ifdef USE_CLEARCOAT
	uniform float clearcoat;
	uniform float clearcoatRoughness;
#endif
#ifdef USE_DISPERSION
	uniform float dispersion;
#endif
#ifdef USE_RETROREFLECTION
	uniform float retroreflectivity;
#endif
#ifdef USE_IRIDESCENCE
	uniform float iridescence;
	uniform float iridescenceIOR;
	uniform float iridescenceThicknessMinimum;
	uniform float iridescenceThicknessMaximum;
#endif
#ifdef USE_SHEEN
	uniform vec3 sheenColor;
	uniform float sheenRoughness;
	#ifdef USE_SHEEN_COLORMAP
		uniform sampler2D sheenColorMap;
	#endif
	#ifdef USE_SHEEN_ROUGHNESSMAP
		uniform sampler2D sheenRoughnessMap;
	#endif
#endif
#ifdef USE_ANISOTROPY
	uniform vec2 anisotropyVector;
	#ifdef USE_ANISOTROPYMAP
		uniform sampler2D anisotropyMap;
	#endif
#endif
varying vec3 vViewPosition;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <iridescence_fragment>
#include <cube_uv_reflection_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_physical_pars_fragment>
#include <fog_pars_fragment>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_physical_pars_fragment>
#include <transmission_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <clearcoat_pars_fragment>
#include <iridescence_pars_fragment>
#include <roughnessmap_pars_fragment>
#include <metalnessmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <roughnessmap_fragment>
	#include <metalnessmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <clearcoat_normal_fragment_begin>
	#include <clearcoat_normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_physical_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 totalDiffuse = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse;
	vec3 totalSpecular = reflectedLight.directSpecular + reflectedLight.indirectSpecular;
	#include <transmission_fragment>
	vec3 outgoingLight = totalDiffuse + totalSpecular + totalEmissiveRadiance;
	#ifdef USE_SHEEN
 
		outgoingLight = outgoingLight + sheenSpecularDirect + sheenSpecularIndirect;
 
 	#endif
	#ifdef USE_CLEARCOAT
		float dotNVcc = saturate( dot( geometryClearcoatNormal, geometryViewDir ) );
		vec3 Fcc = F_Schlick( material.clearcoatF0, material.clearcoatF90, dotNVcc );
		outgoingLight = outgoingLight * ( 1.0 - material.clearcoat * Fcc ) + ( clearcoatSpecularDirect + clearcoatSpecularIndirect ) * material.clearcoat;
	#endif
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,meshtoon_vert:`#define TOON
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,meshtoon_frag:`#define TOON
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float opacity;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <gradientmap_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_toon_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_toon_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + totalEmissiveRadiance;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,points_vert:`uniform float size;
uniform float scale;
#include <common>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
#ifdef USE_POINTS_UV
	varying vec2 vUv;
	uniform mat3 uvTransform;
#endif
void main() {
	#ifdef USE_POINTS_UV
		vUv = ( uvTransform * vec3( uv, 1 ) ).xy;
	#endif
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <project_vertex>
	gl_PointSize = size;
	#ifdef USE_SIZEATTENUATION
		bool isPerspective = isPerspectiveMatrix( projectionMatrix );
		if ( isPerspective ) gl_PointSize *= ( scale / - mvPosition.z );
	#endif
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <worldpos_vertex>
	#include <fog_vertex>
}`,points_frag:`uniform vec3 diffuse;
uniform float opacity;
#include <common>
#include <color_pars_fragment>
#include <map_particle_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	vec3 outgoingLight = vec3( 0.0 );
	#include <logdepthbuf_fragment>
	#include <map_particle_fragment>
	#include <color_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
}`,shadow_vert:`#include <common>
#include <batching_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <shadowmap_pars_vertex>
void main() {
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphinstance_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,shadow_frag:`uniform vec3 color;
uniform float opacity;
#include <common>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <logdepthbuf_pars_fragment>
#include <shadowmap_pars_fragment>
#include <shadowmask_pars_fragment>
void main() {
	#include <logdepthbuf_fragment>
	gl_FragColor = vec4( color, opacity * ( 1.0 - getShadowMask() ) );
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
}`,sprite_vert:`uniform float rotation;
uniform vec2 center;
#include <common>
#include <uv_pars_vertex>
#include <fog_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	vec4 mvPosition = modelViewMatrix[ 3 ];
	vec2 scale = vec2( length( modelMatrix[ 0 ].xyz ), length( modelMatrix[ 1 ].xyz ) );
	#ifndef USE_SIZEATTENUATION
		bool isPerspective = isPerspectiveMatrix( projectionMatrix );
		if ( isPerspective ) scale *= - mvPosition.z;
	#endif
	vec2 alignedPosition = ( position.xy - ( center - vec2( 0.5 ) ) ) * scale;
	vec2 rotatedPosition;
	rotatedPosition.x = cos( rotation ) * alignedPosition.x - sin( rotation ) * alignedPosition.y;
	rotatedPosition.y = sin( rotation ) * alignedPosition.x + cos( rotation ) * alignedPosition.y;
	mvPosition.xy += rotatedPosition;
	gl_Position = projectionMatrix * mvPosition;
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
}`,sprite_frag:`uniform vec3 diffuse;
uniform float opacity;
#include <common>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	vec3 outgoingLight = vec3( 0.0 );
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
}`},Q={common:{diffuse:{value:new Y(16777215)},opacity:{value:1},map:{value:null},mapTransform:{value:new yn},alphaMap:{value:null},alphaMapTransform:{value:new yn},alphaTest:{value:0}},specularmap:{specularMap:{value:null},specularMapTransform:{value:new yn}},envmap:{envMap:{value:null},envMapRotation:{value:new yn},reflectivity:{value:1},ior:{value:1.5},refractionRatio:{value:.98},dfgLUT:{value:null}},aomap:{aoMap:{value:null},aoMapIntensity:{value:1},aoMapTransform:{value:new yn}},lightmap:{lightMap:{value:null},lightMapIntensity:{value:1},lightMapTransform:{value:new yn}},bumpmap:{bumpMap:{value:null},bumpMapTransform:{value:new yn},bumpScale:{value:1}},normalmap:{normalMap:{value:null},normalMapTransform:{value:new yn},normalScale:{value:new K(1,1)}},displacementmap:{displacementMap:{value:null},displacementMapTransform:{value:new yn},displacementScale:{value:1},displacementBias:{value:0}},emissivemap:{emissiveMap:{value:null},emissiveMapTransform:{value:new yn}},metalnessmap:{metalnessMap:{value:null},metalnessMapTransform:{value:new yn}},roughnessmap:{roughnessMap:{value:null},roughnessMapTransform:{value:new yn}},gradientmap:{gradientMap:{value:null}},fog:{fogDensity:{value:25e-5},fogNear:{value:1},fogFar:{value:2e3},fogColor:{value:new Y(16777215)}},lights:{ambientLightColor:{value:[]},lightProbe:{value:[]},sunLights:{value:[],properties:{direction:{},color:{}}},sunLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},sunShadowMatrix:{value:[]},sunShadowCascade:{value:[]},directionalLights:{value:[],properties:{direction:{},color:{}}},directionalLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},directionalShadowMatrix:{value:[]},spotLights:{value:[],properties:{color:{},position:{},direction:{},distance:{},coneCos:{},penumbraCos:{},decay:{}}},spotLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},spotLightMap:{value:[]},spotLightMatrix:{value:[]},pointLights:{value:[],properties:{color:{},position:{},decay:{},distance:{}}},pointLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{},shadowCameraNear:{},shadowCameraFar:{}}},pointShadowMatrix:{value:[]},hemisphereLights:{value:[],properties:{direction:{},skyColor:{},groundColor:{}}},rectAreaLights:{value:[],properties:{color:{},position:{},width:{},height:{}}},ltc_1:{value:null},ltc_2:{value:null},probesSH:{value:null},probesMin:{value:new q},probesMax:{value:new q},probesResolution:{value:new q}},points:{diffuse:{value:new Y(16777215)},opacity:{value:1},size:{value:1},scale:{value:1},map:{value:null},alphaMap:{value:null},alphaMapTransform:{value:new yn},alphaTest:{value:0},uvTransform:{value:new yn}},sprite:{diffuse:{value:new Y(16777215)},opacity:{value:1},center:{value:new K(.5,.5)},rotation:{value:0},map:{value:null},mapTransform:{value:new yn},alphaMap:{value:null},alphaMapTransform:{value:new yn},alphaTest:{value:0}}},zf={basic:{uniforms:Gc([Q.common,Q.specularmap,Q.envmap,Q.aomap,Q.lightmap,Q.fog]),vertexShader:Rf.meshbasic_vert,fragmentShader:Rf.meshbasic_frag},lambert:{uniforms:Gc([Q.common,Q.specularmap,Q.envmap,Q.aomap,Q.lightmap,Q.emissivemap,Q.bumpmap,Q.normalmap,Q.displacementmap,Q.fog,Q.lights,{emissive:{value:new Y(0)},envMapIntensity:{value:1}}]),vertexShader:Rf.meshlambert_vert,fragmentShader:Rf.meshlambert_frag},phong:{uniforms:Gc([Q.common,Q.specularmap,Q.envmap,Q.aomap,Q.lightmap,Q.emissivemap,Q.bumpmap,Q.normalmap,Q.displacementmap,Q.fog,Q.lights,{emissive:{value:new Y(0)},specular:{value:new Y(1118481)},shininess:{value:30},envMapIntensity:{value:1}}]),vertexShader:Rf.meshphong_vert,fragmentShader:Rf.meshphong_frag},standard:{uniforms:Gc([Q.common,Q.envmap,Q.aomap,Q.lightmap,Q.emissivemap,Q.bumpmap,Q.normalmap,Q.displacementmap,Q.roughnessmap,Q.metalnessmap,Q.fog,Q.lights,{emissive:{value:new Y(0)},roughness:{value:1},metalness:{value:0},envMapIntensity:{value:1}}]),vertexShader:Rf.meshphysical_vert,fragmentShader:Rf.meshphysical_frag},toon:{uniforms:Gc([Q.common,Q.aomap,Q.lightmap,Q.emissivemap,Q.bumpmap,Q.normalmap,Q.displacementmap,Q.gradientmap,Q.fog,Q.lights,{emissive:{value:new Y(0)}}]),vertexShader:Rf.meshtoon_vert,fragmentShader:Rf.meshtoon_frag},matcap:{uniforms:Gc([Q.common,Q.bumpmap,Q.normalmap,Q.displacementmap,Q.fog,{matcap:{value:null}}]),vertexShader:Rf.meshmatcap_vert,fragmentShader:Rf.meshmatcap_frag},points:{uniforms:Gc([Q.points,Q.fog]),vertexShader:Rf.points_vert,fragmentShader:Rf.points_frag},dashed:{uniforms:Gc([Q.common,Q.fog,{scale:{value:1},dashSize:{value:1},totalSize:{value:2}}]),vertexShader:Rf.linedashed_vert,fragmentShader:Rf.linedashed_frag},depth:{uniforms:Gc([Q.common,Q.displacementmap]),vertexShader:Rf.depth_vert,fragmentShader:Rf.depth_frag},normal:{uniforms:Gc([Q.common,Q.bumpmap,Q.normalmap,Q.displacementmap,{opacity:{value:1}}]),vertexShader:Rf.meshnormal_vert,fragmentShader:Rf.meshnormal_frag},sprite:{uniforms:Gc([Q.sprite,Q.fog]),vertexShader:Rf.sprite_vert,fragmentShader:Rf.sprite_frag},background:{uniforms:{uvTransform:{value:new yn},t2D:{value:null},backgroundIntensity:{value:1}},vertexShader:Rf.background_vert,fragmentShader:Rf.background_frag},backgroundCube:{uniforms:{envMap:{value:null},backgroundBlurriness:{value:0},backgroundIntensity:{value:1},backgroundRotation:{value:new yn}},vertexShader:Rf.backgroundCube_vert,fragmentShader:Rf.backgroundCube_frag},cube:{uniforms:{tCube:{value:null},tFlip:{value:-1},opacity:{value:1}},vertexShader:Rf.cube_vert,fragmentShader:Rf.cube_frag},equirect:{uniforms:{tEquirect:{value:null}},vertexShader:Rf.equirect_vert,fragmentShader:Rf.equirect_frag},distance:{uniforms:Gc([Q.common,Q.displacementmap,{referencePosition:{value:new q},nearDistance:{value:1},farDistance:{value:1e3}}]),vertexShader:Rf.distance_vert,fragmentShader:Rf.distance_frag},shadow:{uniforms:Gc([Q.lights,Q.fog,{color:{value:new Y(0)},opacity:{value:1}}]),vertexShader:Rf.shadow_vert,fragmentShader:Rf.shadow_frag}};zf.physical={uniforms:Gc([zf.standard.uniforms,{clearcoat:{value:0},clearcoatMap:{value:null},clearcoatMapTransform:{value:new yn},clearcoatNormalMap:{value:null},clearcoatNormalMapTransform:{value:new yn},clearcoatNormalScale:{value:new K(1,1)},clearcoatRoughness:{value:0},clearcoatRoughnessMap:{value:null},clearcoatRoughnessMapTransform:{value:new yn},dispersion:{value:0},retroreflectivity:{value:0},iridescence:{value:0},iridescenceMap:{value:null},iridescenceMapTransform:{value:new yn},iridescenceIOR:{value:1.3},iridescenceThicknessMinimum:{value:100},iridescenceThicknessMaximum:{value:400},iridescenceThicknessMap:{value:null},iridescenceThicknessMapTransform:{value:new yn},sheen:{value:0},sheenColor:{value:new Y(0)},sheenColorMap:{value:null},sheenColorMapTransform:{value:new yn},sheenRoughness:{value:1},sheenRoughnessMap:{value:null},sheenRoughnessMapTransform:{value:new yn},transmission:{value:0},transmissionMap:{value:null},transmissionMapTransform:{value:new yn},transmissionSamplerSize:{value:new K},transmissionSamplerMap:{value:null},thickness:{value:0},thicknessMap:{value:null},thicknessMapTransform:{value:new yn},attenuationDistance:{value:0},attenuationColor:{value:new Y(0)},specularColor:{value:new Y(1,1,1)},specularColorMap:{value:null},specularColorMapTransform:{value:new yn},specularIntensity:{value:1},specularIntensityMap:{value:null},specularIntensityMapTransform:{value:new yn},anisotropyVector:{value:new K},anisotropyMap:{value:null},anisotropyMapTransform:{value:new yn}}]),vertexShader:Rf.meshphysical_vert,fragmentShader:Rf.meshphysical_frag};var Bf={r:0,b:0,g:0},Vf=new J,Hf=new yn;Hf.set(-1,0,0,0,1,0,0,0,1);function Uf(e,t,n,r,i,a){let o=new Y(0),s=i===!0?0:1,c,l,u=null,d=0,f=null;function p(e){let n=e.isScene===!0?e.background:null;if(n&&n.isTexture){let r=e.backgroundBlurriness>0;n=t.get(n,r)}return n}function m(t){let r=!1,i=p(t);i===null?g(o,s):i&&i.isColor&&(g(i,1),r=!0);let c=e.xr.getEnvironmentBlendMode();c===`additive`?n.buffers.color.setClear(0,0,0,1,a):c===`alpha-blend`&&n.buffers.color.setClear(0,0,0,0,a),(e.autoClear||r)&&(n.buffers.depth.setTest(!0),n.buffers.depth.setMask(!0),n.buffers.color.setMask(!0),e.clear(e.autoClearColor,e.autoClearDepth,e.autoClearStencil))}function h(t,n){let i=p(n);i&&(i.isCubeTexture||i.mapping===306)?(l===void 0&&(l=new xa(new $o(1,1,1),new Qc({name:`BackgroundCubeMaterial`,uniforms:Wc(zf.backgroundCube.uniforms),vertexShader:zf.backgroundCube.vertexShader,fragmentShader:zf.backgroundCube.fragmentShader,side:1,depthTest:!1,depthWrite:!1,fog:!1,allowOverride:!1})),l.geometry.deleteAttribute(`normal`),l.geometry.deleteAttribute(`uv`),l.onBeforeRender=function(e,t,n){this.matrixWorld.copyPosition(n.matrixWorld)},Object.defineProperty(l.material,"envMap",{get:function(){return this.uniforms.envMap.value}}),r.update(l)),l.material.uniforms.envMap.value=i,l.material.uniforms.backgroundBlurriness.value=n.backgroundBlurriness,l.material.uniforms.backgroundIntensity.value=n.backgroundIntensity,l.material.uniforms.backgroundRotation.value.setFromMatrix4(Vf.makeRotationFromEuler(n.backgroundRotation)).transpose(),i.isCubeTexture&&i.isRenderTargetTexture===!1&&l.material.uniforms.backgroundRotation.value.premultiply(Hf),l.material.toneMapped=wn.getTransfer(i.colorSpace)!==nt,(u!==i||d!==i.version||f!==e.toneMapping)&&(l.material.needsUpdate=!0,u=i,d=i.version,f=e.toneMapping),l.layers.enableAll(),t.unshift(l,l.geometry,l.material,0,0,null)):i&&i.isTexture&&(c===void 0&&(c=new xa(new jc(2,2),new Qc({name:`BackgroundMaterial`,uniforms:Wc(zf.background.uniforms),vertexShader:zf.background.vertexShader,fragmentShader:zf.background.fragmentShader,side:0,depthTest:!1,depthWrite:!1,fog:!1,allowOverride:!1})),c.geometry.deleteAttribute(`normal`),Object.defineProperty(c.material,"map",{get:function(){return this.uniforms.t2D.value}}),r.update(c)),c.material.uniforms.t2D.value=i,c.material.uniforms.backgroundIntensity.value=n.backgroundIntensity,c.material.toneMapped=wn.getTransfer(i.colorSpace)!==nt,i.matrixAutoUpdate===!0&&i.updateMatrix(),c.material.uniforms.uvTransform.value.copy(i.matrix),(u!==i||d!==i.version||f!==e.toneMapping)&&(c.material.needsUpdate=!0,u=i,d=i.version,f=e.toneMapping),c.layers.enableAll(),t.unshift(c,c.geometry,c.material,0,0,null))}function g(t,r){t.getRGB(Bf,Jc(e)),n.buffers.color.setClear(Bf.r,Bf.g,Bf.b,r,a)}function _(){l!==void 0&&(l.geometry.dispose(),l.material.dispose(),l=void 0),c!==void 0&&(c.geometry.dispose(),c.material.dispose(),c=void 0)}return{getClearColor:function(){return o},setClearColor:function(e,t=1){o.set(e),s=t,g(o,s)},getClearAlpha:function(){return s},setClearAlpha:function(e){s=e,g(o,s)},render:m,addToRenderList:h,dispose:_}}function Wf(e,t){let n=e.getParameter(e.MAX_VERTEX_ATTRIBS),r={},i=f(null),a=i,o=!1;function s(n,r,i,s,c){let u=!1,f=d(n,s,i,r);a!==f&&(a=f,l(a.object)),u=p(n,s,i,c),u&&m(n,s,i,c),c!==null&&t.update(c,e.ELEMENT_ARRAY_BUFFER),(u||o)&&(o=!1,b(n,r,i,s),c!==null&&e.bindBuffer(e.ELEMENT_ARRAY_BUFFER,t.get(c).buffer))}function c(){return e.createVertexArray()}function l(t){return e.bindVertexArray(t)}function u(t){return e.deleteVertexArray(t)}function d(e,t,n,i){let a=i.wireframe===!0,o=r[t.id];o===void 0&&(o={},r[t.id]=o);let s=e.isInstancedMesh===!0?e.id:0,l=o[s];l===void 0&&(l={},o[s]=l);let u=l[n.id];u===void 0&&(u={},l[n.id]=u);let d=u[a];return d===void 0&&(d=f(c()),u[a]=d),d}function f(e){let t=[],r=[],i=[];for(let e=0;e<n;e++)t[e]=0,r[e]=0,i[e]=0;return{geometry:null,program:null,wireframe:!1,newAttributes:t,enabledAttributes:r,attributeDivisors:i,object:e,attributes:{},index:null}}function p(e,t,n,r){let i=a.attributes,o=t.attributes,s=0,c=n.getAttributes();for(let t in c)if(c[t].location>=0){let n=i[t],r=o[t];if(r===void 0&&(t===`instanceMatrix`&&e.instanceMatrix&&(r=e.instanceMatrix),t===`instanceColor`&&e.instanceColor&&(r=e.instanceColor)),n===void 0||n.attribute!==r||r&&n.data!==r.data)return!0;s++}return a.attributesNum!==s||a.index!==r}function m(e,t,n,r){let i={},o=t.attributes,s=0,c=n.getAttributes();for(let t in c)if(c[t].location>=0){let n=o[t];n===void 0&&(t===`instanceMatrix`&&e.instanceMatrix&&(n=e.instanceMatrix),t===`instanceColor`&&e.instanceColor&&(n=e.instanceColor));let r={};r.attribute=n,n&&n.data&&(r.data=n.data),i[t]=r,s++}a.attributes=i,a.attributesNum=s,a.index=r}function h(){let e=a.newAttributes;for(let t=0,n=e.length;t<n;t++)e[t]=0}function g(e){_(e,0)}function _(t,n){let r=a.newAttributes,i=a.enabledAttributes,o=a.attributeDivisors;r[t]=1,i[t]===0&&(e.enableVertexAttribArray(t),i[t]=1),o[t]!==n&&(e.vertexAttribDivisor(t,n),o[t]=n)}function v(){let t=a.newAttributes,n=a.enabledAttributes;for(let r=0,i=n.length;r<i;r++)n[r]!==t[r]&&(e.disableVertexAttribArray(r),n[r]=0)}function y(t,n,r,i,a,o,s){s===!0?e.vertexAttribIPointer(t,n,r,a,o):e.vertexAttribPointer(t,n,r,i,a,o)}function b(n,r,i,a){h();let o=a.attributes,s=i.getAttributes(),c=r.defaultAttributeValues;for(let r in s){let i=s[r];if(i.location>=0){let s=o[r];if(s===void 0&&(r===`instanceMatrix`&&n.instanceMatrix&&(s=n.instanceMatrix),r===`instanceColor`&&n.instanceColor&&(s=n.instanceColor)),s!==void 0){let r=s.normalized,o=s.itemSize,c=t.get(s);if(c===void 0)continue;let l=c.buffer,u=c.type,d=c.bytesPerElement,f=u===e.INT||u===e.UNSIGNED_INT||s.gpuType===1013;if(s.isInterleavedBufferAttribute){let t=s.data,c=t.stride,p=s.offset;if(t.isInstancedInterleavedBuffer){for(let e=0;e<i.locationSize;e++)_(i.location+e,t.meshPerAttribute);n.isInstancedMesh!==!0&&a._maxInstanceCount===void 0&&(a._maxInstanceCount=t.meshPerAttribute*t.count)}else for(let e=0;e<i.locationSize;e++)g(i.location+e);e.bindBuffer(e.ARRAY_BUFFER,l);for(let e=0;e<i.locationSize;e++)y(i.location+e,o/i.locationSize,u,r,c*d,(p+o/i.locationSize*e)*d,f)}else{if(s.isInstancedBufferAttribute){for(let e=0;e<i.locationSize;e++)_(i.location+e,s.meshPerAttribute);n.isInstancedMesh!==!0&&a._maxInstanceCount===void 0&&(a._maxInstanceCount=s.meshPerAttribute*s.count)}else for(let e=0;e<i.locationSize;e++)g(i.location+e);e.bindBuffer(e.ARRAY_BUFFER,l);for(let e=0;e<i.locationSize;e++)y(i.location+e,o/i.locationSize,u,r,o*d,o/i.locationSize*e*d,f)}}else if(c!==void 0){let t=c[r];if(t!==void 0)switch(t.length){case 2:e.vertexAttrib2fv(i.location,t);break;case 3:e.vertexAttrib3fv(i.location,t);break;case 4:e.vertexAttrib4fv(i.location,t);break;default:e.vertexAttrib1fv(i.location,t)}}}}v()}function x(){T();for(let e in r){let t=r[e];for(let e in t){let n=t[e];for(let e in n){let t=n[e];for(let e in t)u(t[e].object),delete t[e];delete n[e]}}delete r[e]}}function S(e){if(r[e.id]===void 0)return;let t=r[e.id];for(let e in t){let n=t[e];for(let e in n){let t=n[e];for(let e in t)u(t[e].object),delete t[e];delete n[e]}}delete r[e.id]}function C(e){for(let t in r){let n=r[t];for(let t in n){let r=n[t];if(r[e.id]===void 0)continue;let i=r[e.id];for(let e in i)u(i[e].object),delete i[e];delete r[e.id]}}}function w(e){for(let t in r){let n=r[t],i=e.isInstancedMesh===!0?e.id:0,a=n[i];if(a!==void 0){for(let e in a){let t=a[e];for(let e in t)u(t[e].object),delete t[e];delete a[e]}delete n[i],Object.keys(n).length===0&&delete r[t]}}}function T(){E(),o=!0,a!==i&&(a=i,l(a.object))}function E(){i.geometry=null,i.program=null,i.wireframe=!1}return{setup:s,reset:T,resetDefaultState:E,dispose:x,releaseStatesOfGeometry:S,releaseStatesOfObject:w,releaseStatesOfProgram:C,initAttributes:h,enableAttribute:g,disableUnusedAttributes:v}}function Gf(e,t,n){let r;function i(e){r=e}function a(t,i){e.drawArrays(r,t,i),n.update(i,r,1)}function o(t,i,a){a!==0&&(e.drawArraysInstanced(r,t,i,a),n.update(i,r,a))}function s(e,i,a){if(a===0)return;t.get(`WEBGL_multi_draw`).multiDrawArraysWEBGL(r,e,0,i,0,a);let o=0;for(let e=0;e<a;e++)o+=i[e];n.update(o,r,1)}this.setMode=i,this.render=a,this.renderInstances=o,this.renderMultiDraw=s}function Kf(e,t,n,r){let i;function a(){if(i!==void 0)return i;if(t.has(`EXT_texture_filter_anisotropic`)===!0){let n=t.get(`EXT_texture_filter_anisotropic`);i=e.getParameter(n.MAX_TEXTURE_MAX_ANISOTROPY_EXT)}else i=0;return i}function o(t){return t===1023||r.convert(t)===e.getParameter(e.IMPLEMENTATION_COLOR_READ_FORMAT)}function s(n){let i=n===1016&&(t.has(`EXT_color_buffer_half_float`)||t.has(`EXT_color_buffer_float`));return!(n!==1009&&n!==1015&&!i&&r.convert(n)!==e.getParameter(e.IMPLEMENTATION_COLOR_READ_TYPE))}function c(t){if(t===`highp`){if(e.getShaderPrecisionFormat(e.VERTEX_SHADER,e.HIGH_FLOAT).precision>0&&e.getShaderPrecisionFormat(e.FRAGMENT_SHADER,e.HIGH_FLOAT).precision>0)return`highp`;t=`mediump`}return t===`mediump`&&e.getShaderPrecisionFormat(e.VERTEX_SHADER,e.MEDIUM_FLOAT).precision>0&&e.getShaderPrecisionFormat(e.FRAGMENT_SHADER,e.MEDIUM_FLOAT).precision>0?`mediump`:`lowp`}let l=n.precision===void 0?`highp`:n.precision,u=c(l);u!==l&&(U(`WebGLRenderer:`,l,`not supported, using`,u,`instead.`),l=u);let d=n.logarithmicDepthBuffer===!0,f=n.reversedDepthBuffer===!0&&t.has(`EXT_clip_control`);n.reversedDepthBuffer===!0&&f===!1&&U(`WebGLRenderer: Unable to use reversed depth buffer due to missing EXT_clip_control extension. Fallback to default depth buffer.`);let p=e.getParameter(e.MAX_TEXTURE_IMAGE_UNITS),m=e.getParameter(e.MAX_VERTEX_TEXTURE_IMAGE_UNITS),h=e.getParameter(e.MAX_TEXTURE_SIZE),g=e.getParameter(e.MAX_CUBE_MAP_TEXTURE_SIZE),_=e.getParameter(e.MAX_VERTEX_ATTRIBS),v=e.getParameter(e.MAX_VERTEX_UNIFORM_VECTORS),y=e.getParameter(e.MAX_VARYING_VECTORS),b=e.getParameter(e.MAX_FRAGMENT_UNIFORM_VECTORS),x=e.getParameter(e.MAX_SAMPLES),S=e.getParameter(e.SAMPLES);return{isWebGL2:!0,getMaxAnisotropy:a,getMaxPrecision:c,textureFormatReadable:o,textureTypeReadable:s,precision:l,logarithmicDepthBuffer:d,reversedDepthBuffer:f,maxTextures:p,maxVertexTextures:m,maxTextureSize:h,maxCubemapSize:g,maxAttributes:_,maxVertexUniforms:v,maxVaryings:y,maxFragmentUniforms:b,maxSamples:x,samples:S}}function qf(e){let t=this,n=null,r=0,i=!1,a=!1,o=new Ii,s=new yn,c={value:null,needsUpdate:!1};this.uniform=c,this.numPlanes=0,this.numIntersection=0,this.init=function(e,t){let n=e.length!==0||t||r!==0||i;return i=t,r=e.length,n},this.beginShadows=function(){a=!0,u(null)},this.endShadows=function(){a=!1},this.setGlobalState=function(e,t){n=u(e,t,0)},this.setState=function(t,o,s){let d=t.clippingPlanes,f=t.clipIntersection,p=t.clipShadows,m=e.get(t);if(!i||d===null||d.length===0||a&&!p)a?u(null):l();else{let e=a?0:r,t=e*4,i=m.clippingState||null;c.value=i,i=u(d,o,t,s);for(let e=0;e!==t;++e)i[e]=n[e];m.clippingState=i,this.numIntersection=f?this.numPlanes:0,this.numPlanes+=e}};function l(){c.value!==n&&(c.value=n,c.needsUpdate=r>0),t.numPlanes=r,t.numIntersection=0}function u(e,n,r,i){let a=e===null?0:e.length,l=null;if(a!==0){if(l=c.value,i!==!0||l===null){let t=r+a*4,i=n.matrixWorldInverse;s.getNormalMatrix(i),(l===null||l.length<t)&&(l=new Float32Array(t));for(let t=0,n=r;t!==a;++t,n+=4)o.copy(e[t]).applyMatrix4(i,s),o.normal.toArray(l,n),l[n+3]=o.constant}c.value=l,c.needsUpdate=!0}return t.numPlanes=a,t.numIntersection=0,l}}var Jf=4,Yf=6,Xf=20,Zf=256,Qf=new _u,$f=new Y,ep=null,tp=0,np=0,rp=!1,ip=new q,ap=new q,op=class{constructor(e){this._renderer=e,this._pingPongRenderTarget=null,this._lodMax=0,this._cubeSize=0,this._sizeLods=[],this._lodMeshes=[],this._backgroundBox=null,this._cubemapMaterial=null,this._equirectMaterial=null,this._blurMaterial=null,this._ggxMaterial=null}fromScene(e,t=0,n=.1,r=100,i={}){let{size:a=256,position:o=ip}=i;ep=this._renderer.getRenderTarget(),tp=this._renderer.getActiveCubeFace(),np=this._renderer.getActiveMipmapLevel(),rp=this._renderer.xr.enabled,this._renderer.xr.enabled=!1,this._setSize(a);let s=this._allocateTargets();return s.depthBuffer=!0,this._sceneToCubeUV(e,n,r,s,o),t>0&&this._blur(s,0,0,t),this._applyPMREM(s),this._cleanup(s),s}fromEquirectangular(e,t=null){return this._fromTexture(e,t)}fromCubemap(e,t=null){return this._fromTexture(e,t)}compileCubemapShader(){this._cubemapMaterial===null&&(this._cubemapMaterial=pp(),this._compileMaterial(this._cubemapMaterial))}compileEquirectangularShader(){this._equirectMaterial===null&&(this._equirectMaterial=fp(),this._compileMaterial(this._equirectMaterial))}dispose(){this._dispose(),this._cubemapMaterial!==null&&this._cubemapMaterial.dispose(),this._equirectMaterial!==null&&this._equirectMaterial.dispose(),this._backgroundBox!==null&&(this._backgroundBox.geometry.dispose(),this._backgroundBox.material.dispose())}_setSize(e){this._lodMax=Math.floor(Math.log2(e)),this._cubeSize=2**this._lodMax}_dispose(){this._blurMaterial!==null&&this._blurMaterial.dispose(),this._ggxMaterial!==null&&this._ggxMaterial.dispose(),this._pingPongRenderTarget!==null&&this._pingPongRenderTarget.dispose();for(let e=0;e<this._lodMeshes.length;e++)this._lodMeshes[e].geometry.dispose()}_cleanup(e){this._renderer.setRenderTarget(ep,tp,np),this._renderer.xr.enabled=rp,e.scissorTest=!1,lp(e,0,0,e.width,e.height)}_fromTexture(e,t){e.mapping===301||e.mapping===302?this._setSize(e.image.length===0?16:e.image[0].width||e.image[0].image.width):this._setSize(e.image.width/4),ep=this._renderer.getRenderTarget(),tp=this._renderer.getActiveCubeFace(),np=this._renderer.getActiveMipmapLevel(),rp=this._renderer.xr.enabled,this._renderer.xr.enabled=!1;let n=t||this._allocateTargets();return this._textureToCubeUV(e,n),this._applyPMREM(n),this._cleanup(n),n}_allocateTargets(){let e=3*Math.max(this._cubeSize,112),t=4*this._cubeSize,n={magFilter:h,minFilter:h,generateMipmaps:!1,type:D,format:F,colorSpace:et,depthBuffer:!1},r=cp(e,t,n);if(this._pingPongRenderTarget===null||this._pingPongRenderTarget.width!==e||this._pingPongRenderTarget.height!==t){this._pingPongRenderTarget!==null&&this._dispose(),this._pingPongRenderTarget=cp(e,t,n);let{_lodMax:r}=this;({lodMeshes:this._lodMeshes,sizeLods:this._sizeLods}=sp(r)),this._blurMaterial=dp(r,e,t),this._ggxMaterial=up(r,e,t)}return r}_compileMaterial(e){let t=new xa(new Z,e);this._renderer.compile(t,Qf)}_sceneToCubeUV(e,t,n,r,i){let a=new fu(90,1,t,n),o=[1,-1,1,1,1,1],s=[1,1,1,-1,-1,-1],c=this._renderer,l=c.autoClear,u=c.toneMapping;c.getClearColor($f),c.toneMapping=0,c.autoClear=!1,c.state.buffers.depth.getReversed()&&(c.setRenderTarget(r),c.clearDepth(),c.setRenderTarget(null)),this._backgroundBox===null&&(this._backgroundBox=new xa(new $o,new la({name:`PMREM.Background`,side:1,depthWrite:!1,depthTest:!1})));let d=this._backgroundBox,f=d.material,p=!1,m=e.background;m?m.isColor&&(f.color.copy(m),e.background=null,p=!0):(f.color.copy($f),p=!0);for(let t=0;t<6;t++){let n=t%3;n===0?(a.up.set(0,o[t],0),a.position.set(i.x,i.y,i.z),a.lookAt(i.x+s[t],i.y,i.z)):n===1?(a.up.set(0,0,o[t]),a.position.set(i.x,i.y,i.z),a.lookAt(i.x,i.y+s[t],i.z)):(a.up.set(0,o[t],0),a.position.set(i.x,i.y,i.z),a.lookAt(i.x,i.y,i.z+s[t]));let l=this._cubeSize;lp(r,n*l,t>2?l:0,l,l),c.setRenderTarget(r),p&&c.render(d,a),c.render(e,a)}c.toneMapping=u,c.autoClear=l,e.background=m}_textureToCubeUV(e,t){let n=this._renderer,r=e.mapping===301||e.mapping===302;r?(this._cubemapMaterial===null&&(this._cubemapMaterial=pp()),this._cubemapMaterial.uniforms.flipEnvMap.value=e.isRenderTargetTexture===!1?-1:1):this._equirectMaterial===null&&(this._equirectMaterial=fp());let i=r?this._cubemapMaterial:this._equirectMaterial,a=this._lodMeshes[0];a.material=i;let o=i.uniforms;o.envMap.value=e;let s=this._cubeSize;lp(t,0,0,3*s,2*s),n.setRenderTarget(t),n.render(a,Qf)}_applyPMREM(e){let t=this._renderer,n=t.autoClear;t.autoClear=!1;let r=this._lodMeshes.length;for(let t=1;t<r;t++)this._applyGGXFilter(e,t-1,t);t.autoClear=n}_applyGGXFilter(e,t,n){let r=this._renderer,i=this._pingPongRenderTarget,a=this._ggxMaterial,o=this._lodMeshes[n];o.material=a;let s=a.uniforms,c=n/(this._lodMeshes.length-1),l=t/(this._lodMeshes.length-1),u=Math.sqrt(c*c-l*l)*(c*1.25),{_lodMax:d}=this,f=this._sizeLods[n],p=3*f*(n>d-Jf?n-d+Jf:0),m=4*(this._cubeSize-f);s.envMap.value=e.texture,s.roughness.value=u,s.mipInt.value=d-t,lp(i,p,m,3*f,2*f),r.setRenderTarget(i),r.render(o,Qf),s.envMap.value=i.texture,s.roughness.value=0,s.mipInt.value=d-n,lp(e,p,m,3*f,2*f),r.setRenderTarget(e),r.render(o,Qf)}_blur(e,t,n,r){let i=this._pingPongRenderTarget,a=Math.min(r,Math.PI)/Math.SQRT2;this._blurPass(e,i,t,n,a),this._blurPass(i,e,n,n,a)}_blurPass(e,t,n,r,i){let a=this._renderer,o=this._blurMaterial,s=this._lodMeshes[r];s.material=o;let c=o.uniforms;c.envMap.value=e.texture,c.sigma.value=i,c.mipInt.value=this._lodMax-n;let l=this._sizeLods[r];lp(t,3*l*(r>this._lodMax-Jf?r-this._lodMax+Jf:0),4*(this._cubeSize-l),3*l,2*l),a.setRenderTarget(t),a.render(s,Qf)}};function sp(e){let t=[],n=[],r=e,i=e-Jf+1+Yf;for(let e=0;e<i;e++){let e=2**r;t.push(e);let i=1/(e-2),a=-i,o=1+i,s=[a,a,o,a,o,o,a,a,o,o,a,o],c=new Float32Array(108),l=new Float32Array(108);for(let e=0;e<6;e++){let t=e%3*2/3-1,n=e>2?0:-1,r=[t,n,0,t+2/3,n,0,t+2/3,n+1,0,t,n,0,t+2/3,n+1,0,t,n+1,0];c.set(r,18*e);for(let t=0;t<6;t++){let n=s[t*2]*2-1,r=s[t*2+1]*2-1;e===0?ap.set(1,r,n):e===1?ap.set(-n,1,-r):e===2?ap.set(-n,r,1):e===3?ap.set(-1,r,-n):e===4?ap.set(-n,-1,r):ap.set(n,r,-1),ap.toArray(l,(e*6+t)*3)}}let u=new Z;u.setAttribute(`position`,new ui(c,3)),u.setAttribute(`outputDirection`,new ui(l,3)),n.push(new xa(u,null)),r>Jf&&r--}return{lodMeshes:n,sizeLods:t}}function cp(e,t,n){let r=new Rn(e,t,n);return r.texture.mapping=306,r.texture.name=`PMREM.cubeUv`,r.scissorTest=!0,r}function lp(e,t,n,r,i){e.viewport.set(t,n,r,i),e.scissor.set(t,n,r,i)}function up(e,t,n){return new Qc({name:`PMREMGGXConvolution`,defines:{GGX_SAMPLES:Zf,CUBEUV_TEXEL_WIDTH:1/t,CUBEUV_TEXEL_HEIGHT:1/n,CUBEUV_MAX_MIP:`${e}.0`},uniforms:{envMap:{value:null},roughness:{value:0},mipInt:{value:0}},vertexShader:mp(),fragmentShader:`

			precision highp float;
			precision highp int;

			varying vec3 vOutputDirection;

			uniform sampler2D envMap;
			uniform float roughness;
			uniform float mipInt;

			#define ENVMAP_TYPE_CUBE_UV
			#include <cube_uv_reflection_fragment>

			#define PI 3.14159265359

			// Van der Corput radical inverse
			float radicalInverse_VdC(uint bits) {
				bits = (bits << 16u) | (bits >> 16u);
				bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);
				bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);
				bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);
				bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);
				return float(bits) * 2.3283064365386963e-10; // / 0x100000000
			}

			// Hammersley sequence
			vec2 hammersley(uint i, uint N) {
				return vec2(float(i) / float(N), radicalInverse_VdC(i));
			}

			// GGX VNDF importance sampling (Eric Heitz 2018)
			// "Sampling the GGX Distribution of Visible Normals"
			// https://jcgt.org/published/0007/04/01/
			vec3 importanceSampleGGX_VNDF(vec2 Xi, vec3 V, float roughness) {
				float alpha = roughness * roughness;

				// Section 4.1: Orthonormal basis
				vec3 T1 = vec3(1.0, 0.0, 0.0);
				vec3 T2 = cross(V, T1);

				// Section 4.2: Parameterization of projected area
				float r = sqrt(Xi.x);
				float phi = 2.0 * PI * Xi.y;
				float t1 = r * cos(phi);
				float t2 = r * sin(phi);
				float s = 0.5 * (1.0 + V.z);
				t2 = (1.0 - s) * sqrt(1.0 - t1 * t1) + s * t2;

				// Section 4.3: Reprojection onto hemisphere
				vec3 Nh = t1 * T1 + t2 * T2 + sqrt(max(0.0, 1.0 - t1 * t1 - t2 * t2)) * V;

				// Section 3.4: Transform back to ellipsoid configuration
				return normalize(vec3(alpha * Nh.x, alpha * Nh.y, max(0.0, Nh.z)));
			}

			void main() {
				vec3 N = normalize(vOutputDirection);
				vec3 V = N; // Assume view direction equals normal for pre-filtering

				vec3 prefilteredColor = vec3(0.0);
				float totalWeight = 0.0;

				// For very low roughness, just sample the environment directly
				if (roughness < 0.001) {
					gl_FragColor = vec4(bilinearCubeUV(envMap, N, mipInt), 1.0);
					return;
				}

				// Tangent space basis for VNDF sampling
				vec3 up = abs(N.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);
				vec3 tangent = normalize(cross(up, N));
				vec3 bitangent = cross(N, tangent);

				for(uint i = 0u; i < uint(GGX_SAMPLES); i++) {
					vec2 Xi = hammersley(i, uint(GGX_SAMPLES));

					// For PMREM, V = N, so in tangent space V is always (0, 0, 1)
					vec3 H_tangent = importanceSampleGGX_VNDF(Xi, vec3(0.0, 0.0, 1.0), roughness);

					// Transform H back to world space
					vec3 H = normalize(tangent * H_tangent.x + bitangent * H_tangent.y + N * H_tangent.z);
					vec3 L = normalize(2.0 * dot(V, H) * H - V);

					float NdotL = max(dot(N, L), 0.0);

					if(NdotL > 0.0) {
						// Sample environment at fixed mip level
						// VNDF importance sampling handles the distribution filtering
						vec3 sampleColor = bilinearCubeUV(envMap, L, mipInt);

						// Weight by NdotL for the split-sum approximation
						// VNDF PDF naturally accounts for the visible microfacet distribution
						prefilteredColor += sampleColor * NdotL;
						totalWeight += NdotL;
					}
				}

				if (totalWeight > 0.0) {
					prefilteredColor = prefilteredColor / totalWeight;
				}

				gl_FragColor = vec4(prefilteredColor, 1.0);
			}
		`,blending:0,depthTest:!1,depthWrite:!1})}function dp(e,t,n){return new Qc({name:`SphericalGaussianBlur`,defines:{SAMPLES:Xf,CUBEUV_TEXEL_WIDTH:1/t,CUBEUV_TEXEL_HEIGHT:1/n,CUBEUV_MAX_MIP:`${e}.0`},uniforms:{envMap:{value:null},sigma:{value:0},mipInt:{value:0}},vertexShader:mp(),fragmentShader:`

			precision highp float;
			precision highp int;

			varying vec3 vOutputDirection;

			uniform sampler2D envMap;
			uniform float sigma;
			uniform float mipInt;

			#define ENVMAP_TYPE_CUBE_UV
			#include <cube_uv_reflection_fragment>

			#define PI 3.14159265359
			#define GOLDEN_ANGLE 2.39996322973

			void main() {

				if ( sigma == 0.0 ) {

					gl_FragColor = vec4( bilinearCubeUV( envMap, vOutputDirection, mipInt ), 1.0 );
					return;

				}

				vec3 outputDirection = normalize( vOutputDirection );

				vec3 up = abs( outputDirection.z ) < 0.999 ? vec3( 0.0, 0.0, 1.0 ) : vec3( 1.0, 0.0, 0.0 );
				vec3 tangent = normalize( cross( up, outputDirection ) );
				vec3 bitangent = cross( outputDirection, tangent );

				// Truncate the kernel at three standard deviations or at the antipode.
				float thetaMax = min( 3.0 * sigma, PI );
				float truncation = 1.0 - exp( - 0.5 * thetaMax * thetaMax / ( sigma * sigma ) );

				vec3 accumColor = vec3( 0.0 );
				float accumWeight = 0.0;

				for ( int i = 0; i < SAMPLES; i ++ ) {

					// Stratified inverse-CDF sampling of the Gaussian, placed on a golden-angle spiral.
					float stratum = ( float( i ) + 0.5 ) / float( SAMPLES );
					float theta = sigma * sqrt( - 2.0 * log( 1.0 - stratum * truncation ) );
					float phi = float( i ) * GOLDEN_ANGLE;

					vec3 offset = cos( phi ) * tangent + sin( phi ) * bitangent;
					vec3 sampleDirection = cos( theta ) * outputDirection + sin( theta ) * offset;

					// Correct the planar sample density to solid angle.
					float weight = sin( theta ) / theta;

					accumColor += weight * bilinearCubeUV( envMap, sampleDirection, mipInt );
					accumWeight += weight;

				}

				gl_FragColor = vec4( accumColor / accumWeight, 1.0 );

			}
		`,blending:0,depthTest:!1,depthWrite:!1})}function fp(){return new Qc({name:`EquirectangularToCubeUV`,uniforms:{envMap:{value:null}},vertexShader:mp(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			varying vec3 vOutputDirection;

			uniform sampler2D envMap;

			#include <common>

			void main() {

				vec3 outputDirection = normalize( vOutputDirection );
				vec2 uv = equirectUv( outputDirection );

				gl_FragColor = vec4( texture2D ( envMap, uv ).rgb, 1.0 );

			}
		`,blending:0,depthTest:!1,depthWrite:!1})}function pp(){return new Qc({name:`CubemapToCubeUV`,uniforms:{envMap:{value:null},flipEnvMap:{value:-1}},vertexShader:mp(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			uniform float flipEnvMap;

			varying vec3 vOutputDirection;

			uniform samplerCube envMap;

			void main() {

				gl_FragColor = textureCube( envMap, vec3( flipEnvMap * vOutputDirection.x, vOutputDirection.yz ) );

			}
		`,blending:0,depthTest:!1,depthWrite:!1})}function mp(){return`

		precision mediump float;
		precision mediump int;

		attribute vec3 outputDirection;

		varying vec3 vOutputDirection;

		void main() {

			vOutputDirection = outputDirection;
			gl_Position = vec4( position, 1.0 );

		}
	`}var hp=class extends Rn{constructor(e=1,t={}){super(e,e,t),this.isWebGLCubeRenderTarget=!0;let n={width:e,height:e,depth:1},r=[n,n,n,n,n,n];this.texture=new qo(r),this._setTextureOptions(t),this.texture.isRenderTargetTexture=!0}fromEquirectangularTexture(e,t){this.texture.type=t.type,this.texture.colorSpace=t.colorSpace,this.texture.generateMipmaps=t.generateMipmaps,this.texture.minFilter=t.minFilter,this.texture.magFilter=t.magFilter;let n={uniforms:{tEquirect:{value:null}},vertexShader:`

				varying vec3 vWorldDirection;

				vec3 transformDirection( in vec3 dir, in mat4 matrix ) {

					return normalize( ( matrix * vec4( dir, 0.0 ) ).xyz );

				}

				void main() {

					vWorldDirection = transformDirection( position, modelMatrix );

					#include <begin_vertex>
					#include <project_vertex>

				}
			`,fragmentShader:`

				uniform sampler2D tEquirect;

				varying vec3 vWorldDirection;

				#include <common>

				void main() {

					vec3 direction = normalize( vWorldDirection );

					vec2 sampleUV = equirectUv( direction );

					gl_FragColor = texture2D( tEquirect, sampleUV );

				}
			`},r=new $o(5,5,5),i=new Qc({name:`CubemapFromEquirect`,uniforms:Wc(n.uniforms),vertexShader:n.vertexShader,fragmentShader:n.fragmentShader,side:1,blending:0});i.uniforms.tEquirect.value=t;let a=new xa(r,i),o=t.minFilter;return t.minFilter===1008&&(t.minFilter=h),new Gu(1,10,this).update(e,a),t.minFilter=o,a.geometry.dispose(),a.material.dispose(),this}clear(e,t=!0,n=!0,r=!0){let i=e.getRenderTarget();for(let i=0;i<6;i++)e.setRenderTarget(this,i),e.clear(t,n,r);e.setRenderTarget(i)}};function gp(e){let t=new WeakMap,n=new WeakMap,r=null;function i(e,t=!1){return e==null?null:t?o(e):a(e)}function a(n){if(n&&n.isTexture){let r=n.mapping;if(r===303||r===304){if(t.has(n)){let e=t.get(n).texture;return s(e,n.mapping)}{let r=n.image;if(r&&r.height>0){let i=new hp(r.height);return i.fromEquirectangularTexture(e,n),t.set(n,i),n.addEventListener(`dispose`,l),s(i.texture,n.mapping)}return null}}}return n}function o(t){if(t&&t.isTexture){let i=t.mapping,a=i===303||i===304,o=i===301||i===302;if(a||o){let i=n.get(t),s=i===void 0?0:i.texture.pmremVersion;if(t.isRenderTargetTexture&&t.pmremVersion!==s)return r===null&&(r=new op(e)),i=a?r.fromEquirectangular(t,i):r.fromCubemap(t,i),i.texture.pmremVersion=t.pmremVersion,n.set(t,i),i.texture;if(i!==void 0)return i.texture;{let s=t.image;return a&&s&&s.height>0||o&&s&&c(s)?(r===null&&(r=new op(e)),i=a?r.fromEquirectangular(t):r.fromCubemap(t),i.texture.pmremVersion=t.pmremVersion,n.set(t,i),t.addEventListener(`dispose`,u),i.texture):null}}}return t}function s(e,t){return t===303?e.mapping=301:t===304&&(e.mapping=302),e}function c(e){let t=0;for(let n=0;n<6;n++)e[n]!==void 0&&t++;return t===6}function l(e){let n=e.target;n.removeEventListener(`dispose`,l);let r=t.get(n);r!==void 0&&(t.delete(n),r.dispose())}function u(e){let t=e.target;t.removeEventListener(`dispose`,u);let r=n.get(t);r!==void 0&&(n.delete(t),r.dispose())}function d(){t=new WeakMap,n=new WeakMap,r!==null&&(r.dispose(),r=null)}return{get:i,dispose:d}}function _p(e){let t={};function n(n){if(t[n]!==void 0)return t[n];let r=e.getExtension(n);return t[n]=r,r}return{has:function(e){return n(e)!==null},init:function(){n(`EXT_color_buffer_float`),n(`WEBGL_clip_cull_distance`),n(`OES_texture_float_linear`),n(`EXT_color_buffer_half_float`),n(`WEBGL_multisampled_render_to_texture`),n(`WEBGL_render_shared_exponent`)},get:function(e){let t=n(e);return t===null&&zt(`WebGLRenderer: `+e+` extension not supported.`),t}}}function vp(e,t,n,r){let i={},a=new WeakMap;function o(e){let s=e.target;s.index!==null&&t.remove(s.index);for(let e in s.attributes)t.remove(s.attributes[e]);s.removeEventListener(`dispose`,o),delete i[s.id];let c=a.get(s);c&&(t.remove(c),a.delete(s)),r.releaseStatesOfGeometry(s),s.isInstancedBufferGeometry===!0&&delete s._maxInstanceCount,n.memory.geometries--}function s(e,t){return i[t.id]===!0||(t.addEventListener(`dispose`,o),i[t.id]=!0,n.memory.geometries++),t}function c(n){let r=n.attributes;for(let n in r)t.update(r[n],e.ARRAY_BUFFER)}function l(e){let n=[],r=e.index,i=e.attributes.position,o=0;if(i===void 0)return;if(r!==null){let e=r.array;o=r.version;for(let t=0,r=e.length;t<r;t+=3){let r=e[t+0],i=e[t+1],a=e[t+2];n.push(r,i,i,a,a,r)}}else{let e=i.array;o=i.version;for(let t=0,r=e.length/3-1;t<r;t+=3){let e=t+0,r=t+1,i=t+2;n.push(e,r,r,i,i,e)}}let s=new(i.count>=65535?_i:hi)(n,1);s.version=o;let c=a.get(e);c&&t.remove(c),a.set(e,s)}function u(e){let t=a.get(e);if(t){let n=e.index;n!==null&&t.version<n.version&&l(e)}else l(e);return a.get(e)}return{get:s,update:c,getWireframeAttribute:u}}function yp(e,t,n){let r;function i(e){r=e}let a,o;function s(e){a=e.type,o=e.bytesPerElement}function c(t,i){e.drawElements(r,i,a,t*o),n.update(i,r,1)}function l(t,i,s){s!==0&&(e.drawElementsInstanced(r,i,a,t*o,s),n.update(i,r,s))}function u(e,i,o){if(o===0)return;t.get(`WEBGL_multi_draw`).multiDrawElementsWEBGL(r,i,0,a,e,0,o);let s=0;for(let e=0;e<o;e++)s+=i[e];n.update(s,r,1)}this.setMode=i,this.setIndex=s,this.render=c,this.renderInstances=l,this.renderMultiDraw=u}function bp(e){let t={geometries:0,textures:0},n={frame:0,calls:0,triangles:0,points:0,lines:0};function r(t,r,i){switch(n.calls++,r){case e.TRIANGLES:n.triangles+=t/3*i;break;case e.LINES:n.lines+=t/2*i;break;case e.LINE_STRIP:n.lines+=i*(t-1);break;case e.LINE_LOOP:n.lines+=i*t;break;case e.POINTS:n.points+=i*t;break;default:W(`WebGLInfo: Unknown draw mode:`,r)}}function i(){n.calls=0,n.triangles=0,n.points=0,n.lines=0}return{memory:t,render:n,programs:null,autoReset:!0,reset:i,update:r}}function xp(e,t,n){let r=new WeakMap,i=new In;function a(a,o,s){let c=a.morphTargetInfluences,l=o.morphAttributes.position||o.morphAttributes.normal||o.morphAttributes.color,u=l===void 0?0:l.length,d=r.get(o);if(d===void 0||d.count!==u){d!==void 0&&d.texture.dispose();let e=o.morphAttributes.position!==void 0,n=o.morphAttributes.normal!==void 0,a=o.morphAttributes.color!==void 0,s=o.morphAttributes.position||[],c=o.morphAttributes.normal||[],l=o.morphAttributes.color||[],f=0;e===!0&&(f=1),n===!0&&(f=2),a===!0&&(f=3);let p=o.attributes.position.count*f,m=1;p>t.maxTextureSize&&(m=Math.ceil(p/t.maxTextureSize),p=t.maxTextureSize);let h=new Float32Array(p*m*4*u),g=new zn(h,p,m,u);g.type=E,g.needsUpdate=!0;let _=f*4;for(let t=0;t<u;t++){let r=s[t],o=c[t],u=l[t],d=p*m*4*t;for(let t=0;t<r.count;t++){let s=t*_;e===!0&&(i.fromBufferAttribute(r,t),h[d+s+0]=i.x,h[d+s+1]=i.y,h[d+s+2]=i.z,h[d+s+3]=0),n===!0&&(i.fromBufferAttribute(o,t),h[d+s+4]=i.x,h[d+s+5]=i.y,h[d+s+6]=i.z,h[d+s+7]=0),a===!0&&(i.fromBufferAttribute(u,t),h[d+s+8]=i.x,h[d+s+9]=i.y,h[d+s+10]=i.z,h[d+s+11]=u.itemSize===4?i.w:1)}}d={count:u,texture:g,size:new K(p,m)},r.set(o,d);function v(){g.dispose(),r.delete(o),o.removeEventListener(`dispose`,v)}o.addEventListener(`dispose`,v)}if(a.isInstancedMesh===!0&&a.morphTexture!==null)s.getUniforms().setValue(e,`morphTexture`,a.morphTexture,n);else{let t=0;for(let e=0;e<c.length;e++)t+=c[e];let n=o.morphTargetsRelative?1:1-t;s.getUniforms().setValue(e,`morphTargetBaseInfluence`,n),s.getUniforms().setValue(e,`morphTargetInfluences`,c)}s.getUniforms().setValue(e,`morphTargetsTexture`,d.texture,n),s.getUniforms().setValue(e,`morphTargetsTextureSize`,d.size)}return{update:a}}function Sp(e,t,n,r,i){let a=new WeakMap;function o(r){let o=i.render.frame,s=r.geometry,l=t.get(r,s);if(a.get(l)!==o&&(t.update(l),a.set(l,o)),r.isInstancedMesh&&(r.hasEventListener(`dispose`,c)===!1&&r.addEventListener(`dispose`,c),a.get(r)!==o&&(n.update(r.instanceMatrix,e.ARRAY_BUFFER),r.instanceColor!==null&&n.update(r.instanceColor,e.ARRAY_BUFFER),a.set(r,o))),r.isSkinnedMesh){let e=r.skeleton;a.get(e)!==o&&(e.update(),a.set(e,o))}return l}function s(){a=new WeakMap}function c(e){let t=e.target;t.removeEventListener(`dispose`,c),r.releaseStatesOfObject(t),n.remove(t.instanceMatrix),t.instanceColor!==null&&n.remove(t.instanceColor)}return{update:o,dispose:s}}var Cp={1:`LINEAR_TONE_MAPPING`,2:`REINHARD_TONE_MAPPING`,3:`CINEON_TONE_MAPPING`,4:`ACES_FILMIC_TONE_MAPPING`,6:`AGX_TONE_MAPPING`,7:`NEUTRAL_TONE_MAPPING`,5:`CUSTOM_TONE_MAPPING`};function wp(e,t,n,r,i,a){let o=new Rn(t,n,{type:e,depthBuffer:i,stencilBuffer:a,samples:r?4:0,storeMultisampledDepthBuffer:!1,storeMultisampledStencilBuffer:!1,resolveDepthBuffer:!1,resolveStencilBuffer:!1}),s=null,c=null,l=new Z;l.setAttribute(`position`,new X([-1,3,0,-1,-1,0,3,-1,0],3)),l.setAttribute(`uv`,new X([0,2,0,0,2,0],2));let u=new $c({uniforms:{tDiffuse:{value:null}},vertexShader:`
			precision highp float;

			uniform mat4 modelViewMatrix;
			uniform mat4 projectionMatrix;

			attribute vec3 position;
			attribute vec2 uv;

			varying vec2 vUv;

			void main() {
				vUv = uv;
				gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
			}`,fragmentShader:`
			precision highp float;

			uniform sampler2D tDiffuse;

			varying vec2 vUv;

			#include <tonemapping_pars_fragment>
			#include <colorspace_pars_fragment>

			void main() {
				gl_FragColor = texture2D( tDiffuse, vUv );

				#ifdef LINEAR_TONE_MAPPING
					gl_FragColor.rgb = LinearToneMapping( gl_FragColor.rgb );
				#elif defined( REINHARD_TONE_MAPPING )
					gl_FragColor.rgb = ReinhardToneMapping( gl_FragColor.rgb );
				#elif defined( CINEON_TONE_MAPPING )
					gl_FragColor.rgb = CineonToneMapping( gl_FragColor.rgb );
				#elif defined( ACES_FILMIC_TONE_MAPPING )
					gl_FragColor.rgb = ACESFilmicToneMapping( gl_FragColor.rgb );
				#elif defined( AGX_TONE_MAPPING )
					gl_FragColor.rgb = AgXToneMapping( gl_FragColor.rgb );
				#elif defined( NEUTRAL_TONE_MAPPING )
					gl_FragColor.rgb = NeutralToneMapping( gl_FragColor.rgb );
				#elif defined( CUSTOM_TONE_MAPPING )
					gl_FragColor.rgb = CustomToneMapping( gl_FragColor.rgb );
				#endif

				#ifdef SRGB_TRANSFER
					gl_FragColor = sRGBTransferOETF( gl_FragColor );
				#endif
			}`,depthTest:!1,depthWrite:!1}),d=new xa(l,u),f=new _u(-1,1,1,-1,0,1),p=null,m=null,h=!1,g,_=null,v=[],y=!1;this.setSize=function(e,t){o.setSize(e,t),s!==null&&s.setSize(e,t),c!==null&&c.setSize(e,t);for(let n=0;n<v.length;n++){let r=v[n];r.setSize&&r.setSize(e,t)}},this.setEffects=function(e){v=e,y=v.length>0&&v[0].isRenderPass===!0;let t=o.width,n=o.height;v.length>0&&s===null&&(s=new Rn(t,n,{type:D,depthBuffer:!1,stencilBuffer:!1}),c=new Rn(t,n,{type:D,depthBuffer:!1,stencilBuffer:!1}));for(let e=0;e<v.length;e++){let r=v[e];r.setSize&&r.setSize(t,n)}},this.begin=function(e,t){if(h||e.toneMapping===0&&v.length===0)return!1;if(_=t,t!==null){let e=t.width,n=t.height;(o.width!==e||o.height!==n)&&this.setSize(e,n)}return y===!1&&e.setRenderTarget(o),g=e.toneMapping,e.toneMapping=0,!0},this.hasRenderPass=function(){return y},this.end=function(e,t){e.toneMapping=g,h=!0;let n=o,r=s;for(let i=0;i<v.length;i++){let a=v[i];a.enabled!==!1&&(a.render(e,r,n,t),a.needsSwap!==!1&&(n=r,r=r===s?c:s))}if(p!==e.outputColorSpace||m!==e.toneMapping){p=e.outputColorSpace,m=e.toneMapping,u.defines={},wn.getTransfer(p)===`srgb`&&(u.defines.SRGB_TRANSFER=``);let t=Cp[m];t&&(u.defines[t]=``),u.needsUpdate=!0}u.uniforms.tDiffuse.value=n.texture,e.setRenderTarget(_),e.render(d,f),_=null,h=!1},this.isCompositing=function(){return h},this.dispose=function(){o.dispose(),s!==null&&s.dispose(),c!==null&&c.dispose(),l.dispose(),u.dispose()}}var Tp=new Fn,Ep=new Xo(1,1),Dp=new zn,Op=new Vn,kp=new qo,Ap=[],jp=[],Mp=new Float32Array(16),Np=new Float32Array(9),Pp=new Float32Array(4);function Fp(e,t,n){let r=e[0];if(r<=0||r>0)return e;let i=t*n,a=Ap[i];if(a===void 0&&(a=new Float32Array(i),Ap[i]=a),t!==0){r.toArray(a,0);for(let r=1,i=0;r!==t;++r)i+=n,e[r].toArray(a,i)}return a}function Ip(e,t){if(e.length!==t.length)return!1;for(let n=0,r=e.length;n<r;n++)if(e[n]!==t[n])return!1;return!0}function Lp(e,t){for(let n=0,r=t.length;n<r;n++)e[n]=t[n]}function Rp(e,t){let n=jp[t];n===void 0&&(n=new Int32Array(t),jp[t]=n);for(let r=0;r!==t;++r)n[r]=e.allocateTextureUnit();return n}function zp(e,t){let n=this.cache;n[0]!==t&&(e.uniform1f(this.addr,t),n[0]=t)}function Bp(e,t){let n=this.cache;if(t.x!==void 0)(n[0]!==t.x||n[1]!==t.y)&&(e.uniform2f(this.addr,t.x,t.y),n[0]=t.x,n[1]=t.y);else{if(Ip(n,t))return;e.uniform2fv(this.addr,t),Lp(n,t)}}function Vp(e,t){let n=this.cache;if(t.x!==void 0)(n[0]!==t.x||n[1]!==t.y||n[2]!==t.z)&&(e.uniform3f(this.addr,t.x,t.y,t.z),n[0]=t.x,n[1]=t.y,n[2]=t.z);else if(t.r!==void 0)(n[0]!==t.r||n[1]!==t.g||n[2]!==t.b)&&(e.uniform3f(this.addr,t.r,t.g,t.b),n[0]=t.r,n[1]=t.g,n[2]=t.b);else{if(Ip(n,t))return;e.uniform3fv(this.addr,t),Lp(n,t)}}function Hp(e,t){let n=this.cache;if(t.x!==void 0)(n[0]!==t.x||n[1]!==t.y||n[2]!==t.z||n[3]!==t.w)&&(e.uniform4f(this.addr,t.x,t.y,t.z,t.w),n[0]=t.x,n[1]=t.y,n[2]=t.z,n[3]=t.w);else{if(Ip(n,t))return;e.uniform4fv(this.addr,t),Lp(n,t)}}function Up(e,t){let n=this.cache,r=t.elements;if(r===void 0){if(Ip(n,t))return;e.uniformMatrix2fv(this.addr,!1,t),Lp(n,t)}else{if(Ip(n,r))return;Pp.set(r),e.uniformMatrix2fv(this.addr,!1,Pp),Lp(n,r)}}function Wp(e,t){let n=this.cache,r=t.elements;if(r===void 0){if(Ip(n,t))return;e.uniformMatrix3fv(this.addr,!1,t),Lp(n,t)}else{if(Ip(n,r))return;Np.set(r),e.uniformMatrix3fv(this.addr,!1,Np),Lp(n,r)}}function Gp(e,t){let n=this.cache,r=t.elements;if(r===void 0){if(Ip(n,t))return;e.uniformMatrix4fv(this.addr,!1,t),Lp(n,t)}else{if(Ip(n,r))return;Mp.set(r),e.uniformMatrix4fv(this.addr,!1,Mp),Lp(n,r)}}function Kp(e,t){let n=this.cache;n[0]!==t&&(e.uniform1i(this.addr,t),n[0]=t)}function qp(e,t){let n=this.cache;if(t.x!==void 0)(n[0]!==t.x||n[1]!==t.y)&&(e.uniform2i(this.addr,t.x,t.y),n[0]=t.x,n[1]=t.y);else{if(Ip(n,t))return;e.uniform2iv(this.addr,t),Lp(n,t)}}function Jp(e,t){let n=this.cache;if(t.x!==void 0)(n[0]!==t.x||n[1]!==t.y||n[2]!==t.z)&&(e.uniform3i(this.addr,t.x,t.y,t.z),n[0]=t.x,n[1]=t.y,n[2]=t.z);else{if(Ip(n,t))return;e.uniform3iv(this.addr,t),Lp(n,t)}}function Yp(e,t){let n=this.cache;if(t.x!==void 0)(n[0]!==t.x||n[1]!==t.y||n[2]!==t.z||n[3]!==t.w)&&(e.uniform4i(this.addr,t.x,t.y,t.z,t.w),n[0]=t.x,n[1]=t.y,n[2]=t.z,n[3]=t.w);else{if(Ip(n,t))return;e.uniform4iv(this.addr,t),Lp(n,t)}}function Xp(e,t){let n=this.cache;n[0]!==t&&(e.uniform1ui(this.addr,t),n[0]=t)}function Zp(e,t){let n=this.cache;if(t.x!==void 0)(n[0]!==t.x||n[1]!==t.y)&&(e.uniform2ui(this.addr,t.x,t.y),n[0]=t.x,n[1]=t.y);else{if(Ip(n,t))return;e.uniform2uiv(this.addr,t),Lp(n,t)}}function Qp(e,t){let n=this.cache;if(t.x!==void 0)(n[0]!==t.x||n[1]!==t.y||n[2]!==t.z)&&(e.uniform3ui(this.addr,t.x,t.y,t.z),n[0]=t.x,n[1]=t.y,n[2]=t.z);else{if(Ip(n,t))return;e.uniform3uiv(this.addr,t),Lp(n,t)}}function $p(e,t){let n=this.cache;if(t.x!==void 0)(n[0]!==t.x||n[1]!==t.y||n[2]!==t.z||n[3]!==t.w)&&(e.uniform4ui(this.addr,t.x,t.y,t.z,t.w),n[0]=t.x,n[1]=t.y,n[2]=t.z,n[3]=t.w);else{if(Ip(n,t))return;e.uniform4uiv(this.addr,t),Lp(n,t)}}function em(e,t,n){let r=this.cache,i=n.allocateTextureUnit();r[0]!==i&&(e.uniform1i(this.addr,i),r[0]=i);let a;this.type===e.SAMPLER_2D_SHADOW?(Ep.compareFunction=n.isReversedDepthBuffer()?518:515,a=Ep):a=Tp,n.setTexture2D(t||a,i)}function tm(e,t,n){let r=this.cache,i=n.allocateTextureUnit();r[0]!==i&&(e.uniform1i(this.addr,i),r[0]=i),n.setTexture3D(t||Op,i)}function nm(e,t,n){let r=this.cache,i=n.allocateTextureUnit();r[0]!==i&&(e.uniform1i(this.addr,i),r[0]=i),n.setTextureCube(t||kp,i)}function rm(e,t,n){let r=this.cache,i=n.allocateTextureUnit();r[0]!==i&&(e.uniform1i(this.addr,i),r[0]=i),n.setTexture2DArray(t||Dp,i)}function im(e){switch(e){case 5126:return zp;case 35664:return Bp;case 35665:return Vp;case 35666:return Hp;case 35674:return Up;case 35675:return Wp;case 35676:return Gp;case 5124:case 35670:return Kp;case 35667:case 35671:return qp;case 35668:case 35672:return Jp;case 35669:case 35673:return Yp;case 5125:return Xp;case 36294:return Zp;case 36295:return Qp;case 36296:return $p;case 35678:case 36198:case 36298:case 36306:case 35682:return em;case 35679:case 36299:case 36307:return tm;case 35680:case 36300:case 36308:case 36293:return nm;case 36289:case 36303:case 36311:case 36292:return rm}}function am(e,t){e.uniform1fv(this.addr,t)}function om(e,t){let n=Fp(t,this.size,2);e.uniform2fv(this.addr,n)}function sm(e,t){let n=Fp(t,this.size,3);e.uniform3fv(this.addr,n)}function cm(e,t){let n=Fp(t,this.size,4);e.uniform4fv(this.addr,n)}function lm(e,t){let n=Fp(t,this.size,4);e.uniformMatrix2fv(this.addr,!1,n)}function um(e,t){let n=Fp(t,this.size,9);e.uniformMatrix3fv(this.addr,!1,n)}function dm(e,t){let n=Fp(t,this.size,16);e.uniformMatrix4fv(this.addr,!1,n)}function fm(e,t){e.uniform1iv(this.addr,t)}function pm(e,t){e.uniform2iv(this.addr,t)}function mm(e,t){e.uniform3iv(this.addr,t)}function hm(e,t){e.uniform4iv(this.addr,t)}function gm(e,t){e.uniform1uiv(this.addr,t)}function _m(e,t){e.uniform2uiv(this.addr,t)}function vm(e,t){e.uniform3uiv(this.addr,t)}function ym(e,t){e.uniform4uiv(this.addr,t)}function bm(e,t,n){let r=this.cache,i=t.length,a=Rp(n,i);Ip(r,a)||(e.uniform1iv(this.addr,a),Lp(r,a));let o;o=this.type===e.SAMPLER_2D_SHADOW?Ep:Tp;for(let e=0;e!==i;++e)n.setTexture2D(t[e]||o,a[e])}function xm(e,t,n){let r=this.cache,i=t.length,a=Rp(n,i);Ip(r,a)||(e.uniform1iv(this.addr,a),Lp(r,a));for(let e=0;e!==i;++e)n.setTexture3D(t[e]||Op,a[e])}function Sm(e,t,n){let r=this.cache,i=t.length,a=Rp(n,i);Ip(r,a)||(e.uniform1iv(this.addr,a),Lp(r,a));for(let e=0;e!==i;++e)n.setTextureCube(t[e]||kp,a[e])}function Cm(e,t,n){let r=this.cache,i=t.length,a=Rp(n,i);Ip(r,a)||(e.uniform1iv(this.addr,a),Lp(r,a));for(let e=0;e!==i;++e)n.setTexture2DArray(t[e]||Dp,a[e])}function wm(e){switch(e){case 5126:return am;case 35664:return om;case 35665:return sm;case 35666:return cm;case 35674:return lm;case 35675:return um;case 35676:return dm;case 5124:case 35670:return fm;case 35667:case 35671:return pm;case 35668:case 35672:return mm;case 35669:case 35673:return hm;case 5125:return gm;case 36294:return _m;case 36295:return vm;case 36296:return ym;case 35678:case 36198:case 36298:case 36306:case 35682:return bm;case 35679:case 36299:case 36307:return xm;case 35680:case 36300:case 36308:case 36293:return Sm;case 36289:case 36303:case 36311:case 36292:return Cm}}var Tm=class{constructor(e,t,n){this.id=e,this.addr=n,this.cache=[],this.type=t.type,this.setValue=im(t.type)}},Em=class{constructor(e,t,n){this.id=e,this.addr=n,this.cache=[],this.type=t.type,this.size=t.size,this.setValue=wm(t.type)}},Dm=class{constructor(e){this.id=e,this.seq=[],this.map={}}setValue(e,t,n){let r=this.seq;for(let i=0,a=r.length;i!==a;++i){let a=r[i];a.setValue(e,t[a.id],n)}}},Om=/(\w+)(\])?(\[|\.)?/g;function km(e,t){e.seq.push(t),e.map[t.id]=t}function Am(e,t,n){let r=e.name,i=r.length;for(Om.lastIndex=0;;){let a=Om.exec(r),o=Om.lastIndex,s=a[1],c=a[2]===`]`,l=a[3];if(c&&(s|=0),l===void 0||l===`[`&&o+2===i){km(n,l===void 0?new Tm(s,e,t):new Em(s,e,t));break}{let e=n.map[s];e===void 0&&(e=new Dm(s),km(n,e)),n=e}}}var jm=class{constructor(e,t){this.seq=[],this.map={};let n=e.getProgramParameter(t,e.ACTIVE_UNIFORMS);for(let r=0;r<n;++r){let n=e.getActiveUniform(t,r);Am(n,e.getUniformLocation(t,n.name),this)}let r=[],i=[];for(let t of this.seq)t.type===e.SAMPLER_2D_SHADOW||t.type===e.SAMPLER_CUBE_SHADOW||t.type===e.SAMPLER_2D_ARRAY_SHADOW?r.push(t):i.push(t);r.length>0&&(this.seq=r.concat(i))}setValue(e,t,n,r){let i=this.map[t];i!==void 0&&i.setValue(e,n,r)}setOptional(e,t,n){let r=t[n];r!==void 0&&this.setValue(e,n,r)}static upload(e,t,n,r){for(let i=0,a=t.length;i!==a;++i){let a=t[i],o=n[a.id];o.needsUpdate!==!1&&a.setValue(e,o.value,r)}}static seqWithValue(e,t){let n=[];for(let r=0,i=e.length;r!==i;++r){let i=e[r];i.id in t&&n.push(i)}return n}};function Mm(e,t,n){let r=e.createShader(t);return e.shaderSource(r,n),e.compileShader(r),r}var Nm=37297,Pm=0;function Fm(e,t){let n=e.split(`
`),r=[],i=Math.max(t-6,0),a=Math.min(t+6,n.length);for(let e=i;e<a;e++){let i=e+1;r.push(`${i===t?`>`:` `} ${i}: ${n[e]}`)}return r.join(`
`)}var Im=new yn;function Lm(e){wn._getMatrix(Im,wn.workingColorSpace,e);let t=`mat3( ${Im.elements.map(e=>e.toFixed(4))} )`;switch(wn.getTransfer(e)){case tt:return[t,`LinearTransferOETF`];case nt:return[t,`sRGBTransferOETF`];default:return U(`WebGLProgram: Unsupported color space: `,e),[t,`LinearTransferOETF`]}}function Rm(e,t,n){let r=e.getShaderParameter(t,e.COMPILE_STATUS),i=(e.getShaderInfoLog(t)||``).trim();if(r&&i===``)return``;let a=/ERROR: 0:(\d+)/.exec(i);if(a){let r=parseInt(a[1]);return n.toUpperCase()+`

`+i+`

`+Fm(e.getShaderSource(t),r)}return i}function zm(e,t){let n=Lm(t);return[`vec4 ${e}( vec4 value ) {`,`	return ${n[1]}( vec4( value.rgb * ${n[0]}, value.a ) );`,`}`].join(`
`)}var Bm={1:`Linear`,2:`Reinhard`,3:`Cineon`,4:`ACESFilmic`,6:`AgX`,7:`Neutral`,5:`Custom`};function Vm(e,t){let n=Bm[t];return n===void 0?(U(`WebGLProgram: Unsupported toneMapping:`,t),`vec3 `+e+`( vec3 color ) { return LinearToneMapping( color ); }`):`vec3 `+e+`( vec3 color ) { return `+n+`ToneMapping( color ); }`}var Hm=new q;function Um(){return wn.getLuminanceCoefficients(Hm),[`float luminance( const in vec3 rgb ) {`,`	const vec3 weights = vec3( ${Hm.x.toFixed(4)}, ${Hm.y.toFixed(4)}, ${Hm.z.toFixed(4)} );`,`	return dot( weights, rgb );`,`}`].join(`
`)}function Wm(e){return[e.extensionClipCullDistance?`#extension GL_ANGLE_clip_cull_distance : require`:``,e.extensionMultiDraw?`#extension GL_ANGLE_multi_draw : require`:``].filter(qm).join(`
`)}function Gm(e){let t=[];for(let n in e){let r=e[n];r!==!1&&t.push(`#define `+n+` `+r)}return t.join(`
`)}function Km(e,t){let n={},r=e.getProgramParameter(t,e.ACTIVE_ATTRIBUTES);for(let i=0;i<r;i++){let r=e.getActiveAttrib(t,i),a=r.name,o=1;r.type===e.FLOAT_MAT2&&(o=2),r.type===e.FLOAT_MAT3&&(o=3),r.type===e.FLOAT_MAT4&&(o=4),n[a]={type:r.type,location:e.getAttribLocation(t,a),locationSize:o}}return n}function qm(e){return e!==``}function Jm(e,t){let n=t.numSpotLightShadows+t.numSpotLightMaps-t.numSpotLightShadowsWithMaps;return e.replace(/NUM_SUN_LIGHTS/g,t.numSunLights).replace(/NUM_DIR_LIGHTS/g,t.numDirLights).replace(/NUM_SPOT_LIGHTS/g,t.numSpotLights).replace(/NUM_SPOT_LIGHT_MAPS/g,t.numSpotLightMaps).replace(/NUM_SPOT_LIGHT_COORDS/g,n).replace(/NUM_RECT_AREA_LIGHTS/g,t.numRectAreaLights).replace(/NUM_POINT_LIGHTS/g,t.numPointLights).replace(/NUM_HEMI_LIGHTS/g,t.numHemiLights).replace(/NUM_SUN_LIGHT_SHADOWS/g,t.numSunLightShadows).replace(/NUM_DIR_LIGHT_SHADOWS/g,t.numDirLightShadows).replace(/NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS/g,t.numSpotLightShadowsWithMaps).replace(/NUM_SPOT_LIGHT_SHADOWS/g,t.numSpotLightShadows).replace(/NUM_POINT_LIGHT_SHADOWS/g,t.numPointLightShadows)}function Ym(e,t){return e.replace(/NUM_CLIPPING_PLANES/g,t.numClippingPlanes).replace(/UNION_CLIPPING_PLANES/g,t.numClippingPlanes-t.numClipIntersection)}var Xm=/^[ \t]*#include +<([\w\d./]+)>/gm;function Zm(e){return e.replace(Xm,$m)}var Qm=new Map;function $m(e,t){let n=Rf[t];if(n===void 0){let e=Qm.get(t);if(e!==void 0)n=Rf[e],U(`WebGLRenderer: Shader chunk "%s" has been deprecated. Use "%s" instead.`,t,e);else throw Error(`THREE.WebGLProgram: Can not resolve #include <`+t+`>`)}return Zm(n)}var eh=/#pragma unroll_loop_start\s+for\s*\(\s*int\s+i\s*=\s*(\d+)\s*;\s*i\s*<\s*(\d+)\s*;\s*i\s*\+\+\s*\)\s*{([\s\S]+?)}\s+#pragma unroll_loop_end/g;function th(e){return e.replace(eh,nh)}function nh(e,t,n,r){let i=``;for(let e=parseInt(t);e<parseInt(n);e++)i+=r.replace(/\[\s*i\s*\]/g,`[ `+e+` ]`).replace(/UNROLLED_LOOP_INDEX/g,e);return i}function rh(e){let t=`precision ${e.precision} float;
	precision ${e.precision} int;
	precision ${e.precision} sampler2D;
	precision ${e.precision} samplerCube;
	precision ${e.precision} sampler3D;
	precision ${e.precision} sampler2DArray;
	precision ${e.precision} sampler2DShadow;
	precision ${e.precision} samplerCubeShadow;
	precision ${e.precision} sampler2DArrayShadow;
	precision ${e.precision} isampler2D;
	precision ${e.precision} isampler3D;
	precision ${e.precision} isamplerCube;
	precision ${e.precision} isampler2DArray;
	precision ${e.precision} usampler2D;
	precision ${e.precision} usampler3D;
	precision ${e.precision} usamplerCube;
	precision ${e.precision} usampler2DArray;
	`;return e.precision===`highp`?t+=`
#define HIGH_PRECISION`:e.precision===`mediump`?t+=`
#define MEDIUM_PRECISION`:e.precision===`lowp`&&(t+=`
#define LOW_PRECISION`),t}var ih={1:`SHADOWMAP_TYPE_PCF`,3:`SHADOWMAP_TYPE_VSM`};function ah(e){return ih[e.shadowMapType]||`SHADOWMAP_TYPE_BASIC`}var oh={301:`ENVMAP_TYPE_CUBE`,302:`ENVMAP_TYPE_CUBE`,306:`ENVMAP_TYPE_CUBE_UV`};function sh(e){return e.envMap===!1?`ENVMAP_TYPE_CUBE`:oh[e.envMapMode]||`ENVMAP_TYPE_CUBE`}var ch={302:`ENVMAP_MODE_REFRACTION`};function lh(e){return e.envMap===!1?`ENVMAP_MODE_REFLECTION`:ch[e.envMapMode]||`ENVMAP_MODE_REFLECTION`}var uh={0:`ENVMAP_BLENDING_MULTIPLY`,1:`ENVMAP_BLENDING_MIX`,2:`ENVMAP_BLENDING_ADD`};function dh(e){return e.envMap===!1?`ENVMAP_BLENDING_NONE`:uh[e.combine]||`ENVMAP_BLENDING_NONE`}function fh(e){let t=e.envMapCubeUVHeight;if(t===null)return null;let n=Math.log2(t)-2,r=1/t;return{texelWidth:1/(3*Math.max(2**n,112)),texelHeight:r,maxMip:n}}function ph(e,t,n,r){let i=e.getContext(),a=n.defines,o=n.vertexShader,s=n.fragmentShader,c=ah(n),l=sh(n),u=lh(n),d=dh(n),f=fh(n),p=Wm(n),m=Gm(a),h=i.createProgram(),g,_,v=n.glslVersion?`#version `+n.glslVersion+`
`:``;n.isRawShaderMaterial?(g=[`#define SHADER_TYPE `+n.shaderType,`#define SHADER_NAME `+n.shaderName,m].filter(qm).join(`
`),g.length>0&&(g+=`
`),_=[`#define SHADER_TYPE `+n.shaderType,`#define SHADER_NAME `+n.shaderName,m].filter(qm).join(`
`),_.length>0&&(_+=`
`)):(g=[rh(n),`#define SHADER_TYPE `+n.shaderType,`#define SHADER_NAME `+n.shaderName,m,n.extensionClipCullDistance?`#define USE_CLIP_DISTANCE`:``,n.batching?`#define USE_BATCHING`:``,n.batchingColor?`#define USE_BATCHING_COLOR`:``,n.instancing?`#define USE_INSTANCING`:``,n.instancingColor?`#define USE_INSTANCING_COLOR`:``,n.instancingMorph?`#define USE_INSTANCING_MORPH`:``,n.useFog&&n.fog?`#define USE_FOG`:``,n.useFog&&n.fogExp2?`#define FOG_EXP2`:``,n.map?`#define USE_MAP`:``,n.envMap?`#define USE_ENVMAP`:``,n.envMap?`#define `+u:``,n.lightMap?`#define USE_LIGHTMAP`:``,n.aoMap?`#define USE_AOMAP`:``,n.bumpMap?`#define USE_BUMPMAP`:``,n.normalMap?`#define USE_NORMALMAP`:``,n.normalMapObjectSpace?`#define USE_NORMALMAP_OBJECTSPACE`:``,n.normalMapTangentSpace?`#define USE_NORMALMAP_TANGENTSPACE`:``,n.displacementMap?`#define USE_DISPLACEMENTMAP`:``,n.emissiveMap?`#define USE_EMISSIVEMAP`:``,n.anisotropy?`#define USE_ANISOTROPY`:``,n.anisotropyMap?`#define USE_ANISOTROPYMAP`:``,n.clearcoatMap?`#define USE_CLEARCOATMAP`:``,n.clearcoatRoughnessMap?`#define USE_CLEARCOAT_ROUGHNESSMAP`:``,n.clearcoatNormalMap?`#define USE_CLEARCOAT_NORMALMAP`:``,n.iridescenceMap?`#define USE_IRIDESCENCEMAP`:``,n.iridescenceThicknessMap?`#define USE_IRIDESCENCE_THICKNESSMAP`:``,n.specularMap?`#define USE_SPECULARMAP`:``,n.specularColorMap?`#define USE_SPECULAR_COLORMAP`:``,n.specularIntensityMap?`#define USE_SPECULAR_INTENSITYMAP`:``,n.roughnessMap?`#define USE_ROUGHNESSMAP`:``,n.metalnessMap?`#define USE_METALNESSMAP`:``,n.alphaMap?`#define USE_ALPHAMAP`:``,n.alphaHash?`#define USE_ALPHAHASH`:``,n.transmission?`#define USE_TRANSMISSION`:``,n.transmissionMap?`#define USE_TRANSMISSIONMAP`:``,n.thicknessMap?`#define USE_THICKNESSMAP`:``,n.sheenColorMap?`#define USE_SHEEN_COLORMAP`:``,n.sheenRoughnessMap?`#define USE_SHEEN_ROUGHNESSMAP`:``,n.mapUv?`#define MAP_UV `+n.mapUv:``,n.alphaMapUv?`#define ALPHAMAP_UV `+n.alphaMapUv:``,n.lightMapUv?`#define LIGHTMAP_UV `+n.lightMapUv:``,n.aoMapUv?`#define AOMAP_UV `+n.aoMapUv:``,n.emissiveMapUv?`#define EMISSIVEMAP_UV `+n.emissiveMapUv:``,n.bumpMapUv?`#define BUMPMAP_UV `+n.bumpMapUv:``,n.normalMapUv?`#define NORMALMAP_UV `+n.normalMapUv:``,n.displacementMapUv?`#define DISPLACEMENTMAP_UV `+n.displacementMapUv:``,n.metalnessMapUv?`#define METALNESSMAP_UV `+n.metalnessMapUv:``,n.roughnessMapUv?`#define ROUGHNESSMAP_UV `+n.roughnessMapUv:``,n.anisotropyMapUv?`#define ANISOTROPYMAP_UV `+n.anisotropyMapUv:``,n.clearcoatMapUv?`#define CLEARCOATMAP_UV `+n.clearcoatMapUv:``,n.clearcoatNormalMapUv?`#define CLEARCOAT_NORMALMAP_UV `+n.clearcoatNormalMapUv:``,n.clearcoatRoughnessMapUv?`#define CLEARCOAT_ROUGHNESSMAP_UV `+n.clearcoatRoughnessMapUv:``,n.iridescenceMapUv?`#define IRIDESCENCEMAP_UV `+n.iridescenceMapUv:``,n.iridescenceThicknessMapUv?`#define IRIDESCENCE_THICKNESSMAP_UV `+n.iridescenceThicknessMapUv:``,n.sheenColorMapUv?`#define SHEEN_COLORMAP_UV `+n.sheenColorMapUv:``,n.sheenRoughnessMapUv?`#define SHEEN_ROUGHNESSMAP_UV `+n.sheenRoughnessMapUv:``,n.specularMapUv?`#define SPECULARMAP_UV `+n.specularMapUv:``,n.specularColorMapUv?`#define SPECULAR_COLORMAP_UV `+n.specularColorMapUv:``,n.specularIntensityMapUv?`#define SPECULAR_INTENSITYMAP_UV `+n.specularIntensityMapUv:``,n.transmissionMapUv?`#define TRANSMISSIONMAP_UV `+n.transmissionMapUv:``,n.thicknessMapUv?`#define THICKNESSMAP_UV `+n.thicknessMapUv:``,n.vertexTangents&&n.flatShading===!1?`#define USE_TANGENT`:``,n.vertexNormals?`#define HAS_NORMAL`:``,n.vertexColors?`#define USE_COLOR`:``,n.vertexAlphas?`#define USE_COLOR_ALPHA`:``,n.vertexUv1s?`#define USE_UV1`:``,n.vertexUv2s?`#define USE_UV2`:``,n.vertexUv3s?`#define USE_UV3`:``,n.pointsUvs?`#define USE_POINTS_UV`:``,n.flatShading?`#define FLAT_SHADED`:``,n.skinning?`#define USE_SKINNING`:``,n.morphTargets?`#define USE_MORPHTARGETS`:``,n.morphNormals&&n.flatShading===!1?`#define USE_MORPHNORMALS`:``,n.morphColors?`#define USE_MORPHCOLORS`:``,n.morphTargetsCount>0?`#define MORPHTARGETS_TEXTURE_STRIDE `+n.morphTextureStride:``,n.morphTargetsCount>0?`#define MORPHTARGETS_COUNT `+n.morphTargetsCount:``,n.doubleSided?`#define DOUBLE_SIDED`:``,n.flipSided?`#define FLIP_SIDED`:``,n.shadowMapEnabled?`#define USE_SHADOWMAP`:``,n.shadowMapEnabled?`#define `+c:``,n.sizeAttenuation?`#define USE_SIZEATTENUATION`:``,n.numLightProbes>0?`#define USE_LIGHT_PROBES`:``,n.logarithmicDepthBuffer?`#define USE_LOGARITHMIC_DEPTH_BUFFER`:``,n.reversedDepthBuffer?`#define USE_REVERSED_DEPTH_BUFFER`:``,`uniform mat4 modelMatrix;`,`uniform mat4 modelViewMatrix;`,`uniform mat4 projectionMatrix;`,`uniform mat4 viewMatrix;`,`uniform mat3 normalMatrix;`,`uniform vec3 cameraPosition;`,`uniform bool isOrthographic;`,`#ifdef USE_INSTANCING`,`	attribute mat4 instanceMatrix;`,`#endif`,`#ifdef USE_INSTANCING_COLOR`,`	attribute vec3 instanceColor;`,`#endif`,`#ifdef USE_INSTANCING_MORPH`,`	uniform sampler2D morphTexture;`,`#endif`,`attribute vec3 position;`,`attribute vec3 normal;`,`attribute vec2 uv;`,`#ifdef USE_UV1`,`	attribute vec2 uv1;`,`#endif`,`#ifdef USE_UV2`,`	attribute vec2 uv2;`,`#endif`,`#ifdef USE_UV3`,`	attribute vec2 uv3;`,`#endif`,`#ifdef USE_TANGENT`,`	attribute vec4 tangent;`,`#endif`,`#if defined( USE_COLOR_ALPHA )`,`	attribute vec4 color;`,`#elif defined( USE_COLOR )`,`	attribute vec3 color;`,`#endif`,`#ifdef USE_SKINNING`,`	attribute vec4 skinIndex;`,`	attribute vec4 skinWeight;`,`#endif`,`
`].filter(qm).join(`
`),_=[rh(n),`#define SHADER_TYPE `+n.shaderType,`#define SHADER_NAME `+n.shaderName,m,n.useFog&&n.fog?`#define USE_FOG`:``,n.useFog&&n.fogExp2?`#define FOG_EXP2`:``,n.alphaToCoverage?`#define ALPHA_TO_COVERAGE`:``,n.map?`#define USE_MAP`:``,n.matcap?`#define USE_MATCAP`:``,n.envMap?`#define USE_ENVMAP`:``,n.envMap?`#define `+l:``,n.envMap?`#define `+u:``,n.envMap?`#define `+d:``,f?`#define CUBEUV_TEXEL_WIDTH `+f.texelWidth:``,f?`#define CUBEUV_TEXEL_HEIGHT `+f.texelHeight:``,f?`#define CUBEUV_MAX_MIP `+f.maxMip+`.0`:``,n.lightMap?`#define USE_LIGHTMAP`:``,n.aoMap?`#define USE_AOMAP`:``,n.bumpMap?`#define USE_BUMPMAP`:``,n.normalMap?`#define USE_NORMALMAP`:``,n.normalMapObjectSpace?`#define USE_NORMALMAP_OBJECTSPACE`:``,n.normalMapTangentSpace?`#define USE_NORMALMAP_TANGENTSPACE`:``,n.packedNormalMap?`#define USE_PACKED_NORMALMAP`:``,n.emissiveMap?`#define USE_EMISSIVEMAP`:``,n.anisotropy?`#define USE_ANISOTROPY`:``,n.anisotropyMap?`#define USE_ANISOTROPYMAP`:``,n.clearcoat?`#define USE_CLEARCOAT`:``,n.clearcoatMap?`#define USE_CLEARCOATMAP`:``,n.clearcoatRoughnessMap?`#define USE_CLEARCOAT_ROUGHNESSMAP`:``,n.clearcoatNormalMap?`#define USE_CLEARCOAT_NORMALMAP`:``,n.dispersion?`#define USE_DISPERSION`:``,n.retroreflection?`#define USE_RETROREFLECTION`:``,n.iridescence?`#define USE_IRIDESCENCE`:``,n.iridescenceMap?`#define USE_IRIDESCENCEMAP`:``,n.iridescenceThicknessMap?`#define USE_IRIDESCENCE_THICKNESSMAP`:``,n.specularMap?`#define USE_SPECULARMAP`:``,n.specularColorMap?`#define USE_SPECULAR_COLORMAP`:``,n.specularIntensityMap?`#define USE_SPECULAR_INTENSITYMAP`:``,n.roughnessMap?`#define USE_ROUGHNESSMAP`:``,n.metalnessMap?`#define USE_METALNESSMAP`:``,n.alphaMap?`#define USE_ALPHAMAP`:``,n.alphaTest?`#define USE_ALPHATEST`:``,n.alphaHash?`#define USE_ALPHAHASH`:``,n.sheen?`#define USE_SHEEN`:``,n.sheenColorMap?`#define USE_SHEEN_COLORMAP`:``,n.sheenRoughnessMap?`#define USE_SHEEN_ROUGHNESSMAP`:``,n.transmission?`#define USE_TRANSMISSION`:``,n.transmissionMap?`#define USE_TRANSMISSIONMAP`:``,n.thicknessMap?`#define USE_THICKNESSMAP`:``,n.vertexTangents&&n.flatShading===!1?`#define USE_TANGENT`:``,n.vertexColors||n.instancingColor?`#define USE_COLOR`:``,n.vertexAlphas||n.batchingColor?`#define USE_COLOR_ALPHA`:``,n.vertexUv1s?`#define USE_UV1`:``,n.vertexUv2s?`#define USE_UV2`:``,n.vertexUv3s?`#define USE_UV3`:``,n.pointsUvs?`#define USE_POINTS_UV`:``,n.gradientMap?`#define USE_GRADIENTMAP`:``,n.flatShading?`#define FLAT_SHADED`:``,n.doubleSided?`#define DOUBLE_SIDED`:``,n.flipSided?`#define FLIP_SIDED`:``,n.shadowMapEnabled?`#define USE_SHADOWMAP`:``,n.shadowMapEnabled?`#define `+c:``,n.premultipliedAlpha?`#define PREMULTIPLIED_ALPHA`:``,n.numLightProbes>0?`#define USE_LIGHT_PROBES`:``,n.numLightProbeGrids>0?`#define USE_LIGHT_PROBES_GRID`:``,n.decodeVideoTexture?`#define DECODE_VIDEO_TEXTURE`:``,n.decodeVideoTextureEmissive?`#define DECODE_VIDEO_TEXTURE_EMISSIVE`:``,n.logarithmicDepthBuffer?`#define USE_LOGARITHMIC_DEPTH_BUFFER`:``,n.reversedDepthBuffer?`#define USE_REVERSED_DEPTH_BUFFER`:``,`uniform mat4 viewMatrix;`,`uniform vec3 cameraPosition;`,`uniform bool isOrthographic;`,n.toneMapping===0?``:`#define TONE_MAPPING`,n.toneMapping===0?``:Rf.tonemapping_pars_fragment,n.toneMapping===0?``:Vm(`toneMapping`,n.toneMapping),n.dithering?`#define DITHERING`:``,n.opaque?`#define OPAQUE`:``,Rf.colorspace_pars_fragment,zm(`linearToOutputTexel`,n.outputColorSpace),Um(),n.useDepthPacking?`#define DEPTH_PACKING `+n.depthPacking:``,`
`].filter(qm).join(`
`)),o=Zm(o),o=Jm(o,n),o=Ym(o,n),s=Zm(s),s=Jm(s,n),s=Ym(s,n),o=th(o),s=th(s),n.isRawShaderMaterial!==!0&&(v=`#version 300 es
`,g=[p,`#define attribute in`,`#define varying out`,`#define texture2D texture`].join(`
`)+`
`+g,_=[`#define varying in`,n.glslVersion===`300 es`?``:`layout(location = 0) out highp vec4 pc_fragColor;`,n.glslVersion===`300 es`?``:`#define gl_FragColor pc_fragColor`,`#define gl_FragDepthEXT gl_FragDepth`,`#define texture2D texture`,`#define textureCube texture`,`#define texture2DProj textureProj`,`#define texture2DLodEXT textureLod`,`#define texture2DProjLodEXT textureProjLod`,`#define textureCubeLodEXT textureLod`,`#define texture2DGradEXT textureGrad`,`#define texture2DProjGradEXT textureProjGrad`,`#define textureCubeGradEXT textureGrad`].join(`
`)+`
`+_);let y=v+g+o,b=v+_+s,x=Mm(i,i.VERTEX_SHADER,y),S=Mm(i,i.FRAGMENT_SHADER,b);i.attachShader(h,x),i.attachShader(h,S),n.index0AttributeName===void 0?n.hasPositionAttribute===!0&&i.bindAttribLocation(h,0,`position`):i.bindAttribLocation(h,0,n.index0AttributeName),i.linkProgram(h);function C(t){if(e.debug.checkShaderErrors){let n=i.getProgramInfoLog(h)||``,r=i.getShaderInfoLog(x)||``,a=i.getShaderInfoLog(S)||``,o=n.trim(),s=r.trim(),c=a.trim(),l=!0,u=!0;if(i.getProgramParameter(h,i.LINK_STATUS)===!1){if(l=!1,typeof e.debug.onShaderError==`function`)e.debug.onShaderError(i,h,x,S);else{let e=Rm(i,x,`vertex`),n=Rm(i,S,`fragment`);W(`WebGLProgram: Shader Error `+i.getError()+` - VALIDATE_STATUS `+i.getProgramParameter(h,i.VALIDATE_STATUS)+`

Material Name: `+t.name+`
Material Type: `+t.type+`

Program Info Log: `+o+`
`+e+`
`+n)}}else o===``?(s===``||c===``)&&(u=!1):U(`WebGLProgram: Program Info Log:`,o);u&&(t.diagnostics={runnable:l,programLog:o,vertexShader:{log:s,prefix:g},fragmentShader:{log:c,prefix:_}})}i.deleteShader(x),i.deleteShader(S),w=new jm(i,h),T=Km(i,h)}let w;this.getUniforms=function(){return w===void 0&&C(this),w};let T;this.getAttributes=function(){return T===void 0&&C(this),T};let E=n.rendererExtensionParallelShaderCompile===!1;return this.isReady=function(){return E===!1&&(E=i.getProgramParameter(h,Nm)),E},this.destroy=function(){r.releaseStatesOfProgram(this),i.deleteProgram(h),this.program=void 0},this.type=n.shaderType,this.name=n.shaderName,this.id=Pm++,this.cacheKey=t,this.usedTimes=1,this.program=h,this.vertexShader=x,this.fragmentShader=S,this}var mh=0,hh=class{constructor(){this.shaderCache=new Map,this.materialCache=new Map}update(e,t,n){let r=this._getShaderCacheForMaterial(e);return r.has(t)===!1&&(r.add(t),t.usedTimes++),r.has(n)===!1&&(r.add(n),n.usedTimes++),this}remove(e){let t=this.materialCache.get(e);for(let e of t)e.usedTimes--,e.usedTimes===0&&this.shaderCache.delete(e.code);return this.materialCache.delete(e),this}getVertexShaderStage(e){return this._getShaderStage(e.vertexShader)}getFragmentShaderStage(e){return this._getShaderStage(e.fragmentShader)}dispose(){this.shaderCache.clear(),this.materialCache.clear()}_getShaderCacheForMaterial(e){let t=this.materialCache,n=t.get(e);return n===void 0&&(n=new Set,t.set(e,n)),n}_getShaderStage(e){let t=this.shaderCache,n=t.get(e);return n===void 0&&(n=new gh(e),t.set(e,n)),n}},gh=class{constructor(e){this.id=mh++,this.code=e,this.usedTimes=0}};function _h(e){return e===1030||e===37490||e===36285}function vh(e,t,n,r,i,a){let o=new $n,s=new hh,c=new Set,l=[],u=new Map,d=r.logarithmicDepthBuffer,f=r.precision,p={MeshDepthMaterial:`depth`,MeshDistanceMaterial:`distance`,MeshNormalMaterial:`normal`,MeshBasicMaterial:`basic`,MeshLambertMaterial:`lambert`,MeshPhongMaterial:`phong`,MeshToonMaterial:`toon`,MeshStandardMaterial:`physical`,MeshPhysicalMaterial:`physical`,MeshMatcapMaterial:`matcap`,LineBasicMaterial:`basic`,LineDashedMaterial:`dashed`,PointsMaterial:`points`,ShadowMaterial:`shadow`,SpriteMaterial:`sprite`};function m(e){return c.add(e),e===0?`uv`:`uv${e}`}function h(i,o,l,u,h,g){let _=u.fog,v=h.geometry,y=i.isMeshStandardMaterial||i.isMeshLambertMaterial||i.isMeshPhongMaterial?u.environment:null,b=i.isMeshStandardMaterial||i.isMeshLambertMaterial&&!i.envMap||i.isMeshPhongMaterial&&!i.envMap,x=t.get(i.envMap||y,b),S=x&&x.mapping===306?x.image.height:null,C=p[i.type];i.precision!==null&&(f=r.getMaxPrecision(i.precision),f!==i.precision&&U(`WebGLProgram.getParameters:`,i.precision,`not supported, using`,f,`instead.`));let w=v.morphAttributes.position||v.morphAttributes.normal||v.morphAttributes.color,T=w===void 0?0:w.length,E=0;v.morphAttributes.position!==void 0&&(E=1),v.morphAttributes.normal!==void 0&&(E=2),v.morphAttributes.color!==void 0&&(E=3);let D,O,k,A;if(C){let e=zf[C];D=e.vertexShader,O=e.fragmentShader}else{D=i.vertexShader,O=i.fragmentShader;let e=s.getVertexShaderStage(i),t=s.getFragmentShaderStage(i);s.update(i,e,t),k=e.id,A=t.id}let j=e.getRenderTarget(),M=e.state.buffers.depth.getReversed(),N=h.isInstancedMesh===!0,P=h.isBatchedMesh===!0,F=!!i.map,I=!!i.matcap,ee=!!x,te=!!i.aoMap,ne=!!i.lightMap,re=!!i.bumpMap&&i.wireframe===!1,ie=!!i.normalMap,ae=!!i.displacementMap,oe=!!i.emissiveMap,L=!!i.metalnessMap,se=!!i.roughnessMap,ce=i.anisotropy>0,le=i.clearcoat>0,ue=i.dispersion>0,de=i.retroreflectivity>0,fe=i.iridescence>0,pe=i.sheen>0,me=i.transmission>0,he=ce&&!!i.anisotropyMap,R=le&&!!i.clearcoatMap,ge=le&&!!i.clearcoatNormalMap,_e=le&&!!i.clearcoatRoughnessMap,ve=fe&&!!i.iridescenceMap,ye=fe&&!!i.iridescenceThicknessMap,be=pe&&!!i.sheenColorMap,xe=pe&&!!i.sheenRoughnessMap,Se=!!i.specularMap,Ce=!!i.specularColorMap,we=!!i.specularIntensityMap,Te=me&&!!i.transmissionMap,Ee=me&&!!i.thicknessMap,De=!!i.gradientMap,Oe=!!i.alphaMap,ke=i.alphaTest>0,z=!!i.alphaHash,Ae=!!i.extensions,je=0;i.toneMapped&&(j===null||j.isXRRenderTarget===!0)&&(je=e.toneMapping);let B={shaderID:C,shaderType:i.type,shaderName:i.name,vertexShader:D,fragmentShader:O,defines:i.defines,customVertexShaderID:k,customFragmentShaderID:A,isRawShaderMaterial:i.isRawShaderMaterial===!0,glslVersion:i.glslVersion,precision:f,batching:P,batchingColor:P&&h._colorsTexture!==null,instancing:N,instancingColor:N&&h.instanceColor!==null,instancingMorph:N&&h.morphTexture!==null,outputColorSpace:j===null?e.outputColorSpace:j.isXRRenderTarget===!0?j.texture.colorSpace:wn.workingColorSpace,alphaToCoverage:!!i.alphaToCoverage,map:F,matcap:I,envMap:ee,envMapMode:ee&&x.mapping,envMapCubeUVHeight:S,aoMap:te,lightMap:ne,bumpMap:re,normalMap:ie,displacementMap:ae,emissiveMap:oe,normalMapObjectSpace:ie&&i.normalMapType===1,normalMapTangentSpace:ie&&i.normalMapType===0,packedNormalMap:ie&&i.normalMapType===0&&_h(i.normalMap.format),metalnessMap:L,roughnessMap:se,anisotropy:ce,anisotropyMap:he,clearcoat:le,clearcoatMap:R,clearcoatNormalMap:ge,clearcoatRoughnessMap:_e,dispersion:ue,retroreflection:de,iridescence:fe,iridescenceMap:ve,iridescenceThicknessMap:ye,sheen:pe,sheenColorMap:be,sheenRoughnessMap:xe,specularMap:Se,specularColorMap:Ce,specularIntensityMap:we,transmission:me,transmissionMap:Te,thicknessMap:Ee,gradientMap:De,opaque:i.transparent===!1&&i.blending===1&&i.alphaToCoverage===!1,alphaMap:Oe,alphaTest:ke,alphaHash:z,combine:i.combine,mapUv:F&&m(i.map.channel),aoMapUv:te&&m(i.aoMap.channel),lightMapUv:ne&&m(i.lightMap.channel),bumpMapUv:re&&m(i.bumpMap.channel),normalMapUv:ie&&m(i.normalMap.channel),displacementMapUv:ae&&m(i.displacementMap.channel),emissiveMapUv:oe&&m(i.emissiveMap.channel),metalnessMapUv:L&&m(i.metalnessMap.channel),roughnessMapUv:se&&m(i.roughnessMap.channel),anisotropyMapUv:he&&m(i.anisotropyMap.channel),clearcoatMapUv:R&&m(i.clearcoatMap.channel),clearcoatNormalMapUv:ge&&m(i.clearcoatNormalMap.channel),clearcoatRoughnessMapUv:_e&&m(i.clearcoatRoughnessMap.channel),iridescenceMapUv:ve&&m(i.iridescenceMap.channel),iridescenceThicknessMapUv:ye&&m(i.iridescenceThicknessMap.channel),sheenColorMapUv:be&&m(i.sheenColorMap.channel),sheenRoughnessMapUv:xe&&m(i.sheenRoughnessMap.channel),specularMapUv:Se&&m(i.specularMap.channel),specularColorMapUv:Ce&&m(i.specularColorMap.channel),specularIntensityMapUv:we&&m(i.specularIntensityMap.channel),transmissionMapUv:Te&&m(i.transmissionMap.channel),thicknessMapUv:Ee&&m(i.thicknessMap.channel),alphaMapUv:Oe&&m(i.alphaMap.channel),vertexTangents:!!v.attributes.tangent&&(ie||ce),vertexNormals:!!v.attributes.normal,vertexColors:i.vertexColors,vertexAlphas:i.vertexColors===!0&&!!v.attributes.color&&v.attributes.color.itemSize===4,pointsUvs:h.isPoints===!0&&!!v.attributes.uv&&(F||Oe),fog:!!_,useFog:i.fog===!0,fogExp2:!!_&&_.isFogExp2,flatShading:i.wireframe===!1&&(i.flatShading===!0||v.attributes.normal===void 0&&ie===!1&&(i.isMeshLambertMaterial||i.isMeshPhongMaterial||i.isMeshStandardMaterial||i.isMeshPhysicalMaterial)),sizeAttenuation:i.sizeAttenuation===!0,logarithmicDepthBuffer:d,reversedDepthBuffer:M,skinning:h.isSkinnedMesh===!0,hasPositionAttribute:v.attributes.position!==void 0,morphTargets:v.morphAttributes.position!==void 0,morphNormals:v.morphAttributes.normal!==void 0,morphColors:v.morphAttributes.color!==void 0,morphTargetsCount:T,morphTextureStride:E,numSunLights:o.sun.length,numDirLights:o.directional.length,numPointLights:o.point.length,numSpotLights:o.spot.length,numSpotLightMaps:o.spotLightMap.length,numRectAreaLights:o.rectArea.length,numHemiLights:o.hemi.length,numSunLightShadows:o.sunShadowMap.length,numDirLightShadows:o.directionalShadowMap.length,numPointLightShadows:o.pointShadowMap.length,numSpotLightShadows:o.spotShadowMap.length,numSpotLightShadowsWithMaps:o.numSpotLightShadowsWithMaps,numLightProbes:o.numLightProbes,numLightProbeGrids:g.length,numClippingPlanes:a.numPlanes,numClipIntersection:a.numIntersection,dithering:i.dithering,shadowMapEnabled:e.shadowMap.enabled&&l.length>0,shadowMapType:e.shadowMap.type,toneMapping:je,decodeVideoTexture:F&&i.map.isVideoTexture===!0&&wn.getTransfer(i.map.colorSpace)===`srgb`,decodeVideoTextureEmissive:oe&&i.emissiveMap.isVideoTexture===!0&&wn.getTransfer(i.emissiveMap.colorSpace)===`srgb`,premultipliedAlpha:i.premultipliedAlpha,doubleSided:i.side===2,flipSided:i.side===1,useDepthPacking:i.depthPacking>=0,depthPacking:i.depthPacking||0,index0AttributeName:i.index0AttributeName,extensionClipCullDistance:Ae&&i.extensions.clipCullDistance===!0&&n.has(`WEBGL_clip_cull_distance`),extensionMultiDraw:(Ae&&i.extensions.multiDraw===!0||P)&&n.has(`WEBGL_multi_draw`),rendererExtensionParallelShaderCompile:n.has(`KHR_parallel_shader_compile`),customProgramCacheKey:i.customProgramCacheKey()};return B.vertexUv1s=c.has(1),B.vertexUv2s=c.has(2),B.vertexUv3s=c.has(3),c.clear(),B}function g(t){let n=[];if(t.shaderID?n.push(t.shaderID):(n.push(t.customVertexShaderID),n.push(t.customFragmentShaderID)),t.defines!==void 0)for(let e in t.defines)n.push(e),n.push(t.defines[e]);return t.isRawShaderMaterial===!1&&(_(n,t),v(n,t),n.push(e.outputColorSpace)),n.push(t.customProgramCacheKey),n.join()}function _(e,t){e.push(t.precision),e.push(t.outputColorSpace),e.push(t.envMapMode),e.push(t.envMapCubeUVHeight),e.push(t.mapUv),e.push(t.alphaMapUv),e.push(t.lightMapUv),e.push(t.aoMapUv),e.push(t.bumpMapUv),e.push(t.normalMapUv),e.push(t.displacementMapUv),e.push(t.emissiveMapUv),e.push(t.metalnessMapUv),e.push(t.roughnessMapUv),e.push(t.anisotropyMapUv),e.push(t.clearcoatMapUv),e.push(t.clearcoatNormalMapUv),e.push(t.clearcoatRoughnessMapUv),e.push(t.iridescenceMapUv),e.push(t.iridescenceThicknessMapUv),e.push(t.sheenColorMapUv),e.push(t.sheenRoughnessMapUv),e.push(t.specularMapUv),e.push(t.specularColorMapUv),e.push(t.specularIntensityMapUv),e.push(t.transmissionMapUv),e.push(t.thicknessMapUv),e.push(t.combine),e.push(t.fogExp2),e.push(t.sizeAttenuation),e.push(t.morphTargetsCount),e.push(t.morphAttributeCount),e.push(t.numSunLights),e.push(t.numDirLights),e.push(t.numPointLights),e.push(t.numSpotLights),e.push(t.numSpotLightMaps),e.push(t.numHemiLights),e.push(t.numRectAreaLights),e.push(t.numSunLightShadows),e.push(t.numDirLightShadows),e.push(t.numPointLightShadows),e.push(t.numSpotLightShadows),e.push(t.numSpotLightShadowsWithMaps),e.push(t.numLightProbes),e.push(t.shadowMapType),e.push(t.toneMapping),e.push(t.numClippingPlanes),e.push(t.numClipIntersection),e.push(t.depthPacking)}function v(e,t){o.disableAll(),t.instancing&&o.enable(0),t.instancingColor&&o.enable(1),t.instancingMorph&&o.enable(2),t.matcap&&o.enable(3),t.envMap&&o.enable(4),t.normalMapObjectSpace&&o.enable(5),t.normalMapTangentSpace&&o.enable(6),t.clearcoat&&o.enable(7),t.iridescence&&o.enable(8),t.alphaTest&&o.enable(9),t.vertexColors&&o.enable(10),t.vertexAlphas&&o.enable(11),t.vertexUv1s&&o.enable(12),t.vertexUv2s&&o.enable(13),t.vertexUv3s&&o.enable(14),t.vertexTangents&&o.enable(15),t.anisotropy&&o.enable(16),t.alphaHash&&o.enable(17),t.batching&&o.enable(18),t.dispersion&&o.enable(19),t.retroreflection&&o.enable(24),t.batchingColor&&o.enable(20),t.gradientMap&&o.enable(21),t.packedNormalMap&&o.enable(22),t.vertexNormals&&o.enable(23),e.push(o.mask),o.disableAll(),t.fog&&o.enable(0),t.useFog&&o.enable(1),t.flatShading&&o.enable(2),t.logarithmicDepthBuffer&&o.enable(3),t.reversedDepthBuffer&&o.enable(4),t.skinning&&o.enable(5),t.morphTargets&&o.enable(6),t.morphNormals&&o.enable(7),t.morphColors&&o.enable(8),t.premultipliedAlpha&&o.enable(9),t.shadowMapEnabled&&o.enable(10),t.doubleSided&&o.enable(11),t.flipSided&&o.enable(12),t.useDepthPacking&&o.enable(13),t.dithering&&o.enable(14),t.transmission&&o.enable(15),t.sheen&&o.enable(16),t.opaque&&o.enable(17),t.pointsUvs&&o.enable(18),t.decodeVideoTexture&&o.enable(19),t.decodeVideoTextureEmissive&&o.enable(20),t.alphaToCoverage&&o.enable(21),t.numLightProbeGrids>0&&o.enable(22),t.hasPositionAttribute&&o.enable(23),e.push(o.mask)}function y(e){let t=p[e.type],n;if(t){let e=zf[t];n=Yc.clone(e.uniforms)}else n=e.uniforms;return n}function b(t,n){let r=u.get(n);return r===void 0?(r=new ph(e,n,t,i),l.push(r),u.set(n,r)):++r.usedTimes,r}function x(e){if(--e.usedTimes===0){let t=l.indexOf(e);l[t]=l[l.length-1],l.pop(),u.delete(e.cacheKey),e.destroy()}}function S(e){s.remove(e)}function C(){s.dispose()}return{getParameters:h,getProgramCacheKey:g,getUniforms:y,acquireProgram:b,releaseProgram:x,releaseShaderCache:S,programs:l,dispose:C}}function yh(){let e=new WeakMap;function t(t){return e.has(t)}function n(t){let n=e.get(t);return n===void 0&&(n={},e.set(t,n)),n}function r(t){e.delete(t)}function i(t,n,r){e.get(t)[n]=r}function a(){e=new WeakMap}return{has:t,get:n,remove:r,update:i,dispose:a}}function bh(e,t){return e.groupOrder===t.groupOrder?e.renderOrder===t.renderOrder?e.material.id===t.material.id?e.materialVariant===t.materialVariant?e.z===t.z?e.id-t.id:e.z-t.z:e.materialVariant-t.materialVariant:e.material.id-t.material.id:e.renderOrder-t.renderOrder:e.groupOrder-t.groupOrder}function xh(e,t){return e.groupOrder===t.groupOrder?e.renderOrder===t.renderOrder?e.z===t.z?e.id-t.id:t.z-e.z:e.renderOrder-t.renderOrder:e.groupOrder-t.groupOrder}function Sh(){let e=[],t=0,n=[],r=[],i=[];function a(){t=0,n.length=0,r.length=0,i.length=0}function o(e){let t=0;return e.isInstancedMesh&&(t+=2),e.isSkinnedMesh&&(t+=1),t}function s(n,r,i,a,s,c){let l=e[t];return l===void 0?(l={id:n.id,object:n,geometry:r,material:i,materialVariant:o(n),groupOrder:a,renderOrder:n.renderOrder,z:s,group:c},e[t]=l):(l.id=n.id,l.object=n,l.geometry=r,l.material=i,l.materialVariant=o(n),l.groupOrder=a,l.renderOrder=n.renderOrder,l.z=s,l.group=c),t++,l}function c(e,t,a,o,c,l,u){u.reversedDepth===!0&&(c=-c);let d=s(e,t,a,o,c,l);a.transmission>0?r.push(d):a.transparent===!0?i.push(d):n.push(d)}function l(e,t,a,o,c,l){let u=s(e,t,a,o,c,l);a.transmission>0?r.unshift(u):a.transparent===!0?i.unshift(u):n.unshift(u)}function u(e,t){n.length>1&&n.sort(e||bh),r.length>1&&r.sort(t||xh),i.length>1&&i.sort(t||xh)}function d(){for(let n=t,r=e.length;n<r;n++){let t=e[n];if(t.id===null)break;t.id=null,t.object=null,t.geometry=null,t.material=null,t.group=null}}return{opaque:n,transmissive:r,transparent:i,init:a,push:c,unshift:l,finish:d,sort:u}}function Ch(){let e=new WeakMap;function t(t,n){let r=e.get(t),i;return r===void 0?(i=new Sh,e.set(t,[i])):n>=r.length?(i=new Sh,r.push(i)):i=r[n],i}function n(){e=new WeakMap}return{get:t,dispose:n}}function wh(){let e={};return{get:function(t){if(e[t.id]!==void 0)return e[t.id];let n;switch(t.type){case`SunLight`:case`DirectionalLight`:n={direction:new q,color:new Y};break;case`SpotLight`:n={position:new q,direction:new q,color:new Y,distance:0,coneCos:0,penumbraCos:0,decay:0};break;case`PointLight`:n={position:new q,color:new Y,distance:0,decay:0};break;case`HemisphereLight`:n={direction:new q,skyColor:new Y,groundColor:new Y};break;case`RectAreaLight`:n={color:new Y,position:new q,halfWidth:new q,halfHeight:new q}}return e[t.id]=n,n}}}function Th(){let e={};return{get:function(t){if(e[t.id]!==void 0)return e[t.id];let n;switch(t.type){case`SunLight`:case`DirectionalLight`:n={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new K};break;case`SpotLight`:n={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new K};break;case`PointLight`:n={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new K,shadowCameraNear:1,shadowCameraFar:1e3}}return e[t.id]=n,n}}}var Eh=0;function Dh(e,t){return(t.castShadow?2:0)-(e.castShadow?2:0)+ +!!t.map-!!e.map}function Oh(e){let t=new wh,n=Th(),r={version:0,hash:{sunLength:-1,directionalLength:-1,pointLength:-1,spotLength:-1,rectAreaLength:-1,hemiLength:-1,numSunShadows:-1,numDirectionalShadows:-1,numPointShadows:-1,numSpotShadows:-1,numSpotMaps:-1,numLightProbes:-1},ambient:[0,0,0],probe:[],sun:[],sunShadow:[],sunShadowMap:[],sunShadowMatrix:[],sunShadowCascade:[],directional:[],directionalShadow:[],directionalShadowMap:[],directionalShadowMatrix:[],spot:[],spotLightMap:[],spotShadow:[],spotShadowMap:[],spotLightMatrix:[],rectArea:[],rectAreaLTC1:null,rectAreaLTC2:null,point:[],pointShadow:[],pointShadowMap:[],pointShadowMatrix:[],hemi:[],numSpotLightShadowsWithMaps:0,numLightProbes:0};for(let e=0;e<9;e++)r.probe.push(new q);let i=new q,a=new J,o=new J;function s(i){let a=0,o=0,s=0;for(let e=0;e<9;e++)r.probe[e].set(0,0,0);let c=0,l=0,u=0,d=0,f=0,p=0,m=0,h=0,g=0,_=0,v=0,y=0,b=0,x=0;i.sort(Dh);for(let e=0,S=i.length;e<S;e++){let S=i[e],C=S.color,w=S.intensity,T=S.distance,E=null;if(S.shadow&&S.shadow.map&&(E=S.shadow.map.texture.format===1030?S.shadow.map.texture:S.shadow.map.depthTexture||S.shadow.map.texture),S.isAmbientLight)a+=C.r*w,o+=C.g*w,s+=C.b*w;else if(S.isLightProbe){for(let e=0;e<9;e++)r.probe[e].addScaledVector(S.sh.coefficients[e],w);x++}else if(S.isSunLight){let e=t.get(S);if(e.color.copy(S.color).multiplyScalar(S.intensity),S.castShadow){let e=S.shadow,t=n.get(S);t.shadowIntensity=e.intensity,t.shadowBias=e.bias,t.shadowNormalBias=e.normalBias,t.shadowRadius=e.radius,t.shadowMapSize.copy(e.mapSize).multiply(e.getFrameExtents()),r.sunShadow[l]=t,r.sunShadowMap[l]=E;let i=e.getViewportCount();for(let t=0;t<i;t++)r.sunShadowMatrix[u+t]=e.getMatrix(t),r.sunShadowCascade[u+t]=e._cascadeData[t];u+=i,l++}r.sun[c]=e,c++}else if(S.isDirectionalLight){let e=t.get(S);if(e.color.copy(S.color).multiplyScalar(S.intensity),S.castShadow){let e=S.shadow,t=n.get(S);t.shadowIntensity=e.intensity,t.shadowBias=e.bias,t.shadowNormalBias=e.normalBias,t.shadowRadius=e.radius,t.shadowMapSize=e.mapSize,r.directionalShadow[d]=t,r.directionalShadowMap[d]=E,r.directionalShadowMatrix[d]=S.shadow.matrix,g++}r.directional[d]=e,d++}else if(S.isSpotLight){let e=t.get(S);e.position.setFromMatrixPosition(S.matrixWorld),e.color.copy(C).multiplyScalar(w),e.distance=T,e.coneCos=Math.cos(S.angle),e.penumbraCos=Math.cos(S.angle*(1-S.penumbra)),e.decay=S.decay,r.spot[p]=e;let i=S.shadow;if(S.map&&(r.spotLightMap[y]=S.map,y++,i.updateMatrices(S),S.castShadow&&b++),r.spotLightMatrix[p]=i.matrix,S.castShadow){let e=n.get(S);e.shadowIntensity=i.intensity,e.shadowBias=i.bias,e.shadowNormalBias=i.normalBias,e.shadowRadius=i.radius,e.shadowMapSize=i.mapSize,r.spotShadow[p]=e,r.spotShadowMap[p]=E,v++}p++}else if(S.isRectAreaLight){let e=t.get(S);e.color.copy(C).multiplyScalar(w),e.halfWidth.set(S.width*.5,0,0),e.halfHeight.set(0,S.height*.5,0),r.rectArea[m]=e,m++}else if(S.isPointLight){let e=t.get(S);if(e.color.copy(S.color).multiplyScalar(S.intensity),e.distance=S.distance,e.decay=S.decay,S.castShadow){let e=S.shadow,t=n.get(S);t.shadowIntensity=e.intensity,t.shadowBias=e.bias,t.shadowNormalBias=e.normalBias,t.shadowRadius=e.radius,t.shadowMapSize=e.mapSize,t.shadowCameraNear=e.camera.near,t.shadowCameraFar=e.camera.far,r.pointShadow[f]=t,r.pointShadowMap[f]=E,r.pointShadowMatrix[f]=S.shadow.matrix,_++}r.point[f]=e,f++}else if(S.isHemisphereLight){let e=t.get(S);e.skyColor.copy(S.color).multiplyScalar(w),e.groundColor.copy(S.groundColor).multiplyScalar(w),r.hemi[h]=e,h++}}m>0&&(e.has(`OES_texture_float_linear`)===!0?(r.rectAreaLTC1=Q.LTC_FLOAT_1,r.rectAreaLTC2=Q.LTC_FLOAT_2):(r.rectAreaLTC1=Q.LTC_HALF_1,r.rectAreaLTC2=Q.LTC_HALF_2)),r.ambient[0]=a,r.ambient[1]=o,r.ambient[2]=s;let S=r.hash;(S.sunLength!==c||S.directionalLength!==d||S.pointLength!==f||S.spotLength!==p||S.rectAreaLength!==m||S.hemiLength!==h||S.numSunShadows!==l||S.numDirectionalShadows!==g||S.numPointShadows!==_||S.numSpotShadows!==v||S.numSpotMaps!==y||S.numLightProbes!==x)&&(r.sun.length=c,r.directional.length=d,r.spot.length=p,r.rectArea.length=m,r.point.length=f,r.hemi.length=h,r.sunShadow.length=l,r.sunShadowMap.length=l,r.sunShadowMatrix.length=u,r.sunShadowCascade.length=u,r.directionalShadow.length=g,r.directionalShadowMap.length=g,r.directionalShadowMatrix.length=g,r.pointShadow.length=_,r.pointShadowMap.length=_,r.pointShadowMatrix.length=_,r.spotShadow.length=v,r.spotShadowMap.length=v,r.spotLightMatrix.length=v+y-b,r.spotLightMap.length=y,r.numSpotLightShadowsWithMaps=b,r.numLightProbes=x,S.sunLength=c,S.directionalLength=d,S.pointLength=f,S.spotLength=p,S.rectAreaLength=m,S.hemiLength=h,S.numSunShadows=l,S.numDirectionalShadows=g,S.numPointShadows=_,S.numSpotShadows=v,S.numSpotMaps=y,S.numLightProbes=x,r.version=Eh++)}function c(e,t){let n=0,s=0,c=0,l=0,u=0,d=0,f=t.matrixWorldInverse;for(let t=0,p=e.length;t<p;t++){let p=e[t];if(p.isSunLight){let e=r.sun[n];e.direction.setFromMatrixPosition(p.matrixWorld),e.direction.transformDirection(f),n++}else if(p.isDirectionalLight){let e=r.directional[s];e.direction.setFromMatrixPosition(p.matrixWorld),i.setFromMatrixPosition(p.target.matrixWorld),e.direction.sub(i),e.direction.transformDirection(f),s++}else if(p.isSpotLight){let e=r.spot[l];e.position.setFromMatrixPosition(p.matrixWorld),e.position.applyMatrix4(f),e.direction.setFromMatrixPosition(p.matrixWorld),i.setFromMatrixPosition(p.target.matrixWorld),e.direction.sub(i),e.direction.transformDirection(f),l++}else if(p.isRectAreaLight){let e=r.rectArea[u];e.position.setFromMatrixPosition(p.matrixWorld),e.position.applyMatrix4(f),o.identity(),a.copy(p.matrixWorld),a.premultiply(f),o.extractRotation(a),e.halfWidth.set(p.width*.5,0,0),e.halfHeight.set(0,p.height*.5,0),e.halfWidth.applyMatrix4(o),e.halfHeight.applyMatrix4(o),u++}else if(p.isPointLight){let e=r.point[c];e.position.setFromMatrixPosition(p.matrixWorld),e.position.applyMatrix4(f),c++}else if(p.isHemisphereLight){let e=r.hemi[d];e.direction.setFromMatrixPosition(p.matrixWorld),e.direction.transformDirection(f),d++}}}return{setup:s,setupView:c,state:r}}function kh(e){let t=new Oh(e),n=[],r=[],i=[];function a(e){d.camera=e,n.length=0,r.length=0,i.length=0}function o(e){n.push(e)}function s(e){r.push(e)}function c(e){i.push(e)}function l(){t.setup(n)}function u(e){t.setupView(n,e)}let d={lightsArray:n,shadowsArray:r,lightProbeGridArray:i,camera:null,lights:t,transmissionRenderTarget:{},textureUnits:0};return{init:a,state:d,setupLights:l,setupLightsView:u,pushLight:o,pushShadow:s,pushLightProbeGrid:c}}function Ah(e){let t=new WeakMap;function n(n,r=0){let i=t.get(n),a;return i===void 0?(a=new kh(e),t.set(n,[a])):r>=i.length?(a=new kh(e),i.push(a)):a=i[r],a}function r(){t=new WeakMap}return{get:n,dispose:r}}var jh=`void main() {
	gl_Position = vec4( position, 1.0 );
}`,Mh=`uniform sampler2D shadow_pass;
uniform vec2 resolution;
uniform float radius;
void main() {
	const float samples = float( VSM_SAMPLES );
	float mean = 0.0;
	float squared_mean = 0.0;
	float uvStride = samples <= 1.0 ? 0.0 : 2.0 / ( samples - 1.0 );
	float uvStart = samples <= 1.0 ? 0.0 : - 1.0;
	for ( float i = 0.0; i < samples; i ++ ) {
		float uvOffset = uvStart + i * uvStride;
		#ifdef HORIZONTAL_PASS
			vec2 distribution = texture2D( shadow_pass, ( gl_FragCoord.xy + vec2( uvOffset, 0.0 ) * radius ) / resolution ).rg;
			mean += distribution.x;
			squared_mean += distribution.y * distribution.y + distribution.x * distribution.x;
		#else
			float depth = texture2D( shadow_pass, ( gl_FragCoord.xy + vec2( 0.0, uvOffset ) * radius ) / resolution ).r;
			mean += depth;
			squared_mean += depth * depth;
		#endif
	}
	mean = mean / samples;
	squared_mean = squared_mean / samples;
	float std_dev = sqrt( max( 0.0, squared_mean - mean * mean ) );
	gl_FragColor = vec4( mean, std_dev, 0.0, 1.0 );
}`,Nh=[new q(1,0,0),new q(-1,0,0),new q(0,1,0),new q(0,-1,0),new q(0,0,1),new q(0,0,-1)],Ph=[new q(0,-1,0),new q(0,-1,0),new q(0,0,1),new q(0,0,-1),new q(0,-1,0),new q(0,-1,0)],Fh=new J,Ih=new q,Lh=new q;function Rh(e,t,n){let r=new Za,i=new K,a=new K,o=new In,s=new ol,c=new sl,l={},d=n.maxTextureSize,f={0:1,1:0,2:2},p=new Qc({defines:{VSM_SAMPLES:8},uniforms:{shadow_pass:{value:null},resolution:{value:new K},radius:{value:4}},vertexShader:jh,fragmentShader:Mh}),m=p.clone();m.defines.HORIZONTAL_PASS=1;let g=new Z;g.setAttribute(`position`,new ui(new Float32Array([-1,-1,.5,3,-1,.5,-1,3,.5]),3));let _=new xa(g,p),v=this;this.enabled=!1,this.autoUpdate=!0,this.needsUpdate=!1,this.type=1;let y=this.type;this.render=function(t,n,s){if(v.enabled===!1||v.autoUpdate===!1&&v.needsUpdate===!1||t.length===0)return;this.type===2&&(U(`WebGLShadowMap: PCFSoftShadowMap has been removed. Using PCFShadowMap instead.`),this.type=1);let c=e.getRenderTarget(),l=e.getActiveCubeFace(),f=e.getActiveMipmapLevel(),p=e.state;p.setBlending(0),p.buffers.depth.getReversed()===!0?p.buffers.color.setClear(0,0,0,0):p.buffers.color.setClear(1,1,1,1),p.buffers.depth.setTest(!0),p.setScissorTest(!1);let m=y!==this.type;m&&n.traverse(function(e){e.material&&(Array.isArray(e.material)?e.material.forEach(e=>e.needsUpdate=!0):e.material.needsUpdate=!0)});for(let c=0,l=t.length;c<l;c++){let l=t[c],f=l.shadow;if(f===void 0){U(`WebGLShadowMap:`,l,`has no shadow.`);continue}if(f.autoUpdate===!1&&f.needsUpdate===!1)continue;i.copy(f.mapSize);let g=f.getFrameExtents();i.multiply(g),a.copy(f.mapSize),(i.x>d||i.y>d)&&(i.x>d&&(a.x=Math.floor(d/g.x),i.x=a.x*g.x,f.mapSize.x=a.x),i.y>d&&(a.y=Math.floor(d/g.y),i.y=a.y*g.y,f.mapSize.y=a.y));let _=e.state.buffers.depth.getReversed();if(f.camera._reversedDepth=_,f.map===null||m===!0){if(f.map!==null&&(f.map.depthTexture!==null&&(f.map.depthTexture.dispose(),f.map.depthTexture=null),f.map.dispose()),this.type===3){if(l.isPointLight){U(`WebGLShadowMap: VSM shadow maps are not supported for PointLights. Use PCF or BasicShadowMap instead.`);continue}f.map=new Rn(i.x,i.y,{format:re,type:D,minFilter:h,magFilter:h,generateMipmaps:!1}),f.map.texture.name=l.name+`.shadowMap`,f.map.depthTexture=new Xo(i.x,i.y,E),f.map.depthTexture.name=l.name+`.shadowMapDepth`,f.map.depthTexture.format=I,f.map.depthTexture.compareFunction=null,f.map.depthTexture.minFilter=u,f.map.depthTexture.magFilter=u}else l.isPointLight?(f.map=new hp(i.x),f.map.depthTexture=new Zo(i.x,T)):(f.map=new Rn(i.x,i.y),f.map.depthTexture=new Xo(i.x,i.y,T)),f.map.depthTexture.name=l.name+`.shadowMap`,f.map.depthTexture.format=I,this.type===1?(f.map.depthTexture.compareFunction=_?518:515,f.map.depthTexture.minFilter=h,f.map.depthTexture.magFilter=h):(f.map.depthTexture.compareFunction=null,f.map.depthTexture.minFilter=u,f.map.depthTexture.magFilter=u);f.camera.updateProjectionMatrix()}f.map.isWebGLCubeRenderTarget!==!0&&(f.map.width!==i.x||f.map.height!==i.y)&&f.map.setSize(i.x,i.y);let v=f.map.isWebGLCubeRenderTarget?6:f.getViewportCount();l.isPointLight!==!0&&f.updateMatrices(l,s);for(let t=0;t<v;t++){let i=f.getCamera(t);if(l.isPointLight){let e=f.camera,n=f.matrix,r=l.distance||e.far;r!==e.far&&(e.far=r,e.updateProjectionMatrix()),Ih.setFromMatrixPosition(l.matrixWorld),e.position.copy(Ih),Lh.copy(e.position),Lh.add(Nh[t]),e.up.copy(Ph[t]),e.lookAt(Lh),e.updateMatrixWorld(),n.makeTranslation(-Ih.x,-Ih.y,-Ih.z),Fh.multiplyMatrices(e.projectionMatrix,e.matrixWorldInverse),f._frustum.setFromProjectionMatrix(Fh,e.coordinateSystem,e.reversedDepth)}if(f.map.isWebGLCubeRenderTarget)e.setRenderTarget(f.map,t),e.clear();else{t===0&&(e.setRenderTarget(f.map),e.clear());let n=f.getViewport(t);o.set(a.x*n.x,a.y*n.y,a.x*n.z,a.y*n.w),p.viewport(o)}r=f.getFrustum(t),S(n,s,i,l,this.type)}f.isPointLightShadow!==!0&&this.type===3&&b(f,s),f.needsUpdate=!1}y=this.type,v.needsUpdate=!1,e.setRenderTarget(c,l,f)};function b(n,r){let a=t.update(_);p.defines.VSM_SAMPLES!==n.blurSamples&&(p.defines.VSM_SAMPLES=n.blurSamples,m.defines.VSM_SAMPLES=n.blurSamples,p.needsUpdate=!0,m.needsUpdate=!0),n.mapPass===null?n.mapPass=new Rn(i.x,i.y,{format:re,type:D}):(n.mapPass.width!==n.map.width||n.mapPass.height!==n.map.height)&&n.mapPass.setSize(n.map.width,n.map.height),p.uniforms.shadow_pass.value=n.map.depthTexture,p.uniforms.resolution.value.set(n.map.width,n.map.height),p.uniforms.radius.value=n.radius,e.setRenderTarget(n.mapPass),e.clear(),e.renderBufferDirect(r,null,a,p,_,null),m.uniforms.shadow_pass.value=n.mapPass.texture,m.uniforms.resolution.value.set(n.map.width,n.map.height),m.uniforms.radius.value=n.radius,e.setRenderTarget(n.map),e.clear(),e.renderBufferDirect(r,null,a,m,_,null)}function x(t,n,r,i){let a=null,o=r.isPointLight===!0?t.customDistanceMaterial:t.customDepthMaterial;if(o!==void 0)a=o;else if(a=r.isPointLight===!0?c:s,e.localClippingEnabled&&n.clipShadows===!0&&Array.isArray(n.clippingPlanes)&&n.clippingPlanes.length!==0||n.displacementMap&&n.displacementScale!==0||n.alphaMap&&n.alphaTest>0||n.map&&n.alphaTest>0||n.alphaToCoverage===!0){let e=a.uuid,t=n.uuid,r=l[e];r===void 0&&(r={},l[e]=r);let i=r[t];i===void 0&&(i=a.clone(),r[t]=i,n.addEventListener(`dispose`,C)),a=i}if(a.visible=n.visible,a.wireframe=n.wireframe,i===3?a.side=n.shadowSide===null?n.side:n.shadowSide:a.side=n.shadowSide===null?f[n.side]:n.shadowSide,a.alphaMap=n.alphaMap,a.alphaTest=n.alphaToCoverage===!0?.5:n.alphaTest,a.map=n.map,a.clipShadows=n.clipShadows,a.clippingPlanes=n.clippingPlanes,a.clipIntersection=n.clipIntersection,a.displacementMap=n.displacementMap,a.displacementScale=n.displacementScale,a.displacementBias=n.displacementBias,a.wireframeLinewidth=n.wireframeLinewidth,a.linewidth=n.linewidth,r.isPointLight===!0&&a.isMeshDistanceMaterial===!0){let t=e.properties.get(a);t.light=r}return a}function S(n,i,a,o,s){if(n.visible===!1)return;if(n.layers.test(i.layers)&&(n.isMesh||n.isLine||n.isPoints)&&(n.castShadow||n.receiveShadow&&s===3)&&(!n.frustumCulled||n.intersectsFrustum(r))){n.modelViewMatrix.multiplyMatrices(a.matrixWorldInverse,n.matrixWorld);let r=t.update(n),c=n.material;if(Array.isArray(c)){let t=r.groups;for(let l=0,u=t.length;l<u;l++){let u=t[l],d=c[u.materialIndex];if(d&&d.visible){let t=x(n,d,o,s);n.onBeforeShadow(e,n,i,a,r,t,u),e.renderBufferDirect(a,null,r,t,n,u),n.onAfterShadow(e,n,i,a,r,t,u)}}}else if(c.visible){let t=x(n,c,o,s);n.onBeforeShadow(e,n,i,a,r,t,null),e.renderBufferDirect(a,null,r,t,n,null),n.onAfterShadow(e,n,i,a,r,t,null)}}let c=n.children;for(let e=0,t=c.length;e<t;e++)S(c[e],i,a,o,s)}function C(e){e.target.removeEventListener(`dispose`,C);for(let t in l){let n=l[t],r=e.target.uuid;r in n&&(n[r].dispose(),delete n[r])}}}function zh(e,t){function n(){let t=!1,n=new In,r=null,i=new In(0,0,0,0);return{setMask:function(n){r!==n&&!t&&(e.colorMask(n,n,n,n),r=n)},setLocked:function(e){t=e},setClear:function(t,r,a,o,s){s===!0&&(t*=o,r*=o,a*=o),n.set(t,r,a,o),i.equals(n)===!1&&(e.clearColor(t,r,a,o),i.copy(n))},reset:function(){t=!1,r=null,i.set(-1,0,0,0)}}}function r(){let n=!1,r=!1,i=null,a=null,o=null;return{setReversed:function(e){if(r!==e){let n=t.get(`EXT_clip_control`);e?n.clipControlEXT(n.LOWER_LEFT_EXT,n.ZERO_TO_ONE_EXT):n.clipControlEXT(n.LOWER_LEFT_EXT,n.NEGATIVE_ONE_TO_ONE_EXT),r=e;let i=o;o=null,this.setClear(i)}},getReversed:function(){return r},setTest:function(t){t?L(e.DEPTH_TEST):se(e.DEPTH_TEST)},setMask:function(t){i!==t&&!n&&(e.depthMask(t),i=t)},setFunc:function(t){if(r&&(t=Vt[t]),a!==t){switch(t){case 0:e.depthFunc(e.NEVER);break;case 1:e.depthFunc(e.ALWAYS);break;case 2:e.depthFunc(e.LESS);break;case 3:e.depthFunc(e.LEQUAL);break;case 4:e.depthFunc(e.EQUAL);break;case 5:e.depthFunc(e.GEQUAL);break;case 6:e.depthFunc(e.GREATER);break;case 7:e.depthFunc(e.NOTEQUAL);break;default:e.depthFunc(e.LEQUAL)}a=t}},setLocked:function(e){n=e},setClear:function(t){o!==t&&(o=t,r&&(t=1-t),e.clearDepth(t))},reset:function(){n=!1,i=null,a=null,o=null,r=!1}}}function i(){let t=!1,n=null,r=null,i=null,a=null,o=null,s=null,c=null,l=null;return{setTest:function(n){t||(n?L(e.STENCIL_TEST):se(e.STENCIL_TEST))},setMask:function(r){n!==r&&!t&&(e.stencilMask(r),n=r)},setFunc:function(t,n,o){(r!==t||i!==n||a!==o)&&(e.stencilFunc(t,n,o),r=t,i=n,a=o)},setOp:function(t,n,r){(o!==t||s!==n||c!==r)&&(e.stencilOp(t,n,r),o=t,s=n,c=r)},setLocked:function(e){t=e},setClear:function(t){l!==t&&(e.clearStencil(t),l=t)},reset:function(){t=!1,n=null,r=null,i=null,a=null,o=null,s=null,c=null,l=null}}}let a=new n,o=new r,s=new i,c=new WeakMap,l=new WeakMap,u={},d={},f={},p=new WeakMap,m=[],h=null,g=!1,_=null,v=null,y=null,b=null,x=null,S=null,C=null,w=new Y(0,0,0),T=0,E=!1,D=null,O=null,k=null,A=null,j=null,M=e.getParameter(e.MAX_COMBINED_TEXTURE_IMAGE_UNITS),N=!1,P=0,F=e.getParameter(e.VERSION);F.indexOf(`WebGL`)===-1?F.indexOf(`OpenGL ES`)!==-1&&(P=parseFloat(/^OpenGL ES (\d)/.exec(F)[1]),N=P>=2):(P=parseFloat(/^WebGL (\d)/.exec(F)[1]),N=P>=1);let I=null,ee={},te=e.getParameter(e.SCISSOR_BOX),ne=e.getParameter(e.VIEWPORT),re=new In().fromArray(te),ie=new In().fromArray(ne);function ae(t,n,r,i){let a=new Uint8Array(4),o=e.createTexture();e.bindTexture(t,o),e.texParameteri(t,e.TEXTURE_MIN_FILTER,e.NEAREST),e.texParameteri(t,e.TEXTURE_MAG_FILTER,e.NEAREST);for(let o=0;o<r;o++)t===e.TEXTURE_3D||t===e.TEXTURE_2D_ARRAY?e.texImage3D(n,0,e.RGBA,1,1,i,0,e.RGBA,e.UNSIGNED_BYTE,a):e.texImage2D(n+o,0,e.RGBA,1,1,0,e.RGBA,e.UNSIGNED_BYTE,a);return o}let oe={};oe[e.TEXTURE_2D]=ae(e.TEXTURE_2D,e.TEXTURE_2D,1),oe[e.TEXTURE_CUBE_MAP]=ae(e.TEXTURE_CUBE_MAP,e.TEXTURE_CUBE_MAP_POSITIVE_X,6),oe[e.TEXTURE_2D_ARRAY]=ae(e.TEXTURE_2D_ARRAY,e.TEXTURE_2D_ARRAY,1,1),oe[e.TEXTURE_3D]=ae(e.TEXTURE_3D,e.TEXTURE_3D,1,1),a.setClear(0,0,0,1),o.setClear(1),s.setClear(0),L(e.DEPTH_TEST),o.setFunc(3),he(!1),R(1),L(e.CULL_FACE),pe(0);function L(t){u[t]!==!0&&(e.enable(t),u[t]=!0)}function se(t){u[t]!==!1&&(e.disable(t),u[t]=!1)}function ce(t,n){return f[t]!==n&&(e.bindFramebuffer(t,n),f[t]=n,t===e.DRAW_FRAMEBUFFER&&(f[e.FRAMEBUFFER]=n),t===e.FRAMEBUFFER&&(f[e.DRAW_FRAMEBUFFER]=n),!0)}function le(t,n){let r=m,i=!1;if(t){r=p.get(n),r===void 0&&(r=[],p.set(n,r));let a=t.textures;if(r.length!==a.length||r[0]!==e.COLOR_ATTACHMENT0){for(let t=0,n=a.length;t<n;t++)r[t]=e.COLOR_ATTACHMENT0+t;r.length=a.length,i=!0}}else r[0]!==e.BACK&&(r[0]=e.BACK,i=!0);i&&e.drawBuffers(r)}function ue(t){return h!==t&&(e.useProgram(t),h=t,!0)}let de={100:e.FUNC_ADD,101:e.FUNC_SUBTRACT,102:e.FUNC_REVERSE_SUBTRACT};de[103]=e.MIN,de[104]=e.MAX;let fe={200:e.ZERO,201:e.ONE,202:e.SRC_COLOR,204:e.SRC_ALPHA,210:e.SRC_ALPHA_SATURATE,208:e.DST_COLOR,206:e.DST_ALPHA,203:e.ONE_MINUS_SRC_COLOR,205:e.ONE_MINUS_SRC_ALPHA,209:e.ONE_MINUS_DST_COLOR,207:e.ONE_MINUS_DST_ALPHA,211:e.CONSTANT_COLOR,212:e.ONE_MINUS_CONSTANT_COLOR,213:e.CONSTANT_ALPHA,214:e.ONE_MINUS_CONSTANT_ALPHA};function pe(t,n,r,i,a,o,s,c,l,u){if(t===0)g===!0&&(se(e.BLEND),g=!1);else if(g===!1&&(L(e.BLEND),g=!0),t!==5){if(t!==_||u!==E){if((v!==100||x!==100)&&(e.blendEquation(e.FUNC_ADD),v=100,x=100),u)switch(t){case 1:e.blendFuncSeparate(e.ONE,e.ONE_MINUS_SRC_ALPHA,e.ONE,e.ONE_MINUS_SRC_ALPHA);break;case 2:e.blendFunc(e.ONE,e.ONE);break;case 3:e.blendFuncSeparate(e.ZERO,e.ONE_MINUS_SRC_COLOR,e.ZERO,e.ONE);break;case 4:e.blendFuncSeparate(e.DST_COLOR,e.ONE_MINUS_SRC_ALPHA,e.ZERO,e.ONE);break;default:W(`WebGLState: Invalid blending: `,t)}else switch(t){case 1:e.blendFuncSeparate(e.SRC_ALPHA,e.ONE_MINUS_SRC_ALPHA,e.ONE,e.ONE_MINUS_SRC_ALPHA);break;case 2:e.blendFuncSeparate(e.SRC_ALPHA,e.ONE,e.ONE,e.ONE);break;case 3:W(`WebGLState: SubtractiveBlending requires material.premultipliedAlpha = true`);break;case 4:W(`WebGLState: MultiplyBlending requires material.premultipliedAlpha = true`);break;default:W(`WebGLState: Invalid blending: `,t)}y=null,b=null,S=null,C=null,w.set(0,0,0),T=0,_=t,E=u}}else a||=n,o||=r,s||=i,(n!==v||a!==x)&&(e.blendEquationSeparate(de[n],de[a]),v=n,x=a),(r!==y||i!==b||o!==S||s!==C)&&(e.blendFuncSeparate(fe[r],fe[i],fe[o],fe[s]),y=r,b=i,S=o,C=s),(c.equals(w)===!1||l!==T)&&(e.blendColor(c.r,c.g,c.b,l),w.copy(c),T=l),_=t,E=!1}function me(t,n){t.side===2?se(e.CULL_FACE):L(e.CULL_FACE);let r=t.side===1;n&&(r=!r),he(r),t.blending===1&&t.transparent===!1?pe(0):pe(t.blending,t.blendEquation,t.blendSrc,t.blendDst,t.blendEquationAlpha,t.blendSrcAlpha,t.blendDstAlpha,t.blendColor,t.blendAlpha,t.premultipliedAlpha),o.setFunc(t.depthFunc),o.setTest(t.depthTest),o.setMask(t.depthWrite),a.setMask(t.colorWrite);let i=t.stencilWrite;s.setTest(i),i&&(s.setMask(t.stencilWriteMask),s.setFunc(t.stencilFunc,t.stencilRef,t.stencilFuncMask),s.setOp(t.stencilFail,t.stencilZFail,t.stencilZPass)),_e(t.polygonOffset,t.polygonOffsetFactor,t.polygonOffsetUnits),t.alphaToCoverage===!0?L(e.SAMPLE_ALPHA_TO_COVERAGE):se(e.SAMPLE_ALPHA_TO_COVERAGE)}function he(t){D!==t&&(t?e.frontFace(e.CW):e.frontFace(e.CCW),D=t)}function R(t){t===0?se(e.CULL_FACE):(L(e.CULL_FACE),t!==O&&(t===1?e.cullFace(e.BACK):t===2?e.cullFace(e.FRONT):e.cullFace(e.FRONT_AND_BACK))),O=t}function ge(t){t!==k&&(N&&e.lineWidth(t),k=t)}function _e(t,n,r){t?(L(e.POLYGON_OFFSET_FILL),(A!==n||j!==r)&&(A=n,j=r,o.getReversed()&&(n=-n),e.polygonOffset(n,r))):se(e.POLYGON_OFFSET_FILL)}function ve(t){t?L(e.SCISSOR_TEST):se(e.SCISSOR_TEST)}function ye(t){t===void 0&&(t=e.TEXTURE0+M-1),I!==t&&(e.activeTexture(t),I=t)}function be(t,n,r){r===void 0&&(r=I===null?e.TEXTURE0+M-1:I);let i=ee[r];i===void 0&&(i={type:void 0,texture:void 0},ee[r]=i),(i.type!==t||i.texture!==n)&&(I!==r&&(e.activeTexture(r),I=r),e.bindTexture(t,n||oe[t]),i.type=t,i.texture=n)}function xe(){let t=ee[I];t!==void 0&&t.type!==void 0&&(e.bindTexture(t.type,null),t.type=void 0,t.texture=void 0)}function Se(){try{e.compressedTexImage2D(...arguments)}catch(e){W(`WebGLState:`,e)}}function Ce(){try{e.compressedTexImage3D(...arguments)}catch(e){W(`WebGLState:`,e)}}function we(){try{e.texSubImage2D(...arguments)}catch(e){W(`WebGLState:`,e)}}function Te(){try{e.texSubImage3D(...arguments)}catch(e){W(`WebGLState:`,e)}}function Ee(){try{e.compressedTexSubImage2D(...arguments)}catch(e){W(`WebGLState:`,e)}}function De(){try{e.compressedTexSubImage3D(...arguments)}catch(e){W(`WebGLState:`,e)}}function Oe(){try{e.texStorage2D(...arguments)}catch(e){W(`WebGLState:`,e)}}function ke(){try{e.texStorage3D(...arguments)}catch(e){W(`WebGLState:`,e)}}function z(){try{e.texImage2D(...arguments)}catch(e){W(`WebGLState:`,e)}}function Ae(){try{e.texImage3D(...arguments)}catch(e){W(`WebGLState:`,e)}}function je(t){return d[t]===void 0?e.getParameter(t):d[t]}function B(t,n){d[t]!==n&&(e.pixelStorei(t,n),d[t]=n)}function V(t){re.equals(t)===!1&&(e.scissor(t.x,t.y,t.z,t.w),re.copy(t))}function Me(t){ie.equals(t)===!1&&(e.viewport(t.x,t.y,t.z,t.w),ie.copy(t))}function H(t,n){let r=l.get(n);r===void 0&&(r=new WeakMap,l.set(n,r));let i=r.get(t);i===void 0&&(i=e.getUniformBlockIndex(n,t.name),r.set(t,i))}function Ne(t,n){let r=l.get(n).get(t);c.get(n)!==r&&(e.uniformBlockBinding(n,r,t.__bindingPointIndex),c.set(n,r))}function Pe(){e.disable(e.BLEND),e.disable(e.CULL_FACE),e.disable(e.DEPTH_TEST),e.disable(e.POLYGON_OFFSET_FILL),e.disable(e.SCISSOR_TEST),e.disable(e.STENCIL_TEST),e.disable(e.SAMPLE_ALPHA_TO_COVERAGE),e.blendEquation(e.FUNC_ADD),e.blendFunc(e.ONE,e.ZERO),e.blendFuncSeparate(e.ONE,e.ZERO,e.ONE,e.ZERO),e.blendColor(0,0,0,0),e.colorMask(!0,!0,!0,!0),e.clearColor(0,0,0,0),e.depthMask(!0),e.depthFunc(e.LESS),o.setReversed(!1),e.clearDepth(1),e.stencilMask(4294967295),e.stencilFunc(e.ALWAYS,0,4294967295),e.stencilOp(e.KEEP,e.KEEP,e.KEEP),e.clearStencil(0),e.cullFace(e.BACK),e.frontFace(e.CCW),e.polygonOffset(0,0),e.activeTexture(e.TEXTURE0),e.bindFramebuffer(e.FRAMEBUFFER,null),e.bindFramebuffer(e.DRAW_FRAMEBUFFER,null),e.bindFramebuffer(e.READ_FRAMEBUFFER,null),e.useProgram(null),e.lineWidth(1),e.scissor(0,0,e.canvas.width,e.canvas.height),e.viewport(0,0,e.canvas.width,e.canvas.height),e.pixelStorei(e.PACK_ALIGNMENT,4),e.pixelStorei(e.UNPACK_ALIGNMENT,4),e.pixelStorei(e.UNPACK_FLIP_Y_WEBGL,!1),e.pixelStorei(e.UNPACK_PREMULTIPLY_ALPHA_WEBGL,!1),e.pixelStorei(e.UNPACK_COLORSPACE_CONVERSION_WEBGL,e.BROWSER_DEFAULT_WEBGL),e.pixelStorei(e.PACK_ROW_LENGTH,0),e.pixelStorei(e.PACK_SKIP_PIXELS,0),e.pixelStorei(e.PACK_SKIP_ROWS,0),e.pixelStorei(e.UNPACK_ROW_LENGTH,0),e.pixelStorei(e.UNPACK_IMAGE_HEIGHT,0),e.pixelStorei(e.UNPACK_SKIP_PIXELS,0),e.pixelStorei(e.UNPACK_SKIP_ROWS,0),e.pixelStorei(e.UNPACK_SKIP_IMAGES,0),u={},d={},I=null,ee={},f={},p=new WeakMap,m=[],h=null,g=!1,_=null,v=null,y=null,b=null,x=null,S=null,C=null,w=new Y(0,0,0),T=0,E=!1,D=null,O=null,k=null,A=null,j=null,re.set(0,0,e.canvas.width,e.canvas.height),ie.set(0,0,e.canvas.width,e.canvas.height),a.reset(),o.reset(),s.reset()}return{buffers:{color:a,depth:o,stencil:s},enable:L,disable:se,bindFramebuffer:ce,drawBuffers:le,useProgram:ue,setBlending:pe,setMaterial:me,setFlipSided:he,setCullFace:R,setLineWidth:ge,setPolygonOffset:_e,setScissorTest:ve,activeTexture:ye,bindTexture:be,unbindTexture:xe,compressedTexImage2D:Se,compressedTexImage3D:Ce,texImage2D:z,texImage3D:Ae,pixelStorei:B,getParameter:je,updateUBOMapping:H,uniformBlockBinding:Ne,texStorage2D:Oe,texStorage3D:ke,texSubImage2D:we,texSubImage3D:Te,compressedTexSubImage2D:Ee,compressedTexSubImage3D:De,scissor:V,viewport:Me,reset:Pe}}function Bh(e,t,n,r,i,a,o){let f=t.has(`WEBGL_multisampled_render_to_texture`)?t.get(`WEBGL_multisampled_render_to_texture`):null,m=typeof navigator>`u`?!1:/OculusBrowser/g.test(navigator.userAgent),_=new K,y=new WeakMap,b=new Set,x,S=new WeakMap,C=!1;try{C=typeof OffscreenCanvas<`u`&&new OffscreenCanvas(1,1).getContext(`2d`)!==null}catch{}function w(e,t){return C?new OffscreenCanvas(e,t):jt(`canvas`)}function T(e,t,n){let r=1,i=je(e);if((i.width>n||i.height>n)&&(r=n/Math.max(i.width,i.height)),r<1){if(typeof HTMLImageElement<`u`&&e instanceof HTMLImageElement||typeof HTMLCanvasElement<`u`&&e instanceof HTMLCanvasElement||typeof ImageBitmap<`u`&&e instanceof ImageBitmap||typeof VideoFrame<`u`&&e instanceof VideoFrame){let n=Math.floor(r*i.width),a=Math.floor(r*i.height);x===void 0&&(x=w(n,a));let o=t?w(n,a):x;return o.width=n,o.height=a,o.getContext(`2d`).drawImage(e,0,0,n,a),U(`WebGLRenderer: Texture has been resized from (`+i.width+`x`+i.height+`) to (`+n+`x`+a+`).`),o}return`data`in e&&U(`WebGLRenderer: Image in DataTexture is too big (`+i.width+`x`+i.height+`).`),e}return e}function E(e){return e.generateMipmaps}function D(t){e.generateMipmap(t)}function O(t){return t.isWebGLCubeRenderTarget?e.TEXTURE_CUBE_MAP:t.isWebGL3DRenderTarget?e.TEXTURE_3D:t.isWebGLArrayRenderTarget||t.isCompressedArrayTexture?e.TEXTURE_2D_ARRAY:e.TEXTURE_2D}function k(n,r,i,a,o,s=!1){if(n!==null){if(e[n]!==void 0)return e[n];U(`WebGLRenderer: Attempt to use non-existing WebGL internal format '`+n+`'`)}let c;a&&(c=t.get(`EXT_texture_norm16`),c||U(`WebGLRenderer: Unable to use normalized textures without EXT_texture_norm16 extension`));let l=r;if(r===e.RED&&(i===e.FLOAT&&(l=e.R32F),i===e.HALF_FLOAT&&(l=e.R16F),i===e.UNSIGNED_BYTE&&(l=e.R8),i===e.UNSIGNED_SHORT&&c&&(l=c.R16_EXT),i===e.SHORT&&c&&(l=c.R16_SNORM_EXT)),r===e.RED_INTEGER&&(i===e.UNSIGNED_BYTE&&(l=e.R8UI),i===e.UNSIGNED_SHORT&&(l=e.R16UI),i===e.UNSIGNED_INT&&(l=e.R32UI),i===e.BYTE&&(l=e.R8I),i===e.SHORT&&(l=e.R16I),i===e.INT&&(l=e.R32I)),r===e.RG&&(i===e.FLOAT&&(l=e.RG32F),i===e.HALF_FLOAT&&(l=e.RG16F),i===e.UNSIGNED_BYTE&&(l=e.RG8),i===e.UNSIGNED_SHORT&&c&&(l=c.RG16_EXT),i===e.SHORT&&c&&(l=c.RG16_SNORM_EXT)),r===e.RG_INTEGER&&(i===e.UNSIGNED_BYTE&&(l=e.RG8UI),i===e.UNSIGNED_SHORT&&(l=e.RG16UI),i===e.UNSIGNED_INT&&(l=e.RG32UI),i===e.BYTE&&(l=e.RG8I),i===e.SHORT&&(l=e.RG16I),i===e.INT&&(l=e.RG32I)),r===e.RGB_INTEGER&&(i===e.UNSIGNED_BYTE&&(l=e.RGB8UI),i===e.UNSIGNED_SHORT&&(l=e.RGB16UI),i===e.UNSIGNED_INT&&(l=e.RGB32UI),i===e.BYTE&&(l=e.RGB8I),i===e.SHORT&&(l=e.RGB16I),i===e.INT&&(l=e.RGB32I)),r===e.RGBA_INTEGER&&(i===e.UNSIGNED_BYTE&&(l=e.RGBA8UI),i===e.UNSIGNED_SHORT&&(l=e.RGBA16UI),i===e.UNSIGNED_INT&&(l=e.RGBA32UI),i===e.BYTE&&(l=e.RGBA8I),i===e.SHORT&&(l=e.RGBA16I),i===e.INT&&(l=e.RGBA32I)),r===e.RGB&&(i===e.UNSIGNED_SHORT&&c&&(l=c.RGB16_EXT),i===e.SHORT&&c&&(l=c.RGB16_SNORM_EXT),i===e.UNSIGNED_INT_5_9_9_9_REV&&(l=e.RGB9_E5),i===e.UNSIGNED_INT_10F_11F_11F_REV&&(l=e.R11F_G11F_B10F)),r===e.RGBA){let t=s?tt:wn.getTransfer(o);i===e.FLOAT&&(l=e.RGBA32F),i===e.HALF_FLOAT&&(l=e.RGBA16F),i===e.UNSIGNED_BYTE&&(l=t===`srgb`?e.SRGB8_ALPHA8:e.RGBA8),i===e.UNSIGNED_SHORT&&c&&(l=c.RGBA16_EXT),i===e.SHORT&&c&&(l=c.RGBA16_SNORM_EXT),i===e.UNSIGNED_SHORT_4_4_4_4&&(l=e.RGBA4),i===e.UNSIGNED_SHORT_5_5_5_1&&(l=e.RGB5_A1)}return(l===e.R16F||l===e.R32F||l===e.RG16F||l===e.RG32F||l===e.RGBA16F||l===e.RGBA32F)&&t.get(`EXT_color_buffer_float`),l}function A(t,n){let r;return t?n===null||n===1014||n===1020?r=e.DEPTH24_STENCIL8:n===1015?r=e.DEPTH32F_STENCIL8:n===1012&&(r=e.DEPTH24_STENCIL8,U(`DepthTexture: 16 bit depth attachment is not supported with stencil. Using 24-bit attachment.`)):n===null||n===1014||n===1020?r=e.DEPTH_COMPONENT24:n===1015?r=e.DEPTH_COMPONENT32F:n===1012&&(r=e.DEPTH_COMPONENT16),r}function j(e,t){return E(e)===!0||e.isFramebufferTexture&&e.minFilter!==1003&&e.minFilter!==1006?Math.log2(Math.max(t.width,t.height))+1:e.mipmaps!==void 0&&e.mipmaps.length>0?e.mipmaps.length:e.isCompressedTexture&&Array.isArray(e.image)?t.mipmaps.length:1}function M(e){let t=e.target;t.removeEventListener(`dispose`,M),P(t),t.isVideoTexture&&y.delete(t),t.isHTMLTexture&&b.delete(t)}function N(e){let t=e.target;t.removeEventListener(`dispose`,N),I(t)}function P(e){let t=r.get(e);if(t.__webglInit===void 0)return;let n=e.source,i=S.get(n);if(i){let r=i[t.__cacheKey];r.usedTimes--,r.usedTimes===0&&F(e),Object.keys(i).length===0&&S.delete(n)}r.remove(e)}function F(t){let n=r.get(t);e.deleteTexture(n.__webglTexture);let i=t.source,a=S.get(i);delete a[n.__cacheKey],o.memory.textures--}function I(t){let n=r.get(t);if(t.depthTexture&&(t.depthTexture.dispose(),r.remove(t.depthTexture)),t.isWebGLCubeRenderTarget)for(let t=0;t<6;t++){if(Array.isArray(n.__webglFramebuffer[t]))for(let r=0;r<n.__webglFramebuffer[t].length;r++)e.deleteFramebuffer(n.__webglFramebuffer[t][r]);else e.deleteFramebuffer(n.__webglFramebuffer[t]);n.__webglDepthbuffer&&e.deleteRenderbuffer(n.__webglDepthbuffer[t])}else{if(Array.isArray(n.__webglFramebuffer))for(let t=0;t<n.__webglFramebuffer.length;t++)e.deleteFramebuffer(n.__webglFramebuffer[t]);else e.deleteFramebuffer(n.__webglFramebuffer);if(n.__webglDepthbuffer&&e.deleteRenderbuffer(n.__webglDepthbuffer),n.__webglMultisampledFramebuffer&&e.deleteFramebuffer(n.__webglMultisampledFramebuffer),n.__webglColorRenderbuffer)for(let t=0;t<n.__webglColorRenderbuffer.length;t++)n.__webglColorRenderbuffer[t]&&e.deleteRenderbuffer(n.__webglColorRenderbuffer[t]);n.__webglDepthRenderbuffer&&e.deleteRenderbuffer(n.__webglDepthRenderbuffer)}let i=t.textures;for(let t=0,n=i.length;t<n;t++){let n=r.get(i[t]);n.__webglTexture&&(e.deleteTexture(n.__webglTexture),o.memory.textures--),r.remove(i[t])}r.remove(t)}let te=0;function ne(){te=0}function re(){return te}function ie(e){te=e}function ae(){let e=te;return e>=i.maxTextures&&U(`WebGLTextures: Trying to use `+(e+1)+` texture units while this GPU supports only `+i.maxTextures),te+=1,e}function oe(e){let t=[];return t.push(e.wrapS),t.push(e.wrapT),t.push(e.wrapR||0),t.push(e.magFilter),t.push(e.minFilter),t.push(e.anisotropy),t.push(e.internalFormat),t.push(e.format),t.push(e.type),t.push(e.generateMipmaps),t.push(e.premultiplyAlpha),t.push(e.flipY),t.push(e.unpackAlignment),t.push(e.colorSpace),t.join()}function L(t,i){let a=r.get(t);if(t.isVideoTexture&&z(t),t.isRenderTargetTexture===!1&&t.isExternalTexture!==!0&&t.version>0&&a.__version!==t.version){let e=t.image;if(e===null)U(`WebGLRenderer: Texture marked for update but no image data found.`);else if(e.complete===!1)U(`WebGLRenderer: Texture marked for update but image is incomplete`);else{ge(a,t,i);return}}else t.isExternalTexture&&(a.__webglTexture=t.sourceTexture?t.sourceTexture:null);n.bindTexture(e.TEXTURE_2D,a.__webglTexture,e.TEXTURE0+i)}function se(t,i){let a=r.get(t);t.isRenderTargetTexture===!1&&t.version>0&&a.__version!==t.version?ge(a,t,i):(t.isExternalTexture&&(a.__webglTexture=t.sourceTexture?t.sourceTexture:null),n.bindTexture(e.TEXTURE_2D_ARRAY,a.__webglTexture,e.TEXTURE0+i))}function ce(t,i){let a=r.get(t);t.isRenderTargetTexture===!1&&t.version>0&&a.__version!==t.version?ge(a,t,i):n.bindTexture(e.TEXTURE_3D,a.__webglTexture,e.TEXTURE0+i)}function le(t,i){let a=r.get(t);t.isCubeDepthTexture!==!0&&t.version>0&&a.__version!==t.version?_e(a,t,i):n.bindTexture(e.TEXTURE_CUBE_MAP,a.__webglTexture,e.TEXTURE0+i)}let ue={[s]:e.REPEAT,[c]:e.CLAMP_TO_EDGE,[l]:e.MIRRORED_REPEAT},de={[u]:e.NEAREST,[d]:e.NEAREST_MIPMAP_NEAREST,[p]:e.NEAREST_MIPMAP_LINEAR,[h]:e.LINEAR,[g]:e.LINEAR_MIPMAP_NEAREST,[v]:e.LINEAR_MIPMAP_LINEAR},fe={512:e.NEVER,519:e.ALWAYS,513:e.LESS,515:e.LEQUAL,514:e.EQUAL,518:e.GEQUAL,516:e.GREATER,517:e.NOTEQUAL};function pe(n,a){if(a.type===1015&&t.has(`OES_texture_float_linear`)===!1&&(a.magFilter===1006||a.magFilter===1007||a.magFilter===1005||a.magFilter===1008||a.minFilter===1006||a.minFilter===1007||a.minFilter===1005||a.minFilter===1008)&&U(`WebGLRenderer: Unable to use linear filtering with floating point textures. OES_texture_float_linear not supported on this device.`),e.texParameteri(n,e.TEXTURE_WRAP_S,ue[a.wrapS]),e.texParameteri(n,e.TEXTURE_WRAP_T,ue[a.wrapT]),(n===e.TEXTURE_3D||n===e.TEXTURE_2D_ARRAY)&&e.texParameteri(n,e.TEXTURE_WRAP_R,ue[a.wrapR]),e.texParameteri(n,e.TEXTURE_MAG_FILTER,de[a.magFilter]),e.texParameteri(n,e.TEXTURE_MIN_FILTER,de[a.minFilter]),a.compareFunction&&(e.texParameteri(n,e.TEXTURE_COMPARE_MODE,e.COMPARE_REF_TO_TEXTURE),e.texParameteri(n,e.TEXTURE_COMPARE_FUNC,fe[a.compareFunction])),t.has(`EXT_texture_filter_anisotropic`)===!0){if(a.magFilter===1003||a.minFilter!==1005&&a.minFilter!==1008||a.type===1015&&t.has(`OES_texture_float_linear`)===!1)return;if(a.anisotropy>1||r.get(a).__currentAnisotropy){let o=t.get(`EXT_texture_filter_anisotropic`);e.texParameterf(n,o.TEXTURE_MAX_ANISOTROPY_EXT,Math.min(a.anisotropy,i.getMaxAnisotropy())),r.get(a).__currentAnisotropy=a.anisotropy}}}function me(t,n){let r=!1;t.__webglInit===void 0&&(t.__webglInit=!0,n.addEventListener(`dispose`,M));let i=n.source,a=S.get(i);a===void 0&&(a={},S.set(i,a));let s=oe(n);if(s!==t.__cacheKey){a[s]===void 0&&(a[s]={texture:e.createTexture(),usedTimes:0},o.memory.textures++,r=!0),a[s].usedTimes++;let i=a[t.__cacheKey];i!==void 0&&(a[t.__cacheKey].usedTimes--,i.usedTimes===0&&F(n)),t.__cacheKey=s,t.__webglTexture=a[s].texture}return r}function he(e,t,n){return Math.floor(Math.floor(e/n)/t)}function R(t,r,i,a){let o=t.updateRanges;if(o.length===0)n.texSubImage2D(e.TEXTURE_2D,0,0,0,r.width,r.height,i,a,r.data);else{o.sort((e,t)=>e.start-t.start);let s=0;for(let e=1;e<o.length;e++){let t=o[s],n=o[e],i=t.start+t.count,a=he(n.start,r.width,4),c=he(t.start,r.width,4);n.start<=i+1&&a===c&&he(n.start+n.count-1,r.width,4)===a?t.count=Math.max(t.count,n.start+n.count-t.start):(++s,o[s]=n)}o.length=s+1;let c=n.getParameter(e.UNPACK_ROW_LENGTH),l=n.getParameter(e.UNPACK_SKIP_PIXELS),u=n.getParameter(e.UNPACK_SKIP_ROWS);n.pixelStorei(e.UNPACK_ROW_LENGTH,r.width);for(let t=0,s=o.length;t<s;t++){let s=o[t],c=Math.floor(s.start/4),l=Math.ceil(s.count/4),u=c%r.width,d=Math.floor(c/r.width),f=l;n.pixelStorei(e.UNPACK_SKIP_PIXELS,u),n.pixelStorei(e.UNPACK_SKIP_ROWS,d),n.texSubImage2D(e.TEXTURE_2D,0,u,d,f,1,i,a,r.data)}t.clearUpdateRanges(),n.pixelStorei(e.UNPACK_ROW_LENGTH,c),n.pixelStorei(e.UNPACK_SKIP_PIXELS,l),n.pixelStorei(e.UNPACK_SKIP_ROWS,u)}}function ge(t,o,s){let c=e.TEXTURE_2D;(o.isDataArrayTexture||o.isCompressedArrayTexture)&&(c=e.TEXTURE_2D_ARRAY),o.isData3DTexture&&(c=e.TEXTURE_3D);let l=me(t,o),u=o.source;n.bindTexture(c,t.__webglTexture,e.TEXTURE0+s);let d=r.get(u);if(u.version!==d.__version||l===!0){if(n.activeTexture(e.TEXTURE0+s),!(typeof ImageBitmap<`u`&&o.image instanceof ImageBitmap)){let t=wn.getPrimaries(wn.workingColorSpace),r=o.colorSpace===``?null:wn.getPrimaries(o.colorSpace),i=o.colorSpace===``||t===r?e.NONE:e.BROWSER_DEFAULT_WEBGL;n.pixelStorei(e.UNPACK_FLIP_Y_WEBGL,o.flipY),n.pixelStorei(e.UNPACK_PREMULTIPLY_ALPHA_WEBGL,o.premultiplyAlpha),n.pixelStorei(e.UNPACK_COLORSPACE_CONVERSION_WEBGL,i)}n.pixelStorei(e.UNPACK_ALIGNMENT,o.unpackAlignment);let t=T(o.image,!1,i.maxTextureSize);t=Ae(o,t);let r=a.convert(o.format,o.colorSpace),f=a.convert(o.type),p=k(o.internalFormat,r,f,o.normalized,o.colorSpace,o.isVideoTexture);pe(c,o);let m,h=o.mipmaps,g=o.isVideoTexture!==!0,_=d.__version===void 0||l===!0,v=u.dataReady,y=j(o,t);if(o.isDepthTexture)p=A(o.format===ee,o.type),_&&(g?n.texStorage2D(e.TEXTURE_2D,1,p,t.width,t.height):n.texImage2D(e.TEXTURE_2D,0,p,t.width,t.height,0,r,f,null));else if(o.isDataTexture){if(h.length>0){g&&_&&n.texStorage2D(e.TEXTURE_2D,y,p,h[0].width,h[0].height);for(let t=0,i=h.length;t<i;t++)m=h[t],g?v&&n.texSubImage2D(e.TEXTURE_2D,t,0,0,m.width,m.height,r,f,m.data):n.texImage2D(e.TEXTURE_2D,t,p,m.width,m.height,0,r,f,m.data);o.generateMipmaps=!1}else g?(_&&n.texStorage2D(e.TEXTURE_2D,y,p,t.width,t.height),v&&R(o,t,r,f)):n.texImage2D(e.TEXTURE_2D,0,p,t.width,t.height,0,r,f,t.data)}else if(o.isCompressedTexture){if(o.isCompressedArrayTexture){g&&_&&n.texStorage3D(e.TEXTURE_2D_ARRAY,y,p,h[0].width,h[0].height,t.depth);for(let i=0,a=h.length;i<a;i++)if(m=h[i],o.format!==1023){if(r!==null){if(g){if(v){if(o.layerUpdates.size>0){let t=Mf(m.width,m.height,o.format,o.type);for(let a of o.layerUpdates){let o=m.data.subarray(a*t/m.data.BYTES_PER_ELEMENT,(a+1)*t/m.data.BYTES_PER_ELEMENT);n.compressedTexSubImage3D(e.TEXTURE_2D_ARRAY,i,0,0,a,m.width,m.height,1,r,o)}}else n.compressedTexSubImage3D(e.TEXTURE_2D_ARRAY,i,0,0,0,m.width,m.height,t.depth,r,m.data)}}else n.compressedTexImage3D(e.TEXTURE_2D_ARRAY,i,p,m.width,m.height,t.depth,0,m.data,0,0)}else U(`WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()`)}else g?v&&n.texSubImage3D(e.TEXTURE_2D_ARRAY,i,0,0,0,m.width,m.height,t.depth,r,f,m.data):n.texImage3D(e.TEXTURE_2D_ARRAY,i,p,m.width,m.height,t.depth,0,r,f,m.data);o.layerUpdates.size>0&&o.clearLayerUpdates()}else{g&&_&&n.texStorage2D(e.TEXTURE_2D,y,p,h[0].width,h[0].height);for(let t=0,i=h.length;t<i;t++)m=h[t],o.format===1023?g?v&&n.texSubImage2D(e.TEXTURE_2D,t,0,0,m.width,m.height,r,f,m.data):n.texImage2D(e.TEXTURE_2D,t,p,m.width,m.height,0,r,f,m.data):r===null?U(`WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()`):g?v&&n.compressedTexSubImage2D(e.TEXTURE_2D,t,0,0,m.width,m.height,r,m.data):n.compressedTexImage2D(e.TEXTURE_2D,t,p,m.width,m.height,0,m.data)}}else if(o.isDataArrayTexture){if(g){if(_&&n.texStorage3D(e.TEXTURE_2D_ARRAY,y,p,t.width,t.height,t.depth),v){if(o.layerUpdates.size>0){let i=Mf(t.width,t.height,o.format,o.type);for(let a of o.layerUpdates){let o=t.data.subarray(a*i/t.data.BYTES_PER_ELEMENT,(a+1)*i/t.data.BYTES_PER_ELEMENT);n.texSubImage3D(e.TEXTURE_2D_ARRAY,0,0,0,a,t.width,t.height,1,r,f,o)}o.clearLayerUpdates()}else n.texSubImage3D(e.TEXTURE_2D_ARRAY,0,0,0,0,t.width,t.height,t.depth,r,f,t.data)}}else n.texImage3D(e.TEXTURE_2D_ARRAY,0,p,t.width,t.height,t.depth,0,r,f,t.data)}else if(o.isData3DTexture)g?(_&&n.texStorage3D(e.TEXTURE_3D,y,p,t.width,t.height,t.depth),v&&n.texSubImage3D(e.TEXTURE_3D,0,0,0,0,t.width,t.height,t.depth,r,f,t.data)):n.texImage3D(e.TEXTURE_3D,0,p,t.width,t.height,t.depth,0,r,f,t.data);else if(o.isFramebufferTexture){if(_){if(g)n.texStorage2D(e.TEXTURE_2D,y,p,t.width,t.height);else{let i=t.width,a=t.height;for(let t=0;t<y;t++)n.texImage2D(e.TEXTURE_2D,t,p,i,a,0,r,f,null),i>>=1,a>>=1}}}else if(o.isHTMLTexture){if(`texElementImage2D`in e){let n=e.canvas;if(n.hasAttribute(`layoutsubtree`)||n.setAttribute(`layoutsubtree`,`true`),t.parentNode!==n){n.appendChild(t),b.add(o),n.onpaint=e=>{let t=e.changedElements;for(let e of b)t.includes(e.image)&&(e.needsUpdate=!0)},n.requestPaint();return}if(e.texElementImage2D.length===3)e.texElementImage2D(e.TEXTURE_2D,e.RGBA8,t);else{let n=e.RGBA,r=e.RGBA,i=e.UNSIGNED_BYTE;e.texElementImage2D(e.TEXTURE_2D,0,n,r,i,t)}e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MIN_FILTER,e.LINEAR),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_S,e.CLAMP_TO_EDGE),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_T,e.CLAMP_TO_EDGE)}}else if(h.length>0){if(g&&_){let t=je(h[0]);n.texStorage2D(e.TEXTURE_2D,y,p,t.width,t.height)}for(let t=0,i=h.length;t<i;t++)m=h[t],g?v&&n.texSubImage2D(e.TEXTURE_2D,t,0,0,r,f,m):n.texImage2D(e.TEXTURE_2D,t,p,r,f,m);o.generateMipmaps=!1}else if(g){if(_){let r=je(t);n.texStorage2D(e.TEXTURE_2D,y,p,r.width,r.height)}v&&n.texSubImage2D(e.TEXTURE_2D,0,0,0,r,f,t)}else n.texImage2D(e.TEXTURE_2D,0,p,r,f,t);E(o)&&D(c),d.__version=u.version,o.onUpdate&&o.onUpdate(o)}t.__version=o.version}function _e(t,o,s){if(o.image.length!==6)return;let c=me(t,o),l=o.source;n.bindTexture(e.TEXTURE_CUBE_MAP,t.__webglTexture,e.TEXTURE0+s);let u=r.get(l);if(l.version!==u.__version||c===!0){n.activeTexture(e.TEXTURE0+s);let t=wn.getPrimaries(wn.workingColorSpace),r=o.colorSpace===``?null:wn.getPrimaries(o.colorSpace),d=o.colorSpace===``||t===r?e.NONE:e.BROWSER_DEFAULT_WEBGL;n.pixelStorei(e.UNPACK_FLIP_Y_WEBGL,o.flipY),n.pixelStorei(e.UNPACK_PREMULTIPLY_ALPHA_WEBGL,o.premultiplyAlpha),n.pixelStorei(e.UNPACK_ALIGNMENT,o.unpackAlignment),n.pixelStorei(e.UNPACK_COLORSPACE_CONVERSION_WEBGL,d);let f=o.isCompressedTexture||o.image[0].isCompressedTexture,p=o.image[0]&&o.image[0].isDataTexture,m=[];for(let e=0;e<6;e++)!f&&!p?m[e]=T(o.image[e],!0,i.maxCubemapSize):m[e]=p?o.image[e].image:o.image[e],m[e]=Ae(o,m[e]);let h=m[0],g=a.convert(o.format,o.colorSpace),_=a.convert(o.type),v=k(o.internalFormat,g,_,o.normalized,o.colorSpace),y=o.isVideoTexture!==!0,b=u.__version===void 0||c===!0,x=l.dataReady,S=j(o,h);pe(e.TEXTURE_CUBE_MAP,o);let C;if(f){y&&b&&n.texStorage2D(e.TEXTURE_CUBE_MAP,S,v,h.width,h.height);for(let t=0;t<6;t++){C=m[t].mipmaps;for(let r=0;r<C.length;r++){let i=C[r];o.format===1023?y?x&&n.texSubImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,r,0,0,i.width,i.height,g,_,i.data):n.texImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,r,v,i.width,i.height,0,g,_,i.data):g===null?U(`WebGLRenderer: Attempt to load unsupported compressed texture format in .setTextureCube()`):y?x&&n.compressedTexSubImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,r,0,0,i.width,i.height,g,i.data):n.compressedTexImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,r,v,i.width,i.height,0,i.data)}}}else{if(C=o.mipmaps,y&&b){C.length>0&&S++;let t=je(m[0]);n.texStorage2D(e.TEXTURE_CUBE_MAP,S,v,t.width,t.height)}for(let t=0;t<6;t++)if(p){y?x&&n.texSubImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,0,0,0,m[t].width,m[t].height,g,_,m[t].data):n.texImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,0,v,m[t].width,m[t].height,0,g,_,m[t].data);for(let r=0;r<C.length;r++){let i=C[r].image[t].image;y?x&&n.texSubImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,r+1,0,0,i.width,i.height,g,_,i.data):n.texImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,r+1,v,i.width,i.height,0,g,_,i.data)}}else{y?x&&n.texSubImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,0,0,0,g,_,m[t]):n.texImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,0,v,g,_,m[t]);for(let r=0;r<C.length;r++){let i=C[r];y?x&&n.texSubImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,r+1,0,0,g,_,i.image[t]):n.texImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+t,r+1,v,g,_,i.image[t])}}}E(o)&&D(e.TEXTURE_CUBE_MAP),u.__version=l.version,o.onUpdate&&o.onUpdate(o)}t.__version=o.version}function ve(t,i,o,s,c,l){let u=a.convert(o.format,o.colorSpace),d=a.convert(o.type),p=k(o.internalFormat,u,d,o.normalized,o.colorSpace),m=r.get(i),h=r.get(o);if(h.__renderTarget=i,!m.__hasExternalTextures){let t=Math.max(1,i.width>>l),r=Math.max(1,i.height>>l);c===e.TEXTURE_3D||c===e.TEXTURE_2D_ARRAY?n.texImage3D(c,l,p,t,r,i.depth,0,u,d,null):n.texImage2D(c,l,p,t,r,0,u,d,null)}n.bindFramebuffer(e.FRAMEBUFFER,t),ke(i)?f.framebufferTexture2DMultisampleEXT(e.FRAMEBUFFER,s,c,h.__webglTexture,0,Oe(i)):(c===e.TEXTURE_2D||c>=e.TEXTURE_CUBE_MAP_POSITIVE_X&&c<=e.TEXTURE_CUBE_MAP_NEGATIVE_Z)&&e.framebufferTexture2D(e.FRAMEBUFFER,s,c,h.__webglTexture,l),n.bindFramebuffer(e.FRAMEBUFFER,null)}function ye(t,n,r){if(e.bindRenderbuffer(e.RENDERBUFFER,t),n.depthBuffer){let i=n.depthTexture,a=i&&i.isDepthTexture?i.type:null,o=A(n.stencilBuffer,a),s=n.stencilBuffer?e.DEPTH_STENCIL_ATTACHMENT:e.DEPTH_ATTACHMENT;ke(n)?f.renderbufferStorageMultisampleEXT(e.RENDERBUFFER,Oe(n),o,n.width,n.height):r?e.renderbufferStorageMultisample(e.RENDERBUFFER,Oe(n),o,n.width,n.height):e.renderbufferStorage(e.RENDERBUFFER,o,n.width,n.height),e.framebufferRenderbuffer(e.FRAMEBUFFER,s,e.RENDERBUFFER,t)}else{let t=n.textures;for(let i=0;i<t.length;i++){let o=t[i],s=a.convert(o.format,o.colorSpace),c=a.convert(o.type),l=k(o.internalFormat,s,c,o.normalized,o.colorSpace);ke(n)?f.renderbufferStorageMultisampleEXT(e.RENDERBUFFER,Oe(n),l,n.width,n.height):r?e.renderbufferStorageMultisample(e.RENDERBUFFER,Oe(n),l,n.width,n.height):e.renderbufferStorage(e.RENDERBUFFER,l,n.width,n.height)}}e.bindRenderbuffer(e.RENDERBUFFER,null)}function be(t,i,o){let s=i.isWebGLCubeRenderTarget===!0;if(n.bindFramebuffer(e.FRAMEBUFFER,t),!(i.depthTexture&&i.depthTexture.isDepthTexture))throw Error(`THREE.WebGLTextures: renderTarget.depthTexture must be an instance of THREE.DepthTexture.`);let c=r.get(i.depthTexture);if(c.__renderTarget=i,c.__webglTexture&&i.depthTexture.image.width===i.width&&i.depthTexture.image.height===i.height||(i.depthTexture.image.width=i.width,i.depthTexture.image.height=i.height,i.depthTexture.needsUpdate=!0),s){if(c.__webglInit===void 0&&(c.__webglInit=!0,i.depthTexture.addEventListener(`dispose`,M)),c.__webglTexture===void 0){c.__webglTexture=e.createTexture(),n.bindTexture(e.TEXTURE_CUBE_MAP,c.__webglTexture),pe(e.TEXTURE_CUBE_MAP,i.depthTexture);let t=a.convert(i.depthTexture.format),r=a.convert(i.depthTexture.type),o;i.depthTexture.format===1026?o=e.DEPTH_COMPONENT24:i.depthTexture.format===1027&&(o=e.DEPTH24_STENCIL8);for(let n=0;n<6;n++)e.texImage2D(e.TEXTURE_CUBE_MAP_POSITIVE_X+n,0,o,i.width,i.height,0,t,r,null)}}else L(i.depthTexture,0);let l=c.__webglTexture,u=Oe(i),d=s?e.TEXTURE_CUBE_MAP_POSITIVE_X+o:e.TEXTURE_2D,p=i.depthTexture.format===1027?e.DEPTH_STENCIL_ATTACHMENT:e.DEPTH_ATTACHMENT;if(i.depthTexture.format===1026)ke(i)?f.framebufferTexture2DMultisampleEXT(e.FRAMEBUFFER,p,d,l,0,u):e.framebufferTexture2D(e.FRAMEBUFFER,p,d,l,0);else if(i.depthTexture.format===1027)ke(i)?f.framebufferTexture2DMultisampleEXT(e.FRAMEBUFFER,p,d,l,0,u):e.framebufferTexture2D(e.FRAMEBUFFER,p,d,l,0);else throw Error(`THREE.WebGLTextures: Unknown depthTexture format.`)}function xe(t){let i=r.get(t),a=t.isWebGLCubeRenderTarget===!0;if(i.__boundDepthTexture!==t.depthTexture){let e=t.depthTexture;if(i.__depthDisposeCallback&&i.__depthDisposeCallback(),e){let t=()=>{delete i.__boundDepthTexture,delete i.__depthDisposeCallback,e.removeEventListener(`dispose`,t)};e.addEventListener(`dispose`,t),i.__depthDisposeCallback=t}i.__boundDepthTexture=e}if(t.depthTexture&&!i.__autoAllocateDepthBuffer){if(a)for(let e=0;e<6;e++)be(i.__webglFramebuffer[e],t,e);else{let e=t.texture.mipmaps;e&&e.length>0?be(i.__webglFramebuffer[0],t,0):be(i.__webglFramebuffer,t,0)}}else if(a){i.__webglDepthbuffer=[];for(let r=0;r<6;r++)if(n.bindFramebuffer(e.FRAMEBUFFER,i.__webglFramebuffer[r]),i.__webglDepthbuffer[r]===void 0)i.__webglDepthbuffer[r]=e.createRenderbuffer(),ye(i.__webglDepthbuffer[r],t,!1);else{let n=t.stencilBuffer?e.DEPTH_STENCIL_ATTACHMENT:e.DEPTH_ATTACHMENT,a=i.__webglDepthbuffer[r];e.bindRenderbuffer(e.RENDERBUFFER,a),e.framebufferRenderbuffer(e.FRAMEBUFFER,n,e.RENDERBUFFER,a)}}else{let r=t.texture.mipmaps;if(r&&r.length>0?n.bindFramebuffer(e.FRAMEBUFFER,i.__webglFramebuffer[0]):n.bindFramebuffer(e.FRAMEBUFFER,i.__webglFramebuffer),i.__webglDepthbuffer===void 0)i.__webglDepthbuffer=e.createRenderbuffer(),ye(i.__webglDepthbuffer,t,!1);else{let n=t.stencilBuffer?e.DEPTH_STENCIL_ATTACHMENT:e.DEPTH_ATTACHMENT,r=i.__webglDepthbuffer;e.bindRenderbuffer(e.RENDERBUFFER,r),e.framebufferRenderbuffer(e.FRAMEBUFFER,n,e.RENDERBUFFER,r)}}n.bindFramebuffer(e.FRAMEBUFFER,null)}function Se(t,n,i){let a=r.get(t);n!==void 0&&ve(a.__webglFramebuffer,t,t.texture,e.COLOR_ATTACHMENT0,e.TEXTURE_2D,0),i!==void 0&&xe(t)}function Ce(t){let i=t.texture,s=r.get(t),c=r.get(i);t.addEventListener(`dispose`,N);let l=t.textures,u=t.isWebGLCubeRenderTarget===!0,d=l.length>1;if(d||(c.__webglTexture===void 0&&(c.__webglTexture=e.createTexture()),c.__version=i.version,o.memory.textures++),u){s.__webglFramebuffer=[];for(let t=0;t<6;t++)if(i.mipmaps&&i.mipmaps.length>0){s.__webglFramebuffer[t]=[];for(let n=0;n<i.mipmaps.length;n++)s.__webglFramebuffer[t][n]=e.createFramebuffer()}else s.__webglFramebuffer[t]=e.createFramebuffer()}else{if(i.mipmaps&&i.mipmaps.length>0){s.__webglFramebuffer=[];for(let t=0;t<i.mipmaps.length;t++)s.__webglFramebuffer[t]=e.createFramebuffer()}else s.__webglFramebuffer=e.createFramebuffer();if(d)for(let t=0,n=l.length;t<n;t++){let n=r.get(l[t]);n.__webglTexture===void 0&&(n.__webglTexture=e.createTexture(),o.memory.textures++)}if(t.samples>0&&ke(t)===!1){s.__webglMultisampledFramebuffer=e.createFramebuffer(),s.__webglColorRenderbuffer=[],n.bindFramebuffer(e.FRAMEBUFFER,s.__webglMultisampledFramebuffer);for(let n=0;n<l.length;n++){let r=l[n];s.__webglColorRenderbuffer[n]=e.createRenderbuffer(),e.bindRenderbuffer(e.RENDERBUFFER,s.__webglColorRenderbuffer[n]);let i=a.convert(r.format,r.colorSpace),o=a.convert(r.type),c=k(r.internalFormat,i,o,r.normalized,r.colorSpace,t.isXRRenderTarget===!0),u=Oe(t);e.renderbufferStorageMultisample(e.RENDERBUFFER,u,c,t.width,t.height),e.framebufferRenderbuffer(e.FRAMEBUFFER,e.COLOR_ATTACHMENT0+n,e.RENDERBUFFER,s.__webglColorRenderbuffer[n])}e.bindRenderbuffer(e.RENDERBUFFER,null),t.depthBuffer&&(s.__webglDepthRenderbuffer=e.createRenderbuffer(),ye(s.__webglDepthRenderbuffer,t,!0)),n.bindFramebuffer(e.FRAMEBUFFER,null)}}if(u){n.bindTexture(e.TEXTURE_CUBE_MAP,c.__webglTexture),pe(e.TEXTURE_CUBE_MAP,i);for(let n=0;n<6;n++)if(i.mipmaps&&i.mipmaps.length>0)for(let r=0;r<i.mipmaps.length;r++)ve(s.__webglFramebuffer[n][r],t,i,e.COLOR_ATTACHMENT0,e.TEXTURE_CUBE_MAP_POSITIVE_X+n,r);else ve(s.__webglFramebuffer[n],t,i,e.COLOR_ATTACHMENT0,e.TEXTURE_CUBE_MAP_POSITIVE_X+n,0);E(i)&&D(e.TEXTURE_CUBE_MAP),n.unbindTexture()}else if(d){for(let i=0,a=l.length;i<a;i++){let a=l[i],o=r.get(a),c=e.TEXTURE_2D;(t.isWebGL3DRenderTarget||t.isWebGLArrayRenderTarget)&&(c=t.isWebGL3DRenderTarget?e.TEXTURE_3D:e.TEXTURE_2D_ARRAY),n.bindTexture(c,o.__webglTexture),pe(c,a),ve(s.__webglFramebuffer,t,a,e.COLOR_ATTACHMENT0+i,c,0),E(a)&&D(c)}n.unbindTexture()}else{let r=e.TEXTURE_2D;if((t.isWebGL3DRenderTarget||t.isWebGLArrayRenderTarget)&&(r=t.isWebGL3DRenderTarget?e.TEXTURE_3D:e.TEXTURE_2D_ARRAY),n.bindTexture(r,c.__webglTexture),pe(r,i),i.mipmaps&&i.mipmaps.length>0)for(let n=0;n<i.mipmaps.length;n++)ve(s.__webglFramebuffer[n],t,i,e.COLOR_ATTACHMENT0,r,n);else ve(s.__webglFramebuffer,t,i,e.COLOR_ATTACHMENT0,r,0);E(i)&&D(r),n.unbindTexture()}t.depthBuffer&&xe(t)}function we(e){let t=e.textures;for(let i=0,a=t.length;i<a;i++){let a=t[i];if(E(a)){let t=O(e),i=r.get(a).__webglTexture;n.bindTexture(t,i),D(t),n.unbindTexture()}}}let Te=[],Ee=[];function De(t){if(t.samples>0){if(ke(t)===!1){let i=t.textures,a=t.width,o=t.height,s=e.COLOR_BUFFER_BIT,c=t.stencilBuffer?e.DEPTH_STENCIL_ATTACHMENT:e.DEPTH_ATTACHMENT,l=r.get(t),u=i.length>1;if(u)for(let t=0;t<i.length;t++)n.bindFramebuffer(e.FRAMEBUFFER,l.__webglMultisampledFramebuffer),e.framebufferRenderbuffer(e.FRAMEBUFFER,e.COLOR_ATTACHMENT0+t,e.RENDERBUFFER,null),n.bindFramebuffer(e.FRAMEBUFFER,l.__webglFramebuffer),e.framebufferTexture2D(e.DRAW_FRAMEBUFFER,e.COLOR_ATTACHMENT0+t,e.TEXTURE_2D,null,0);n.bindFramebuffer(e.READ_FRAMEBUFFER,l.__webglMultisampledFramebuffer);let d=t.texture.mipmaps;d&&d.length>0?n.bindFramebuffer(e.DRAW_FRAMEBUFFER,l.__webglFramebuffer[0]):n.bindFramebuffer(e.DRAW_FRAMEBUFFER,l.__webglFramebuffer);for(let n=0;n<i.length;n++){if(t.resolveDepthBuffer&&(t.depthBuffer&&(s|=e.DEPTH_BUFFER_BIT),t.stencilBuffer&&t.resolveStencilBuffer&&(s|=e.STENCIL_BUFFER_BIT)),u){e.framebufferRenderbuffer(e.READ_FRAMEBUFFER,e.COLOR_ATTACHMENT0,e.RENDERBUFFER,l.__webglColorRenderbuffer[n]);let t=r.get(i[n]).__webglTexture;e.framebufferTexture2D(e.DRAW_FRAMEBUFFER,e.COLOR_ATTACHMENT0,e.TEXTURE_2D,t,0)}e.blitFramebuffer(0,0,a,o,0,0,a,o,s,e.NEAREST),m===!0&&(Te.length=0,Ee.length=0,Te.push(e.COLOR_ATTACHMENT0+n),t.depthBuffer&&t.storeMultisampledDepthBuffer===!1&&(Te.push(c),Ee.push(c),e.invalidateFramebuffer(e.DRAW_FRAMEBUFFER,Ee)),e.invalidateFramebuffer(e.READ_FRAMEBUFFER,Te))}if(n.bindFramebuffer(e.READ_FRAMEBUFFER,null),n.bindFramebuffer(e.DRAW_FRAMEBUFFER,null),u)for(let t=0;t<i.length;t++){n.bindFramebuffer(e.FRAMEBUFFER,l.__webglMultisampledFramebuffer),e.framebufferRenderbuffer(e.FRAMEBUFFER,e.COLOR_ATTACHMENT0+t,e.RENDERBUFFER,l.__webglColorRenderbuffer[t]);let a=r.get(i[t]).__webglTexture;n.bindFramebuffer(e.FRAMEBUFFER,l.__webglFramebuffer),e.framebufferTexture2D(e.DRAW_FRAMEBUFFER,e.COLOR_ATTACHMENT0+t,e.TEXTURE_2D,a,0)}n.bindFramebuffer(e.DRAW_FRAMEBUFFER,l.__webglMultisampledFramebuffer)}else if(t.depthBuffer&&t.storeMultisampledDepthBuffer===!1&&m){let n=t.stencilBuffer?e.DEPTH_STENCIL_ATTACHMENT:e.DEPTH_ATTACHMENT;e.invalidateFramebuffer(e.DRAW_FRAMEBUFFER,[n])}}}function Oe(e){return Math.min(i.maxSamples,e.samples)}function ke(e){let n=r.get(e);return e.samples>0&&t.has(`WEBGL_multisampled_render_to_texture`)===!0&&n.__useRenderToTexture!==!1}function z(e){let t=o.render.frame;y.get(e)!==t&&(y.set(e,t),e.update())}function Ae(e,t){let n=e.colorSpace,r=e.format,i=e.type;return e.isCompressedTexture===!0||e.isVideoTexture===!0||n!==`srgb-linear`&&n!==``&&(wn.getTransfer(n)===`srgb`?(r!==1023||i!==1009)&&U(`WebGLTextures: sRGB encoded textures have to use RGBAFormat and UnsignedByteType.`):W(`WebGLTextures: Unsupported texture color space:`,n)),t}function je(e){return typeof HTMLImageElement<`u`&&e instanceof HTMLImageElement?(_.width=e.naturalWidth||e.width,_.height=e.naturalHeight||e.height):typeof VideoFrame<`u`&&e instanceof VideoFrame?(_.width=e.displayWidth,_.height=e.displayHeight):(_.width=e.width,_.height=e.height),_}this.allocateTextureUnit=ae,this.resetTextureUnits=ne,this.getTextureUnits=re,this.setTextureUnits=ie,this.setTexture2D=L,this.setTexture2DArray=se,this.setTexture3D=ce,this.setTextureCube=le,this.rebindTextures=Se,this.setupRenderTarget=Ce,this.updateRenderTargetMipmap=we,this.updateMultisampleRenderTarget=De,this.setupDepthRenderbuffer=xe,this.setupFrameBufferTexture=ve,this.useMultisampledRTT=ke,this.isReversedDepthBuffer=function(){return n.buffers.depth.getReversed()}}function Vh(e,t){function n(n,r=``){let i,a=wn.getTransfer(r);if(n===1009)return e.UNSIGNED_BYTE;if(n===1017)return e.UNSIGNED_SHORT_4_4_4_4;if(n===1018)return e.UNSIGNED_SHORT_5_5_5_1;if(n===35902)return e.UNSIGNED_INT_5_9_9_9_REV;if(n===35899)return e.UNSIGNED_INT_10F_11F_11F_REV;if(n===1010)return e.BYTE;if(n===1011)return e.SHORT;if(n===1012)return e.UNSIGNED_SHORT;if(n===1013)return e.INT;if(n===1014)return e.UNSIGNED_INT;if(n===1015)return e.FLOAT;if(n===1016)return e.HALF_FLOAT;if(n===1021)return e.ALPHA;if(n===1022)return e.RGB;if(n===1023)return e.RGBA;if(n===1026)return e.DEPTH_COMPONENT;if(n===1027)return e.DEPTH_STENCIL;if(n===1028)return e.RED;if(n===1029)return e.RED_INTEGER;if(n===1030)return e.RG;if(n===1031)return e.RG_INTEGER;if(n===1033)return e.RGBA_INTEGER;if(n===33776||n===33777||n===33778||n===33779){if(a===`srgb`){if(i=t.get(`WEBGL_compressed_texture_s3tc_srgb`),i!==null){if(n===33776)return i.COMPRESSED_SRGB_S3TC_DXT1_EXT;if(n===33777)return i.COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT;if(n===33778)return i.COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT;if(n===33779)return i.COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT}else return null}else if(i=t.get(`WEBGL_compressed_texture_s3tc`),i!==null){if(n===33776)return i.COMPRESSED_RGB_S3TC_DXT1_EXT;if(n===33777)return i.COMPRESSED_RGBA_S3TC_DXT1_EXT;if(n===33778)return i.COMPRESSED_RGBA_S3TC_DXT3_EXT;if(n===33779)return i.COMPRESSED_RGBA_S3TC_DXT5_EXT}else return null}if(n===35840||n===35841||n===35842||n===35843){if(i=t.get(`WEBGL_compressed_texture_pvrtc`),i!==null){if(n===35840)return i.COMPRESSED_RGB_PVRTC_4BPPV1_IMG;if(n===35841)return i.COMPRESSED_RGB_PVRTC_2BPPV1_IMG;if(n===35842)return i.COMPRESSED_RGBA_PVRTC_4BPPV1_IMG;if(n===35843)return i.COMPRESSED_RGBA_PVRTC_2BPPV1_IMG}else return null}if(n===36196||n===37492||n===37496||n===37488||n===37489||n===37490||n===37491){if(i=t.get(`WEBGL_compressed_texture_etc`),i!==null){if(n===36196||n===37492)return a===`srgb`?i.COMPRESSED_SRGB8_ETC2:i.COMPRESSED_RGB8_ETC2;if(n===37496)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ETC2_EAC:i.COMPRESSED_RGBA8_ETC2_EAC;if(n===37488)return i.COMPRESSED_R11_EAC;if(n===37489)return i.COMPRESSED_SIGNED_R11_EAC;if(n===37490)return i.COMPRESSED_RG11_EAC;if(n===37491)return i.COMPRESSED_SIGNED_RG11_EAC}else return null}if(n===37808||n===37809||n===37810||n===37811||n===37812||n===37813||n===37814||n===37815||n===37816||n===37817||n===37818||n===37819||n===37820||n===37821){if(i=t.get(`WEBGL_compressed_texture_astc`),i!==null){if(n===37808)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR:i.COMPRESSED_RGBA_ASTC_4x4_KHR;if(n===37809)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR:i.COMPRESSED_RGBA_ASTC_5x4_KHR;if(n===37810)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR:i.COMPRESSED_RGBA_ASTC_5x5_KHR;if(n===37811)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR:i.COMPRESSED_RGBA_ASTC_6x5_KHR;if(n===37812)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR:i.COMPRESSED_RGBA_ASTC_6x6_KHR;if(n===37813)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR:i.COMPRESSED_RGBA_ASTC_8x5_KHR;if(n===37814)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR:i.COMPRESSED_RGBA_ASTC_8x6_KHR;if(n===37815)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR:i.COMPRESSED_RGBA_ASTC_8x8_KHR;if(n===37816)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR:i.COMPRESSED_RGBA_ASTC_10x5_KHR;if(n===37817)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR:i.COMPRESSED_RGBA_ASTC_10x6_KHR;if(n===37818)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR:i.COMPRESSED_RGBA_ASTC_10x8_KHR;if(n===37819)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR:i.COMPRESSED_RGBA_ASTC_10x10_KHR;if(n===37820)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR:i.COMPRESSED_RGBA_ASTC_12x10_KHR;if(n===37821)return a===`srgb`?i.COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR:i.COMPRESSED_RGBA_ASTC_12x12_KHR}else return null}if(n===36492||n===36494||n===36495){if(i=t.get(`EXT_texture_compression_bptc`),i!==null){if(n===36492)return a===`srgb`?i.COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT:i.COMPRESSED_RGBA_BPTC_UNORM_EXT;if(n===36494)return i.COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT;if(n===36495)return i.COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT}else return null}if(n===36283||n===36284||n===36285||n===36286){if(i=t.get(`EXT_texture_compression_rgtc`),i!==null){if(n===36283)return i.COMPRESSED_RED_RGTC1_EXT;if(n===36284)return i.COMPRESSED_SIGNED_RED_RGTC1_EXT;if(n===36285)return i.COMPRESSED_RED_GREEN_RGTC2_EXT;if(n===36286)return i.COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT}else return null}return n===1020?e.UNSIGNED_INT_24_8:e[n]===void 0?null:e[n]}return{convert:n}}var Hh=`
void main() {

	gl_Position = vec4( position, 1.0 );

}`,Uh=`
uniform sampler2DArray depthColor;
uniform float depthWidth;
uniform float depthHeight;

void main() {

	vec2 coord = vec2( gl_FragCoord.x / depthWidth, gl_FragCoord.y / depthHeight );

	if ( coord.x >= 1.0 ) {

		gl_FragDepth = texture( depthColor, vec3( coord.x - 1.0, coord.y, 1 ) ).r;

	} else {

		gl_FragDepth = texture( depthColor, vec3( coord.x, coord.y, 0 ) ).r;

	}

}`,Wh=class{constructor(){this.texture=null,this.mesh=null,this.depthNear=0,this.depthFar=0}init(e,t){if(this.texture===null){let n=new Qo(e.texture);(e.depthNear!==t.depthNear||e.depthFar!==t.depthFar)&&(this.depthNear=e.depthNear,this.depthFar=e.depthFar),this.texture=n}}getMesh(e){if(this.texture!==null&&this.mesh===null){let t=e.cameras[0].viewport,n=new Qc({vertexShader:Hh,fragmentShader:Uh,uniforms:{depthColor:{value:this.texture},depthWidth:{value:t.z},depthHeight:{value:t.w}}});this.mesh=new xa(new jc(20,20),n)}return this.mesh}reset(){this.texture=null,this.mesh=null}getDepthTexture(){return this.texture}},Gh=class extends Ht{constructor(e,t){super();let n=this,r=null,i=1,a=null,o=`local-floor`,s=1,c=null,l=null,u=null,d=null,f=null,p=null,m=typeof XRWebGLBinding<`u`,h=new Wh,g={},_=t.getContextAttributes(),v=null,y=null,x=[],S=[],C=new K,w=null,E=null,D=new fu;D.viewport=new In;let O=new fu;O.viewport=new In;let k=[D,O],j=new Ku,M=null,N=null;this.cameraAutoUpdate=!0,this.enabled=!1,this.isPresenting=!1,this.getController=function(e){let t=x[e];return t===void 0&&(t=new vr,x[e]=t),t.getTargetRaySpace()},this.getControllerGrip=function(e){let t=x[e];return t===void 0&&(t=new vr,x[e]=t),t.getGripSpace()},this.getHand=function(e){let t=x[e];return t===void 0&&(t=new vr,x[e]=t),t.getHandSpace()};function P(e){let t=S.indexOf(e.inputSource);if(t===-1)return;let n=x[t];n!==void 0&&(n.update(e.inputSource,e.frame,c||a),n.dispatchEvent({type:e.type,data:e.inputSource}))}function te(){r.removeEventListener(`select`,P),r.removeEventListener(`selectstart`,P),r.removeEventListener(`selectend`,P),r.removeEventListener(`squeeze`,P),r.removeEventListener(`squeezestart`,P),r.removeEventListener(`squeezeend`,P),r.removeEventListener(`end`,te),r.removeEventListener(`inputsourceschange`,ne);for(let e=0;e<x.length;e++){let t=S[e];t!==null&&(S[e]=null,x[e].disconnect(t))}M=null,N=null,h.reset();for(let e in g)delete g[e];if(e.setRenderTarget(v),f=null,d=null,u=null,r=null,y=null,le.stop(),n.isPresenting=!1,e.setPixelRatio(w),e.setSize(C.width,C.height,!1),E!==null){let e=E.camera;e.fov=E.fov,e.zoom=E.zoom,e.updateProjectionMatrix(),E=null}n.dispatchEvent({type:`sessionend`})}this.setFramebufferScaleFactor=function(e){i=e,n.isPresenting===!0&&U(`WebXRManager: Cannot change framebuffer scale while presenting.`)},this.setReferenceSpaceType=function(e){o=e,n.isPresenting===!0&&U(`WebXRManager: Cannot change reference space type while presenting.`)},this.getReferenceSpace=function(){return c||a},this.setReferenceSpace=function(e){c=e},this.getBaseLayer=function(){return d===null?f:d},this.getBinding=function(){return u===null&&m&&(u=new XRWebGLBinding(r,t)),u},this.getFrame=function(){return p},this.getSession=function(){return r},this.setSession=async function(l){if(r=l,r!==null){if(v=e.getRenderTarget(),r.addEventListener(`select`,P),r.addEventListener(`selectstart`,P),r.addEventListener(`selectend`,P),r.addEventListener(`squeeze`,P),r.addEventListener(`squeezestart`,P),r.addEventListener(`squeezeend`,P),r.addEventListener(`end`,te),r.addEventListener(`inputsourceschange`,ne),_.xrCompatible!==!0&&await t.makeXRCompatible(),w=e.getPixelRatio(),e.getSize(C),m&&`createProjectionLayer`in XRWebGLBinding.prototype){let n=null,a=null,o=null;_.depth&&(o=_.stencil?t.DEPTH24_STENCIL8:t.DEPTH_COMPONENT24,n=_.stencil?ee:I,a=_.stencil?A:T);let s={colorFormat:t.RGBA8,depthFormat:o,scaleFactor:i};u=this.getBinding(),d=u.createProjectionLayer(s),r.updateRenderState({layers:[d]}),e.setPixelRatio(1),e.setSize(d.textureWidth,d.textureHeight,!1),y=new Rn(d.textureWidth,d.textureHeight,{format:F,type:b,depthTexture:new Xo(d.textureWidth,d.textureHeight,a,void 0,void 0,void 0,void 0,void 0,void 0,n),stencilBuffer:_.stencil,colorSpace:e.outputColorSpace,samples:_.antialias?4:0,resolveDepthBuffer:d.ignoreDepthValues===!1,resolveStencilBuffer:d.ignoreDepthValues===!1,storeMultisampledDepthBuffer:d.ignoreDepthValues===!1,storeMultisampledStencilBuffer:d.ignoreDepthValues===!1})}else{let n={antialias:_.antialias,alpha:!0,depth:_.depth,stencil:_.stencil,framebufferScaleFactor:i};f=new XRWebGLLayer(r,t,n),r.updateRenderState({baseLayer:f}),e.setPixelRatio(1),e.setSize(f.framebufferWidth,f.framebufferHeight,!1),y=new Rn(f.framebufferWidth,f.framebufferHeight,{format:F,type:b,colorSpace:e.outputColorSpace,stencilBuffer:_.stencil,resolveDepthBuffer:f.ignoreDepthValues===!1,resolveStencilBuffer:f.ignoreDepthValues===!1,storeMultisampledDepthBuffer:f.ignoreDepthValues===!1,storeMultisampledStencilBuffer:f.ignoreDepthValues===!1})}y.isXRRenderTarget=!0,this.setFoveation(s),c=null,a=await r.requestReferenceSpace(o),le.setContext(r),le.start(),n.isPresenting=!0,n.dispatchEvent({type:`sessionstart`})}},this.getEnvironmentBlendMode=function(){if(r!==null)return r.environmentBlendMode},this.getDepthTexture=function(){return h.getDepthTexture()};function ne(e){for(let t=0;t<e.removed.length;t++){let n=e.removed[t],r=S.indexOf(n);r>=0&&(S[r]=null,x[r].disconnect(n))}for(let t=0;t<e.added.length;t++){let n=e.added[t],r=S.indexOf(n);if(r===-1){for(let e=0;e<x.length;e++)if(e>=S.length){S.push(n),r=e;break}else if(S[e]===null){S[e]=n,r=e;break}if(r===-1)break}let i=x[r];i&&i.connect(n)}}let re=new q,ie=new q;function ae(e,t,n){re.setFromMatrixPosition(t.matrixWorld),ie.setFromMatrixPosition(n.matrixWorld);let r=re.distanceTo(ie),i=t.projectionMatrix.elements,a=n.projectionMatrix.elements,o=i[14]/(i[10]-1),s=i[14]/(i[10]+1),c=(i[9]+1)/i[5],l=(i[9]-1)/i[5],u=(i[8]-1)/i[0],d=(a[8]+1)/a[0],f=o*u,p=o*d,m=r/(-u+d),h=m*-u;if(t.matrixWorld.decompose(e.position,e.quaternion,e.scale),e.translateX(h),e.translateZ(m),e.matrixWorld.compose(e.position,e.quaternion,e.scale),e.matrixWorldInverse.copy(e.matrixWorld).invert(),i[10]===-1)e.projectionMatrix.copy(t.projectionMatrix),e.projectionMatrixInverse.copy(t.projectionMatrixInverse);else{let t=o+m,n=s+m,i=f-h,a=p+(r-h),u=c*s/n*t,d=l*s/n*t;e.projectionMatrix.makePerspective(i,a,u,d,t,n),e.projectionMatrixInverse.copy(e.projectionMatrix).invert()}}function oe(e,t){t===null?e.matrixWorld.copy(e.matrix):e.matrixWorld.multiplyMatrices(t.matrixWorld,e.matrix),e.matrixWorldInverse.copy(e.matrixWorld).invert()}this.updateCamera=function(e){if(r===null)return;let t=e.near,n=e.far;h.texture!==null&&(h.depthNear>0&&(t=h.depthNear),h.depthFar>0&&(n=h.depthFar)),j.near=O.near=D.near=t,j.far=O.far=D.far=n,(M!==j.near||N!==j.far)&&(r.updateRenderState({depthNear:j.near,depthFar:j.far}),M=j.near,N=j.far),j.layers.mask=e.layers.mask|6,D.layers.mask=j.layers.mask&-5,O.layers.mask=j.layers.mask&-3;let i=e.parent,a=j.cameras;oe(j,i);for(let e=0;e<a.length;e++)oe(a[e],i);a.length===2?ae(j,D,O):j.projectionMatrix.copy(D.projectionMatrix),E===null&&e.isPerspectiveCamera&&(E={camera:e,fov:e.fov,zoom:e.zoom}),L(e,j,i)};function L(e,t,n){n===null?e.matrix.copy(t.matrixWorld):(e.matrix.copy(n.matrixWorld),e.matrix.invert(),e.matrix.multiply(t.matrixWorld)),e.matrix.decompose(e.position,e.quaternion,e.scale),e.updateMatrixWorld(!0),e.projectionMatrix.copy(t.projectionMatrix),e.projectionMatrixInverse.copy(t.projectionMatrixInverse),e.isPerspectiveCamera&&(e.fov=Kt*2*Math.atan(1/e.projectionMatrix.elements[5]),e.zoom=1)}this.getCamera=function(){return j},this.getFoveation=function(){if(d!==null||f!==null)return s},this.setFoveation=function(e){s=e,d!==null&&(d.fixedFoveation=e),f!==null&&f.fixedFoveation!==void 0&&(f.fixedFoveation=e)},this.hasDepthSensing=function(){return h.texture!==null},this.getDepthSensingMesh=function(){return h.getMesh(j)},this.getCameraTexture=function(e){return g[e]};let se=null;function ce(t,i){if(l=i.getViewerPose(c||a),p=i,l!==null){let t=l.views;f!==null&&(e.setRenderTargetFramebuffer(y,f.framebuffer),e.setRenderTarget(y));let i=!1;t.length!==j.cameras.length&&(j.cameras.length=0,i=!0);for(let n=0;n<t.length;n++){let r=t[n],a=null;if(f!==null)a=f.getViewport(r);else{let t=u.getViewSubImage(d,r);a=t.viewport,n===0&&(e.setRenderTargetTextures(y,t.colorTexture,t.depthStencilTexture),e.setRenderTarget(y))}let o=k[n];o===void 0&&(o=new fu,o.layers.enable(n),o.viewport=new In,k[n]=o),o.matrix.fromArray(r.transform.matrix),o.matrix.decompose(o.position,o.quaternion,o.scale),o.projectionMatrix.fromArray(r.projectionMatrix),o.projectionMatrixInverse.copy(o.projectionMatrix).invert(),o.viewport.set(a.x,a.y,a.width,a.height),n===0&&(j.matrix.copy(o.matrix),j.matrix.decompose(j.position,j.quaternion,j.scale)),i===!0&&j.cameras.push(o)}let a=r.enabledFeatures;if(a&&a.includes(`depth-sensing`)&&r.depthUsage==`gpu-optimized`&&m){u=n.getBinding();let e=u.getDepthInformation(t[0]);e&&e.isValid&&e.texture&&h.init(e,r.renderState)}if(a&&a.includes(`camera-access`)&&m){e.state.unbindTexture(),u=n.getBinding();for(let e=0;e<t.length;e++){let n=t[e].camera;if(n){let e=g[n];e||(e=new Qo,g[n]=e);let t=u.getCameraImage(n);e.sourceTexture=t}}}}for(let e=0;e<x.length;e++){let t=S[e],n=x[e];t!==null&&n!==void 0&&n.update(t,i,c||a)}se&&se(t,i),i.detectedPlanes&&n.dispatchEvent({type:`planesdetected`,data:i}),p=null}let le=new If;le.setAnimationLoop(ce),this.setAnimationLoop=function(e){se=e},this.dispose=function(){}}},Kh=new J,qh=new yn;qh.set(-1,0,0,0,1,0,0,0,1);function Jh(e,t){function n(e,t){e.matrixAutoUpdate===!0&&e.updateMatrix(),t.value.copy(e.matrix)}function r(t,n){n.color.getRGB(t.fogColor.value,Jc(e)),n.isFog?(t.fogNear.value=n.near,t.fogFar.value=n.far):n.isFogExp2&&(t.fogDensity.value=n.density)}function i(e,t,n,r,i){t.isNodeMaterial?t.uniformsNeedUpdate=!1:t.isMeshBasicMaterial?a(e,t):t.isMeshLambertMaterial?(a(e,t),t.envMap&&(e.envMapIntensity.value=t.envMapIntensity)):t.isMeshToonMaterial?(a(e,t),d(e,t)):t.isMeshPhongMaterial?(a(e,t),u(e,t),t.envMap&&(e.envMapIntensity.value=t.envMapIntensity)):t.isMeshStandardMaterial?(a(e,t),f(e,t),t.isMeshPhysicalMaterial&&p(e,t,i)):t.isMeshMatcapMaterial?(a(e,t),m(e,t)):t.isMeshDepthMaterial?a(e,t):t.isMeshDistanceMaterial?(a(e,t),h(e,t)):t.isMeshNormalMaterial?a(e,t):t.isLineBasicMaterial?(o(e,t),t.isLineDashedMaterial&&s(e,t)):t.isPointsMaterial?c(e,t,n,r):t.isSpriteMaterial?l(e,t):t.isShadowMaterial?(e.color.value.copy(t.color),e.opacity.value=t.opacity):t.isShaderMaterial&&(t.uniformsNeedUpdate=!1)}function a(e,r){e.opacity.value=r.opacity,r.color&&e.diffuse.value.copy(r.color),r.emissive&&e.emissive.value.copy(r.emissive).multiplyScalar(r.emissiveIntensity),r.map&&(e.map.value=r.map,n(r.map,e.mapTransform)),r.alphaMap&&(e.alphaMap.value=r.alphaMap,n(r.alphaMap,e.alphaMapTransform)),r.bumpMap&&(e.bumpMap.value=r.bumpMap,n(r.bumpMap,e.bumpMapTransform),e.bumpScale.value=r.bumpScale,r.side===1&&(e.bumpScale.value*=-1)),r.normalMap&&(e.normalMap.value=r.normalMap,n(r.normalMap,e.normalMapTransform),e.normalScale.value.copy(r.normalScale),r.side===1&&e.normalScale.value.negate()),r.displacementMap&&(e.displacementMap.value=r.displacementMap,n(r.displacementMap,e.displacementMapTransform),e.displacementScale.value=r.displacementScale,e.displacementBias.value=r.displacementBias),r.emissiveMap&&(e.emissiveMap.value=r.emissiveMap,n(r.emissiveMap,e.emissiveMapTransform)),r.specularMap&&(e.specularMap.value=r.specularMap,n(r.specularMap,e.specularMapTransform)),r.alphaTest>0&&(e.alphaTest.value=r.alphaTest);let i=t.get(r),a=i.envMap,o=i.envMapRotation;a&&(e.envMap.value=a,e.envMapRotation.value.setFromMatrix4(Kh.makeRotationFromEuler(o)).transpose(),a.isCubeTexture&&a.isRenderTargetTexture===!1&&e.envMapRotation.value.premultiply(qh),e.reflectivity.value=r.reflectivity,e.ior.value=r.ior,e.refractionRatio.value=r.refractionRatio),r.lightMap&&(e.lightMap.value=r.lightMap,e.lightMapIntensity.value=r.lightMapIntensity,n(r.lightMap,e.lightMapTransform)),r.aoMap&&(e.aoMap.value=r.aoMap,e.aoMapIntensity.value=r.aoMapIntensity,n(r.aoMap,e.aoMapTransform))}function o(e,t){e.diffuse.value.copy(t.color),e.opacity.value=t.opacity,t.map&&(e.map.value=t.map,n(t.map,e.mapTransform))}function s(e,t){e.dashSize.value=t.dashSize,e.totalSize.value=t.dashSize+t.gapSize,e.scale.value=t.scale}function c(e,t,r,i){e.diffuse.value.copy(t.color),e.opacity.value=t.opacity,e.size.value=t.size*r,e.scale.value=i*.5,t.map&&(e.map.value=t.map,n(t.map,e.uvTransform)),t.alphaMap&&(e.alphaMap.value=t.alphaMap,n(t.alphaMap,e.alphaMapTransform)),t.alphaTest>0&&(e.alphaTest.value=t.alphaTest)}function l(e,t){e.diffuse.value.copy(t.color),e.opacity.value=t.opacity,e.rotation.value=t.rotation,t.map&&(e.map.value=t.map,n(t.map,e.mapTransform)),t.alphaMap&&(e.alphaMap.value=t.alphaMap,n(t.alphaMap,e.alphaMapTransform)),t.alphaTest>0&&(e.alphaTest.value=t.alphaTest)}function u(e,t){e.specular.value.copy(t.specular),e.shininess.value=Math.max(t.shininess,1e-4)}function d(e,t){t.gradientMap&&(e.gradientMap.value=t.gradientMap)}function f(e,t){e.metalness.value=t.metalness,t.metalnessMap&&(e.metalnessMap.value=t.metalnessMap,n(t.metalnessMap,e.metalnessMapTransform)),e.roughness.value=t.roughness,t.roughnessMap&&(e.roughnessMap.value=t.roughnessMap,n(t.roughnessMap,e.roughnessMapTransform)),t.envMap&&(e.envMapIntensity.value=t.envMapIntensity)}function p(e,t,r){e.ior.value=t.ior,t.sheen>0&&(e.sheenColor.value.copy(t.sheenColor).multiplyScalar(t.sheen),e.sheenRoughness.value=t.sheenRoughness,t.sheenColorMap&&(e.sheenColorMap.value=t.sheenColorMap,n(t.sheenColorMap,e.sheenColorMapTransform)),t.sheenRoughnessMap&&(e.sheenRoughnessMap.value=t.sheenRoughnessMap,n(t.sheenRoughnessMap,e.sheenRoughnessMapTransform))),t.clearcoat>0&&(e.clearcoat.value=t.clearcoat,e.clearcoatRoughness.value=t.clearcoatRoughness,t.clearcoatMap&&(e.clearcoatMap.value=t.clearcoatMap,n(t.clearcoatMap,e.clearcoatMapTransform)),t.clearcoatRoughnessMap&&(e.clearcoatRoughnessMap.value=t.clearcoatRoughnessMap,n(t.clearcoatRoughnessMap,e.clearcoatRoughnessMapTransform)),t.clearcoatNormalMap&&(e.clearcoatNormalMap.value=t.clearcoatNormalMap,n(t.clearcoatNormalMap,e.clearcoatNormalMapTransform),e.clearcoatNormalScale.value.copy(t.clearcoatNormalScale),t.side===1&&e.clearcoatNormalScale.value.negate())),t.dispersion>0&&(e.dispersion.value=t.dispersion),t.retroreflectivity>0&&(e.retroreflectivity.value=t.retroreflectivity),t.iridescence>0&&(e.iridescence.value=t.iridescence,e.iridescenceIOR.value=t.iridescenceIOR,e.iridescenceThicknessMinimum.value=t.iridescenceThicknessRange[0],e.iridescenceThicknessMaximum.value=t.iridescenceThicknessRange[1],t.iridescenceMap&&(e.iridescenceMap.value=t.iridescenceMap,n(t.iridescenceMap,e.iridescenceMapTransform)),t.iridescenceThicknessMap&&(e.iridescenceThicknessMap.value=t.iridescenceThicknessMap,n(t.iridescenceThicknessMap,e.iridescenceThicknessMapTransform))),t.transmission>0&&(e.transmission.value=t.transmission,e.transmissionSamplerMap.value=r.texture,e.transmissionSamplerSize.value.set(r.width,r.height),t.transmissionMap&&(e.transmissionMap.value=t.transmissionMap,n(t.transmissionMap,e.transmissionMapTransform)),e.thickness.value=t.thickness,t.thicknessMap&&(e.thicknessMap.value=t.thicknessMap,n(t.thicknessMap,e.thicknessMapTransform)),e.attenuationDistance.value=t.attenuationDistance,e.attenuationColor.value.copy(t.attenuationColor)),t.anisotropy>0&&(e.anisotropyVector.value.set(t.anisotropy*Math.cos(t.anisotropyRotation),t.anisotropy*Math.sin(t.anisotropyRotation)),t.anisotropyMap&&(e.anisotropyMap.value=t.anisotropyMap,n(t.anisotropyMap,e.anisotropyMapTransform))),e.specularIntensity.value=t.specularIntensity,e.specularColor.value.copy(t.specularColor),t.specularColorMap&&(e.specularColorMap.value=t.specularColorMap,n(t.specularColorMap,e.specularColorMapTransform)),t.specularIntensityMap&&(e.specularIntensityMap.value=t.specularIntensityMap,n(t.specularIntensityMap,e.specularIntensityMapTransform))}function m(e,t){t.matcap&&(e.matcap.value=t.matcap)}function h(e,n){let r=t.get(n).light;e.referencePosition.value.setFromMatrixPosition(r.matrixWorld),e.nearDistance.value=r.shadow.camera.near,e.farDistance.value=r.shadow.camera.far}return{refreshFogUniforms:r,refreshMaterialUniforms:i}}function Yh(e,t,n,r){let i={},a={},o=[],s=e.getParameter(e.MAX_UNIFORM_BUFFER_BINDINGS);function c(e,t){let n=t.program;r.uniformBlockBinding(e,n)}function l(e,n){let o=i[e.id];o===void 0&&(g(e),o=u(e),i[e.id]=o,e.addEventListener(`dispose`,v));let s=n.program;r.updateUBOMapping(e,s);let c=t.render.frame;a[e.id]!==c&&(f(e),a[e.id]=c)}function u(t){let n=d();t.__bindingPointIndex=n;let r=e.createBuffer(),i=t.__size,a=t.usage;return e.bindBuffer(e.UNIFORM_BUFFER,r),e.bufferData(e.UNIFORM_BUFFER,i,a),e.bindBuffer(e.UNIFORM_BUFFER,null),e.bindBufferBase(e.UNIFORM_BUFFER,n,r),r}function d(){for(let e=0;e<s;e++)if(o.indexOf(e)===-1)return o.push(e),e;return W(`WebGLRenderer: Maximum number of simultaneously usable uniforms groups reached.`),0}function f(t){let n=i[t.id],r=t.uniforms,a=t.__cache;e.bindBuffer(e.UNIFORM_BUFFER,n);for(let e=0,t=r.length;e<t;e++){let t=r[e];if(Array.isArray(t))for(let n=0,r=t.length;n<r;n++)p(t[n],e,n,a);else p(t,e,0,a)}e.bindBuffer(e.UNIFORM_BUFFER,null)}function p(t,n,r,i){if(h(t,n,r,i)===!0){let n=t.__offset,r=t.value;if(Array.isArray(r)){let e=0;for(let n=0;n<r.length;n++){let i=r[n],a=_(i);m(i,t.__data,e),typeof i!=`number`&&typeof i!=`boolean`&&!i.isMatrix3&&!ArrayBuffer.isView(i)&&(e+=a.storage/Float32Array.BYTES_PER_ELEMENT)}}else m(r,t.__data,0);e.bufferSubData(e.UNIFORM_BUFFER,n,t.__data)}}function m(e,t,n){typeof e==`number`||typeof e==`boolean`?t[0]=e:e.isMatrix3?(t[0]=e.elements[0],t[1]=e.elements[1],t[2]=e.elements[2],t[3]=0,t[4]=e.elements[3],t[5]=e.elements[4],t[6]=e.elements[5],t[7]=0,t[8]=e.elements[6],t[9]=e.elements[7],t[10]=e.elements[8],t[11]=0):ArrayBuffer.isView(e)?t.set(new e.constructor(e.buffer,e.byteOffset,t.length)):e.toArray(t,n)}function h(e,t,n,r){let i=e.value,a=t+`_`+n;if(r[a]===void 0)return r[a]=typeof i==`number`||typeof i==`boolean`?i:ArrayBuffer.isView(i)?i.slice():i.clone(),!0;{let e=r[a];if(typeof i==`number`||typeof i==`boolean`){if(e!==i)return r[a]=i,!0}else if(ArrayBuffer.isView(i))return!0;else if(e.equals(i)===!1)return e.copy(i),!0}return!1}function g(e){let t=e.uniforms,n=0;for(let e=0,r=t.length;e<r;e++){let r=Array.isArray(t[e])?t[e]:[t[e]];for(let e=0,t=r.length;e<t;e++){let t=r[e],i=Array.isArray(t.value)?t.value:[t.value];for(let e=0,r=i.length;e<r;e++){let r=i[e],a=_(r),o=n%16,s=o%a.boundary,c=o+s;n+=s,c!==0&&16-c<a.storage&&(n+=16-c),t.__data=new Float32Array(a.storage/Float32Array.BYTES_PER_ELEMENT),t.__offset=n,n+=a.storage}}}let r=n%16;return r>0&&(n+=16-r),e.__size=n,e.__cache={},this}function _(e){let t={boundary:0,storage:0};return typeof e==`number`||typeof e==`boolean`?(t.boundary=4,t.storage=4):e.isVector2?(t.boundary=8,t.storage=8):e.isVector3||e.isColor?(t.boundary=16,t.storage=12):e.isVector4?(t.boundary=16,t.storage=16):e.isMatrix3?(t.boundary=48,t.storage=48):e.isMatrix4?(t.boundary=64,t.storage=64):e.isTexture?U(`WebGLRenderer: Texture samplers can not be part of an uniforms group.`):ArrayBuffer.isView(e)?(t.boundary=16,t.storage=e.byteLength):U(`WebGLRenderer: Unsupported uniform value type.`,e),t}function v(t){let n=t.target;n.removeEventListener(`dispose`,v);let r=o.indexOf(n.__bindingPointIndex);o.splice(r,1),e.deleteBuffer(i[n.id]),delete i[n.id],delete a[n.id]}function y(){for(let t in i)e.deleteBuffer(i[t]);o=[],i={},a={}}return{bind:c,update:l,dispose:y}}var Xh=new Uint16Array([12469,15057,12620,14925,13266,14620,13807,14376,14323,13990,14545,13625,14713,13328,14840,12882,14931,12528,14996,12233,15039,11829,15066,11525,15080,11295,15085,10976,15082,10705,15073,10495,13880,14564,13898,14542,13977,14430,14158,14124,14393,13732,14556,13410,14702,12996,14814,12596,14891,12291,14937,11834,14957,11489,14958,11194,14943,10803,14921,10506,14893,10278,14858,9960,14484,14039,14487,14025,14499,13941,14524,13740,14574,13468,14654,13106,14743,12678,14818,12344,14867,11893,14889,11509,14893,11180,14881,10751,14852,10428,14812,10128,14765,9754,14712,9466,14764,13480,14764,13475,14766,13440,14766,13347,14769,13070,14786,12713,14816,12387,14844,11957,14860,11549,14868,11215,14855,10751,14825,10403,14782,10044,14729,9651,14666,9352,14599,9029,14967,12835,14966,12831,14963,12804,14954,12723,14936,12564,14917,12347,14900,11958,14886,11569,14878,11247,14859,10765,14828,10401,14784,10011,14727,9600,14660,9289,14586,8893,14508,8533,15111,12234,15110,12234,15104,12216,15092,12156,15067,12010,15028,11776,14981,11500,14942,11205,14902,10752,14861,10393,14812,9991,14752,9570,14682,9252,14603,8808,14519,8445,14431,8145,15209,11449,15208,11451,15202,11451,15190,11438,15163,11384,15117,11274,15055,10979,14994,10648,14932,10343,14871,9936,14803,9532,14729,9218,14645,8742,14556,8381,14461,8020,14365,7603,15273,10603,15272,10607,15267,10619,15256,10631,15231,10614,15182,10535,15118,10389,15042,10167,14963,9787,14883,9447,14800,9115,14710,8665,14615,8318,14514,7911,14411,7507,14279,7198,15314,9675,15313,9683,15309,9712,15298,9759,15277,9797,15229,9773,15166,9668,15084,9487,14995,9274,14898,8910,14800,8539,14697,8234,14590,7790,14479,7409,14367,7067,14178,6621,15337,8619,15337,8631,15333,8677,15325,8769,15305,8871,15264,8940,15202,8909,15119,8775,15022,8565,14916,8328,14804,8009,14688,7614,14569,7287,14448,6888,14321,6483,14088,6171,15350,7402,15350,7419,15347,7480,15340,7613,15322,7804,15287,7973,15229,8057,15148,8012,15046,7846,14933,7611,14810,7357,14682,7069,14552,6656,14421,6316,14251,5948,14007,5528,15356,5942,15356,5977,15353,6119,15348,6294,15332,6551,15302,6824,15249,7044,15171,7122,15070,7050,14949,6861,14818,6611,14679,6349,14538,6067,14398,5651,14189,5311,13935,4958,15359,4123,15359,4153,15356,4296,15353,4646,15338,5160,15311,5508,15263,5829,15188,6042,15088,6094,14966,6001,14826,5796,14678,5543,14527,5287,14377,4985,14133,4586,13869,4257,15360,1563,15360,1642,15358,2076,15354,2636,15341,3350,15317,4019,15273,4429,15203,4732,15105,4911,14981,4932,14836,4818,14679,4621,14517,4386,14359,4156,14083,3795,13808,3437,15360,122,15360,137,15358,285,15355,636,15344,1274,15322,2177,15281,2765,15215,3223,15120,3451,14995,3569,14846,3567,14681,3466,14511,3305,14344,3121,14037,2800,13753,2467,15360,0,15360,1,15359,21,15355,89,15346,253,15325,479,15287,796,15225,1148,15133,1492,15008,1749,14856,1882,14685,1886,14506,1783,14324,1608,13996,1398,13702,1183]),Zh=null;function Qh(){return Zh===null&&(Zh=new Fa(Xh,16,16,re,D),Zh.name=`DFG_LUT`,Zh.minFilter=h,Zh.magFilter=h,Zh.wrapS=c,Zh.wrapT=c,Zh.generateMipmaps=!1,Zh.needsUpdate=!0),Zh}var $h=class{constructor(e={}){let{canvas:t=Mt(),context:n=null,depth:r=!0,stencil:i=!1,alpha:a=!1,antialias:o=!1,premultipliedAlpha:s=!0,preserveDrawingBuffer:c=!1,powerPreference:l=`default`,failIfMajorPerformanceCaveat:u=!1,reversedDepthBuffer:d=!1,outputBufferType:f=b}=e;this.isWebGLRenderer=!0;let p;if(n!==null){if(typeof WebGLRenderingContext<`u`&&n instanceof WebGLRenderingContext)throw Error(`THREE.WebGLRenderer: WebGL 1 is not supported since r163.`);p=n.getContextAttributes().alpha}else p=a;let m=f,h=new Set([oe,ie,ne]),g=new Set([b,T,C,A,O,k]),_=new Uint32Array(4),y=new Int32Array(4),x=new q,S=null,w=null,E=[],j=[],M=null;this.domElement=t,this.debug={checkShaderErrors:!0,diagnostics:{keywords:!1},onShaderError:null},this.autoClear=!0,this.autoClearColor=!0,this.autoClearDepth=!0,this.autoClearStencil=!0,this.sortObjects=!0,this.clippingPlanes=[],this.localClippingEnabled=!1,this.toneMapping=0,this.toneMappingExposure=1,this.transmissionResolutionScale=1;let N=this,P=!1,F=null,I=null,ee=null,te=null;this._outputColorSpace=$e;let re=0,ae=0,L=null,se=-1,ce=null,le=new In,ue=new In,de=null,fe=new Y(0),pe=0,me=t.width,he=t.height,R=1,ge=null,_e=null,ve=new In(0,0,me,he),ye=new In(0,0,me,he),be=!1,xe=new Za,Se=!1,Ce=!1,we=new J,Te=new q,Ee=new In,De={background:null,fog:null,environment:null,overrideMaterial:null,isScene:!0},Oe=!1;function ke(){return L===null?R:1}let z=n;function Ae(e,n){return t.getContext(e,n)}let je,B,V,Me,H,Ne,Pe,Fe,Ie,Le,Re,ze,Be,Ve,He,Ue,We,Ge,Ke,qe,Je,Ye,Xe;try{let e={alpha:!0,depth:r,stencil:i,antialias:o,premultipliedAlpha:s,preserveDrawingBuffer:c,powerPreference:l,failIfMajorPerformanceCaveat:u};if(`setAttribute`in t&&t.setAttribute(`data-engine`,`three.js r186`),t.addEventListener(`webglcontextlost`,et,!1),t.addEventListener(`webglcontextrestored`,tt,!1),t.addEventListener(`webglcontextcreationerror`,nt,!1),z===null){let t=`webgl2`;if(z=Ae(t,e),z===null)throw Ae(t)?Error(`THREE.WebGLRenderer: Error creating WebGL context with your selected attributes.`):Error(`THREE.WebGLRenderer: Error creating WebGL context.`)}Ze()}catch(e){throw t.removeEventListener(`webglcontextlost`,et,!1),t.removeEventListener(`webglcontextrestored`,tt,!1),t.removeEventListener(`webglcontextcreationerror`,nt,!1),W(`WebGLRenderer: `+e.message),e}function Ze(){je=new _p(z),je.init(),Je=new Vh(z,je),B=new Kf(z,je,e,Je),V=new zh(z,je),B.reversedDepthBuffer&&d&&V.buffers.depth.setReversed(!0),I=z.createFramebuffer(),ee=z.createFramebuffer(),te=z.createFramebuffer(),Me=new bp(z),H=new yh,Ne=new Bh(z,je,V,H,B,Je,Me),Pe=new gp(N),Fe=new Lf(z),Ye=new Wf(z,Fe),Ie=new vp(z,Fe,Me,Ye),Le=new Sp(z,Ie,Fe,Ye,Me),Ge=new xp(z,B,Ne),He=new qf(H),Re=new vh(N,Pe,je,B,Ye,He),ze=new Jh(N,H),Be=new Ch,Ve=new Ah(je),We=new Uf(N,Pe,V,Le,p,s),Ue=new Rh(N,Le,B),Xe=new Yh(z,Me,B,V),Ke=new Gf(z,je,Me),qe=new yp(z,je,Me),Me.programs=Re.programs,N.capabilities=B,N.extensions=je,N.properties=H,N.renderLists=Be,N.shadowMap=Ue,N.state=V,N.info=Me}m!==1009&&(M=new wp(m,t.width,t.height,o,r,i));let Qe=new Gh(N,z);this.xr=Qe,this.getContext=function(){return z},this.getContextAttributes=function(){return z.getContextAttributes()},this.forceContextLoss=function(){let e=je.get(`WEBGL_lose_context`);e&&e.loseContext()},this.forceContextRestore=function(){let e=je.get(`WEBGL_lose_context`);e&&e.restoreContext()},this.getPixelRatio=function(){return R},this.setPixelRatio=function(e){e!==void 0&&(R=e,this.setSize(me,he,!1))},this.getSize=function(e){return e.set(me,he)},this.setSize=function(e,n,r=!0){Qe.isPresenting?U(`WebGLRenderer: Can't change size while VR device is presenting.`):(me=e,he=n,t.width=Math.floor(e*R),t.height=Math.floor(n*R),r===!0&&(t.style.width=e+`px`,t.style.height=n+`px`),M!==null&&M.setSize(t.width,t.height),this.setViewport(0,0,e,n))},this.getDrawingBufferSize=function(e){return e.set(me*R,he*R).floor()},this.setDrawingBufferSize=function(e,n,r){me=e,he=n,R=r,t.width=Math.floor(e*r),t.height=Math.floor(n*r),this.setViewport(0,0,e,n)},this.setEffects=function(e){if(m===1009)W(`WebGLRenderer: setEffects() requires outputBufferType set to HalfFloatType or FloatType.`);else{if(e){for(let t=0;t<e.length;t++)if(e[t].isOutputPass===!0){U(`WebGLRenderer: OutputPass is not needed in setEffects(). Tone mapping and color space conversion are applied automatically.`);break}}M.setEffects(e||[])}},this.getCurrentViewport=function(e){return e.copy(le)},this.getViewport=function(e){return e.copy(ve)},this.setViewport=function(e,t,n,r){e.isVector4?ve.set(e.x,e.y,e.z,e.w):ve.set(e,t,n,r),V.viewport(le.copy(ve).multiplyScalar(R).round())},this.getScissor=function(e){return e.copy(ye)},this.setScissor=function(e,t,n,r){e.isVector4?ye.set(e.x,e.y,e.z,e.w):ye.set(e,t,n,r),V.scissor(ue.copy(ye).multiplyScalar(R).round())},this.getScissorTest=function(){return be},this.setScissorTest=function(e){V.setScissorTest(be=e)},this.setOpaqueSort=function(e){ge=e},this.setTransparentSort=function(e){_e=e},this.getClearColor=function(e){return e.copy(We.getClearColor())},this.setClearColor=function(){We.setClearColor(...arguments)},this.getClearAlpha=function(){return We.getClearAlpha()},this.setClearAlpha=function(){We.setClearAlpha(...arguments)},this.clear=function(e=!0,t=!0,n=!0){let r=0;if(e){let e=!1;if(L!==null){let t=L.texture.format;e=h.has(t)}if(e){let e=L.texture.type,t=g.has(e),n=We.getClearColor(),r=We.getClearAlpha(),i=n.r,a=n.g,o=n.b;t?(_[0]=i,_[1]=a,_[2]=o,_[3]=r,z.clearBufferuiv(z.COLOR,0,_)):(y[0]=i,y[1]=a,y[2]=o,y[3]=r,z.clearBufferiv(z.COLOR,0,y))}else r|=z.COLOR_BUFFER_BIT}t&&(r|=z.DEPTH_BUFFER_BIT,this.state.buffers.depth.setMask(!0)),n&&(r|=z.STENCIL_BUFFER_BIT,this.state.buffers.stencil.setMask(4294967295)),r!==0&&z.clear(r)},this.clearColor=function(){this.clear(!0,!1,!1)},this.clearDepth=function(){this.clear(!1,!0,!1)},this.clearStencil=function(){this.clear(!1,!1,!0)},this.setNodesHandler=function(e){e.setRenderer(this),F=e},this.dispose=function(){t.removeEventListener(`webglcontextlost`,et,!1),t.removeEventListener(`webglcontextrestored`,tt,!1),t.removeEventListener(`webglcontextcreationerror`,nt,!1),We.dispose(),Be.dispose(),Ve.dispose(),H.dispose(),Pe.dispose(),Le.dispose(),Ye.dispose(),Xe.dispose(),Re.dispose(),Qe.dispose(),Qe.removeEventListener(`sessionstart`,lt),Qe.removeEventListener(`sessionend`,ut),dt.stop()};function et(e){e.preventDefault(),Lt(`WebGLRenderer: Context Lost.`),P=!0}function tt(){Lt(`WebGLRenderer: Context Restored.`),P=!1;let e=Me.autoReset,t=Ue.enabled,n=Ue.autoUpdate,r=Ue.needsUpdate,i=Ue.type;Ze(),Me.autoReset=e,Ue.enabled=t,Ue.autoUpdate=n,Ue.needsUpdate=r,Ue.type=i}function nt(e){W(`WebGLRenderer: A WebGL context could not be created. Reason: `,e.statusMessage)}function rt(e){let t=e.target;t.removeEventListener(`dispose`,rt),it(t)}function it(e){at(e),H.remove(e)}function at(e){let t=H.get(e).programs;t!==void 0&&(t.forEach(function(e){Re.releaseProgram(e)}),e.isShaderMaterial&&Re.releaseShaderCache(e))}this.renderBufferDirect=function(e,t,n,r,i,a){t===null&&(t=De);let o=i.isMesh&&i.matrixWorld.determinantAffine()<0,s=St(e,t,n,r,i);V.setMaterial(r,o);let c=n.index,l=1;if(r.wireframe===!0){if(c=Ie.getWireframeAttribute(n),c===void 0)return;l=2}let u=n.drawRange,d=n.attributes.position,f=u.start*l,p=(u.start+u.count)*l;a!==null&&(f=Math.max(f,a.start*l),p=Math.min(p,(a.start+a.count)*l)),c===null?d!=null&&(f=Math.max(f,0),p=Math.min(p,d.count)):(f=Math.max(f,0),p=Math.min(p,c.count));let m=p-f;if(m<0||m===1/0)return;Ye.setup(i,r,s,n,c);let h,g=Ke;if(c!==null&&(h=Fe.get(c),g=qe,g.setIndex(h)),i.isMesh)r.wireframe===!0?(V.setLineWidth(r.wireframeLinewidth*ke()),g.setMode(z.LINES)):g.setMode(z.TRIANGLES);else if(i.isLine){let e=r.linewidth;e===void 0&&(e=1),V.setLineWidth(e*ke()),i.isLineSegments?g.setMode(z.LINES):i.isLineLoop?g.setMode(z.LINE_LOOP):g.setMode(z.LINE_STRIP)}else i.isPoints?g.setMode(z.POINTS):i.isSprite&&g.setMode(z.TRIANGLES);if(i.isBatchedMesh){if(je.get(`WEBGL_multi_draw`))g.renderMultiDraw(i._multiDrawStarts,i._multiDrawCounts,i._multiDrawCount);else{let e=i._multiDrawStarts,t=i._multiDrawCounts,n=i._multiDrawCount,a=c?Fe.get(c).bytesPerElement:1,o=H.get(r).currentProgram.getUniforms();for(let r=0;r<n;r++)o.setValue(z,`_gl_DrawID`,r),g.render(e[r]/a,t[r])}}else if(i.isInstancedMesh)g.renderInstances(f,m,i.count);else if(n.isInstancedBufferGeometry){let e=n._maxInstanceCount===void 0?1/0:n._maxInstanceCount,t=Math.min(n.instanceCount,e);g.renderInstances(f,m,t)}else g.render(f,m)};function ot(e,t,n,r){F!==null&&e.isNodeMaterial&&F.setObject(r,e),Se===!0&&He.setState(e,n,!1),e.transparent===!0&&e.side===2&&e.forceSinglePass===!1?(e.side=1,e.needsUpdate=!0,_t(e,t,r),e.side=0,e.needsUpdate=!0,_t(e,t,r),e.side=2):_t(e,t,r)}this.compile=function(e,t,n=null){n===null&&(n=e),F!==null&&F.renderStart(e,t,n),w=Ve.get(n),w.init(t),j.push(w),n.traverseVisible(function(e){e.isLight&&e.layers.test(t.layers)&&(w.pushLight(e),e.castShadow&&w.pushShadow(e))}),e!==n&&e.traverseVisible(function(e){e.isLight&&e.layers.test(t.layers)&&(w.pushLight(e),e.castShadow&&w.pushShadow(e))}),w.setupLights(),F!==null&&F.updateLights(w.state.lightsArray),Ce=this.localClippingEnabled,Se=He.init(this.clippingPlanes,Ce),Se===!0&&He.setGlobalState(this.clippingPlanes,t),F!==null&&Ue.render(w.state.shadowsArray,n,t);let r=new Set;return e.traverse(function(e){if(!(e.isMesh||e.isPoints||e.isLine||e.isSprite))return;let i=e.material;if(i){if(Array.isArray(i))for(let a=0;a<i.length;a++){let o=i[a];ot(o,n,t,e),r.add(o)}else ot(i,n,t,e),r.add(i)}}),w=j.pop(),F!==null&&F.renderEnd(),r},this.compileAsync=function(e,t,n=null){let r=this.compile(e,t,n);return new Promise(t=>{function n(){r.forEach(function(e){let t=H.get(e).currentProgram;(t===void 0||t.isReady())&&r.delete(e)}),r.size===0?t(e):setTimeout(n,10)}je.get(`KHR_parallel_shader_compile`)===null?setTimeout(n,10):n()})};let st=null;function ct(e){st&&st(e)}function lt(){dt.stop()}function ut(){dt.start()}let dt=new If;dt.setAnimationLoop(ct),typeof self<`u`&&dt.setContext(self),this.setAnimationLoop=function(e){st=e,Qe.setAnimationLoop(e),e===null?dt.stop():dt.start()},Qe.addEventListener(`sessionstart`,lt),Qe.addEventListener(`sessionend`,ut),this.render=function(e,t){if(t!==void 0&&t.isCamera!==!0){W(`WebGLRenderer.render: camera is not an instance of THREE.Camera.`);return}if(P===!0)return;F!==null&&F.renderStart(e,t);let n=Qe.enabled===!0&&Qe.isPresenting===!0,r=M!==null&&(L===null||n)&&M.begin(N,L);if(e.matrixWorldAutoUpdate===!0&&e.updateMatrixWorld(),t.parent===null&&t.matrixWorldAutoUpdate===!0&&t.updateMatrixWorld(),Qe.enabled===!0&&Qe.isPresenting===!0&&(M===null||M.isCompositing()===!1)&&(Qe.cameraAutoUpdate===!0&&Qe.updateCamera(t),t=Qe.getCamera()),e.isScene===!0&&e.onBeforeRender(N,e,t,L),w=Ve.get(e,j.length),w.init(t),w.state.textureUnits=Ne.getTextureUnits(),j.push(w),we.multiplyMatrices(t.projectionMatrix,t.matrixWorldInverse),xe.setFromProjectionMatrix(we,bt,t.reversedDepth),Ce=this.localClippingEnabled,Se=He.init(this.clippingPlanes,Ce),S=Be.get(e,E.length),S.init(),E.push(S),Qe.enabled===!0&&Qe.isPresenting===!0){let e=N.xr.getDepthSensingMesh();e!==null&&ft(e,t,-1/0,N.sortObjects)}ft(e,t,0,N.sortObjects),S.finish(),F!==null&&F.updateLights(w.state.lightsArray),N.sortObjects===!0&&S.sort(ge,_e),Oe=Qe.enabled===!1||Qe.isPresenting===!1||Qe.hasDepthSensing()===!1,Oe&&We.addToRenderList(S,e),this.info.render.frame++,this.info.autoReset===!0&&this.info.reset(),Se===!0&&He.beginShadows();let i=w.state.shadowsArray;if(Ue.render(i,e,t),Se===!0&&He.endShadows(),(r&&M.hasRenderPass())===!1){let n=S.opaque,r=S.transmissive;if(w.setupLights(),t.isArrayCamera){let i=t.cameras;if(r.length>0)for(let t=0,a=i.length;t<a;t++){let a=i[t];mt(n,r,e,a)}Oe&&We.render(e);for(let t=0,n=i.length;t<n;t++){let n=i[t];pt(S,e,n,n.viewport)}}else r.length>0&&mt(n,r,e,t),Oe&&We.render(e),pt(S,e,t)}L!==null&&ae===0&&(Ne.updateMultisampleRenderTarget(L),Ne.updateRenderTargetMipmap(L)),r&&M.end(N),e.isScene===!0&&e.onAfterRender(N,e,t),Ye.resetDefaultState(),se=-1,ce=null,j.pop(),j.length>0?(w=j[j.length-1],Ne.setTextureUnits(w.state.textureUnits),Se===!0&&He.setGlobalState(N.clippingPlanes,w.state.camera)):w=null,E.pop(),S=E.length>0?E[E.length-1]:null,F!==null&&F.renderEnd()};function ft(e,t,n,r){if(e.visible===!1)return;if(e.layers.test(t.layers)){if(e.isGroup)n=e.renderOrder;else if(e.isLOD)e.autoUpdate===!0&&e.update(t);else if(e.isLightProbeGrid)w.pushLightProbeGrid(e);else if(e.isLight)w.pushLight(e),e.castShadow&&w.pushShadow(e);else if(e.isSprite){if(!e.frustumCulled||e.intersectsFrustum(xe)){r&&Ee.setFromMatrixPosition(e.matrixWorld).applyMatrix4(we);let i=Le.update(e),a=e.material;a.visible&&S.push(e,i,a,n,Ee.z,null,t)}}else if((e.isMesh||e.isLine||e.isPoints)&&(!e.frustumCulled||e.intersectsFrustum(xe))){let i=Le.update(e),a=e.material;if(r&&(e.boundingSphere===void 0?(i.boundingSphere===null&&i.computeBoundingSphere(),Ee.copy(i.boundingSphere.center)):(e.boundingSphere===null&&e.computeBoundingSphere(),Ee.copy(e.boundingSphere.center)),Ee.applyMatrix4(e.matrixWorld).applyMatrix4(we)),Array.isArray(a)){let r=i.groups;for(let o=0,s=r.length;o<s;o++){let s=r[o],c=a[s.materialIndex];c&&c.visible&&S.push(e,i,c,n,Ee.z,s,t)}}else a.visible&&S.push(e,i,a,n,Ee.z,null,t)}}let i=e.children;for(let e=0,a=i.length;e<a;e++)ft(i[e],t,n,r)}function pt(e,t,n,r){let{opaque:i,transmissive:a,transparent:o}=e;w.setupLightsView(n),Se===!0&&He.setGlobalState(N.clippingPlanes,n),r&&V.viewport(le.copy(r)),i.length>0&&ht(i,t,n),a.length>0&&ht(a,t,n),o.length>0&&ht(o,t,n),V.buffers.depth.setTest(!0),V.buffers.depth.setMask(!0),V.buffers.color.setMask(!0),V.setPolygonOffset(!1)}function mt(e,t,n,r){if((n.isScene===!0?n.overrideMaterial:null)!==null)return;if(w.state.transmissionRenderTarget[r.id]===void 0){let e=je.has(`EXT_color_buffer_half_float`)||je.has(`EXT_color_buffer_float`);w.state.transmissionRenderTarget[r.id]=new Rn(1,1,{generateMipmaps:!0,type:e?D:b,minFilter:v,samples:Math.max(4,B.samples),stencilBuffer:i,resolveDepthBuffer:!1,resolveStencilBuffer:!1,storeMultisampledDepthBuffer:!1,storeMultisampledStencilBuffer:!1,colorSpace:wn.workingColorSpace})}let a=w.state.transmissionRenderTarget[r.id],o=r.viewport||le;a.setSize(o.z*N.transmissionResolutionScale,o.w*N.transmissionResolutionScale);let s=N.getRenderTarget(),c=N.getActiveCubeFace(),l=N.getActiveMipmapLevel();N.setRenderTarget(a),N.getClearColor(fe),pe=N.getClearAlpha(),pe<1&&N.setClearColor(16777215,.5),N.clear(),Oe&&We.render(n);let u=N.toneMapping;N.toneMapping=0;let d=r.viewport;if(r.viewport!==void 0&&(r.viewport=void 0),w.setupLightsView(r),Se===!0&&He.setGlobalState(N.clippingPlanes,r),ht(e,n,r),Ne.updateMultisampleRenderTarget(a),Ne.updateRenderTargetMipmap(a),je.has(`WEBGL_multisampled_render_to_texture`)===!1){let e=!1;for(let i=0,a=t.length;i<a;i++){let{object:a,geometry:o,material:s,group:c}=t[i];if(s.side===2&&a.layers.test(r.layers)){let t=s.side;s.side=1,s.needsUpdate=!0,gt(a,n,r,o,s,c),s.side=t,s.needsUpdate=!0,e=!0}}e===!0&&(Ne.updateMultisampleRenderTarget(a),Ne.updateRenderTargetMipmap(a))}N.setRenderTarget(s,c,l),N.setClearColor(fe,pe),d!==void 0&&(r.viewport=d),N.toneMapping=u}function ht(e,t,n){let r=t.isScene===!0?t.overrideMaterial:null;for(let i=0,a=e.length;i<a;i++){let a=e[i],{object:o,geometry:s,group:c}=a,l=a.material;l.allowOverride===!0&&r!==null&&(l=r),o.layers.test(n.layers)&&gt(o,t,n,s,l,c)}}function gt(e,t,n,r,i,a){F!==null&&i.isNodeMaterial&&F.setObject(e,i),e.onBeforeRender(N,t,n,r,i,a),e.modelViewMatrix.multiplyMatrices(n.matrixWorldInverse,e.matrixWorld),e.normalMatrix.getNormalMatrix(e.modelViewMatrix),i.onBeforeRender(N,t,n,r,e,a),i.transparent===!0&&i.side===2&&i.forceSinglePass===!1?(i.side=1,i.needsUpdate=!0,N.renderBufferDirect(n,t,r,i,e,a),i.side=0,i.needsUpdate=!0,N.renderBufferDirect(n,t,r,i,e,a),i.side=2):N.renderBufferDirect(n,t,r,i,e,a),e.onAfterRender(N,t,n,r,i,a)}function _t(e,t,n){t.isScene!==!0&&(t=De);let r=H.get(e),i=w.state.lights,a=w.state.shadowsArray,o=i.state.version,s=Re.getParameters(e,i.state,a,t,n,w.state.lightProbeGridArray),c=Re.getProgramCacheKey(s),l=r.programs;r.environment=e.isMeshStandardMaterial||e.isMeshLambertMaterial||e.isMeshPhongMaterial?t.environment:null,r.fog=t.fog;let u=e.isMeshStandardMaterial||e.isMeshLambertMaterial&&!e.envMap||e.isMeshPhongMaterial&&!e.envMap;r.envMap=Pe.get(e.envMap||r.environment,u),r.envMapRotation=r.environment!==null&&e.envMap===null?t.environmentRotation:e.envMapRotation,l===void 0&&(e.addEventListener(`dispose`,rt),l=new Map,r.programs=l);let d=l.get(c);if(d!==void 0){if(r.currentProgram===d&&r.lightsStateVersion===o)return yt(e,s),d}else s.uniforms=Re.getUniforms(e),F!==null&&e.isNodeMaterial&&F.build(e,n,s),e.onBeforeCompile(s,N),d=Re.acquireProgram(s,c),l.set(c,d),r.uniforms=s.uniforms;let f=r.uniforms;return(!e.isShaderMaterial&&!e.isRawShaderMaterial||e.clipping===!0)&&(f.clippingPlanes=He.uniform),yt(e,s),r.needsLights=wt(e),r.lightsStateVersion=o,r.needsLights&&(f.ambientLightColor.value=i.state.ambient,f.lightProbe.value=i.state.probe,f.sunLights.value=i.state.sun,f.sunLightShadows.value=i.state.sunShadow,f.directionalLights.value=i.state.directional,f.directionalLightShadows.value=i.state.directionalShadow,f.spotLights.value=i.state.spot,f.spotLightShadows.value=i.state.spotShadow,f.rectAreaLights.value=i.state.rectArea,f.ltc_1.value=i.state.rectAreaLTC1,f.ltc_2.value=i.state.rectAreaLTC2,f.pointLights.value=i.state.point,f.pointLightShadows.value=i.state.pointShadow,f.hemisphereLights.value=i.state.hemi,f.sunShadowMatrix.value=i.state.sunShadowMatrix,f.sunShadowCascade.value=i.state.sunShadowCascade,f.directionalShadowMatrix.value=i.state.directionalShadowMatrix,f.spotLightMatrix.value=i.state.spotLightMatrix,f.spotLightMap.value=i.state.spotLightMap,f.pointShadowMatrix.value=i.state.pointShadowMatrix),r.lightProbeGrid=w.state.lightProbeGridArray.length>0,r.currentProgram=d,r.uniformsList=null,d}function vt(e){if(e.uniformsList===null){let t=e.currentProgram.getUniforms();e.uniformsList=jm.seqWithValue(t.seq,e.uniforms)}return e.uniformsList}function yt(e,t){let n=H.get(e);n.outputColorSpace=t.outputColorSpace,n.batching=t.batching,n.batchingColor=t.batchingColor,n.instancing=t.instancing,n.instancingColor=t.instancingColor,n.instancingMorph=t.instancingMorph,n.skinning=t.skinning,n.morphTargets=t.morphTargets,n.morphNormals=t.morphNormals,n.morphColors=t.morphColors,n.morphTargetsCount=t.morphTargetsCount,n.numClippingPlanes=t.numClippingPlanes,n.numIntersection=t.numClipIntersection,n.vertexAlphas=t.vertexAlphas,n.vertexTangents=t.vertexTangents,n.toneMapping=t.toneMapping}function xt(e,t){if(e.length===0)return null;if(e.length===1)return e[0].texture===null?null:e[0];x.setFromMatrixPosition(t.matrixWorld);for(let t=0,n=e.length;t<n;t++){let n=e[t];if(n.texture!==null&&n.boundingBox.containsPoint(x))return n}return null}function St(e,t,n,r,i){t.isScene!==!0&&(t=De),Ne.resetTextureUnits();let a=t.fog,o=r.isMeshStandardMaterial||r.isMeshLambertMaterial||r.isMeshPhongMaterial?t.environment:null,s=L===null?N.outputColorSpace:L.isXRRenderTarget===!0?L.texture.colorSpace:wn.workingColorSpace,c=r.isMeshStandardMaterial||r.isMeshLambertMaterial&&!r.envMap||r.isMeshPhongMaterial&&!r.envMap,l=Pe.get(r.envMap||o,c),u=r.vertexColors===!0&&!!n.attributes.color&&n.attributes.color.itemSize===4,d=!!n.attributes.tangent&&(!!r.normalMap||r.anisotropy>0),f=!!n.morphAttributes.position,p=!!n.morphAttributes.normal,m=!!n.morphAttributes.color,h=0;r.toneMapped&&(L===null||L.isXRRenderTarget===!0)&&(h=N.toneMapping);let g=n.morphAttributes.position||n.morphAttributes.normal||n.morphAttributes.color,_=g===void 0?0:g.length,v=H.get(r),y=w.state.lights;if(Se===!0&&(Ce===!0||e!==ce)){let t=e===ce&&r.id===se;He.setState(r,e,t)}let b=!1;r.version===v.__version?v.needsLights&&v.lightsStateVersion!==y.state.version?b=!0:v.outputColorSpace===s?i.isBatchedMesh&&v.batching===!1||!i.isBatchedMesh&&v.batching===!0||i.isBatchedMesh&&v.batchingColor===!0&&i._colorsTexture===null||i.isBatchedMesh&&v.batchingColor===!1&&i._colorsTexture!==null||i.isInstancedMesh&&v.instancing===!1||!i.isInstancedMesh&&v.instancing===!0||i.isSkinnedMesh&&v.skinning===!1||!i.isSkinnedMesh&&v.skinning===!0||i.isInstancedMesh&&v.instancingColor===!0&&i.instanceColor===null||i.isInstancedMesh&&v.instancingColor===!1&&i.instanceColor!==null||i.isInstancedMesh&&v.instancingMorph===!0&&i.morphTexture===null||i.isInstancedMesh&&v.instancingMorph===!1&&i.morphTexture!==null?b=!0:v.envMap===l?r.fog===!0&&v.fog!==a||v.numClippingPlanes!==void 0&&(v.numClippingPlanes!==He.numPlanes||v.numIntersection!==He.numIntersection)?b=!0:v.vertexAlphas===u&&v.vertexTangents===d&&v.morphTargets===f&&v.morphNormals===p&&v.morphColors===m&&v.toneMapping===h&&v.morphTargetsCount===_?!!v.lightProbeGrid!=w.state.lightProbeGridArray.length>0&&(b=!0):b=!0:b=!0:b=!0:(b=!0,v.__version=r.version);let x=v.currentProgram;b===!0&&(x=_t(r,t,i),F&&r.isNodeMaterial&&F.onUpdateProgram(r,x,v));let S=!1,C=!1,T=!1,E=x.getUniforms(),D=v.uniforms;if(V.useProgram(x.program)&&(S=!0,C=!0,T=!0),r.id!==se&&(se=r.id,C=!0),v.needsLights){let e=xt(w.state.lightProbeGridArray,i);v.lightProbeGrid!==e&&(v.lightProbeGrid=e,C=!0)}if(S||ce!==e){V.buffers.depth.getReversed()&&e.reversedDepth!==!0&&(e._reversedDepth=!0,e.updateProjectionMatrix()),E.setValue(z,`projectionMatrix`,e.projectionMatrix),E.setValue(z,`viewMatrix`,e.matrixWorldInverse);let t=E.map.cameraPosition;t!==void 0&&t.setValue(z,Te.setFromMatrixPosition(e.matrixWorld)),B.logarithmicDepthBuffer&&E.setValue(z,`logDepthBufFC`,2/(Math.log(e.far+1)/Math.LN2)),(r.isMeshPhongMaterial||r.isMeshToonMaterial||r.isMeshLambertMaterial||r.isMeshBasicMaterial||r.isMeshStandardMaterial||r.isShaderMaterial)&&E.setValue(z,`isOrthographic`,e.isOrthographicCamera===!0),ce!==e&&(ce=e,C=!0,T=!0)}if(v.needsLights&&(y.state.sunShadowMap.length>0&&E.setValue(z,`sunShadowMap`,y.state.sunShadowMap,Ne),y.state.directionalShadowMap.length>0&&E.setValue(z,`directionalShadowMap`,y.state.directionalShadowMap,Ne),y.state.spotShadowMap.length>0&&E.setValue(z,`spotShadowMap`,y.state.spotShadowMap,Ne),y.state.pointShadowMap.length>0&&E.setValue(z,`pointShadowMap`,y.state.pointShadowMap,Ne)),i.isSkinnedMesh){E.setOptional(z,i,`bindMatrix`),E.setOptional(z,i,`bindMatrixInverse`);let e=i.skeleton;e&&(e.boneTexture===null&&e.computeBoneTexture(),E.setValue(z,`boneTexture`,e.boneTexture,Ne))}i.isBatchedMesh&&(E.setOptional(z,i,`batchingTexture`),E.setValue(z,`batchingTexture`,i._matricesTexture,Ne),E.setOptional(z,i,`batchingIdTexture`),E.setValue(z,`batchingIdTexture`,i._indirectTexture,Ne),E.setOptional(z,i,`batchingColorTexture`),i._colorsTexture!==null&&E.setValue(z,`batchingColorTexture`,i._colorsTexture,Ne));let O=n.morphAttributes;if((O.position!==void 0||O.normal!==void 0||O.color!==void 0)&&Ge.update(i,n,x),(C||v.receiveShadow!==i.receiveShadow)&&(v.receiveShadow=i.receiveShadow,E.setValue(z,`receiveShadow`,i.receiveShadow)),(r.isMeshStandardMaterial||r.isMeshLambertMaterial||r.isMeshPhongMaterial)&&r.envMap===null&&t.environment!==null&&(D.envMapIntensity.value=t.environmentIntensity),D.dfgLUT!==void 0&&(D.dfgLUT.value=Qh()),C){if(E.setValue(z,`toneMappingExposure`,N.toneMappingExposure),v.needsLights&&Ct(D,T),a&&r.fog===!0&&ze.refreshFogUniforms(D,a),ze.refreshMaterialUniforms(D,r,R,he,w.state.transmissionRenderTarget[e.id]),v.needsLights&&v.lightProbeGrid){let e=v.lightProbeGrid;D.probesSH.value=e.texture,D.probesMin.value.copy(e.boundingBox.min),D.probesMax.value.copy(e.boundingBox.max),D.probesResolution.value.copy(e.resolution)}jm.upload(z,vt(v),D,Ne)}if(r.isShaderMaterial&&r.uniformsNeedUpdate===!0&&(jm.upload(z,vt(v),D,Ne),r.uniformsNeedUpdate=!1),r.isSpriteMaterial&&E.setValue(z,`center`,i.center),E.setValue(z,`modelViewMatrix`,i.modelViewMatrix),E.setValue(z,`normalMatrix`,i.normalMatrix),E.setValue(z,`modelMatrix`,i.matrixWorld),r.uniformsGroups!==void 0){let e=r.uniformsGroups;for(let t=0,n=e.length;t<n;t++){let n=e[t];Xe.update(n,x),Xe.bind(n,x)}}return x}function Ct(e,t){e.ambientLightColor.needsUpdate=t,e.lightProbe.needsUpdate=t,e.sunLights.needsUpdate=t,e.sunLightShadows.needsUpdate=t,e.directionalLights.needsUpdate=t,e.directionalLightShadows.needsUpdate=t,e.pointLights.needsUpdate=t,e.pointLightShadows.needsUpdate=t,e.spotLights.needsUpdate=t,e.spotLightShadows.needsUpdate=t,e.rectAreaLights.needsUpdate=t,e.hemisphereLights.needsUpdate=t}function wt(e){return e.isMeshLambertMaterial||e.isMeshToonMaterial||e.isMeshPhongMaterial||e.isMeshStandardMaterial||e.isShadowMaterial||e.isShaderMaterial&&e.lights===!0}this.getActiveCubeFace=function(){return re},this.getActiveMipmapLevel=function(){return ae},this.getRenderTarget=function(){return L},this.setRenderTargetTextures=function(e,t,n){let r=H.get(e);r.__autoAllocateDepthBuffer=e.resolveDepthBuffer===!1,r.__autoAllocateDepthBuffer===!1&&(r.__useRenderToTexture=!1),H.get(e.texture).__webglTexture=t,H.get(e.depthTexture).__webglTexture=r.__autoAllocateDepthBuffer?void 0:n,r.__hasExternalTextures=!0},this.setRenderTargetFramebuffer=function(e,t){let n=H.get(e);n.__webglFramebuffer=t,n.__useDefaultFramebuffer=t===void 0},this.setRenderTarget=function(e,t=0,n=0){L=e,re=t,ae=n;let r=null,i=!1,a=!1;if(e){let o=H.get(e);if(o.__useDefaultFramebuffer!==void 0){V.bindFramebuffer(z.FRAMEBUFFER,o.__webglFramebuffer),le.copy(e.viewport),ue.copy(e.scissor),de=e.scissorTest,V.viewport(le),V.scissor(ue),V.setScissorTest(de),se=-1;return}if(o.__webglFramebuffer===void 0)Ne.setupRenderTarget(e);else if(o.__hasExternalTextures)Ne.rebindTextures(e,H.get(e.texture).__webglTexture,H.get(e.depthTexture).__webglTexture);else if(e.depthBuffer){let t=e.depthTexture;if(o.__boundDepthTexture!==t){if(t!==null&&H.has(t)&&(e.width!==t.image.width||e.height!==t.image.height))throw Error(`THREE.WebGLRenderer: Attached DepthTexture is initialized to the incorrect size.`);Ne.setupDepthRenderbuffer(e)}}let s=e.texture;(s.isData3DTexture||s.isDataArrayTexture||s.isCompressedArrayTexture)&&(a=!0);let c=H.get(e).__webglFramebuffer;e.isWebGLCubeRenderTarget?(r=Array.isArray(c[t])?c[t][n]:c[t],i=!0):r=e.samples>0&&Ne.useMultisampledRTT(e)===!1?H.get(e).__webglMultisampledFramebuffer:Array.isArray(c)?c[n]:c,le.copy(e.viewport),ue.copy(e.scissor),de=e.scissorTest}else le.copy(ve).multiplyScalar(R).floor(),ue.copy(ye).multiplyScalar(R).floor(),de=be;if(n!==0&&(r=I),V.bindFramebuffer(z.FRAMEBUFFER,r)&&V.drawBuffers(e,r),V.viewport(le),V.scissor(ue),V.setScissorTest(de),i){let r=H.get(e.texture);z.framebufferTexture2D(z.FRAMEBUFFER,z.COLOR_ATTACHMENT0,z.TEXTURE_CUBE_MAP_POSITIVE_X+t,r.__webglTexture,n)}else if(a){let r=t;for(let t=0;t<e.textures.length;t++){let i=H.get(e.textures[t]);z.framebufferTextureLayer(z.FRAMEBUFFER,z.COLOR_ATTACHMENT0+t,i.__webglTexture,n,r)}}else if(e!==null&&n!==0){let t=H.get(e.texture);z.framebufferTexture2D(z.FRAMEBUFFER,z.COLOR_ATTACHMENT0,z.TEXTURE_2D,t.__webglTexture,n)}se=-1};function Tt(e){let t=H.get(e);return(t.__readFormat!==e.format||t.__readType!==e.type)&&(t.__readFormat=e.format,t.__readType=e.type,t.__formatReadable=B.textureFormatReadable(e.format),t.__typeReadable=B.textureTypeReadable(e.type)),t}this.readRenderTargetPixels=function(e,t,n,r,i,a,o,s=0){if(!(e&&e.isWebGLRenderTarget)){W(`WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.`);return}let c=H.get(e).__webglFramebuffer;if(e.isWebGLCubeRenderTarget&&o!==void 0&&(c=c[o]),c){V.bindFramebuffer(z.FRAMEBUFFER,c);try{let o=e.textures[s],c=o.format,l=o.type;e.textures.length>1&&z.readBuffer(z.COLOR_ATTACHMENT0+s);let u=Tt(o);if(u.__formatReadable===!1){W(`WebGLRenderer.readRenderTargetPixels: renderTarget is not in RGBA or implementation defined format.`);return}if(u.__typeReadable===!1){W(`WebGLRenderer.readRenderTargetPixels: renderTarget is not in UnsignedByteType or implementation defined type.`);return}t>=0&&t<=e.width-r&&n>=0&&n<=e.height-i&&z.readPixels(t,n,r,i,Je.convert(c),Je.convert(l),a)}finally{let e=L===null?null:H.get(L).__webglFramebuffer;V.bindFramebuffer(z.FRAMEBUFFER,e)}}},this.readRenderTargetPixelsAsync=async function(e,t,n,r,i,a,o,s=0){if(!(e&&e.isWebGLRenderTarget))throw Error(`THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.`);let c=H.get(e).__webglFramebuffer;if(e.isWebGLCubeRenderTarget&&o!==void 0&&(c=c[o]),c){if(t>=0&&t<=e.width-r&&n>=0&&n<=e.height-i){V.bindFramebuffer(z.FRAMEBUFFER,c);let o=e.textures[s],l=o.format,u=o.type;e.textures.length>1&&z.readBuffer(z.COLOR_ATTACHMENT0+s);let d=Tt(o);if(d.__formatReadable===!1)throw Error(`THREE.WebGLRenderer.readRenderTargetPixelsAsync: renderTarget is not in RGBA or implementation defined format.`);if(d.__typeReadable===!1)throw Error(`THREE.WebGLRenderer.readRenderTargetPixelsAsync: renderTarget is not in UnsignedByteType or implementation defined type.`);let f=z.createBuffer();z.bindBuffer(z.PIXEL_PACK_BUFFER,f),z.bufferData(z.PIXEL_PACK_BUFFER,a.byteLength,z.STREAM_READ),z.readPixels(t,n,r,i,Je.convert(l),Je.convert(u),0),z.bindBuffer(z.PIXEL_PACK_BUFFER,null);let p=L===null?null:H.get(L).__webglFramebuffer;V.bindFramebuffer(z.FRAMEBUFFER,p);let m=z.fenceSync(z.SYNC_GPU_COMMANDS_COMPLETE,0);return z.flush(),await Bt(z,m,4),z.bindBuffer(z.PIXEL_PACK_BUFFER,f),z.getBufferSubData(z.PIXEL_PACK_BUFFER,0,a),z.bindBuffer(z.PIXEL_PACK_BUFFER,null),z.deleteBuffer(f),z.deleteSync(m),a}throw Error(`THREE.WebGLRenderer.readRenderTargetPixelsAsync: requested read bounds are out of range.`)}},this.copyFramebufferToTexture=function(e,t=null,n=0){let r=2**-n,i=Math.floor(e.image.width*r),a=Math.floor(e.image.height*r),o=t===null?0:t.x,s=t===null?0:t.y;Ne.setTexture2D(e,0),z.copyTexSubImage2D(z.TEXTURE_2D,n,0,0,o,s,i,a),V.unbindTexture()},this.copyTextureToTexture=function(e,t,n=null,r=null,i=0,a=0){let o,s,c,l,u,d,f,p,m,h=e.isCompressedTexture?e.mipmaps[a]:e.image;if(n!==null)o=n.max.x-n.min.x,s=n.max.y-n.min.y,c=n.isBox3?n.max.z-n.min.z:1,l=n.min.x,u=n.min.y,d=n.isBox3?n.min.z:0;else{let t=2**-i;o=Math.floor(h.width*t),s=Math.floor(h.height*t),c=e.isDataArrayTexture?h.depth:e.isData3DTexture?Math.floor(h.depth*t):1,l=0,u=0,d=0}r===null?(f=0,p=0,m=0):(f=r.x,p=r.y,m=r.z);let g=Je.convert(t.format),_=Je.convert(t.type),v;t.isData3DTexture?(Ne.setTexture3D(t,0),v=z.TEXTURE_3D):t.isDataArrayTexture||t.isCompressedArrayTexture?(Ne.setTexture2DArray(t,0),v=z.TEXTURE_2D_ARRAY):(Ne.setTexture2D(t,0),v=z.TEXTURE_2D),V.activeTexture(z.TEXTURE0),V.pixelStorei(z.UNPACK_FLIP_Y_WEBGL,t.flipY),V.pixelStorei(z.UNPACK_PREMULTIPLY_ALPHA_WEBGL,t.premultiplyAlpha),V.pixelStorei(z.UNPACK_ALIGNMENT,t.unpackAlignment);let y=V.getParameter(z.UNPACK_ROW_LENGTH),b=V.getParameter(z.UNPACK_IMAGE_HEIGHT),x=V.getParameter(z.UNPACK_SKIP_PIXELS),S=V.getParameter(z.UNPACK_SKIP_ROWS),C=V.getParameter(z.UNPACK_SKIP_IMAGES);V.pixelStorei(z.UNPACK_ROW_LENGTH,h.width),V.pixelStorei(z.UNPACK_IMAGE_HEIGHT,h.height),V.pixelStorei(z.UNPACK_SKIP_PIXELS,l),V.pixelStorei(z.UNPACK_SKIP_ROWS,u),V.pixelStorei(z.UNPACK_SKIP_IMAGES,d);let w=e.isDataArrayTexture||e.isData3DTexture,T=t.isDataArrayTexture||t.isData3DTexture;if(e.isDepthTexture){let n=H.get(e),r=H.get(t),h=H.get(n.__renderTarget),g=H.get(r.__renderTarget);V.bindFramebuffer(z.READ_FRAMEBUFFER,h.__webglFramebuffer),V.bindFramebuffer(z.DRAW_FRAMEBUFFER,g.__webglFramebuffer);for(let n=0;n<c;n++)w&&(z.framebufferTextureLayer(z.READ_FRAMEBUFFER,z.COLOR_ATTACHMENT0,H.get(e).__webglTexture,i,d+n),z.framebufferTextureLayer(z.DRAW_FRAMEBUFFER,z.COLOR_ATTACHMENT0,H.get(t).__webglTexture,a,m+n)),z.blitFramebuffer(l,u,o,s,f,p,o,s,z.DEPTH_BUFFER_BIT,z.NEAREST);V.bindFramebuffer(z.READ_FRAMEBUFFER,null),V.bindFramebuffer(z.DRAW_FRAMEBUFFER,null)}else if(i!==0||e.isRenderTargetTexture||H.has(e)){let n=H.get(e),r=H.get(t);V.bindFramebuffer(z.READ_FRAMEBUFFER,ee),V.bindFramebuffer(z.DRAW_FRAMEBUFFER,te);for(let e=0;e<c;e++)w?z.framebufferTextureLayer(z.READ_FRAMEBUFFER,z.COLOR_ATTACHMENT0,n.__webglTexture,i,d+e):z.framebufferTexture2D(z.READ_FRAMEBUFFER,z.COLOR_ATTACHMENT0,z.TEXTURE_2D,n.__webglTexture,i),T?z.framebufferTextureLayer(z.DRAW_FRAMEBUFFER,z.COLOR_ATTACHMENT0,r.__webglTexture,a,m+e):z.framebufferTexture2D(z.DRAW_FRAMEBUFFER,z.COLOR_ATTACHMENT0,z.TEXTURE_2D,r.__webglTexture,a),i===0?T?z.copyTexSubImage3D(v,a,f,p,m+e,l,u,o,s):z.copyTexSubImage2D(v,a,f,p,l,u,o,s):z.blitFramebuffer(l,u,o,s,f,p,o,s,z.COLOR_BUFFER_BIT,z.NEAREST);V.bindFramebuffer(z.READ_FRAMEBUFFER,null),V.bindFramebuffer(z.DRAW_FRAMEBUFFER,null)}else T?e.isDataTexture||e.isData3DTexture?z.texSubImage3D(v,a,f,p,m,o,s,c,g,_,h.data):t.isCompressedArrayTexture?z.compressedTexSubImage3D(v,a,f,p,m,o,s,c,g,h.data):z.texSubImage3D(v,a,f,p,m,o,s,c,g,_,h):e.isDataTexture?z.texSubImage2D(z.TEXTURE_2D,a,f,p,o,s,g,_,h.data):e.isCompressedTexture?z.compressedTexSubImage2D(z.TEXTURE_2D,a,f,p,h.width,h.height,g,h.data):z.texSubImage2D(z.TEXTURE_2D,a,f,p,o,s,g,_,h);V.pixelStorei(z.UNPACK_ROW_LENGTH,y),V.pixelStorei(z.UNPACK_IMAGE_HEIGHT,b),V.pixelStorei(z.UNPACK_SKIP_PIXELS,x),V.pixelStorei(z.UNPACK_SKIP_ROWS,S),V.pixelStorei(z.UNPACK_SKIP_IMAGES,C),a===0&&t.generateMipmaps&&z.generateMipmap(v),V.unbindTexture()},this.initRenderTarget=function(e){H.get(e).__webglFramebuffer===void 0&&Ne.setupRenderTarget(e)},this.initTexture=function(e){e.isCubeTexture?Ne.setTextureCube(e,0):e.isData3DTexture?Ne.setTexture3D(e,0):e.isDataArrayTexture||e.isCompressedArrayTexture?Ne.setTexture2DArray(e,0):Ne.setTexture2D(e,0),V.unbindTexture()},this.resetState=function(){re=0,ae=0,L=null,V.reset(),Ye.reset()},typeof __THREE_DEVTOOLS__<`u`&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent(`observe`,{detail:this}))}get coordinateSystem(){return bt}get outputColorSpace(){return this._outputColorSpace}set outputColorSpace(e){this._outputColorSpace=e;let t=this.getContext();t.drawingBufferColorSpace=wn._getDrawingBufferColorSpace(e),t.unpackColorSpace=wn._getUnpackColorSpace()}};function eg(e,t=!1){let n=e[0].index!==null,r=new Set(Object.keys(e[0].attributes)),i=new Set(Object.keys(e[0].morphAttributes)),a={},o={},s=e[0].morphTargetsRelative,c=new Z,l=0;for(let u=0;u<e.length;++u){let d=e[u],f=0;if(n!==(d.index!==null))return console.error(`THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index `+u+`. All geometries must have compatible attributes; make sure index attribute exists among all geometries, or in none of them.`),null;for(let e in d.attributes){if(!r.has(e))return console.error(`THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index `+u+`. All geometries must have compatible attributes; make sure "`+e+`" attribute exists among all geometries, or in none of them.`),null;a[e]===void 0&&(a[e]=[]),a[e].push(d.attributes[e]),f++}if(f!==r.size)return console.error(`THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index `+u+`. Make sure all geometries have the same number of attributes.`),null;if(s!==d.morphTargetsRelative)return console.error(`THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index `+u+`. .morphTargetsRelative must be consistent throughout all geometries.`),null;for(let e in d.morphAttributes){if(!i.has(e))return console.error(`THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index `+u+`.  .morphAttributes must be consistent throughout all geometries.`),null;o[e]===void 0&&(o[e]=[]),o[e].push(d.morphAttributes[e])}if(t){let e;if(n)e=d.index.count;else if(d.attributes.position!==void 0)e=d.attributes.position.count;else return console.error(`THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index `+u+`. The geometry must have either an index or a position attribute`),null;c.addGroup(l,e,u),l+=e}}if(n){let t=0,n=[];for(let r=0;r<e.length;++r){let i=e[r].index;for(let e=0;e<i.count;++e)n.push(i.getX(e)+t);t+=e[r].attributes.position.count}c.setIndex(n)}for(let e in a){let t=tg(a[e]);if(!t)return console.error(`THREE.BufferGeometryUtils: .mergeGeometries() failed while trying to merge the `+e+` attribute.`),null;c.setAttribute(e,t)}for(let e in o){let t=o[e][0].length;if(t!==0){c.morphAttributes=c.morphAttributes||{},c.morphAttributes[e]=[];for(let n=0;n<t;++n){let t=[];for(let r=0;r<o[e].length;++r)t.push(o[e][r][n]);let r=tg(t);if(!r)return console.error(`THREE.BufferGeometryUtils: .mergeGeometries() failed while trying to merge the `+e+` morphAttribute.`),null;c.morphAttributes[e].push(r)}}}return c}function tg(e){let t,n,r,i=-1,a=0;for(let o=0;o<e.length;++o){let s=e[o];if(t===void 0&&(t=s.array.constructor),t!==s.array.constructor)return console.error(`THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.array must be of consistent array types across matching attributes.`),null;if(n===void 0&&(n=s.itemSize),n!==s.itemSize)return console.error(`THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.itemSize must be consistent across matching attributes.`),null;if(r===void 0&&(r=s.normalized),r!==s.normalized)return console.error(`THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.normalized must be consistent across matching attributes.`),null;if(i===-1&&(i=s.gpuType),i!==s.gpuType)return console.error(`THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.gpuType must be consistent across matching attributes.`),null;a+=s.count*n}let o=new t(a),s=new ui(o,n,r),c=0;for(let t=0;t<e.length;++t){let r=e[t];if(r.isInterleavedBufferAttribute){let e=c/n;for(let t=0,i=r.count;t<i;t++)for(let i=0;i<n;i++){let n=r.getComponent(t,i);s.setComponent(t+e,i,n)}}else o.set(r.array,c);c+=r.count*n}return i!==void 0&&(s.gpuType=i),s}var ng=`
precision highp float;
precision highp sampler2D;

uniform float uTime;
uniform vec4 uWind;          // direction x, z (unit), speed m/s, gust 0..1
uniform vec2 uWindScroll;    // how far the gust field has travelled, metres
uniform vec2 uExtentMin;     // corner of the ground the field knows (height, growth)
uniform float uExtentInv;    // 1 / its side
uniform float uHeightRes;    // texels across tHeight
uniform vec2 uStateMin;      // corner of the part that reacts and remembers
uniform float uStateInv;     // 1 / its side
uniform sampler2D tHeight;   // R32F, metres
uniform sampler2D tMap;      // dry, growth, wet, flowers
uniform sampler2D tState;    // bend x, bend z (128 = none), cut height in metres (255 = uncut), spare
uniform sampler2D tFire;     // heat, char, smoulder, ash
uniform sampler2D tGlow;     // where fire is (2 m cells, spread out), its light, spare, spare
uniform sampler2D tNoise;    // smooth, tiling: four different fields
uniform sampler2D tNoiseHi;  // the same fields at 16 bits, for reading stretched wide (wind, clouds)
uniform sampler2D tWhite;    // white noise, tiling, nearest

uniform vec3 uSunDir;        // towards the sun
uniform vec3 uSunColor;
uniform vec3 uSkyColor;      // light from above
uniform vec3 uBounceColor;   // light from below
uniform vec3 uFogColor;
uniform vec3 uFogSunColor;   // the haze towards the sun
uniform float uFogDensity;
uniform vec3 uFireColor;     // the colour fire throws on things, times its flicker
uniform vec3 uCamPos;
uniform vec4 uPushers[6];    // things standing in the grass: centre xyz, radius (0 for none)
uniform sampler2D tShade;      // tree shade, 0 to 1
uniform sampler2D tFloor;      // painted farmland; alpha where there is any
uniform vec3 uHeroEye;
uniform sampler2D tSlope;      // the ground's slope: see index.js
uniform sampler2D tStrokes;    // brushwork: tone in red (half is none), stones in blue
uniform vec4 uFloor;
uniform float uFloorInv;
uniform float uCloudShade;   // how dark a cloud's shadow is, 0 for none
uniform vec4 uWalker;        // someone walking in the open: x, z, how dark their shadow, how long (0 for nobody)
uniform vec4 uLamp;          // a flame carried through the field: where it is, how bright (0 for none)

// The carried flame's light at p, in the same measure as tGlow's.
float lampAt(vec3 p) {
  vec3 d = p - uLamp.xyz;
  return uLamp.w / (1.0 + dot(d, d) * 0.9);
}

const float TAU = 6.28318531;

vec4 hash42(vec2 p) {
  vec4 p4 = fract(vec4(p.xyxy) * vec4(0.1031, 0.1030, 0.0973, 0.1099));
  p4 += dot(p4, p4.wzxy + 33.33);
  return fract((p4.xxyz + p4.yzzw) * p4.zywx);
}

uvec3 pcg3d(uvec3 v) {
  v = v * 1664525u + 1013904223u;
  v.x += v.y * v.z; v.y += v.z * v.x; v.z += v.x * v.y;
  v ^= v >> 16u;
  v.x += v.y * v.z; v.y += v.z * v.x; v.z += v.x * v.y;
  return v;
}

// Four random numbers for a cell of the world, the same every frame and on every machine.
vec4 cellRand(vec2 cell, float salt) {
  uvec3 a = pcg3d(uvec3(ivec3(int(cell.x) + 65536, int(cell.y) + 65536, int(salt))));
  uvec3 b = pcg3d(a + 7u);
  return vec4(vec3(a >> 8u), float(b.x >> 8u)) / 16777216.0;
}

float hash12(vec2 p) {
  vec3 p3 = fract(vec3(p.xyx) * 0.1031);
  p3 += dot(p3, p3.yzx + 33.33);
  return fract((p3.x + p3.y) * p3.z);
}

vec2 extentUv(vec2 xz) { return (xz - uExtentMin) * uExtentInv; }
vec2 stateUv(vec2 xz) { return (xz - uStateMin) * uStateInv; }

// The ground's height, read the same way the CPU reads it (field.heightAt): bilinear between texel centres.
float groundAt(vec2 xz) {
  vec2 p = extentUv(xz) * uHeightRes - 0.5;
  vec2 i = floor(p);
  vec2 f = p - i;
  ivec2 lo = ivec2(0);
  ivec2 hi = ivec2(int(uHeightRes) - 1);
  ivec2 a = clamp(ivec2(i), lo, hi);
  ivec2 b = clamp(ivec2(i) + 1, lo, hi);
  float h00 = texelFetch(tHeight, a, 0).r;
  float h10 = texelFetch(tHeight, ivec2(b.x, a.y), 0).r;
  float h01 = texelFetch(tHeight, ivec2(a.x, b.y), 0).r;
  float h11 = texelFetch(tHeight, b, 0).r;
  return mix(mix(h00, h10, f.x), mix(h01, h11, f.x), f.y);
}

// 0 in a lull to 1 in a gust: two fields riding the wind, the broad one faster.
float gustAt(vec2 xz) {
  vec2 a = (xz - uWindScroll) / 61.0;
  vec2 b = (xz - uWindScroll * 0.62) / 17.0;
  float g = texture(tNoiseHi, a).r * 0.68 + texture(tNoiseHi, b + 0.37).g * 0.32;
  return smoothstep(0.26, 0.78, g);
}

// 1 in the open, less under a cloud or a tree. The clouds drift with the wind, slowly.
float openAt(vec2 xz) {
  // (summer cumulus: shadows the size of fields, crossing the valley at the wind's pace)
  vec2 p = (xz - uWindScroll * 2.6) / 3400.0;
  float c = texture(tNoiseHi, p).b * 0.7 + texture(tNoiseHi, p * 2.7 + 0.19).a * 0.3;
  float open = 1.0 - uCloudShade * smoothstep(0.47, 0.6, c);
  float tree = texture(tShade, extentUv(xz)).r;
  if (tree > 0.004) {
    // under a tree the light comes through the leaves in spots, and the spots move as the leaves do
    float s1 = texture(tNoise, xz / 2.7 + uTime * vec2(0.011, 0.008)).g;
    float s2 = texture(tNoise, xz / 0.9 - uTime * vec2(0.013, 0.017)).a;
    float spot = mix(smoothstep(0.53, 0.66, s1 * 0.62 + s2 * 0.38), 0.3, smoothstep(50.0, 150.0, distance(xz, uCamPos.xz)));
    open *= 1.0 - tree * (0.88 - 0.8 * spot);
  }
  return open;
}

// The shadow of someone walking: a body's width, lying away from the sun as long as the hour makes it, with the
// clean edge a painted shadow has.
float walkerShade(vec2 xz) {
  if (uWalker.w <= 0.0) return 1.0;
  vec2 d = xz - uWalker.xy;
  vec2 away = -normalize(uSunDir.xz + vec2(1e-5));
  float along = clamp(dot(d, away), 0.0, uWalker.w);
  float r = length(d - away * along);
  // (narrower towards the head)
  float wide = mix(0.36, 0.2, along / uWalker.w);
  return 1.0 - uWalker.z * (1.0 - smoothstep(wide, wide + 0.1, r));
}

// What the ground gets of the sun: the open sky, less anyone's shadow.
float cloudAt(vec2 xz) {
  return openAt(xz) * walkerShade(xz);
}

// Distance haze, warmer towards the sun.
vec3 applyFog(vec3 c, vec3 world) {
  vec3 d = world - uCamPos;
  float dist = length(d);
  // the haze lies in the valleys: what stands above it is seen through less of it
  float thin = mix(1.0, 0.3, smoothstep(250.0, 2400.0, world.y));
  float f = 1.0 - exp(-dist * uFogDensity * thin);
  float toSun = pow(max(dot(d / max(dist, 1e-3), uSunDir), 0.0), 6.0);
  return mix(c, mix(uFogColor, uFogSunColor, toSun), clamp(f, 0.0, 1.0));
}

// What grass is, as colour: green where it's lush, straw where it's dry. s is 0 at the root and 1 at the tip.
vec3 grassColor(float dry, float s) {
  vec3 green = mix(vec3(0.035, 0.135, 0.05), vec3(0.25, 0.43, 0.085), s);
  vec3 straw = mix(vec3(0.085, 0.090, 0.030), vec3(0.420, 0.400, 0.150), s);
  return mix(green, straw, dry);
}

// Broad patches of yellower and bluer green, as a painter lays a meadow in: the same wherever grass is drawn.
vec3 meadowTint(vec2 xz) {
  float a = texture(tNoiseHi, xz / 520.0).a;
  float b = texture(tNoiseHi, xz / 37.0 + 0.23).r;
  float w = smoothstep(0.3, 0.7, a * 0.6 + b * 0.4);
  vec3 t = mix(vec3(0.8, 0.96, 1.1), vec3(1.2, 1.06, 0.76), w);
  // here and there a patch of something darker and bluer: clover, sedge
  float c = texture(tNoiseHi, xz / 13.0 + 0.61).g;
  return t * mix(vec3(1.0), vec3(0.72, 0.87, 0.96), smoothstep(0.6, 0.72, c));
}

// Grass stands taller in some places than in others: 1 is its usual height.
float tallAt(vec2 xz) {
  float a = texture(tNoiseHi, xz / 23.0 + 0.47).b;
  float b = texture(tNoiseHi, xz / 7.0 + 0.11).r;
  return mix(0.72, 1.5, smoothstep(0.34, 0.68, a * 0.7 + b * 0.3));
}

// Which way the ground faces at a place on the field.
vec3 landNormal(vec2 xz) {
  vec2 s = (texture(tSlope, extentUv(xz)).rg - 0.5) * 0.5;
  return normalize(vec3(-s.x, 1.0, -s.y));
}

// How a slope stands to the sun, as a painter would have it: 0.5 for level ground, more where it is turned to the
// light and less where it is turned away. Slopes are made three times what they are, or gentle ground shows nothing.
float slopeRel(vec3 N) {
  vec3 n = normalize(vec3(N.x * 3.2, N.y, N.z * 3.2));
  vec3 L = normalize(vec3(uSunDir.x, max(uSunDir.y, 0.3), uSunDir.z));
  return clamp(0.5 + (dot(n, L) - L.y) * 1.5, 0.0, 1.0);
}

// How much of the sun reaches ground that stands so: a hollow is lit by the sky.
float slopeSun(float rel) {
  return mix(0.3, 1.0, smoothstep(0.04, 0.5, rel));
}

// The meadow as it is painted: three greens, chosen by tone (0 in deep shade, 1 where the light is full).
vec3 meadowGreen(float tone) {
  vec3 c = mix(vec3(0.05, 0.25, 0.07), vec3(0.15, 0.46, 0.085), smoothstep(0.2, 0.5, tone));
  return mix(c, vec3(0.3, 0.57, 0.12), smoothstep(0.56, 0.9, tone));
}

// The light on it at this hour. shade is how open the place is to the sun (cloudAt's measure).
vec3 meadowLight(float shade) {
  vec3 sun = mix(uSunColor, vec3(dot(uSunColor, vec3(0.333))), 0.3) * 0.42 + uSkyColor * 0.35;
  float lum = dot(uSkyColor, vec3(0.333));
  vec3 sky = mix(vec3(0.4, 0.74, 0.95), uSkyColor / max(lum, 0.01) * 0.62, 0.35) * (lum * 1.9 + 0.06);
  return mix(sky * 0.85, sun, smoothstep(0.25, 0.95, shade));
}

// A slope turned to the sun is lighter than one turned away, seen from far enough off to read as a slope.
float slopeLight(vec3 N) {
  return mix(0.7, 1.12, smoothstep(-0.1, 0.75, dot(N, uSunDir)));
}

// A meadow seen from off is brushwork: dabs of lighter and darker green at every size, the light ones leaning
// yellow and the dark ones blue, and (where stones is 1) a grey stone here and there.
// Where a point on the land is in the picture seen from the start of the walk: radians right of the valley's line,
// and radians below the level of the eye.
vec2 heroAngle(vec3 world) {
  vec2 d = world.xz - uHeroEye.xz;
  return vec2(atan(dot(d, vec2(-uFloor.y, uFloor.x)), dot(d, uFloor.xy)), atan(uHeroEye.y - world.y, length(d)));
}

// Cells: how far a point is from the edge of the cell it is in, and two numbers that are the cell's own.
vec3 paintCells(vec2 x) {
  vec2 n = floor(x);
  vec2 f = x - n;
  float d1 = 8.0;
  float d2 = 8.0;
  vec2 id = n;
  for (int j = -1; j <= 1; j++) {
    for (int i = -1; i <= 1; i++) {
      vec2 c = vec2(float(i), float(j));
      vec2 r = c + hash42(n + c).xy - f;
      float d = dot(r, r);
      if (d < d1) { d2 = d1; d1 = d; id = n + c; }
      else if (d < d2) d2 = d;
    }
  }
  vec4 h = hash42(id + 0.37);
  return vec3(sqrt(d2) - sqrt(d1), h.z, h.w);
}

// The far hills' recipe (backdrop.js), on the meadow. q is heroAngle there and w how much it changes across a
// pixel (a fragment shader's fwidth of it).
vec3 heroPaint(vec3 g, vec3 world, vec2 q, vec2 w, float dist) {
  float r = distance(world.xz, uHeroEye.xz);
  // not at your feet, where the blades are the brushwork; not past the plain; not behind the start, where the
  // picture's two edges meet
  float k = smoothstep(70.0, 150.0, dist) * (1.0 - smoothstep(2200.0, 3600.0, r)) * (1.0 - smoothstep(2.1, 2.8, abs(q.x)));
  if (k <= 0.0) return g;
  float px = max(w.x, w.y);
  // soft patches of three greens at two sizes, and long strokes of them lying level
  float soft = k * (1.0 - smoothstep(0.0015, 0.005, px));
  float v = texture(tNoiseHi, q / 0.42 + 0.2).r * 0.4 + texture(tNoiseHi, q / 0.15 + 0.6).g * 0.3 + texture(tNoiseHi, vec2(q.x / 0.5, q.y / 0.07) + 0.4).b * 0.3;
  g *= mix(vec3(1.0), vec3(0.7, 0.87, 1.0), (1.0 - smoothstep(0.42, 0.48, v)) * soft * 0.9);
  g *= mix(vec3(1.0), vec3(1.34, 1.15, 0.8), smoothstep(0.53, 0.59, v) * soft * 0.9);
  // clumps of rough grass, gathered into masses as the woods on the hills are
  float coarse = k * (1.0 - smoothstep(0.3, 0.7, px * 70.0)) * smoothstep(0.016, 0.03, px * 70.0);
  vec3 t = paintCells(q * 70.0);
  float mass = smoothstep(0.48, 0.56, texture(tNoiseHi, q / 0.3 + 0.7).a);
  g *= mix(vec3(1.0), vec3(0.62, 0.82, 0.9), smoothstep(0.1, 0.2, t.x) * step(t.y, 0.34 * mass + 0.04) * coarse);
  // dabs: a darker tuft, a fleck of sun, a few flowers
  float fine = k * (1.0 - smoothstep(0.3, 0.7, px * 220.0)) * smoothstep(0.05, 0.09, px * 220.0);
  vec3 c = paintCells(q * 220.0);
  float dab = smoothstep(0.1, 0.22, c.x) * fine;
  g *= mix(vec3(1.0), vec3(0.72, 0.85, 0.92), dab * step(0.8, c.y));
  g *= mix(vec3(1.0), vec3(1.26, 1.13, 0.84), dab * step(c.y, 0.15));
  g = mix(g, c.z > 0.985 ? vec3(0.88, 0.86, 0.78) : vec3(0.86, 0.8, 0.4), smoothstep(0.3, 0.42, c.x) * fine * step(0.965, c.z) * 0.6);
  return g;
}

// Where the long strokes on the land lie: lines of level ground, wandered a little so they are not a map's.
float strokeCoord(vec3 world) {
  return (world.y + 7.0 * texture(tNoiseHi, world.xz / 270.0 + 0.4).g) / 5.5;
}

// On level ground there is no lie of the land to follow, so a second set of strokes lies across the valley, wandering.
float strokeCoord2(vec2 xz) {
  return (dot(xz, uFloor.xy) + 70.0 * texture(tNoiseHi, xz / 430.0 + 0.7).a) / 31.0 + 57.0;
}

// Long thin strokes that follow the lie of the land, a few of them, some lighter than the grass and some deeper:
// what a painter draws on a far meadow to show how it lies. x is strokeCoord there and w how much it changes
// across a pixel (a fragment shader's fwidth of it), which keeps a stroke the same few pixels thick anywhere.
vec3 landStrokes(vec3 g, vec2 xz, float dist, float x, float w) {
  float far = smoothstep(70.0, 170.0, dist) * (1.0 - smoothstep(2600.0, 4600.0, dist));
  float line = 1.0 - smoothstep(1.5, 3.2, abs(fract(x) - 0.5) / max(w, 1e-5));
  float id = floor(x);
  // only here and there, and each for a different length
  float here = smoothstep(0.45, 0.53, texture(tNoiseHi, xz / 340.0 + id * 0.37).b);
  float k = line * here * far;
  // most are pale, as a track or a line of sunlit grass is; some are a deeper green
  return fract(id * 0.618) < 0.62 ? mix(g, vec3(0.5, 0.64, 0.22), k * 0.6) : g * mix(vec3(1.0), vec3(0.62, 0.8, 0.86), k);
}

vec3 paintGround(vec3 g, vec2 xz, float dist, float stones) {
  // at your feet, the brush's dabs, the same the blades take their tone from; gone by a hundred metres
  float nearK = 1.0 - smoothstep(40.0, 110.0, dist);
  float t = ((texture(tStrokes, xz / 6.0).r - 0.5) * 0.5 + 0.7 * (texture(tStrokes, xz.yx / 27.0 + 0.37).r - 0.5) + 0.9 * (texture(tStrokes, xz / 95.0 + 0.11).r - 0.5)) * 0.7 * nearK;
  float farK = 1.0 - nearK;
  // beyond, one slow change of green across the land,
  t += (texture(tNoiseHi, xz / 1100.0 + 0.2).r - 0.5) * 0.5 * farK;
  g *= (1.0 + t * 0.5) * mix(vec3(0.9, 1.0, 1.1), vec3(1.1, 1.03, 0.84), clamp(0.5 + t * 1.4, 0.0, 1.0));
  // and on it, as on the far hills, soft patches at two sizes: a sunlit yellower green here, a deeper bluer one
  // there. They differ in colour more than in tone and their edges are soft, which is what keeps them paint and
  // not blotches.
  float v = texture(tNoiseHi, xz / 520.0 + 0.2).g * 0.58 + texture(tNoiseHi, xz / 170.0 + 0.61).b * 0.42;
  g *= mix(vec3(1.0), vec3(1.26, 1.13, 0.76), smoothstep(0.5, 0.62, v) * farK * 0.4);
  g *= mix(vec3(1.0), vec3(0.76, 0.92, 1.06), (1.0 - smoothstep(0.36, 0.48, v)) * farK * 0.4);
  // a few tufts of longer grass standing darker in it: short dashes lying across the view, on the nearer ground only
  vec2 q = vec2(dot(xz, vec2(-uFloor.y, uFloor.x)) / 130.0, dot(xz, uFloor.xy) / 64.0);
  float tuft = 1.0 - smoothstep(0.13, 0.2, texture(tStrokes, q).r);
  g *= mix(vec3(1.0), vec3(0.7, 0.86, 0.9), tuft * farK * (1.0 - smoothstep(350.0, 800.0, dist)) * 0.6);
  // (a far meadow in the sun is a lighter, yellower green than the grass at your feet, which you see into)
  return g * mix(vec3(1.0), vec3(1.16, 1.1, 0.9), 1.0 - nearK);
}

vec3 paintGroundOld(vec3 g, vec2 xz, float dist, float stones) {
  float a = texture(tStrokes, xz / 6.0).r - 0.5;
  float b = texture(tStrokes, xz.yx / 27.0 + 0.37).r - 0.5;
  float c = texture(tStrokes, xz / 121.0 + 0.71).r - 0.5;
  float e = texture(tStrokes, xz.yx / 470.0 + 0.13).r - 0.5;
  float t = a * (1.0 - smoothstep(50.0, 200.0, dist)) + b * 0.9 * (1.0 - smoothstep(500.0, 1700.0, dist)) + c * 1.5 + e * 1.3;
  // past the blades the meadow is a painting: mid is how much of one
  float mid = smoothstep(40.0, 120.0, dist) * (1.0 - smoothstep(2500.0, 5000.0, dist));
  // its greens are laid in as separate paints with an edge between one and the next, not blended
  float s = t * 3.0 + 0.5;
  t = mix(t, (floor(s) + smoothstep(0.38, 0.62, fract(s)) - 0.5) / 3.0, mid * 0.85);
  g *= (1.0 + t * 0.85) * mix(vec3(0.86, 1.0, 1.16), vec3(1.14, 1.04, 0.78), clamp(0.5 + t * 1.4, 0.0, 1.0));
  // Flat ground seen from a hill is squeezed twentyfold from front to back, so a mark must be long and lie across
  // the view to be seen at all: the meadow is painted in bands, as it is in the films. Broad bands of deeper and
  // of sunlit grass, thin dark and light streaks along them, and drifts of flowers lying the same way.
  vec2 across = vec2(-uFloor.y, uFloor.x);
  vec2 q1 = vec2(dot(xz, across) / 1300.0, dot(xz, uFloor.xy) / 250.0);
  vec2 q2 = vec2(dot(xz, across * 0.86 + uFloor.xy * 0.5) / 900.0, dot(xz, uFloor.xy * 0.86 - across * 0.5) / 170.0) + 0.43;
  float l1 = texture(tStrokes, q1).r;
  float l2 = texture(tStrokes, q2).r;
  float reach = mid * (1.0 - smoothstep(1400.0, 3000.0, dist));
  float lo = min(l1, l2);
  float hi = max(l1, l2);
  g *= mix(vec3(1.0), vec3(0.8, 0.9, 0.96), (1.0 - smoothstep(0.42, 0.46, lo)) * reach);
  g *= mix(vec3(1.0), vec3(0.72, 0.84, 0.9), (1.0 - smoothstep(0.26, 0.32, lo)) * reach);
  g *= mix(vec3(1.0), vec3(1.08, 1.06, 0.94), smoothstep(0.55, 0.59, hi) * reach);
  g *= mix(vec3(1.0), vec3(1.12, 1.08, 0.9), smoothstep(0.7, 0.76, hi) * reach);
  float bloom = texture(tStrokes, q1 * vec2(1.7, 1.3) + 0.61).r;
  float drift = smoothstep(0.46, 0.58, texture(tNoiseHi, xz / 310.0 + 0.3).g);
  vec3 petal = mix(vec3(0.85, 0.82, 0.62), vec3(0.86, 0.66, 0.2), step(0.5, texture(tNoiseHi, xz / 170.0).a));
  g = mix(g, petal, max(smoothstep(0.55, 0.6, bloom) * 0.3, smoothstep(0.7, 0.76, bloom) * 0.7) * drift * reach * (1.0 - smoothstep(350.0, 800.0, dist)));
  float rock = texture(tStrokes, xz / 31.0 + 0.19).b * stones * (1.0 - smoothstep(500.0, 1100.0, dist));
  return mix(g, vec3(0.5, 0.5, 0.47), smoothstep(0.35, 0.6, rock));
}

// The farmland's paint at a place, and in alpha how much of the ground there is farmland.
vec4 floorAt(vec2 xz) {
  vec2 uv = (vec2(dot(xz, uFloor.xy), dot(xz, vec2(-uFloor.y, uFloor.x))) - uFloor.zw) * uFloorInv;
  return texture(tFloor, clamp(uv, 0.0, 1.0));
}
`,rg=`
// up: 0 at the ground to 1 at the top of the sward. shade: cloudAt there. glow: tGlow's light there.
vec3 leafLight(vec3 albedo, vec3 N, vec3 V, float up, float gloss, float shade, float glow) {
  float ndl = dot(N, uSunDir);
  // low sun reaches the tips and little else: the blades in front stand in the way
  float reach = pow(clamp(up, 0.0, 1.0), mix(3.0, 0.55, clamp(uSunDir.y * 2.2, 0.0, 1.0)));
  float sun = shade * mix(0.05, 1.0, reach);
  vec3 through = albedo * vec3(1.0, 1.16, 0.36) + vec3(0.012, 0.02, 0.0);
  vec3 c = albedo * uSunColor * max(ndl, 0.0) * sun;
  c += through * uSunColor * max(-ndl, 0.0) * 0.72 * sun;
  vec3 H = normalize(uSunDir + V);
  c += uSunColor * pow(abs(dot(N, H)), 42.0) * gloss * sun;
  float ao = mix(0.22, 1.0, clamp(up, 0.0, 1.0));
  c += albedo * mix(uBounceColor, uSkyColor, N.y * 0.5 + 0.5) * ao;
  c += albedo * uFireColor * glow * (0.35 + 0.65 * up);
  return c;
}
`,ig=`
// dark red, red, orange, yellow, white-hot, in linear light (from Patch's fire block)
vec3 fireRamp(float T) {
  vec3 c = mix(vec3(0.05, 0.002, 0.0), vec3(0.42, 0.028, 0.002), smoothstep(0.0, 0.25, T));
  c = mix(c, vec3(1.1, 0.13, 0.008), smoothstep(0.25, 0.45, T));
  c = mix(c, vec3(1.6, 0.34, 0.018), smoothstep(0.45, 0.65, T));
  c = mix(c, vec3(2.25, 0.9, 0.07), smoothstep(0.65, 0.85, T));
  c = mix(c, vec3(5.0, 3.6, 1.6), smoothstep(0.85, 1.1, T));
  return c;
}
`;function ag(e){let t=e>>>0;return()=>{t=t+1831565813>>>0;let e=t;return e=Math.imul(e^e>>>15,e|1),e^=e+Math.imul(e^e>>>7,e|61),((e^e>>>14)>>>0)/4294967296}}var og=e=>e*e*e*(e*(e*6-15)+10);function sg(e,t){let n=new Float32Array(e*e);for(let e=0;e<n.length;e++)n[e]=t();return(t,r)=>{let i=Math.floor(t),a=Math.floor(r),o=og(t-i),s=og(r-a),c=(i%e+e)%e,l=(a%e+e)%e,u=(c+1)%e,d=(l+1)%e,f=n[c+l*e],p=n[u+l*e],m=n[c+d*e],h=n[u+d*e];return f+(p-f)*o+(m-f)*s+(f-p-m+h)*o*s}}function cg(e=256,t=11,n=!1){let r=ag(t),i=n?new Uint16Array(e*e*4):new Uint8Array(e*e*4);for(let t=0;t<4;t++){let a=[],o=4;for(let e=0;e<5;e++)a.push({period:o,at:sg(o,r)}),o*=2;let s=new Float32Array(e*e),c=1/0,l=-1/0;for(let t=0;t<e;t++)for(let n=0;n<e;n++){let r=0,i=.5;for(let o of a)r+=i*o.at(n/e*o.period,t/e*o.period),i*=.5;s[n+t*e]=r,c=Math.min(c,r),l=Math.max(l,r)}for(let e=0;e<s.length;e++){let r=(s[e]-c)/(l-c);i[e*4+t]=n?oi.toHalfFloat(r):Math.round(r*255)}}let a=new Fa(i,e,e,F,n?D:b);return a.wrapS=a.wrapT=s,a.minFilter=n?h:v,a.magFilter=h,a.generateMipmaps=!n,a.needsUpdate=!0,a}function lg(e=256,t=5){let n=ag(t),r=new Uint8Array(e*e*4);for(let e=0;e<r.length;e++)r[e]=Math.floor(n()*256);let i=new Fa(r,e,e,F,b);return i.wrapS=i.wrapT=s,i.minFilter=i.magFilter=u,i.needsUpdate=!0,i}function ug(e=64,t=23){let n=ag(t),r=[],i=4;for(let e=0;e<3;e++){let e=new Float32Array(i*i*i);for(let t=0;t<e.length;t++)e[t]=n();r.push({period:i,v:e}),i*=2}let a=new Float32Array(e*e*e),o=1/0,c=-1/0;for(let t=0;t<e;t++)for(let n=0;n<e;n++)for(let i=0;i<e;i++){let s=0,l=.5;for(let{period:a,v:o}of r){let r=i/e*a,c=n/e*a,u=t/e*a,d=Math.floor(r),f=Math.floor(c),p=Math.floor(u),m=og(r-d),h=og(c-f),g=og(u-p),_=(d+1)%a,v=(f+1)%a,y=(p+1)%a,b=(e,t,n)=>o[e+a*(t+a*n)],x=b(d,f,p)+(b(_,f,p)-b(d,f,p))*m,S=b(d,v,p)+(b(_,v,p)-b(d,v,p))*m,C=b(d,f,y)+(b(_,f,y)-b(d,f,y))*m,w=b(d,v,y)+(b(_,v,y)-b(d,v,y))*m,T=x+(S-x)*h,E=C+(w-C)*h;s+=l*(T+(E-T)*g),l*=.5}a[i+e*(n+e*t)]=s,o=Math.min(o,s),c=Math.max(c,s)}let l=new Uint8Array(a.length);for(let e=0;e<a.length;e++)l[e]=Math.round((a[e]-o)/(c-o)*255);let u=new Vn(l,e,e,e);return u.format=te,u.type=b,u.wrapS=u.wrapT=u.wrapR=s,u.minFilter=u.magFilter=h,u.unpackAlignment=1,u.needsUpdate=!0,u}var dg=[{name:`near`,cell:.05,tile:3,tiles:6,rows:8,sub:1,fade:[-2,-1,2.8,6],width:1,tops:0},{name:`mid`,cell:.09,tile:7.2,tiles:8,rows:4,sub:1,fade:[2.8,6,13,21.6],width:2,tops:.05},{name:`wide`,cell:.2,tile:16,tiles:8,rows:3,sub:1,fade:[13,21.6,34,48],width:4.6,tops:.22},{name:`far`,cell:.625,tile:40,tiles:8,rows:3,sub:3,fade:[34,48,96,120],width:10,tops:.4}],fg=`
${ng}
in vec2 aCell;
uniform float uCell;
uniform vec4 uFade;
uniform float uWidth;
uniform float uSalt;
uniform float uRows;
uniform float uHeight;      // metres, a tall blade in good ground
uniform float uTops;        // 0 for a ring of single blades; more where one blade stands for many and you mostly see their tops

out vec3 vWorld;
out vec3 vNormal;
out vec3 vAcross;
out vec3 vAlbedo;
out vec4 vMisc;   // side, up, gloss, heat
out vec2 vLight;  // cloud shade, fire light

// where a seed stalk's rows sit: most of them in the head
float stalkRow(float t) {
  float k = t * (uRows - 1.0);
  if (uRows > 6.5) {
    float r[8] = float[8](0.0, 0.3, 0.55, 0.74, 0.8, 0.87, 0.94, 1.0);
    return r[int(k + 0.5)];
  }
  if (uRows > 3.5) {
    float r[4] = float[4](0.0, 0.76, 0.88, 1.0);
    return r[int(k + 0.5)];
  }
  return t;
}

void main() {
  float side = position.x;
  float t = position.y;
  float sub = position.z;
  vec2 origin = modelMatrix[3].xz;
  vec2 cell = floor(origin / uCell + 0.5) + aCell;
  vec4 r = cellRand(cell, uSalt + sub * 7.0);
  vec4 q = cellRand(cell, uSalt + 31.0 + sub * 7.0);
  vec2 xz = (cell + r.xy) * uCell;
  float dist = distance(xz, uCamPos.xz);
  float keep = (uFade.y < 0.0 ? 1.0 : smoothstep(uFade.x, uFade.y, dist)) * (1.0 - smoothstep(uFade.z, uFade.w, dist));
  float live = smoothstep(0.0, 0.18, keep - q.x * 0.82);
  vec4 m = texture(tMap, extentUv(xz));
  float growth = m.g;
  live *= smoothstep(0.0, 0.22, growth - q.y * 0.78);
  if (live <= 0.0) {
    gl_Position = vec4(2.0, 2.0, 2.0, 1.0);
    return;
  }

  vec2 suv = stateUv(xz);
  vec4 st = texture(tState, suv);
  vec4 fr = texture(tFire, suv);
  vec2 bend = (st.rg * 255.0 - 128.0) / 127.0;
  float flat_ = min(length(bend), 1.0);
  float cutH = st.b + (q.y - 0.5) * 0.024;
  // stubble is too short and stiff to tread flat: trodden, mown ground was showing as pale combed stripes
  bend *= smoothstep(0.08, 0.26, st.b);
  flat_ = min(length(bend), 1.0);
  float charred = smoothstep(0.12, 0.7, fr.g + (q.x - 0.5) * 0.3);
  float heat = fr.r;

  // neighbours share a look: a clump is one texel of white noise, its edge broken up blade by blade
  vec4 cl = texture(tWhite, (xz + (r.zw - 0.5) * 0.34) / (0.44 * 256.0));
  float dry = smoothstep(0.3, 0.7, m.r + (q.z - 0.5) * 0.5 + (cl.g - 0.5) * 0.26);
  bool stalk = uRows > 3.5 && q.w < 0.012 + 0.045 * m.r;
  float h = uHeight * mix(0.5, 1.0, cl.r) * mix(0.7, 1.22, r.z) * mix(0.7, 1.0, growth) * (1.0 + 0.7 * m.b) * mix(0.55, 1.0, live);
  if (stalk) h *= 1.5;
  // (broad patches: long, deeper grass in some, short sunlit grass in others)
  float patchT = (textureLod(tStrokes, xz.yx / 27.0 + 0.37, 0.0).r - 0.5) * 0.7 + (textureLod(tStrokes, xz / 95.0 + 0.11, 0.0).r - 0.5) * 0.9;
  h *= tallAt(xz) * mix(1.32, 0.78, clamp(0.5 + patchT * 1.5, 0.0, 1.0));
  h *= 1.0 + 0.3 * uTops;
  h *= 1.0 - 0.8 * charred;

  float gy = groundAt(xz);
  vec3 base = vec3(xz.x, gy, xz.y);

  // which way it leans, and how far, as an angle from upright
  float yaw = (cl.b * 0.45 + r.w * 0.55) * TAU;
  vec2 L = vec2(cos(yaw), sin(yaw)) * mix(0.22, 0.85, q.z) * (stalk ? 0.3 : 1.0);
  float gust = gustAt(xz);
  float U = uWind.z;
  float flutter = sin(r.w * 40.0 + uTime * (5.0 + 4.0 * r.z)) * (0.028 + 0.013 * U);
  L += uWind.xy * (U * mix(0.028, 0.105, gust) + flutter * 0.6) + vec2(-uWind.y, uWind.x) * flutter * 0.5;
  L += vec2(sin(uTime * 0.7 + r.x * 30.0), cos(uTime * 0.6 + r.y * 30.0)) * 0.03;
  for (int i = 0; i < 6; i++) {
    vec4 p = uPushers[i];
    if (p.w <= 0.0) continue;
    vec2 d = xz - p.xz;
    float dd = length(d);
    float k = (1.0 - smoothstep(p.w * 0.7, p.w * 1.9, dd)) * (1.0 - smoothstep(gy + h * 0.5, gy + h + 0.12, p.y - p.w));
    L += d / max(dd, 1e-3) * k * 0.95;
  }
  L = L * (1.0 - 0.8 * flat_) + bend * 1.42;
  L *= 1.0 + 0.9 * charred;
  float lean = length(L);
  vec2 dir = L / max(lean, 1e-4);
  lean = min(lean, 1.5);
  float theta0 = lean * mix(0.25, 0.82, flat_);
  float A = lean - theta0;

  // cut: the blade stops where the edge went through it
  float top = A > 1e-3 ? h * (sin(theta0 + A) - sin(theta0)) / A : h * cos(theta0);
  float sCut = clamp(cutH / max(top, 1e-3), 0.0, 1.0);
  bool isCut = sCut < 0.995;
  float s = (stalk ? stalkRow(t) : t) * sCut;

  float w = uWidth * 0.0115 * mix(0.72, 1.3, r.x) * (uWidth < 1.5 ? 2.1 : uWidth < 3.0 ? 1.45 : 1.0);
  if (stalk) w *= s < 0.78 ? 0.22 : 0.22 + 0.62 * sin(3.14159 * (s - 0.78) / 0.22);
  else w *= min(1.0, (1.0 - s) * 3.2) * (0.72 + 0.28 * (1.0 - s));
  w *= (1.0 - 0.45 * charred) * mix(0.5, 1.0, live);
  // stubble is the thick foot of the blade
  if (isCut) w *= 1.0 + 0.7 * (1.0 - sCut);

  float phi = theta0 + A * s;
  float along = A > 1e-3 ? h * (cos(theta0) - cos(phi)) / A : h * s * sin(theta0);
  float rise = A > 1e-3 ? h * (sin(phi) - sin(theta0)) / A : h * s * cos(theta0);
  vec3 tangent = vec3(dir.x * sin(phi), cos(phi), dir.y * sin(phi));
  vec3 across = vec3(-dir.y, 0.0, dir.x);
  // turned partly to face the camera, so a blade seen edge on doesn't vanish
  vec3 toCam = normalize(uCamPos - base);
  vec3 facing = cross(tangent, toCam);
  float fl = length(facing);
  if (fl > 1e-3) {
    facing /= fl;
    across = normalize(mix(across, facing * (dot(facing, across) < 0.0 ? -1.0 : 1.0), 0.68));
  }
  vec3 world = base + vec3(dir.x * along, rise, dir.y * along) + across * side * w * 0.5;

  vWorld = world;
  vAcross = across;
  vNormal = cross(across, tangent);

  float open = max(1.0 - smoothstep(0.1, 0.4, st.b), charred);
  float up = mix(clamp(rise / (uHeight * 0.85), 0.0, 1.0), 0.8, open * 0.8);
  // a tuft stands for many blades, and of those you mostly see the tops
  float sc = mix(s, 1.0, uTops);
  up = max(up, uTops * 1.2);
  vec3 col = grassColor(dry * (1.0 - 0.5 * m.b), sc);
  col *= mix(0.88, 1.14, cl.a) * mix(0.94, 1.06, q.z) * (1.0 + 0.3 * gust * min(U / 6.0, 1.0));
  // some clumps are yellower, some bluer: more than one kind of grass grows here
  col *= mix(vec3(1.08, 1.0, 0.82), vec3(0.9, 1.0, 1.08), cl.g);
  col *= meadowTint(xz);
  // each blade is a stroke of the brush and takes the tone of the dab it stands in: light dabs yellower, dark bluer
  float dab = (textureLod(tStrokes, xz / 6.0, 0.0).r - 0.5) * 0.5 + patchT;
  // only some blades are drawn: one in a dozen is a darker stroke, one in fifteen catches the light
  float tone = 0.58 + dab * 0.7 + (cl.a - 0.5) * 0.08 + (sc - 0.6) * 0.1 + 0.1 * gust * min(U / 6.0, 1.0);
  tone += q.z < 0.045 ? -0.26 : q.z > 0.965 ? 0.18 : 0.0;
  float rel = slopeRel(landNormal(xz));
  tone += (rel - 0.5) * 0.6;
  // the grass nearest you is the picture's foreground, and a tone deeper
  tone -= 0.2 * (1.0 - smoothstep(1.5, 9.0, dist));
  col = meadowGreen(tone) * mix(vec3(1.0), meadowTint(xz), 0.5);
  col = mix(col, vec3(0.44, 0.47, 0.17), dry * 0.45);
  if (stalk) col = mix(col, mix(vec3(0.46, 0.52, 0.18), vec3(0.4, 0.46, 0.2), step(0.72, cl.g)), smoothstep(0.75, 0.82, s));
  // far off, a few tufts carry the colour of the flowers among them
  if (uTops > 0.35 && q.w < m.a * 0.1) col = mix(col, mix(vec3(0.75, 0.72, 0.6), vec3(0.7, 0.5, 0.05), step(0.5, q.z)), smoothstep(0.4, 1.0, s));
  if (isCut) col = mix(col, col * 1.35 + vec3(0.025, 0.028, 0.008), smoothstep(0.7, 1.0, t) * 0.7);
  col = mix(col, col * 1.2 + 0.008, flat_ * 0.7);
  col = mix(col, vec3(0.011, 0.010, 0.009), charred);
  vAlbedo = col;
  float gloss = 0.015 + 0.05 * gust * min(U / 8.0, 1.0) + 0.05 * flat_;
  vMisc = vec4(side, up, gloss * (1.0 - charred), heat * (1.0 - 0.5 * charred) * (0.35 + 0.65 * s));
  // a low sun lights the tall clumps and leaves the short ones behind them in shade
  float among = mix(mix(0.45, 1.0, smoothstep(0.15, 0.75, cl.r)), 1.0, clamp(uSunDir.y * 2.5, 0.0, 1.0));
  vLight = vec2(cloudAt(xz) * mix(among, 1.0, open) * slopeSun(rel), texture(tGlow, suv).g + lampAt(world));
  gl_Position = projectionMatrix * viewMatrix * vec4(world, 1.0);
}
`,pg=`
${ng}
${rg}
${ig}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
in vec3 vAcross;
in vec3 vAlbedo;
in vec4 vMisc;
in vec2 vLight;

void main() {
  vec3 V = normalize(uCamPos - vWorld);
  vec3 N = normalize(vNormal);
  if (dot(N, V) < 0.0) N = -N;
  // a blade is folded along its middle, not flat
  N = normalize(N + normalize(vAcross) * vMisc.x * 0.5);
  vec3 c = vAlbedo * meadowLight(vLight.x) * mix(0.84, 1.0, vMisc.y) + vAlbedo * uFireColor * vLight.y * 0.5;
  c += fireRamp(0.3 + 0.55 * vMisc.w) * vMisc.w * 1.5;
  outC = vec4(applyFog(c, vWorld), 1.0);
}
`;function mg(e,t){let{rows:n,sub:r,tile:i,cell:a}=e,o=[],s=[];for(let e=0;e<r;e++){let t=o.length/3;for(let r=0;r<n;r++){let i=r/(n-1);if(o.push(-1,i,e,1,i,e),r<n-1){let e=t+r*2;s.push(e,e+1,e+3,e,e+3,e+2)}}}let c=Math.round(i/a),l=new Float32Array(c*c*2);for(let e=0;e<c;e++)for(let t=0;t<c;t++)l[(t+e*c)*2]=t,l[(t+e*c)*2+1]=e;for(let e=c*c-1;e>0;e--){let n=Math.floor(t()*(e+1));for(let t=0;t<2;t++){let r=l[e*2+t];l[e*2+t]=l[n*2+t],l[n*2+t]=r}}let u=new Du;return u.setAttribute(`position`,new X(o,3)),u.setIndex(s),u.setAttribute(`aCell`,new za(l,2)),u.instanceCount=c*c,u.boundingSphere=new Si(new q(i/2,.3,i/2),i*.75+2.5),{geometry:u,count:c*c}}function hg(e,{height:t=.52,density:n=1,seed:r=1}={},i){let a=new gr,o=ag(r*977+3),s=dg.map((n,i)=>{let{geometry:s,count:c}=mg(n,o),l={...e,uCell:{value:n.cell},uFade:{value:new In(...n.fade)},uWidth:{value:n.width},uSalt:{value:i*101+r},uRows:{value:n.rows},uHeight:{value:t},uTops:{value:n.tops}},u=new Qc({glslVersion:yt,vertexShader:fg,fragmentShader:pg,uniforms:l,side:2}),d=[];for(let e=0;e<n.tiles;e++)for(let t=0;t<n.tiles;t++){let r=new xa(s,u);r.matrixAutoUpdate=!1,d.push({mesh:r,i:t-n.tiles/2,j:e-n.tiles/2}),a.add(r)}return{ring:n,geometry:s,material:u,uniforms:l,tiles:d,count:c,at:[NaN,NaN]}});return{group:a,rings:s,setDensity(e){for(let t of s)t.geometry.instanceCount=Math.max(1,Math.round(t.count*Math.min(1,Math.max(.05,e))))},setHeight(e){for(let t of s)t.uniforms.uHeight.value=e},follow(e,t){for(let n of s){let r=Math.floor(e/n.ring.tile),a=Math.floor(t/n.ring.tile);if(r!==n.at[0]||a!==n.at[1]){n.at=[r,a];for(let e of n.tiles){let t=(r+e.i)*n.ring.tile,o=(a+e.j)*n.ring.tile;e.mesh.position.set(t,i(t+n.ring.tile/2,o+n.ring.tile/2),o),e.mesh.updateMatrix(),e.mesh.updateMatrixWorld(!0)}}}},init(){this.setDensity(n)}}}var gg=49152,_g=30,vg=300,yg=`
${ng}
in vec3 aPos;
in vec3 aDir;
in vec3 aSide;
in vec4 aColor;   // its colour when cut, and when that was
out vec3 vWorld;
out vec3 vNormal;
out vec3 vAlbedo;
out vec3 vLight;
void main() {
  float t = position.y * 0.5 + 0.5;
  float taper = 1.0 - 0.8 * t * t;
  vec3 world = aPos + aDir * position.y + aSide * position.x * taper;
  vWorld = world;
  vNormal = cross(aSide, aDir);
  // green when cut; hay by the time it has lain a few minutes
  float lain = smoothstep(20.0, 240.0, uTime - aColor.a);
  float lum = dot(aColor.rgb, vec3(0.3, 0.6, 0.1));
  vAlbedo = mix(aColor.rgb, vec3(0.62, 0.56, 0.30) * (0.35 + 1.6 * lum), lain * 0.75);
  // deep in the heap or down in the stubble it is in shade; on top it is in the light
  float up = clamp(0.3 + (world.y - groundAt(world.xz)) / 0.22, 0.0, 0.8);
  vLight = vec3(cloudAt(world.xz), texture(tGlow, stateUv(world.xz)).g + lampAt(world), up);
  gl_Position = projectionMatrix * viewMatrix * vec4(world, 1.0);
}
`,bg=`
${ng}
${rg}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
in vec3 vAlbedo;
in vec3 vLight;
void main() {
  vec3 V = normalize(uCamPos - vWorld);
  vec3 N = normalize(vNormal + vec3(0.0, 1e-4, 0.0));
  if (dot(N, V) < 0.0) N = -N;
  // a cut stem is curled and creased, not a flat plate: some of it always faces the light
  N = normalize(N + uSunDir * 0.28 + vec3(0.0, 0.2, 0.0));
  vec3 c = leafLight(vAlbedo, N, V, vLight.z, 0.05, vLight.x, vLight.y);
  outC = vec4(applyFog(c, vWorld), 1.0);
}
`;function xg(e,t,n){let r=(e,t,n)=>e+(t-e)*n;return n[0]=r(r(.02,.118,t),r(.095,.39,t),e),n[1]=r(r(.042,.19,t),r(.076,.325,t),e),n[2]=r(r(.011,.038,t),r(.034,.17,t),e),n}function Sg(e,t,n,r){let i=new Du;i.setAttribute(`position`,new X([-1,-1,0,1,-1,0,-1,1,0,1,1,0],3)),i.setIndex([0,1,3,0,3,2]);let a=new za(new Float32Array(gg*3),3),o=new za(new Float32Array(gg*3),3),s=new za(new Float32Array(gg*3),3),c=new za(new Float32Array(gg*4),4);for(let e of[a,o,s])e.setUsage(dt);i.setAttribute(`aPos`,a),i.setAttribute(`aDir`,o),i.setAttribute(`aSide`,s),i.setAttribute(`aColor`,c),i.instanceCount=0;let l=new xa(i,new Qc({glslVersion:yt,vertexShader:yg,fragmentShader:bg,uniforms:e,side:2}));l.frustumCulled=!1;let u=new Float32Array(gg*3),d=new Float32Array(gg*3),f=new Float32Array(gg),p=new Float32Array(gg),m=new Float32Array(gg),h=new Float32Array(gg),g=new Float32Array(gg),_=new Uint8Array(gg),v=new Float32Array(gg).fill(-1),y=new Float32Array(gg),b=0,x=0,S=0,C=0,w=[0,0,0];function T(e,t,n,r,i,a){let o=Math.cos(a),s=Math.sin(a),c=e[t],l=e[t+1],u=e[t+2],d=(n*c+r*l+i*u)*(1-o);e[t]=c*o+(r*u-i*l)*s+n*d,e[t+1]=l*o+(i*c-n*u)*s+r*d,e[t+2]=u*o+(n*l-r*c)*s+i*d}function E(i,l,T,E,D,O,k){let A=b;b=(b+1)%gg,A>=x&&(x=A+1),_[A]===1&&S--,_[A]===2&&C--,_[A]=1,S++,h[A]=0,g[A]=k?vg:_g;let j=Math.min(T,.26+.3*r()),M=k?Math.max(.14,Math.min(.46,(j-E)*(.7+.5*r()))):Math.max(.04,Math.min(.2,(j-E)*(.35+.5*r())));p[A]=M/2,m[A]=k?.0045+.0045*r():.0028+.0028*r();let N=A*3;if(a.array[N]=i+(r()-.5)*.12,a.array[N+1]=n(i,l)+E+M*(.3+r()),a.array[N+2]=l+(r()-.5)*.12,k){let e=(r()+r()+r()-1.5)/.5,t=(r()*2-1)*k.along,i=k.x+k.dx*t-k.dz*e*k.width,o=k.z+k.dz*t+k.dx*e*k.width;v[A]=Math.exp(-.5*e*e)*Math.min(1,(1-Math.abs(t)/k.along)*3+.25),y[A]=Math.atan2(k.dz,k.dx);let s=Math.min(.34,Math.max(.09,Math.hypot(i-a.array[N],o-a.array[N+2])/13))*(.8+.4*r());u[N]=(i-a.array[N])/s,u[N+1]=(n(i,o)+.05-a.array[N+1]+4.9*s*s)/s,u[N+2]=(o-a.array[N+2])/s}else{v[A]=-1;let e=.25+.5*r();u[N]=D*e+(r()-.5)*1.6,u[N+1]=1+2.6*r()+.12*Math.hypot(D,O),u[N+2]=O*e+(r()-.5)*1.6}let P=r()*Math.PI*2;o.array[N]=.25*Math.cos(P)*p[A],o.array[N+1]=.97*p[A],o.array[N+2]=.25*Math.sin(P)*p[A],s.array[N]=-Math.sin(P)*m[A],s.array[N+1]=0,s.array[N+2]=Math.cos(P)*m[A];let F=r()*Math.PI*2,I=(r()-.5)*.9;d[N]=Math.cos(F)*Math.cos(I),d[N+1]=Math.sin(I),d[N+2]=Math.sin(F)*Math.cos(I),f[A]=(k?3+7*r():4+12*r())*(r()<.5?-1:1);let ee=t.stateAt(i,l);xg(Math.min(1,Math.max(0,((ee.dry??.3)-.3)/.4+(r()-.5)*.4)),k?.4+.4*r():.55+.4*r(),w);let te=.88+.26*r();c.array[A*4]=w[0]*te,c.array[A*4+1]=w[1]*te,c.array[A*4+2]=w[2]*te,c.array[A*4+3]=e.uTime.value,c.needsUpdate=!0}function D(e){let t=e*3;o.array[t]=o.array[t+1]=o.array[t+2]=0,s.array[t]=s.array[t+1]=s.array[t+2]=0,_[e]===1&&S--,_[e]===2&&C--,_[e]=0}function O(e,t,i,c){let l=e*3,u=p[e],d=Math.cos(t),f=Math.sin(t),h=a.array[l],g=a.array[l+2],_=n(h-d*u,g-f*u),v=n(h+d*u,g+f*u);a.array[l+1]=(_+v)/2+.01+i;let y=(v-_)/2+c*u,b=Math.sqrt(Math.max(u*u-y*y,1e-8));o.array[l]=d*b,o.array[l+1]=y,o.array[l+2]=f*b;let x=(r()-.5)*1.2;s.array[l]=-f*m[e]*Math.cos(x),s.array[l+1]=m[e]*Math.sin(x),s.array[l+2]=d*m[e]*Math.cos(x)}return{mesh:l,get flying(){return S},take(e,n,a=null){let o=t.cutEvents,s=Math.min(a?3.2:3,Math.max(.35,900/Math.max(o.length/4,1)));for(let t=0;t<o.length;t+=4){let i=Math.floor(s)+ +(r()<s%1);for(;i-->0;)E(o[t],o[t+1],o[t+2],o[t+3],e,n,a)}o.length=0,i.instanceCount=x},clear(){for(let e=0;e<x;e++)D(e);x=0,b=0,i.instanceCount=0,o.needsUpdate=s.needsUpdate=!0},update(i){if(!S&&!C)return;let c=e.uWind.value,l=Math.exp(-2.4*i),p=!1;for(let e=0;e<x;e++){let m=_[e];if(!m)continue;let b=e*3;if(m===2){if(h[e]+=i,h[e]>g[e]){if(h[e]>g[e]+2)D(e);else for(let e=0;e<3;e++)o.array[b+e]*=1-Math.min(1,i*1.5);p=!0}continue}p=!0,h[e]+=i;let x=v[e]>=0;u[b+1]-=9.8*i,x||(u[b]=u[b]*l+c.x*c.z*.5*(1-l),u[b+1]*=l,u[b+2]=u[b+2]*l+c.y*c.z*.5*(1-l));let w=x?0:Math.sin(h[e]*9+e)*.6*i;a.array[b]+=u[b]*i+d[b+2]*w,a.array[b+1]+=u[b+1]*i,a.array[b+2]+=u[b+2]*i-d[b]*w;let E=f[e]*i;T(o.array,b,d[b],d[b+1],d[b+2],E),T(s.array,b,d[b],d[b+1],d[b+2],E);let k=n(a.array[b],a.array[b+2])+.03;if(a.array[b+1]<=k&&u[b+1]<0){let n=t.stateAt(a.array[b],a.array[b+2]);if(x){let t=y[e]+Math.PI/2+(r()+r()+r()-1.5)*1.5,n=v[e]*.15*r()**1.3;O(e,t,n,(r()-.5)*.7*v[e])}else O(e,Math.atan2(o.array[b+2],o.array[b]),Math.min(n.cut??1,.12)*.4*r(),(r()-.5)*.3);_[e]=2,S--,C++,h[e]=r()*6,(n.burning||n.burnt>.5)&&D(e)}}p&&(a.needsUpdate=o.needsUpdate=s.needsUpdate=!0)}}}var Cg={cell:.4,tile:12,tiles:6,fade:[27,35]},wg=8,Tg=`
${ng}
in vec2 aCell;
uniform float uCell;
uniform vec2 uFade;
uniform float uSalt;
uniform float uHeight;
out vec3 vWorld;
out vec3 vNormal;
out vec3 vAlbedo;
out vec3 vLight;  // cloud shade, fire light, how far up

void main() {
  float part = position.z;           // 0 stem, 1..8 a petal, 9 the middle
  float sub = floor(part / 16.0);
  part -= sub * 16.0;
  vec2 origin = modelMatrix[3].xz;
  vec2 cell = floor(origin / uCell + 0.5) + aCell;
  vec4 r = cellRand(cell, uSalt + sub * 5.0);
  vec4 q = cellRand(cell, uSalt + 53.0 + sub * 5.0);
  vec2 xz = (cell + r.xy) * uCell;
  float dist = distance(xz, uCamPos.xz);
  vec4 m = texture(tMap, extentUv(xz));
  // they grow in drifts
  float drift = smoothstep(0.42, 0.7, texture(tNoise, xz / 23.0).g);
  float chance = m.a * (0.16 + 0.84 * drift) * step(0.5, m.g);
  float live = step(q.x, chance) * (1.0 - smoothstep(uFade.x, uFade.y, dist + q.y * 3.0));
  if (live <= 0.0) {
    gl_Position = vec4(2.0, 2.0, 2.0, 1.0);
    return;
  }
  vec2 suv = stateUv(xz);
  vec4 st = texture(tState, suv);
  vec4 fr = texture(tFire, suv);
  vec2 bend = (st.rg * 255.0 - 128.0) / 127.0;
  float flat_ = min(length(bend), 1.0);
  float charred = smoothstep(0.1, 0.6, fr.g);

  // one kind to a drift, with a few strays
  float kindN = texture(tWhite, xz / (7.0 * 256.0)).r;
  if (q.z > 0.8) kindN = q.w;
  int kind = int(kindN * 3.999);
  vec3 petal = vec3(0.80, 0.80, 0.74);
  vec3 middle = vec3(0.75, 0.50, 0.03);
  float count = 8.0, pl = 0.024, pw = 0.0075, cup = 0.12, mid = 0.0095, tall = 1.0;
  if (kind == 1) { petal = vec3(0.85, 0.62, 0.02); middle = vec3(0.6, 0.45, 0.02); count = 5.0; pl = 0.012; pw = 0.011; cup = 0.6; mid = 0.004; tall = 0.85; }
  if (kind == 2) { petal = vec3(0.86, 0.34, 0.55); middle = vec3(0.7, 0.5, 0.08); count = 8.0; pl = 0.019; pw = 0.005; cup = 0.55; mid = 0.007; tall = 1.1; }
  if (kind == 3) { petal = vec3(0.93, 0.88, 0.8); middle = vec3(0.8, 0.55, 0.06); count = 4.0; pl = 0.03; pw = 0.03; cup = 0.75; mid = 0.006; tall = 1.05; }
  float size = mix(2.0, 2.7, r.z);

  float gy = groundAt(xz);
  float h = uHeight * tall * mix(0.98, 1.32, r.w) * (1.0 - 0.7 * charred) * tallAt(xz);
  float yaw = q.w * TAU;
  vec2 L = vec2(cos(yaw), sin(yaw)) * 0.18;
  float gust = gustAt(xz);
  float U = uWind.z;
  float nod = sin(r.w * 40.0 + uTime * (2.4 + 2.0 * r.z)) * (0.03 + 0.012 * U);
  L += uWind.xy * (U * mix(0.02, 0.07, gust) + nod);
  for (int i = 0; i < 6; i++) {
    vec4 p = uPushers[i];
    if (p.w <= 0.0) continue;
    vec2 d = xz - p.xz;
    float dd = length(d);
    L += d / max(dd, 1e-3) * (1.0 - smoothstep(p.w * 0.7, p.w * 1.9, dd)) * (1.0 - smoothstep(gy + h * 0.5, gy + h + 0.12, p.y - p.w)) * 0.95;
  }
  L = L * (1.0 - 0.8 * flat_) + bend * 1.42;
  float lean = length(L);
  vec2 dir = L / max(lean, 1e-4);
  lean = min(lean, 1.5);
  float theta0 = lean * mix(0.3, 0.85, flat_);
  float A = lean - theta0;
  float top = A > 1e-3 ? h * (sin(theta0 + A) - sin(theta0)) / A : h * cos(theta0);
  float sCut = clamp(st.b / max(top, 1e-3), 0.0, 1.0);
  bool headless = sCut < 0.97 || charred > 0.5;

  float s = (part < 0.5 ? position.y : 1.0) * sCut;
  float phi = theta0 + A * s;
  float along = A > 1e-3 ? h * (cos(theta0) - cos(phi)) / A : h * s * sin(theta0);
  float rise = A > 1e-3 ? h * (sin(phi) - sin(theta0)) / A : h * s * cos(theta0);
  vec3 tangent = vec3(dir.x * sin(phi), cos(phi), dir.y * sin(phi));
  vec3 at = vec3(xz.x + dir.x * along, gy + rise, xz.y + dir.y * along);
  vec3 world;
  vec3 N;
  vec3 col;
  if (part < 0.5) {
    vec3 across = normalize(cross(tangent, uCamPos - at));
    world = at + across * position.x * 0.0016;
    N = cross(across, tangent);
    col = mix(vec3(0.16, 0.4, 0.1), vec3(0.24, 0.52, 0.13), s);
  } else {
    // the head looks up and a little towards the sun
    vec3 axis = normalize(tangent * 0.6 + vec3(0.0, 0.5, 0.0) + uSunDir * 0.25);
    vec3 u = normalize(cross(axis, vec3(0.31, 0.0, 0.95)));
    vec3 v = cross(axis, u);
    if (headless) {
      world = at;
      N = axis;
      col = vec3(0.0);
    } else if (part > 8.5) {
      world = at + (u * position.x + v * position.y) * mid * size + axis * 0.002;
      N = normalize(axis + (u * position.x + v * position.y) * 0.6);
      col = middle;
    } else {
      float k = part - 1.0;
      float a = (k + r.x) / count * TAU;
      vec3 out_ = cos(a) * u + sin(a) * v;
      vec3 side = -sin(a) * u + cos(a) * v;
      float t = position.y;
      float wide = sin(3.14159 * clamp(t * 0.85 + 0.12, 0.0, 1.0));
      float droop = cup - 0.5 * t * t * (1.0 - cup);
      world = at + (out_ * cos(droop) + axis * sin(droop)) * t * pl * size + side * position.x * pw * size * wide;
      if (k >= count) world = at;
      N = normalize(axis * cos(droop) - out_ * sin(droop));
      col = petal * mix(0.8, 1.05, t);
    }
  }
  col = mix(col, vec3(0.011, 0.010, 0.009), charred);
  vWorld = world;
  vNormal = N;
  vAlbedo = col;
  vLight = vec3(cloudAt(xz), texture(tGlow, suv).g + lampAt(world), clamp(rise / (uHeight * 0.85), 0.0, 1.0));
  gl_Position = projectionMatrix * viewMatrix * vec4(world, 1.0);
}
`,Eg=`
${ng}
${rg}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
in vec3 vAlbedo;
in vec3 vLight;
void main() {
  vec3 V = normalize(uCamPos - vWorld);
  vec3 N = normalize(vNormal + vec3(0.0, 1e-4, 0.0));
  if (dot(N, V) < 0.0) N = -N;
  // petals are matt and a little see-through: lit like a leaf, but they never dazzle
  vec3 c = min(leafLight(vAlbedo, N, V, vLight.z, 0.04, vLight.x, vLight.y) * 0.62, vec3(1.25));
  outC = vec4(applyFog(c, vWorld), 1.0);
}
`;function Dg(e=2){let t=[],n=[];for(let r=0;r<e;r++){let e=r*16,i=t.length/3;for(let r=0;r<4;r++)t.push(-1,r/3,e,1,r/3,e),r<3&&n.push(i+r*2,i+r*2+1,i+r*2+3,i+r*2,i+r*2+3,i+r*2+2);for(let r=0;r<wg;r++){i=t.length/3;let a=e+1+r;t.push(0,0,a,-1,.4,a,1,.4,a,-1,.75,a,1,.75,a,0,1,a),n.push(i,i+2,i+1,i+1,i+2,i+4,i+1,i+4,i+3,i+3,i+4,i+5)}i=t.length/3,t.push(0,0,e+9);for(let n=0;n<6;n++)t.push(Math.cos(n/6*Math.PI*2),Math.sin(n/6*Math.PI*2),e+9);for(let e=0;e<6;e++)n.push(i,i+1+e,i+1+(e+1)%6)}let r=Math.round(Cg.tile/Cg.cell),i=new Float32Array(r*r*2);for(let e=0;e<r;e++)for(let t=0;t<r;t++)i[(t+e*r)*2]=t,i[(t+e*r)*2+1]=e;let a=new Du;return a.setAttribute(`position`,new X(t,3)),a.setIndex(n),a.setAttribute(`aCell`,new za(i,2)),a.instanceCount=r*r,a.boundingSphere=new Si(new q(Cg.tile/2,.3,Cg.tile/2),Cg.tile*.75+2.5),a}function Og(e,{height:t=.52,seed:n=1}={},r){let i=new gr,a=Dg(),o={...e,uCell:{value:Cg.cell},uFade:{value:new K(...Cg.fade)},uSalt:{value:400+n},uHeight:{value:t}},s=new Qc({glslVersion:yt,vertexShader:Tg,fragmentShader:Eg,uniforms:o,side:2}),c=[];for(let e=0;e<Cg.tiles;e++)for(let t=0;t<Cg.tiles;t++){let n=new xa(a,s);n.matrixAutoUpdate=!1,c.push({mesh:n,i:t-Cg.tiles/2,j:e-Cg.tiles/2}),i.add(n)}let l=[NaN,NaN];return{group:i,uniforms:o,follow(e,t){let n=Math.floor(e/Cg.tile),i=Math.floor(t/Cg.tile);if(n!==l[0]||i!==l[1]){l=[n,i];for(let e of c){let t=(n+e.i)*Cg.tile,a=(i+e.j)*Cg.tile;e.mesh.position.set(t,r(t+Cg.tile/2,a+Cg.tile/2),a),e.mesh.updateMatrix(),e.mesh.updateMatrixWorld(!0)}}}}}var kg=`
${ng}
uniform vec2 uGroundOrigin;
out vec3 vWorld;
out vec3 vNormal;
void main() {
  vec2 xz = uGroundOrigin + position.xz;
  float h = groundAt(xz);
  vNormal = normalize(vec3(groundAt(xz - vec2(1.0, 0.0)) - groundAt(xz + vec2(1.0, 0.0)), 2.0, groundAt(xz - vec2(0.0, 1.0)) - groundAt(xz + vec2(0.0, 1.0))));
  vWorld = vec3(xz.x, h, xz.y);
  gl_Position = projectionMatrix * viewMatrix * vec4(vWorld, 1.0);
}
`,Ag=`
${ng}
${rg}
${ig}
layout(location = 0) out vec4 outC;
uniform sampler2D tSoil;
uniform sampler2D tSoilN;
uniform sampler2D tDirt;
uniform sampler2D tDirtN;
uniform float uHeight;
in vec3 vWorld;
in vec3 vNormal;

void main() {
  vec2 uvE = extentUv(vWorld.xz);
  if (uvE.x < 0.0 || uvE.y < 0.0 || uvE.x > 1.0 || uvE.y > 1.0) discard;
  vec4 m = texture(tMap, uvE);
  vec2 suv = stateUv(vWorld.xz);
  vec4 st = texture(tState, suv);
  vec4 fr = texture(tFire, suv);
  float dist = distance(vWorld, uCamPos);
  vec3 V = normalize(uCamPos - vWorld);

  // two sizes of the same soil, mixed by a slow noise, so no repeat shows
  vec2 u1 = vWorld.xz / 1.9;
  vec2 u2 = vWorld.xz.yx / 5.7 + 0.37;
  float blend = smoothstep(0.3, 0.7, texture(tNoise, vWorld.xz / 11.0).a);
  vec3 soil = mix(texture(tSoil, u1).rgb, texture(tSoil, u2).rgb, 0.3 + 0.4 * blend) * 0.85;
  vec3 dirt = mix(texture(tDirt, u1 * 0.8).rgb, texture(tDirt, u2).rgb, 0.3 + 0.4 * blend);
  float bare = 1.0 - smoothstep(0.12, 0.5, m.g + (texture(tNoise, vWorld.xz / 2.3).r - 0.5) * 0.2);
  vec3 albedo = mix(soil, vec3(0.5, 0.43, 0.27) * (0.85 + 0.3 * texture(tStrokes, vWorld.xz / 9.0).r), bare);
  vec3 nt = mix(texture(tSoilN, u1).xyz, texture(tDirtN, u1 * 0.8).xyz, bare) * 2.0 - 1.0;
  float detail = 1.0 - smoothstep(12.0, 40.0, dist);
  vec3 N = normalize(vNormal + vec3(nt.x, 0.0, nt.y) * 0.2 * detail);
  albedo *= mix(1.0, 0.5, m.b);

  float charred = smoothstep(0.1, 0.8, fr.g);
  float standing = m.g * smoothstep(0.05, 0.3, st.b) * (1.0 - charred);
  float mown = m.g * (1.0 - smoothstep(0.08, 0.3, st.b)) * (1.0 - charred);
  // under the grass the soil is mostly hidden: last year's dead blades lie on it, and low green things grow between
  float under = texture(tNoise, vWorld.xz / 0.37).r;
  float low = texture(tNoise, vWorld.xz / 1.1 + 0.6).b;
  vec3 litter = mix(vec3(0.026, 0.085, 0.03), vec3(0.018, 0.06, 0.034), smoothstep(0.4, 0.62, low)) * (0.7 + 0.6 * under);
  albedo = mix(albedo, litter, m.g * (1.0 - bare));
  // mown, it is a mat of stubble and clippings
  albedo = mix(albedo, mix(vec3(0.075, 0.105, 0.028), vec3(0.2, 0.17, 0.07), smoothstep(0.3, 0.7, m.r)) * (0.7 + 0.6 * texture(tNoise, vWorld.xz / 0.7).b), mown * 0.82);
  float fleck = hash12(floor(vWorld.xz * 70.0));
  float ashN = texture(tNoise, vWorld.xz / 3.7).g * 0.6 + texture(tNoise, vWorld.xz.yx / 0.83 + 0.3).a * 0.4;
  vec3 ash = mix(vec3(0.010, 0.009, 0.009), vec3(0.12, 0.115, 0.11), smoothstep(0.42, 0.72, ashN) * 0.5 + step(0.975, fleck) * 0.5);
  albedo = mix(albedo, ash, charred);

  float cloud = cloudAt(vWorld.xz);
  float glow = texture(tGlow, suv).g + lampAt(vWorld) * mix(1.0, 0.35, standing);
  float shade = mix(1.0, 0.3, standing);
  vec3 c = albedo * (uSunColor * cloud * max(dot(N, uSunDir), 0.0) * shade + mix(uBounceColor, uSkyColor, N.y * 0.5 + 0.5) * mix(1.0, 0.4, standing));
  c += albedo * uFireColor * glow;
  // embers left in the ash, and the ground lit from just above while it burns
  // a few points of it still glow, each round, each breathing in its own time
  vec2 sp = vWorld.xz * 38.0;
  vec2 sc = floor(sp);
  float sr = hash12(sc);
  vec2 off = sp - sc - 0.5 - (vec2(hash12(sc + 3.0), hash12(sc + 5.0)) - 0.5) * 0.5;
  float spark = step(0.955, sr) * exp(-dot(off, off) * 22.0) * (0.35 + 0.65 * sin(uTime * (1.0 + 2.5 * hash12(sc + 7.0)) + sr * 60.0)) * detail;
  c += vec3(2.2, 0.42, 0.03) * fr.b * (max(spark, 0.0) * 2.4 + 0.025);
  c += fireRamp(0.55) * fr.r * 0.5;

  // beyond the last blades the ground is the grass
  float farK = max(m.g, 0.88 * smoothstep(50.0, 200.0, dist)) * (1.0 - charred);
  if (farK > 0.0) {
    float gust = gustAt(vWorld.xz);
    float U = uWind.z;
    float dry = smoothstep(0.3, 0.7, m.r) * (1.0 - 0.5 * m.b);
    float rel = slopeRel(landNormal(vWorld.xz));
    vec3 g = meadowGreen(0.6 + 0.1 * gust * min(U / 6.0, 1.0) + (rel - 0.5) * mix(0.6, 0.4, smoothstep(150.0, 450.0, dist))) * mix(vec3(1.0), meadowTint(vWorld.xz), 0.5 * (1.0 - smoothstep(40.0, 110.0, dist)));
    g = mix(g, vec3(0.44, 0.47, 0.17), dry * 0.45);
    // seen from off, a meadow is not one green: darker tufts stand in it, and where flowers grow it is dusted
    // with their colour
    float tuft = texture(tNoise, vWorld.xz / 3.1).g * 0.6 + texture(tNoise, vWorld.xz / 0.83 + 0.2).r * 0.4;
    g *= mix(1.0, 0.76, smoothstep(0.52, 0.62, tuft) * smoothstep(9.0, 30.0, dist) * (1.0 - smoothstep(50.0, 130.0, dist)));
    float drift = smoothstep(0.42, 0.7, texture(tNoise, vWorld.xz / 23.0).g);
    float away = smoothstep(30.0, 110.0, dist);
    float dust = m.a * (0.16 + 0.84 * drift) * smoothstep(0.5, 0.62, mix(texture(tNoise, vWorld.xz / 0.6).a, 0.54, away) + 0.2 * drift);
    float kn = texture(tWhite, vWorld.xz / (7.0 * 256.0)).r * 3.999;
    vec3 heads = kn < 1.0 ? vec3(0.8, 0.8, 0.74) : kn < 2.0 ? vec3(0.85, 0.62, 0.02) : kn < 3.0 ? vec3(0.86, 0.34, 0.55) : vec3(0.93, 0.88, 0.8);
    // (one kind to a drift near by; far off, where a drift is a few pixels, the colour of them all together)
    heads = mix(heads, vec3(0.88, 0.76, 0.7), smoothstep(60.0, 180.0, dist));
    g = mix(g, heads, dust * 0.5 * smoothstep(20.0, 34.0, dist) * (1.0 - smoothstep(80.0, 170.0, dist)));
    // past the meadow it is farmland, as it is painted
    vec4 fl = floorAt(vWorld.xz);
    g = mix(g, fl.rgb, fl.a);
    g = paintGround(g, vWorld.xz, dist, 0.0);
    float sx = strokeCoord(vWorld);
    g = landStrokes(g, vWorld.xz, dist, sx, fwidth(sx));
    float sy = strokeCoord2(vWorld.xz);
    g = landStrokes(g, vWorld.xz, dist, sy, fwidth(sy));
    vec2 hq = heroAngle(vWorld);
    g = heroPaint(g, vWorld, hq, fwidth(hq), dist);
    g = mix(g, mix(vec3(0.1, 0.14, 0.035), vec3(0.24, 0.2, 0.085), dry), mown / max(m.g, 1e-3) * 0.8);
    vec3 Ng = normalize(V * 0.75 + vec3(0.0, 0.6, 0.0));
    vec3 lit = g * meadowLight(cloud * slopeSun(rel)) + g * uFireColor * glow * 0.5;
    // nearer, between the blades that are drawn, it is the dim green under them
    c = mix(c, lit * mix(0.78, 1.0, smoothstep(8.0, 60.0, dist)), farK);
  }
  outC = vec4(applyFog(c, vWorld), mix(1.0, 0.4, smoothstep(120.0, 200.0, dist)));
}
`;function jg(e=80,t=60,n=1.06){let r=[0];for(let t=1;t<=e;t++)r.push(t);let i=1;for(let e=0;e<t;e++)i*=n,r.push(r[r.length-1]+i);let a=[...r.slice(1).reverse().map(e=>-e),...r],o=a.length,s=new Float32Array(o*o*3);for(let e=0;e<o;e++)for(let t=0;t<o;t++)s[(t+e*o)*3]=a[t],s[(t+e*o)*3+2]=a[e];let c=new Uint32Array((o-1)*(o-1)*6),l=0;for(let e=0;e<o-1;e++)for(let t=0;t<o-1;t++){let n=t+e*o;c[l++]=n,c[l++]=n+o,c[l++]=n+1,c[l++]=n+1,c[l++]=n+o,c[l++]=n+o+1}let u=new Z;return u.setAttribute(`position`,new ui(s,3)),u.setIndex(new ui(c,1)),u}function Mg(e,t,{height:n=.52,extentMin:r,texel:i}){let a={...e,uGroundOrigin:{value:new K},tSoil:{value:t.soil},tSoilN:{value:t.soilNormal},tDirt:{value:t.dirt},tDirtN:{value:t.dirtNormal},uHeight:{value:n}},o=new Qc({glslVersion:yt,vertexShader:kg,fragmentShader:Ag,uniforms:a}),s=new xa(jg(),o);return s.frustumCulled=!1,{mesh:s,uniforms:a,follow(e,t){a.uGroundOrigin.value.set(r[0]+(Math.floor((e-r[0])/i)+.5)*i,r[1]+(Math.floor((t-r[1])/i)+.5)*i)}}}function Ng(){let e=(e,t,n,r)=>{let i=new Fa(new Uint8Array([e,t,n,255]),1,1);return i.wrapS=i.wrapT=s,r&&(i.colorSpace=$e),i.needsUpdate=!0,i};return{soil:e(70,48,34,!0),soilNormal:e(128,128,255),dirt:e(120,100,72,!0),dirtNormal:e(128,128,255)}}async function Pg(e,t=`./textures/`){let n=new Ql,r=async(r,i)=>{let a=await n.loadAsync(t+r);return a.wrapS=a.wrapT=s,a.anisotropy=Math.min(8,e.capabilities.getMaxAnisotropy()),i&&(a.colorSpace=$e),a},[i,a,o,c]=await Promise.all([r(`Ground048_Color.jpg`,!0),r(`Ground048_NormalGL.jpg`),r(`Ground103_Color.jpg`,!0),r(`Ground103_NormalGL.jpg`)]);return{soil:i,soilNormal:a,dirt:o,dirtNormal:c}}var Fg=255,Ig=0,Lg=1,Rg=2,zg=3,Bg=9,Vg=1/30,Hg=(e,t,n)=>{let r=Math.min(1,Math.max(0,(n-e)/(t-e)));return r*r*(3-2*r)};function Ug(e,t,n=h){let r=new Fa(e,t,t,F,b);return r.wrapS=r.wrapT=c,r.minFilter=r.magFilter=n,r.generateMipmaps=!1,r.needsUpdate=!0,r}var Wg=class{constructor(e,t){this.tex=e,this.n=t,this.rects=[],this.rows=0,this.area=0,this.all=!1}add(e,t,n,r){this.all||(this.rects.push(e,t,n,r),this.rows+=r-t+1,this.area+=(r-t+1)*(n-e+1),this.area>this.n*this.n*.6&&(this.all=!0))}flush(e){if(!this.all&&!this.rects.length)return;let{tex:t,n}=this;t.clearUpdateRanges();let r=e?.properties.get(t).__webglTexture;if(r&&!this.all&&t.version>0&&!t.needsUpdate){let i=e.getContext(),a=e.state;a.activeTexture(i.TEXTURE0),a.bindTexture(i.TEXTURE_2D,r),a.pixelStorei(i.UNPACK_FLIP_Y_WEBGL,!1),a.pixelStorei(i.UNPACK_PREMULTIPLY_ALPHA_WEBGL,!1),a.pixelStorei(i.UNPACK_ALIGNMENT,4),a.pixelStorei(i.UNPACK_ROW_LENGTH,n);for(let e=0;e<this.rects.length;e+=4){let n=this.rects[e],r=this.rects[e+1];a.pixelStorei(i.UNPACK_SKIP_PIXELS,n),a.pixelStorei(i.UNPACK_SKIP_ROWS,r),i.texSubImage2D(i.TEXTURE_2D,0,n,r,this.rects[e+2]-n+1,this.rects[e+3]-r+1,i.RGBA,i.UNSIGNED_BYTE,t.image.data)}a.pixelStorei(i.UNPACK_ROW_LENGTH,0),a.pixelStorei(i.UNPACK_SKIP_PIXELS,0),a.pixelStorei(i.UNPACK_SKIP_ROWS,0),this.rects.length=0,this.rows=0,this.area=0}else{if(this.rows>420&&(this.all=!0),!this.all)for(let e=0;e<this.rects.length;e+=4){let r=this.rects[e],i=this.rects[e+2]-r+1;for(let a=this.rects[e+1];a<=this.rects[e+3];a++)t.addUpdateRange((a*n+r)*4,i*4)}t.needsUpdate=!0,this.rects.length=0,this.rows=0,this.area=0,this.all=!1}}},Gg=class{constructor({size:e=256,centre:t=[0,0],heightAt:n,growthAt:r,dryAt:i,wetAt:a,recover:o=70,regrow:s=0}){this.size=e,this.minX=t[0]-e/2,this.minZ=t[1]-e/2,this.heightAt=n,this.recover=o,this.regrow=s,this.N=Math.round(e/.125),this.cell=e/this.N,this.data=new Uint8Array(this.N*this.N*4),this.tex=Ug(this.data,this.N),this.dirty=new Wg(this.tex,this.N),this.scanRow=0,this.cutEvents=[],this.M=this.N/2,this.fcell=e/this.M;let c=this.M*this.M;this.fdata=new Uint8Array(c*4),this.ftex=Ug(this.fdata,this.M),this.fdirty=new Wg(this.ftex,this.M),this.fState=new Uint8Array(c),this.fAge=new Float32Array(c),this.fAcc=new Float32Array(c),this.fPeak=new Float32Array(c),this.fDur=new Float32Array(c),this.fGrow=new Uint8Array(c),this.fDry=new Uint8Array(c),this.fWet=new Uint8Array(c),this.fRand=new Uint8Array(c),this.fDone=new Uint8Array(c),this.active=new Int32Array(c),this.activeCount=0,this.burning=0,this.burntCells=0,this.acc=0,this.time=0,this.fireBox={x0:0,z0:0,x1:0,z1:0,y0:0,y1:0},this.G=Math.round(e/2),this.gcell=e/this.G,this.gsum=new Float32Array(this.G*this.G),this.gtmp=new Float32Array(this.G*this.G),this.gwide=new Float32Array(this.G*this.G),this.gdata=new Uint8Array(this.G*this.G*4),this.gtex=Ug(this.gdata,this.G),this.glowLive=!1;let l=1234567;for(let e=0;e<this.M;e++)for(let t=0;t<this.M;t++){let n=t+e*this.M,o=this.minX+(t+.5)*this.fcell,s=this.minZ+(e+.5)*this.fcell;this.fGrow[n]=Math.round(255*Math.min(1,Math.max(0,r(o,s)))),this.fDry[n]=Math.round(255*Math.min(1,Math.max(0,i(o,s)))),this.fWet[n]=Math.round(255*Math.min(1,Math.max(0,a(o,s)))),l=Math.imul(l,1664525)+1013904223>>>0,this.fRand[n]=l>>>24}this.kernel=[],this.setWind(1,0,0),this.reset()}reset(){let e=this.data;for(let t=0;t<e.length;t+=4)e[t]=128,e[t+1]=128,e[t+2]=Fg,e[t+3]=255;this.dirty.all=!0,this.fdata.fill(0),this.fdirty.all=!0,this.fState.fill(Ig),this.fAge.fill(0),this.fAcc.fill(0),this.fDone.fill(0),this.activeCount=0,this.burning=0,this.burntCells=0,this.gdata.fill(0),this.gtex.needsUpdate=!0,this.glowLive=!1,this.cutEvents.length=0}setWind(e,t,n){this.windX=e,this.windZ=t,this.windSpeed=n;let r=[];for(let i=-2;i<=2;i++)for(let a=-2;a<=2;a++){if(!a&&!i)continue;let o=Math.hypot(a,i),s=(a*e+i*t)/o,c;if(o<1.5){let e=s>0?1+1.05*n*s*s:1/(1+.55*n*-s);c=.55/o*e}else c=n>1.5&&s>.55?.085*(n-1.5)*s**4/o:0;c>0&&r.push(a+i*this.M,c)}this.kernel=r}inside(e,t,n=0){return e>this.minX+n&&t>this.minZ+n&&e<this.minX+this.size-n&&t<this.minZ+this.size-n}bend(e,t,n=.3,r=0,i=0,a=1){let{N:o,cell:s,data:c}=this,l=(e-this.minX)/s-.5,u=(t-this.minZ)/s-.5,d=n/s,f=Math.max(2,Math.floor(l-d)),p=Math.min(o-3,Math.ceil(l+d)),m=Math.max(2,Math.floor(u-d)),h=Math.min(o-3,Math.ceil(u+d));if(f>p||m>h)return;let g=Math.hypot(r,i),_=g<1e-6,v=_?0:r/g,y=_?0:i/g;for(let e=m;e<=h;e++)for(let t=f;t<=p;t++){let n=t-l,r=e-u,i=Math.hypot(n,r)/d;if(i>=1)continue;let s=(1-i*i)**1.5*a,f=v,p=y;if(_){let e=Math.hypot(n,r)||1;f=n/e,p=r/e}let m=(t+e*o)*4,h=(c[m]-128)/127,g=(c[m+1]-128)/127,b=h+(f-h)*s,x=g+(p-g)*s;c[m]=128+Math.round(Math.max(-1,Math.min(1,b))*127),c[m+1]=128+Math.round(Math.max(-1,Math.min(1,x))*127)}this.dirty.add(f,m,p,h)}cutLine(e,t,n,r,i=.2,a=.07,o=a){let{N:s,cell:c,data:l}=this,u=i/2/c,d=(e-this.minX)/c-.5,f=(t-this.minZ)/c-.5,p=(n-this.minX)/c-.5,m=(r-this.minZ)/c-.5,h=Math.max(2,Math.floor(Math.min(d,p)-u)),g=Math.min(s-3,Math.ceil(Math.max(d,p)+u)),_=Math.max(2,Math.floor(Math.min(f,m)-u)),v=Math.min(s-3,Math.ceil(Math.max(f,m)+u));if(h>g||_>v)return 0;let y=p-d,b=m-f,x=y*y+b*b||1e-9,S=0;for(let e=_;e<=v;e++)for(let t=h;t<=g;t++){let n=Math.min(1,Math.max(0,((t-d)*y+(e-f)*b)/x)),r=t-(d+y*n),i=e-(f+b*n);if(r*r+i*i>u*u)continue;let p=(t+e*s)*4+2,m=Math.max(1,Math.min(254,Math.round((a+(o-a)*n)*255))),h=l[p];m>=h||(l[p]=m,h-m>20&&this.fState[(t>>1)+(e>>1)*this.M]===Ig&&(this.cutEvents.push(this.minX+(t+.5)*c,this.minZ+(e+.5)*c,h/255,m/255),S++))}return this.dirty.add(h,_,g,v),S}cut(e,t,n=.3,r=.07){return this.cutLine(e,t,e,t,n*2,r)}fuel(e){if(this.fState[e]!==Ig||this.fWet[e]>140)return 0;let t=e%this.M*2,n=Math.floor(e/this.M)*2,r=this.data,i=this.N,a=(r[(t+n*i)*4+2]+r[(t+1+n*i)*4+2]+r[(t+(n+1)*i)*4+2]+r[(t+1+(n+1)*i)*4+2])/1020;return this.fGrow[e]/255*Hg(.08,.26,a)*(1-this.fWet[e]/280)}light(e,t=!1){let n=this.fuel(e);if(n<(t?.06:.22))return this.fAcc[e]=0,!1;let r=this.fDry[e]/255,i=this.fRand[e]/255;return this.fState[e]=Lg,this.fAge[e]=0,this.fPeak[e]=Math.min(1,(.42+.58*n)*(.72+.4*r)),this.fDur[e]=(2+2.6*n)*(.75+.5*i),this.active[this.activeCount++]=e,!0}ignite(e,t,n=.3){let{M:r,fcell:i}=this,a=(e-this.minX)/i-.5,o=(t-this.minZ)/i-.5,s=Math.max(n/i,.75),c=0;for(let e=Math.max(3,Math.floor(o-s));e<=Math.min(r-4,Math.ceil(o+s));e++)for(let t=Math.max(3,Math.floor(a-s));t<=Math.min(r-4,Math.ceil(a+s));t++)(t-a)**2+(e-o)**2>s*s||this.light(t+e*r,!0)&&c++;return c}stateAt(e,t){if(!this.inside(e,t))return{inside:!1,burning:!1,heat:0,burnt:0,cut:1,flat:0,fuel:0};let n=Math.min(this.N-1,Math.floor((e-this.minX)/this.cell)),r=Math.min(this.N-1,Math.floor((t-this.minZ)/this.cell)),i=(n+r*this.N)*4,a=(n>>1)+(r>>1)*this.M,o=this.data;return{inside:!0,burning:this.fState[a]===Lg,heat:this.fdata[a*4]/255,burnt:this.fdata[a*4+1]/255,cut:o[i+2]/255,flat:Math.min(1,Math.hypot(o[i]-128,o[i+1]-128)/127),fuel:this.fuel(a),dry:this.fDry[a]/255,wet:this.fWet[a]/255,growth:this.fGrow[a]/255}}sample(e){for(let t=0;t<6&&this.activeCount;t++){let t=this.active[Math.floor(e()*this.activeCount)],n=this.fdata[t*4]/255,r=this.fdata[t*4+2]/255;if(n<.12&&r<.3)continue;let i=t%this.M,a=Math.floor(t/this.M);return[this.minX+(i+e())*this.fcell,this.minZ+(a+e())*this.fcell,n,r]}return null}stepFire(e){let{M:t,fState:n,fAge:r,fAcc:i,fPeak:a,fDur:o,fdata:s,fDry:c,fRand:l,active:u,kernel:d}=this,f=t,p=-1,m=t,h=-1,g=0,_=this.gsum;_.fill(0);let v=this.G,y=t/v;for(let b=this.activeCount-1;b>=0;b--){let x=u[b],S=x%t,C=(x-S)/t;S<f&&(f=S),S>p&&(p=S),C<m&&(m=C),C>h&&(h=C);let w=r[x]+=e,T=x*4;if(n[x]===Lg){let u=o[x];if(w>=u){n[x]=Rg,r[x]=0,s[T]=0,s[T+1]=255,s[T+2]=255,this.burntCells++;continue}let f=a[x]*Hg(0,.14,w)*(1-Hg(u*.45,u,w));if(s[T]=Math.round(f*255),s[T+1]=Math.round(255*Hg(.1,u*.55,w)),s[T+2]=Math.round(200*Hg(u*.4,u,w)),g++,_[Math.floor(S/y)+Math.floor(C/y)*v]+=f,S<3||C<3||S>t-4||C>t-4||this.fDone[x])continue;let p=f*e,m=0;for(let e=0;e<d.length;e+=2){let t=x+d[e];n[t]===Ig&&(m++,(i[t]+=p*d[e+1])*(.5+c[t]/255)>.62+l[t]/300&&this.light(t))}m||(this.fDone[x]=1)}else{if(w>=Bg){n[x]=zg,s[T+2]=0,u[b]=u[--this.activeCount];continue}let e=1-w/Bg;s[T+2]=Math.round(255*e*e)}}if(this.burning=g,p>=f&&this.fdirty.add(f,m,p,h),g){let e=this.fireBox;e.x0=this.minX+f*this.fcell,e.x1=this.minX+(p+1)*this.fcell,e.z0=this.minZ+m*this.fcell,e.z1=this.minZ+(h+1)*this.fcell;let n=1/0,r=-1/0,i=Math.max(1,Math.floor(this.activeCount/96));for(let e=0;e<this.activeCount;e+=i){let i=u[e],a=this.heightAt(this.minX+(i%t+.5)*this.fcell,this.minZ+(Math.floor(i/t)+.5)*this.fcell);a<n&&(n=a),a>r&&(r=a)}e.y0=n-.6,e.y1=r+.6}this.stepGlow(g>0)}blur(e,t){let n=this.G;for(let r=1;r<n-1;r++)for(let i=1;i<n-1;i++){let a=i+r*n;t[a]=(e[a]*4+(e[a-1]+e[a+1]+e[a-n]+e[a+n])*2+e[a-n-1]+e[a-n+1]+e[a+n-1]+e[a+n+1])/16}}stepGlow(e){if(!e&&!this.glowLive)return;let{G:t,gsum:n,gtmp:r,gwide:i,gdata:a}=this;for(let e=1;e<t-1;e++)for(let r=1;r<t-1;r++){let i=r+e*t,o=0;for(let e=-1;e<=1&&!o;e++)for(let r=-1;r<=1;r++)n[i+r+e*t]>0&&(o=1);a[i*4]=o?255:0}this.blur(n,r),this.blur(r,n),i.set(n);for(let e=0;e<3;e++)this.blur(i,r),this.blur(r,i);for(let e=0;e<t*t;e++)a[e*4+1]=Math.min(255,Math.round((n[e]*.055+i[e]*.11)*255));this.gtex.needsUpdate=!0,this.glowLive=e}scan(e){let{N:t,data:n}=this,r=1.5,i=Math.max(1,Math.round(t*e/r)),a=this.recover>0?.5**(r/this.recover):1,o=this.regrow>0?255*r/this.regrow:0;for(let e=0;e<i;e++){let e=this.scanRow;this.scanRow=(this.scanRow+1)%t;let r=t,i=-1,s=e*t*4;for(let e=0;e<t;e++,s+=4){let t=n[s],c=n[s+1],l=!1;if(a<1&&(t!==128||c!==128)&&(n[s]=128+Math.trunc((t-128)*a),n[s+1]=128+Math.trunc((c-128)*a),l=!0),o>0&&n[s+2]<Fg){let e=Math.floor(o)+ +(Math.random()<o%1);e&&(n[s+2]=Math.min(Fg,n[s+2]+e),l=!0)}l&&(e<r&&(r=e),i=e)}i>=0&&this.dirty.add(r,e,i,e)}}update(e,t){for(this.time+=e,this.acc=Math.min(this.acc+e,Vg*4);this.acc>=Vg;)this.acc-=Vg,this.activeCount?this.stepFire(Vg):this.glowLive&&(this.gsum.fill(0),this.stepGlow(!1));this.activeCount||(this.burning=0),this.scan(e),this.dirty.flush(t),this.fdirty.flush(t)}},Kg=e=>Math.min(1,Math.max(0,e));function qg({heightAt:e,paint:t,extentMin:n,texel:r,extentRes:i},a,o,s,c){for(let l=s;l<c;l++)for(let s=0;s<i;s++){let c=n[0]+(s+.5)*r,u=n[1]+(l+.5)*r,d=s+l*i;a[d]=e(c,u);let f=t(c,u);o[d*4]=Math.round(255*Kg(f.dry??.3)),o[d*4+1]=Math.round(255*Kg(f.growth??1)),o[d*4+2]=Math.round(255*Kg(f.wet??0)),o[d*4+3]=Math.round(255*Kg(f.flowers??0))}}var Jg=`
out vec2 vUv;
void main() {
  vUv = position.xy * 0.5 + 0.5;
  gl_Position = vec4(position.xy, 0.0, 1.0);
}
`,Yg=`
${ng}
${ig}
precision highp sampler3D;
layout(location = 0) out vec4 outC;
in vec2 vUv;
uniform sampler2D tDepth;
uniform sampler3D tVol;
uniform vec3 uCamRight;
uniform vec3 uCamUp;
uniform vec3 uCamFwd;
uniform vec2 uTan;
uniform mat4 uProj;
uniform vec4 uBox;     // x0, z0, x1, z1 of everything alight, or x1 < x0 for nothing
uniform vec2 uBoxY;    // the ground under it, lowest and highest
uniform float uFlameH; // metres, a tall flame
uniform vec4 uTorch;   // a flame held in the hand: its foot xyz and its height, 0 for none

float vol(vec3 p) { return texture(tVol, p).r; }

// density and temperature of the grass fire at p, hag metres above the ground
vec2 flame(vec3 p, float hag) {
  float U = uWind.z;
  // flames lean with the wind, more the higher they reach, and wander as they rise
  vec2 q = p.xz - uWind.xy * min(U * 0.15, 1.4) * hag * (0.45 + 0.4 * hag);
  float w1 = vol(vec3(q.x * 0.19, hag * 0.15 - uTime * 0.33, q.y * 0.19));
  float w2 = vol(vec3(q.y * 0.19 + 0.5, hag * 0.15 - uTime * 0.29, q.x * 0.19 + 0.25));
  q += (vec2(w1, w2) - 0.5) * 0.85 * hag;
  float heat = texture(tFire, stateUv(q)).r;
  if (heat < 0.012) return vec2(0.0);
  float n2 = vol(vec3(q.x * 0.47, hag * 0.24 - uTime * 0.66, q.y * 0.47));
  float n3 = vol(vec3(q.x * 1.55 + 0.3, hag * 0.72 - uTime * 2.2, q.y * 1.55));
  float tall = uFlameH * (0.2 + heat) * (0.25 + 1.5 * n2) * (1.0 + 0.05 * U);
  float u = hag / max(tall, 0.05);
  float f = heat * 1.3 * (1.0 - u) + (n3 - 0.5) * (0.45 + 0.6 * u) - 0.16 - 0.1 * u;
  if (f <= 0.0) return vec2(0.0);
  // nothing loose above the flames: scraps that small only show as speckle
  float dens = smoothstep(0.0, 0.22, f) * smoothstep(-0.04, 0.1, hag) * (1.0 - smoothstep(0.85, 1.2, u));
  float T = 0.16 + 0.72 * f + 0.3 * heat * (1.0 - u) * (1.0 - u) + 0.36 * (n3 - 0.5);
  return vec2(dens, T);
}

// the hand torch: one teardrop of flame
vec2 torch(vec3 l) {
  float y = l.y;
  if (y < -0.05 || y > 1.0) return vec2(0.0);
  float n = vol(vec3(l.x * 0.5, y * 0.3 - uTime * 0.85, l.z * 0.5));
  float n3 = vol(vec3(l.x * 1.7 + 0.4, y * 0.8 - uTime * 2.6, l.z * 1.7));
  vec2 axis = (vec2(n, n3) - 0.5) * 0.3 * y + uWind.xy * min(uWind.z * 0.03, 0.3) * y * y;
  float rr = length(l.xz - axis);
  float radius = 0.17 * (1.0 - y) * sqrt(max(1.0 - y, 0.0)) * (1.0 + 1.6 * y) + 0.02;
  float f = radius - rr + (n3 - 0.5) * 0.13;
  if (f <= 0.0) return vec2(0.0);
  return vec2(smoothstep(0.0, 0.05, f), 0.42 + 2.4 * f + 0.25 * (1.0 - y));
}

void main() {
  vec2 ndc = vUv * 2.0 - 1.0;
  vec3 rd = normalize(uCamFwd + uCamRight * ndc.x * uTan.x + uCamUp * ndc.y * uTan.y);
  float z = texture(tDepth, vUv).r * 2.0 - 1.0;
  float tMax = z >= 0.99999 ? 1e5 : (uProj[3][2] / (z + uProj[2][2])) / dot(rd, uCamFwd);
  vec3 ro = uCamPos;
  vec3 col = vec3(0.0);
  float Tr = 1.0;
  float jitter = fract(52.9829189 * fract(dot(gl_FragCoord.xy, vec2(0.06711056, 0.00583715))) + fract(uTime * 5.3) * 0.618);

  if (uTorch.w > 0.0) {
    vec3 c = uTorch.xyz + vec3(0.0, uTorch.w * 0.5, 0.0);
    float R = uTorch.w * 0.62;
    vec3 oc = ro - c;
    float b = dot(oc, rd);
    float disc = b * b - (dot(oc, oc) - R * R);
    if (disc > 0.0) {
      float sq = sqrt(disc);
      float ta = max(-b - sq, 0.0);
      float tb = min(-b + sq, tMax);
      if (tb > ta) {
        float dt = (tb - ta) / 16.0;
        float t = ta + dt * jitter;
        for (int i = 0; i < 16; i++) {
          vec2 f = torch((ro + rd * t - uTorch.xyz) / uTorch.w);
          if (f.x > 0.0) {
            float a = 1.0 - exp(-f.x * dt / uTorch.w * 26.0);
            col += Tr * a * fireRamp(f.y) * 1.2;
            Tr *= 1.0 - a;
          }
          t += dt;
        }
      }
    }
  }

  if (uBox.z > uBox.x) {
    float topH = uFlameH * 2.6;
    vec3 lo = vec3(uBox.x - 3.0, uBoxY.x - 0.3, uBox.y - 3.0);
    vec3 hi = vec3(uBox.z + 3.0, uBoxY.y + topH, uBox.w + 3.0);
    vec3 inv = 1.0 / (rd + vec3(1e-6) * sign(rd + 1e-9));
    vec3 a0 = (lo - ro) * inv;
    vec3 a1 = (hi - ro) * inv;
    vec3 tn = min(a0, a1);
    vec3 tf = max(a0, a1);
    float t0 = max(max(tn.x, tn.y), max(tn.z, 0.0));
    float t1 = min(min(min(tf.x, tf.y), tf.z), min(tMax, 280.0));
    if (t1 > t0) {
      float t = t0 + max(0.045, t0 * 0.011) * jitter;
      for (int i = 0; i < 150; i++) {
        if (t >= t1 || Tr < 0.02) break;
        vec3 p = ro + rd * t;
        float ds = max(0.045, t * 0.011);
        // 2 m cells say whether any fire is near: step straight over the ones with none
        if (texture(tGlow, stateUv(p.xz)).r < 0.004) { t += max(ds, 0.8); continue; }
        float hag = p.y - groundAt(p.xz);
        if (hag > topH) {
          if (rd.y > -1e-3) break;
          t += max(ds, (hag - topH) / -rd.y * 0.9);
          continue;
        }
        if (hag < -0.2) {
          if (rd.y < 1e-3) break;
          t += max(ds, (-0.2 - hag) / rd.y * 0.9);
          continue;
        }
        vec2 f = flame(p, hag);
        if (f.x > 0.0) {
          float a = 1.0 - exp(-f.x * min(ds, t1 - t) * 3.4);
          col += Tr * a * fireRamp(f.y) * 1.5;
          Tr *= 1.0 - a;
          t += ds * (0.88 + 0.24 * fract(jitter + float(i) * 0.61803399));
        } else {
          t += max(ds * 1.6, 0.2);
        }
      }
    }
  }
  outC = vec4(col, 1.0 - Tr);
}
`,Xg=`
${ng}
in vec4 aPos;    // centre, size
in vec4 aLook;   // turn, opacity, warmth (lit by fire from below), seed
uniform vec3 uCamRight;
uniform vec3 uCamUp;
uniform vec3 uCamFwd;
uniform vec2 uTan;
uniform vec2 uRes;
uniform float uMinPx;   // never drawn smaller than this many pixels across: dimmer instead (0 for no limit)
out vec2 vUv;
out vec4 vLook;
out vec3 vWorld;
out vec3 vRight;
out vec3 vUp;
void main() {
  float c = cos(aLook.x), s = sin(aLook.x);
  vec2 p = vec2(position.x * c - position.y * s, position.x * s + position.y * c);
  float size = aPos.w;
  float dim = 1.0;
  if (uMinPx > 0.0) {
    float depth = max(dot(aPos.xyz - uCamPos, uCamFwd), 0.1);
    float px = size * uRes.y / (uTan.y * depth);
    float k = max(1.0, uMinPx / px);
    size *= k;
    dim = 1.0 / (k * k);
  }
  vec3 world = aPos.xyz + (uCamRight * p.x + uCamUp * p.y) * size;
  vUv = position.xy;
  vLook = vec4(aLook.x, aLook.y * dim, aLook.zw);
  vWorld = world;
  vRight = uCamRight * c + uCamUp * s;
  vUp = -uCamRight * s + uCamUp * c;
  gl_Position = aLook.y <= 0.0 ? vec4(2.0, 2.0, 2.0, 1.0) : projectionMatrix * viewMatrix * vec4(world, 1.0);
}
`,Zg=`
${ng}
layout(location = 0) out vec4 outC;
uniform sampler2D tDepth;
uniform vec2 uRes;
uniform mat4 uProj;
in vec2 vUv;
in vec4 vLook;
in vec3 vWorld;
in vec3 vRight;
in vec3 vUp;
float lin(float d) { return uProj[3][2] / (d * 2.0 - 1.0 + uProj[2][2]); }
void main() {
  float r2 = dot(vUv, vUv);
  if (r2 > 1.0) discard;
  // a ragged ball of smoke: two noises eat into its edge
  vec2 nuv = vUv * 0.075 + vLook.w * vec2(7.3, 3.1);
  float n = texture(tNoiseHi, nuv).r * 0.62 + texture(tNoiseHi, nuv * 2.6 + 0.4).g * 0.38;
  float body = (1.0 - r2) * smoothstep(0.34, 0.72, n * 0.85 + (1.0 - r2) * 0.42);
  // where it meets the ground or the grass it fades instead of cutting
  float scene = lin(texture(tDepth, gl_FragCoord.xy / uRes).r);
  float mine = lin(gl_FragCoord.z);
  float soft = clamp((scene - mine) / 0.6, 0.0, 1.0) * clamp((mine - 0.4) / 3.0, 0.0, 1.0);
  float a = body * vLook.y * soft;
  if (a < 0.003) discard;
  // lit as a ball: the sun on one side, the sky on top, the fire underneath
  vec3 N = normalize(vRight * vUv.x + vUp * vUv.y + cross(vRight, vUp) * sqrt(max(1.0 - r2, 0.0)));
  float sun = clamp(dot(N, uSunDir) * 0.6 + 0.4, 0.0, 1.0);
  vec3 albedo = mix(vec3(0.2, 0.185, 0.17), vec3(0.58, 0.56, 0.54), smoothstep(0.2, 0.8, n)) * mix(0.5, 1.0, 1.0 - vLook.z);
  vec3 c = albedo * (uSunColor * sun * 0.5 + mix(uBounceColor, uSkyColor, N.y * 0.5 + 0.5) * 1.1);
  c += uFireColor * (vLook.z * 0.7 + 0.3 * texture(tGlow, stateUv(vWorld.xz)).g) * (0.05 + 0.2 * clamp(-N.y * 0.5 + 0.5, 0.0, 1.0)) * albedo * 2.6;
  c = applyFog(c, vWorld);
  outC = vec4(c * a, a);
}
`,Qg=`
layout(location = 0) out vec4 outC;
uniform sampler2D tDepth;
uniform vec2 uRes;
in vec2 vUv;
in vec4 vLook;
void main() {
  if (gl_FragCoord.z > texture(tDepth, gl_FragCoord.xy / uRes).r) discard;
  float d = length(vUv);
  float dot_ = exp(-d * d * 5.0) * (1.0 - smoothstep(0.7, 1.0, d));
  vec3 c = mix(vec3(0.9, 0.08, 0.004), vec3(3.0, 1.1, 0.16), vLook.z);
  outC = vec4(c * dot_ * vLook.y, 1.0);
}
`,$g=720,e_=640;function t_(e,t){let n=ag(4242),r=new X([-1,-1,0,1,-1,0,-1,1,0,1,1,0],3),i=(e,n,i,a)=>{let o=new Du;o.setAttribute(`position`,r),o.setIndex([0,1,3,0,3,2]);let s=new za(new Float32Array(e*4),4),c=new za(new Float32Array(e*4),4);s.setUsage(dt),c.setUsage(dt),o.setAttribute(`aPos`,s),o.setAttribute(`aLook`,c),o.instanceCount=0;let l=new xa(o,new Qc({glslVersion:yt,vertexShader:Xg,fragmentShader:n,uniforms:{...t,uMinPx:{value:a}},transparent:!0,depthTest:!1,depthWrite:!1,...i}));return l.frustumCulled=!1,{geometry:o,aPos:s,aLook:c,mesh:l}},a=i($g,Zg,{blending:5,blendSrc:201,blendDst:205,blendSrcAlpha:201,blendDstAlpha:205},0),o=i(e_,Qg,{blending:2},2.2),s=new Er;s.add(a.mesh,o.mesh);let c=new Float32Array($g*14),l=0,u=new Uint16Array($g),d=new Float32Array($g),f=new Float32Array(e_*10),p=0,m=0,h=0;return{scene:s,get count(){return l+p},clear(){l=p=0,a.geometry.instanceCount=o.geometry.instanceCount=0},update(t,r){let i=e.state,s=e.uniforms.uWind.value,g=s.x*s.z,_=s.y*s.z,v=i.burning;for(m+=t*Math.min(55,v*.012+(v?4:0)),h+=t*Math.min(140,v*.04);m>=1;){--m;let t=i.sample(n);if(!t||l>=$g)continue;let r=l++*14,a=Math.max(t[2],t[3]*.4);c[r]=t[0],c[r+1]=e.heightAt(t[0],t[1])+.5+.9*n()*a,c[r+2]=t[1],c[r+3]=(n()-.5)*.6,c[r+4]=1.4+2.2*n()*(.4+a),c[r+5]=(n()-.5)*.6,c[r+6]=0,c[r+7]=7+7*n(),c[r+8]=.6+.6*n(),c[r+9]=n()*6.283,c[r+10]=(n()-.5)*.5,c[r+11]=t[2],c[r+12]=n(),c[r+13]=t[2]>.12?1:.45}for(;h>=1;){--h;let t=i.sample(n);if(!t||t[2]<.15||p>=e_)continue;let r=p++*10;f[r]=t[0],f[r+1]=e.heightAt(t[0],t[1])+.15+.5*n(),f[r+2]=t[1],f[r+3]=(n()-.5)*1.6,f[r+4]=1.2+2.4*n(),f[r+5]=(n()-.5)*1.6,f[r+6]=0,f[r+7]=.5+1.3*n(),f[r+8]=.008+.014*n()*n(),f[r+9]=n()*100}let y=r.getWorldPosition(new q),b=r.getWorldDirection(new q);for(let e=l-1;e>=0;e--){let n=e*14,r=c[n+6]+=t;if(r>=c[n+7]){c.copyWithin(n,(l-1)*14,l*14),l--;continue}let i=Math.min(1,r*.5);c[n+3]+=(g*.85-c[n+3])*Math.min(1,t*.9)*i,c[n+5]+=(_*.85-c[n+5])*Math.min(1,t*.9)*i,c[n+4]+=(.9-c[n+4])*Math.min(1,t*.45),c[n]+=c[n+3]*t,c[n+1]+=c[n+4]*t,c[n+2]+=c[n+5]*t,c[n+8]+=t*(.55+.07*s.z),c[n+9]+=c[n+10]*t,c[n+11]*=Math.exp(-t*1.3)}for(let e=0;e<l;e++)u[e]=e,d[e]=(c[e*14]-y.x)*b.x+(c[e*14+1]-y.y)*b.y+(c[e*14+2]-y.z)*b.z;let x=Array.from(u.subarray(0,l)).sort((e,t)=>d[t]-d[e]);for(let e=0;e<l;e++){let t=x[e]*14,n=c[t+6],r=c[t+7],i=Math.min(1,n/.7),o=1-Math.min(1,Math.max(0,(n-r*.45)/(r*.55)));a.aPos.array.set([c[t],c[t+1],c[t+2],c[t+8]],e*4),a.aLook.array.set([c[t+9],.42*i*o*o*c[t+13],Math.min(1,c[t+11]*1.4),c[t+12]],e*4)}a.geometry.instanceCount=l,a.aPos.needsUpdate=a.aLook.needsUpdate=l>0;for(let e=p-1;e>=0;e--){let n=e*10,r=f[n+6]+=t;if(r>=f[n+7]){f.copyWithin(n,(p-1)*10,p*10),p--;continue}let i=f[n+9];f[n+3]+=(g*.7-f[n+3])*Math.min(1,t*1.6)+Math.sin(r*9+i)*4*t,f[n+5]+=(_*.7-f[n+5])*Math.min(1,t*1.6)+Math.cos(r*7+i*1.7)*4*t,f[n+4]+=(.4-f[n+4])*Math.min(1,t*.8),f[n]+=f[n+3]*t,f[n+1]+=f[n+4]*t,f[n+2]+=f[n+5]*t}for(let e=0;e<p;e++){let t=e*10,n=f[t+6]/f[t+7],r=.7+.3*Math.sin(f[t+6]*23+f[t+9]);o.aPos.array.set([f[t],f[t+1],f[t+2],f[t+8]*(1-.4*n)],e*4),o.aLook.array.set([0,Math.min(1,n*8)*(1-n*n)*r,1-n,0],e*4)}o.geometry.instanceCount=p,o.aPos.needsUpdate=o.aLook.needsUpdate=p>0}}}function n_(e,t,{samples:n=4,fireScale:r=.5,bloom:i=.22,exposure:a=1,flameHeight:o=1.5}={}){let s={type:D,format:F,minFilter:h,magFilter:h,generateMipmaps:!1},c=new Xo(1,1,T);c.minFilter=c.magFilter=u;let l=new Rn(1,1,{...s,samples:n,depthBuffer:!0,depthTexture:c}),d=new Rn(1,1,{...s,depthBuffer:!1}),f=new Rn(1,1,{...s,depthBuffer:!1}),p=[],m=[];for(let e=0;e<5;e++)p.push(new Rn(1,1,{...s,depthBuffer:!1})),m.push(new Rn(1,1,{...s,depthBuffer:!1}));let g=new Z;g.setAttribute(`position`,new ui(new Float32Array([-1,-1,0,3,-1,0,-1,3,0]),3));let _=new xa(g);_.frustumCulled=!1;let v=new Er;v.add(_);let y=new cu,b=(e,t)=>new Qc({glslVersion:yt,vertexShader:Jg,fragmentShader:e,uniforms:t,depthTest:!1,depthWrite:!1,blending:0}),x=`precision highp float;
layout(location = 0) out vec4 outC;
in vec2 vUv;
`,S={tDepth:{value:c},tVol:{value:ug(64)},uCamRight:{value:new q},uCamUp:{value:new q},uCamFwd:{value:new q},uTan:{value:new K},uProj:{value:new J},uBox:{value:new In(0,0,-1,0)},uBoxY:{value:new K},uFlameH:{value:o},uTorch:{value:new In(0,0,0,0)},uRes:{value:new K}},C=b(Yg,{...t.uniforms,...S}),w=t_(t,{...t.uniforms,...S}),E=b(x+`
    uniform sampler2D tScene, tFire;
    uniform vec2 uFireRes;
    void main() {
      // a one pixel tent evens out the march's jitter
      vec2 px = 0.75 / uFireRes;
      vec4 f = 0.25 * (texture(tFire, vUv + px * vec2(-1, -1)) + texture(tFire, vUv + px * vec2(1, -1)) + texture(tFire, vUv + px * vec2(-1, 1)) + texture(tFire, vUv + px * vec2(1, 1)));
      outC = vec4(texture(tScene, vUv).rgb * (1.0 - f.a) + f.rgb, texture(tScene, vUv).a);
    }`,{tScene:{value:l.texture},tFire:{value:f.texture},uFireRes:{value:new K(1,1)}}),O=b(x+`
    uniform sampler2D tSrc; uniform vec2 uTexel; uniform float uThreshold;
    vec3 pick(vec3 c) { float m = max(c.r, max(c.g, c.b)); float k = smoothstep(uThreshold * 0.7, uThreshold * 1.6, m); return min(c, vec3(14.0)) * k; }
    void main() {
      vec3 c = pick(texture(tSrc, vUv + uTexel * vec2(-1, -1)).rgb) + pick(texture(tSrc, vUv + uTexel * vec2(1, -1)).rgb)
        + pick(texture(tSrc, vUv + uTexel * vec2(-1, 1)).rgb) + pick(texture(tSrc, vUv + uTexel * vec2(1, 1)).rgb);
      outC = vec4(c * 0.25, 1.0);
    }`,{tSrc:{value:null},uTexel:{value:new K},uThreshold:{value:1.5}}),k=b(x+`
    uniform sampler2D tSrc; uniform vec2 uTexel;
    void main() {
      vec3 c = texture(tSrc, vUv).rgb * 4.0;
      c += texture(tSrc, vUv + uTexel * vec2(-1, -1)).rgb + texture(tSrc, vUv + uTexel * vec2(1, -1)).rgb
        + texture(tSrc, vUv + uTexel * vec2(-1, 1)).rgb + texture(tSrc, vUv + uTexel * vec2(1, 1)).rgb;
      outC = vec4(c / 8.0, 1.0);
    }`,{tSrc:{value:null},uTexel:{value:new K}}),A=b(x+`
    uniform sampler2D tSrc, tPrev; uniform vec2 uTexel; uniform float uMix;
    void main() {
      vec3 c = texture(tPrev, vUv + uTexel * vec2(-1, 0)).rgb + texture(tPrev, vUv + uTexel * vec2(1, 0)).rgb
        + texture(tPrev, vUv + uTexel * vec2(0, -1)).rgb + texture(tPrev, vUv + uTexel * vec2(0, 1)).rgb;
      c += (texture(tPrev, vUv + uTexel * vec2(-1, -1)).rgb + texture(tPrev, vUv + uTexel * vec2(1, -1)).rgb
        + texture(tPrev, vUv + uTexel * vec2(-1, 1)).rgb + texture(tPrev, vUv + uTexel * vec2(1, 1)).rgb) * 0.5;
      outC = vec4(c / 6.0 * uMix + texture(tSrc, vUv).rgb, 1.0);
    }`,{tSrc:{value:null},tPrev:{value:null},uTexel:{value:new K},uMix:{value:.85}}),j=b(x+`
    uniform sampler2D tHdr, tBloom; uniform float uBloom, uExposure, uVignette, uPaint; uniform vec2 uPaintTexel; uniform highp sampler2D tDepth;
    // The frame as paint: round each point, four overlapping patches; the calmest of them gives its colour. Edges
    // stay where they are and everything between them goes flat, as a loaded brush leaves it.
    vec3 painted(vec2 uv) {
      // a finer brush for what is near: grass at your feet keeps its blades, the far country goes to flat paint
      float away = 0.12 / max(1.0 - textureLod(tDepth, uv, 0.0).r, 1e-6);
      int r = int(uPaint);
      // (what is painted already keeps its detail)
      bool done = textureLod(tHdr, uv, 0.0).a < 0.6;
      // The brush is broadest in the middle distance, where grass is a mass. Near, it leaves the strokes of the
      // blades alone; far off, where a house is a few pixels across, it leaves those alone too.
      r = min(r, 2);
      if (away < 16.0 || away > 140.0) r = 1;
      if (done) r = 0;
      vec3 m[4];
      vec3 s[4];
      for (int k = 0; k < 4; k++) { m[k] = vec3(0.0); s[k] = vec3(0.0); }
      for (int j = -r; j <= r; j++) {
        for (int i = -r; i <= r; i++) {
          vec3 c = textureLod(tHdr, uv + vec2(float(i), float(j)) * uPaintTexel, 0.0).rgb;
          c = c / (1.0 + c);
          vec3 cc = c * c;
          if (i <= 0 && j <= 0) { m[0] += c; s[0] += cc; }
          if (i >= 0 && j <= 0) { m[1] += c; s[1] += cc; }
          if (i <= 0 && j >= 0) { m[2] += c; s[2] += cc; }
          if (i >= 0 && j >= 0) { m[3] += c; s[3] += cc; }
        }
      }
      float n = float((r + 1) * (r + 1));
      vec3 sum = vec3(0.0);
      float total = 0.0;
      for (int k = 0; k < 4; k++) {
        vec3 mean = m[k] / n;
        vec3 var = abs(s[k] / n - mean * mean);
        float v = var.r + var.g + var.b;
        // (the calmest patch nearly alone, but never a hard choice: a hard one flickers)
        float w = 1.0 / (1e-7 + v * v * v * v * 1e12);
        sum += mean * w;
        total += w;
      }
      vec3 o = sum / total;
      return o / (1.0 - min(o, vec3(0.985)));
    }
    // The inked line round a figure. A figure's parts each write a small alpha of their own (valley/wanderer.js),
    // smaller than anything else writes; where one of those meets a different alpha, there is a line.
    float inkAt(vec2 uv) {
      float a = textureLod(tHdr, uv, 0.0).a;
      float da = textureLod(tDepth, uv, 0.0).r;
      vec2 w = uPaintTexel * max(1.0, 1.0 / (uPaintTexel.y * 1080.0));
      float e = 0.0;
      for (int k = 0; k < 4; k++) {
        vec2 o = k == 0 ? vec2(1.0, 0.0) : k == 1 ? vec2(-1.0, 0.0) : k == 2 ? vec2(0.0, 1.0) : vec2(0.0, -1.0);
        float b = textureLod(tHdr, uv + o * w, 0.0).a;
        if (min(a, b) >= 0.23) continue;
        // (a line where the figure stands in front of something, or one of its parts meets another: none where a
        // blade of grass stands in front of it, or every blade would be inked)
        float db = textureLod(tDepth, uv + o * w, 0.0).r;
        if (max(a, b) >= 0.23 && (a < b ? db < da : da < db)) continue;
        e = max(e, abs(a - b));
      }
      return smoothstep(0.008, 0.022, e);
    }
    vec3 aces(vec3 x) { return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), 0.0, 1.0); }
    vec3 toSrgb(vec3 c) { return mix(c * 12.92, 1.055 * pow(c, vec3(1.0 / 2.4)) - 0.055, step(0.0031308, c)); }
    void main() {
      vec3 c = (uPaint > 0.5 ? painted(vUv) : texture(tHdr, vUv).rgb) + texture(tBloom, vUv).rgb * uBloom;
      // (the line is the colour under it, gone dark and a little warm: never a black wire)
      c = mix(c, min(c, vec3(1.2)) * vec3(0.2, 0.14, 0.15), inkAt(vUv) * 0.92);
      vec2 d = vUv - 0.5;
      c *= 1.0 - uVignette * dot(d, d) * 1.6;
      c *= uExposure;
      // a painter's curve: what is below white stays as it was mixed, and only the brightest lights are eased in
      vec3 hi = max(c - 0.62, 0.0);
      c = min(c, 0.62) + 0.38 * (1.0 - exp(-hi / 0.38));
      float l = dot(c, vec3(0.2126, 0.7152, 0.0722));
      c = toSrgb(max(mix(vec3(l), c, 1.06), 0.0));
      float n = fract(52.9829189 * fract(dot(gl_FragCoord.xy, vec2(0.06711056, 0.00583715))));
      outC = vec4(c + (n - 0.5) / 255.0, 1.0);
    }`,{tHdr:{value:d.texture},tBloom:{value:m[0].texture},uBloom:{value:i},uExposure:{value:a},uVignette:{value:.14},uPaint:{value:0},tDepth:{value:c},uPaintTexel:{value:new K(1,1)}});t.renderer=e;let M=new K,N=0,P=0,I=(t,n)=>{_.material=t,e.setRenderTarget(n),e.render(v,y)},ee=(e,t)=>{if(e===N&&t===P)return;N=e,P=t,c.image.width=e,c.image.height=t,l.setSize(e,t),d.setSize(e,t);let n=Math.max(1,Math.round(e*te.fireScale)),r=Math.max(1,Math.round(t*te.fireScale));f.setSize(n,r),E.uniforms.uFireRes.value.set(n,r),S.uRes.value.set(e,t);let i=e,a=t;for(let e=0;e<p.length;e++)i=Math.max(1,i>>1),a=Math.max(1,a>>1),p[e].setSize(i,a),m[e].setSize(i,a)},te={bloom:i,exposure:a,fireScale:r,particles:w,torch:{position:new q,size:0},rtScene:l,update(e,t){w.update(e,t)},render(n,r){e.getDrawingBufferSize(M),ee(M.x,M.y),r.updateMatrixWorld();let i=r.matrixWorld.elements;S.uCamRight.value.set(i[0],i[1],i[2]).normalize(),S.uCamUp.value.set(i[4],i[5],i[6]).normalize(),S.uCamFwd.value.set(-i[8],-i[9],-i[10]).normalize();let a=Math.tan(hn.degToRad(r.fov)/2);S.uTan.value.set(a*r.aspect,a),S.uProj.value.copy(r.projectionMatrix);let o=t.state,s=o.fireBox;if(o.burning>0?(S.uBox.value.set(s.x0,s.z0,s.x1,s.z1),S.uBoxY.value.set(s.y0,s.y1)):S.uBox.value.set(0,0,-1,0),S.uTorch.value.set(te.torch.position.x,te.torch.position.y,te.torch.position.z,te.torch.size),e.setRenderTarget(l),e.render(n,r),o.burning>0||te.torch.size>0?I(C,f):(e.setRenderTarget(f),e.setClearColor(0,0),e.clear()),I(E,d),w.count){let t=e.autoClear;e.autoClear=!1,e.setRenderTarget(d),e.render(w.scene,r),e.autoClear=t}let c=d;for(let e=0;e<p.length;e++){let t=e===0?O:k;t.uniforms.tSrc.value=c.texture,t.uniforms.uTexel.value.set(.5/p[e].width,.5/p[e].height),I(t,p[e]),c=p[e]}let u=p[p.length-1];for(let e=p.length-2;e>=0;e--)A.uniforms.tSrc.value=p[e].texture,A.uniforms.tPrev.value=u.texture,A.uniforms.uTexel.value.set(.5/u.width,.5/u.height),I(A,m[e]),u=m[e];j.uniforms.uBloom.value=te.bloom,j.uniforms.uExposure.value=te.exposure,j.uniforms.uPaint.value=te.paint??0,j.uniforms.uPaintTexel.value.set(1/M.x,1/M.y),I(j,null)},async compile(t,n,r){e.getDrawingBufferSize(M),ee(M.x,M.y);let i=()=>new Promise(e=>{let t=new MessageChannel;t.port1.onmessage=()=>e(),t.port2.postMessage(0)}),a=new Map,o=new Set([S.tVol.value]),s=e=>{for(let t of Object.values(e.uniforms??{}))t?.value?.isTexture&&!t.value.isRenderTargetTexture&&!t.value.isDepthTexture&&o.add(t.value);for(let t of[`map`,`normalMap`,`roughnessMap`,`metalnessMap`,`aoMap`,`emissiveMap`,`alphaMap`])e[t]?.isTexture&&o.add(e[t])};t.traverse(e=>{(e.isMesh||e.isPoints||e.isLine)&&e.material&&!Array.isArray(e.material)&&(a.has(e.material)||a.set(e.material,e),s(e.material))});let c=a.size+o.size+8,u=0,p=async()=>{r?.(++u,c),await i()};for(let t of o)e.initTexture(t),await p();let m=[],h=e.getRenderTarget();for(let r of a.values())e.setRenderTarget(l),m.push(e.compileAsync(r,n,t)),e.setRenderTarget(h),await p();e.setRenderTarget(d),m.push(e.compileAsync(w.scene,n)),e.setRenderTarget(h),await p();for(let t of[C,E,O,k,A,j])_.material=t,e.setRenderTarget(t===C?f:d),m.push(e.compileAsync(v,y)),e.setRenderTarget(h),await p();await Promise.all(m)}};return te}async function r_({heightAt:e=()=>0,paint:t=()=>({}),extent:n=1024,extentRes:r=1024,centre:i=[0,0]}={},a){let o={heightAt:e,paint:t,extentMin:[i[0]-n/2,i[1]-n/2],texel:n/r,extentRes:r},s=new Float32Array(r*r),c=new Uint8Array(r*r*4),l=0;for(;l<r;){let e=performance.now()+24;for(;l<r&&performance.now()<e;)qg(o,s,c,l,Math.min(r,l+4)),l+=4;a?.(Math.min(1,l/r)),await new Promise(e=>{let t=new MessageChannel;t.port1.onmessage=()=>e(),t.port2.postMessage(0)})}return{heights:s,map:c}}function i_({heightAt:e=()=>0,paint:t=()=>({dry:.3,growth:1,wet:0,flowers:.3}),extent:n=1024,extentRes:r=1024,size:i=256,centre:a=[0,0],textures:o=Ng(),height:s=.52,density:l=1,seed:d=1,recover:f=70,regrow:p=0,flowers:m=!0,baked:g=null}={}){let _=[a[0]-n/2,a[1]-n/2],v=n/r,y=g?.heights??new Float32Array(r*r),x=g?.map??new Uint8Array(r*r*4);g||qg({heightAt:e,paint:t,extentMin:_,texel:v,extentRes:r},y,x,0,r);let S=new Fa(y,r,r,te,E);S.minFilter=S.magFilter=u,S.needsUpdate=!0;let C=new Fa(x,r,r,F,b);C.minFilter=C.magFilter=h,C.wrapS=C.wrapT=c,C.needsUpdate=!0;let w=(e,t,n,i)=>(a,o)=>{let s=Math.min(r-1,Math.max(0,(a-_[0])/v-.5)),c=Math.min(r-1,Math.max(0,(o-_[1])/v-.5)),l=Math.min(r-2,Math.floor(s)),u=Math.min(r-2,Math.floor(c)),d=s-l,f=c-u,p=(l+u*r)*t+n,m=p+r*t;return((e[p]*(1-d)+e[p+t]*d)*(1-f)+(e[m]*(1-d)+e[m+t]*d)*f)*i},T=w(y,1,0,1),D=w(x,4,0,1/255),O=w(x,4,1,1/255),k=w(x,4,2,1/255),A=new Gg({size:i,centre:a,heightAt:T,growthAt:O,dryAt:D,wetAt:k,recover:f,regrow:p}),j=Array.from({length:6},()=>new In(0,0,0,0)),M={uTime:{value:0},uWind:{value:new In(1,0,3,1)},uWindScroll:{value:new K},uExtentMin:{value:new K(..._)},uExtentInv:{value:1/n},uHeightRes:{value:r},uStateMin:{value:new K(A.minX,A.minZ)},uStateInv:{value:1/i},tHeight:{value:S},tMap:{value:C},tState:{value:A.tex},tFire:{value:A.ftex},tGlow:{value:A.gtex},tNoise:{value:cg(256,d*13+1)},tNoiseHi:{value:cg(256,d*13+1,!0)},tWhite:{value:lg(256,d*7+2)},uSunDir:{value:new q(.6,.25,.3).normalize()},uSunColor:{value:new Y(3.2,2.5,1.6)},uSkyColor:{value:new Y(.34,.42,.6)},uBounceColor:{value:new Y(.1,.09,.05)},uFogColor:{value:new Y(.55,.6,.7)},uFogSunColor:{value:new Y(1,.75,.45)},uFogDensity:{value:.0016},uFireColor:{value:new Y(0,0,0)},uCamPos:{value:new q},uPushers:{value:j},uCloudShade:{value:.55},tShade:{value:new Fa(new Uint8Array([0,0,0,255]),1,1)},tFloor:{value:new Fa(new Uint8Array([0,0,0,0]),1,1)},tStrokes:{value:new Fa(new Uint8Array([128,0,0,255]),1,1)},tSlope:{value:new Fa(new Uint8Array([128,128]),1,1,re)},uHeroEye:{value:new q(0,60,0)},uFloor:{value:new In(1,0,0,0)},uFloorInv:{value:0},uLamp:{value:new In(0,0,0,0)},uWalker:{value:new In(0,0,0,0)}},N=new gr,P=Mg(M,o,{height:s,extentMin:_,texel:v}),I=hg(M,{height:s,density:l,seed:d},T);I.init(),N.add(P.mesh,I.group);let ee=Sg(M,A,T,ag(d*31+9));N.add(ee.mesh);let ne=m?Og(M,{height:s,seed:d},T):null;ne&&N.add(ne.group);let ie={angle:0,speed:3,gust:1},ae={group:N,uniforms:M,state:A,blades:I,ground:P,clippings:ee,flowers:ne,size:i,extent:n,wind:ie,renderer:null,heightAt:T,growthAt:O,dryAt:D,wetAt:k,bend:(e,t,n,r,i,a)=>A.bend(e,t,n,r,i,a),cutLine(e,t,n,r,i,a,o,s=0,c=0,l=null){let u=A.cutLine(e,t,n,r,i,a,o);return u?ee.take(s,c,l):A.cutEvents.length=0,u},cut(e,t,n=.3,r=.07){return this.cutLine(e,t,e,t,n*2,r,r)},ignite:(e,t,n)=>A.ignite(e,t,n),stateAt:(e,t)=>A.stateAt(e,t),setWind(e,t,n=1){ie.angle=e,ie.speed=t,ie.gust=n,M.uWind.value.set(Math.cos(e),Math.sin(e),t,n),A.setWind(Math.cos(e),Math.sin(e),t)},setPusher(e,t,n,r,i){j[e].set(t,n,r,i)},setLight(e){e.sunDir&&M.uSunDir.value.copy(e.sunDir).normalize(),e.sunColor&&M.uSunColor.value.copy(e.sunColor),e.skyColor&&M.uSkyColor.value.copy(e.skyColor),e.bounceColor&&M.uBounceColor.value.copy(e.bounceColor),e.fogColor&&M.uFogColor.value.copy(e.fogColor),e.fogSunColor&&M.uFogSunColor.value.copy(e.fogSunColor),e.fogDensity!==void 0&&(M.uFogDensity.value=e.fogDensity),e.cloudShade!==void 0&&(M.uCloudShade.value=e.cloudShade)},setLamp(e,t,n,r){M.uLamp.value.set(e,t,n,r)},setDensity:e=>I.setDensity(e),reset(){A.reset(),ee.clear()},update(e,t){M.uTime.value+=e;let n=M.uTime.value,r=.35+ie.speed*.55;M.uWindScroll.value.x+=Math.cos(ie.angle)*r*e,M.uWindScroll.value.y+=Math.sin(ie.angle)*r*e,t.getWorldPosition(M.uCamPos.value);let{x:i,z:a}=M.uCamPos.value;I.follow(i,a),P.follow(i,a),ne&&ne.follow(i,a),A.update(e,ae.renderer),ee.update(e);let o=1+.1*Math.sin(n*9.1)+.07*Math.sin(n*23.3+1)+.05*Math.sin(n*41.7+2);M.uFireColor.value.setRGB(3.2*o,1.15*o,.28*o)}};return ae.setWind(.6,3),ae}var a_=[.6,.8],[o_,s_]=a_,c_=Math.PI*2,l_=[0,0],u_=f_(-9,-6),d_={at:[0,0],rx:17,rz:11,level:0};function f_(e,t){return[e*o_-t*s_,e*s_+t*o_]}var p_=10,m_=13,h_=Math.atan2(s_,o_),g_=.02,__=new Float32Array([0]),v_={length:0,thetaMax:0,radius:p_,tarnFloor:0},y_=e=>p_+m_*e/c_;function b_({length:e}){let t=[0],n=0,r=0;for(;r<e;){let e=y_(n+g_/2);r+=Math.hypot(e,m_/c_)*g_,n+=g_,t.push(r)}__=Float32Array.from(t),v_.length=r,v_.thetaMax=n,v_.radius=y_(n);let i=C_(v_.length);d_.at=[i.x+i.ox*24+i.tx*10,i.z+i.oz*24+i.tz*10],v_.tarnFloor=F_(d_.at[0],d_.at[1]),d_.level=v_.tarnFloor-.55}var x_=()=>v_.length,S_=()=>v_.radius;function C_(e){let t=Math.min(Math.max(e,0),v_.length),n=0,r=__.length-1;for(;r-n>1;){let e=n+r>>1;__[e]<=t?n=e:r=e}let i=__[r]-__[n]||1,a=(n+(t-__[n])/i)*g_,o=y_(a),s=h_+a,c=Math.cos(s),l=Math.sin(s),u=m_/c_/o,d=Math.hypot(1,u);return{x:o*c,z:o*l,ox:c,oz:l,tx:(-l+u*c)/d,tz:(c+u*l)/d}}function w_(e,t){let n=Math.hypot(e,t),r=(Math.atan2(t,e)-h_)%c_;r<0&&(r+=c_);let i=Math.round((n-p_)/m_-r/c_),a=r+c_*i;if(a>=0&&a<=v_.thetaMax)return Math.abs(n-y_(a));let o=a<0?0:v_.thetaMax,s=y_(o);return Math.hypot(e-s*Math.cos(h_+o),t-s*Math.sin(h_+o))+1.2}var T_=(e,t)=>Math.hypot(e,t)<v_.radius+9;function E_(e,t,n){let r=Math.imul(e,374761393)^Math.imul(t,668265263)^Math.imul(n,1274126177);return r=Math.imul(r^r>>>13,1274126177),r^=r>>>16,(r>>>0)/4294967296}function D_(e,t,n){let r=Math.floor(e),i=Math.floor(t),a=e-r,o=t-i,s=a*a*a*(a*(a*6-15)+10),c=o*o*o*(o*(o*6-15)+10),l=E_(r,i,n),u=E_(r+1,i,n),d=E_(r,i+1,n),f=E_(r+1,i+1,n);return l+(u-l)*s+(d-l)*c+(l-u-d+f)*s*c}function O_(e,t,n,r){let i=0,a=.5,o=0;for(let s=0;s<n;s++)i+=a*D_(e,t,r+s*17),o+=a,a*=.5,e=e*2.03+11.7,t=t*2.03-5.3;return i/o}var k_=(e,t,n)=>{let r=Math.min(1,Math.max(0,(n-e)/(t-e)));return r*r*(3-2*r)};function A_(e,t){let n=e-d_.at[0],r=t-d_.at[1],i=n*o_+r*s_,a=-n*s_+r*o_,o=1+.16*Math.sin(Math.atan2(i,a)*3+.7)+.09*Math.sin(Math.atan2(i,a)*7+2.1);return Math.hypot(a/d_.rx,i/d_.rz)/o}var j_=()=>1e6,M_=()=>0,N_=[1e6,1e6];function P_(e,t){let n=Math.hypot(e,t),r=k_(.44,.52,O_(e/1700+4.1,t/1700+8.6,4,191)*.72+O_(e/430+6.2,t/430+1.3,3,197)*.28);return r=Math.max(r,k_(55,120,F_(e,t))*k_(.36,.5,O_(e/900+7.7,t/900+2.4,3,201))),r*k_(820,1500,n)}function F_(e,t){let n=e*o_+t*s_,r=-e*s_+t*o_,i=Math.hypot(e,t),a=v_.radius,o=Math.min(46,7+a*.17)*(1-k_(5,a*1.25+34,i));o+=(O_(e/240+5.3,t/240-2.9,4,101)-.5)*24*k_(a+20,a+320,i),o+=(O_(e/64+3.9,t/64+7.1,3,105)-.5)*3.2*k_(a-10,a+70,i),o+=(O_(e/110+8.7,t/110+1.3,2,113)-.5)*16*k_(a+40,a+160,i)*(1-k_(900,1800,i));let s=Math.exp(-((Math.atan2(r,n)/.8)**2)),c=k_(a+240,a+1150,i)*(.6+.8*O_(e/800+6,t/800+3,3,137));return o+=125*c*(1-.9*s),o-=22*k_(a+50,a+520,i)*s,o}function I_(e,t){let n=F_(e,t),r=A_(e,t);if(r<2.2){let e=1-k_(1.25,2.2,r);n+=(v_.tarnFloor-n)*e,n-=2.4*(1-k_(.25,1.06,r))}return n}function L_(e,t){let n=.08+.3*k_(.54,.8,O_(e/90+7.2,t/90+3.1,3,121)),r=A_(e,t),i=Math.max(k_(.68,.76,O_(e/47+1.5,t/47+6.9,2,133))*.6,1-k_(1,1.7,r)),a=1,o=w_(e,t);a=Math.min(a,k_(.42,1.1,o+(O_(e/2.7,t/2.7,2,161)-.5)*.7)),a=Math.min(a,k_(.8,.86,r+(O_(e/3.3,t/3.3,2,167)-.5)*.04));let s=k_(.4,.68,O_(e/38-2,t/38+9,2,155)),c=Math.min(1,s*(.25+.65*k_(v_.radius,v_.radius+40,Math.hypot(e,t))));return{dry:n*(1-i),growth:a,wet:i,flowers:c}}var R_={captured:`Taken`,uploaded:`Uploaded`,imported:`Imported`,screenshot:`Screenshot`,ai_generated:`Made by AI`},z_={caption:`Caption`,description:`Description`,ocr_text:`Text in the image`,transcript:`Transcript`},B_={day:`numeric`,month:`long`,year:`numeric`},V_=e=>String(e??``).replace(/[&<>"]/g,e=>({"&":`&amp;`,"<":`&lt;`,">":`&gt;`,'"':`&quot;`})[e]);function H_(e,t){let n=String(e.mediaType??``).split(`/`)[0]||`image`,r=e.acquisitions?.[0]??{},i=(e.interpretations??[]).map(e=>({label:z_[String(e.predicate??``).replace(`mother.media:`,``)]??`Note`,text:String(e.text??``),by:e.attribution?.producer?.name??e.contributor??``})),a=i.find(e=>e.label===`Caption`)?.text,o=Date.parse(r.eventTime?.at??r.acquiredAt??``)||0,s=a||r.originalFilename||(n===`audio`?`A recording`:n===`video`?`A video`:`A photo`);return{photo:!0,id:`media:${e.sha256}`,sha:e.sha256,kind:n,mediaType:e.mediaType,blob:t,url:t?URL.createObjectURL(t):``,title:s,file:r.originalFilename??``,origin:R_[r.origin]??``,source:r.source??``,at:o,eventPrecision:r.eventTime?.precision??``,readings:i,variant:e.variant,uses:e.uses??0,width:e.width,height:e.height,text:[s,r.originalFilename,r.source,R_[r.origin],n,n===`image`?`photo`:``,...i.map(e=>e.text)].join(`
`).toLowerCase()}}function U_(e,t){let n=[],r=-1,i=()=>{};e.innerHTML=`
    <div class="v-shade" data-close></div>
    <figure class="v-frame">
      <div class="v-media"></div>
      <figcaption class="v-text"></figcaption>
    </figure>
    <button class="v-nav v-prev" aria-label="Previous" data-step="-1"><svg viewBox="0 0 24 24" class="ico"><path d="m14.5 5.5-6.5 6.5 6.5 6.5"/></svg></button>
    <button class="v-nav v-next" aria-label="Next" data-step="1"><svg viewBox="0 0 24 24" class="ico"><path d="m9.5 5.5 6.5 6.5-6.5 6.5"/></svg></button>
    <button class="v-close" aria-label="Close" data-close>×</button>`;let a=e.querySelector(`.v-media`),o=e.querySelector(`.v-text`);function s(t){r=(t+n.length)%n.length;let i=n[r];if(a.innerHTML=``,i.url){let e=document.createElement(i.kind===`image`?`img`:i.kind);e.src=i.url,i.kind===`image`?e.alt=i.title:(e.controls=!0,e.autoplay=!0),a.append(e)}else a.innerHTML=`<p class="v-missing">This item could not be shown.</p>`;let s=i.at?new Date(i.at).toLocaleDateString(void 0,B_):``;o.innerHTML=`
      <span class="v-kind">${V_(i.kind===`image`?`Photo`:i.kind===`audio`?`Recording`:`Video`)}${i.origin?` · ${V_(i.origin)}`:``}</span>
      <h2>${V_(i.title)}</h2>
      ${s?`<p class="v-when">${V_(s)}</p>`:``}
      ${i.readings.map(e=>`<div class="v-read"><b>${V_(e.label)}</b><p>${V_(e.text)}</p>${e.by?`<span>by ${V_(e.by)}</span>`:``}</div>`).join(``)}
      <dl>
        ${i.file?`<dt>File</dt><dd>${V_(i.file)}</dd>`:``}
        ${i.source?`<dt>Source</dt><dd>${V_(i.source)}</dd>`:``}
        <dt>Shown</dt><dd>${i.variant===`derivative`?`A copy with location and device data removed`:`The original`}</dd>
        <dt>Cited by</dt><dd>${i.uses===1?`1 memory`:`${V_(i.uses)} memories`}</dd>
      </dl>
      ${n.length>1?`<p class="v-count">${r+1} of ${n.length}</p>`:``}`,e.querySelector(`.v-prev`).hidden=e.querySelector(`.v-next`).hidden=n.length<2}function c(){if(!e.hidden){for(let e of a.querySelectorAll(`audio, video`))e.pause();e.hidden=!0,i()}}return e.addEventListener(`click`,e=>{let t=e.target.closest(`[data-step]`);t?s(r+Number(t.dataset.step)):e.target.closest(`[data-close]`)&&c()},{signal:t}),document.addEventListener(`keydown`,t=>{if(!e.hidden){if(t.key===`Escape`)c();else if(t.key===`ArrowRight`)s(r+1);else if(t.key===`ArrowLeft`)s(r-1);else return;t.preventDefault(),t.stopImmediatePropagation()}},{capture:!0,signal:t}),{open(t,r,a=()=>{}){n=t,i=a,e.hidden=!1,s(r)},close:c,get isOpen(){return!e.hidden}}}function W_(e,t=512){let n=document.createElement(`canvas`);n.width=n.height=t;let r=n.getContext(`2d`),i=r.createLinearGradient(0,0,t,t);return i.addColorStop(0,e.kind===`audio`?`#6d5aa8`:`#3f6c8f`),i.addColorStop(1,e.kind===`audio`?`#d99ab4`:`#9fc7c0`),r.fillStyle=i,r.fillRect(0,0,t,t),r.fillStyle=`rgba(255, 255, 255, 0.92)`,r.font=`${t*.3}px system-ui, sans-serif`,r.textAlign=`center`,r.textBaseline=`middle`,r.fillText(e.kind===`audio`?`♪`:`▶`,t/2,t*.44),r.font=`${t*.055}px system-ui, sans-serif`,r.fillText(e.title.slice(0,28),t/2,t*.78),n}var G_=null;async function K_(){try{let e=location.hash.slice(1);try{G_=sessionStorage.getItem(`garden-session`)}catch{}if(e){history.replaceState(null,``,location.pathname+location.search);let t=await fetch(`./api/session`,{method:`POST`,headers:{Authorization:`Bearer ${e}`}});if(t.ok){G_=(await t.json()).session;try{sessionStorage.setItem(`garden-session`,G_)}catch{}}}return await X_()}catch{return null}}async function q_(e){if(!G_)return null;let t=await fetch(`./api/garden/records/${encodeURIComponent(e)}`,{headers:{Authorization:`Bearer ${G_}`}});if(!t.ok)return null;let n=await t.json(),r=await J_();return n.attachments=(r.images??[]).flatMap(t=>(t.memoryLinks??[]).filter(t=>t.memoryId===e).map(e=>({sha256:t.sha256,originalSha256:t.originalSha256,mediaType:t.mediaType,variant:t.variant,link:e,acquisition:t.acquisitions?.find(t=>t.id===e.acquisitionId)}))),n.mediaWithheld=(r.unavailableMemories??[]).includes(e),n}async function J_(){if(!G_)return{};try{let e=await fetch(`./api/media`,{headers:{Authorization:`Bearer ${G_}`}});return e.ok?await e.json():{}}catch{return{}}}async function Y_(e){if(!G_||!/^[0-9a-f]{64}$/.test(e))return null;let t=await fetch(`./api/media/${e}`,{headers:{Authorization:`Bearer ${G_}`}});return t.ok?t.blob():null}async function X_(e){if(!G_)return null;let t=e?`./api/garden?since=${encodeURIComponent(e)}`:`./api/garden`,n=await fetch(t,{headers:{Authorization:`Bearer ${G_}`}});return n.ok?n.json():null}async function Z_(){if(!G_)return[];try{let e={headers:{Authorization:`Bearer ${G_}`}},t=await fetch(`./api/media`,e);if(!t.ok)return[];let{images:n=[]}=await t.json();return(await Promise.all(n.map(async t=>{let n=await fetch(`./api/media/${t.sha256}`,e).catch(()=>null);return n?.ok?{item:t,blob:await n.blob()}:null}))).filter(Boolean)}catch{return[]}}var Q_=e=>String(e??``).replace(/[&<>"]/g,e=>({"&":`&amp;`,"<":`&lt;`,">":`&gt;`,'"':`&quot;`})[e]),$_=`@owner-review`,ev=[[`mother`,`Mother's Eyes`],[`owner`,`My Memories`],[`archive`,`The Archive`]],tv={memory:`Memory`,decision:`Decision`,task:`Task`,checkpoint:`Checkpoint`,artifact:`Artifact`,action:`Action`},nv={retracted:`Retracted`,superseded:`Superseded`,cancelled:`Cancelled`},rv={owner_statement:`something the owner said, written down by the writer`,source:`a source the writer read`,model_inference:`the writer's own interpretation`},iv=[15,16,17,18,20,22,24],av=e=>{let t=new Date(e);return Number.isNaN(t.getTime())?`an unrecorded date`:t.toLocaleDateString(`en-GB`,{day:`numeric`,month:`long`,year:`numeric`})},ov=e=>{let t=new Date(e);return Number.isNaN(t.getTime())?`Not recorded`:`${t.toLocaleDateString(`en-GB`,{day:`numeric`,month:`short`,year:`numeric`})}, ${t.toTimeString().slice(0,5)}`},sv=e=>e?Q_(e):`a writer not recorded here`,cv=(e,t)=>{try{return localStorage.getItem(e)??t}catch{return t}},lv=(e,t)=>{try{localStorage.setItem(e,t)}catch{}},uv=e=>e.kind===`memory`&&e.basis===`model_inference`;function dv(e){return e.kind===`memory`?{owner_statement:`captured_paraphrase`,source:`source_summary`,model_inference:`interpretation`}[e.basis]??`unlabelled`:`working_record`}function fv(e,t){let n=new Map(t.map(e=>[e.id,e])),r=e=>({id:e.id,kind:e.kind,title:e.title,status:e.status,updatedAt:e.updatedAt,basis:e.basis}),i=e.actor===$_,a=(e,t)=>({recordId:e.id,revision:e.revision,title:e.title,text:e.content,truncated:e.truncated,relation:t,writer:e.actor===$_?null:e.actor,attribution:null,writtenAt:e.updatedAt,reviewStatus:e.actor===$_?`reviewed`:`unknown`,status:e.status,references:e.references??[],sources:e.sources??[]}),o=t.filter(t=>t.id!==e.id&&(t.references??[]).includes(e.id)).sort((e,t)=>String(t.updatedAt).localeCompare(String(e.updatedAt))),s=[...uv(e)?[a(e,`self`)]:[],...o.filter(e=>uv(e)&&e.status===`active`).map(e=>a(e,`references`))];return{recordId:e.id,kind:e.kind,status:e.status,title:e.title,revision:e.revision,actor:e.actor,basis:e.basis??null,createdAt:e.createdAt,updatedAt:e.updatedAt,eventDate:null,content:e.content,truncated:e.truncated,length:null,sources:e.sources??[],account:{form:dv(e),ownerWords:!1,writer:i?null:e.actor,attribution:null,reviewStatus:i?`reviewed`:`unknown`},witnessAccounts:s,witnessOmitted:0,references:(e.references??[]).map(e=>n.has(e)?r(n.get(e)):{id:e,available:!1}),referencedBy:o.map(r),referencedByOmitted:0,history:e.history??[],historyOmitted:e.historyOmitted??0,defaultLens:s.length?`mother`:`owner`,...e.nextStep?{nextStep:e.nextStep}:{},partial:!0}}function pv(e,{fetchReading:t,snapshotOf:n,all:r,order:i,walk:a,signal:o}){let s=null,c=cv(`garden-reader-lens`,``),l=Number(cv(`garden-reader-size`,`17`))||17,u=null,d=null,f=0,p=`owner`,m=null;e.setAttribute(`role`,`dialog`),e.setAttribute(`aria-modal`,`true`),e.setAttribute(`aria-labelledby`,`reader-title`);let h=()=>e.querySelector(`.sheet`);function g(e){return c===`archive`||c===`owner`?c:c===`mother`&&e.witnessAccounts.length?`mother`:e.defaultLens}function _(e){let t=nv[e.status];return`
      <header class="rhead">
        <p class="rkind">${Q_(tv[e.kind]??e.kind)}${t?` · <span class="rretired">${t}, kept as history</span>`:``}</p>
        <h2 id="reader-title" tabindex="-1">${Q_(e.title)}</h2>
        <p class="rdate">${e.eventDate?.value?`${Q_(e.eventDate.value)}`:`Recorded ${av(e.createdAt)}`}${e.updatedAt&&e.updatedAt!==e.createdAt?` · last revised ${av(e.updatedAt)}`:``}</p>
      </header>`}function v(e){return`<div class="rtabs" role="tablist" aria-label="Ways of reading this memory">${ev.map(([t,n])=>`<button role="tab" id="lens-${t}" aria-controls="reader-panel" aria-selected="${t===e}" tabindex="${t===e?0:-1}" data-lens="${t}">${n}</button>`).join(``)}</div>`}let y=e=>`<div class="rtext">${Q_(e)}</div>`;function b(e){return e.partial&&e.truncated?`<p class="rnote">Only the beginning of this record is in the garden right now. Its full text could not be read.</p>`:e.truncated?`<p class="rnote">This record is very long: the first ${24e3.toLocaleString(`en-GB`)} characters are shown.</p>`:``}let x=e=>{let t=e.map(e=>[e,n(e)]);return t.length?`<div class="rcites"><span>Cites</span>${t.map(([e,t])=>t?`<button class="chip" data-read="${Q_(e)}">${Q_(t.title)}</button>`:`<span class="chip off">a record not in this garden view</span>`).join(``)}</div>`:``};function S(e){return e.witnessAccounts.length?e.witnessAccounts.map(t=>`
        <article class="rwitness">
          <p class="rlabel">Mother's interpretation</p>
          ${t.relation===`self`?``:`<h3>${Q_(t.title)}</h3>`}
          ${y(t.text)}
          ${t.truncated?`<p class="rnote">Only the beginning of this reflection is shown.</p>`:``}
          <p class="rmeta">Written by ${sv(t.writer)} · ${av(t.writtenAt)} · ${t.reviewStatus===`reviewed`?`<span class="rok">Owner-reviewed</span>`:`Review status unavailable`}</p>
          ${t.attribution?`<p class="rmeta">Source given: ${Q_(t.attribution)}</p>`:``}
          <p class="rmeta">${t.relation===`self`?`This record is itself a reflection.`:`Shown because this reflection names this memory among its references. It was not chosen as the memory’s story.`}</p>
          ${x(t.references.filter(t=>t!==e.recordId))}
        </article>`).join(``)+(e.witnessOmitted?`<p class="rnote">${e.witnessOmitted} more ${e.witnessOmitted===1?`reflection names`:`reflections name`} this memory. They are listed in the Archive.</p>`:``):`
        <div class="rempty">
          <p><b>No witness reflection has been recorded for this memory yet.</b></p>
          <p>Mother doesn't write one on her own when you open a memory. If you ask for a Witness Journal letter about it and approve it in your review inbox, it will appear here.</p>
        </div>`}function C(e){let t=e.account,n=t.reviewStatus===`reviewed`?` You approved it in your review inbox.`:``,r={captured_paraphrase:[`Account captured from a conversation`,`Written down by ${sv(t.writer)} from something you said. The wording is theirs, not a quotation of you.${n}`],source_summary:[`Summary recorded from a source`,`Written by ${sv(t.writer)} from a source, not in your words.${n}`],unlabelled:[`How this account was captured is not recorded`,`Written by ${sv(t.writer)}.${n}`],working_record:[`A working record, not a recollection`,`Written by ${sv(t.writer)}.${n}`]};if(t.form===`interpretation`)return`
        <div class="rempty">
          <p><b>No account in your own words is recorded for this.</b></p>
          <p>This record is an interpretation written by ${sv(t.writer)}. Read it under Mother's Eyes, or see what it cites in the Archive.</p>
        </div>`;let[i,a]=r[t.form],o=[];return e.nextStep&&o.push([`Next step`,Q_(e.nextStep)]),e.completed?.length&&o.push([`Done`,e.completed.map(Q_).join(`<br>`)]),e.openQuestions?.length&&o.push([`Open questions`,e.openQuestions.map(Q_).join(`<br>`)]),e.kind!==`memory`&&o.push([`Status`,Q_(e.status)]),`
      <p class="rlabel">${i}</p>
      <p class="rmeta">${a}</p>
      ${y(e.content)}
      ${b(e)}
      ${o.length?`<dl class="rfacts">${o.map(([e,t])=>`<dt>${e}</dt><dd>${t}</dd>`).join(``)}</dl>`:``}`}function w(e){let t=(e,t)=>`<dt>${e}</dt><dd>${t}</dd>`,n=(e,t)=>`<button class="rcopy" data-copy="${Q_(e)}" aria-label="Copy ${t}">Copy</button>`,r=e=>e.available===!1?`<li><code>${Q_(e.id)}</code> ${n(e.id,`record ID`)}<br><span class="rgap">Not available here. This is a gap in what can be shown, not proof that it never existed.</span></li>`:`<li><button class="chip" data-read="${Q_(e.id)}">${Q_(e.title)}</button> <span class="rmeta">${Q_(e.kind)}${e.basis?` · ${Q_(e.basis)}`:``} · ${Q_(e.status)}</span></li>`,i=e.length==null?e.truncated?`Only the first 4,000 characters are in this snapshot`:`As in the snapshot`:`${e.length.toLocaleString(`en-GB`)} characters${e.truncated?`, cut short for reading`:`, complete`}`;return`
      <h3 id="provenance" tabindex="-1">This record</h3>
      <dl class="rfacts">
        ${t(`Record ID`,`<code>${Q_(e.recordId)}</code> ${n(e.recordId,`record ID`)}`)}
        ${t(`Kind`,Q_(e.kind))}
        ${t(`Status`,Q_(e.status))}
        ${t(`Revision`,Q_(e.revision))}
        ${t(`Last written by`,Q_(e.actor??`Not recorded`))}
        ${t(`Basis`,e.basis?`${Q_(e.basis)} <span class="rmeta">(${Q_(rv[e.basis]??`meaning not known`)})</span>`:`Not recorded for this kind of record`)}
        ${t(`Owner review`,e.account.reviewStatus===`reviewed`?`Approved in the review inbox`:`No review is recorded for this revision`)}
        ${e.account.attribution?t(`Source given`,Q_(e.account.attribution)):``}
        ${t(`Event date`,e.eventDate?.value?Q_(e.eventDate.value):`Not recorded in this record`)}
        ${t(`First recorded`,ov(e.createdAt))}
        ${t(`Last revised`,ov(e.updatedAt))}
        ${t(`Text`,i)}
      </dl>
      <h3>Sources</h3>
      ${e.sources.length?`<ul class="rlist">${e.sources.map(e=>`<li>${Q_(e.label)}<br><code>${Q_(e.uri)}</code> ${n(e.uri,`source address`)}</li>`).join(``)}</ul><p class="rmeta">Source addresses are stored, not opened: the garden has not checked what they lead to.</p>`:`<p class="rmeta">No source links are stored with this record.</p>`}
      <h3>Records this one names</h3>
      ${e.references.length?`<ul class="rlist">${e.references.map(r).join(``)}</ul>`:`<p class="rmeta">None.</p>`}
      <h3>Records that name this one</h3>
      ${e.referencedBy.length?`<ul class="rlist">${e.referencedBy.map(r).join(``)}</ul>`:`<p class="rmeta">None.</p>`}
      ${e.referencedByOmitted?`<p class="rmeta">And ${e.referencedByOmitted} more.</p>`:``}
      <h3>Revisions</h3>
      ${e.historyOmitted?`<p class="rmeta">${e.historyOmitted} earlier ${e.historyOmitted===1?`revision is`:`revisions are`} not shown.</p>`:``}
      <ol class="rhist">${[...e.history].reverse().map(e=>`<li><span class="rmeta">rev ${Q_(e.revision)} · ${Q_(e.actor)} · ${ov(e.at)}${e.changed?` · changed ${e.changed.map(Q_).join(`, `)}`:``}</span>${e.excerpt?`<span class="rx">“${Q_(e.excerpt)}”</span>`:``}</li>`).join(``)}</ol>`}function T(e){let t=new Set,n=[...e.references,...e.referencedBy].filter(n=>n.available!==!1&&n.id!==e.recordId&&!t.has(n.id)&&t.add(n.id));return n.length?`
      <section class="rrelated" aria-labelledby="related-title">
        <h3 id="related-title">Linked records</h3>
        <p class="rmeta">Exact links stored with these records. Nothing here is a suggestion.</p>
        <div class="chips">${n.map(e=>`<button class="chip" data-read="${Q_(e.id)}">${Q_(e.title)}${nv[e.status]?` <span class="rgap">· ${nv[e.status].toLowerCase()}</span>`:``}</button>`).join(``)}</div>
      </section>`:``}let E=new Set;function D(){for(let e of E)URL.revokeObjectURL(e);E.clear()}function O(e){return!e.attachments?.length&&!e.mediaWithheld?``:`<section aria-label="Moments captured"><h3>Moments captured</h3>${(e.attachments??[]).map((e,t)=>{let n=Q_(e.acquisition?.originalFilename||e.mediaType),r=p===`archive`?`<dl class="rfacts"><dt>Original SHA-256</dt><dd><code>${Q_(e.originalSha256)}</code></dd><dt>Evidence</dt><dd>${Q_(e.link.evidenceId)}</dd><dt>Source</dt><dd>${Q_(e.acquisition?.source||`Owner-selected file`)}</dd><dt>Approved</dt><dd>${Q_(e.link.approvedAt)}</dd><dt>Event date</dt><dd>${Q_(e.acquisition?.eventTime?.at||`Unknown`)}</dd></dl>`:``;return`<figure><button data-attachment="${t}">Open ${n}</button><figcaption>${Q_(e.variant)} · Owner-approved</figcaption>${r}</figure>`}).join(``)}${e.mediaWithheld?`<p>Media not available in this view.</p>`:``}</section>`}async function k(e){let t=s?.attachments?.[Number(e.dataset.attachment)];if(t){e.disabled=!0;try{let n=await Y_(t.sha256);if(!e.isConnected)return;if(!n)throw Error();let r=t.mediaType.split(`/`)[0],i=document.createElement(r===`image`?`img`:r===`audio`?`audio`:`video`),a=URL.createObjectURL(n);E.add(a),i.src=a,i.style.maxWidth=`100%`,i.style.maxHeight=`70vh`,r===`image`?i.alt=t.acquisition?.originalFilename||`Attached image`:(i.controls=!0,i.preload=`metadata`),e.after(i),e.hidden=!0,i.tabIndex=0,i.focus()}catch{e.isConnected&&(e.disabled=!1,e.textContent=`Media unavailable. Retry`)}}}function A(t=!1){D();let r=s,o=p,c=d??i(),u=c.indexOf(r.recordId),f={mother:S,owner:C,archive:w}[o](r);e.innerHTML=`
      <div class="sheet" style="--rsize:${l}px">
        <div class="rbar">
          <div class="rnav">
            <button data-step="-1" ${u<=0?`disabled`:``} aria-label="Later record">‹ Later</button>
            <button data-step="1" ${u<0||u>=c.length-1?`disabled`:``} aria-label="Earlier record">Earlier ›</button>
          </div>
          <div class="rnav">
            <button data-size="-1" aria-label="Smaller text" ${l<=iv[0]?`disabled`:``}>A−</button>
            <button data-size="1" aria-label="Larger text" ${l>=iv[iv.length-1]?`disabled`:``}>A+</button>
            ${a?`<button data-walk ${n(r.recordId)?``:`disabled`}>Walk there</button>`:``}
            <button class="close" data-close aria-label="Close the reader">×</button>
          </div>
        </div>
        ${_(r)}
        ${r.loading?`<p class="rnote" role="status">Reading the full record…</p>`:r.partial?`<p class="rnote" role="status">Showing what this garden snapshot holds. The full record could not be read just now.</p>`:``}
        ${v(o)}
        <div class="rpanel" id="reader-panel" role="tabpanel" aria-labelledby="lens-${o}" tabindex="0">${f}${O(r)}</div>
        <footer class="rfoot">
          ${o===`archive`?``:`<button class="rhow" data-how>How do we know? <span aria-hidden="true">→</span> The Archive</button>`}
          ${T(r)}
        </footer>
      </div>`,t&&e.querySelector(`#lens-${o}`)?.focus()}function j(e){c=p=e,m=s?.recordId??null,lv(`garden-reader-lens`,c)}function M(t){m!==t.recordId&&(p=g(t),m=null),s=t,A(),h().scrollTop=0,e.querySelector(`#reader-title`)?.focus()}async function N(i){let a=++f,o=n(i);o&&M({...fv(o,r()),loading:!0});let s=await t(i).catch(()=>null);a!==f||e.hidden||(s?M(s):o?M(fv(o,r())):(e.innerHTML=`<div class="sheet"><div class="rbar"><span></span><button class="close" data-close aria-label="Close the reader">×</button></div><p class="rempty">This record could not be read.</p></div>`,e.querySelector(`[data-close]`).focus()))}function P(t,n=document.activeElement,r=null){e.hidden&&(u=n,d=r),document.pointerLockElement&&document.exitPointerLock(),e.hidden=!1,N(t)}function F(){if(e.hidden)return;f++,D(),e.hidden=!0,e.innerHTML=``,s=null,d=null;let t=u;u=null,t?.isConnected&&t.focus()}let I=(t,n)=>e.addEventListener(t,n,{signal:o});return I(`click`,t=>{if(t.target===e)return F();let n=t.target.closest(`button`);if(n&&!n.disabled){if(n.dataset.attachment!==void 0)k(n);else if(n.dataset.close!==void 0)F();else if(n.dataset.lens)j(n.dataset.lens),A(!0);else if(n.dataset.how!==void 0)j(`archive`),A(),e.querySelector(`#provenance`)?.focus();else if(n.dataset.read)N(n.dataset.read);else if(n.dataset.step){let e=d??i(),t=e[e.indexOf(s.recordId)+Number(n.dataset.step)];t&&N(t)}else if(n.dataset.size)l=iv[Math.min(iv.length-1,Math.max(0,iv.indexOf(l)+Number(n.dataset.size)))]??17,lv(`garden-reader-size`,String(l)),h().style.setProperty(`--rsize`,`${l}px`),A(),e.querySelector(`[data-size="${n.dataset.size}"]`)?.focus();else if(n.dataset.walk!==void 0){let e=s.recordId;u=null,F(),a(e)}else if(n.dataset.copy){let e=e=>{n.textContent=e,setTimeout(()=>n.isConnected&&(n.textContent=`Copy`),1500)};navigator.clipboard?.writeText(n.dataset.copy).then(()=>e(`Copied`),()=>e(`Copy failed`))??e(`Copy failed`)}}}),I(`keydown`,t=>{if(t.stopPropagation(),t.key===`Escape`)return t.preventDefault(),F();let n=t.target.closest?.(`[role="tab"]`);if(n&&[`ArrowLeft`,`ArrowRight`,`Home`,`End`].includes(t.key)){t.preventDefault();let e=ev.findIndex(([e])=>e===n.dataset.lens);j(ev[t.key===`Home`?0:t.key===`End`?ev.length-1:(e+(t.key===`ArrowRight`?1:-1)+ev.length)%ev.length][0]),A(!0)}else if(t.key===`Tab`){let n=[...e.querySelectorAll(`button:not([disabled]), [tabindex="0"], [href]`)].filter(e=>e.offsetParent!==null);if(!n.length)return;let r=n[0],i=n[n.length-1];t.shiftKey&&(document.activeElement===r||!e.contains(document.activeElement))?(t.preventDefault(),i.focus()):!t.shiftKey&&document.activeElement===i&&(t.preventDefault(),r.focus())}}),{open:P,close:F,get isOpen(){return!e.hidden},get recordId(){return s?.recordId??null}}}var mv=e=>String(e??``).replace(/[&<>"]/g,e=>({"&":`&amp;`,"<":`&lt;`,">":`&gt;`,'"':`&quot;`})[e]),hv=[`memory`,`decision`,`task`,`checkpoint`,`artifact`,`action`],gv={memory:`Memory`,decision:`Decision`,task:`Task`,checkpoint:`Checkpoint`,artifact:`Artifact`,action:`Action`},_v={memory:`Memories`,decision:`Decisions`,task:`Tasks`,checkpoint:`Checkpoints`,artifact:`Artifacts`,action:`Actions`},vv={retracted:`Retracted`,superseded:`Superseded`,cancelled:`Cancelled`},yv={memory:`#c6abf2`,decision:`#86d4c0`,task:`#f2a66e`,checkpoint:`#8db6f2`,artifact:`#e3cf7d`,action:`#ef93ae`,pinned:`#f0d9a0`},bv=e=>{let t=new Date(e);return Number.isNaN(t.getTime())?`an unrecorded date`:t.toLocaleDateString(`en-GB`,{day:`numeric`,month:`short`,year:`numeric`})},xv=e=>{let t=new Date(e);return Number.isNaN(t.getTime())?`Undated`:t.toLocaleDateString(`en-GB`,{month:`long`,year:`numeric`})};function Sv(e,t=240){let n=String(e??``).replace(/\s+/g,` `).trim();if(n.length<=t)return n;let r=n.slice(0,t);return`${r.slice(0,Math.max(r.lastIndexOf(` `),t*.6)).replace(/[\s.,;:!?]+$/,``)}…`}function Cv(e){return e.kind===`memory`?{owner_statement:`Captured from a conversation`,source:`From a source`,model_inference:`Mother's interpretation`}[e.basis]??`How it was captured is not recorded`:`Working record`}var wv=e=>e.kind===`memory`&&e.basis===`model_inference`&&e.status===`active`;function Tv(e,{snapshot:t,read:n,closeable:r=!0,banner:i=``,onOpen:a=()=>{},onClose:o=()=>{},signal:s}){let c=[],l=new Set,u=new Map,d=null,f=[],p={words:[],kinds:new Set(hv),retired:!1,arrange:`date`};function m(e){t=e,c=(e.records??[]).filter(e=>hv.includes(e.kind)),u=new Map;for(let e of c)if(wv(e))for(let t of new Set(e.references??[]))t!==e.id&&u.set(t,(u.get(t)??0)+1)}m(t);let h=e=>Date.parse(e.createdAt)||0,g=e=>{if(p.media&&!l.has(e.id)||!p.kinds.has(e.kind)||p.retired&&vv[e.status])return!1;if(!p.words.length)return!0;let t=`${e.title}\n${e.content}\n${e.status}\n${e.kind}`.toLowerCase();return p.words.every(e=>t.includes(e))};function _(){let e=c.filter(g).sort((e,t)=>h(t)-h(e)),t=e.filter(e=>e.pinned),n=e.filter(e=>!e.pinned),r=t.length?[[`Kept close`,t]]:[];if(p.arrange===`kind`)for(let e of hv){let t=n.filter(t=>t.kind===e);t.length&&r.push([_v[e],t])}else for(let e of n){let t=xv(e.createdAt);r.length&&r[r.length-1][0]===t&&r[r.length-1].month?r[r.length-1][1].push(e):r.push(Object.assign([t,[e]],{month:!0}))}return r}let v=()=>_().flatMap(([,e])=>e.map(e=>e.id));function y(e){let t=vv[e.status],n=u.get(e.id)??0,r=[`<span class="dot" style="background:${yv[e.pinned?`pinned`:e.kind]}"></span>${mv(gv[e.kind])}`,`Recorded ${bv(e.createdAt)}`,mv(Cv(e)),...l.has(e.id)?[`Has media`]:[],...n?[`${n} ${n===1?`reflection`:`reflections`}`]:[],...t?[`<span class="rpoff">${t}, kept as history</span>`]:[]],i=mv(e.id);return`
      <li>
        <button class="rpentry${t?` retired`:``}" data-id="${i}" aria-label="Read ${mv(e.title)}" aria-describedby="rpd-${i}">
          <span class="rpmeta">${r.join(`<span class="rpsep" aria-hidden="true">·</span>`)}</span>
          <span class="rptitle">${mv(e.title)}</span>
          <span class="rpexcerpt" id="rpd-${i}">${mv(Sv(e.content))}</span>
        </button>
      </li>`}function b(){let t=_(),n=t.reduce((e,[,t])=>e+t.length,0);e.querySelector(`.rpcount`).textContent=c.length?n===c.length?`${n} ${n===1?`record`:`records`}, newest first.`:`${n} of ${c.length} records shown.`:`No records yet.`,e.querySelector(`.rpgroups`).innerHTML=t.length?t.map(([e,t],n)=>`
        <section class="rpgroup" aria-labelledby="rpg-${n}">
          <h2 id="rpg-${n}">${mv(e)} <span class="rpn">${t.length}</span></h2>
          <ol class="rpitems">${t.map(y).join(``)}</ol>
        </section>`).join(``):`<p class="rpnone">${c.length?`Nothing matches. Try fewer words, or bring back a kind.`:`The garden has no records yet.`}</p>`}function x(){let n=t.shown??c.length,a=t.total>n?`<p class="rpnote">This garden view holds the ${n} most recent of ${t.total} records, with pinned ones first.</p>`:``;e.innerHTML=`
      <div class="rpwrap">
        <header class="rptop">
          <div>
            <p class="rpkicker">Memory Garden · read-only</p>
            <h1 id="rp-title" tabindex="-1">Reading Path</h1>
          </div>
          ${r?`<button class="rpback" data-back>Back to the garden</button>`:``}
        </header>
        ${i}
        ${a}
        <div class="rpcontrols">
          <label class="rpfind"><span>Search</span><input id="rp-q" type="search" autocomplete="off" spellcheck="false" placeholder="Words in a title or its text" /></label>
          <div class="rprow">
            <div class="rpseg" role="group" aria-label="Arrange the path">
              <button data-arrange="date" aria-pressed="${p.arrange===`date`}">By month</button>
              <button data-arrange="kind" aria-pressed="${p.arrange===`kind`}">By kind</button>
            </div>
            <div class="rpkinds" role="group" aria-label="Kinds shown">${hv.filter(e=>c.some(t=>t.kind===e)).map(e=>`<button data-kind="${e}" aria-pressed="${p.kinds.has(e)}"><span class="dot" style="background:${yv[e]}"></span>${_v[e]}</button>`).join(``)}</div>
            <label class="check"><input type="checkbox" id="rp-media" ${p.media?`checked`:``} /> Has media</label>
            <label class="check"><input type="checkbox" id="rp-retired" ${p.retired?`checked`:``} /> Set aside retired records</label>
          </div>
        </div>
        <p class="rpcount" role="status" aria-live="polite"></p>
        <div class="rpgroups"></div>
      </div>`,e.querySelector(`#rp-q`).value=p.words.join(` `),b()}let S=()=>[...e.querySelectorAll(`.rpentry`)],C=(t,n)=>e.addEventListener(t,n,{signal:s});C(`input`,e=>{e.target.id===`rp-q`?(p.words=e.target.value.trim().toLowerCase().split(/\s+/).filter(Boolean),b()):e.target.id===`rp-media`?(p.media=e.target.checked,b()):e.target.id===`rp-retired`&&(p.retired=e.target.checked,b())}),C(`click`,t=>{let r=t.target.closest(`button`);if(r){if(r.dataset.back!==void 0)T();else if(r.dataset.id)n(r.dataset.id,r,v());else if(r.dataset.arrange)p.arrange=r.dataset.arrange,e.querySelectorAll(`[data-arrange]`).forEach(e=>e.setAttribute(`aria-pressed`,e.dataset.arrange===p.arrange)),b();else if(r.dataset.kind){let e=r.dataset.kind;p.kinds.has(e)?p.kinds.delete(e):p.kinds.add(e),r.setAttribute(`aria-pressed`,p.kinds.has(e)),b()}}}),C(`keydown`,t=>{r&&t.stopPropagation();let n=t.target.id===`rp-q`;if(t.key===`Escape`){if(n&&t.target.value)return;r&&(t.preventDefault(),T());return}if(t.key===`/`&&!n&&t.target.tagName!==`INPUT`){t.preventDefault(),e.querySelector(`#rp-q`).focus();return}let i=S(),a=i.indexOf(t.target),o=-1;if(t.key===`ArrowDown`&&(a>=0||n))o=a+1;else if(t.key===`ArrowUp`&&a>=0)o=a-1;else if(t.key===`Home`&&a>=0)o=0;else if(t.key===`End`&&a>=0)o=i.length-1;else return;t.preventDefault(),o<0?e.querySelector(`#rp-q`).focus():i[Math.min(o,i.length-1)]?.focus()});function w(t=document.activeElement){if(e.hidden){if(d=t,x(),e.hidden=!1,J_().then(({images:t=[]})=>{l=new Set(t.flatMap(e=>(e.memoryLinks??[]).map(e=>e.memoryId))),e.hidden||b()}),r){f=[...e.parentElement.children].filter(t=>t!==e&&t.id!==`reader`&&!t.inert&&![`SCRIPT`,`NOSCRIPT`].includes(t.tagName));for(let e of f)e.inert=!0}a(),e.querySelector(`#rp-title`).focus()}}function T(){if(e.hidden||!r)return;e.hidden=!0,e.innerHTML=``;for(let e of f)e.inert=!1;f=[],o();let t=d;d=null,t?.isConnected&&t.focus()}function E(t){if(m(t),e.hidden)return;let n=document.activeElement?.closest?.(`.rpentry`)?.dataset.id,r=e.scrollTop;b(),e.scrollTop=r,n&&e.querySelector(`.rpentry[data-id="${window.CSS.escape(n)}"]`)?.focus()}return{open:w,close:T,update:E,get isOpen(){return!e.hidden}}}function Ev({snapshot:e,fetchSnapshot:t,fetchReading:n,reason:r}){document.body.classList.add(`reading-only`),document.getElementById(`loading`)?.remove();let i=r===`no-webgl`?`<p class="rpnote">This browser does not offer WebGL 2, so the 3D garden can’t be drawn here. Every record is on this page to read.</p>`:`<p class="rpintro">The records as text, without the 3D garden. <a href="./">Open the 3D garden</a></p>`,a=()=>(e.records??[]).filter(e=>hv.includes(e.kind)),o=pv(document.getElementById(`reader`),{fetchReading:n,snapshotOf:e=>a().find(t=>t.id===e),all:a,order:()=>a().map(e=>e.id)}),s=Tv(document.getElementById(`path`),{snapshot:e,read:(e,t,n)=>o.open(e,t,n),closeable:!1,banner:i});s.open(),document.title=`Reading Path · Memory Garden`;async function c(){try{if(document.hidden||o.isOpen)return;let n=await t(e.liveRevision);if(!n||n.unchanged||o.isOpen)return;e=n,s.update(n)}catch{}finally{setTimeout(c,5e3)}}return e.liveRevision&&setTimeout(c,5e3),{path:s,reader:o}}var Dv=Math.PI*2,Ov=[`memory`,`decision`,`task`,`checkpoint`,`artifact`,`action`],kv={memory:`Memories`,decision:`Decisions`,task:`Tasks`,checkpoint:`Checkpoints`,artifact:`Artifacts`,action:`Actions`,pinned:`Pinned`},Av={memory:`daisy`,decision:`tulip`,task:`poppy`,checkpoint:`bellflower`,artifact:`sunflower`,action:`rose`,pinned:`stone lantern`},jv={memory:`#c6abf2`,decision:`#86d4c0`,task:`#f2a66e`,checkpoint:`#8db6f2`,artifact:`#e3cf7d`,action:`#ef93ae`,pinned:`#f0d9a0`},Mv=new Set([`retracted`,`superseded`,`cancelled`]),Nv=e=>e.kind===`task`&&(e.status===`doing`||e.status===`blocked`)||e.kind===`action`&&(e.status===`pending`||e.status===`unknown`),Pv={memory:[[.74,.6,.98],[.4,.3,.7]],decision:[[.42,.86,.72],[.17,.5,.5]],task:[[.99,.56,.22],[.68,.27,.2]],checkpoint:[[.4,.58,.98],[.2,.29,.68]],artifact:[[1,.8,.2],[.74,.46,.14]],action:[[.98,.45,.62],[.66,.22,.42]],pinned:[[1,.94,.74],[.82,.62,.42]],gold:[[1,.82,.3],[.72,.46,.16]],dark:[[.26,.14,.2],[.12,.07,.12]],brown:[[.46,.28,.14],[.24,.14,.1]],green:[[.38,.62,.28],[.15,.33,.22]],leaf:[[.42,.7,.3],[.13,.33,.24]],pale:[[1,.96,.78],[.72,.68,.6]],wood:[[.56,.4,.26],[.27,.2,.2]],plank:[[.78,.62,.42],[.42,.32,.3]],stone:[[.74,.72,.66],[.4,.41,.52]],flame:[[2.6,1.7,.75],[2.2,1.4,.6]]},Fv={outer:.05,inner:.08,core:.11,green:.17};function Iv(e){let t=new Set(e.records.map(e=>e.id));return e.records.filter(e=>Ov.includes(e.kind)).map(e=>{let n=(e.references??[]).filter(n=>t.has(n)&&n!==e.id);return{id:e.id,kind:e.kind,status:e.status,title:e.title,content:e.content,truncated:e.truncated,revision:e.revision,pinned:!!e.pinned,created:Date.parse(e.createdAt)||0,refs:n,hiddenRefs:(e.references??[]).length-n.length,assigned:e.assignment?[e.assignment.assignee,e.assignment.assignedBy,e.assignment.assignedAt]:null,next:e.nextStep??``,actors:[...new Set([e.actor,...e.history.map(e=>e.actor)].filter(Boolean))],text:`${e.title}
${e.content}
${e.status}
${e.kind}`.toLowerCase(),src:e,h:e.history.map(t=>[t.revision,t.actor,t.at,t.changed,t.revision!==e.revision&&(!t.changed||t.changed.includes(`content`))?t.excerpt:``])}})}var Lv=new Fc(1,14,9),Rv=new ns(1,1,1,8),zv=new ns(.55,1,1,8),Bv=new rs(1,1,14,3,!0),Vv=()=>new J;function Hv(e,t,n=.3){let[r,i]=Pv[e],a=1-n*(1-t);return[[r[0]*a,r[1]*a,r[2]*(a+(1-a)*.35)],[i[0]*a,i[1]*a,i[2]*(a+(1-a)*.35)]]}function Uv(e,t,n,r,i=.25){let a=e.clone();a.computeBoundingBox();let{min:o,max:s}=a.boundingBox,c=s.y-o.y||1,l=a.attributes.position,u=l.count,d=new Float32Array(u*3),f=new Float32Array(u*3);for(let e=0;e<u;e++){let[t,r]=Hv(n,(l.getY(e)-o.y)/c,i);d.set(t,e*3),f.set(r,e*3)}return a.applyMatrix4(t),a.deleteAttribute(`uv`),a.setAttribute(`aLit`,new ui(d,3)),a.setAttribute(`aShade`,new ui(f,3)),a.setAttribute(`aInk`,new ui(new Float32Array(u).fill(r),1)),a}var Wv=(e,t,n,r,i,a,o=0,s=0)=>Vv().makeTranslation(e,t,n).multiply(Vv().makeRotationZ(o)).multiply(Vv().makeRotationX(s)).multiply(Vv().makeScale(r,i,a));function Gv(e,t,{len:n,wid:r,cup:i=.3,curl:a=0,round:o=.6,wave:s=0,deep:c=.45}){let l=[],u=[],d=[],f=[];for(let t=0;t<=7;t++){let f=t/7,p=r*.5*Math.sin(Math.PI*f**.62)**(1-.7*o)*(o+(1-o)*(1-f*f))+r*.04,[m,h]=Hv(e,f,c);for(let e=0;e<=4;e++){let t=e/4*2-1,r=f*n,o=t*p,c=i*p*t*t+a*n*f*f+s*p*Math.sin(t*3.1+f*5)*f;l.push(r,c,o);let g=1-.1*(1-Math.abs(t))*(1-f);u.push(m[0]*g,m[1]*g,m[2]*g),d.push(h[0],h[1],h[2])}}for(let e=0;e<7;e++)for(let t=0;t<4;t++){let n=e*5+t;f.push(n,n+4+1,n+1,n+1,n+4+1,n+4+2)}let p=new Z;return p.setAttribute(`position`,new X(l,3)),p.setAttribute(`aLit`,new X(u,3)),p.setAttribute(`aShade`,new X(d,3)),p.setAttribute(`aInk`,new X(new Float32Array(l.length/3).fill(t),1)),p.setIndex(f),p.computeVertexNormals(),p}function Kv(e,t,n,r,i,a,{base:o=Vv().makeTranslation(0,1,0),y:s=0,r0:c=0,turn:l=0,jitter:u=.08}={}){for(let d=0;d<t;d++){let f=o.clone().multiply(Vv().makeTranslation(0,s,0)).multiply(Vv().makeRotationY(d/t*Dv+l+Math.sin(d*12.9898)*u)).multiply(Vv().makeRotationZ(n+Math.sin(d*78.233)*u*.6)).multiply(Vv().makeTranslation(c,0,0));e.push(Gv(r,i,a).applyMatrix4(f))}}function qv(e,t,n,r,i,a,{y:o=1,lift:s=0,base:c=Vv()}={}){for(let l=0;l<t;l++){let u=l*2.39996,d=n*Math.sqrt((l+.5)/t);e.push(Uv(Lv,c.clone().multiply(Wv(Math.cos(u)*d,o+s*(1-d/n),Math.sin(u)*d,r,r,r)),i,a,.1))}}function Jv(e){if(e===`pinned`)return Yv();let t=[Uv(zv,Wv(0,.5,0,.017,1,.017),`green`,Fv.green,.35)],n={len:.32,wid:.1,cup:.25,curl:-.25,round:.15,deep:.35};if(t.push(Gv(`leaf`,Fv.green,n).applyMatrix4(Vv().makeTranslation(0,.26,0).multiply(Vv().makeRotationZ(.75)))),t.push(Gv(`leaf`,Fv.green,{...n,len:.27}).applyMatrix4(Vv().makeTranslation(0,.48,0).multiply(Vv().makeRotationY(2.6)).multiply(Vv().makeRotationZ(.65)))),e!==`checkpoint`&&Kv(t,5,-.5,`leaf`,Fv.green,{len:.07,wid:.045,cup:.4,round:.3},{y:-.015}),e===`memory`)Kv(t,14,.12,e,Fv.outer,{len:.26,wid:.06,cup:.35,curl:-.08,round:.9},{r0:.05}),Kv(t,11,.32,e,Fv.inner,{len:.2,wid:.055,cup:.35,curl:-.05,round:.9},{r0:.04,y:.01,turn:.14}),t.push(Uv(Lv,Wv(0,1.025,0,.068,.045,.068),`gold`,Fv.core,.4));else if(e===`decision`)Kv(t,3,1.18,e,Fv.outer,{len:.27,wid:.2,cup:.9,curl:.15,round:.75,deep:.55},{r0:.015}),Kv(t,3,1.3,e,Fv.inner,{len:.25,wid:.18,cup:.9,curl:.12,round:.75,deep:.55},{r0:.01,turn:Dv/6});else if(e===`task`)Kv(t,4,.22,e,Fv.outer,{len:.25,wid:.3,cup:.22,curl:.06,round:1,wave:.14,deep:.55}),Kv(t,4,.5,e,Fv.inner,{len:.2,wid:.25,cup:.3,curl:.06,round:1,wave:.12,deep:.55},{y:.01,turn:Dv/8}),t.push(Uv(Lv,Wv(0,1.045,0,.042,.04,.042),`dark`,Fv.core,.1)),qv(t,14,.065,.011,`dark`,Fv.core,{y:1.05,lift:.01});else if(e===`checkpoint`){t.push(Uv(Rv,Wv(0,1,0,.012,.12,.012),`green`,Fv.green));for(let n=0;n<3;n++){let r=n*2.1+.4,i=Vv().makeRotationY(-r).multiply(Vv().makeTranslation(.15,.98-n*.06,0)).multiply(Vv().makeRotationZ(-.35));t.push(Uv(Rv,Vv().makeRotationY(-r).multiply(Wv(.075,1-n*.03,0,.007,.16,.007,-1.1)),`green`,Fv.green)),t.push(Uv(Bv,i.clone().multiply(Wv(0,-.1,0,.105,.22,.105)),e,n===1?Fv.inner:Fv.outer,.45)),Kv(t,5,-.55,e,n===1?Fv.inner:Fv.outer,{len:.08,wid:.08,cup:-.3,round:.2,deep:.1},{base:i.clone().multiply(Vv().makeTranslation(0,-.205,0)),r0:.085}),qv(t,1,.001,.018,`pale`,Fv.core,{base:i,y:-.19})}}else if(e===`artifact`){let n=[],r={base:Vv().makeTranslation(0,1,0)};Kv(n,18,.05,e,Fv.outer,{len:.19,wid:.07,cup:.3,curl:-.08,round:.4},{...r,r0:.12}),Kv(n,14,.18,e,Fv.inner,{len:.16,wid:.07,cup:.3,curl:-.05,round:.4},{...r,r0:.11,y:.008,turn:.11}),n.push(Uv(Lv,Wv(0,1,0,.14,.035,.14),`brown`,Fv.core,.5)),qv(n,26,.12,.012,`dark`,Fv.core,{y:1.026,lift:.012});let i=Vv().makeTranslation(0,1,0).multiply(Vv().makeRotationX(.85)).multiply(Vv().makeTranslation(0,-1,.04));for(let e of n)t.push(e.applyMatrix4(i))}else e===`action`&&[[5,.5,.17,.17,.5,0],[5,.85,.14,.14,.7,.01],[4,1.15,.11,.12,.9,.02],[3,1.4,.08,.1,1.1,.03]].forEach(([n,r,i,a,o,s],c)=>Kv(t,n,r,e,c%2?Fv.inner:Fv.outer,{len:i,wid:a,cup:o,curl:.18,round:1,deep:.5},{y:s,turn:c*.7}));let r=eg(t);for(let e of t)e.dispose();return r}function Yv(){let e=[Uv(Rv,Wv(0,.04,0,.24,.08,.24),`stone`,Fv.green,.3),Uv(zv,Wv(0,.36,0,.08,.56,.08),`stone`,Fv.inner,.3),Uv(Rv,Wv(0,.66,0,.21,.05,.21),`stone`,Fv.green,.2),Uv(Rv,Wv(0,.79,0,.13,.21,.13),`flame`,Fv.core,0),Uv(Bv,Wv(0,.98,0,.3,.2,.3),`stone`,Fv.outer,.3),Uv(Lv,Wv(0,1.1,0,.05,.05,.05),`stone`,Fv.inner,.2)],t=eg(e);for(let t of e)t.dispose();return t}function Xv(){let e=[Uv(Rv,Wv(0,.62,0,.045,1.24,.045),`wood`,Fv.green,.35),Uv(Rv,Wv(0,1.06,.06,.34,.04,.12,0,Math.PI/2),`plank`,Fv.outer,.15),Uv(Rv,Wv(0,1.27,0,.06,.04,.06),`wood`,Fv.inner,.2)],t=eg(e);for(let t of e)t.dispose();return t}var Zv=`
uniform float uTime;
uniform float uFocus;
in vec3 aLit;
in vec3 aShade;
in float aInk;
in float aState;
in vec2 aMark;
out vec3 vWorld;
out float vMark;
out vec3 vN;
out vec3 vLit;
out vec3 vShade;
out float vInk;
out float vState;
out float vFocus;
void main() {
  vFocus = abs(float(gl_InstanceID) - uFocus) < 0.5 ? 1.0 : 0.0;
  mat4 m = modelMatrix * instanceMatrix;
  // (the flower you are looking at stands a little taller)
  // (grown: 0 before its time on the timeline, 1 once it is there; marked: 1 found, -1 set aside by a filter)
  vMark = aMark.y;
  vec4 w = m * vec4(position * aMark.x * (1.0 + 0.1 * vFocus + 0.08 * max(aMark.y, 0.0) - 0.3 * max(-aMark.y, 0.0)), 1.0);
  vec3 foot = m[3].xyz;
  // a slow nod in the wind, more at the head than the foot
  float up = max(w.y - foot.y, 0.0);
  float sway = sin(uTime * 1.5 + foot.x * 0.61 + foot.z * 0.47) + 0.5 * sin(uTime * 2.7 + foot.z * 0.9);
  // (stone and wood do not nod)
  if (aState > 2.5) sway = 0.0;
  w.x += sway * 0.03 * up * up;
  w.z += sway * 0.018 * up * up;
  vWorld = w.xyz;
  vN = normalize(mat3(m) * normal);
  vLit = aLit;
  vShade = aShade;
  vInk = aInk;
  vState = aState;
  gl_Position = projectionMatrix * viewMatrix * w;
}
`,Qv=`
${ng}
in vec3 vWorld;
in vec3 vN;
in vec3 vLit;
in vec3 vShade;
in float vInk;
in float vState;
in float vFocus;
in float vMark;
layout(location = 0) out vec4 outC;
void main() {
  vec3 N = normalize(vN);
  vec3 V = normalize(uCamPos - vWorld);
  if (dot(N, V) < 0.0) N = -N;
  // in the light or out of it, with a short soft turn between, as a painted figure is
  float open = smoothstep(0.35, 0.6, openAt(vWorld.xz));
  float lit = smoothstep(-0.02, 0.09, dot(N, uSunDir) + 0.06) * open;
  vec3 litPaint = vLit;
  vec3 shadePaint = vShade;
  if (vState > 0.5 && vState < 1.5) {
    // retired: the colour has gone out of it
    litPaint = mix(vec3(dot(litPaint, vec3(0.333))), litPaint, 0.18) * vec3(0.78, 0.76, 0.84);
    shadePaint = mix(vec3(dot(shadePaint, vec3(0.333))), shadePaint, 0.18) * vec3(0.8, 0.78, 0.9);
  }
  vec3 sun = uSunColor * 0.42 + uSkyColor * 0.3;
  float grey = dot(uSkyColor, vec3(0.333));
  vec3 sky = mix(uSkyColor, vec3(grey), 0.8) * 1.15 + 0.2;
  vec3 col = mix(shadePaint * sky, litPaint * sun, lit);
  // thin petals glow where the low sun comes through them from behind
  float through = pow(max(dot(-V, uSunDir), 0.0), 3.0) * (1.0 - lit) * open;
  col += litPaint * uSunColor * 0.32 * through * step(vInk, 0.12);
  // and catch a little of the sky along their edges
  float rim = pow(1.0 - abs(dot(N, V)), 3.0);
  col += litPaint * uSkyColor * 0.18 * rim;
  // unsettled work breathes
  if (vState > 1.5 && vState < 2.5) col *= 1.0 + 0.22 * sin(uTime * 2.6 + vWorld.x);
  col *= 1.0 + 0.3 * vFocus;
  // found by a search: lit warmly from within; set aside by a filter: faded into the meadow
  col = mix(col, col * 1.25 + litPaint * 0.18, max(vMark, 0.0));
  float aside = max(-vMark, 0.0);
  col = mix(col, mix(vec3(dot(col, vec3(0.333))), col, 0.25) * 0.82, aside * 0.85);
  outC = vec4(applyFog(col, vWorld), vInk);
}
`,$v=`
uniform float uTime;
uniform vec3 uSunDir;
in vec4 aHome;
in vec2 aInfo;
out vec2 vCorner;
out float vKind;
out float vBright;
void main() {
  vec3 p = aHome.xyz;
  float seed = aHome.w;
  float dusk = 1.0 - smoothstep(0.04, 0.32, uSunDir.y);
  if (aInfo.y > 1.5) {
    // a search's beacon: a soft light hanging over the flower that was found, rising and falling slowly
    p.y += sin(uTime * 1.6 + seed * 6.0) * 0.12;
    vBright = 0.85 + 0.15 * sin(uTime * 3.0 + seed * 9.0);
  } else if (aInfo.y > 0.5) {
    // a firefly wanders round its home, blinking slowly, and only once the light goes
    float t = uTime * (0.25 + 0.2 * fract(seed * 7.3)) + seed * 40.0;
    p += vec3(sin(t) * 0.9 + sin(t * 2.3) * 0.25, sin(t * 1.7) * 0.35, cos(t * 0.8) * 0.9);
    float blink = smoothstep(0.35, 1.0, sin(uTime * (1.1 + fract(seed * 3.1)) + seed * 17.0));
    vBright = dusk * (0.45 + 0.55 * blink);
  } else {
    // a lantern's window: always lit, and plainly so as the sun goes down
    vBright = 0.35 + 0.65 * dusk + 0.05 * sin(uTime * 9.0 + seed * 30.0);
  }
  vKind = aInfo.y;
  vCorner = position.xy;
  vec3 right = vec3(viewMatrix[0][0], viewMatrix[1][0], viewMatrix[2][0]);
  vec3 up = vec3(viewMatrix[0][1], viewMatrix[1][1], viewMatrix[2][1]);
  float size = vBright < 0.01 ? 0.0 : aInfo.x;
  vec3 c = (modelMatrix * vec4(p, 1.0)).xyz;
  // (brought a little towards the eye, so the stone round a lantern's window does not hide the glow's heart)
  c += normalize(cameraPosition - c) * (aInfo.y > 0.5 && aInfo.y < 1.5 ? 0.0 : 0.35);
  vec3 w = c + (right * position.x + up * position.y) * size;
  gl_Position = projectionMatrix * viewMatrix * vec4(w, 1.0);
}
`,ey=`
precision highp float;
in vec2 vCorner;
in float vKind;
in float vBright;
layout(location = 0) out vec4 outC;
void main() {
  float r = dot(vCorner, vCorner);
  if (r > 1.0) discard;
  float core = exp(-r * 18.0);
  float halo = exp(-r * 3.0) * (1.0 - r);
  vec3 warm = vKind > 1.5 ? vec3(1.0, 0.86, 0.6) : vKind > 0.5 ? vec3(0.8, 1.0, 0.4) : vec3(1.0, 0.66, 0.3);
  outC = vec4(warm * (core * 4.0 + halo * 1.1) * vBright, 0.0);
}
`,ty=7,ny=.8,ry=9,iy={month:`long`,year:`numeric`},ay=e=>e?new Date(e).toLocaleDateString(`en-GB`,iy):`Undated`;function oy(e){let t=e.records.filter(e=>Ov.includes(e.kind)&&!e.pinned).length;return Math.max(70,ty+t*ny+ry)}function sy(e,t){let n=e.filter(e=>e.pinned);n.forEach((e,t)=>{let r=t/n.length*Dv+.5;e.x=l_[0]+Math.cos(r)*4.6,e.z=l_[1]+Math.sin(r)*4.6,e.s=null});let r=e.filter(e=>!e.pinned).sort((e,t)=>t.created-e.created),i=[],a=null;r.forEach((e,t)=>{e.s=ty+t*ny;let n=C_(e.s),r=t%2?1:-1,o=r*(1.45+t*7%5*.34);e.x=n.x+n.tz*o,e.z=n.z-n.tx*o,e.away=[n.tz*r,-n.tx*r],e.along=[n.tx,n.tz];let s=ay(e.created);s!==a&&(i.push({s:Math.max(2,e.s-ny/2),text:t?s:`Today · ${s}`,post:!0}),a=s)}),r.length&&i.push({s:Math.min(x_(),ty+r.length*ny+2),text:`The first record`,post:!0});for(let n of e)n.y=t(n.x,n.z);let o=[];for(let e of r){let n=3+Math.floor(e.s*13.7%1*4);for(let r=0;r<n;r++){let n=Math.sin((e.s+r)*91.7)*.5+.5,i=.35+.5*((r*.618+n)%1),a=(r%2?1:-1)*(.28+.3*n),s=e.x+e.away[0]*i+e.along[0]*a,c=e.z+e.away[1]*i+e.along[1]*a;o.push({of:e,x:s,z:c,y:t(s,c),small:.32+.22*(n*7.1%1),turn:n*Dv})}}for(let e of i){let n=C_(e.s),r=n.x+n.ox*3.6,i=n.z+n.oz*3.6;Object.assign(e,{x:n.x,z:n.z,y:t(n.x,n.z),px:r,pz:i,py:t(r,i),ox:n.ox,oz:n.oz})}return{marks:i,line:r,companions:o}}var cy=e=>String(e).replace(/[&<>"]/g,e=>({"&":`&amp;`,"<":`&lt;`,">":`&gt;`,'"':`&quot;`})[e]),ly={day:`numeric`,month:`short`,year:`numeric`},uy=e=>{let t=new Date(e);return Number.isNaN(t.getTime())?``:`${t.toLocaleDateString(`en-GB`,ly)} ${t.toTimeString().slice(0,5)}`},dy=e=>document.getElementById(e);function fy({snapshot:e,field:t,scene:n,canvas:r,goTo:i,media:a=[],chime:o=()=>{},kinds:s=[...Ov,`pinned`]}){let c=new AbortController,{signal:l}=c,u=(e,t,n)=>e.addEventListener(t,n,{signal:l}),d=new gr;n.add(d);let f=Iv(e),p=new Map(f.map(e=>[e.id,e])),m=e=>f.filter(t=>t.refs.includes(e)).map(e=>e.id),{marks:h,line:g,companions:_}=sy(f,t.heightAt),v=[],y=new hr;for(let e of[...Ov,`pinned`]){let n=f.filter(t=>e===`pinned`?t.pinned:t.kind===e&&!t.pinned);if(!n.length)continue;let r=e===`pinned`?[]:_.filter(t=>t.of.kind===e),i=Jv(e),a=new Float32Array(n.length+r.length),o=new qa(i,new Qc({glslVersion:yt,side:2,uniforms:{...t.uniforms,uFocus:{value:-1}},vertexShader:Zv,fragmentShader:Qv}),n.length+r.length);n.forEach((t,n)=>{let r=Mv.has(t.status);t.tall=e===`pinned`?1.2:r?.62:.85+.09*Math.min(t.revision,5),t.mesh=o,t.index=n,a[n]=e===`pinned`?3:r?1:Nv(t)?2:0,y.position.set(t.x,t.y-.02,t.z),y.rotation.set(0,n*2.39996%Dv,0),y.scale.setScalar(t.tall),y.updateMatrix(),o.setMatrixAt(n,y.matrix)}),r.forEach((e,t)=>{let r=n.length+t;a[r]=a[e.of.index],y.position.set(e.x,e.y-.02,e.z),y.rotation.set(0,e.turn,0),y.scale.setScalar(e.of.tall*e.small),y.updateMatrix(),o.setMatrixAt(r,y.matrix)}),i.setAttribute(`aState`,new za(a,1));let s=new Float32Array((n.length+r.length)*2);for(let e=0;e<n.length+r.length;e++)s[e*2]=1;let c=new za(s,2);c.setUsage(dt),i.setAttribute(`aMark`,c),o.userData={owners:[...n,...r.map(e=>e.of)],mark:s,want:s.slice(),markAttr:c},o.frustumCulled=!1,d.add(o),v.push(o)}let b=h.filter(e=>e.post);if(b.length){let e=new Qc({glslVersion:yt,side:2,uniforms:{...t.uniforms,uFocus:{value:-1}},vertexShader:Zv,fragmentShader:Qv}),n=Xv();n.setAttribute(`aState`,new za(new Float32Array(b.length).fill(3),1)),n.setAttribute(`aMark`,new za(new Float32Array(b.length*2).map((e,t)=>t%2?0:1),2));let r=new qa(n,e,b.length);b.forEach((e,t)=>{y.position.set(e.px,e.py-.05,e.pz),y.rotation.set(0,Math.atan2(-e.ox,-e.oz),0),y.scale.setScalar(1),y.updateMatrix(),r.setMatrixAt(t,y.matrix)}),r.frustumCulled=!1,d.add(r)}let x={uTime:t.uniforms.uTime,uSunDir:t.uniforms.uSunDir};{let e=[],t=[],n=0;for(let r of f.filter(e=>e.pinned))e.push(r.x,r.y+.79*r.tall,r.z,n++*.618%1),t.push(.85,0);let r=f.filter(e=>!e.pinned),i=Math.min(420,60+r.length);for(let n=0;n<i&&r.length;n++){let a=r[Math.floor(n*r.length/i)%r.length],o=n*2.39996;e.push(a.x+Math.cos(o)*1.2,a.y+.5+n*.37%1.2,a.z+Math.sin(o)*1.2,n*.754877%1),t.push(.3,1)}if(e.length){let n=new Du;n.setAttribute(`position`,new X([-1,-1,0,1,-1,0,1,1,0,-1,1,0],3)),n.setIndex([0,1,2,0,2,3]),n.setAttribute(`aHome`,new za(new Float32Array(e),4)),n.setAttribute(`aInfo`,new za(new Float32Array(t),2)),n.instanceCount=e.length/4;let r=new xa(n,new Qc({glslVersion:yt,uniforms:x,vertexShader:$v,fragmentShader:ey,transparent:!0,depthWrite:!1,blending:5,blendEquation:100,blendSrc:201,blendDst:201,blendSrcAlpha:200,blendDstAlpha:201}));r.frustumCulled=!1,r.renderOrder=7,d.add(r)}}let S=(()=>{let e=f.length||1,t=new Du;t.setAttribute(`position`,new X([-1,-1,0,1,-1,0,1,1,0,-1,1,0],3)),t.setIndex([0,1,2,0,2,3]);let n=new za(new Float32Array(e*4),4),r=new za(new Float32Array(e*2),2);t.setAttribute(`aHome`,n),t.setAttribute(`aInfo`,r),t.instanceCount=0;let i=new xa(t,new Qc({glslVersion:yt,uniforms:x,vertexShader:$v,fragmentShader:ey,transparent:!0,depthWrite:!1,blending:5,blendEquation:100,blendSrc:201,blendDst:201,blendSrcAlpha:200,blendDstAlpha:201}));return i.frustumCulled=!1,i.renderOrder=7,d.add(i),e=>{e.forEach((e,t)=>{n.array.set([e.x,e.y+e.tall*1.25+.35,e.z,t*.618%1],t*4),r.array.set([.6,2],t*2)}),t.instanceCount=e.length,n.needsUpdate=r.needsUpdate=!0}})(),C=new Qc({glslVersion:yt,uniforms:{uTime:t.uniforms.uTime},vertexShader:`
      in float aT;
      out float vT;
      void main() { vT = aT; gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0); }`,fragmentShader:`
      precision highp float;
      uniform float uTime;
      in float vT;
      layout(location = 0) out vec4 outC;
      void main() {
        // a bead of light running along the thread, from the record to what it names
        float bead = pow(0.5 + 0.5 * sin((vT * 5.0 - uTime * 0.9) * 6.2832), 8.0);
        float ends = smoothstep(0.0, 0.05, vT) * smoothstep(1.0, 0.95, vT);
        outC = vec4(vec3(1.0, 0.82, 0.5) * (0.35 + bead * 1.4) * ends, 0.0);
      }`,transparent:!0,depthWrite:!1,blending:5,blendEquation:100,blendSrc:201,blendDst:201,blendSrcAlpha:200,blendDstAlpha:201,side:2}),w=new gr;w.renderOrder=7,d.add(w);function T(e){for(let e of w.children)e.geometry.dispose();if(w.clear(),!e)return;let t=e=>new q(e.x,e.y+e.tall*1.02,e.z);for(let[n,r]of[...e.refs.map(t=>[e,p.get(t)]),...m(e.id).map(t=>[p.get(t),e])]){let e=t(n),i=t(r),a=e.clone().add(i).multiplyScalar(.5);a.y+=1.5+e.distanceTo(i)*.18;let o=new zc(new Is(e,a,i),64,.016,5,!1),s=new Float32Array(o.attributes.position.count);for(let e=0;e<s.length;e++)s[e]=Math.floor(e/6)/64;o.setAttribute(`aT`,new ui(s,1));let c=new xa(o,C);c.frustumCulled=!1,c.renderOrder=7,w.add(c)}}let E=a.map(({item:e,blob:t})=>H_(e,t)),D=`
    ${ng}
    uniform sampler2D tPhoto;
    uniform float uLook;
    uniform float uDim;
    in vec2 vUv;
    in vec3 vWorld;
    in vec3 vN;
    layout(location = 0) out vec4 outC;
    void main() {
      vec3 c = texture(tPhoto, vUv).rgb;
      vec3 N = normalize(vN);
      vec3 V = normalize(uCamPos - vWorld);
      if (dot(N, V) < 0.0) N = -N;
      // lit as a painted board is: warm where the sun falls on it, never black in the shade
      float lit = smoothstep(-0.2, 0.6, dot(N, uSunDir));
      vec3 col = c * mix(uSkyColor * 0.55 + 0.45, uSunColor * 0.5 + 0.55, lit);
      col *= 1.0 + 0.22 * uLook;
      col = mix(col, vec3(dot(col, vec3(0.333))) * 0.8, uDim * 0.8);
      outC = vec4(applyFog(col, vWorld), 0.14);
    }`,O=[];if(E.length){for(let e=0;O.length<E.length&&e<8;e++){let n=1.55+e*.42,r=[];for(let e=0;e<720&&O.length<E.length;e++){let i=e/720*Dv,a=Math.cos(i),o=Math.sin(i),s=A_(d_.at[0]+a,d_.at[1]+o),c=n/Math.max(s,1e-4),l=d_.at[0]+a*c,u=d_.at[1]+o*c;if([...r,...O].some(e=>Math.hypot(e.x-l,e.z-u)<3.6))continue;let d={x:l,z:u,y:t.heightAt(l,u),yaw:Math.atan2(d_.at[0]-l,d_.at[1]-u)};r.push(d),O.push(d)}}E.forEach((e,t)=>Object.assign(e,O[t],{tall:1.75,created:e.at}));let e=[Uv(Rv,Wv(-.42,.95,.12,.026,1.95,.026,-.12,-.08),`wood`,Fv.green,.35),Uv(Rv,Wv(.42,.95,.12,.026,1.95,.026,.12,-.08),`wood`,Fv.green,.35),Uv(Rv,Wv(0,.9,-.32,.024,1.85,.024,0,.22),`wood`,Fv.inner,.35),Uv(Rv,Wv(0,.66,.1,.035,.84,.035,Math.PI/2),`plank`,Fv.outer,.2),Uv(new $o(1.34,1.34,.045),Wv(0,1.34,.06,1,1,1),`pale`,Fv.inner,.15),Uv(Rv,Wv(0,2.04,.08,.03,.2,.03,Math.PI/2),`wood`,Fv.outer,.2)],n=eg(e);for(let t of e)t.dispose();n.setAttribute(`aState`,new za(new Float32Array(E.length).fill(3),1)),n.setAttribute(`aMark`,new za(new Float32Array(E.length*2).map((e,t)=>t%2?0:1),2));let r=new qa(n,new Qc({glslVersion:yt,side:2,uniforms:{...t.uniforms,uFocus:{value:-1}},vertexShader:Zv,fragmentShader:Qv}),E.length);E.forEach((e,t)=>{y.position.set(e.x,e.y-.03,e.z),y.rotation.set(0,e.yaw,0),y.scale.setScalar(1),y.updateMatrix(),r.setMatrixAt(t,y.matrix)}),r.frustumCulled=!1,d.add(r);let i=(e,n,r,i,a)=>{if(l.aborted){n.close?.();return}let o=n instanceof HTMLCanvasElement?new Jo(n):new Fn(n);o.colorSpace=$e,o.flipY=a,o.anisotropy=4,o.needsUpdate=!0;let s=1.18/Math.max(r,i),c=new Qc({glslVersion:yt,side:2,uniforms:{...t.uniforms,tPhoto:{value:o},uLook:{value:0},uDim:{value:0}},vertexShader:`
    out vec2 vUv;
    out vec3 vWorld;
    out vec3 vN;
    void main() {
      vUv = uv;
      vec4 w = modelMatrix * vec4(position, 1.0);
      vWorld = w.xyz;
      vN = normalize(mat3(modelMatrix) * normal);
      gl_Position = projectionMatrix * viewMatrix * w;
    }`,fragmentShader:D});if(e.picture){let t=e.picture.material.uniforms.tPhoto.value;t.source.data.close?.(),t.dispose(),e.picture.material.dispose(),e.picture.geometry.dispose(),d.remove(e.picture)}let u=new xa(new jc(r*s,i*s),c);u.position.set(e.x+Math.sin(e.yaw)*.09,e.y+1.31,e.z+Math.cos(e.yaw)*.09),u.rotation.set(0,e.yaw,0),u.frustumCulled=!1,d.add(u),e.picture=u};for(let e of E)if(i(e,W_(e,256),1,1,!0),e.kind===`image`&&e.blob)createImageBitmap(e.blob,{imageOrientation:`flipY`}).then(t=>{let n=Math.min(1,1024/Math.max(t.width,t.height));if(n===1)return t;let r=Math.round(t.width*n),i=Math.round(t.height*n);return t.close(),createImageBitmap(e.blob,{imageOrientation:`flipY`,resizeWidth:r,resizeHeight:i,resizeQuality:`high`})}).then(t=>i(e,t,t.width,t.height,!1)).catch(()=>{});else if(e.kind===`video`&&e.url){let t=document.createElement(`video`);t.muted=!0,t.preload=`auto`,t.src=e.url,t.addEventListener(`loadeddata`,()=>t.currentTime=Math.min(1,(t.duration||2)/3),{once:!0,signal:l}),t.addEventListener(`seeked`,()=>{let n=document.createElement(`canvas`),r=512/Math.max(t.videoWidth,t.videoHeight,1);n.width=Math.max(1,Math.round(t.videoWidth*r)),n.height=Math.max(1,Math.round(t.videoHeight*r)),n.getContext(`2d`).drawImage(t,0,0,n.width,n.height),i(e,n,n.width,n.height,!0)},{once:!0,signal:l})}}let k=U_(dy(`viewer`),l);function A(e){o(),document.pointerLockElement&&document.exitPointerLock(),k.open(E,E.indexOf(e))}let j=f.length;dy(`count`).textContent=j?j<e.total?`Read-only · ${j} of ${e.total} records`:`Read-only · ${j} ${j===1?`record`:`records`}`:`Read-only · no records yet`;let M=dy(`labels`),N=h.map(e=>{let t=document.createElement(`div`);return t.className=`label bed`,t.textContent=e.text,M.appendChild(t),{el:t,at:e.post?new q(e.px,e.py+1.75,e.pz):new q(e.x,e.y+2.3,e.z)}}),P=dy(`era`),F=document.createElement(`div`);F.className=`label`,F.hidden=!0,M.appendChild(F);let I=dy(`card`),ee=dy(`list`),te=dy(`q`),ne=null,re=null,ie=e=>`<span class="dot" style="background:${jv[e.pinned?`pinned`:e.kind]}"></span>`,ae=e=>e.map(e=>`<button class="chip" data-go="${cy(e)}">${ie(p.get(e))}${cy(p.get(e).title)}</button>`).join(``);function oe(e){re!==e&&o(),re=e,T(e),ee.hidden=!0;let t=e.status===`blocked`||e.status===`unknown`||e.status===`pending`,n=e.h[e.h.length-1],r=[...e.refs,...m(e.id)];I.innerHTML=`
      <button class="close" aria-label="Close" data-close>×</button>
      <div class="pills"><span class="pill kind" style="background:${jv[e.kind]}">${cy(e.kind)}</span><span class="pill ${t?`warn`:Mv.has(e.status)?`bad`:``}">${cy(e.status)}</span>${e.pinned?`<span class="pill pinned">pinned</span>`:``}</div>
      <h2>${cy(e.title)}</h2>
      <p class="sub">rev ${cy(e.revision)}${n?` · ${cy(n[1])} · ${uy(n[2])}`:``}</p>
      <p class="content">${cy(e.content)}${e.truncated?` …`:``}</p>
      ${e.truncated?`<p class="sub">A long record: only its beginning is shown here.</p>`:``}
      ${e.assigned?`<div class="note"><b>Waiting for ${cy(e.assigned[0])}</b>Handed over by ${cy(e.assigned[1])}, ${uy(e.assigned[2])}</div>`:``}
      ${e.next?`<div class="note"><b>Next step</b>${cy(e.next)}</div>`:``}
      ${r.length?`<div class="rel"><h3>Linked · walk there</h3><div class="chips">${ae(r)}</div></div>`:``}
      ${e.hiddenRefs?`<p class="sub">${e.hiddenRefs===1?`1 linked record is`:`${e.hiddenRefs} linked records are`} outside this snapshot.</p>`:``}
      <button class="readit" data-read="${cy(e.id)}">Read this ${e.kind===`memory`?`memory`:`record`}</button>
      <p class="sub">Its sources, links and ${e.revision===1?`one revision`:`${cy(e.revision)} revisions`} are in the reader's Archive.</p>`,I.hidden=!1}function L(){re=null,T(null),I.hidden=ee.hidden=!0}function se(){let e=R.words.length,t=[[`Pinned core`,f.filter(e=>e.pinned)],...Ov.map(e=>[kv[e],f.filter(t=>t.kind===e&&!t.pinned)])],n=E.filter(t=>!e||be(t)),r=n.length?`<h3>Photos and recordings</h3>${n.map(e=>`<button class="item" data-photo="${E.indexOf(e)}"><span class="dot photo"></span>${cy(e.title)}</button>`).join(``)}`:``;dy(`items`).innerHTML=r+t.map(([t,n])=>{let r=n.filter(t=>!e||be(t));return r.length?`<h3>${t}</h3>${r.map(e=>`<div class="row"><button class="item ${Mv.has(e.status)?`off`:``}" data-go="${cy(e.id)}">${ie(e)}${cy(e.title)}</button><button class="readbtn" data-read="${cy(e.id)}" aria-label="Read ${cy(e.title)}">Read</button></div>`).join(``)}`:``}).join(``)||(r?``:`<p class="none">Nothing matches.</p>`)}function ce(){re=null,T(null),I.hidden=!0,se(),ee.hidden=!1,te.focus()}function le(e){if(e.photo){let n=e.x+Math.sin(e.yaw)*2.6,r=e.z+Math.cos(e.yaw)*2.6,a=t.heightAt(n,r)+1.68-(e.y+1.31);i(n,r,Math.atan2(-(e.x-n),-(e.z-r)),-Math.atan2(a,2.6)),A(e);return}let n,r;if(e.s===null){let t=Math.hypot(e.x-l_[0],e.z-l_[1])||1;n=e.x-(e.x-l_[0])/t*3,r=e.z-(e.z-l_[1])/t*3}else{let t=C_(e.s);n=t.x,r=t.z}let a=e.x-n,o=e.z-r,s=Math.hypot(a,o)||1,c=t.heightAt(n,r)+1.68-(e.y+e.tall);i(n,r,Math.atan2(-a,-o),-Math.atan2(c,s)),oe(e)}let ue=()=>document.pointerLockElement&&document.exitPointerLock();function de(){if(ne){if(ne.photo)return A(ne);oe(ne),ue()}}let fe=f.filter(e=>!e.pinned).sort((e,t)=>t.created-e.created),pe=[...f.filter(e=>e.pinned),...fe].map(e=>e.id),me=pv(dy(`reader`),{fetchReading:q_,snapshotOf:e=>p.get(e)?.src,all:()=>f.map(e=>e.src),order:()=>pe,walk:e=>{he.close(),p.has(e)&&le(p.get(e))},signal:l}),he=Tv(dy(`path`),{snapshot:e,read:(e,t,n)=>me.open(e,t,n),onOpen:()=>{ue(),L()},signal:l});u(dy(`pathBtn`),`click`,e=>he.open(e.currentTarget));for(let e of[I,ee])u(e,`click`,e=>{let t=e.target.closest(`[data-go]`),n=e.target.closest(`[data-photo]`),r=e.target.closest(`[data-read]`);r?me.open(r.dataset.read,r):t?le(p.get(t.dataset.go)):n?le(E[Number(n.dataset.photo)]):e.target.closest(`[data-close]`)&&L()});let R={kinds:new Set(s),harness:dy(`harness`).value,retired:dy(`retired`).checked,cutoff:1/0,words:[]},ge=[],_e=-1,ve=e=>e.photo||R.kinds.has(e.pinned?`pinned`:e.kind)&&(!R.harness||e.actors.includes(R.harness))&&!(R.retired&&Mv.has(e.status)),ye=e=>e.photo||e.pinned||e.created<=R.cutoff,be=e=>R.words.length>0&&R.words.every(t=>e.text.includes(t));function xe(){R.words=te.value.trim().toLowerCase().split(/\s+/).filter(Boolean),ge=R.words.length?[...E,...f].filter(e=>be(e)&&ve(e)&&ye(e)).sort((e,t)=>(t.created||0)-(e.created||0)):[];for(let e of E)e.picture&&(e.picture.material.uniforms.uDim.value=R.words.length&&!be(e)?1:0);_e=-1;for(let e of v){let{owners:t,want:n}=e.userData;t.forEach((e,t)=>{n[t*2]=+!!ye(e),n[t*2+1]=!ve(e)||R.words.length&&!be(e)?-1:+!!be(e)})}S(ge.filter(e=>!e.photo));let e=dy(`found`);e.hidden=!R.words.length,e.textContent=ge.length?`${ge.length} found · Enter to go to each`:`Nothing found`,ee.hidden||se()}u(te,`input`,xe),u(te,`keydown`,e=>{e.key===`Escape`?te.value?(te.value=``,xe()):te.blur():e.key===`Enter`&&ge.length&&(_e=(_e+(e.shiftKey?-1:1)+ge.length)%ge.length,dy(`found`).textContent=`${_e+1} of ${ge.length} · Enter for the next`,le(ge[_e]))});let Se=Object.fromEntries([...Ov,`pinned`].map(e=>[e,f.filter(t=>e===`pinned`?t.pinned:!t.pinned&&t.kind===e).length]));dy(`legend`).innerHTML=[...Ov,`pinned`].filter(e=>Se[e]).map(e=>`<button class="kind" data-kind="${e}" aria-pressed="${R.kinds.has(e)}">${ie(e===`pinned`?{pinned:!0}:{kind:e})}<span class="kn">${kv[e]}</span><span class="kf">${Av[e]}</span><span class="kc">${Se[e]}</span></button>`).join(``),u(dy(`legend`),`click`,e=>{let t=e.target.closest(`[data-kind]`);if(!t)return;let n=t.dataset.kind;R.kinds.has(n)?R.kinds.delete(n):R.kinds.add(n),t.setAttribute(`aria-pressed`,R.kinds.has(n)),xe()});let Ce=[...new Set(f.flatMap(e=>e.actors))].sort();dy(`harness`).innerHTML=`<option value="">Every harness</option>${Ce.map(e=>`<option value="${cy(e)}">${cy(e)}</option>`).join(``)}`,dy(`harness`).value=R.harness=Ce.includes(R.harness)?R.harness:``,u(dy(`harness`),`change`,e=>{R.harness=e.target.value,xe()}),u(dy(`retired`),`change`,e=>{R.retired=e.target.checked,xe()});let we=g.filter(e=>e.created),Te=we.length?we[we.length-1].created:0,Ee=we.length?we[0].created:0,De=dy(`time`),Oe={day:`numeric`,month:`short`,year:`numeric`};function ke(e){De.value=e,R.cutoff=e>=1e3?1/0:Te+(Ee-Te)*e/1e3,dy(`timeLabel`).textContent=e>=1e3?`Today`:new Date(R.cutoff).toLocaleDateString(`en-GB`,Oe),De.style.setProperty(`--p`,`${e/10}%`),xe()}let z=0,Ae=()=>{cancelAnimationFrame(z),z=0,dy(`play`).setAttribute(`aria-pressed`,`false`)};u(De,`input`,()=>{Ae(),ke(Number(De.value))}),u(dy(`play`),`click`,()=>{if(z)return Ae();let e=Number(De.value)>=1e3?0:Number(De.value),t=performance.now();dy(`play`).setAttribute(`aria-pressed`,`true`);let n=r=>{if(e=Math.min(1e3,e+(r-t)/15e3*1e3),t=r,ke(Math.round(e)),e>=1e3)return Ae();z=requestAnimationFrame(n)};z=requestAnimationFrame(n)}),dy(`timeline`).hidden=Ee<=Te,Ee>Te&&(dy(`ticks`).innerHTML=h.filter(e=>e.post&&e.text!==`The first record`).map(e=>{let t=g[Math.min(g.length-1,Math.max(0,Math.round((e.s-ty)/ny)))];return t?.created?`<span style="left:${((t.created-Te)/(Ee-Te)*100).toFixed(2)}%"></span>`:``}).join(``));let je=dy(`map`),B=je.getContext(`2d`),V=S_()+14,Me=[];for(let e=0;e<=x_();e+=1.5){let t=C_(e);Me.push([t.x,t.z])}let H=(e,t,n)=>[n/2+(e-l_[0])/V*(n/2),n/2+(t-l_[1])/V*(n/2)];function Ne(){let e=je.clientWidth;if(!e)return;let t=devicePixelRatio||1;je.width!==Math.round(e*t)&&(je.width=je.height=Math.round(e*t)),B.setTransform(t,0,0,t,0,0),B.clearRect(0,0,e,e),B.save(),B.beginPath(),B.arc(e/2,e/2,e/2-1,0,Dv),B.clip(),B.fillStyle=`rgba(92, 132, 72, 0.92)`,B.fillRect(0,0,e,e),B.strokeStyle=`rgba(244, 236, 214, 0.9)`,B.lineWidth=3,B.lineCap=B.lineJoin=`round`,B.beginPath(),Me.forEach(([t,n],r)=>{let[i,a]=H(t,n,e);r?B.lineTo(i,a):B.moveTo(i,a)}),B.stroke();for(let t of f){if(!ye(t))continue;let[n,r]=H(t.x,t.z,e);B.globalAlpha=ve(t)&&(!R.words.length||be(t))?1:.25,B.fillStyle=jv[t.pinned?`pinned`:t.kind],B.beginPath(),B.arc(n,r,be(t)?3:1.7,0,Dv),B.fill()}for(let t of E){let[n,r]=H(t.x,t.z,e);B.globalAlpha=!R.words.length||be(t)?1:.3,B.fillStyle=`#ffffff`;let i=be(t)?3.2:2.2;B.fillRect(n-i,r-i,i*2,i*2)}if(B.globalAlpha=1,re){let[t,n]=H(re.x,re.z,e);B.strokeStyle=`#fff8e8`,B.lineWidth=2,B.beginPath(),B.arc(t,n,5,0,Dv),B.stroke()}let[n,r]=H(Pe.x,Pe.z,e),i=Math.atan2(Fe.z,Fe.x);B.fillStyle=`rgba(255, 255, 255, 0.35)`,B.beginPath(),B.moveTo(n,r),B.arc(n,r,22,i-.5,i+.5),B.closePath(),B.fill(),B.fillStyle=`#fff8e8`,B.strokeStyle=`rgba(0, 0, 0, 0.45)`,B.lineWidth=1.5,B.beginPath(),B.arc(n,r,3.5,0,Dv),B.fill(),B.stroke(),B.restore()}u(je,`click`,e=>{let t=je.getBoundingClientRect(),n=null,r=8;for(let i of[...E,...f]){if(!ye(i))continue;let[a,o]=H(i.x,i.z,t.width),s=Math.hypot(a-(e.clientX-t.left),o-(e.clientY-t.top));s<r&&(r=s,n=i)}n&&le(n)}),u(dy(`listBtn`),`click`,()=>ee.hidden?ce():L()),u(document,`keydown`,e=>{e.target instanceof HTMLInputElement||e.target instanceof HTMLSelectElement||(e.key===`/`?(e.preventDefault(),ue(),te.focus(),te.select()):e.code===`KeyE`?de():e.code===`KeyR`&&(re||ne)&&!(re??ne).photo?(ue(),me.open((re??ne).id)):e.code===`KeyL`?(e.preventDefault(),ue(),ee.hidden?ce():L()):e.key===`Escape`&&L())}),u(r,`mousedown`,e=>{e.button===0&&document.pointerLockElement===r&&de()}),u(document,`pointerlockchange`,()=>{document.pointerLockElement===r&&L()});let Pe=new q,Fe=new q,Ie=new q,Le=new q;function Re(e,t,n){Le.copy(t).project(n),Le.z>1||Le.z<-1?e.hidden=!0:(e.hidden=!1,e.style.left=`${(Le.x+1)/2*innerWidth}px`,e.style.top=`${(1-Le.y)/2*innerHeight}px`)}function ze(e,t){t.getWorldPosition(Pe),t.getWorldDirection(Fe);let n=null,r=.94;for(let e of f){if(!ye(e))continue;Ie.set(e.x-Pe.x,e.y+e.tall*.9-Pe.y,e.z-Pe.z);let t=Ie.length();if(t>16||t<.3)continue;let i=Ie.dot(Fe)/t-t*.0012;i>r&&(r=i,n=e)}for(let e of E){Ie.set(e.x-Pe.x,e.y+1.31-Pe.y,e.z-Pe.z);let t=Ie.length();if(t>14||t<.3)continue;let i=Ie.dot(Fe)/t-t*.0012+.01;i>r&&(r=i,n=e)}for(let t of E)t.picture&&(t.picture.material.uniforms.uLook.value+=(+(t===n)-t.picture.material.uniforms.uLook.value)*Math.min(1,e*8));let i=re??n;n!==ne&&(ne=n,dy(`hint`).hidden=!ne);for(let e of v)e.material.uniforms.uFocus.value=i&&i.mesh===e?i.index:-1;i?(F.textContent=i.title,Re(F,Le.set(i.x,i.y+(i.photo?2.25:i.tall+.42),i.z).clone(),t)):F.hidden=!0;for(let e of N)Math.hypot(e.at.x-Pe.x,e.at.z-Pe.z)>34?e.el.hidden=!0:Re(e.el,e.at,t);let a=null,o=9;for(let e of g){let t=Math.hypot(e.x-Pe.x,e.z-Pe.z);t<o&&(o=t,a=e)}let s=a?ay(a.created):``;P.textContent!==s&&(P.textContent=s);let c=1-Math.exp(-e*5);for(let e of v){let{mark:t,want:n,markAttr:r}=e.userData,i=!1;for(let e=0;e<t.length;e++){let r=n[e]-t[e];r!==0&&(t[e]=Math.abs(r)<.002?n[e]:t[e]+r*c,i=!0)}i&&(r.needsUpdate=!0)}Ne(),document.body.classList.toggle(`listing`,!ee.hidden)}let Be=()=>me.isOpen||he.isOpen||[`INPUT`,`SELECT`].includes(document.activeElement?.tagName);ke(1e3);function Ve(){c.abort(),Ae(),k.close(),me.close(),he.close(),N.forEach(({el:e})=>e.remove()),F.remove(),E.forEach(({url:e})=>URL.revokeObjectURL(e));let e=new Set([C]);d.traverse(t=>{t.geometry?.dispose(),t.isInstancedMesh&&t.dispose(),t.material&&e.add(t.material)});for(let t of e){let e=t.uniforms?.tPhoto?.value;e?.source.data.close?.(),e?.dispose(),t.dispose()}n.remove(d)}return{records:f,marks:h,update:ze,visit:le,typing:Be,selectedKinds:()=>[...R.kinds],canRefresh:()=>!k.isOpen&&!me.isOpen&&!he.isOpen&&I.hidden&&ee.hidden&&!Be()&&!z&&R.cutoff===1/0,dispose:Ve,flowers:Av}}var py=Math.PI*2;function my(e){let t=e>>>0;return()=>{t=t+1831565813>>>0;let e=t;return e=Math.imul(e^e>>>15,e|1),e^=e+Math.imul(e^e>>>7,e|61),((e^e>>>14)>>>0)/4294967296}}function hy(e,t,n,r){let i=(e,t,n)=>{let i=Math.imul(e,374761393)^Math.imul(t,668265263)^Math.imul(n,2147483647)^Math.imul(r,1274126177);return i=Math.imul(i^i>>>13,1274126177),((i^i>>>16)>>>0)/4294967296},a=Math.floor(e),o=Math.floor(t),s=Math.floor(n),c=e=>e*e*(3-2*e),l=c(e-a),u=c(t-o),d=c(n-s),f=(e,t,n)=>e+(t-e)*n;return f(f(f(i(a,o,s),i(a+1,o,s),l),f(i(a,o+1,s),i(a+1,o+1,s),l),u),f(f(i(a,o,s+1),i(a+1,o,s+1),l),f(i(a,o+1,s+1),i(a+1,o+1,s+1),l),u),d)}function gy(e){let t=my(e),n=[];for(let e=0;e<3;e++){let e=t()*py,r=-.2+1.1*t(),i=Math.sqrt(Math.max(0,1-r*r));n.push([Math.cos(e)*i,r,Math.sin(e)*i,.62+.25*t()])}let r=(t,r,i)=>{let a=.78+.34*hy(t*1.4+9,r*1.4,i*1.4,e)+.1*hy(t*4.1,r*4.1+3,i*4.1,e+7),o=t*a,s=r*a*.72,c=i*a;for(let[e,t,r,i]of n){let n=o*e+s*t+c*r-i;n>0&&(o-=e*n,s-=t*n,c-=r*n)}return[o,Math.max(s,-.3),c]},i=new Oc(1,3).attributes.position,a=new Float32Array(i.count*3),o=new Float32Array(i.count*3),s=new q,c=new q,l=new q;for(let e=0;e<i.count;e++){s.fromBufferAttribute(i,e).normalize();let t=r(s.x,s.y,s.z);a.set(t,e*3),c.set(s.y,-s.x,0),c.lengthSq()<1e-4&&c.set(1,0,0),c.normalize(),l.crossVectors(s,c);let n=.045,u=r(...s.clone().addScaledVector(c,n).normalize().toArray()),d=r(...s.clone().addScaledVector(l,n).normalize().toArray());c.set(u[0]-t[0],u[1]-t[1],u[2]-t[2]),l.set(d[0]-t[0],d[1]-t[1],d[2]-t[2]),c.cross(l).normalize(),c.dot(s)<0&&c.negate(),o.set([c.x,c.y,c.z],e*3)}let u=new Z;return u.setAttribute(`position`,new ui(a,3)),u.setAttribute(`normal`,new ui(o,3)),u}var _y=`
${ng}
out vec3 vWorld;
out vec3 vNormal;
out vec3 vObj;
void main() {
  mat4 m = modelMatrix * instanceMatrix;
  vec4 w = m * vec4(position, 1.0);
  vWorld = w.xyz;
  vNormal = normalize(mat3(m) * normal);
  vObj = position * length(m[0].xyz) + m[3].xyz * 0.37;
  gl_Position = projectionMatrix * viewMatrix * w;
}
`,vy=`
${ng}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
in vec3 vObj;
void main() {
  vec3 N = normalize(vNormal);
  float n1 = texture(tNoise, vObj.xy * 0.31 + vObj.z * 0.17).r;
  float n2 = texture(tNoise, vObj.zx * 1.1 + vObj.y * 0.4).g;
  // light and shade as two paints with a narrow turn; under a tree or a cloud it is all in shade
  float open = cloudAt(vWorld.xz);
  float lit = smoothstep(0.02, 0.2, dot(N, uSunDir) + (n1 - 0.5) * 0.28) * smoothstep(0.42, 0.78, open);
  vec3 light = mix(vec3(0.36, 0.35, 0.33), vec3(0.54, 0.51, 0.45), n2 * 0.6 + n1 * 0.4);
  vec3 shade = mix(vec3(0.27, 0.3, 0.3), vec3(0.36, 0.38, 0.37), n2);
  // lichen where it looks at the sky, moss low down
  float lichen = smoothstep(0.52, 0.66, N.y * 0.45 + n1 * 0.7);
  light = mix(light, vec3(0.42, 0.47, 0.2), lichen * 0.75);
  shade = mix(shade, vec3(0.09, 0.19, 0.12), lichen * 0.75);
  vec3 col = mix(shade * (uSkyColor * 1.25 + dot(uSkyColor, vec3(0.08))), light * (uSunColor * 0.36 + uSkyColor * 0.4), lit);
  // it stands in grass: green comes up it, and its foot is dark
  float foot = 1.0 - smoothstep(0.0, 0.45, vWorld.y - groundAt(vWorld.xz));
  col = mix(col, vec3(0.03, 0.1, 0.035) * (uSkyColor + uSunColor * 0.25 * open), foot * 0.7);
  outC = vec4(applyFog(col, vWorld), 1.0);
}
`;function yy(e){let t=my(424),n=[];for(let e=0;e<28;e++){let e=t()*py,r=S_()+12+230*t()**1.3;n.push([Math.cos(e)*r,Math.sin(e)*r])}let r=[];for(let[e,i]of n){let n=3+Math.floor(t()*7),a=4+t()*7;for(let o=0;o<n;o++){let n=t()*py,s=a*Math.sqrt(t()),[c,l]=f_(e+Math.cos(n)*s,i+Math.sin(n)*s);if(w_(c,l)<1.7||A_(c,l)<1.12||T_(c,l))continue;let u=o<2?.7+.9*t():.2+.45*t();r.push([c,l,u,t()*py,Math.floor(t()*3),.7+.5*t(),(t()-.5)*.3])}}let i=new gr,a=new Qc({glslVersion:yt,vertexShader:_y,fragmentShader:vy,uniforms:e.uniforms}),o=new J,s=new gn,c=new gn;for(let e=0;e<3;e++){let t=r.filter(t=>t[4]===e);if(!t.length)continue;let n=new qa(gy(11+e*17),a,t.length);t.forEach(([e,t,r,i,,a,l],u)=>{s.setFromAxisAngle(new q(0,1,0),i),c.setFromAxisAngle(new q(1,0,0),l),s.multiply(c),o.compose(new q(e,I_(e,t)+r*.08,t),s,new q(r,r*a,r)),n.setMatrixAt(u,o)}),n.frustumCulled=!1,i.add(n)}return{group:i,solid:r.filter(e=>e[2]>.6).map(([e,t,n])=>[e,t,n*.82])}}var by=`
${ng}
in vec4 aSeed;
uniform vec3 uBox;
out vec2 vQ;
out vec3 vWorld;
out vec2 vLook;  // how much of it shows, petal or fluff
void main() {
  float petal = step(0.72, aSeed.w);
  // carried on the wind, each at its own pace, in a box that goes with you
  vec2 gone = uWind.xy * uWind.z * uTime * (0.4 + 0.5 * fract(aSeed.w * 7.0));
  vec2 xz = aSeed.xz * uBox.xz + gone;
  xz = mod(xz - uCamPos.xz + uBox.xz * 0.5, uBox.xz) - uBox.xz * 0.5 + uCamPos.xz;
  float gust = gustAt(xz);
  xz += vec2(sin(uTime * 0.9 + aSeed.z * 30.0), cos(uTime * 0.7 + aSeed.x * 30.0)) * 0.45;
  float y = groundAt(xz) + 0.35 + aSeed.y * aSeed.y * uBox.y + sin(uTime * (0.5 + aSeed.w) + aSeed.x * 40.0) * 0.4 + gust * 0.7;
  vec3 at = vec3(xz.x, y, xz.y);
  float dist = distance(at, uCamPos);
  // never smaller than a pixel or so, or it flickers
  float size = max(mix(0.011, 0.019, fract(aSeed.w * 13.0)), dist * 0.001);
  vec3 cr = vec3(viewMatrix[0][0], viewMatrix[1][0], viewMatrix[2][0]);
  vec3 cu = vec3(viewMatrix[0][1], viewMatrix[1][1], viewMatrix[2][1]);
  // a petal turns over as it falls
  float turn = uTime * (2.0 + 3.0 * aSeed.x) + aSeed.z * 20.0;
  vec2 q = position.xy;
  if (petal > 0.5) q = vec2(q.x * cos(turn) - q.y * sin(turn), (q.x * sin(turn) + q.y * cos(turn)) * (0.35 + 0.65 * abs(sin(turn * 0.7))));
  vec3 world = at + (cr * q.x + cu * q.y) * size;
  vQ = position.xy;
  vWorld = world;
  // they come with the gusts, and are gone in the lulls
  vLook = vec2((1.0 - smoothstep(13.0, 19.0, dist)) * smoothstep(1.2, 3.5, dist) * (0.25 + 0.75 * smoothstep(0.3, 0.8, gust)), petal);
  gl_Position = projectionMatrix * viewMatrix * vec4(world, 1.0);
}
`,xy=`
${ng}
layout(location = 0) out vec4 outC;
in vec2 vQ;
in vec3 vWorld;
in vec2 vLook;
void main() {
  float r = length(vQ);
  float a = (vLook.y > 0.5 ? 1.0 - smoothstep(0.75, 1.0, r) : (1.0 - smoothstep(0.15, 1.0, r)) * 0.85) * vLook.x;
  if (a < 0.01) discard;
  vec3 col = mix(vec3(0.95, 0.94, 0.86), vec3(0.96, 0.66, 0.74), vLook.y);
  col *= uSunColor * 0.33 * cloudAt(vWorld.xz) + uSkyColor * 0.55;
  outC = vec4(applyFog(col, vWorld), a);
}
`;function Sy(e,t=320){let n=my(99),r=new Float32Array(t*4);for(let e=0;e<r.length;e++)r[e]=n();let i=new Du;i.setAttribute(`position`,new X([-1,-1,0,1,-1,0,1,1,0,-1,1,0],3)),i.setIndex([0,1,2,0,2,3]),i.setAttribute(`aSeed`,new za(r,4)),i.instanceCount=t;let a=new xa(i,new Qc({glslVersion:yt,vertexShader:by,fragmentShader:xy,uniforms:{...e.uniforms,uBox:{value:new q(40,6,40)}},transparent:!0,depthWrite:!1}));return a.frustumCulled=!1,a.renderOrder=6,a}var Cy={u0:250,v0:-1200,size:2400},wy=150,Ty=[{name:`pasture`,colour:[.17,.46,.14]},{name:`pasture`,colour:[.27,.57,.13]},{name:`hay`,colour:[.4,.62,.14]},{name:`wheat`,colour:[.66,.6,.2]},{name:`young`,colour:[.24,.58,.16]},{name:`clover`,colour:[.11,.4,.17]},{name:`orchard`,colour:[.2,.5,.14]},{name:`plough`,colour:[.4,.3,.17]}],Ey=6,Dy=[];function Oy(e,t,n){let r=Math.imul(e,374761393)^Math.imul(t,668265263)^Math.imul(n,1274126177);return r=Math.imul(r^r>>>13,1274126177),r^=r>>>16,(r>>>0)/4294967296}function ky(e,t=1){let n=[],r=t=>e[Math.min(e.length-1,Math.max(0,t))];for(let i=0;i<e.length-1;i++){let[e,a,o,s]=[r(i-1),r(i),r(i+1),r(i+2)],c=Math.max(2,Math.ceil(Math.hypot(o[0]-a[0],o[1]-a[1])/t));for(let t=0;t<c;t++){let r=t/c,i=r*r,l=i*r;n.push([0,1].map(t=>.5*(2*a[t]+(o[t]-e[t])*r+(2*e[t]-5*a[t]+4*o[t]-s[t])*i+(3*a[t]-e[t]-3*o[t]+s[t])*l)))}}return n.push(e[e.length-1]),n}var Ay={kind:0,border:0,hedge:!1,key:0,dirx:1,diry:0};function jy(e,t){let n=e+24*Math.sin(t/97+1.3)+11*Math.sin(t/41),r=t+24*Math.sin(e/113+.4)+11*Math.sin(e/37+2),i=n/wy,a=r/wy,o=Math.floor(i),s=Math.floor(a),c=1e9,l=1e9,u=0,d=0,f=0,p=0;for(let e=-1;e<=1;e++)for(let t=-1;t<=1;t++){let n=o+t,r=s+e,m=n+.12+.76*Oy(n,r,1)-i,h=r+.12+.76*Oy(n,r,2)-a,g=m*m+h*h;g<c?(l=c,f=u,p=d,c=g,u=n,d=r):g<l&&(l=g,f=n,p=r)}Ay.border=(Math.sqrt(l)-Math.sqrt(c))*.5*wy,Ay.key=u*4096+d,Ay.hedge=Oy(Math.min(u,f)*64+Math.max(u,f),Math.min(d,p)*64+Math.max(d,p),3)>.3;let m=Oy(u,d,4),h=m<.26?0:m<.46?1:m<.6?2:m<.74?3:m<.83?4:m<.89?5:m<.95?Ey:7;Math.hypot((u+.5)*wy,(d+.5)*wy)<690&&h>2&&(h%=3),Ay.kind=h;let g=Oy(u,d,5)*Math.PI;return Ay.dirx=Math.cos(g),Ay.diry=Math.sin(g),Ay}var My=e=>Math.round(255*Math.min(1,Math.max(0,e<=.0031308?e*12.92:1.055*e**(1/2.4)-.055)));function Ny(e=1536){let t=new Uint8Array(e*e*4),n=Cy.size/e,r=new Float32Array(e*e);for(let t of Dy)for(let[i,a]of ky(t,.7)){let t=(i-Cy.u0)/n,o=(a-Cy.v0)/n,s=3.4/n;for(let i=Math.max(0,Math.floor(o-s));i<=Math.min(e-1,Math.ceil(o+s));i++)for(let a=Math.max(0,Math.floor(t-s));a<=Math.min(e-1,Math.ceil(t+s));a++){let s=Math.hypot(a+.5-t,i+.5-o)*n,c=1-Math.min(1,Math.max(0,(s-1.9)/1.3));c>r[a+i*e]&&(r[a+i*e]=c)}}let i=0;for(let a=0;a<e;a++){let o=Cy.v0+(a+.5)*n;for(let s=0;s<e;s++,i+=4){let c=Cy.u0+(s+.5)*n,[l,u]=f_(c,o),d=Math.min(s,a,e-1-s,e-1-a)/24,f=M_(l,u)*Math.min(1,d);if(f<=0){t[i]=70,t[i+1]=130,t[i+2]=60;continue}let p=jy(c,o),m=Ty[p.kind].colour,h=p.kind===0||p.kind===1||p.kind===5?0:Math.sin((c*p.dirx+o*p.diry)*Math.PI*2/7)*.11,g=.9+.2*O_(c/55+3,o/55+7,2,211)+h,_=m[0]*g,v=m[1]*g,y=m[2]*g,b=1-Math.min(1,Math.max(0,(p.border-(p.hedge?4.2:1.4))/1.2));if(b>0){let e=b*(p.hedge?1:.5);_+=(.035-_)*e,v+=(.16-v)*e,y+=(.085-y)*e}let x=1-Math.min(1,Math.max(0,(Math.hypot((c-N_[0])/80,(o-N_[1])/96)-.9)/.2));x>0&&(_+=(.3-_)*x,v+=(.52-v)*x,y+=(.15-y)*x);let S=r[s+a*e];S>0&&(_+=(.5-_)*S,v+=(.45-v)*S,y+=(.32-y)*S),t[i]=My(_),t[i+1]=My(v),t[i+2]=My(y),t[i+3]=Math.round(255*f)}}return t}function Py(e,t=1536,n=Ny(t)){let r=new Fa(n,t,t,F,b);return r.colorSpace=$e,r.generateMipmaps=!0,r.minFilter=v,r.magFilter=h,r.anisotropy=e.capabilities.getMaxAnisotropy(),r.wrapS=r.wrapT=c,r.needsUpdate=!0,r}var Fy=6200,Iy=`
out vec3 vWorld;
out vec3 vNormal;
void main() {
  vWorld = position;
  vNormal = normal;
  gl_Position = projectionMatrix * viewMatrix * vec4(position, 1.0);
}
`,Ly=`
${ng}
${rg}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
uniform sampler2D tFar;     // forest, fields, over the square the plain covers
uniform float uFarInv;      // 1 / that square's side
uniform vec2 uAlong;        // the way the valley runs

void main() {
  vec3 N = normalize(vNormal);
  if (N.y < 0.0) N = -N;
  vec2 xz = vWorld.xz;
  float dist = distance(vWorld, uCamPos);
  vec3 V = normalize(uCamPos - vWorld);
  float cloud = cloudAt(xz);
  float rel = slopeRel(N);
  float sunny = cloud * slopeSun(rel);

  // near: the meadow, exactly as the field's own ground paints it far off
  // the farmland, as it is painted (floor.js): fields, hedges, the road
  vec4 fl = floorAt(xz);
  float sx = strokeCoord(vWorld);
  float sw = fwidth(sx);
  float sy = strokeCoord2(xz);
  float syw = fwidth(sy);
  vec2 hq = heroAngle(vWorld);
  vec2 hw = fwidth(hq);
  vec3 g = paintGround(mix(meadowGreen(0.6 + (rel - 0.5) * 0.4), fl.rgb, fl.a), xz, dist, 0.0);
  g = heroPaint(landStrokes(landStrokes(g, xz, dist, sx, sw), xz, dist, sy, syw), vWorld, hq, hw, dist);
  vec3 near = g * meadowLight(sunny);

  vec4 a = texture(tNoiseHi, xz / 2300.0);
  vec4 b = texture(tNoiseHi, xz / 810.0 + 0.31);
  vec4 c = texture(tNoiseHi, xz / 230.0 + 0.57);
  vec4 e = texture(tNoiseHi, xz / 47.0 + 0.13);
  vec4 far4 = texture(tFar, xz * uFarInv + 0.5);

  // meadow: the same greens in broad soft patches, yellower on the rises, with a brush's streaks in it
  vec3 col = meadowGreen(0.5 + 0.2 * smoothstep(0.3, 0.7, a.g) + (rel - 0.5) * 0.4);

  col = paintGround(mix(col, fl.rgb, fl.a), xz, dist, 0.0);
  col = heroPaint(landStrokes(landStrokes(col, xz, dist, sx, sw), xz, dist, sy, syw), vWorld, hq, hw, dist);

  // forest floor: dark under the trees that stand on it, its edge ragged
  float forest = smoothstep(0.38, 0.62, far4.r + (c.r - 0.5) * 0.34 + (e.g - 0.5) * 0.2);
  col = mix(col, mix(vec3(0.03, 0.14, 0.07), vec3(0.06, 0.2, 0.08), e.b), forest);

  float sun = max(dot(N, uSunDir), 0.0);
  // (green land seen from far off is lit a little from every side)
  vec3 far = col * meadowLight(sunny);

  vec3 outCol = mix(near, far, smoothstep(650.0, 1400.0, dist));
  outC = vec4(applyFog(outCol, vWorld), mix(1.0, 0.4, smoothstep(520.0, 700.0, dist)));
}
`;function Ry(e=1024){let t=new Uint8Array(e*e*4),n=Fy*2;for(let r=0;r<e;r++)for(let i=0;i<e;i++){let a=((i+.5)/e-.5)*n,o=((r+.5)/e-.5)*n,s=(i+r*e)*4;t[s]=255*P_(a,o),t[s+1]=255*M_(a,o),t[s+3]=255}let r=new Fa(t,e,e,F,b);return r.minFilter=r.magFilter=h,r.needsUpdate=!0,r}function zy(e,t=512){let n=1440,r=t-6,i=Fy/r,a=new Float32Array(n*121*3),o=0;for(let e=0;e<=120;e++){let t=i**(e/120);for(let i=0;i<n;i++){let s=i/n*4,c=Math.floor(s),l=(s-c)*2-1,u=c===0?l:c===1?1:c===2?-l:-1,d=c===0?-1:c===1?l:c===2?1:-l,f=Math.min(1,e/48),p=Math.hypot(u,d);u*=r*t*(1+(1/p-1)*f),d*=r*t*(1+(1/p-1)*f),a[o++]=u;let m=Math.min(1,Math.max(0,(Math.hypot(u,d)-4e3)/2e3));a[o++]=I_(u,d)*(1-.9*m*m*(3-2*m)),a[o++]=d}}let s=new Uint32Array(120*n*6),c=0;for(let e=0;e<120;e++)for(let t=0;t<n;t++){let r=e*n+t,i=e*n+(t+1)%n;s[c++]=r,s[c++]=r+n,s[c++]=i,s[c++]=i,s[c++]=r+n,s[c++]=i+n}let l=new Z;l.setAttribute(`position`,new ui(a,3)),l.setIndex(new ui(s,1)),l.computeVertexNormals();let u=Ry(),d=new xa(l,new Qc({glslVersion:yt,vertexShader:Iy,fragmentShader:Ly,uniforms:{...e.uniforms,tFar:{value:u},uFarInv:{value:1/(Fy*2)},uAlong:{value:new K(a_[0],a_[1])}},side:2,polygonOffset:!0,polygonOffsetFactor:2,polygonOffsetUnits:12}));return d.frustumCulled=!1,d}var By=86,Vy=`
out vec2 vUv;
void main() {
  vUv = position.xy * 0.5 + 0.5;
  gl_Position = vec4(position.xy, 0.0, 1.0);
}
`,Hy=`
float hash(vec2 p) {
  p = 50.0 * fract(p * 0.3183099 + vec2(0.71, 0.113));
  return fract(p.x * p.y * (p.x + p.y));
}

float noise(vec2 x) {
  vec2 p = floor(x);
  vec2 f = fract(x);
  vec2 u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
  float a = hash(p);
  float b = hash(p + vec2(1.0, 0.0));
  float c = hash(p + vec2(0.0, 1.0));
  float d = hash(p + vec2(1.0, 1.0));
  return a + (b - a) * u.x + (c - a) * u.y + (a - b - c + d) * u.x * u.y;
}

// Cells: how far it is to the edge of the cell a point is in, and the cell's name.
vec3 voro(vec2 x) {
  vec2 n = floor(x);
  vec2 f = x - n;
  float d1 = 8.0;
  float d2 = 8.0;
  vec2 id = n;
  for (int j = -1; j <= 1; j++) {
    for (int i = -1; i <= 1; i++) {
      vec2 g = vec2(float(i), float(j));
      vec2 r = g + vec2(hash(n + g), hash(n + g + 17.3)) - f;
      float d = dot(r, r);
      if (d < d1) { d2 = d1; d1 = d; id = n + g; }
      else if (d < d2) d2 = d;
    }
  }
  return vec3(sqrt(d2) - sqrt(d1), id);
}


// Where the hills are farmed: 0 to 1.
float farmAt(vec2 q) {
  return smoothstep(0.42, 0.52, noise(q * 0.42 + 11.0));
}

// The field a point is in: how far it is to the field's edge, and the field's name.
vec3 fieldOf(vec2 q) {
  return voro(q * 1.9 + 0.6 * vec2(noise(q * 0.7), noise(q * 0.7 + 5.0)));
}

// Trees on a far hill, as paint. x: 1 where there is tree. y: how high the crown stands there, 0 to 1. Woods whose
// edges are whole round crowns; broken rows of trees along the fields' edges; trees alone and in twos and threes
// on the open grass; bushes.
vec2 treesAt(vec2 q) {
  float m = noise(q * 1.3 + 3.0) * 0.6 + noise(q * 4.3 + 9.0) * 0.4 + 0.07 * smoothstep(9.5, 12.5, length(q));
  vec3 c = voro(q * 9.0);
  float mass = max(step(0.66, m), step(0.61, m + (hash(c.yz + 2.0) - 0.5) * 0.12) * step(0.1, c.x));
  vec3 f = fieldOf(q);
  vec3 b = voro(q * 12.0 + 3.0);
  float row = step(f.x, 0.05) * step(0.3, hash(f.yz + 8.2)) * farmAt(q) * step(0.12, b.x) * step(0.3, hash(b.yz + 1.0));
  vec3 one = voro(q * 4.5 + 7.0);
  float alone = step(0.8, hash(one.yz + 5.0)) * step(0.3, one.x) * step(0.36, m);
  vec3 bu = voro(q * 15.0 + 1.0);
  float bush = step(0.95, hash(bu.yz + 3.0)) * step(0.22, bu.x);
  float dome = max(mass * (0.4 + 0.6 * sqrt(clamp(c.x * 2.2, 0.0, 1.0))), alone * sqrt(clamp((one.x - 0.3) * 4.0, 0.0, 1.0)));
  dome = max(dome, row * 0.75 * sqrt(clamp((b.x - 0.12) * 3.5, 0.0, 1.0)));
  dome = max(dome, bush * 0.4 * sqrt(clamp((bu.x - 0.22) * 4.0, 0.0, 1.0)));
  return vec2(max(max(mass, alone), max(row, bush)), dome);
}
`,Uy=`
precision highp float;
layout(location = 0) out vec4 outC;
in vec2 vUv;
uniform vec2 uHero;    // the way the valley runs
uniform int uDetail;   // octaves
const mat2 M2 = mat2(0.8, -0.6, 0.6, 0.8);
${Hy}

// value noise and its slope
vec3 noised(vec2 x) {
  vec2 p = floor(x);
  vec2 f = fract(x);
  vec2 u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
  vec2 du = 30.0 * f * f * (f * (f - 2.0) + 1.0);
  float a = hash(p);
  float b = hash(p + vec2(1.0, 0.0));
  float c = hash(p + vec2(0.0, 1.0));
  float d = hash(p + vec2(1.0, 1.0));
  return vec3(a + (b - a) * u.x + (c - a) * u.y + (a - b - c + d) * u.x * u.y, du * (vec2(b - a, c - a) + (a - b - c + d) * u.yx));
}

// rounded, water-worn ground: each finer fold is laid on less where the ground is already steep (the foothills)
float erode(vec2 p, int oct) {
  float a = 0.0;
  float b = 1.0;
  vec2 d = vec2(0.0);
  for (int i = 0; i < 9; i++) {
    if (i >= oct) break;
    vec3 n = noised(p);
    d += n.yz;
    a += b * n.x / (1.0 + dot(d, d));
    b *= 0.5;
    p = M2 * p * 2.0;
  }
  return a;
}

// alpine ground: sharp crests, and the finer folds only along them, so the flanks between are broad and clean
float ridged(vec2 p, int oct) {
  float a = 0.0;
  float amp = 0.5;
  float w = 1.0;
  for (int i = 0; i < 10; i++) {
    if (i >= oct) break;
    float r = 1.0 - abs(2.0 * noise(p) - 1.0);
    r *= r;
    a += r * amp * w;
    w = clamp(r * 1.7, 0.0, 1.0);
    amp *= 0.5;
    p = M2 * p * 2.07 + 3.7;
  }
  return a;
}

void main() {
  vec2 p = (vUv - 0.5) * ${By.toFixed(1)};
  float r = length(p);
  vec2 dir = p / max(r, 1e-3);
  float ahead = 0.5 + 0.5 * smoothstep(-0.6, 0.7, dot(dir, uHero));
  // wooded foothills, in ranks
  // (their detail is painted on them, not modelled into them: lumps are what makes far ground look computed)
  // three ranks of rounded hills, each behind and above the last, so that one's outline lies across the next
  float h = 0.0;
  for (int k = 0; k < 3; k++) {
    float fk = float(k);
    float at = 7.4 + 2.2 * fk + 1.3 * (noise(p * 0.19 + vec2(3.1 + 9.0 * fk, 7.7)) - 0.5) + 0.8 * (noise(p * 0.62 + vec2(5.0 * fk, 2.0)) - 0.5);
    float off = (r - at) / (1.0 + 0.35 * fk);
    // (hills a kilometre or so across, on broader ones: seen from the walk a rank is several hills, not one long back)
    float lump = smoothstep(0.2, 0.75, noise(p * (0.42 - 0.07 * fk) + vec2(1.9, 4.3 + 5.0 * fk)));
    float crest = (noise(p * (1.05 - 0.15 * fk) + vec2(7.3 + 4.0 * fk, 1.1)) - 0.5) * 0.34 + (noise(p * 2.3 + vec2(2.0, 9.0 * fk)) - 0.5) * 0.08;
    h = max(h, max((0.42 + 0.62 * fk) * (0.42 + 0.58 * lump) + crest, 0.0) * exp(-off * off));
  }
  // trees stand up out of them: crowns are part of the ground's shape, so they have outlines and hide what is behind
  vec2 wood = treesAt(p);
  h += 0.055 * wood.x * wood.y * (1.0 - smoothstep(1.45, 1.75, h)) * smoothstep(5.6, 7.0, r) * (1.0 - smoothstep(13.5, 15.0, r));
  // the range: massifs with passes between them
  vec2 q = p + 1.6 * vec2(noise(p * 0.11 + 3.0), noise(p * 0.11 + 9.0));
  float mass = smoothstep(0.34, 0.6, noise(p * 0.052 + 9.3));
  float band = smoothstep(12.0, 17.5, r) * (1.0 - 0.65 * smoothstep(21.0, 26.0, r));
  float m = ridged(q * 0.095 + vec2(11.3, 1.9), uDetail);
  h += pow(m, 1.25) * 4.4 * (0.22 + 0.78 * mass) * band * ahead;
  // two great peaks, where the eye goes: one a little left of the valley's line, one well to its right
  vec2 perp = vec2(-uHero.y, uHero.x);
  float d1 = length(p - (uHero * 18.5 - perp * 4.2));
  float d2 = length(p - (uHero * 17.0 + perp * 11.5));
  h += exp(-d1 * d1 / 7.5) * (0.7 + 1.6 * m) + exp(-d2 * d2 / 8.0) * (0.5 + 1.4 * m);
  // and a further range, seen through the passes
  h += pow(ridged(p * 0.07 + vec2(31.0, 17.0), 7), 1.2) * 3.6 * smoothstep(25.5, 31.0, r);
  outC = vec4(h, 0.0, 0.0, 1.0);
}
`,Wy=`
precision highp float;
precision highp sampler2D;
layout(location = 0) out vec4 oLight;   // its colour in the light; 1 where there is land
layout(location = 1) out vec4 oShade;   // its colour in shade; how lit it is
layout(location = 2) out vec4 oAir;     // metres of air before it, thinned with height; mist
in vec2 vUv;
uniform sampler2D tHeight;
uniform float uTexel;  // one texel of the height map, km
uniform vec2 uSide;    // the side the afternoon's light comes from, level
uniform vec2 uHero;    // the way the valley runs
uniform vec2 uEl;      // the strip's lowest and highest elevation, radians
uniform float uEye;    // the eye's height, km

const float TAU = 6.28318531;
${Hy}

// The land's height at p. Kilometres both.
float height(vec2 p) {
  return textureLod(tHeight, p / ${By.toFixed(1)} + 0.5, 0.0).r;
}

float hash3(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}

// Cells in space: how far it is to the edge of the cell a point is in, and three numbers that are the cell's own.
vec4 voro3(vec3 x) {
  vec3 n = floor(x);
  vec3 f = x - n;
  float d1 = 8.0;
  float d2 = 8.0;
  vec3 id = n;
  for (int k = -1; k <= 1; k++) {
    for (int j = -1; j <= 1; j++) {
      for (int i = -1; i <= 1; i++) {
        vec3 g = vec3(float(i), float(j), float(k));
        vec3 c = n + g;
        vec3 r = g + vec3(hash3(c), hash3(c + 17.3), hash3(c + 41.7)) - f;
        float d = dot(r, r);
        if (d < d1) { d2 = d1; d1 = d; id = c; }
        else if (d < d2) d2 = d;
      }
    }
  }
  return vec4(sqrt(d2) - sqrt(d1), hash3(id + 3.1), hash3(id + 7.7), hash3(id + 12.9));
}

// A bank of cloud lying along the feet of the range, as painted backgrounds have one: the hills in front stand
// dark against it and the peaks rise out of it, and that is three layers where there were two. Returns how much
// of it there is this way; lit comes back as how much of that is its sunlit top.
float bankAt(float az, float el, float u, out float lit) {
  vec2 round_ = vec2(cos(az), sin(az));
  float top = 0.158 + 0.02 * (noise(round_ * 2.3 + 3.0) - 0.5) * 2.0;
  // heaped tops, big heaps with small ones on them, no two alike: rounded above, creased where they meet
  float h1 = sqrt(abs(2.0 * noise(round_ * 9.0 + 1.7) - 1.0));
  float h2 = sqrt(abs(2.0 * noise(round_ * 27.0 + 5.1) - 1.0));
  float h3 = sqrt(abs(2.0 * noise(round_ * 71.0 + 9.4) - 1.0));
  top += 0.022 * h1 + 0.014 * h2 + 0.006 * h3;
  // it lies in lengths, with gaps where the mountains come down to their feet
  // (s: radians right of the valley's line)
  float s = atan(dot(round_, vec2(-uHero.y, uHero.x)), dot(round_, uHero));
  float here = max(1.0 - smoothstep(0.2, 0.46, abs(s - 0.44)), 1.0 - smoothstep(0.14, 0.34, abs(s + 0.98)));
  float peak = 1.0 - smoothstep(0.14, 0.38, abs(s + 0.2));
  here = max(here, peak);
  top += 0.014 * peak;
  here = max(here, smoothstep(0.5, 0.64, noise(round_ * 1.4 + 5.0)) * smoothstep(1.5, 1.9, abs(s)));
  top -= (1.0 - here) * 0.26;
  // inside, it is heaps too: each lit above and shaded below
  // (read along whichever way round the horizon is running here, so it is never stretched)
  float billow = mix(noise(vec2(round_.x * 38.0, el * 70.0)), noise(vec2(round_.y * 38.0 + 7.0, el * 70.0)), round_.x * round_.x);
  // its heaps are lit above and in shade below, and the whole of it darkens downwards
  float heap = mix(noise(vec2(round_.x * 15.0 + 2.0, el * 34.0)), noise(vec2(round_.y * 15.0 + 9.0, el * 34.0)), round_.x * round_.x);
  // (heaps within it too: round, each white above)
  float h4 = sqrt(abs(2.0 * mix(noise(vec2(round_.x * 21.0 + 4.0, el * 46.0)), noise(vec2(round_.y * 21.0 + 8.0, el * 46.0)), round_.x * round_.x) - 1.0));
  float down = top - el;
  lit = step(down - 0.034 * h4 - (billow - 0.5) * 0.02, 0.026) + step(0.6, h4 * (0.55 + 0.45 * heap)) * step(down, 0.09);
  lit = clamp(lit, 0.0, 1.0);
  float cover = smoothstep(0.0, 0.0018, top - el + (billow - 0.5) * 0.008);
  return cover;
}

vec3 normalAt(vec2 p, float e) {
  return normalize(vec3(height(p - vec2(e, 0.0)) - height(p + vec2(e, 0.0)), 2.0 * e, height(p - vec2(0.0, e)) - height(p + vec2(0.0, e))));
}

void main() {
  float az = vUv.x * TAU - 3.14159265;
  float el = mix(uEl.x, uEl.y, vUv.y);
  vec3 rd = vec3(cos(az) * cos(el), sin(el), sin(az) * cos(el));
  vec3 ro = vec3(0.0, uEye, 0.0);
  float flat_ = length(rd.xz);
  float t = 5.0 / flat_;
  float tMax = 41.5 / flat_;
  float before = t;
  bool hit = false;
  for (int i = 0; i < 520; i++) {
    vec3 p = ro + rd * t;
    float dy = p.y - height(p.xz);
    if (dy < 0.0) { hit = true; break; }
    if (t > tMax || (rd.y > 0.0 && p.y > 7.8)) break;
    before = t;
    t += max(0.0045 * t, 0.4 * dy);
  }
  float bankLit;
  float bank = bankAt(az, el, vUv.x, bankLit);
  if (!hit) {
    if (rd.y < 0.0) {
      // below the horizon and nothing struck: the plain goes on under the haze
      oLight = vec4(sqrt(vec3(0.2, 0.3, 0.15)), 1.0);
      oShade = vec4(sqrt(vec3(0.2, 0.3, 0.15)), 1.0);
      oAir = vec4(1.0, 0.0, 0.0, 0.0);
    } else {
      // open sky, but for the bank
      oLight = vec4(0.0, 0.0, 0.0, bank);
      oShade = vec4(0.0);
      oAir = vec4(0.0, 0.0, bank, bank * bankLit);
    }
    return;
  }
  float lo = before;
  float hi = t;
  for (int k = 0; k < 9; k++) {
    float mid = 0.5 * (lo + hi);
    vec3 q = ro + rd * mid;
    if (q.y - height(q.xz) > 0.0) lo = mid; else hi = mid;
  }
  t = hi;
  vec3 p = ro + rd * t;
  float alt = p.y;
  float r = length(p.xz);
  // the faces a brush would lay in: broad
  vec3 N = normalAt(p.xz, max(0.014 + 0.0022 * t, uTexel * 2.0));
  // and the ground as it is, for where rock breaks out and snow cannot lie
  float crag = 1.0 - normalAt(p.xz, uTexel).y;

  // A painter lights far mountains from the side wherever they stand in the picture, so their forms turn: here
  // the light comes from whichever side of the view the sun is on, and low.
  vec2 vh = rd.xz / flat_;
  vec2 rt = vec2(-vh.y, vh.x);
  float side = clamp(dot(rt, uSide) * 3.0, -1.0, 1.0);
  vec2 lh = normalize(rt * side * 0.9 + uSide * 0.45);
  vec3 L = normalize(vec3(lh.x, 0.45, lh.y));

  // the shadow one ridge throws on the next
  float sh = 1.0;
  float ts = 0.05;
  for (int i = 0; i < 40; i++) {
    vec3 q = p + L * ts;
    float dq = q.y - height(q.xz);
    sh = min(sh, 14.0 * dq / ts);
    if (sh < 0.0 || q.y > 7.8) break;
    ts += max(0.04, 0.09 * ts);
  }
  sh = clamp(sh, 0.0, 1.0);

  // gullies: streaks that run down the fall line, for snow to lie in and rock to show between
  vec2 fall = N.xz / max(length(N.xz), 1e-3);
  vec2 contour = vec2(-fall.y, fall.x);
  // (read across the slope whichever way it faces, from marks that stay where they are on the ground)
  float wx = fall.x * fall.x;
  float streak = mix(noise(vec2(p.x * 11.0, alt * 1.3 + 4.0)), noise(vec2(p.z * 11.0, alt * 1.3 + 4.0)), wx) * 0.6 + mix(noise(vec2(p.x * 3.7 + 9.0, alt * 0.6)), noise(vec2(p.z * 3.7 + 9.0, alt * 0.6)), wx) * 0.4;
  float brush = noise(p.xz * 2.3 + 7.0);

  // what grows where
  float treeline = 1.5 + 0.3 * brush;
  // The green hills are open grass, lit in two flat tones off their broad shape, with trees standing on them.
  vec3 Nb = normalAt(p.xz, 0.09);
  // (their light turns softly into shade, as a wash does)
  float hillLit = smoothstep(0.0, 0.22, dot(Nb, L)) * smoothstep(0.25, 0.5, sh);
  float below = (1.0 - smoothstep(treeline - 0.1, treeline + 0.06, alt + crag * 0.3)) * (1.0 - smoothstep(0.5, 0.72, crag));
  // (more of them in the folds and on the shaded sides, fewer high up)
  float wooded = treesAt(p.xz).x * below;
  // the shade a wood throws on the grass beside it
  float thrown = treesAt(p.xz + L.xz * 0.07).x * below * (1.0 - wooded);
  float meadow = (1.0 - smoothstep(1.9, 2.3, alt + (streak - 0.5) * 0.5)) * (1.0 - smoothstep(0.3, 0.55, crag));
  float rocky = (1.0 - wooded) * (1.0 - 0.9 * meadow);

  // Rock as it is painted: broken into facets, large ones and small ones on them, each turned its own way to the
  // light, the joints between them drawn dark. This is where a far mountain gets its detail.
  vec3 fq = vec3(p.x, alt * 1.6, p.z);
  vec4 f1 = voro3(fq * 2.6);
  vec4 f2 = voro3(fq * 7.5 + 5.0);
  vec4 f3 = voro3(fq * 19.0 + 11.0);
  vec2 t1 = f1.yz - 0.5;
  vec2 t2 = f2.yz - 0.5;
  vec2 t3 = f3.yz - 0.5;
  vec3 Nf = normalize(N + (vec3(contour.x, 0.0, contour.y) * (t1.x * 1.0 + t2.x * 0.75 + t3.x * 0.4) + vec3(0.0, t1.y * 0.8 + t2.y * 0.6 + t3.y * 0.35, 0.0)) * rocky);
  float joint = ((1.0 - smoothstep(0.0, 0.05, f1.x)) * 0.8 + (1.0 - smoothstep(0.0, 0.08, f2.x)) * 0.36 + (1.0 - smoothstep(0.0, 0.1, f3.x)) * 0.1) * rocky;
  float tone = f1.w * 0.4 + f2.w * 0.35 + f3.w * 0.25;

  // rock and snow turn from light to shade along an edge; a wooded hill turns slowly
  float ndl = dot(Nf, L);
  // (in three clear tones, and each crown and tuft tips it one way or the other)
  float turn = dot(N, L) + (fract(f2.w * 7.3) - 0.5) * 0.3;
  float soft = smoothstep(-0.05, 0.02, turn) * 0.5 + smoothstep(0.26, 0.33, turn) * 0.5;
  float crownTurn = dot(normalAt(p.xz, uTexel * 1.5), L);
  // a crown is lobes of leaf, each catching the light or not: lighter at its middle, and on the side the sun is
  vec3 lobe = voro(p.xz * 26.0 + 2.0);
  float tscore = crownTurn * 0.9 + (hash(lobe.yz + 6.0) - 0.5) * 0.5 + (lobe.x - 0.14) * 0.9;
  float hillTone = mix(hillLit * (1.0 - 0.9 * thrown), smoothstep(-0.02, 0.05, tscore) * smoothstep(0.3, 0.4, sh), wooded);
  float lit = mix(smoothstep(0.0, 0.12, ndl) * smoothstep(0.1, 0.5, sh), hillTone, max(wooded, meadow));

  // snow: whole only on the highest tops; below them it lies on the ledges, a facet here and a facet there
  float deep = smoothstep(2.6, 5.4, alt + (streak - 0.5) * 0.9 + 0.3 * noise(p.xz * 0.45 + 2.0));
  float snow = max(step(fract(f1.y * 7.31 + f1.w * 3.7), deep * 0.62) * (1.0 - 0.85 * smoothstep(0.5, 0.78, crag)), smoothstep(0.93, 1.0, deep));
  snow = max(snow, step(fract(f2.z * 5.17 + f2.w * 2.3), deep * 0.12)) * (1.0 - wooded) * step(0.02, deep);

  // rock: cream and warm grey in the light; out of it grey-green, not blue
  vec3 light = mix(vec3(0.38, 0.34, 0.26), vec3(0.68, 0.62, 0.47), tone);
  vec3 shade = mix(vec3(0.25, 0.27, 0.23), vec3(0.43, 0.43, 0.36), tone);
  // (lichen and thin grass stain it, lower down most)
  float stain = smoothstep(0.4, 0.64, noise(p.xz * 1.7 + 11.0) * 0.6 + fract(f2.z * 9.1 + f2.w * 3.3) * 0.4) * (1.0 - smoothstep(2.3, 3.9, alt));
  light = mix(light, vec3(0.4, 0.4, 0.17), stain * 0.8);
  shade = mix(shade, vec3(0.2, 0.26, 0.15), stain * 0.7);
  light *= 1.0 - 0.55 * joint;
  shade *= 1.0 - 0.5 * joint;
  // high pasture
  // The grass, as it is painted: nowhere one colour. Soft patches of four greens at two sizes, long strokes laid in
  // two directions, a turn of tone that follows the hill round; then paths, fields and hedges, and small dabs.
  float v1 = noise(p.xz * 0.9 + 1.0);
  float v2 = noise(p.xz * 2.7 + 4.0);
  float s1 = noise(vec2(dot(p.xz, vec2(0.8, 0.6)) * 1.6, dot(p.xz, vec2(-0.6, 0.8)) * 11.0));
  float s2 = noise(vec2(dot(p.xz, vec2(0.96, -0.28)) * 2.2, dot(p.xz, vec2(0.28, 0.96)) * 17.0) + 5.0);
  float round_ = noise(vec2((alt + 0.08 * v1) * 24.0, v2 * 2.0));
  float v = v1 * 0.34 + v2 * 0.2 + s1 * 0.2 + s2 * 0.12 + round_ * 0.14;
  vec3 grassL = mix(vec3(0.17, 0.46, 0.14), vec3(0.27, 0.57, 0.13), smoothstep(0.36, 0.5, v));
  grassL = mix(grassL, vec3(0.42, 0.66, 0.15), smoothstep(0.54, 0.66, v));
  vec3 grassS = mix(vec3(0.08, 0.31, 0.16), vec3(0.13, 0.42, 0.17), smoothstep(0.36, 0.6, v));
  // fields: each a tone of its own, a darker strip under the hedge along most of their edges
  float farmed = farmAt(p.xz) * (1.0 - smoothstep(0.2, 0.4, 1.0 - Nb.y));
  vec3 fld = fieldOf(p.xz);
  float kind = hash(fld.yz + 1.7);
  vec3 tint = kind < 0.25 ? vec3(1.3, 1.12, 0.9) : kind < 0.5 ? vec3(0.8, 0.92, 1.0) : kind < 0.62 ? vec3(1.75, 1.2, 1.1) : vec3(1.0);
  grassL = mix(grassL, grassL * tint, farmed);
  grassS = mix(grassS, grassS * mix(vec3(1.0), tint, 0.6), farmed);
  float hedge = (1.0 - smoothstep(0.02, 0.05, fld.x)) * step(0.3, hash(fld.yz + 8.2)) * farmed;
  grassL = mix(grassL, vec3(0.07, 0.27, 0.11), hedge * 0.8);
  grassS = mix(grassS, vec3(0.04, 0.17, 0.11), hedge * 0.8);
  // paths: pale, winding, here and gone
  float way = 1.0 - smoothstep(0.004, 0.008, abs(noise(p.xz * 0.5 + 7.0) - 0.5));
  way = max(way, (1.0 - smoothstep(0.004, 0.0075, abs(noise(p.xz * 0.9 + 13.0) - 0.46))) * step(0.45, v1));
  grassL = mix(grassL, vec3(0.62, 0.71, 0.3), way * 0.75);
  grassS = mix(grassS, vec3(0.3, 0.5, 0.3), way * 0.6);
  // dabs: a darker tuft, a fleck of sun, a few flowers
  vec3 d1 = voro(p.xz * 38.0);
  float dh = hash(d1.yz + 9.0);
  float dab = step(0.2, d1.x);
  grassL = mix(grassL, grassL * vec3(0.72, 0.84, 0.9), dab * step(0.8, dh));
  grassL = mix(grassL, grassL * vec3(1.25, 1.12, 0.85), dab * step(dh, 0.14));
  grassL = mix(grassL, vec3(0.86, 0.82, 0.46), step(0.3, d1.x) * step(0.955, fract(dh * 7.13)));
  grassS *= 1.0 - 0.2 * dab * step(0.8, dh);
  light = mix(light, grassL, meadow * 0.96);
  shade = mix(shade, grassS, meadow * 0.96);
  // trees: light, half-light and shade by the lobe, a touch of yellow where the sun is full on them, dark between crowns
  vec3 cr = voro(p.xz * 9.0);
  float gap = (1.0 - smoothstep(0.0, 0.05, cr.x)) * 0.6;
  vec3 treeL = mix(vec3(0.055, 0.27, 0.12), vec3(0.11, 0.37, 0.13), smoothstep(0.3, 0.36, tscore));
  treeL = mix(treeL, vec3(0.24, 0.5, 0.15), smoothstep(0.62, 0.68, tscore));
  vec3 treeS = mix(vec3(0.022, 0.11, 0.095), vec3(0.045, 0.19, 0.115), smoothstep(-0.4, -0.3, tscore));
  light = mix(light, mix(treeL, vec3(0.03, 0.14, 0.09), gap), wooded);
  shade = mix(shade, mix(treeS, vec3(0.018, 0.085, 0.085), gap), wooded);
  // snow: white, and blue-grey where the light does not reach
  light = mix(light, vec3(1.07, 1.06, 1.03) * (0.94 + 0.08 * tone), snow);
  shade = mix(shade, mix(vec3(0.6, 0.65, 0.72), vec3(0.7, 0.74, 0.8), tone), snow);

  // in the sun, a face that meets it squarely is a tone brighter than one it glances along
  light *= mix(1.0, mix(0.72, 1.05, smoothstep(0.25, 0.6, ndl)), rocky);
  // out of it a face still turns: lighter where it looks up at the sky, a tone deeper where it looks away
  shade *= mix(1.0, mix(0.78, 1.16, smoothstep(0.1, 0.5, dot(Nf, normalize(vec3(-L.x, 1.1, -L.z))))), rocky);

  // (a painter keeps more of the far country's colour than the air would)
  float thin = mix(1.0, 0.3, smoothstep(0.25, 2.6, alt));
  // mist lying at the feet of each range, so one stands clear of the next
  float lie = noise(vec2(az * 9.0, 1.0)) - 0.5;
  float mist = smoothstep(10.5, 14.5, r) * (1.0 - smoothstep(0.2, 0.85, alt + lie * 0.4)) * 0.2;
  mist = max(mist, smoothstep(25.0, 29.0, r) * (1.0 - smoothstep(0.6, 1.9, alt + lie * 0.6)) * 0.6);
  // the bank of cloud lies behind the foothills and in front of everything else
  float front = bank * smoothstep(14.0, 15.0, r);
  float land = 1.0 - front;
  // (colours as their square roots, and the air as how much of the view it has taken at a set thickness: both
  // fit eight bits, which lets the picture be half as fine again)
  oLight = vec4(sqrt(min(light, 1.0)) * land, 1.0);
  oShade = vec4(sqrt(min(shade, 1.0)) * land, lit * land);
  oAir = vec4((1.0 - exp(-t * 1000.0 * thin * 0.3 * mix(mix(0.85, 3.0, smoothstep(7.4, 12.0, r)), 1.0, rocky) * 0.00015)) * land, mist * land, front, front * bankLit);
}
`;function Gy(e,t,n,{width:r=8192,ground:i=4096}={}){r=Math.min(r,e.capabilities.maxTextureSize),i=Math.min(i,e.capabilities.maxTextureSize);let a=[hn.degToRad(-2.6),hn.degToRad(21.4)],o=Math.round(r*(a[1]-a[0])/(Math.PI*2)),l=new Rn(r,o,{count:3,type:b,format:F,depthBuffer:!1,generateMipmaps:!1});for(let e of l.textures)e.minFilter=e.magFilter=h,e.wrapS=s,e.wrapT=c;let u=new Rn(i,i,{type:D,format:te,depthBuffer:!1,generateMipmaps:!1});u.texture.minFilter=u.texture.magFilter=h,u.texture.wrapS=u.texture.wrapT=c;let d=new Z;d.setAttribute(`position`,new X([-1,-1,0,3,-1,0,-1,3,0],3));let f=(e,t)=>{let n=new xa(d,new Qc({glslVersion:yt,vertexShader:Vy,fragmentShader:e,uniforms:t,depthTest:!1,depthWrite:!1}));n.frustumCulled=!1;let r=new Er;return r.add(n),r},p=f(Uy,{uHero:{value:new K(t[0],t[1])},uDetail:{value:i>2048?10:9}}),m=f(Wy,{tHeight:{value:u.texture},uTexel:{value:By/i},uSide:{value:new K(n[0],n[1]).normalize()},uHero:{value:new K(t[0],t[1])},uEl:{value:new K(a[0],a[1])},uEye:{value:.06}}),g=new cu,_=(t,n,r,i,a)=>{let o=Math.ceil(r/a);n.scissorTest=!0;for(let r=0;r<a;r++)n.scissor.set(r*o,0,o,i),e.setRenderTarget(n),e.render(t,g);n.scissorTest=!1};return{textures:l.textures,el:a,paint(){let t=e.getRenderTarget();_(p,u,i,i,8),_(m,l,r,o,48),e.setRenderTarget(t)},compile:()=>Promise.all([e.compileAsync(p,g),e.compileAsync(m,g)])}}var Ky=240,qy=14,Jy=44;function Yy(e){let t=e>>>0;return()=>{t=t+1831565813>>>0;let e=t;return e=Math.imul(e^e>>>15,e|1),e^=e+Math.imul(e^e>>>7,e|61),((e^e>>>14)>>>0)/4294967296}}function Xy(e=64){let t=Yy(90210),n=e=>{let n=new Float32Array(e*e*e*3);for(let e=0;e<n.length;e++)n[e]=t();return n},r=(e,t,n,r,i)=>{let a=n*t,o=r*t,s=i*t,c=Math.floor(a),l=Math.floor(o),u=Math.floor(s),d=9;for(let n=-1;n<=1;n++){let r=(u+n+t)%t;for(let i=-1;i<=1;i++){let f=(l+i+t)%t;for(let p=-1;p<=1;p++){let m=((c+p+t)%t+(f+r*t)*t)*3,h=c+p+e[m]-a,g=l+i+e[m+1]-o,_=u+n+e[m+2]-s,v=h*h+g*g+_*_;v<d&&(d=v)}}}return Math.sqrt(Math.max(0,1-d*1.5))},i=e=>{let n=new Float32Array(e*e*e);for(let e=0;e<n.length;e++)n[e]=t();return n},a=(e,t,n,r,i)=>{let a=n*t,o=r*t,s=i*t,c=Math.floor(a),l=Math.floor(o),u=Math.floor(s),d=e=>e*e*(3-2*e),f=d(a-c),p=d(o-l),m=d(s-u),h=(n,r,i)=>e[(c+n)%t+((l+r)%t+(u+i)%t*t)*t],g=h(0,0,0)+(h(1,0,0)-h(0,0,0))*f,_=h(0,1,0)+(h(1,1,0)-h(0,1,0))*f,v=h(0,0,1)+(h(1,0,1)-h(0,0,1))*f,y=h(0,1,1)+(h(1,1,1)-h(0,1,1))*f,b=g+(_-g)*p;return b+(v+(y-v)*p-b)*m},o=[n(3),n(6),n(12),n(24)],c=[i(4),i(8),i(16)],l=new Uint16Array(e*e*e*4),u=oi.toHalfFloat,d=0;for(let t=0;t<e;t++){let n=(t+.5)/e;for(let t=0;t<e;t++){let i=(t+.5)/e;for(let t=0;t<e;t++){let s=(t+.5)/e,f=r(o[0],3,s,i,n),p=r(o[1],6,s,i,n),m=r(o[2],12,s,i,n),h=r(o[3],24,s,i,n);l[d++]=u(Math.min(1,f*.6+p*.28+m*.12)),l[d++]=u(Math.min(1,p*.55+m*.3+h*.15)),l[d++]=u(a(c[0],4,s,i,n)*.57+a(c[1],8,s,i,n)*.29+a(c[2],16,s,i,n)*.14),l[d++]=u(Math.min(1,m*.6+h*.4))}}}let f=new Vn(l,e,e,e);return f.format=F,f.type=D,f.minFilter=f.magFilter=h,f.wrapS=f.wrapT=f.wrapR=s,f.unpackAlignment=1,f.needsUpdate=!0,f}function Zy(e){let t=Yy(4),[n,r]=e,i=(e,t)=>[e*n-t*r,e*r+t*n],a=[],o=(e,n,r,i,a,o,s)=>{for(let c=0;c<s;c++){let s=t()*Math.PI*2,c=-.15+1.15*t(),l=Math.sqrt(Math.max(0,1-c*c))*(.35+.65*Math.sqrt(t())),u=Math.min(a*.3,o*.62)*(.72+.5*t());e.push([n+Math.cos(s)*l*(a*.5-u*.6),r+c*(o-u*.7),i+Math.sin(s)*l*(a*.5-u*.6)*.85,u])}},s=(e,n,r,s,c,l,u=0)=>{let[d,f]=i(e,n),p=[],m=Math.max(4,Math.floor(l/(u+1.6))),h=u?c*.42:c;o(p,d,r+h*.1,f,s,h,Math.round(m*1.6));let g=t()*Math.PI*2,_=d,v=f,y=r+h*.55,b=s;for(let e=0;e<u;e++){b*=.66+.12*t();let e=g+(t()-.5)*1.6;_+=Math.cos(e)*b*.2,v+=Math.sin(e)*b*.2;let n=(c-h*.55)/u*1.25;o(p,_,y,v,b,n,m),y+=n*.72}a.push({puffs:p.slice(0,Jy),base:r,scale:s*.07})};return s(19e3,5200,1500,14500,10800,42,3),s(15500,-12500,1450,8200,5200,28,2),s(11500,15e3,1400,6800,3900,24,2),s(-13e3,9e3,1500,8400,5600,26,2),s(-9e3,-15e3,1450,7200,3400,20,1),s(6200,-2900,1250,2500,1050,10),s(4300,3900,1200,1800,800,8),s(2600,-5200,1220,2100,900,8),s(-1200,2400,1180,1500,660,7),s(1100,-1800,1160,1200,540,6),s(-4200,-3300,1230,2200,920,8),s(8600,7800,1300,3e3,1300,10,1),s(9800,12500,1350,4400,2300,14,1),s(7400,-9e3,1300,3600,1700,12,1),a}var Qy=`
out vec2 vUv;
void main() {
  vUv = position.xy;
  gl_Position = vec4(position.xy, 0.0, 1.0);
}
`,$y=`
precision highp float;
precision highp sampler3D;
layout(location = 0) out vec4 outC;
in vec2 vUv;
uniform sampler3D tBillow;
uniform vec4 uPuffs[${Ky}];
uniform vec4 uClouds[${qy}];   // centre, radius of everything in it
uniform vec4 uCloudInfo[${qy}]; // first puff, how many, base (metres), scale of its billows
uniform int uCount;
uniform int uFace;
uniform vec3 uSunDir;
uniform vec3 uLit, uMid, uShade, uUnder, uHaze;
uniform float uTime;
uniform int uSteps;
uniform float uTexel;

// The billows, read so they have no creases where one texel meets the next.
vec4 billow(vec3 q) {
  q = q * 64.0 + 0.5;
  vec3 i = floor(q);
  vec3 f = q - i;
  f = f * f * (3.0 - 2.0 * f);
  return texture(tBillow, (i + f - 0.5) / 64.0);
}

float smin(float a, float b, float k) {
  float h = max(k - abs(a - b), 0.0) / k;
  return min(a, b) - h * h * k * 0.25;
}

// Signed distance to the clouds' surface, about: negative inside. scale comes back as the size of the billows there.
// fine: 1 for the surface as it is drawn, 0 for its big forms alone.
float map(vec3 p, float fine, out float base, out float scale) {
  float d = 1e9;
  base = 1200.0;
  scale = 300.0;
  for (int c = 0; c < ${qy}; c++) {
    if (c >= uCount) break;
    vec4 cl = uClouds[c];
    float db = length(p - cl.xyz) - cl.w;
    if (db > 0.0) {
      d = min(d, db + 150.0);
      continue;
    }
    vec4 info = uCloudInfo[c];
    int first = int(info.x);
    int n = int(info.y);
    float dc = 1e9;
    for (int i = 0; i < ${Jy}; i++) {
      if (i >= n) break;
      vec4 s = uPuffs[first + i];
      dc = smin(dc, length(p - s.xyz) - s.w, s.w * 0.55);
    }
    // billows: big round lumps, smaller ones riding on them, and a fine edge
    float sc = info.w;
    vec3 q = p / (sc * 7.5) + vec3(0.0, -uTime * 0.004, uTime * 0.002) + float(c) * 0.37;
    vec4 n1 = billow(q);
    // heaps on heaps on heaps: the same round lumps at three sizes, which is what a cumulus is
    float lump = (0.55 - n1.r) * 2.4;
    if (fine > 0.0) lump += (0.55 - billow(q * 2.7 + 0.3).r) * 1.0 + (0.55 - billow(q * 7.3 + 0.7).r) * 0.36;
    dc += sc * lump;
    // a level base, rounded into the sides and gently uneven (in plain waves: seen edge on, a base shows every
    // crease its surface has)
    float under = info.z - p.y + sc * (0.55 * sin(p.x / (sc * 2.3) + float(c)) * sin(p.z / (sc * 1.9) + 1.7) + 0.3 * sin(p.x / (sc * 0.9) + p.z / (sc * 1.2)));
    float kk = sc * 1.3;
    float hh = max(kk - abs(dc - under), 0.0) / kk;
    dc = max(dc, under) + hh * hh * kk * 0.25;
    if (dc < d) {
      d = dc;
      base = info.z;
      scale = sc;
    }
  }
  return d;
}

// The colour of a cloud's surface at p, as it is painted: white where its heaps face the light, with a clean edge
// to the pale shade beside them, and a deeper tone down in the folds and underneath. No greys, and nothing soft.
vec3 paint(vec3 p, float base, float scale) {
  float b2, s2;
  const vec3 k1 = vec3(1.0, -1.0, -1.0);
  const vec3 k2 = vec3(-1.0, -1.0, 1.0);
  const vec3 k3 = vec3(-1.0, 1.0, -1.0);
  const vec3 k4 = vec3(1.0, 1.0, 1.0);
  // which way its big forms face, and which way the heap you are looking at does
  float e = scale * 1.2;
  vec3 Ns = normalize(k1 * map(p + k1 * e, 0.0, b2, s2) + k2 * map(p + k2 * e, 0.0, b2, s2) + k3 * map(p + k3 * e, 0.0, b2, s2) + k4 * map(p + k4 * e, 0.0, b2, s2));
  e = scale * 0.65;
  vec3 Nf = normalize(k1 * map(p + k1 * e, 1.0, b2, s2) + k2 * map(p + k2 * e, 1.0, b2, s2) + k3 * map(p + k3 * e, 1.0, b2, s2) + k4 * map(p + k4 * e, 1.0, b2, s2));
  vec3 N = normalize(Ns * 0.8 + Nf);
  // A painter keeps a cloud mostly white whatever the hour: its light comes from the sun's side, but raised, and
  // turned a little towards whoever is looking.
  vec3 L = normalize(uSunDir + vec3(0.0, 0.5, 0.0) - normalize(vec3(p.x, 0.0, p.z)) * 0.3);
  // how much cloud lies between here and that light
  float shadow = clamp(-map(p + L * scale * 2.6, 0.0, b2, s2) / (scale * 0.8), 0.0, 1.0) * 0.5;
  shadow += clamp(-map(p + L * scale * 6.5, 0.0, b2, s2) / (scale * 1.4), 0.0, 1.0) * 0.7;
  // how open it is here: 0 down in a fold between heaps, 1 on the crown of one
  float open = clamp(map(p + Nf * scale * 1.2, 1.0, b2, s2) / (scale * 1.2), 0.0, 1.0);
  float up = smoothstep(0.0, scale * 7.0, p.y - base);
  float light = (dot(N, L) * 0.5 + 0.5) * exp(-shadow * 1.3) + 0.1 * up + 0.04 * (open - 0.5) + 0.1 * N.y;
  // white where the light is, with a clean edge to it; in the shade it is modelled softly, from pale to deeper
  // The shade is one quiet tone that deepens slowly towards the base and away from the light. Its modelling comes
  // from the cloud's big forms alone: taken from every small heap, it breaks into stains.
  float broad = (dot(Ns, L) * 0.5 + 0.5) * exp(-shadow) + 0.25 * up + 0.1 * Ns.y + 0.34 * (billow(p / (scale * 16.0) + 0.3).b - 0.5);
  vec3 inShade = mix(uShade, uMid, smoothstep(0.25, 0.85, broad));
  // (a touch deeper down in the folds between heaps, with an edge to it)
  inShade *= 1.0 - 0.07 * (1.0 - smoothstep(0.1, 0.18, open));
  vec3 col = mix(inShade, uLit, smoothstep(0.55, 0.6, light));
  return mix(uUnder, col, smoothstep(-0.2, 0.5, (p.y - base) / (scale * 2.2) + light));
}

vec3 faceDir(vec2 st) {
  if (uFace == 0) return vec3(1.0, -st.y, -st.x);
  if (uFace == 1) return vec3(-1.0, -st.y, st.x);
  if (uFace == 2) return vec3(st.x, 1.0, st.y);
  if (uFace == 3) return vec3(st.x, -1.0, -st.y);
  if (uFace == 4) return vec3(st.x, -st.y, 1.0);
  return vec3(-st.x, -st.y, -1.0);
}

void main() {
  vec3 rd = normalize(faceDir(vUv));
  outC = vec4(0.0);
  if (rd.y < 0.012) return;
  vec3 ro = vec3(0.0, 70.0, 0.0);
  float t = (520.0 - ro.y) / rd.y;
  float tEnd = min((14500.0 - ro.y) / rd.y, 56000.0);
  float base, scale;
  float before = t;
  // the nearest the ray comes to cloud it misses, as an angle: that is what gives a painted edge and not a stair
  float near = 1e9;
  float tNear = t;
  bool hit = false;
  for (int i = 0; i < 260; i++) {
    if (i >= uSteps || t > tEnd) break;
    float d = map(ro + rd * t, 1.0, base, scale);
    if (d <= 0.0) { hit = true; break; }
    if (d < near * t) { near = d / t; tNear = t; }
    before = t;
    t += max(d * 0.5, 30.0);
  }
  float alpha = 0.0;
  if (hit) {
    // onto the skin itself, by halving between the last point outside and this one
    float lo = before;
    float hi = t;
    for (int k = 0; k < 9; k++) {
      float mid = 0.5 * (lo + hi);
      if (map(ro + rd * mid, 1.0, base, scale) > 0.0) lo = mid; else hi = mid;
    }
    t = hi;
    alpha = 1.0;
  } else if (near < uTexel) {
    t = tNear;
    alpha = 1.0 - near / uTexel;
  }
  if (alpha <= 0.0) return;
  vec3 p = ro + rd * t;
  map(p, 1.0, base, scale);
  vec3 col = paint(p, base, scale);
  // far clouds sink into the haze
  float haze = (1.0 - exp(-t / 46000.0)) * mix(1.0, 0.45, smoothstep(0.03, 0.35, rd.y));
  outC = vec4(mix(col, uHaze, haze) * alpha, alpha);
}
`;function eb(e,t,{size:n=1024,steps:r=110,live:i=!0}={}){let a=Zy(t),o=[],s=[],c=[];for(let e of a){let t=o.length,n=new Vr;for(let t of e.puffs)o.push(new In(...t)),n.expandByPoint(new q(t[0]-t[3],t[1]-t[3],t[2]-t[3])),n.expandByPoint(new q(t[0]+t[3],t[1]+t[3],t[2]+t[3]));let r=n.getBoundingSphere(new Si);s.push(new In(r.center.x,r.center.y,r.center.z,r.radius+e.scale*4)),c.push(new In(t,e.puffs.length,e.base,e.scale))}for(;o.length<Ky;)o.push(new In);for(;s.length<qy;)s.push(new In),c.push(new In);let l=new hp(n,{type:D,format:F,depthBuffer:!1,generateMipmaps:!1});l.texture.minFilter=l.texture.magFilter=h;let u={tBillow:{value:Xy()},uPuffs:{value:o.slice(0,Ky)},uClouds:{value:s.slice(0,qy)},uCloudInfo:{value:c.slice(0,qy)},uCount:{value:Math.min(a.length,qy)},uFace:{value:0},uSunDir:{value:new q(0,1,0)},uLit:{value:new Y(1.22,1.2,1.14)},uMid:{value:new Y(.7,.79,.88)},uShade:{value:new Y(.31,.46,.63)},uUnder:{value:new Y(.23,.33,.52)},uHaze:{value:new Y(.5,.72,.92)},uTime:{value:0},uSteps:{value:r},uTexel:{value:3/n}},d=new Qc({glslVersion:yt,vertexShader:Qy,fragmentShader:$y,uniforms:u,depthTest:!1,depthWrite:!1}),f=new Z;f.setAttribute(`position`,new X([-1,-1,0,3,-1,0,-1,3,0],3));let p=new xa(f,d);p.frustumCulled=!1;let m=new Er;m.add(p);let g=new cu,_=[0,1,2,4,5],v=0,y=0;function b(t,r){let i=e.getRenderTarget(),a=n/8;l.scissorTest=r>=0,r>=0&&l.scissor.set(0,r*a,n,a),u.uFace.value=t,e.setRenderTarget(l,t),e.render(m,g),l.scissorTest=!1,e.setRenderTarget(i)}return{texture:l.texture,uniforms:u,setLight(e){e.sunDir&&u.uSunDir.value.copy(e.sunDir).normalize();for(let[t,n]of[[`lit`,`uLit`],[`mid`,`uMid`],[`shade`,`uShade`],[`under`,`uUnder`],[`haze`,`uHaze`]])e[t]&&u[n].value.copy(e[t])},drawAll(){u.uTime.value=0;for(let e of _)b(e,-1);v=0},update(e){y+=e,i&&(u.uTime.value=0,b(_[Math.floor(v/8)],v%8),v=(v+1)%(_.length*8))},compile:()=>e.compileAsync(m,g)}}var tb=[{e:2,sun:[2.5,.95,.34],amb:[.2,.24,.5],bounce:[.08,.06,.04],zenith:[.028,.085,.36],mid:[.34,.33,.56],horizon:[1.12,.56,.24],glow:[2.2,.9,.3],fog:[.6,.42,.42],fogSun:[1.8,.8,.3],haze:17e-5,cloudLit:[1.6,.74,.4],cloudMid:[.82,.4,.42],cloudShade:[.27,.22,.45],cloudUnder:[.16,.14,.34],exposure:1.12},{e:11,sun:[3,1.9,.9],amb:[.3,.36,.62],bounce:[.12,.11,.05],zenith:[.03,.14,.46],mid:[.2,.4,.66],horizon:[1,.76,.44],glow:[1.6,.9,.35],fog:[.66,.6,.58],fogSun:[1.5,.95,.5],haze:2e-4,cloudLit:[1.5,1.1,.7],cloudMid:[.84,.64,.56],cloudShade:[.34,.34,.52],cloudUnder:[.22,.22,.42],exposure:1},{e:27,sun:[2.75,2.4,1.8],amb:[.3,.45,.78],bounce:[.1,.15,.05],zenith:[.04,.26,.62],mid:[.095,.4,.72],horizon:[.46,.7,.82],glow:[.7,.62,.42],fog:[.42,.68,.84],fogSun:[.85,.84,.78],haze:17e-5,cloudLit:[1.26,1.2,1.08],cloudMid:[.62,.74,.86],cloudShade:[.37,.51,.7],cloudUnder:[.3,.44,.64],exposure:1},{e:50,sun:[2.7,2.55,2.2],amb:[.3,.47,.82],bounce:[.1,.16,.05],zenith:[.035,.25,.62],mid:[.08,.39,.72],horizon:[.38,.7,.84],glow:[.5,.5,.45],fog:[.4,.67,.85],fogSun:[.66,.78,.9],haze:17e-5,cloudLit:[1.24,1.22,1.16],cloudMid:[.62,.74,.86],cloudShade:[.37,.51,.7],cloudUnder:[.3,.44,.64],exposure:1}];function nb(e){let t=tb[0],n=tb[tb.length-1];for(let r=0;r<tb.length-1;r++)if(e>=tb[r].e&&e<=tb[r+1].e){t=tb[r],n=tb[r+1];break}e<=tb[0].e&&(n=t),e>=n.e&&(t=n);let r=t===n?0:(e-t.e)/(n.e-t.e),i={};for(let e of Object.keys(t))i[e]=Array.isArray(t[e])?t[e].map((t,i)=>t+(n[e][i]-t)*r):t[e]+(n[e]-t[e])*r;return i}var rb=20.4;function ib(e,t){let[n,r]=a_,i=hn.clamp((e-12)/8.5,0,1),a=64-62*i**1.25,o=hn.degToRad(88-64*i),s=hn.degToRad(a),c=Math.cos(o)*n+Math.sin(o)*r,l=Math.cos(o)*r-Math.sin(o)*n;return t.set(c*Math.cos(s),Math.sin(s),l*Math.cos(s)).normalize(),a}var ab=`
out vec3 vDir;
void main() {
  vDir = position;
  vec4 p = projectionMatrix * vec4(mat3(viewMatrix) * position, 1.0);
  gl_Position = p.xyww;
}
`,ob=`
${ng}
precision highp samplerCube;
layout(location = 0) out vec4 outC;
in vec3 vDir;
uniform vec3 uZenith, uMid, uHorizon, uGlow;
uniform samplerCube tClouds;
uniform sampler2D tBack0, tBack1, tBack2;
uniform vec3 uBankLit, uBankShade;
uniform vec2 uBackdropEl;

void main() {
  vec3 d = normalize(vDir);
  float h = max(d.y, 0.0);
  vec3 col = mix(uHorizon, uMid, smoothstep(0.0, 0.3, pow(h, 0.8)));
  col = mix(col, uZenith, smoothstep(0.18, 0.85, h));
  float sd = max(dot(d, uSunDir), 0.0);
  col += uGlow * (pow(sd, 6.0) * 0.05 + pow(sd, 90.0) * 0.3 + pow(sd, 1400.0) * 1.4) + uSunColor * smoothstep(0.99988, 0.99996, sd) * 8.0;
  vec4 c = texture(tClouds, d);
  // towards the sun a cloud's edge is lit through
  c.rgb += uGlow * pow(sd, 40.0) * 0.5 * c.a * (1.0 - c.a) * 4.0;
  col = col * (1.0 - c.a) + c.rgb;
  // the far country stands in front of the clouds, mixed from its paint with this hour's light
  float v = (asin(clamp(d.y, -1.0, 1.0)) - uBackdropEl.x) / (uBackdropEl.y - uBackdropEl.x);
  float done = 0.0;
  if (v < 1.0) {
    vec2 uv = vec2(atan(d.z, d.x) / TAU + 0.5, max(v, 0.0));
    vec4 b0 = textureLod(tBack0, uv, 0.0);
    if (b0.a > 0.002) {
      vec4 b1 = textureLod(tBack1, uv, 0.0);
      vec4 b2 = textureLod(tBack2, uv, 0.0);
      float inv = 1.0 / b0.a;
      // how much of what is here is the bank of cloud, and how much the land behind it
      float bank = clamp(b2.b * inv, 0.0, 1.0);
      float w = inv / max(1.0 - bank, 1e-3);
      vec3 ambient = uSkyColor * 0.6 + dot(uSkyColor, vec3(0.38));
      vec3 inLight = b0.rgb * w;
      vec3 inShade = b1.rgb * w;
      vec3 land = mix(inShade * inShade * ambient, inLight * inLight * (uSunColor * 0.42 + uSkyColor * 0.35), clamp(b1.a * w, 0.0, 1.0));
      // (towards a low sun the far country warms, but it is not set alight)
      vec3 air = mix(uFogColor, uFogSunColor, pow(sd, 6.0) * 0.45);
      land = mix(land, air, 1.0 - pow(1.0 - clamp(b2.r * w, 0.0, 0.996), uFogDensity / 0.00015));
      land = mix(land, vec3(0.86, 0.92, 1.0) * (uSkyColor * 0.5 + uSunColor * 0.26), clamp(b2.g * w, 0.0, 1.0));
      vec3 cloud = mix(uBankShade, uBankLit, clamp(b2.a * inv / max(bank, 1e-3), 0.0, 1.0) * (0.2 + 0.8 * smoothstep(0.04, 0.4, uSunDir.y)));
      land = mix(land, mix(cloud, air, 0.18), bank);
      col = mix(col, land, b0.a);
      done = b0.a * (1.0 - bank);
    }
  }
  // (alpha marks what is painted already and wants no more of the brush: the far country)
  outC = vec4(col, 1.0 - 0.75 * done);
}
`,sb=()=>new Y;function cb(e,{cloudSize:t=1024,cloudSteps:n=110,liveClouds:r=!0,backdropWidth:i=8192}={}){let a=eb(e,a_,{size:t,steps:n,live:r}),o=new q;ib(16,o);let s=Gy(e,a_,[o.x,o.z],{width:i,ground:i>4096?4096:2048}),c=new q,l={fog:sb(),lit:sb(),mid:sb(),shade:sb(),under:sb()},u={clouds:a,backdrop:s,drawn:NaN,light(e){let t=nb(ib(e,c));a.setLight({sunDir:c,lit:l.lit.setRGB(...t.cloudLit),mid:l.mid.setRGB(...t.cloudMid),shade:l.shade.setRGB(...t.cloudShade),under:l.under.setRGB(...t.cloudUnder),haze:l.fog.setRGB(...t.fog)}),a.drawAll(),u.drawn=e}};return u}function lb(e,t,{parts:n=null,...r}={}){n??=cb(e,r);let{clouds:i,backdrop:a}=n,o={...t.uniforms,uZenith:{value:new Y},uMid:{value:new Y},uHorizon:{value:new Y},uGlow:{value:new Y},tClouds:{value:i.texture},tBack0:{value:a.textures[0]},tBack1:{value:a.textures[1]},tBack2:{value:a.textures[2]},uBackdropEl:{value:new K(a.el[0],a.el[1])},uBankLit:{value:new Y},uBankShade:{value:new Y}},s=new Qc({glslVersion:yt,vertexShader:ab,fragmentShader:ob,uniforms:o,side:1,depthWrite:!1,depthTest:!0}),c=new xa(new Oc(1,4),s);c.frustumCulled=!1,c.renderOrder=-10;let l=new q,u={sun:sb(),amb:sb(),bounce:sb(),fog:sb(),fogSun:sb(),lit:sb(),mid:sb(),shade:sb(),under:sb()},d={mesh:c,uniforms:o,clouds:i,backdrop:a,hour:15.5,exposure:1,sunDir:l,elevation:0,setHour(e,r=!1){d.hour=e;let a=ib(e,l);d.elevation=a;let s=nb(a);t.setLight({sunDir:l,sunColor:u.sun.setRGB(...s.sun),skyColor:u.amb.setRGB(...s.amb),bounceColor:u.bounce.setRGB(...s.bounce),fogColor:u.fog.setRGB(...s.fog),fogSunColor:u.fogSun.setRGB(...s.fogSun),fogDensity:s.haze,cloudShade:.15}),o.uZenith.value.setRGB(...s.zenith),o.uMid.value.setRGB(...s.mid),o.uHorizon.value.setRGB(...s.horizon),o.uGlow.value.setRGB(...s.glow),o.uBankLit.value.setRGB(...s.cloudLit),o.uBankShade.value.setRGB(...s.cloudMid.map((e,t)=>e*.35+s.cloudShade[t]*.65)),i.setLight({sunDir:l,lit:u.lit.setRGB(...s.cloudLit),mid:u.mid.setRGB(...s.cloudMid),shade:u.shade.setRGB(...s.cloudShade),under:u.under.setRGB(...s.cloudUnder),haze:u.fog}),r?n.drawn=NaN:n.drawn!==e&&(i.drawAll(),n.drawn=e),d.exposure=s.exposure},update(e,t){c.position.copy(t.position),c.scale.setScalar(t.far*.5),i.update(e)}};return d}function ub(e,t=6,n=.985){let r=Math.floor(e.sampleRate*t),i=e.createBuffer(2,r,e.sampleRate);for(let t=0;t<2;t++){let a=i.getChannelData(t),o=0;for(let e=0;e<r;e++)o=n*o+(Math.random()*2-1)*(1-n)*6,a[e]=o;let s=Math.floor(e.sampleRate*.25);for(let e=0;e<s;e++){let t=e/s;a[e]=a[e]*t+a[r-s+e]*(1-t)}}return i}function db(e,t,n=1){let r=e.createBufferSource();return r.buffer=t,r.loop=!0,r.playbackRate.value=n,r.start(0,Math.random()*t.duration),r}function fb(e,t,n,r=.7){let i=e.createBiquadFilter();return i.type=t,i.frequency.value=n,i.Q.value=r,i}function pb(e,t,n,r){let i=e.createOscillator();i.frequency.value=t;let a=e.createGain();return a.gain.value=n,i.connect(a).connect(r),i.start(),i}function mb(e,t){let n=e.createDynamicsCompressor();n.threshold.value=-10,n.knee.value=8,n.ratio.value=6,n.attack.value=.005,n.release.value=.25,n.connect(t);let r=ub(e,7,.985),i=ub(e,5,.6),a={},o=(t,r=1)=>{let i=e.createGain();i.gain.value=0;let o=e.createGain();return o.gain.value=r,i.connect(o).connect(n),a[t]=i,i};{let t=o(`grass`,.9),n=fb(e,`bandpass`,700,.45),i=fb(e,`lowpass`,2400);pb(e,.07,280,n.frequency),db(e,r).connect(n).connect(i).connect(t)}{let t=o(`leaves`,.2),n=fb(e,`highpass`,1800),r=fb(e,`bandpass`,4200,.8),a=e.createGain();a.gain.value=.5;let s=fb(e,`lowpass`,9),c=e.createGain();c.gain.value=1.2,db(e,i,.37).connect(s).connect(c).connect(a.gain),db(e,i).connect(n).connect(r).connect(a).connect(t)}{let t=o(`lake`,.45),n=fb(e,`lowpass`,420),a=e.createGain();a.gain.value=.7,pb(e,.23,.35,a.gain),db(e,r,.8).connect(n).connect(a).connect(t);let s=fb(e,`bandpass`,1100,3),c=e.createGain();c.gain.value=.15;let l=fb(e,`lowpass`,4),u=e.createGain();u.gain.value=1.2,db(e,i,.21).connect(l).connect(u).connect(c.gain),pb(e,.31,300,s.frequency),db(e,i,.9).connect(s).connect(c).connect(t)}{let t=o(`crickets`,.22);for(let[n,r,i,a]of[[4650,16,.55,-.6],[4380,19,.8,.55]]){let o=e.createOscillator();o.frequency.value=n;let s=e.createGain();s.gain.value=.5;let c=e.createOscillator();c.type=`square`,c.frequency.value=r;let l=e.createGain();l.gain.value=.5,c.connect(l).connect(s.gain);let u=e.createGain();u.gain.value=.5;let d=e.createOscillator();d.type=`square`,d.frequency.value=i;let f=e.createGain();f.gain.value=.5,d.connect(f).connect(u.gain);let p=e.createStereoPanner();p.pan.value=a,o.connect(s).connect(u).connect(p).connect(t);for(let t of[o,c,d])t.start(e.currentTime+Math.random())}}let s=o(`summer`,1.4);{let t=fb(e,`bandpass`,5600,6),n=e.createGain();n.gain.value=.02,db(e,i).connect(t).connect(n).connect(s)}let c=1;function l(){let t=e.currentTime+.05,n=e.createStereoPanner();n.pan.value=Math.random()*1.6-.8;let r=e.createGain();r.gain.value=.25+Math.random()*.5,n.connect(r).connect(s);let i=2+Math.floor(Math.random()*5),a=2400+Math.random()*2200,o=t;for(let t=0;t<i;t++){let t=e.createOscillator(),r=e.createGain(),i=.05+Math.random()*.12,s=a*(.85+Math.random()*.4),c=s*(Math.random()<.5?1.35:.72);t.frequency.setValueAtTime(s,o),t.frequency.exponentialRampToValueAtTime(c,o+i),r.gain.setValueAtTime(0,o),r.gain.linearRampToValueAtTime(.5,o+.012),r.gain.exponentialRampToValueAtTime(.001,o+i),t.connect(r).connect(n),t.start(o),t.stop(o+i+.02),o+=i+.03+Math.random()*.09}}return{voices:a,update(e){s.gain.value<.02||(c-=e,c>0||(l(),c=1.2+Math.random()*4.5))},chime(){let t=e.currentTime,r=[784,880,988,1175][Math.floor(Math.random()*4)];for(let[i,a,o]of[[1,.16,2.4],[2.01,.06,1.4],[3,.03,.9],[4.12,.015,.6]]){let s=e.createOscillator();s.frequency.value=r*i;let c=e.createGain();c.gain.setValueAtTime(0,t),c.gain.linearRampToValueAtTime(a,t+.006),c.gain.exponentialRampToValueAtTime(1e-4,t+o),s.connect(c).connect(n),s.start(t),s.stop(t+o+.05)}}}}var hb=[`grass`,`leaves`,`summer`,`lake`,`crickets`];function gb(e=`./audio/`){let t=null,n=null,r=[],i={},a={},o=!1,s=-1,c=null,l=!0;return{async load(){let a=window.AudioContext||window.webkitAudioContext;if(!a)return;let o=[];try{let t=await fetch(`${e}manifest.json`);t.ok&&(o=await t.json())}catch{}try{t=new a,n=t.createGain(),n.gain.value=0,n.connect(t.destination),await Promise.all(o.map(async n=>{try{let a=await t.decodeAudioData(await(await fetch(`${e}${n}.ogg`)).arrayBuffer());n.startsWith(`footstep`)?r.push(a):i[n]=a}catch{}}))}catch{t=null}},start(){if(t&&(t.state===`suspended`&&t.resume(),!o)){o=!0,c=mb(t,n);for(let e of hb){if(!i[e]){a[e]=c.voices[e];continue}let r=t.createBufferSource();r.buffer=i[e],r.loop=!0;let o=t.createGain();o.gain.value=0,r.connect(o).connect(n),r.start(0,Math.random()*r.buffer.duration),a[e]=o}}},step(e){if(!t||t.state!==`running`||!r.length)return;let i=Math.floor(Math.random()*r.length);i===s&&(i=(i+1)%r.length),s=i;let a=t.createBufferSource();a.buffer=r[i],a.playbackRate.value=.92+Math.random()*.16;let o=t.createGain();o.gain.value=e;let c=t.createStereoPanner();c.pan.value=(Math.random()-.5)*.3,a.connect(o).connect(c).connect(n),a.start()},update(e,t){if(!o)return;c?.update(e);let n=Math.min(1,e*1.6),r=(e,t)=>{a[e]&&(a[e].gain.value+=(t-a[e].gain.value)*n)};r(`grass`,(.1+Math.min(.5,t.wind*.055))*(.55+.45*t.gust)),r(`leaves`,t.tree*(.3+.5*t.gust)),r(`summer`,.55*(1-.5*t.dusk)),r(`lake`,t.tarn*.5),r(`crickets`,t.dusk*.3)},chime(){t?.state===`running`&&!l&&c?.chime()},get muted(){return l},set muted(e){l=e,n&&n.gain.setTargetAtTime(e?0:.8,t.currentTime,.25)}}}function _b(e){let t=e>>>0;return()=>{t=t+1831565813>>>0;let e=t;return e=Math.imul(e^e>>>15,e|1),e^=e+Math.imul(e^e>>>7,e|61),((e^e>>>14)>>>0)/4294967296}}function vb(e,t=512){let n=document.createElement(`canvas`);n.width=n.height=t;let r=n.getContext(`2d`),i=_b(8128);r.fillStyle=`rgb(128,0,0)`,r.fillRect(0,0,t,t);let a=(e,n,i,a,o,s)=>{r.fillStyle=s;for(let s of[-t,0,t])for(let c of[-t,0,t])e+s<-i-a||e+s>t+i+a||n+c<-i-a||n+c>t+i+a||(r.beginPath(),r.ellipse(e+s,n+c,i,a,o,0,Math.PI*2),r.fill())};for(let e=0;e<70;e++)a(i()*t,i()*t,26+i()*44,12+i()*20,i()*Math.PI,`rgba(${i()<.5?86:172},0,0,0.5)`);for(let e=0;e<420;e++){let e=i()<.52?40+i()*60:160+i()*80;a(i()*t,i()*t,7+i()*16,2+i()*4,i()*Math.PI,`rgba(${Math.round(e)},0,0,0.85)`)}for(let e=0;e<900;e++){let e=i()<.5?30+i()*60:170+i()*85;a(i()*t,i()*t,2+i()*4,1+i()*2,i()*Math.PI,`rgb(${Math.round(e)},0,0)`)}r.globalCompositeOperation=`lighter`;for(let e=0;e<9;e++){let e=i()*t,n=i()*t,r=2+Math.floor(i()*5);for(let t=0;t<r;t++)a(e+(i()-.5)*90,n+(i()-.5)*90,4+i()*9,3+i()*5,i()*Math.PI,`rgb(0,0,255)`)}let o=new Jo(n);return o.wrapS=o.wrapT=s,o.generateMipmaps=!0,o.minFilter=v,o.magFilter=h,o.anisotropy=e.capabilities.getMaxAnisotropy(),o}var yb=Math.PI*2,bb=q;function xb(e){let t=e>>>0;return()=>{t=t+1831565813>>>0;let e=t;return e=Math.imul(e^e>>>15,e|1),e^=e+Math.imul(e^e>>>7,e|61),((e^e>>>14)>>>0)/4294967296}}function Sb(){let e=document.createElement(`canvas`);e.width=e.height=256;let t=e.getContext(`2d`);t.fillStyle=`#000`,t.fillRect(0,0,256,256),t.fillStyle=`#fff`;let n=xb(77),r=(e,r,i,a)=>{let o=a*(.34+n()*.16);t.fillStyle=`rgb(${130+Math.floor(n()*125)},255,0)`,t.save(),t.translate(e,r),t.rotate(i),t.beginPath(),t.moveTo(-a/2,0),t.quadraticCurveTo(-a*.05,-o,a/2,0),t.quadraticCurveTo(-a*.05,o,-a/2,0),t.fill(),t.restore()};for(let e=0;e<30;e++){let t=e/30*yb*2+n()*.6,i=22+n()*78;r(128+Math.cos(t)*i,128+Math.sin(t)*i,t+(n()-.5)*1.5,26+n()*22)}for(let e=0;e<8;e++)r(128+(n()-.5)*60,128+(n()-.5)*60,n()*yb,28+n()*18);let i=new Jo(e);return i.generateMipmaps=!0,i.minFilter=v,i.magFilter=h,i.anisotropy=4,i.wrapS=i.wrapT=s,i}function Cb(e,t){let n=xb(e),r=[],i=[],a=n()*yb,o=new bb(Math.cos(a)*t.trunk*.1,t.trunk,Math.sin(a)*t.trunk*.1);r.push({pts:[new bb(0,-.8,0),new bb(Math.cos(a+1)*t.girth*.25,t.trunk*.45,Math.sin(a+1)*t.girth*.25),o],r0:t.girth,r1:t.girth*.74,sides:14,root:!0});let s=t.trunk+t.rise*.42,c=a+2+n()*2,l=n=>t.crown*(.82+.26*Math.cos(n-c)+.1*Math.sin(n*3+e));for(let e=0;e<t.limbs;e++){let a=(e+.15+n()*.7)/t.limbs*yb,s=.38+n()*.62,c=l(a)*(.62+.26*n()),u=new bb(Math.cos(a)*Math.cos(s),Math.sin(s),Math.sin(a)*Math.cos(s)),d=new bb(-Math.sin(a),0,Math.cos(a)),f=(n()-.5)*c*.3,p=o.clone().addScaledVector(u,c*.5).addScaledVector(d,f).add(new bb(0,-c*.07,0)),m=o.clone().addScaledVector(u,c).addScaledVector(d,f*.4).add(new bb(0,c*.14,0)),h=o.clone().lerp(p,.5).add(new bb((n()-.5)*c*.12,c*.05,(n()-.5)*c*.12)),g=p.clone().lerp(m,.5).add(new bb((n()-.5)*c*.14,-c*.04,(n()-.5)*c*.14)),_=new bs([o.clone().add(new bb(0,-t.girth*.5,0)),h,p,g,m]);r.push({curve:_,r0:t.girth*(.36+.12*n()),r1:t.girth*.1,sides:9}),i.push({c:m,r:t.clump*(.85+.4*n())});let v=2+ +(n()<.5);for(let e=0;e<v;e++){let e=_.getPoint(.4+.45*n()),o=a+(n()-.5)*2.2,s=.25+n()*.85,l=c*(.34+.26*n()),u=new bb(Math.cos(o)*Math.cos(s),Math.sin(s),Math.sin(o)*Math.cos(s)),d=e.clone().addScaledVector(u,l),f=e.clone().addScaledVector(u,l*.5).add(new bb((n()-.5)*l*.2,l*.06,(n()-.5)*l*.2));r.push({pts:[e,f,d],r0:t.girth*.13,r1:t.girth*.035,sides:6}),i.push({c:d,r:t.clump*(.75+.45*n())})}}for(let e=0;e<t.fill;e++){let e=n()*yb,r=Math.min(.96,-.3+1.3*n()),a=Math.sqrt(1-r*r),o=l(e)*.8;i.push({c:new bb(Math.cos(e)*a*o,s+r*t.rise*.5,Math.sin(e)*a*o),r:t.clump*(.8+.5*n())})}let u=[],d=[],f=[],p=[];for(let e of r){let t=e.curve??new bs(e.pts),r=e.root?14:10,i=t.computeFrenetFrames(r,!1),a=u.length/3,o=n()*yb,s=t.getLength();for(let n=0;n<=r;n++){let a=n/r,c=t.getPoint(a);for(let t=0;t<=e.sides;t++){let r=t/e.sides*yb,l=e.r0+(e.r1-e.r0)*a;e.root&&(l*=1+1.15*Math.exp(-a*7.5)*(.62+.38*Math.sin(r*5+o)*Math.sin(r*2+1))+.07*Math.sin(r*3+a*6+o));let p=i.normals[n].clone().multiplyScalar(Math.cos(r)).addScaledVector(i.binormals[n],Math.sin(r));u.push(c.x+p.x*l,c.y+p.y*l,c.z+p.z*l),d.push(p.x,p.y,p.z),f.push(t/e.sides,a*s)}}for(let t=0;t<r;t++)for(let n=0;n<e.sides;n++){let r=a+t*(e.sides+1)+n,i=r+e.sides+1;p.push(r,i,r+1,r+1,i,i+1)}}let m=new Z;m.setAttribute(`position`,new X(u,3)),m.setAttribute(`normal`,new X(d,3)),m.setAttribute(`aBark`,new X(f,2)),m.setIndex(p);let h=[],g=[],_=[],v=[],y=[],b=[],x=[],S=[],C=new Oc(1,1).attributes.position,w=new bb(0,s,0),T=new bb,E=new bb,D=new bb;for(let e of i){let r=n();{let i=Math.hypot((e.c.x-w.x)/t.crown,(e.c.y-w.y)/(t.rise*.5),(e.c.z-w.z)/t.crown),a=hn.smoothstep((e.c.y-w.y)/(t.rise*.5),-.9,.2),o=hn.clamp((.22+.78*hn.smoothstep(i,.4,.95))*(.55+.45*a),0,1),s=n(),c=e.r*.7;for(let t=0;t<C.count;t++){let n=C.getX(t),i=C.getY(t),a=C.getZ(t),l=c*(.9+.2*Math.sin(n*5+r*9)*Math.sin(a*4+i*3));b.push(e.c.x+n*l,e.c.y+i*l*.82,e.c.z+a*l),x.push(n,i,a),S.push(s,r,o,e.c.y)}}for(let i=0;i<t.cards;i++){let a=1-2*(i+.5)/t.cards,o=Math.sqrt(1-a*a),s=i*2.39996+r*yb,c=new bb(Math.cos(s)*o,a,Math.sin(s)*o).add(new bb(n()-.5,n()-.5,n()-.5).multiplyScalar(.5)).normalize(),l=e.c.clone().addScaledVector(c,e.r*(.62+.42*n()));D.copy(c).add(new bb(n()-.5,n()-.5,n()-.5).multiplyScalar(1.1)).normalize(),T.set(n()-.5,n()-.5,n()-.5).cross(D).normalize(),E.crossVectors(D,T);let u=t.spray*(.75+.6*n()),d=h.length/3,f=Math.hypot((l.x-w.x)/t.crown,(l.y-w.y)/(t.rise*.5),(l.z-w.z)/t.crown),p=hn.smoothstep((l.y-w.y)/(t.rise*.5),-.9,.2),m=hn.clamp((.22+.78*hn.smoothstep(f,.4,.95))*(.55+.45*p),0,1),b=n();for(let[e,t]of[[-1,-1],[1,-1],[1,1],[-1,1]])h.push(l.x+(T.x*e+E.x*t)*u*.5,l.y+(T.y*e+E.y*t)*u*.5,l.z+(T.z*e+E.z*t)*u*.5),g.push(e*.5+.5,t*.5+.5),_.push(c.x,c.y,c.z),v.push(b,r,m,l.y);y.push(d,d+1,d+2,d,d+2,d+3)}}let O=new Z;O.setAttribute(`position`,new X(h,3)),O.setAttribute(`uv`,new X(g,2)),O.setAttribute(`aBall`,new X(_,3)),O.setAttribute(`aRand`,new X(v,4)),O.setIndex(y);let k=new Z;return k.setAttribute(`position`,new X(b,3)),k.setAttribute(`aBall`,new X(x,3)),k.setAttribute(`aRand`,new X(S,4)),{wood:m,leaves:O,core:k,crown:t.crown,crownY:s,height:t.trunk+t.rise}}var wb=`
${ng}
in vec3 aBall;
in vec4 aRand;   // tone, its clump's phase, how open to the sky, height in the tree
out vec2 vUv;
out vec3 vWorld;
out vec3 vBall;
out vec3 vRand;
void main() {
  mat4 m = modelMatrix * instanceMatrix;
  vec4 w = m * vec4(position, 1.0);
  vBall = normalize(mat3(m) * aBall);
  // the crown rides the gusts, each clump in its own time, each spray trembling
  float gust = gustAt(w.xz);
  float U = uWind.z;
  float high = smoothstep(1.0, 9.0, aRand.w);
  float sway = sin(uTime * (0.9 + 0.5 * aRand.y) + aRand.y * 37.0);
  w.xz += uWind.xy * (U * (0.012 + 0.05 * gust) + sway * 0.012 * U) * high;
  w.xyz += vBall * sin(uTime * (2.6 + 2.0 * aRand.x) + aRand.x * 61.0) * 0.006 * U * (0.4 + gust);
  vWorld = w.xyz;
  vUv = uv;
  vRand = aRand.xyz;
  gl_Position = projectionMatrix * viewMatrix * w;
}
`,Tb=`
${ng}
layout(location = 0) out vec4 outC;
uniform sampler2D tLeaf;
in vec2 vUv;
in vec3 vWorld;
in vec3 vBall;
in vec3 vRand;
void main() {
  float dist = distance(vWorld, uCamPos);
  vec4 leaf = texture(tLeaf, vUv);
  float a = leaf.g;
  // far off the sprays thin to nothing as their picture blurs: keep the crown whole
  float cut = mix(0.5, 0.16, smoothstep(20.0, 100.0, dist));
  a = clamp((a - cut) / max(fwidth(a), 1e-4) + 0.5, 0.0, 1.0);
  if (a < 0.04) discard;
  vec3 N = normalize(vBall);
  vec3 V = normalize(uCamPos - vWorld);
  // each clump is lit as a ball; each spray is a shade of its own, as brush marks are
  float tone = dot(N, uSunDir) * 0.5 + 0.5;
  // (from far off a crown is its lobes, not its leaves: the brush gets broader)
  float fine = mix(1.0, 0.25, smoothstep(14.0, 90.0, dist));
  tone = tone * mix(0.5, 1.0, vRand.z) + ((vRand.x - 0.5) * 0.36 + (leaf.r / max(leaf.g, 0.05) - 0.75) * 0.5) * fine;
  vec3 col = mix(vec3(0.014, 0.1, 0.1), vec3(0.035, 0.2, 0.12), smoothstep(0.3, 0.42, tone));
  col = mix(col, vec3(0.1, 0.36, 0.12), smoothstep(0.56, 0.68, tone));
  col = mix(col, vec3(0.3, 0.56, 0.15), smoothstep(0.84, 0.92, tone));
  float cloud = cloudAt(vWorld.xz + uSunDir.xz * 30.0);
  vec3 light = uSkyColor * 0.9 + uSunColor * (0.2 + 0.16 * smoothstep(0.5, 0.9, tone)) * mix(0.45, 1.0, clamp((cloud - 0.5) * 2.0, 0.0, 1.0));
  // seen against the sun, leaves glow
  float through = pow(max(dot(-V, uSunDir), 0.0), 2.0);
  col = col * light + vec3(0.3, 0.5, 0.05) * uSunColor * through * (0.05 + 0.24 * vRand.x * vRand.x);
  outC = vec4(applyFog(col, vWorld), a);
}
`,Eb=`
${ng}
layout(location = 0) out vec4 outC;
in vec2 vUv;
in vec3 vWorld;
in vec3 vBall;
in vec3 vRand;
uniform sampler2D tLeaf;
void main() {
  vec3 N = normalize(vBall);
  // not a solid: a shell of leaves, cut from the same sprays, that you see into
  vec3 w = abs(N);
  vec2 uv = w.y > max(w.x, w.z) ? N.xz : w.x > w.z ? N.zy : N.xy;
  if (texture(tLeaf, uv * 2.3 + vRand.y * 7.0).g < 0.4) discard;
  if (!gl_FrontFacing) N = -N;
  float n = texture(tNoise, vWorld.xz * 0.9 + vWorld.y * 0.37).r;
  // lit as the sprays on it are, a tone deeper: it is the inside of the crown
  float tone = dot(N, uSunDir) * 0.5 + 0.5;
  tone = tone * mix(0.5, 1.0, vRand.z) + (n - 0.5) * 0.36 - 0.04;
  vec3 col = mix(vec3(0.014, 0.1, 0.1), vec3(0.035, 0.19, 0.115), smoothstep(0.3, 0.42, tone));
  col = mix(col, vec3(0.09, 0.33, 0.11), smoothstep(0.58, 0.7, tone));
  float cloud = cloudAt(vWorld.xz + uSunDir.xz * 30.0);
  col *= uSkyColor * 0.9 + uSunColor * (0.2 + 0.14 * smoothstep(0.5, 0.9, tone)) * mix(0.45, 1.0, clamp((cloud - 0.5) * 2.0, 0.0, 1.0));
  outC = vec4(applyFog(col, vWorld), 1.0);
}
`,Db=`
${ng}
in vec2 aBark;
out vec3 vWorld;
out vec3 vNormal;
out vec2 vBark;
void main() {
  mat4 m = modelMatrix * instanceMatrix;
  vec4 w = m * vec4(position, 1.0);
  vWorld = w.xyz;
  vNormal = normalize(mat3(m) * normal);
  vBark = aBark;
  gl_Position = projectionMatrix * viewMatrix * w;
}
`,Ob=`
${ng}
layout(location = 0) out vec4 outC;
in vec3 vWorld;
in vec3 vNormal;
in vec2 vBark;
void main() {
  vec3 N = normalize(vNormal);
  // bark in long strokes, greyer in places, green with moss where it faces up or away from the sun
  float stroke = texture(tNoise, vec2(vBark.x * 3.0, vBark.y * 0.16)).r;
  float patchy = texture(tNoise, vec2(vBark.x * 0.9 + 0.3, vBark.y * 0.07)).g;
  vec3 col = mix(vec3(0.085, 0.062, 0.05), vec3(0.2, 0.16, 0.125), smoothstep(0.3, 0.7, stroke));
  col = mix(col, vec3(0.24, 0.23, 0.2), smoothstep(0.55, 0.75, patchy) * 0.6);
  float moss = smoothstep(0.45, 0.7, patchy * 0.6 + N.y * 0.5 - dot(N, uSunDir) * 0.2 + 0.25) * 0.75;
  col = mix(col, vec3(0.06, 0.13, 0.035), moss);
  // it stands in its own shade: light reaches it in spots
  float spot = smoothstep(0.56, 0.7, texture(tNoise, vec2(vBark.x * 1.7, vBark.y * 0.21) + uTime * vec2(0.011, 0.007)).b);
  float sun = smoothstep(-0.15, 0.6, dot(N, uSunDir)) * mix(0.1, 1.0, spot);
  vec3 c = col * (uSunColor * sun * 0.62 + mix(uBounceColor * 2.0, uSkyColor, N.y * 0.5 + 0.5) * 0.85);
  outC = vec4(applyFog(c, vWorld), 1.0);
}
`;function kb(){let e=xb(20),t=[[u_[0],u_[1],0,1,2.4]],n=(e,n,r)=>{if(A_(e,n)<1.75||w_(e,n)<6||T_(e,n))return!1;for(let i of t)if(Math.hypot(i[0]-e,i[1]-n)<r*(i[2]===0?5:1))return!1;return!0},r=(r,i,a,o)=>{let s=e()*yb,c=1.3+.9*e(),l=0;for(let u=0;u<a&&l<a*40;l++){let a=e()*yb,l=e()**.7,d=Math.cos(a)*l*o*c,f=Math.sin(a)*l*o/c,[p,m]=f_(r+d*Math.cos(s)-f*Math.sin(s),i+d*Math.sin(s)+f*Math.cos(s));Math.hypot(p,m)>596||!n(p,m,4.4)||(t.push([p,m,1+Math.floor(e()*3),(1.95-1.05*l)*(.75+.5*e()),e()*yb]),u++)}},i=(r,i)=>{let[a,o]=f_(r,i);n(a,o,9)&&t.push([a,o,1+Math.floor(e()*3),1.9+.5*e(),e()*yb])},a=S_();for(let t=0;t<26;t++){let t=(e()-.5)*yb;Math.abs(t)<.55&&e()<.8&&(t+=Math.sign(t||1)*.6);let n=a+36+250*e()**1.5;r(Math.cos(t)*n,Math.sin(t)*n,5+Math.floor(e()*10),11+15*e())}r(a+70,-(a*.6+46),9,16),r(a+96,a*.6+52,8,15);for(let t=0;t<7;t++){let t=e()*yb,n=a+24+110*e();i(Math.cos(t)*n,Math.sin(t)*n)}for(let t=0;t<12;t++){let t=e()*yb;r((455+95*e())*Math.cos(t),(455+95*e())*Math.sin(t),5+Math.floor(e()*9),16+14*e())}return t}function Ab(e,{cards:t=1}={}){let n=[Cb(11,{trunk:3.6,girth:.92,crown:11.5,rise:12.5,limbs:6,clump:2.7,fill:26,cards:Math.round(92*t),spray:1.05}),Cb(23,{trunk:2.6,girth:.42,crown:5.6,rise:8.2,limbs:4,clump:1.9,fill:9,cards:Math.round(48*t),spray:1.15}),Cb(37,{trunk:3.3,girth:.36,crown:4.6,rise:9.6,limbs:4,clump:1.7,fill:10,cards:Math.round(48*t),spray:1.1}),Cb(51,{trunk:2.2,girth:.5,crown:6.6,rise:7.2,limbs:5,clump:2.1,fill:8,cards:Math.round(48*t),spray:1.15})],r=kb();for(let e=1;e<=3;e++){let t=[0,23,37,51][e],r=[null,{trunk:2.6,girth:.42,crown:5.6,rise:8.2,limbs:4,clump:1.9,fill:9},{trunk:3.3,girth:.36,crown:4.6,rise:9.6,limbs:4,clump:1.7,fill:10},{trunk:2.2,girth:.5,crown:6.6,rise:7.2,limbs:5,clump:2.1,fill:8}][e];n.push(Cb(t,{...r,cards:22,spray:1.85}))}{let e=xb(808),[t,n]=[.6,.8],i=0;for(let a=0;a<34&&i<330;a++){let a=-700+1900*e(),o=(e()-.5)*2200,s=4+Math.floor(e()*13),c=7+s*1.7;for(let l=0;l<s;l++){let s=e()*yb,l=c*e()**.7,u=(a+Math.cos(s)*l)*t-(o+Math.sin(s)*l)*n,d=(a+Math.cos(s)*l)*n+(o+Math.sin(s)*l)*t;Math.hypot(u,d)<640||Math.hypot(u,d)>1250||j_(u,d)<26||(r.push([u,d,4+Math.floor(e()*3),.9+.9*e()**1.5,e()*yb]),i++)}}}let i=n.length;n.push(Cb(71,{trunk:.25,girth:.07,crown:1.5,rise:1.9,limbs:3,clump:.8,fill:7,cards:Math.round(30*t),spray:.62}),Cb(83,{trunk:.2,girth:.06,crown:2.1,rise:1.5,limbs:4,clump:.85,fill:8,cards:Math.round(30*t),spray:.62}));{let e=xb(313),t=(t,n,a)=>{w_(t,n)<2.4||A_(t,n)<.9||T_(t,n)||Math.hypot(t-u_[0],n-u_[1])<9||r.push([t,n,i+(e()<.5?0:1),a,e()*yb])},n=[];for(let t=0;t<24;t++){let t=e()*yb,r=S_()+14+150*e();n.push([Math.cos(t)*r,Math.sin(t)*r])}for(let[r,i]of n){let n=1+Math.floor(e()*4);for(let a=0;a<n;a++){let n=e()*yb,a=1+5*e(),[o,s]=f_(r+Math.cos(n)*a,i+Math.sin(n)*a);t(o,s,.6+.9*e())}}for(let n=0,r=0;n<600&&r<16;n++){let n=d_.at[0]+(e()-.5)*d_.rx*3,i=d_.at[1]+(e()-.5)*d_.rx*3,a=A_(n,i);a<.93||a>1.16||(t(n,i,.6+.8*e()),r++)}for(let n of r.slice()){if(n[2]<1||n[2]>3||e()>.5)continue;let r=e()*yb,i=2.5+3.5*e();t(n[0]+Math.cos(r)*i,n[1]+Math.sin(r)*i,.6+.8*e())}for(let[n,r,i,a]of[[250,-60,14,15],[330,112,12,13],[382,-150,16,17],[472,44,12,13],[204,150,10,11],[524,-92,14,15],[162,-152,10,11],[432,222,12,13],[292,-232,10,11],[562,152,12,13],[352,12,8,8],[452,-232,10,11],[232,62,8,8],[584,-12,10,10],[140,212,9,10],[402,300,10,11],[498,-300,10,11],[310,-20,6,7]]){let o=e()*yb,s=1.4+e();for(let c=0;c<i;c++){let i=e()*yb,c=Math.sqrt(e()),l=Math.cos(i)*c*a*s,u=Math.sin(i)*c*a/s,[d,f]=f_(n+l*Math.cos(o)-u*Math.sin(o),r+l*Math.sin(o)+u*Math.cos(o));t(d,f,(1.5-.7*c)*(.6+.5*e()))}}for(let n=0;n<10;n++){let r=170+400*e(),i=-270+540*e(),a=Math.PI/2+(e()-.5)*1.3,o=70+90*e();for(let s=-o/2;s<o/2;s+=2.6+2.6*e()){if(Math.sin(s/15+n*2.1)>.7)continue;let o=7*Math.sin(s/26+n),[c,l]=f_(r+Math.cos(a)*s-Math.sin(a)*o,i+Math.sin(a)*s+Math.cos(a)*o);t(c,l,.55+.6*e())}}for(let n=0;n<24;n++){let n=e()*yb,r=90+500*Math.sqrt(e()),i=1+Math.floor(e()*3.5);for(let a=0;a<i;a++)t(Math.cos(n)*r+(e()-.5)*9,Math.sin(n)*r+(e()-.5)*9,.5+.9*e())}}let a=new gr,o=new Qc({glslVersion:yt,vertexShader:wb,fragmentShader:Tb,uniforms:{...e.uniforms,tLeaf:{value:Sb()}},side:2,alphaToCoverage:!0}),s=new Qc({glslVersion:yt,vertexShader:Db,fragmentShader:Ob,uniforms:e.uniforms}),c=new Qc({glslVersion:yt,vertexShader:wb,fragmentShader:Eb,uniforms:{...e.uniforms,tLeaf:o.uniforms.tLeaf},side:2}),l=new J,u=new gn,d=new bb(0,1,0);n.forEach((e,t)=>{let n=r.filter(e=>e[2]===t);if(!n.length)return;let i=new qa(e.leaves,o,n.length),f=new qa(e.wood,s,n.length),p=new qa(e.core,c,n.length);n.forEach(([e,t,,n,r],a)=>{u.setFromAxisAngle(d,r),l.compose(new bb(e,I_(e,t),t),u,new bb(n,n*(.94+.12*(a*7%5)*.25),n)),i.setMatrixAt(a,l),f.setMatrixAt(a,l),p.setMatrixAt(a,l)}),i.frustumCulled=f.frustumCulled=p.frustumCulled=!1,a.add(f,p,i)});let f=1024,p=new Uint8Array(f*f),m=new Fa(p,f,f,te,b);m.minFilter=m.magFilter=h,m.unpackAlignment=1;let g=e.uniforms.uExtentMin.value,_=1/e.uniforms.uExtentInv.value;return e.uniforms.tShade.value=m,{group:a,trees:r,cast(e){p.fill(0);let t=Math.hypot(e.x,e.z)||.001,a=e.x/t,o=e.z/t,s=t/Math.max(e.y,.12),c=_/f;for(let[e,t,l,u]of r){if(l>3&&l<i)continue;let r=n[l],d=r.crown*u*1.02,m=Math.hypot(d,r.height*.28*u*s),h=e-a*r.crownY*u*s,_=t-o*r.crownY*u*s,v=m+2,y=Math.max(0,Math.floor((h-v-g.x)/c)),b=Math.min(1023,Math.ceil((h+v-g.x)/c)),x=Math.max(0,Math.floor((_-v-g.y)/c)),S=Math.min(1023,Math.ceil((_+v-g.y)/c));for(let n=x;n<=S;n++)for(let r=y;r<=b;r++){let i=g.x+(r+.5)*c-h,s=g.y+(n+.5)*c-_,l=(i*a+s*o)/m,u=(-i*o+s*a)/d,v=Math.hypot(l,u);if(v>=1.08)continue;let y=Math.atan2(u,l),b=.9+.09*Math.sin(y*5+e)+.06*Math.sin(y*9+t),x=255*(1-hn.smoothstep(v,b-.16,b+.04)),S=r+n*f;x>p[S]&&(p[S]=x)}}m.needsUpdate=!0}}}var jb=`
out vec3 vWorld;
void main() {
  vec4 w = modelMatrix * vec4(position, 1.0);
  vWorld = w.xyz;
  gl_Position = projectionMatrix * viewMatrix * w;
}
`,Mb=`
${ng}
precision highp samplerCube;
layout(location = 0) out vec4 outC;
in vec3 vWorld;
uniform vec3 uZenith, uMid, uHorizon, uGlow;
uniform samplerCube tClouds;
uniform vec4 uTarn;   // centre x, z, and its half-lengths across and along the valley
uniform vec2 uAlong;  // the way the valley runs

vec3 skyAt(vec3 d) {
  float h = max(d.y, 0.0);
  vec3 col = mix(uHorizon, uMid, smoothstep(0.0, 0.3, pow(h, 0.8)));
  col = mix(col, uZenith, smoothstep(0.18, 0.85, h));
  vec4 c = texture(tClouds, d);
  return col * (1.0 - c.a) + c.rgb;
}

void main() {
  vec2 xz = vWorld.xz;
  float depth = vWorld.y - groundAt(xz);
  if (depth <= 0.0) discard;
  // only the tarn: low ground beyond its banks stays dry
  vec2 off = xz - uTarn.xy;
  if (length(vec2(dot(off, vec2(-uAlong.y, uAlong.x)) / uTarn.z, dot(off, uAlong) / uTarn.w)) > 1.32) discard;
  vec3 V = normalize(uCamPos - vWorld);
  float dist = distance(uCamPos, vWorld);
  // the wind's small ripples, travelling; calmer in the lee of the bank
  float U = uWind.z * (0.35 + 0.65 * gustAt(xz));
  vec2 flow = uWind.xy * uTime;
  vec2 r1 = texture(tNoiseHi, (xz - flow * 0.22) / 7.3).rg - 0.5;
  vec2 r2 = texture(tNoiseHi, (xz.yx + flow * 0.31) / 2.1 + 0.4).ba - 0.5;
  vec2 rip = (r1 + r2 * 0.6) * 0.035 * U * smoothstep(0.0, 0.5, depth) / (1.0 + dist * 0.012);
  vec3 N = normalize(vec3(rip.x, 1.0, rip.y));
  vec3 R = reflect(-V, N);
  R.y = abs(R.y);
  // (it holds more of the sky's blue than a true mirror seen this low would: that is how it is painted)
  vec3 mirror = mix(skyAt(normalize(R + vec3(0.0, 0.3, 0.0))), uMid * 1.05, 0.5);
  // what stands low on the far side is mirrored too: the bank, the hills
  float shore = 1.0 - smoothstep(0.015, 0.11, R.y + r1.x * 0.03);
  mirror = mix(mirror, vec3(0.03, 0.115, 0.05) * (uSkyColor + uSunColor * 0.25), shore * 0.9);
  float fres = 0.03 + 0.97 * pow(1.0 - max(dot(N, V), 0.0), 5.0);
  float shade = cloudAt(xz);
  vec3 bed = mix(vec3(0.1, 0.19, 0.11), vec3(0.01, 0.06, 0.065), smoothstep(0.0, 1.5, depth)) * (uSkyColor + uSunColor * 0.3 * shade);
  vec3 col = mix(bed, mirror, clamp(fres * 1.15 + 0.16, 0.0, 1.0));
  // the sun's path on it, in sparks
  float glint = pow(max(dot(R, uSunDir), 0.0), 420.0) * shade;
  col += uSunColor * glint * 2.5;
  // a dark line where the bank meets the water
  col = mix(vec3(0.045, 0.12, 0.075) * (uSkyColor + uSunColor * 0.25), col, smoothstep(0.04, 0.3, depth));
  float alpha = smoothstep(0.0, 0.1, depth);
  outC = vec4(applyFog(col, vWorld), alpha);
}
`;function Nb(e,t){let n=Math.max(d_.rx,d_.rz)*1.5,r=new jc(n*2,n*2,1,1);r.rotateX(-Math.PI/2);let i=t.uniforms,a=new xa(r,new Qc({glslVersion:yt,vertexShader:jb,fragmentShader:Mb,uniforms:{...e.uniforms,uZenith:i.uZenith,uMid:i.uMid,uHorizon:i.uHorizon,uGlow:i.uGlow,tClouds:i.tClouds,uTarn:{value:new In(d_.at[0],d_.at[1],d_.rx,d_.rz)},uAlong:{value:new K(a_[0],a_[1])}},transparent:!0,depthWrite:!1}));return a.position.set(d_.at[0],d_.level,d_.at[1]),a.renderOrder=5,a}var Pb=new URLSearchParams(location.search),Fb={},Ib=performance.now(),Lb=e=>{let t=performance.now();Fb[e]=Math.round(t-Ib),Ib=t},Rb=Pb.has(`step`),zb=e=>document.getElementById(e),Bb=!1,Vb=!0,Hb=n({onVisible(e){Vb=e,Bb&&qx()}}),Ub=async(e,t)=>{Hb.progress(e,t),zb(`loading`).firstElementChild.textContent=t,await new Promise(e=>setTimeout(e))},Wb=(e,t)=>new Promise(n=>{let r=null,i=!1,a=()=>{i||(i=!0,r?.terminate(),Promise.resolve(t()).then(n))};try{r=new Worker(new URL(new URL(`bake.worker.js`,import.meta.url).href,``+import.meta.url),{type:`module`}),r.onmessage=e=>{i||(i=!0,r.terminate(),n(e.data))},r.onerror=a,r.postMessage(e)}catch{a()}}),Gb=await K_()??null;if(!Gb)throw zb(`loading`).firstElementChild.textContent=`This garden link has already been used, or the garden was closed. Run the command again for a new link.`,Error(`no snapshot`);function Kb(){if(Pb.has(`nowebgl`))return!1;try{let e=document.createElement(`canvas`).getContext(`webgl2`);return e?.getExtension(`WEBGL_lose_context`)?.loseContext(),!!e}catch{return!1}}var qb=Pb.get(`view`)===`read`?`chosen`:Kb()?null:`no-webgl`;qb&&(Ev({snapshot:Gb,fetchSnapshot:X_,fetchReading:q_,reason:qb}),Hb.ready(),await new Promise(()=>{}));var Jb=oy(Gb),Yb={length:Gb.liveRevision?Math.ceil((Jb+16)/32)*32:Jb};b_(Yb);var Xb=Pb.get(`quality`)??`high`,Zb=1280,Qb={extent:Zb,extentRes:Zb,centre:[0,0]},$b=Xb===`low`?1024:1536,ex=Pb.has(`hour`)?Number(Pb.get(`hour`)):19.2,tx=Wb({job:`ground`,world:Yb,...Qb},()=>r_({heightAt:I_,paint:L_,...Qb})),nx=Wb({job:`floor`,world:Yb,size:$b},()=>({pixels:Ny($b)}));await Ub(.04,`Painting the sky`);var rx=gb(),ix=Pb.has(`nosound`)?null:rx.load(),ax=zb(`view`),ox;try{ox=new $h({canvas:ax,antialias:!1,powerPreference:`high-performance`,preserveDrawingBuffer:Rb})}catch{throw zb(`loading`).firstElementChild.textContent=`The garden needs WebGL 2, which this browser does not offer.`,Error(`no WebGL 2`)}ox.setPixelRatio(Math.min(window.devicePixelRatio||1,Pb.has(`dpr`)?Number(Pb.get(`dpr`)):1.5)),ox.setSize(window.innerWidth,window.innerHeight);var sx=new Er,cx=new fu(68,window.innerWidth/window.innerHeight,.12,4e4);cx.rotation.order=`YXZ`;function lx(){ox.setSize(window.innerWidth,window.innerHeight),cx.aspect=window.innerWidth/window.innerHeight,cx.updateProjectionMatrix()}window.addEventListener(`resize`,lx);var ux=Pg(ox,`./textures/`),dx=cb(ox,{cloudSize:Xb===`low`?512:Xb===`medium`?1024:2048,cloudSteps:Xb===`low`?150:240,liveClouds:Xb!==`low`,backdropWidth:Xb===`high`?12288:4096}),fx=(async()=>{let e=performance.now(),t=t=>{Fb[t]=Math.round(performance.now()-e)};await dx.clouds.compile(),await dx.backdrop.compile(),t(`skyShadersAt`),dx.backdrop.paint(),await Vx(),t(`mountainsAt`),dx.light(ex),await Vx(),t(`cloudsAt`)})();Lb(`skyBegun`),await Ub(.12,`Laying out the valley`);var px=await tx;Lb(`groundWait`);var mx=i_({heightAt:I_,paint:L_,...Qb,size:256,baked:px,textures:await ux,density:Xb===`low`?.45:Xb===`medium`?.7:1,height:.5,seed:3});sx.add(mx.group),Lb(`field`),mx.uniforms.tFloor.value=Py(ox,$b,(await nx).pixels),Lb(`floorPaint`),mx.uniforms.uFloor.value.set(a_[0],a_[1],Cy.u0,Cy.v0),mx.uniforms.uFloorInv.value=1/Cy.size,mx.uniforms.tStrokes.value=vb(ox),Lb(`strokes`);{let e=new Uint8Array(524288),t=Zb/512,n=e=>Math.round(255*Math.min(1,Math.max(0,.5+e*2)));for(let r=0,i=0;r<512;r++){let a=-640+(r+.5)*t;for(let r=0;r<512;r++){let o=-640+(r+.5)*t;e[i++]=n((mx.heightAt(o+24,a)-mx.heightAt(o-24,a))/48),e[i++]=n((mx.heightAt(o,a+24)-mx.heightAt(o,a-24))/48)}}let r=new Fa(e,512,512,re,b);r.minFilter=r.magFilter=h,r.unpackAlignment=1,r.needsUpdate=!0,mx.uniforms.tSlope.value=r,mx.uniforms.uHeroEye.value.set(l_[0],mx.heightAt(l_[0],l_[1])+1.68,l_[1])}Lb(`floor`),await Ub(.3,`Building the village`);var hx=n_(ox,mx,{samples:Xb===`low`?0:4,bloom:.1});Lb(`frame`);var gx=lb(ox,mx,{parts:dx});Lb(`sky`);var _x=Ab(mx,{cards:Xb===`low`?.6:Xb===`medium`?.8:1});Lb(`trees`),sx.add(gx.mesh,zy(mx,Zb/2),_x.group,Nb(mx,gx)),hx.paint=Pb.has(`paint`)?Number(Pb.get(`paint`)):Xb===`low`?0:Xb===`medium`?2:3;var vx=hx.paint||3;Lb(`build`),await Ub(.45,`Planting trees`);var yx=yy(mx);sx.add(yx.group),Xb!==`low`&&Pb.get(`seeds`)!==`0`&&sx.add(Sy(mx,Xb===`medium`?200:320));var bx=Pb.has(`drift`)?Number(Pb.get(`drift`)):0,xx=[{hour:12.5,name:`Noon`},{hour:15.5,name:`Afternoon`},{hour:19,name:`Golden hour`},{hour:20.3,name:`Sunset`}],Sx=(rb-15.5)/1200,Cx=0,wx=null,Tx=0,Ex=1.68,Dx=Zb/2-110,Ox=4.2,kx=6.3,$={x:l_[0],z:l_[1],y:0,vy:0,vx:0,vz:0,yaw:Math.atan2(-a_[0],-a_[1]),pitch:.04,onGround:!0,walked:0,stride:0,foot:1,bob:0,dip:0};$.y=mx.heightAt($.x,$.z);var Ax=new Set;function jx(e){let t=-Math.sin($.yaw),n=-Math.cos($.yaw),r=-n,i=t,a=0,o=0;(Ax.has(`KeyW`)||Ax.has(`ArrowUp`))&&(o+=1),(Ax.has(`KeyS`)||Ax.has(`ArrowDown`))&&--o,(Ax.has(`KeyD`)||Ax.has(`ArrowRight`))&&(a+=1),(Ax.has(`KeyA`)||Ax.has(`ArrowLeft`))&&--a;let s=Math.hypot(a,o)||1,c=Ax.has(`ShiftLeft`)||Ax.has(`ShiftRight`)?kx:Ox,l=(t*o+r*a)/s*c,u=(n*o+i*a)/s*c,d=$.onGround?9:2;$.vx+=(l-$.vx)*Math.min(1,e*d),$.vz+=(u-$.vz)*Math.min(1,e*d);let f=$.x,p=$.z;$.x+=$.vx*e,$.z+=$.vz*e,A_($.x,$.z)<.88&&A_($.x,$.z)<A_(f,p)&&($.x=f,$.z=p,$.vx*=.5,$.vz*=.5);for(let[e,t,n]of yx.solid){let r=$.x-e,i=$.z-t,a=Math.hypot(r,i);a<n&&a>1e-4&&($.x=e+r/a*n,$.z=t+i/a*n)}for(let[e,t,n,r]of _x.trees){if(n>3)break;let i=(n===0?1.25:.6)*r,a=$.x-e,o=$.z-t,s=Math.hypot(a,o);s<i&&s>1e-4&&($.x=e+a/s*i,$.z=t+o/s*i)}let m=Math.hypot($.x,$.z);m>Dx&&($.x*=Dx/m,$.z*=Dx/m);let h=mx.heightAt($.x,$.z);$.onGround?($.y=h,Ax.has(`Space`)&&($.vy=4.4,$.onGround=!1)):($.vy-=12.5*e,$.y+=$.vy*e,$.y<=h&&(mx.bend($.x,$.z,.55,0,0,Math.min(1,-$.vy/5)),rx.step(.7),$.dip=Math.min(.1,-$.vy*.016),$.y=h,$.vy=0,$.onGround=!0)),Mx(e,!0)}function Mx(e,t){let n=Math.hypot($.vx,$.vz),r=n*e;if($.onGround&&r>0){if($.walked+=r,$.stride+=r,$.stride>.16&&n>.05){$.stride=0;let e=$.foot*.12;mx.bend($.x-$.vz/n*e,$.z+$.vx/n*e,.27,$.vx,$.vz,.8)}$.walked>(n>5?1.2:.95)&&($.walked=0,$.foot=-$.foot,t&&rx.step(n>5?.42:.28)),$.bob+=Math.PI/.95*r}$.dip*=Math.exp(-e*9),mx.setPusher(0,$.x,$.y+.2,$.z,.17)}function Nx(e,t,n,r=-.06){Object.assign($,{x:e,z:t,y:mx.heightAt(e,t),yaw:n,pitch:r,vx:0,vz:0,vy:0,onGround:!0})}function Px(){let e=window.__tg?.view;if(e){cx.position.set(e.x,mx.heightAt(e.x,e.z)+(e.y??Ex),e.z),cx.rotation.set(e.pitch??0,e.yaw??0,0),e.fov&&e.fov!==cx.fov&&(cx.fov=e.fov,cx.updateProjectionMatrix()),cx.updateMatrixWorld();return}let t=Math.min(1,Math.hypot($.vx,$.vz)/kx),n=$.onGround?Math.sin($.bob)*.022*t:0;cx.position.set($.x,$.y+Ex+n-$.dip,$.z),cx.rotation.set($.pitch,$.yaw,0),cx.updateMatrixWorld()}var Fx={angle:Math.atan2(a_[1],a_[0])+.9,speed:4.5};function Ix(e){gx.setHour(Math.min(rb,Math.max(12,e))),_x.cast(gx.sunDir),Cx=gx.hour}var Lx=0;function Rx(e){let t=zb(`note`);t.textContent=e,t.classList.add(`on`),clearTimeout(Lx),Lx=setTimeout(()=>t.classList.remove(`on`),1400)}function zx(e){let t=xx[(e+xx.length)%xx.length];bx=0,Ix(t.hour),Rx(t.name)}function Bx(e){Fx.speed=Math.min(12,Math.max(0,e)),mx.setWind(Fx.angle,Fx.speed)}ax.addEventListener(`click`,()=>{rx.start(),document.pointerLockElement!==ax&&ax.requestPointerLock()?.catch?.(()=>{})}),document.addEventListener(`pointerlockchange`,()=>{let e=document.pointerLockElement===ax;document.body.classList.toggle(`playing`,e),e?rx.start():Ax.clear()}),document.addEventListener(`mousemove`,e=>{document.pointerLockElement===ax&&($.yaw-=e.movementX*.0021,$.pitch=Math.min(1.45,Math.max(-1.45,$.pitch-e.movementY*.0021)))}),document.addEventListener(`keydown`,e=>{if(!(e.target instanceof HTMLInputElement)){Ax.add(e.code),e.code===`KeyT`&&zx(Math.max(0,xx.findIndex(e=>e.hour>gx.hour+.05)));for(let t=0;t<xx.length;t++)e.code===`Digit${t+1}`&&zx(t);e.code===`Digit0`&&(bx=1,Rx(`Time passing`)),e.code===`Comma`&&Ix(gx.hour-.25),e.code===`Period`&&Ix(gx.hour+.25),e.code===`KeyP`&&(hx.paint=hx.paint?0:vx),e.code===`Space`&&e.preventDefault()}}),document.addEventListener(`keyup`,e=>Ax.delete(e.code)),window.addEventListener(`blur`,()=>Ax.clear());function Vx(){let e=ox.getContext(),t=e.fenceSync(e.SYNC_GPU_COMMANDS_COMPLETE,0);return e.flush(),new Promise(n=>{let r=()=>{if(e.clientWaitSync(t,0,0)===e.TIMEOUT_EXPIRED)return void setTimeout(r,2);e.deleteSync(t),n()};r()})}function Hx(e){wx?.typing()&&Ax.clear(),jx(e),Px(),mx.update(e,cx),hx.update(e,cx),Tx+=e,bx>0&&gx.hour<20.4&&!window.__tg?.view&&(gx.setHour(Math.min(rb,gx.hour+e*bx*Sx),!0),Math.abs(gx.hour-Cx)>.04&&(_x.cast(gx.sunDir),Cx=gx.hour));let t=0;for(let[e,n,r,i]of _x.trees){if(r>3)break;let a=(r===0?11.5:5.6)*i,o=Math.hypot($.x-e,$.z-n);t=Math.max(t,1-Math.min(1,Math.max(0,(o-a*.8)/(a*2.2))))}let n=1-Math.min(1,Math.max(0,(A_($.x,$.z)-1)/1.6)),r=.5+.5*Math.sin(Tx*.23)*Math.sin(Tx*.37+1);rx.update(e,{wind:Fx.speed,gust:r,tree:t,tarn:n,dusk:Math.min(1,Math.max(0,(gx.hour-18.8)/1.4))}),gx.update(e,cx),wx?.update(e,cx),hx.exposure=gx.exposure,hx.render(sx,cx)}Lb(`land`),await Ub(.48,`Planting the garden`);var Ux=await Z_(),Wx=e=>fy({snapshot:Gb,field:mx,scene:sx,canvas:ax,goTo:Nx,media:Ux,chime:()=>rx.chime(),kinds:e});wx=Wx();{let e=zb(`soundBtn`),t=()=>{e.setAttribute(`aria-pressed`,String(!rx.muted)),e.setAttribute(`aria-label`,rx.muted?`Sound is off`:`Sound is on`),e.classList.toggle(`off`,rx.muted)};rx.muted=!0;try{rx.muted=localStorage.getItem(`garden-sound`)!==`on`}catch{}t();let n=()=>{rx.start(),rx.muted=!rx.muted;try{localStorage.setItem(`garden-sound`,rx.muted?`off`:`on`)}catch{}t()};e.addEventListener(`click`,n),document.addEventListener(`keydown`,e=>{e.code===`KeyM`&&!(e.target instanceof HTMLInputElement||e.target instanceof HTMLSelectElement)&&n()})}Lb(`garden`),await Ub(.5,`Painting the mountains`),await fx,Lb(`skyWait`),await Ub(.64,`Painting the clouds`),Ix(ex),Bx(Pb.has(`wind`)?Number(Pb.get(`wind`)):4.5),Px(),mx.update(0,cx),gx.update(0,cx),await Vx(),Lb(`clouds`),await Ub(.78,`Mixing paint`),await Promise.all([hx.compile(sx,cx),ix]),Lb(`sceneShaders`),await Ub(.9,`Warming up`);{let[e,t]=[$.x,$.z];Ax.add(`KeyW`);for(let e=0;e<44;e++)Hx(1/30),await Vx(),e%11==10&&await Ub(.9+e/44*.08,`Warming up`);Ax.clear(),Object.assign($,{x:e,z:t,vx:0,vz:0,walked:0,stride:0,bob:0,yaw:Math.atan2(-a_[0],-a_[1]),pitch:.04}),mx.reset(),Tx=0,Ix(ex),Hx(1/60),await Vx()}if(Lb(`warm`),Gb.liveRevision)try{let e=JSON.parse(sessionStorage.getItem(`garden-walk`)??`null`);sessionStorage.removeItem(`garden-walk`),e?.length===4&&e.every(Number.isFinite)&&Nx(...e)}catch{}window.__tg={garden:wx,goTo:Nx,sounds:rx,pickTime:zx,timings:Fb,view:null,wind:Fx,THREE:Ff,renderer:ox,scene:sx,camera:cx,field:mx,frame:hx,sky:gx,trees:_x,stones:yx,player:$,keys:Ax,tick:Hx,setHour:Ix,setWind:Bx,fromValley:f_,ready:!0};var Gx=0,Kx=!1;function qx(){let e=Vb&&!Rb;e!==Kx&&(Kx=e,Gx=performance.now(),ox.setAnimationLoop(e?e=>{let t=Math.min(.05,Math.max(0,(e-Gx)/1e3));Gx=e,Hx(t)}:null))}lx(),Bb=!0,qx(),zb(`loading`).remove(),document.body.classList.add(`ready`),Hb.ready(),window.__block={ready:!0,block:Hb,timings:Fb};async function Jx(){try{if(document.hidden||!wx.canRefresh())return;let e=await X_(Gb.liveRevision);if(!e||e.unchanged||!wx.canRefresh())return;if(oy(e)>Yb.length){if(document.pointerLockElement)return;try{sessionStorage.setItem(`garden-walk`,JSON.stringify([$.x,$.z,$.yaw,$.pitch]))}catch{}location.reload();return}let t=e.liveMediaRevision===Gb.liveMediaRevision?Ux:await Z_();if(!wx.canRefresh())return;let n=wx.selectedKinds();wx.dispose(),Gb=e,Ux=t,wx=Wx(n),window.__tg.garden=wx}catch{}finally{setTimeout(Jx,5e3)}}Gb.liveRevision&&setTimeout(Jx,5e3);