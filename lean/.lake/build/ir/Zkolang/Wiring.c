// Lean compiler output
// Module: Zkolang.Wiring
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
LEAN_EXPORT lean_object* l_Zkolang_Wiring_next(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Wiring_next___boxed(lean_object*, lean_object*);
lean_object* l_List_getD___rarg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Wiring_indexOf(lean_object*, lean_object*);
lean_object* l_List_lengthTRAux___rarg(lean_object*, lean_object*);
static lean_object* l_Zkolang_Wiring_indexOf___closed__1;
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Zkolang_Wiring_0__Zkolang_Wiring_indexOf_match__1_splitter___rarg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Zkolang_Wiring_0__Zkolang_Wiring_indexOf_match__1_splitter(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Wiring_indexOf___boxed(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
static lean_object* _init_l_Zkolang_Wiring_indexOf___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_2, 0, x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Wiring_indexOf(lean_object* x_1, lean_object* x_2) {
_start:
{
if (lean_obj_tag(x_1) == 0)
{
lean_object* x_3; 
x_3 = lean_box(0);
return x_3;
}
else
{
lean_object* x_4; lean_object* x_5; uint8_t x_6; 
x_4 = lean_ctor_get(x_1, 0);
x_5 = lean_ctor_get(x_1, 1);
x_6 = lean_nat_dec_eq(x_4, x_2);
if (x_6 == 0)
{
lean_object* x_7; 
x_7 = l_Zkolang_Wiring_indexOf(x_5, x_2);
if (lean_obj_tag(x_7) == 0)
{
lean_object* x_8; 
x_8 = lean_box(0);
return x_8;
}
else
{
uint8_t x_9; 
x_9 = !lean_is_exclusive(x_7);
if (x_9 == 0)
{
lean_object* x_10; lean_object* x_11; lean_object* x_12; 
x_10 = lean_ctor_get(x_7, 0);
x_11 = lean_unsigned_to_nat(1u);
x_12 = lean_nat_add(x_10, x_11);
lean_dec(x_10);
lean_ctor_set(x_7, 0, x_12);
return x_7;
}
else
{
lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; 
x_13 = lean_ctor_get(x_7, 0);
lean_inc(x_13);
lean_dec(x_7);
x_14 = lean_unsigned_to_nat(1u);
x_15 = lean_nat_add(x_13, x_14);
lean_dec(x_13);
x_16 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_16, 0, x_15);
return x_16;
}
}
}
else
{
lean_object* x_17; 
x_17 = l_Zkolang_Wiring_indexOf___closed__1;
return x_17;
}
}
}
}
LEAN_EXPORT lean_object* l_Zkolang_Wiring_indexOf___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_Zkolang_Wiring_indexOf(x_1, x_2);
lean_dec(x_2);
lean_dec(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Wiring_next(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_Zkolang_Wiring_indexOf(x_1, x_2);
if (lean_obj_tag(x_3) == 0)
{
lean_inc(x_2);
return x_2;
}
else
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; 
x_4 = lean_ctor_get(x_3, 0);
lean_inc(x_4);
lean_dec(x_3);
x_5 = lean_unsigned_to_nat(1u);
x_6 = lean_nat_add(x_4, x_5);
lean_dec(x_4);
x_7 = lean_unsigned_to_nat(0u);
x_8 = l_List_lengthTRAux___rarg(x_1, x_7);
x_9 = lean_nat_mod(x_6, x_8);
lean_dec(x_8);
lean_dec(x_6);
x_10 = l_List_getD___rarg(x_1, x_9, x_2);
return x_10;
}
}
}
LEAN_EXPORT lean_object* l_Zkolang_Wiring_next___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_Zkolang_Wiring_next(x_1, x_2);
lean_dec(x_2);
lean_dec(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* l___private_Zkolang_Wiring_0__Zkolang_Wiring_indexOf_match__1_splitter___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
if (lean_obj_tag(x_1) == 0)
{
lean_object* x_5; 
lean_dec(x_4);
x_5 = lean_apply_1(x_3, x_2);
return x_5;
}
else
{
lean_object* x_6; lean_object* x_7; lean_object* x_8; 
lean_dec(x_3);
x_6 = lean_ctor_get(x_1, 0);
lean_inc(x_6);
x_7 = lean_ctor_get(x_1, 1);
lean_inc(x_7);
lean_dec(x_1);
x_8 = lean_apply_3(x_4, x_6, x_7, x_2);
return x_8;
}
}
}
LEAN_EXPORT lean_object* l___private_Zkolang_Wiring_0__Zkolang_Wiring_indexOf_match__1_splitter(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l___private_Zkolang_Wiring_0__Zkolang_Wiring_indexOf_match__1_splitter___rarg), 4, 0);
return x_2;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_Wiring(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Zkolang_Wiring_indexOf___closed__1 = _init_l_Zkolang_Wiring_indexOf___closed__1();
lean_mark_persistent(l_Zkolang_Wiring_indexOf___closed__1);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
