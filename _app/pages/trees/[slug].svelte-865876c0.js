import{S as s,i as a,s as e,e as r,t as o,k as t,c as n,a as c,g as l,d as u,n as i,f,G as p,M as d,l as m}from"../../chunks/vendor-491fc14c.js";import{v as h}from"../../chunks/variables-fc0f151f.js";function g(s){let a,e,h,g,E=v&&function(s){let a,e,m,h,g,E,k,x,P,j,w,D=v.id+"",T=v.name+"",$=v.genus+"";return{c(){a=r("p"),e=o("id: "),m=o(D),h=t(),g=r("p"),E=o("name: "),k=o(T),x=t(),P=r("p"),j=o("genus: "),w=o($)},l(s){a=n(s,"P",{});var r=c(a);e=l(r,"id: "),m=l(r,D),r.forEach(u),h=i(s),g=n(s,"P",{});var o=c(g);E=l(o,"name: "),k=l(o,T),o.forEach(u),x=i(s),P=n(s,"P",{});var t=c(P);j=l(t,"genus: "),w=l(t,$),t.forEach(u)},m(s,r){f(s,a,r),p(a,e),p(a,m),f(s,h,r),f(s,g,r),p(g,E),p(g,k),f(s,x,r),f(s,P,r),p(P,j),p(P,w)},p:d,d(s){s&&u(a),s&&u(h),s&&u(g),s&&u(x),s&&u(P)}}}();return{c(){a=r("h1"),e=o("Tree Details"),h=t(),E&&E.c(),g=m()},l(s){a=n(s,"H1",{});var r=c(a);e=l(r,"Tree Details"),r.forEach(u),h=i(s),E&&E.l(s),g=m()},m(s,r){f(s,a,r),p(a,e),f(s,h,r),E&&E.m(s,r),f(s,g,r)},p(s,[a]){v&&E.p(s,a)},i:d,o:d,d(s){s&&u(a),s&&u(h),E&&E.d(s),s&&u(g)}}}let v;async function E({page:s,fetch:a,session:e,context:r}){const o=h.apiPath+`/trees/${s.params.slug}`;console.log("url",o);const t=await a(o);return console.log("res",t),t.ok?(v=await t.json(),console.log("current_tree",v),{props:{tree:v}}):{status:t.status,error:new Error(`Could not load ${o}`)}}export default class extends s{constructor(s){super(),a(this,s,null,g,e,{})}}export{E as load};