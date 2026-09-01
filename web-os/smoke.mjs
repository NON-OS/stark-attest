import { readFile } from "node:fs/promises";
const dir=new URL(".",import.meta.url).pathname;
const rd=f=>readFile(dir+f);
const {instance}=await WebAssembly.instantiate(await rd("verifier.wasm"),{});
const w=instance.exports;
const index=JSON.parse(await readFile(dir+"index.json","utf8"));
const policy=new Uint8Array(await rd("policy_root.bin"));
const kroot=new Uint8Array(await rd("kernel.root.bin"));
const push=b=>{const p=w.wasm_alloc(b.length);new Uint8Array(w.memory.buffer,p,b.length).set(b);return p;};
const gate=(root,tr,ctx)=>{const r=push(root),t=push(tr),c=push(ctx);const ok=w.verify(r,t,tr.length,c,ctx.length);w.wasm_free(r,root.length);w.wasm_free(t,tr.length);w.wasm_free(c,ctx.length);return ok===1;};
// the page's proof-anatomy parser, kept in lockstep: same offsets as app.js
function anatomy(t){
  const dv=new DataView(t.buffer,t.byteOffset,t.length);let o=8;
  const depth=t[o];o+=1;o+=depth*32;o+=Math.ceil(depth/8);
  o+=64; // trace_root + comp_root
  const oodLen=dv.getUint32(o,true);o+=4;o+=oodLen*16;
  const friLayers=dv.getUint32(o,true);o+=4;o+=friLayers*32;
  const finalLen=dv.getUint32(o,true);o+=4;o+=finalLen*16;
  const friQueries=dv.getUint32(o,true);
  return {depth,oodLen,friLayers,finalLen,friQueries};
}
let ok=0,bad=0,anatBad=0;
for(const it of index.items){
  const root=it.kind==="kernel"?kroot:policy;
  const tr=new Uint8Array(await rd(it.slug+".trailer.bin"));
  const ctx=new Uint8Array(await rd(it.slug+".context.bin"));
  if(gate(root,tr,ctx)) ok++; else {bad++;console.log("REFUSED",it.slug);}
  const a=anatomy(tr);
  if(a.depth!==8||a.friQueries!==32||a.friLayers<10||a.oodLen<1){anatBad++;console.log("ANATOMY WRONG",it.slug,JSON.stringify(a));}
}
// in-wasm root reconstruction from the complete leaf sets
const hex=b=>[...b].map(x=>x.toString(16).padStart(2,"0")).join("");
async function fold(f){
  const leaves=new Uint8Array(await rd(f));
  const p=push(leaves),o=w.wasm_alloc(32);
  const ok=w.fold_root(p,leaves.length/32,o);
  const root=new Uint8Array(w.memory.buffer,o,32).slice();
  return {ok:ok===1,root};
}
const fp=await fold("leaves.bin"), fk=await fold("kernel_leaves.bin");
const kr=new Uint8Array(await rd("kernel.root.bin"));
const reconOk = fp.ok && hex(fp.root)===hex(policy) && fk.ok && hex(fk.root)===hex(kr);
console.log("root reconstruction:", reconOk ? "both roots reproduced from the complete leaf sets" : "MISMATCH");

// tamper check on the kernel
const ktr=new Uint8Array(await rd("kernel.trailer.bin"));ktr[ktr.length-1]^=1;
const kctx=new Uint8Array(await rd("kernel.context.bin"));
const tamperRejected=!gate(kroot,ktr,kctx);
console.log(`browser logic: ${ok}/${index.items.length} verified, ${bad} refused, ${anatBad} anatomy mismatches; tamper rejected: ${tamperRejected}`);
process.exit(bad===0 && anatBad===0 && tamperRejected && reconOk ? 0 : 1);
