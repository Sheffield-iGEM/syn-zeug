(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))r(o);new MutationObserver(o=>{for(const i of o)if(i.type==="childList")for(const a of i.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&r(a)}).observe(document,{childList:!0,subtree:!0});function n(o){const i={};return o.integrity&&(i.integrity=o.integrity),o.referrerpolicy&&(i.referrerPolicy=o.referrerpolicy),o.crossorigin==="use-credentials"?i.credentials="include":o.crossorigin==="anonymous"?i.credentials="omit":i.credentials="same-origin",i}function r(o){if(o.ep)return;o.ep=!0;const i=n(o);fetch(o.href,i)}})();function B(){}function tt(t){return t()}function Ge(){return Object.create(null)}function Z(t){t.forEach(tt)}function at(t){return typeof t=="function"}function nt(t,e){return t!=t?e==e:t!==e||t&&typeof t=="object"||typeof t=="function"}let _e;function lt(t,e){return _e||(_e=document.createElement("a")),_e.href=e,t===_e.href}function _t(t){return Object.keys(t).length===0}function ue(t){return t==null?"":t}function u(t,e){t.appendChild(e)}function rt(t,e,n){t.insertBefore(e,n||null)}function Se(t){t.parentNode.removeChild(t)}function ut(t,e){for(let n=0;n<t.length;n+=1)t[n]&&t[n].d(e)}function b(t){return document.createElement(t)}function ot(t){return document.createTextNode(t)}function O(){return ot(" ")}function z(t,e,n,r){return t.addEventListener(e,n,r),()=>t.removeEventListener(e,n,r)}function _(t,e,n){n==null?t.removeAttribute(e):t.getAttribute(e)!==n&&t.setAttribute(e,n)}function dt(t){return Array.from(t.childNodes)}function de(t,e){t.value=e==null?"":e}let qe;function X(t){qe=t}const Q=[],Ke=[],be=[],Qe=[],ft=Promise.resolve();let Ee=!1;function gt(){Ee||(Ee=!0,ft.then(it))}function $e(t){be.push(t)}const Ce=new Set;let fe=0;function it(){const t=qe;do{for(;fe<Q.length;){const e=Q[fe];fe++,X(e),bt(e.$$)}for(X(null),Q.length=0,fe=0;Ke.length;)Ke.pop()();for(let e=0;e<be.length;e+=1){const n=be[e];Ce.has(n)||(Ce.add(n),n())}be.length=0}while(Q.length);for(;Qe.length;)Qe.pop()();Ee=!1,Ce.clear(),X(t)}function bt(t){if(t.fragment!==null){t.update(),Z(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach($e)}}const vt=new Set;function pt(t,e){t&&t.i&&(vt.delete(t),t.i(e))}function wt(t,e,n,r){const{fragment:o,on_mount:i,on_destroy:a,after_update:c}=t.$$;o&&o.m(e,n),r||$e(()=>{const v=i.map(tt).filter(at);a?a.push(...v):Z(v),t.$$.on_mount=[]}),c.forEach($e)}function mt(t,e){const n=t.$$;n.fragment!==null&&(Z(n.on_destroy),n.fragment&&n.fragment.d(e),n.on_destroy=n.fragment=null,n.ctx=[])}function yt(t,e){t.$$.dirty[0]===-1&&(Q.push(t),gt(),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}function ht(t,e,n,r,o,i,a,c=[-1]){const v=qe;X(t);const d=t.$$={fragment:null,ctx:null,props:i,update:B,not_equal:o,bound:Ge(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(e.context||(v?v.$$.context:[])),callbacks:Ge(),dirty:c,skip_bound:!1,root:e.target||v.$$.root};a&&a(d.root);let k=!1;if(d.ctx=n?n(t,e.props||{},(p,T,...P)=>{const S=P.length?P[0]:T;return d.ctx&&o(d.ctx[p],d.ctx[p]=S)&&(!d.skip_bound&&d.bound[p]&&d.bound[p](S),k&&yt(t,p)),T}):[],d.update(),k=!0,Z(d.before_update),d.fragment=r?r(d.ctx):!1,e.target){if(e.hydrate){const p=dt(e.target);d.fragment&&d.fragment.l(p),p.forEach(Se)}else d.fragment&&d.fragment.c();e.intro&&pt(t.$$.fragment),wt(t,e.target,e.anchor,e.customElement),it()}X(v)}class kt{$destroy(){mt(this,1),this.$destroy=B}$on(e,n){const r=this.$$.callbacks[e]||(this.$$.callbacks[e]=[]);return r.push(n),()=>{const o=r.indexOf(n);o!==-1&&r.splice(o,1)}}$set(e){this.$$set&&!_t(e)&&(this.$$.skip_bound=!0,this.$$set(e),this.$$.skip_bound=!1)}}const J=[];function At(t,e=B){let n;const r=new Set;function o(c){if(nt(t,c)&&(t=c,n)){const v=!J.length;for(const d of r)d[1](),J.push(d,t);if(v){for(let d=0;d<J.length;d+=2)J[d][0](J[d+1]);J.length=0}}}function i(c){o(c(t))}function a(c,v=B){const d=[c,v];return r.add(d),r.size===1&&(n=e(o)||B),c(t),()=>{r.delete(d),r.size===0&&(n(),n=null)}}return{set:o,update:i,subscribe:a}}const Ot={message:"Default Toast Message",autohide:!0,timeout:5e3};function jt(){const{subscribe:t,set:e,update:n}=At([]);return{subscribe:t,trigger:r=>n(o=>{let i={...Ot,...r};return o.push(i),o}),close:()=>n(r=>(r.length>0&&r.shift(),r)),clear:()=>e([])}}const Xe=jt();let Ye;Xe.subscribe(t=>{clearTimeout(Ye),t.length&&(!t[0].autohide||(Ye=setTimeout(()=>{Xe.close()},t[0].timeout)))});let s;const st=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});st.decode();let ve=new Uint8Array;function pe(){return ve.byteLength===0&&(ve=new Uint8Array(s.memory.buffer)),ve}function V(t,e){return st.decode(pe().subarray(t,t+e))}const U=new Array(32).fill(void 0);U.push(void 0,null,!0,!1);let Y=U.length;function w(t){Y===U.length&&U.push(U.length+1);const e=Y;return Y=U[e],U[e]=t,e}function g(t){return U[t]}function Ct(t){t<36||(U[t]=Y,Y=t)}function h(t){const e=g(t);return Ct(t),e}function xe(t){return t==null}let we=new Float64Array;function xt(){return we.byteLength===0&&(we=new Float64Array(s.memory.buffer)),we}let me=new Int32Array;function l(){return me.byteLength===0&&(me=new Int32Array(s.memory.buffer)),me}let E=0;const ye=new TextEncoder("utf-8"),Et=typeof ye.encodeInto=="function"?function(t,e){return ye.encodeInto(t,e)}:function(t,e){const n=ye.encode(t);return e.set(n),{read:t.length,written:n.length}};function L(t,e,n){if(n===void 0){const c=ye.encode(t),v=e(c.length);return pe().subarray(v,v+c.length).set(c),E=c.length,v}let r=t.length,o=e(r);const i=pe();let a=0;for(;a<r;a++){const c=t.charCodeAt(a);if(c>127)break;i[o+a]=c}if(a!==r){a!==0&&(t=t.slice(a)),o=n(o,r,r=a+t.length*3);const c=pe().subarray(o+a,o+r);a+=Et(t,c).written}return E=a,o}function Le(t){const e=typeof t;if(e=="number"||e=="boolean"||t==null)return`${t}`;if(e=="string")return`"${t}"`;if(e=="symbol"){const o=t.description;return o==null?"Symbol":`Symbol(${o})`}if(e=="function"){const o=t.name;return typeof o=="string"&&o.length>0?`Function(${o})`:"Function"}if(Array.isArray(t)){const o=t.length;let i="[";o>0&&(i+=Le(t[0]));for(let a=1;a<o;a++)i+=", "+Le(t[a]);return i+="]",i}const n=/\[object ([^\]]+)\]/.exec(toString.call(t));let r;if(n.length>1)r=n[1];else return toString.call(t);if(r=="Object")try{return"Object("+JSON.stringify(t)+")"}catch{return"Object"}return t instanceof Error?`${t.name}: ${t.message}
${t.stack}`:r}let he=new Uint32Array;function $t(){return he.byteLength===0&&(he=new Uint32Array(s.memory.buffer)),he}function Lt(t,e){const r=$t().subarray(t/4,t/4+e),o=[];for(let i=0;i<r.length;i++)o.push(h(r[i]));return o}function ge(t,e){try{return t.apply(this,e)}catch(n){s.__wbindgen_exn_store(w(n))}}class j{static __wrap(e){const n=Object.create(j.prototype);return n.ptr=e,n}__destroy_into_raw(){const e=this.ptr;return this.ptr=0,e}free(){const e=this.__destroy_into_raw();s.__wbg_seq_free(e)}constructor(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16),a=L(e,s.__wbindgen_malloc,s.__wbindgen_realloc),c=E;s.seq_new(i,a,c);var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}static from_js(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16);s.seq_from_js(i,w(e));var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}static dna(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16),a=L(e,s.__wbindgen_malloc,s.__wbindgen_realloc),c=E;s.seq_dna(i,a,c);var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}static dna_n(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16),a=L(e,s.__wbindgen_malloc,s.__wbindgen_realloc),c=E;s.seq_dna_n(i,a,c);var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}static dna_iupac(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16),a=L(e,s.__wbindgen_malloc,s.__wbindgen_realloc),c=E;s.seq_dna_iupac(i,a,c);var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}static rna(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16),a=L(e,s.__wbindgen_malloc,s.__wbindgen_realloc),c=E;s.seq_rna(i,a,c);var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}static rna_n(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16),a=L(e,s.__wbindgen_malloc,s.__wbindgen_realloc),c=E;s.seq_rna_n(i,a,c);var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}static rna_iupac(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16),a=L(e,s.__wbindgen_malloc,s.__wbindgen_realloc),c=E;s.seq_rna_iupac(i,a,c);var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}static protein(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16),a=L(e,s.__wbindgen_malloc,s.__wbindgen_realloc),c=E;s.seq_protein(i,a,c);var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}static protein_iupac(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16),a=L(e,s.__wbindgen_malloc,s.__wbindgen_realloc),c=E;s.seq_protein_iupac(i,a,c);var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}kind(){try{const r=s.__wbindgen_add_to_stack_pointer(-16);s.seq_kind(r,this.ptr);var e=l()[r/4+0],n=l()[r/4+1];return V(e,n)}finally{s.__wbindgen_add_to_stack_pointer(16),s.__wbindgen_free(e,n)}}alphabet(){try{const r=s.__wbindgen_add_to_stack_pointer(-16);s.seq_alphabet(r,this.ptr);var e=l()[r/4+0],n=l()[r/4+1];return V(e,n)}finally{s.__wbindgen_add_to_stack_pointer(16),s.__wbindgen_free(e,n)}}len(){return s.seq_len(this.ptr)>>>0}is_empty(){return s.seq_is_empty(this.ptr)!==0}subseq(e,n){const r=s.seq_subseq(this.ptr,e,n);return j.__wrap(r)}rev(){const e=s.seq_rev(this.ptr);return j.__wrap(e)}reverse_complement(){try{const o=s.__wbindgen_add_to_stack_pointer(-16);s.seq_reverse_complement(o,this.ptr);var e=l()[o/4+0],n=l()[o/4+1],r=l()[o/4+2];if(r)throw h(n);return j.__wrap(e)}finally{s.__wbindgen_add_to_stack_pointer(16)}}normalize_case(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16);s.seq_normalize_case(i,this.ptr,w(e));var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}convert(e){try{const i=s.__wbindgen_add_to_stack_pointer(-16);s.seq_convert(i,this.ptr,w(e));var n=l()[i/4+0],r=l()[i/4+1],o=l()[i/4+2];if(o)throw h(r);return j.__wrap(n)}finally{s.__wbindgen_add_to_stack_pointer(16)}}count_elements(){try{const o=s.__wbindgen_add_to_stack_pointer(-16);s.seq_count_elements(o,this.ptr);var e=l()[o/4+0],n=l()[o/4+1],r=l()[o/4+2];if(r)throw h(n);return h(e)}finally{s.__wbindgen_add_to_stack_pointer(16)}}find_orfs(e){try{const c=s.__wbindgen_add_to_stack_pointer(-16);s.seq_find_orfs(c,this.ptr,e);var n=l()[c/4+0],r=l()[c/4+1],o=l()[c/4+2],i=l()[c/4+3];if(i)throw h(o);var a=Lt(n,r).slice();return s.__wbindgen_free(n,r*4),a}finally{s.__wbindgen_add_to_stack_pointer(16)}}to_string(){try{const r=s.__wbindgen_add_to_stack_pointer(-16);s.seq_to_string(r,this.ptr);var e=l()[r/4+0],n=l()[r/4+1];return V(e,n)}finally{s.__wbindgen_add_to_stack_pointer(16),s.__wbindgen_free(e,n)}}}async function St(t,e){if(typeof Response=="function"&&t instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(t,e)}catch(r){if(t.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",r);else throw r}const n=await t.arrayBuffer();return await WebAssembly.instantiate(n,e)}else{const n=await WebAssembly.instantiate(t,e);return n instanceof WebAssembly.Instance?{instance:n,module:t}:n}}function qt(){const t={};return t.wbg={},t.wbg.__wbindgen_string_new=function(e,n){const r=V(e,n);return w(r)},t.wbg.__wbindgen_is_object=function(e){const n=g(e);return typeof n=="object"&&n!==null},t.wbg.__wbg_get_093fe3cdafaf8976=function(e,n){const r=g(e)[h(n)];return w(r)},t.wbg.__wbindgen_is_string=function(e){return typeof g(e)=="string"},t.wbg.__wbg_entries_44c418612784cc9b=function(e){const n=Object.entries(g(e));return w(n)},t.wbg.__wbg_length_a73bfd4c96dd97ef=function(e){return g(e).length},t.wbg.__wbg_get_ad41fee29b7e0f53=function(e,n){const r=g(e)[n>>>0];return w(r)},t.wbg.__wbindgen_object_drop_ref=function(e){h(e)},t.wbg.__wbg_isArray_a1a8c3a8ac24bdf1=function(e){return Array.isArray(g(e))},t.wbg.__wbindgen_number_get=function(e,n){const r=g(n),o=typeof r=="number"?r:void 0;xt()[e/8+1]=xe(o)?0:o,l()[e/4+0]=!xe(o)},t.wbg.__wbg_isSafeInteger_f6dd91807e9c4d35=function(e){return Number.isSafeInteger(g(e))},t.wbg.__wbg_iterator_22ed2b976832ff0c=function(){return w(Symbol.iterator)},t.wbg.__wbg_get_72332cd2bc57924c=function(){return ge(function(e,n){const r=Reflect.get(g(e),g(n));return w(r)},arguments)},t.wbg.__wbindgen_is_function=function(e){return typeof g(e)=="function"},t.wbg.__wbg_call_33d7bcddbbfa394a=function(){return ge(function(e,n){const r=g(e).call(g(n));return w(r)},arguments)},t.wbg.__wbg_next_726d1c2255989269=function(e){const n=g(e).next;return w(n)},t.wbg.__wbg_next_3d0c4cc33e7418c9=function(){return ge(function(e){const n=g(e).next();return w(n)},arguments)},t.wbg.__wbg_done_e5655b169bb04f60=function(e){return g(e).done},t.wbg.__wbg_value_8f901bca1014f843=function(e){const n=g(e).value;return w(n)},t.wbg.__wbg_new_ac586205e4424583=function(){return w(new Map)},t.wbg.__wbg_fromCodePoint_721be91df0158faf=function(){return ge(function(e){const n=String.fromCodePoint(e>>>0);return w(n)},arguments)},t.wbg.__wbindgen_number_new=function(e){return w(e)},t.wbg.__wbg_set_a55cff623a9eaa21=function(e,n,r){const o=g(e).set(g(n),g(r));return w(o)},t.wbg.__wbg_new_ee1a3da85465d621=function(){const e=new Array;return w(e)},t.wbg.__wbg_new_e6a9fecc2bf26696=function(){const e=new Object;return w(e)},t.wbg.__wbg_set_e93b31d47b90bff6=function(e,n,r){g(e)[h(n)]=h(r)},t.wbg.__wbg_set_64cc39858b2ec3f1=function(e,n,r){g(e)[n>>>0]=h(r)},t.wbg.__wbg_new_3ee7ebe9952c1fbd=function(e,n){const r=new Error(V(e,n));return w(r)},t.wbg.__wbindgen_string_get=function(e,n){const r=g(n),o=typeof r=="string"?r:void 0;var i=xe(o)?0:L(o,s.__wbindgen_malloc,s.__wbindgen_realloc),a=E;l()[e/4+1]=a,l()[e/4+0]=i},t.wbg.__wbindgen_is_null=function(e){return g(e)===null},t.wbg.__wbindgen_is_undefined=function(e){return g(e)===void 0},t.wbg.__wbg_length_51f19f73d6d9eff3=function(e){return g(e).length},t.wbg.__wbindgen_memory=function(){const e=s.memory;return w(e)},t.wbg.__wbg_buffer_34f5ec9f8a838ba0=function(e){const n=g(e).buffer;return w(n)},t.wbg.__wbg_new_cda198d9dbc6d7ea=function(e){const n=new Uint8Array(g(e));return w(n)},t.wbg.__wbg_set_1a930cfcda1a8067=function(e,n,r){g(e).set(g(n),r>>>0)},t.wbg.__wbindgen_boolean_get=function(e){const n=g(e);return typeof n=="boolean"?n?1:0:2},t.wbg.__wbg_instanceof_Uint8Array_36c37b9ca15e3e0a=function(e){return g(e)instanceof Uint8Array},t.wbg.__wbg_instanceof_ArrayBuffer_02bbeeb60438c785=function(e){return g(e)instanceof ArrayBuffer},t.wbg.__wbg_String_7462bcc0fcdbaf7d=function(e,n){const r=String(g(n)),o=L(r,s.__wbindgen_malloc,s.__wbindgen_realloc),i=E;l()[e/4+1]=i,l()[e/4+0]=o},t.wbg.__wbindgen_object_clone_ref=function(e){const n=g(e);return w(n)},t.wbg.__wbindgen_debug_string=function(e,n){const r=Le(g(n)),o=L(r,s.__wbindgen_malloc,s.__wbindgen_realloc),i=E;l()[e/4+1]=i,l()[e/4+0]=o},t.wbg.__wbindgen_throw=function(e,n){throw new Error(V(e,n))},t}function Mt(t,e){return s=t.exports,ct.__wbindgen_wasm_module=e,we=new Float64Array,me=new Int32Array,he=new Uint32Array,ve=new Uint8Array,s}async function ct(t){typeof t>"u"&&(t="/syn-zeug/assets/biobox_bg.wasm");const e=qt();(typeof t=="string"||typeof Request=="function"&&t instanceof Request||typeof URL=="function"&&t instanceof URL)&&(t=fetch(t));const{instance:n,module:r}=await St(await t,e);return Mt(n,r)}const It="/syn-zeug/assets/Logo.14ee8233.svg";function Ze(t,e,n){const r=t.slice();return r[20]=e[n],r}function et(t){let e,n,r=t[20].name+"",o,i,a,c,v,d;return{c(){e=b("div"),n=b("a"),o=ot(r),a=O(),_(n,"class",i=ue(t[20].name.includes(t[3])||t[3]==""?"":"inactive")+" svelte-lovd9v"),_(e,"class",c=ue(t[20].name.includes(t[3])||t[3]==""?"functions":"functions inactive")+" svelte-lovd9v")},m(k,p){rt(k,e,p),u(e,n),u(n,o),u(e,a),v||(d=z(e,"click",t[12]),v=!0)},p(k,p){p&8&&i!==(i=ue(k[20].name.includes(k[3])||k[3]==""?"":"inactive")+" svelte-lovd9v")&&_(n,"class",i),p&8&&c!==(c=ue(k[20].name.includes(k[3])||k[3]==""?"functions":"functions inactive")+" svelte-lovd9v")&&_(e,"class",c)},d(k){k&&Se(e),v=!1,d()}}}function Tt(t){let e,n,r,o,i,a,c,v,d,k,p,T,P,S,$,ee,te,G,f,m,y,D,Me,R,ne,Ie,re,Te,ke,Ne,K,q,M,oe,Ue,Ae,Re,Oe,Fe,ie,We,N,ze,I,se,Pe,ce,Be,ae,De,le,He,F,je,Je,H=t[4],x=[];for(let A=0;A<H.length;A+=1)x[A]=et(Ze(t,H,A));return{c(){e=b("main"),n=b("nav"),r=b("div"),o=b("img"),a=O(),c=b("ul"),c.innerHTML=`<li class="svelte-lovd9v"><a href="#" class="svelte-lovd9v">About</a></li> 
      <li class="svelte-lovd9v"><a href="#" class="svelte-lovd9v">Contact</a></li> 
      <li class="svelte-lovd9v"><a href="#" class="svelte-lovd9v">Projects</a></li>`,v=O(),d=b("div"),k=b("div"),p=b("div"),T=b("div"),T.textContent="Operations",P=O(),S=b("div"),$=b("input"),ee=O();for(let A=0;A<x.length;A+=1)x[A].c();te=O(),G=b("div"),f=O(),m=b("div"),y=b("div"),D=b("div"),D.innerHTML=`<p class="svelte-lovd9v">Recipe</p> 
          <i class="fas fa-save svelte-lovd9v"></i> 
          <i class="fas fa-folder svelte-lovd9v"></i> 
          <i class="fas fa-trash svelte-lovd9v"></i>`,Me=O(),R=b("textarea"),Ie=O(),re=b("div"),re.innerHTML='<button class="btn svelte-lovd9v">Verbose Recipe</button>',Te=O(),ke=b("div"),Ne=O(),K=b("div"),q=b("div"),M=b("div"),oe=b("p"),oe.textContent="Input",Ue=O(),Ae=b("i"),Re=O(),Oe=b("i"),Fe=O(),ie=b("i"),We=O(),N=b("textarea"),ze=O(),I=b("div"),se=b("p"),se.textContent="Output",Pe=O(),ce=b("i"),Be=O(),ae=b("i"),De=O(),le=b("i"),He=O(),F=b("textarea"),lt(o.src,i=It)||_(o,"src",i),_(o,"alt","University of Sheffield iGEM Logo"),_(o,"class","logo svelte-lovd9v"),_(r,"class","site-title svelte-lovd9v"),_(c,"class","nav-links flex-row svelte-lovd9v"),_(n,"class","nav-grid svelte-lovd9v"),_(T,"class","title svelte-lovd9v"),_($,"id","search"),_($,"type","search"),_($,"name","search-function"),_($,"placeholder","Search a function.."),_($,"class","svelte-lovd9v"),_(S,"class","search-functions svelte-lovd9v"),_(p,"class","flex-column svelte-lovd9v"),_(k,"id","operations"),_(k,"class","svelte-lovd9v"),_(G,"class","gutter svelte-lovd9v"),_(D,"class","title grid-title svelte-lovd9v"),_(R,"name","recipe"),_(R,"class","text-area svelte-lovd9v"),_(R,"cols","30"),_(R,"rows","10"),R.value=ne=t[1][0],_(re,"class","controls flex-row svelte-lovd9v"),_(y,"class","flex-column svelte-lovd9v"),_(m,"id","recipe"),_(m,"class","svelte-lovd9v"),_(ke,"class","gutter svelte-lovd9v"),_(oe,"class","svelte-lovd9v"),_(Ae,"class","fas fa-folder-plus svelte-lovd9v"),_(Oe,"class","fas fa-upload svelte-lovd9v"),_(ie,"class","fas fa-trash svelte-lovd9v"),_(M,"class","title grid-title svelte-lovd9v"),_(N,"name","input"),_(N,"class","text-area svelte-lovd9v"),_(N,"cols","30"),_(N,"rows","10"),_(se,"class","svelte-lovd9v"),_(ce,"class","fas fa-save svelte-lovd9v"),_(ae,"class","fas fa-copy svelte-lovd9v"),_(le,"class","fas fa-reply-all svelte-lovd9v"),_(I,"class","title grid-title svelte-lovd9v"),_(F,"name","output"),_(F,"class","text-area svelte-lovd9v"),_(F,"cols","30"),_(F,"rows","10"),F.value=t[2],_(q,"class","flex-column svelte-lovd9v"),_(K,"id","IO"),_(K,"class","svelte-lovd9v"),_(d,"id","wrapper"),_(d,"class","grid-content svelte-lovd9v"),_(e,"class","svelte-lovd9v")},m(A,W){rt(A,e,W),u(e,n),u(n,r),u(r,o),u(n,a),u(n,c),u(e,v),u(e,d),u(d,k),u(k,p),u(p,T),u(p,P),u(p,S),u(S,$),de($,t[3]),u(p,ee);for(let C=0;C<x.length;C+=1)x[C].m(p,null);u(d,te),u(d,G),u(d,f),u(d,m),u(m,y),u(y,D),u(y,Me),u(y,R),u(y,Ie),u(y,re),u(d,Te),u(d,ke),u(d,Ne),u(d,K),u(K,q),u(q,M),u(M,oe),u(M,Ue),u(M,Ae),u(M,Re),u(M,Oe),u(M,Fe),u(M,ie),u(q,We),u(q,N),de(N,t[0]),u(q,ze),u(q,I),u(I,se),u(I,Pe),u(I,ce),u(I,Be),u(I,ae),u(I,De),u(I,le),u(q,He),u(q,F),je||(Je=[z(o,"click",t[8]),z($,"input",t[11]),z(ie,"click",t[13]),z(N,"input",t[14]),z(ce,"click",t[7]),z(ae,"click",t[5]),z(le,"click",t[6])],je=!0)},p(A,[W]){if(W&8&&de($,A[3]),W&536){H=A[4];let C;for(C=0;C<H.length;C+=1){const Ve=Ze(A,H,C);x[C]?x[C].p(Ve,W):(x[C]=et(Ve),x[C].c(),x[C].m(p,null))}for(;C<x.length;C+=1)x[C].d(1);x.length=H.length}W&2&&ne!==(ne=A[1][0])&&(R.value=ne),W&1&&de(N,A[0]),W&4&&(F.value=A[2])},i:B,o:B,d(A){A&&Se(e),ut(x,A),je=!1,Z(Je)}}}const Nt=1e3;function Ut(t,e,n){let r="",o="",i="",a=!0,c=null,v=[{name:"Type",functionality:f=>`${f.kind()} (${f.alphabet()})`},{name:"Sequence Length",functionality:f=>f.len()},{name:"Convert to Lower Case",functionality:f=>f.normalize_case("Lower").to_string()},{name:"Convert to Upper Case",functionality:f=>f.normalize_case("Upper").to_string()},{name:"Reverse Sequence",functionality:f=>f.rev().to_string()},{name:"Count Elements",functionality:f=>[...f.count_elements()].sort().map(m=>m.join(": ")).join(`
`)},{name:"Reverse Complement",functionality:f=>f.reverse_complement().to_string()},{name:"Convert to RNA",functionality:f=>f.convert("Rna").to_string()},{name:"Convert to DNA",functionality:f=>f.convert("Dna").to_string()},{name:"Convert to Protein",functionality:f=>f.convert("Protein").to_string()},{name:"Find ORFs",functionality:f=>f.find_orfs(1).map(m=>{let y=j.from_js(m[1]).convert("Protein").to_string();return`S${m[0].start} E${m[0].end} O${m[0].offset}: ${y}`}).join(`
`)}],d=["No tool selected",[{name:"No tool selected",functionality:f=>null}]];const k=()=>{let f=document.createElement("textarea");document.body.appendChild(f),f.value=d[1][0].functionality(c),f.select(),document.execCommand("copy"),document.body.removeChild(f)},p=()=>{let f=document.createElement("textarea");document.body.appendChild(f);let m=[];for(let y=0;y<Nt;y++)try{m.push(JSON.parse(localStorage.getItem(localStorage.key(y))))}catch(D){console.log(D)}m=m.filter(y=>y!==null),console.log(m),f.value=JSON.stringify(m),f.select(),document.execCommand("copy"),document.body.removeChild(f)},T=()=>{const f=Date.now(),y={time:new Date(f),name:d[0],outputText:d[1][0].functionality(c)};localStorage.setItem(y.name,JSON.stringify(y)),console.log("Output has been saves to local storage",y)},P=()=>{document.querySelector("body").classList.toggle("light",!a),a=!a},S=f=>{let m=f.target.innerText;n(1,d=[m,v.filter(y=>y.name==m)]),console.log(d[1][0])};function $(){i=this.value,n(3,i)}const ee=f=>S(f),te=()=>n(0,r="");function G(){r=this.value,n(0,r)}return t.$$.update=()=>{if(t.$$.dirty&1,t.$$.dirty&1)try{n(10,c=new j(r.trim()))}catch(f){n(2,o=f)}if(t.$$.dirty&1026)try{n(2,o=d[1][0].functionality(c))}catch(f){n(2,o=f)}},[r,d,o,i,v,k,p,T,P,S,c,$,ee,te,G]}class Rt extends kt{constructor(e){super(),ht(this,e,Ut,Tt,nt,{})}}(async()=>(await ct(),new Rt({target:document.body})))();
