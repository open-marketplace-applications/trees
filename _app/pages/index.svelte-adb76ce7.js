import{S as a,i as s,s as e,e as t,c as o,a as r,d as n,b as c,f as l,G as i,M as p,z as f,j as u,m as h,o as m,v as d,r as g,w as b}from"../chunks/vendor-491fc14c.js";import{_ as w}from"../chunks/preload-helper-9f12a5fd.js";import{a as $}from"../chunks/areas-a1feba8e.js";import{v}from"../chunks/variables-fc0f151f.js";function j(a){let s,e;return{c(){s=t("main"),e=t("div"),this.h()},l(a){s=o(a,"MAIN",{class:!0});var t=r(s);e=o(t,"DIV",{id:!0,class:!0}),r(e).forEach(n),t.forEach(n),this.h()},h(){c(e,"id","map"),c(e,"class","svelte-1b3xabu"),c(s,"class","svelte-1b3xabu")},m(a,t){l(a,s,t),i(s,e)},p:p,i:p,o:p,d(a){a&&n(s)}}}function k(a){let s=[];return f((async()=>{{const a=await fetch(v.apiPath+"/trees");console.log("returnValue",a);const e=await a.json();console.log("response",e),s=e;const t=await w((()=>import("../chunks/leaflet-src-353024f8.js").then((function(a){return a.l}))),["/_app/chunks/leaflet-src-353024f8.js","/_app/chunks/vendor-491fc14c.js","/_app/assets/vendor-d28434f8.css"]);let o=[49.291676,7.373426];const r=t.map("map").setView(o,21);t.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",{attribution:'© <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(r),$.forEach((a=>{t.polygon(a.edges,{color:"blue",weight:1}).on("click",(function(a){console.info(a)})).addTo(r)})),s.forEach((a=>{console.info("tree",a),t.marker([parseFloat(a.lat),parseFloat(a.lng)]).addTo(r).bindPopup(`${a.name} - ${a.genus}`).openPopup()}))}})),[]}class E extends a{constructor(a){super(),s(this,a,k,j,e,{})}}function x(a){let s,e,c;return e=new E({}),{c(){s=t("main"),u(e.$$.fragment)},l(a){s=o(a,"MAIN",{});var t=r(s);h(e.$$.fragment,t),t.forEach(n)},m(a,t){l(a,s,t),m(e,s,null),c=!0},p:p,i(a){c||(d(e.$$.fragment,a),c=!0)},o(a){g(e.$$.fragment,a),c=!1},d(a){a&&n(s),b(e)}}}function _(a){return console.log("loaded??"),[]}export default class extends a{constructor(a){super(),s(this,a,_,x,e,{})}}
