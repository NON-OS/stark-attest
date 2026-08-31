// Lean compiler output
// Module: Zkolang
// Imports: Init Zkolang.Field Zkolang.Math Zkolang.Poly Zkolang.Hash Zkolang.Opening Zkolang.Wiring Zkolang.Stream Zkolang.BatchInv Zkolang.Reduce Zkolang.Trailer Zkolang.Padding Zkolang.Params
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
lean_object* initialize_Init(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Field(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Math(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Poly(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Hash(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Opening(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Wiring(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Stream(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_BatchInv(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Reduce(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Trailer(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Padding(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Params(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Field(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Math(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Poly(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Hash(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Opening(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Wiring(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Stream(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_BatchInv(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Reduce(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Trailer(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Padding(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Params(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
