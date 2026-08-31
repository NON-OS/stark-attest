// Lean compiler output
// Module: Zkolang.Params
// Imports: Init
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
static lean_object* l_Zkolang_Params_deployed___closed__1;
LEAN_EXPORT lean_object* l_Zkolang_Params_effective(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Params_survivalNum___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Params_effective___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Params_deployed;
lean_object* lean_nat_pow(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Params_survivalNum(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Params_effective(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_inc(x_1);
return x_1;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Params_effective___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_Zkolang_Params_effective(x_1, x_2);
lean_dec(x_2);
lean_dec(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Params_survivalNum(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lean_nat_pow(x_1, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Params_survivalNum___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_Zkolang_Params_survivalNum(x_1, x_2);
lean_dec(x_2);
lean_dec(x_1);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Params_deployed___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; lean_object* x_4; 
x_1 = lean_unsigned_to_nat(32u);
x_2 = lean_unsigned_to_nat(16u);
x_3 = lean_unsigned_to_nat(3u);
x_4 = lean_alloc_ctor(0, 3, 0);
lean_ctor_set(x_4, 0, x_1);
lean_ctor_set(x_4, 1, x_2);
lean_ctor_set(x_4, 2, x_3);
return x_4;
}
}
static lean_object* _init_l_Zkolang_Params_deployed() {
_start:
{
lean_object* x_1; 
x_1 = l_Zkolang_Params_deployed___closed__1;
return x_1;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_Params(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Zkolang_Params_deployed___closed__1 = _init_l_Zkolang_Params_deployed___closed__1();
lean_mark_persistent(l_Zkolang_Params_deployed___closed__1);
l_Zkolang_Params_deployed = _init_l_Zkolang_Params_deployed();
lean_mark_persistent(l_Zkolang_Params_deployed);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
