// Lean compiler output
// Module: Zkolang.BatchInv
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
LEAN_EXPORT lean_object* l_Zkolang_BatchInv_outAt(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_BatchInv_prodL___boxed(lean_object*);
LEAN_EXPORT lean_object* l___private_Zkolang_BatchInv_0__Zkolang_BatchInv_prodL_match__1_splitter___rarg___boxed(lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_to_int(lean_object*);
lean_object* lean_int_mul(lean_object*, lean_object*);
lean_object* l_List_takeTR_go___rarg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_List_drop___rarg(lean_object*, lean_object*);
static lean_object* l_Zkolang_BatchInv_prodL___closed__1;
LEAN_EXPORT lean_object* l___private_Zkolang_BatchInv_0__Zkolang_BatchInv_prodL_match__1_splitter___rarg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_BatchInv_prodL(lean_object*);
lean_object* lean_array_mk(lean_object*);
static lean_object* l_Zkolang_BatchInv_outAt___closed__1;
LEAN_EXPORT lean_object* l_Zkolang_BatchInv_outAt___boxed(lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Zkolang_BatchInv_0__Zkolang_BatchInv_prodL_match__1_splitter(lean_object*);
static lean_object* _init_l_Zkolang_BatchInv_prodL___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(1u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_BatchInv_prodL(lean_object* x_1) {
_start:
{
if (lean_obj_tag(x_1) == 0)
{
lean_object* x_2; 
x_2 = l_Zkolang_BatchInv_prodL___closed__1;
return x_2;
}
else
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_3 = lean_ctor_get(x_1, 0);
x_4 = lean_ctor_get(x_1, 1);
x_5 = l_Zkolang_BatchInv_prodL(x_4);
x_6 = lean_int_mul(x_3, x_5);
lean_dec(x_5);
return x_6;
}
}
}
LEAN_EXPORT lean_object* l_Zkolang_BatchInv_prodL___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_BatchInv_prodL(x_1);
lean_dec(x_1);
return x_2;
}
}
static lean_object* _init_l_Zkolang_BatchInv_outAt___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_box(0);
x_2 = lean_array_mk(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_BatchInv_outAt(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; 
x_4 = lean_unsigned_to_nat(1u);
x_5 = lean_nat_add(x_3, x_4);
x_6 = l_List_drop___rarg(x_5, x_2);
x_7 = l_Zkolang_BatchInv_prodL(x_6);
lean_dec(x_6);
x_8 = lean_int_mul(x_1, x_7);
lean_dec(x_7);
x_9 = l_Zkolang_BatchInv_outAt___closed__1;
lean_inc(x_2);
x_10 = l_List_takeTR_go___rarg(x_2, x_2, x_3, x_9);
lean_dec(x_2);
x_11 = l_Zkolang_BatchInv_prodL(x_10);
lean_dec(x_10);
x_12 = lean_int_mul(x_8, x_11);
lean_dec(x_11);
lean_dec(x_8);
return x_12;
}
}
LEAN_EXPORT lean_object* l_Zkolang_BatchInv_outAt___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_Zkolang_BatchInv_outAt(x_1, x_2, x_3);
lean_dec(x_1);
return x_4;
}
}
LEAN_EXPORT lean_object* l___private_Zkolang_BatchInv_0__Zkolang_BatchInv_prodL_match__1_splitter___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
if (lean_obj_tag(x_1) == 0)
{
lean_dec(x_3);
lean_inc(x_2);
return x_2;
}
else
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_4 = lean_ctor_get(x_1, 0);
lean_inc(x_4);
x_5 = lean_ctor_get(x_1, 1);
lean_inc(x_5);
lean_dec(x_1);
x_6 = lean_apply_2(x_3, x_4, x_5);
return x_6;
}
}
}
LEAN_EXPORT lean_object* l___private_Zkolang_BatchInv_0__Zkolang_BatchInv_prodL_match__1_splitter(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l___private_Zkolang_BatchInv_0__Zkolang_BatchInv_prodL_match__1_splitter___rarg___boxed), 3, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l___private_Zkolang_BatchInv_0__Zkolang_BatchInv_prodL_match__1_splitter___rarg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l___private_Zkolang_BatchInv_0__Zkolang_BatchInv_prodL_match__1_splitter___rarg(x_1, x_2, x_3);
lean_dec(x_2);
return x_4;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
lean_object* initialize_Zkolang_Field(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_BatchInv(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Zkolang_Field(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Zkolang_BatchInv_prodL___closed__1 = _init_l_Zkolang_BatchInv_prodL___closed__1();
lean_mark_persistent(l_Zkolang_BatchInv_prodL___closed__1);
l_Zkolang_BatchInv_outAt___closed__1 = _init_l_Zkolang_BatchInv_outAt___closed__1();
lean_mark_persistent(l_Zkolang_BatchInv_outAt___closed__1);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
