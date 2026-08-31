// Lean compiler output
// Module: Zkolang.Field
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
static lean_object* l_Zkolang_Field_p___closed__1;
static lean_object* l_Zkolang_Field_p___closed__2;
static lean_object* l_Zkolang_Field_p___closed__6;
lean_object* lean_nat_to_int(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Field_p;
lean_object* l_Int_pow(lean_object*, lean_object*);
static lean_object* l_Zkolang_Field_p___closed__3;
LEAN_EXPORT lean_object* l_Zkolang_Field_neg(lean_object*);
lean_object* lean_int_sub(lean_object*, lean_object*);
static lean_object* l_Zkolang_Field_p___closed__4;
static lean_object* l_Zkolang_Field_p___closed__5;
lean_object* lean_int_add(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Field_neg___boxed(lean_object*);
static lean_object* l_Zkolang_Field_neg___closed__1;
static lean_object* _init_l_Zkolang_Field_p___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(2u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_l_Zkolang_Field_p___closed__2() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Zkolang_Field_p___closed__1;
x_2 = lean_unsigned_to_nat(64u);
x_3 = l_Int_pow(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Field_p___closed__3() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Zkolang_Field_p___closed__1;
x_2 = lean_unsigned_to_nat(32u);
x_3 = l_Int_pow(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Field_p___closed__4() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Zkolang_Field_p___closed__2;
x_2 = l_Zkolang_Field_p___closed__3;
x_3 = lean_int_sub(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Field_p___closed__5() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(1u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_l_Zkolang_Field_p___closed__6() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Zkolang_Field_p___closed__4;
x_2 = l_Zkolang_Field_p___closed__5;
x_3 = lean_int_add(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Field_p() {
_start:
{
lean_object* x_1; 
x_1 = l_Zkolang_Field_p___closed__6;
return x_1;
}
}
static lean_object* _init_l_Zkolang_Field_neg___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Field_neg(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = l_Zkolang_Field_neg___closed__1;
x_3 = lean_int_sub(x_2, x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Field_neg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Field_neg(x_1);
lean_dec(x_1);
return x_2;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_Field(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Zkolang_Field_p___closed__1 = _init_l_Zkolang_Field_p___closed__1();
lean_mark_persistent(l_Zkolang_Field_p___closed__1);
l_Zkolang_Field_p___closed__2 = _init_l_Zkolang_Field_p___closed__2();
lean_mark_persistent(l_Zkolang_Field_p___closed__2);
l_Zkolang_Field_p___closed__3 = _init_l_Zkolang_Field_p___closed__3();
lean_mark_persistent(l_Zkolang_Field_p___closed__3);
l_Zkolang_Field_p___closed__4 = _init_l_Zkolang_Field_p___closed__4();
lean_mark_persistent(l_Zkolang_Field_p___closed__4);
l_Zkolang_Field_p___closed__5 = _init_l_Zkolang_Field_p___closed__5();
lean_mark_persistent(l_Zkolang_Field_p___closed__5);
l_Zkolang_Field_p___closed__6 = _init_l_Zkolang_Field_p___closed__6();
lean_mark_persistent(l_Zkolang_Field_p___closed__6);
l_Zkolang_Field_p = _init_l_Zkolang_Field_p();
lean_mark_persistent(l_Zkolang_Field_p);
l_Zkolang_Field_neg___closed__1 = _init_l_Zkolang_Field_neg___closed__1();
lean_mark_persistent(l_Zkolang_Field_neg___closed__1);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
