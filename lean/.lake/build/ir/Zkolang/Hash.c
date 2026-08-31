// Lean compiler output
// Module: Zkolang.Hash
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
static lean_object* l_Zkolang_Hash_p___closed__3;
LEAN_EXPORT lean_object* l_Zkolang_Hash_p;
static lean_object* l_Zkolang_Hash_p___closed__1;
static lean_object* l_Zkolang_Hash_p___closed__2;
lean_object* lean_nat_sub(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Hash_sboxExp;
static lean_object* l_Zkolang_Hash_groupOrder___closed__1;
LEAN_EXPORT lean_object* l_Zkolang_Hash_groupOrder;
lean_object* lean_nat_add(lean_object*, lean_object*);
static lean_object* _init_l_Zkolang_Hash_p___closed__1() {
_start:
{
lean_object* x_1; 
x_1 = lean_cstr_to_nat("18446744073709551616");
return x_1;
}
}
static lean_object* _init_l_Zkolang_Hash_p___closed__2() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Zkolang_Hash_p___closed__1;
x_2 = lean_cstr_to_nat("4294967296");
x_3 = lean_nat_sub(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Hash_p___closed__3() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Zkolang_Hash_p___closed__2;
x_2 = lean_unsigned_to_nat(1u);
x_3 = lean_nat_add(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Hash_p() {
_start:
{
lean_object* x_1; 
x_1 = l_Zkolang_Hash_p___closed__3;
return x_1;
}
}
static lean_object* _init_l_Zkolang_Hash_groupOrder___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Zkolang_Hash_p;
x_2 = lean_unsigned_to_nat(1u);
x_3 = lean_nat_sub(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_Zkolang_Hash_groupOrder() {
_start:
{
lean_object* x_1; 
x_1 = l_Zkolang_Hash_groupOrder___closed__1;
return x_1;
}
}
static lean_object* _init_l_Zkolang_Hash_sboxExp() {
_start:
{
lean_object* x_1; 
x_1 = lean_unsigned_to_nat(7u);
return x_1;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_Hash(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Zkolang_Hash_p___closed__1 = _init_l_Zkolang_Hash_p___closed__1();
lean_mark_persistent(l_Zkolang_Hash_p___closed__1);
l_Zkolang_Hash_p___closed__2 = _init_l_Zkolang_Hash_p___closed__2();
lean_mark_persistent(l_Zkolang_Hash_p___closed__2);
l_Zkolang_Hash_p___closed__3 = _init_l_Zkolang_Hash_p___closed__3();
lean_mark_persistent(l_Zkolang_Hash_p___closed__3);
l_Zkolang_Hash_p = _init_l_Zkolang_Hash_p();
lean_mark_persistent(l_Zkolang_Hash_p);
l_Zkolang_Hash_groupOrder___closed__1 = _init_l_Zkolang_Hash_groupOrder___closed__1();
lean_mark_persistent(l_Zkolang_Hash_groupOrder___closed__1);
l_Zkolang_Hash_groupOrder = _init_l_Zkolang_Hash_groupOrder();
lean_mark_persistent(l_Zkolang_Hash_groupOrder);
l_Zkolang_Hash_sboxExp = _init_l_Zkolang_Hash_sboxExp();
lean_mark_persistent(l_Zkolang_Hash_sboxExp);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
