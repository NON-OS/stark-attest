// Lean compiler output
// Module: Zkolang.Reduce
// Imports: Init Zkolang.Field
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
static lean_object* l_Zkolang_Reduce_epsilon___closed__4;
lean_object* lean_nat_to_int(lean_object*);
lean_object* l_Int_pow(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Reduce_reduced(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Reduce_epsilon;
lean_object* lean_int_sub(lean_object*, lean_object*);
lean_object* lean_int_mul(lean_object*, lean_object*);
static lean_object* l_Zkolang_Reduce_epsilon___closed__2;
LEAN_EXPORT lean_object* l_Zkolang_Reduce_reduced___boxed(lean_object*, lean_object*, lean_object*);
lean_object* lean_int_add(lean_object*, lean_object*);
static lean_object* l_Zkolang_Reduce_original___closed__1;
LEAN_EXPORT lean_object* l_Zkolang_Reduce_original(lean_object*, lean_object*, lean_object*);
static lean_object* l_Zkolang_Reduce_epsilon___closed__1;
LEAN_EXPORT lean_object* l_Zkolang_Reduce_original___boxed(lean_object*, lean_object*, lean_object*);
static lean_object* l_Zkolang_Reduce_epsilon___closed__3;
static lean_object* _init_l_Zkolang_Reduce_epsilon___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(2u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_l_Zkolang_Reduce_epsilon___closed__2() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Zkolang_Reduce_epsilon___closed__1;
x_2 = lean_unsigned_to_nat(32u);
x_3 = l_Int_pow(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Reduce_epsilon___closed__3() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(1u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_l_Zkolang_Reduce_epsilon___closed__4() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Zkolang_Reduce_epsilon___closed__2;
x_2 = l_Zkolang_Reduce_epsilon___closed__3;
x_3 = lean_int_sub(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Reduce_epsilon() {
_start:
{
lean_object* x_1; 
x_1 = l_Zkolang_Reduce_epsilon___closed__4;
return x_1;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Reduce_reduced(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; 
x_4 = lean_int_sub(x_1, x_3);
x_5 = l_Zkolang_Reduce_epsilon;
x_6 = lean_int_mul(x_2, x_5);
x_7 = lean_int_add(x_4, x_6);
lean_dec(x_6);
lean_dec(x_4);
return x_7;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Reduce_reduced___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_Zkolang_Reduce_reduced(x_1, x_2, x_3);
lean_dec(x_3);
lean_dec(x_2);
lean_dec(x_1);
return x_4;
}
}
static lean_object* _init_l_Zkolang_Reduce_original___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Zkolang_Reduce_epsilon___closed__1;
x_2 = lean_unsigned_to_nat(64u);
x_3 = l_Int_pow(x_1, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Reduce_original(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; 
x_4 = l_Zkolang_Reduce_epsilon___closed__2;
x_5 = lean_int_mul(x_4, x_3);
x_6 = lean_int_add(x_2, x_5);
lean_dec(x_5);
x_7 = l_Zkolang_Reduce_original___closed__1;
x_8 = lean_int_mul(x_7, x_6);
lean_dec(x_6);
x_9 = lean_int_add(x_1, x_8);
lean_dec(x_8);
return x_9;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Reduce_original___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_Zkolang_Reduce_original(x_1, x_2, x_3);
lean_dec(x_3);
lean_dec(x_2);
lean_dec(x_1);
return x_4;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Field(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_Reduce(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Field(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Zkolang_Reduce_epsilon___closed__1 = _init_l_Zkolang_Reduce_epsilon___closed__1();
lean_mark_persistent(l_Zkolang_Reduce_epsilon___closed__1);
l_Zkolang_Reduce_epsilon___closed__2 = _init_l_Zkolang_Reduce_epsilon___closed__2();
lean_mark_persistent(l_Zkolang_Reduce_epsilon___closed__2);
l_Zkolang_Reduce_epsilon___closed__3 = _init_l_Zkolang_Reduce_epsilon___closed__3();
lean_mark_persistent(l_Zkolang_Reduce_epsilon___closed__3);
l_Zkolang_Reduce_epsilon___closed__4 = _init_l_Zkolang_Reduce_epsilon___closed__4();
lean_mark_persistent(l_Zkolang_Reduce_epsilon___closed__4);
l_Zkolang_Reduce_epsilon = _init_l_Zkolang_Reduce_epsilon();
lean_mark_persistent(l_Zkolang_Reduce_epsilon);
l_Zkolang_Reduce_original___closed__1 = _init_l_Zkolang_Reduce_original___closed__1();
lean_mark_persistent(l_Zkolang_Reduce_original___closed__1);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
