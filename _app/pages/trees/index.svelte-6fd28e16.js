import{S as e,i as t,s,l as n,f as r,e as a,t as o,k as l,c,a as f,g as h,n as u,d as i,b as g,G as m,h as d,M as p,O as $,z as w,j as v,m as j,o as x,v as E,r as L,w as k}from"../../chunks/vendor-491fc14c.js";function N(e,t,s){const n=e.slice();return n[1]=t[s],n}function T(e){let t,s,n,p,$,w,v,j=e[1].name+"",x=e[1].genus+"";return{c(){t=a("li"),s=a("a"),n=o(j),p=o(",  "),$=o(x),w=l(),this.h()},l(e){t=c(e,"LI",{});var r=f(t);s=c(r,"A",{href:!0});var a=f(s);n=h(a,j),p=h(a,",  "),$=h(a,x),w=u(a),a.forEach(i),r.forEach(i),this.h()},h(){g(s,"href",v="/trees/new"+e[1].id)},m(e,a){r(e,t,a),m(t,s),m(s,n),m(s,p),m(s,$),m(s,w)},p(e,t){1&t&&j!==(j=e[1].name+"")&&d(n,j),1&t&&x!==(x=e[1].genus+"")&&d($,x),1&t&&v!==(v="/trees/new"+e[1].id)&&g(s,"href",v)},d(e){e&&i(t)}}}function b(e){let t,s=e[1]&&T(e);return{c(){s&&s.c(),t=n()},l(e){s&&s.l(e),t=n()},m(e,n){s&&s.m(e,n),r(e,t,n)},p(e,n){e[1]?s?s.p(e,n):(s=T(e),s.c(),s.m(t.parentNode,t)):s&&(s.d(1),s=null)},d(e){s&&s.d(e),e&&i(t)}}}function y(e){let t,s=e[0],a=[];for(let n=0;n<s.length;n+=1)a[n]=b(N(e,s,n));return{c(){for(let e=0;e<a.length;e+=1)a[e].c();t=n()},l(e){for(let t=0;t<a.length;t+=1)a[t].l(e);t=n()},m(e,s){for(let t=0;t<a.length;t+=1)a[t].m(e,s);r(e,t,s)},p(e,[n]){if(1&n){let r;for(s=e[0],r=0;r<s.length;r+=1){const o=N(e,s,r);a[r]?a[r].p(o,n):(a[r]=b(o),a[r].c(),a[r].m(t.parentNode,t))}for(;r<a.length;r+=1)a[r].d(1);a.length=s.length}},i:p,o:p,d(e){$(a,e),e&&i(t)}}}function z(e,t,s){let n=[];return w((async()=>{const e=await fetch("http://localhost:5000/trees");console.log("returnValue",e);const t=await e.json();console.log("response",t),s(0,n=t)})),[n]}class A extends e{constructor(e){super(),t(this,e,z,y,s,{})}}function G(e){let t,s,n,g,d;return g=new A({}),{c(){t=a("h1"),s=o("Tree List"),n=l(),v(g.$$.fragment)},l(e){t=c(e,"H1",{});var r=f(t);s=h(r,"Tree List"),r.forEach(i),n=u(e),j(g.$$.fragment,e)},m(e,a){r(e,t,a),m(t,s),r(e,n,a),x(g,e,a),d=!0},p:p,i(e){d||(E(g.$$.fragment,e),d=!0)},o(e){L(g.$$.fragment,e),d=!1},d(e){e&&i(t),e&&i(n),k(g,e)}}}export default class extends e{constructor(e){super(),t(this,e,null,G,s,{})}}