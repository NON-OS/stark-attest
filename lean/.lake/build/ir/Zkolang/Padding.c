// Lean compiler output
// Module: Zkolang.Padding
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
LEAN_EXPORT lean_object* l_Zkolang_Padding_padImage;
static lean_object* l_Zkolang_Padding_padImage___closed__5;
static lean_object* l_Zkolang_Padding_padImage___closed__1;
static lean_object* l_Zkolang_Padding_padImage___closed__2;
static lean_object* l_Zkolang_Padding_padImage___closed__6;
static lean_object* l_Zkolang_Padding_padImage___closed__4;
static lean_object* l_Zkolang_Padding_padImage___closed__3;
static lean_object* _init_l_Zkolang_Padding_padImage___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = lean_box(0);
x_2 = lean_unsigned_to_nat(75u);
x_3 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_3, 0, x_2);
lean_ctor_set(x_3, 1, x_1);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Padding_padImage___closed__2() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = lean_unsigned_to_nat(82u);
x_2 = l_Zkolang_Padding_padImage___closed__1;
x_3 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_3, 0, x_1);
lean_ctor_set(x_3, 1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Padding_padImage___closed__3() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = lean_unsigned_to_nat(65u);
x_2 = l_Zkolang_Padding_padImage___closed__2;
x_3 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_3, 0, x_1);
lean_ctor_set(x_3, 1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Padding_padImage___closed__4() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = lean_unsigned_to_nat(84u);
x_2 = l_Zkolang_Padding_padImage___closed__3;
x_3 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_3, 0, x_1);
lean_ctor_set(x_3, 1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Padding_padImage___closed__5() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = lean_unsigned_to_nat(83u);
x_2 = l_Zkolang_Padding_padImage___closed__4;
x_3 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_3, 0, x_1);
lean_ctor_set(x_3, 1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Padding_padImage___closed__6() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = l_Zkolang_Padding_padImage___closed__5;
x_3 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_3, 0, x_1);
lean_ctor_set(x_3, 1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Padding_padImage() {
_start:
{
lean_object* x_1; 
x_1 = l_Zkolang_Padding_padImage___closed__6;
return x_1;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_Padding(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Zkolang_Padding_padImage___closed__1 = _init_l_Zkolang_Padding_padImage___closed__1();
lean_mark_persistent(l_Zkolang_Padding_padImage___closed__1);
l_Zkolang_Padding_padImage___closed__2 = _init_l_Zkolang_Padding_padImage___closed__2();
lean_mark_persistent(l_Zkolang_Padding_padImage___closed__2);
l_Zkolang_Padding_padImage___closed__3 = _init_l_Zkolang_Padding_padImage___closed__3();
lean_mark_persistent(l_Zkolang_Padding_padImage___closed__3);
l_Zkolang_Padding_padImage___closed__4 = _init_l_Zkolang_Padding_padImage___closed__4();
lean_mark_persistent(l_Zkolang_Padding_padImage___closed__4);
l_Zkolang_Padding_padImage___closed__5 = _init_l_Zkolang_Padding_padImage___closed__5();
lean_mark_persistent(l_Zkolang_Padding_padImage___closed__5);
l_Zkolang_Padding_padImage___closed__6 = _init_l_Zkolang_Padding_padImage___closed__6();
lean_mark_persistent(l_Zkolang_Padding_padImage___closed__6);
l_Zkolang_Padding_padImage = _init_l_Zkolang_Padding_padImage();
lean_mark_persistent(l_Zkolang_Padding_padImage);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
