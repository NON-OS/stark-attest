"use strict";
// The attestation surface: parse each proof, show its real structure, verify it
// in the same gate the kernel runs. A verdict without the work behind it is a
// checkbox; this shows the work.

const $ = id => document.getElementById(id);
const hex = b => [...b].map(x => x.toString(16).padStart(2, "0")).join("");
const short = (h, n = 10) => h.slice(0, n) + "…" + h.slice(-4);

// capability bits, from src/security/policy/capability/types.rs
const CAPS = ["process:create","process:kill","memory:map","memory:unmap","file:read","file:write",
"file:create","file:delete","net:bind","net:connect","device","syscall","irq","module:load",
"module:unload","crypto:keys","vault","ephemeral-mem","isolation","zerostate","cap:grant","cap:revoke",
"attest:create","attest:verify","secure-boot","crypto-fs-vault","pq-signatures","hw-abstraction","debug","audit"];
const decodeCaps = n => { const o = []; for (let i = 0; i < 32; i++) if ((n >>> i) & 1) o.push(CAPS[i] || ("bit" + i)); return o; };

let wasm, mem, index, policyRoot, kernelRoot;
const times = [];
const cache = {};

async function bytes(p) { const r = await fetch(p); if (!r.ok) throw new Error(p + ": " + r.status); return new Uint8Array(await r.arrayBuffer()); }
function push(b) { const p = wasm.wasm_alloc(b.length); new Uint8Array(mem.buffer, p, b.length).set(b); return p; }
function gate(root, trailer, ctx) {
  const r = push(root), t = push(trailer), c = push(ctx);
  const t0 = performance.now();
  const ok = wasm.verify(r, t, trailer.length, c, ctx.length);
  const ms = performance.now() - t0;
  wasm.wasm_free(r, root.length); wasm.wasm_free(t, trailer.length); wasm.wasm_free(c, ctx.length);
  return { ok: ok === 1, ms };
}

// Parse the trailer header far enough to expose the proof's shape. The layout
// is fixed by serialize_ext.rs: fp is 8 bytes, fp2 is 16, a Merkle path is a
// u32 count then that many 32-byte digests.
function anatomy(trailer) {
  const dv = new DataView(trailer.buffer, trailer.byteOffset, trailer.length);
  let o = 8;                                   // past the magic
  const depth = trailer[o]; o += 1;
  const siblings = [];
  for (let i = 0; i < depth; i++) { siblings.push(hex(trailer.slice(o, o + 32))); o += 32; }
  const dirBytes = Math.ceil(depth / 8);
  const dirs = [];
  for (let i = 0; i < depth; i++) dirs.push((trailer[o + (i >> 3)] >> (i & 7)) & 1);
  o += dirBytes;
  const traceRoot = hex(trailer.slice(o, o + 32)); o += 32;
  const compRoot = hex(trailer.slice(o, o + 32)); o += 32;
  const oodLen = dv.getUint32(o, true); o += 4; o += oodLen * 16;
  const friLayers = dv.getUint32(o, true); o += 4;
  const friRoots = [];
  for (let i = 0; i < friLayers; i++) { friRoots.push(hex(trailer.slice(o, o + 32))); o += 32; }
  const finalLen = dv.getUint32(o, true); o += 4; o += finalLen * 16;
  const friQueries = dv.getUint32(o, true);
  return { depth, siblings, dirs, traceRoot, compRoot, oodLen, friLayers, friRoots, finalLen, friQueries };
}

function foldRoot(leaves) {
  const p = push(leaves), o = wasm.wasm_alloc(32);
  const t0 = performance.now();
  const ok = wasm.fold_root(p, leaves.length / 32, o);
  const ms = performance.now() - t0;
  const root = new Uint8Array(mem.buffer, o, 32).slice();
  wasm.wasm_free(p, leaves.length); wasm.wasm_free(o, 32);
  return { ok: ok === 1, root, ms };
}

// slot index from the trailer's direction bits: bit k of the index is the
// k-th direction, so the path IS the address
function slotOf(trailer) {
  const depth = trailer[8]; const off = 9 + depth * 32;
  let idx = 0;
  for (let i = 0; i < depth; i++) idx |= ((trailer[off + (i >> 3)] >> (i & 7)) & 1) << i;
  return idx;
}

function median(a) { if (!a.length) return 0; const s = [...a].sort((x, y) => x - y); return s[s.length >> 1]; }
function refresh() { $("s-ok").textContent = times.length; if (times.length) $("s-ms").textContent = median(times).toFixed(0) + " ms"; }

function detailHtml(it, a, meas, ms) {
  const path = a.siblings.map((s, i) =>
    `<div class="pl"><span class="lv">L${i}</span><span class="sib mono">${short(s, 14)}</span><span class="dir">${a.dirs[i] ? "node is right child" : "node is left child"}</span></div>`
  ).join("");
  const roots = a.friRoots.map((r, i) => `<div class="fl"><span class="lv">fold ${i}</span><span class="mono">${short(r, 14)}</span><span class="dom">domain / ${2 ** (i + 1)}</span></div>`).join("");
  return `<div class="detail">
    <div class="col">
      <div class="ch">Merkle authentication path &nbsp; depth ${a.depth}</div>
      <div class="pl"><span class="lv">leaf</span><span class="sib mono">${short(meas, 14)}</span><span class="dir">measured binary, capabilities, epoch</span></div>
      ${path}
      <div class="pl"><span class="lv">root</span><span class="sib mono">${short(index.policy_root, 14)}</span><span class="dir">the committed statement</span></div>
    </div>
    <div class="col">
      <div class="ch">STARK proof anatomy</div>
      <div class="kv"><span>trace commitment</span><span class="mono">${short(a.traceRoot, 14)}</span></div>
      <div class="kv"><span>composition commitment</span><span class="mono">${short(a.compRoot, 14)}</span></div>
      <div class="kv"><span>out-of-domain frame</span><span class="mono">${a.oodLen} field elements</span></div>
      <div class="kv"><span>FRI folding layers</span><span class="mono">${a.friLayers}</span></div>
      ${roots}
      <div class="kv"><span>final layer</span><span class="mono">${a.finalLen} coefficients</span></div>
      <div class="kv"><span>FRI queries</span><span class="mono">${a.friQueries}</span></div>
      <div class="done">verified in ${ms.toFixed(0)} ms: recomputed the leaf, walked ${a.depth} Merkle levels to the root, checked ${a.friQueries} FRI queries for proximity to low degree over ${a.friLayers} folds, confirmed the grinding.</div>
    </div>
  </div>`;
}

async function verifyOne(it) {
  const root = it.kind === "kernel" ? kernelRoot : policyRoot;
  $("v-" + it.slug).className = "verdict run"; $("v-" + it.slug).textContent = "verifying";
  if (!cache[it.slug]) {
    const [tr, ctx] = await Promise.all([bytes(it.slug + ".trailer.bin"), bytes(it.slug + ".context.bin")]);
    cache[it.slug] = { tr, ctx };
  }
  const { tr, ctx } = cache[it.slug];
  const res = gate(root, tr, ctx);
  const a = anatomy(tr);
  const meas = hex(ctx.slice(0, 32));
  const det = $("d-" + it.slug);
  if (res.ok) {
    times.push(res.ms);
    $("v-" + it.slug).className = "verdict ok"; $("v-" + it.slug).textContent = "verified " + res.ms.toFixed(0) + " ms";
    det.innerHTML = detailHtml(it, a, meas, res.ms);
    det.style.display = "";
  } else {
    $("v-" + it.slug).className = "verdict bad"; $("v-" + it.slug).textContent = "REFUSED";
  }
  refresh();
  return res.ok;
}

function render() {
  const tb = $("rows"); tb.innerHTML = "";
  for (const it of index.items) {
    const caps = it.kind === "kernel" ? "<span class='cap'>boot chain</span>"
      : decodeCaps(it.caps).map(c => `<span class="cap">${c}</span>`).join("") || "<span class='cap'>none</span>";
    const tr = document.createElement("tr"); tr.className = "main";
    tr.innerHTML = `
      <td class="name">${it.slug}<span class="h">${it.handle}</span></td>
      <td><span class="kind ${it.kind}">${it.kind}</span></td>
      <td><div class="caps">${caps}</div></td>
      <td class="meas mono">${it.measurement.slice(0, 14)}…</td>
      <td class="mono dimc">${(it.trailer_bytes / 1024).toFixed(0)} KB</td>
      <td><span class="verdict idle" id="v-${it.slug}">not yet</span></td>
      <td><button class="rowbtn" data-s="${it.slug}">inspect</button></td>`;
    tb.appendChild(tr);
    const dr = document.createElement("tr"); dr.className = "drow";
    dr.innerHTML = `<td colspan="7"><div id="d-${it.slug}" style="display:none"></div></td>`;
    tb.appendChild(dr);
  }
  tb.querySelectorAll("button").forEach(b => { b.onclick = () => verifyOne(index.items.find(x => x.slug === b.dataset.s)); });
}

$("verify-all").onclick = async () => {
  $("verify-all").disabled = true; $("reset").disabled = true;
  times.length = 0; refresh();
  for (const it of index.items) await verifyOne(it);
  $("verify-all").disabled = false; $("reset").disabled = false;
};
$("reset").onclick = () => {
  times.length = 0; refresh();
  index.items.forEach(it => { $("v-" + it.slug).className = "verdict idle"; $("v-" + it.slug).textContent = "not yet"; $("d-" + it.slug).style.display = "none"; });
};

(async () => {
  try {
    const wasmBytes = await bytes("verifier.wasm");
    const { instance } = await WebAssembly.instantiate(wasmBytes, {});
    wasm = instance.exports; mem = wasm.memory;
    index = JSON.parse(new TextDecoder().decode(await bytes("index.json")));
    [policyRoot, kernelRoot] = await Promise.all([bytes("policy_root.bin"), bytes("kernel.root.bin")]);
    $("s-total").textContent = index.items.length;
    $("root").textContent = index.policy_root;
    $("epoch").textContent = index.epoch;
    try {
      const [leaves, kleaves] = await Promise.all([bytes("leaves.bin"), bytes("kernel_leaves.bin")]);
      const members = index.items.filter(x => x.kind === "capsule").length;
      const f = foldRoot(leaves), fk = foldRoot(kleaves);
      const match = f.ok && hex(f.root) === index.policy_root;
      const kmatch = fk.ok && hex(fk.root) === hex(kernelRoot);
      const el = $("recon");
      if (match && kmatch) {
        el.className = "recon ok";
        el.innerHTML = `set transparency: this page just refolded the complete tree, all 256 slots, ${members} members plus ${256 - members} reserved, through 255 Poseidon compressions in ${(f.ms + fk.ms).toFixed(0)} ms, and reproduced both roots exactly. Nothing else is enrolled under them.`;
      } else {
        el.className = "recon bad";
        el.textContent = "ROOT RECONSTRUCTION FAILED: the served leaf set does not fold to the enforced root";
      }
    } catch (e) { $("recon").textContent = "leaf set unavailable: " + e.message; }
    render();
    $("verify-all").disabled = false; $("reset").disabled = false;
    if (location.hash === "#auto") $("verify-all").click();
    const m = location.hash.match(/#one=(\w+)/); if (m) { const it = index.items.find(x => x.slug === m[1]); if (it) verifyOne(it); }
  } catch (e) { $("root").textContent = "failed to load: " + e.message; }
})();
