import{d as I,w as R,r as S,a as C,o as _,c as d,b as F,v as E,u as k,e as l,i as B,p as g,f as v,g as M,h as b,j as N,k as W,S as O,F as U,l as V}from"./vendor.56ee30fe.js";const H=function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const n of document.querySelectorAll('link[rel="modulepreload"]'))o(n);new MutationObserver(n=>{for(const s of n)if(s.type==="childList")for(const a of s.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&o(a)}).observe(document,{childList:!0,subtree:!0});function r(n){const s={};return n.integrity&&(s.integrity=n.integrity),n.referrerpolicy&&(s.referrerPolicy=n.referrerpolicy),n.crossorigin==="use-credentials"?s.credentials="include":n.crossorigin==="anonymous"?s.credentials="omit":s.credentials="same-origin",s}function o(n){if(n.ep)return;n.ep=!0;const s=r(n);fetch(n.href,s)}};H();let i,w=0,f=null;function p(){return(f===null||f.buffer!==i.memory.buffer)&&(f=new Uint8Array(i.memory.buffer)),f}let m=new TextEncoder("utf-8");const P=typeof m.encodeInto=="function"?function(e,t){return m.encodeInto(e,t)}:function(e,t){const r=m.encode(e);return t.set(r),{read:e.length,written:r.length}};function D(e,t,r){if(r===void 0){const c=m.encode(e),u=t(c.length);return p().subarray(u,u+c.length).set(c),w=c.length,u}let o=e.length,n=t(o);const s=p();let a=0;for(;a<o;a++){const c=e.charCodeAt(a);if(c>127)break;s[n+a]=c}if(a!==o){a!==0&&(e=e.slice(a)),n=r(n,o,o=a+e.length*3);const c=p().subarray(n+a,n+o);a+=P(e,c).written}return w=a,n}let h=null;function L(){return(h===null||h.buffer!==i.memory.buffer)&&(h=new Int32Array(i.memory.buffer)),h}let $=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});$.decode();function q(e,t){return $.decode(p().subarray(e,e+t))}function j(e){try{const s=i.__wbindgen_add_to_stack_pointer(-16);var t=D(e,i.__wbindgen_malloc,i.__wbindgen_realloc),r=w;i.baseCompile(s,t,r);var o=L()[s/4+0],n=L()[s/4+1];return q(o,n)}finally{i.__wbindgen_add_to_stack_pointer(16),i.__wbindgen_free(o,n)}}async function K(e,t){if(typeof Response=="function"&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(e,t)}catch(o){if(e.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",o);else throw o}const r=await e.arrayBuffer();return await WebAssembly.instantiate(r,t)}else{const r=await WebAssembly.instantiate(e,t);return r instanceof WebAssembly.Instance?{instance:r,module:e}:r}}async function T(e){typeof e=="undefined"&&(e=new URL("/vue-compiler/assets/vue_compiler_wasm_bg.40f40082.wasm",self.location));const t={};(typeof e=="string"||typeof Request=="function"&&e instanceof Request||typeof URL=="function"&&e instanceof URL)&&(e=fetch(e));const{instance:r,module:o}=await K(await e,t);return i=r.exports,T.__wbindgen_wasm_module=o,i}var y=(e,t)=>{for(const[r,o]of t)e[r]=o;return e};const z={class:"playground"},G=["innerHTML"],J=I({async setup(e){let t,r;[t,r]=R(()=>T()),await t,r();let o=S("<p>Hello World from wasm!</p>"),n=S("");return C(async()=>{n.value=j(o.value)}),(s,a)=>(_(),d("div",z,[F(l("textarea",{"onUpdate:modelValue":a[0]||(a[0]=c=>B(o)?o.value=c:o=c)},null,512),[[E,k(o)]]),l("code",{innerHTML:k(n)},null,8,G)]))}});var Q=y(J,[["__scopeId","data-v-1eb71834"]]),X="/vue-compiler/assets/wasm-ferris.eb749a19.png",Y="/vue-compiler/assets/logo.03d6d6da.png";const Z={},x=e=>(g("data-v-921708e0"),e=e(),v(),e),ee={class:"demo-header"},te=x(()=>l("h1",{class:"title"},"Rusty Vue Playground",-1)),ne=x(()=>l("img",{class:"crab",alt:"Ferris with WA hat",src:X},null,-1)),oe=M(" \u2764\uFE0F "),re=x(()=>l("img",{class:"vue",alt:"Vue logo",src:Y},null,-1)),se=[te,ne,oe,re];function ae(e,t){return _(),d("header",ee,se)}var ce=y(Z,[["render",ae],["__scopeId","data-v-921708e0"]]);const ie={},A=e=>(g("data-v-66ab9a06"),e=e(),v(),e),le=A(()=>l("a",{href:"https://github.com/HerringtonDarkholme/vue-compiler"}," Source Code ",-1)),_e=A(()=>l("a",{href:"https://herringtondarkholme.github.io/vue-compiler/dev/bench/"}," Benchmark ",-1)),ue=A(()=>l("span",null,[l("a",{href:"https://rustwasm.github.io/wasm-pack/"},"wasm-pack"),M(" for tooling and logo. ")],-1)),de=[le,_e,ue];function fe(e,t){return _(),d("footer",null,de)}var pe=y(ie,[["render",fe],["__scopeId","data-v-66ab9a06"]]);const me=e=>(g("data-v-2e3dc787"),e=e(),v(),e),he=me(()=>l("h1",null," Loading WASM... ",-1)),ye=I({setup(e){return(t,r)=>(_(),d(U,null,[b(ce),l("main",null,[(_(),N(O,null,{fallback:W(()=>[he]),default:W(()=>[b(Q)]),_:1}))]),b(pe)],64))}});var ge=y(ye,[["__scopeId","data-v-2e3dc787"]]);V(ge).mount("#app");
