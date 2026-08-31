// Lean compiler output
// Module: Zkolang.Poly
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
LEAN_EXPORT lean_object* l_Zkolang_Poly_cubic___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Poly_lerp___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Poly_cubic(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Poly_line(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Poly_line___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Poly_quad(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lean_int_sub(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Poly_lerp(lean_object*, lean_object*, lean_object*);
lean_object* lean_int_mul(lean_object*, lean_object*);
lean_object* lean_int_add(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Poly_quad___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Poly_line(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; 
x_4 = lean_int_mul(x_1, x_3);
x_5 = lean_int_add(x_4, x_2);
lean_dec(x_4);
return x_5;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Poly_line___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_Zkolang_Poly_line(x_1, x_2, x_3);
lean_dec(x_3);
lean_dec(x_2);
lean_dec(x_1);
return x_4;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Poly_quad(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; 
x_5 = lean_int_mul(x_1, x_4);
x_6 = lean_int_add(x_5, x_2);
lean_dec(x_5);
x_7 = lean_int_mul(x_6, x_4);
lean_dec(x_6);
x_8 = lean_int_add(x_7, x_3);
lean_dec(x_7);
return x_8;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Poly_quad___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = l_Zkolang_Poly_quad(x_1, x_2, x_3, x_4);
lean_dec(x_4);
lean_dec(x_3);
lean_dec(x_2);
lean_dec(x_1);
return x_5;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Poly_cubic(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; 
x_6 = lean_int_mul(x_1, x_5);
x_7 = lean_int_add(x_6, x_2);
lean_dec(x_6);
x_8 = lean_int_mul(x_7, x_5);
lean_dec(x_7);
x_9 = lean_int_add(x_8, x_3);
lean_dec(x_8);
x_10 = lean_int_mul(x_9, x_5);
lean_dec(x_9);
x_11 = lean_int_add(x_10, x_4);
lean_dec(x_10);
return x_11;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Poly_cubic___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
lean_object* x_6; 
x_6 = l_Zkolang_Poly_cubic(x_1, x_2, x_3, x_4, x_5);
lean_dec(x_5);
lean_dec(x_4);
lean_dec(x_3);
lean_dec(x_2);
lean_dec(x_1);
return x_6;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Poly_lerp(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_4 = lean_int_sub(x_2, x_1);
x_5 = lean_int_mul(x_3, x_4);
lean_dec(x_4);
x_6 = lean_int_add(x_1, x_5);
lean_dec(x_5);
return x_6;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Poly_lerp___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_Zkolang_Poly_lerp(x_1, x_2, x_3);
lean_dec(x_3);
lean_dec(x_2);
lean_dec(x_1);
return x_4;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_Poly(uint8_t builtin, lean_object* w) {
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
