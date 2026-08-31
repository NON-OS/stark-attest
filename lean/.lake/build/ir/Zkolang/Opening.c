// Lean compiler output
// Module: Zkolang.Opening
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
LEAN_EXPORT lean_object* l_Zkolang_Opening_inject___rarg(lean_object*, lean_object*, uint8_t);
lean_object* l_List_appendTR___rarg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Zkolang_Opening_0__Zkolang_Opening_inject_match__1_splitter___rarg(uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Opening_nodeHalf___rarg___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Zkolang_Opening_0__Zkolang_Opening_inject_match__1_splitter___rarg___boxed(lean_object*, lean_object*, lean_object*);
lean_object* l_List_takeTR_go___rarg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_List_drop___rarg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Opening_inject(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Opening_nodeHalf(lean_object*);
lean_object* lean_array_mk(lean_object*);
LEAN_EXPORT lean_object* l___private_Zkolang_Opening_0__Zkolang_Opening_inject_match__1_splitter(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Opening_inject___rarg___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Opening_nodeHalf___rarg(lean_object*, lean_object*, uint8_t);
static lean_object* l_Zkolang_Opening_nodeHalf___rarg___closed__1;
LEAN_EXPORT lean_object* l_Zkolang_Opening_inject___rarg(lean_object* x_1, lean_object* x_2, uint8_t x_3) {
_start:
{
if (x_3 == 0)
{
lean_object* x_4; 
x_4 = l_List_appendTR___rarg(x_1, x_2);
return x_4;
}
else
{
lean_object* x_5; 
x_5 = l_List_appendTR___rarg(x_2, x_1);
return x_5;
}
}
}
LEAN_EXPORT lean_object* l_Zkolang_Opening_inject(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_Zkolang_Opening_inject___rarg___boxed), 3, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Opening_inject___rarg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
uint8_t x_4; lean_object* x_5; 
x_4 = lean_unbox(x_3);
lean_dec(x_3);
x_5 = l_Zkolang_Opening_inject___rarg(x_1, x_2, x_4);
return x_5;
}
}
static lean_object* _init_l_Zkolang_Opening_nodeHalf___rarg___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_box(0);
x_2 = lean_array_mk(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Opening_nodeHalf___rarg(lean_object* x_1, lean_object* x_2, uint8_t x_3) {
_start:
{
if (x_3 == 0)
{
lean_object* x_4; lean_object* x_5; 
x_4 = l_Zkolang_Opening_nodeHalf___rarg___closed__1;
lean_inc(x_1);
x_5 = l_List_takeTR_go___rarg(x_1, x_1, x_2, x_4);
lean_dec(x_1);
return x_5;
}
else
{
lean_object* x_6; 
x_6 = l_List_drop___rarg(x_2, x_1);
lean_dec(x_1);
return x_6;
}
}
}
LEAN_EXPORT lean_object* l_Zkolang_Opening_nodeHalf(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_Zkolang_Opening_nodeHalf___rarg___boxed), 3, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Opening_nodeHalf___rarg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
uint8_t x_4; lean_object* x_5; 
x_4 = lean_unbox(x_3);
lean_dec(x_3);
x_5 = l_Zkolang_Opening_nodeHalf___rarg(x_1, x_2, x_4);
return x_5;
}
}
LEAN_EXPORT lean_object* l___private_Zkolang_Opening_0__Zkolang_Opening_inject_match__1_splitter___rarg(uint8_t x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
if (x_1 == 0)
{
lean_inc(x_2);
return x_2;
}
else
{
lean_inc(x_3);
return x_3;
}
}
}
LEAN_EXPORT lean_object* l___private_Zkolang_Opening_0__Zkolang_Opening_inject_match__1_splitter(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l___private_Zkolang_Opening_0__Zkolang_Opening_inject_match__1_splitter___rarg___boxed), 3, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l___private_Zkolang_Opening_0__Zkolang_Opening_inject_match__1_splitter___rarg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
uint8_t x_4; lean_object* x_5; 
x_4 = lean_unbox(x_1);
lean_dec(x_1);
x_5 = l___private_Zkolang_Opening_0__Zkolang_Opening_inject_match__1_splitter___rarg(x_4, x_2, x_3);
lean_dec(x_3);
lean_dec(x_2);
return x_5;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_Opening(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Zkolang_Opening_nodeHalf___rarg___closed__1 = _init_l_Zkolang_Opening_nodeHalf___rarg___closed__1();
lean_mark_persistent(l_Zkolang_Opening_nodeHalf___rarg___closed__1);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
