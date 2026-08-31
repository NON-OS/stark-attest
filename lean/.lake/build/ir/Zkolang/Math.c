// Lean compiler output
// Module: Zkolang.Math
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
LEAN_EXPORT lean_object* l_Zkolang_Math_sq(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_pow8___boxed(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_double___boxed(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_triple(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_cube(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_cube___boxed(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_sq___boxed(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_pow8(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_pow6(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_double(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_pow4___boxed(lean_object*);
lean_object* lean_int_mul(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_triple___boxed(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_pow6___boxed(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_pow4(lean_object*);
lean_object* lean_int_add(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Math_sq(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_int_mul(x_1, x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_sq___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Math_sq(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_cube(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = lean_int_mul(x_1, x_1);
x_3 = lean_int_mul(x_2, x_1);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_cube___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Math_cube(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_pow4(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = lean_int_mul(x_1, x_1);
x_3 = lean_int_mul(x_2, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_pow4___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Math_pow4(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_pow6(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = l_Zkolang_Math_cube(x_1);
x_3 = lean_int_mul(x_2, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_pow6___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Math_pow6(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_pow8(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = l_Zkolang_Math_pow4(x_1);
x_3 = lean_int_mul(x_2, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_pow8___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Math_pow8(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_double(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_int_add(x_1, x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_double___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Math_double(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_triple(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = lean_int_add(x_1, x_1);
x_3 = lean_int_add(x_2, x_1);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Math_triple___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Math_triple(x_1);
lean_dec(x_1);
return x_2;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_Math(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
