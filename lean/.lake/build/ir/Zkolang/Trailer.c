// Lean compiler output
// Module: Zkolang.Trailer
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
LEAN_EXPORT lean_object* l_Zkolang_Trailer_sibStart___boxed(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Trailer_dirBytes(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Trailer_sibStart(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Trailer_rate;
lean_object* lean_nat_div(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Trailer_header___boxed(lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Trailer_header(lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Zkolang_Trailer_dirBytes___boxed(lean_object*);
static lean_object* _init_l_Zkolang_Trailer_rate() {
_start:
{
lean_object* x_1; 
x_1 = lean_unsigned_to_nat(4u);
return x_1;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Trailer_dirBytes(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; 
x_2 = lean_unsigned_to_nat(7u);
x_3 = lean_nat_add(x_1, x_2);
x_4 = lean_unsigned_to_nat(8u);
x_5 = lean_nat_div(x_3, x_4);
lean_dec(x_3);
return x_5;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Trailer_dirBytes___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Trailer_dirBytes(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Trailer_header(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; 
x_2 = l_Zkolang_Trailer_dirBytes(x_1);
x_3 = lean_unsigned_to_nat(9u);
x_4 = lean_nat_add(x_3, x_2);
lean_dec(x_2);
x_5 = l_Zkolang_Trailer_rate;
x_6 = lean_nat_mul(x_1, x_5);
x_7 = lean_unsigned_to_nat(8u);
x_8 = lean_nat_mul(x_6, x_7);
lean_dec(x_6);
x_9 = lean_nat_add(x_4, x_8);
lean_dec(x_8);
lean_dec(x_4);
return x_9;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Trailer_header___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Trailer_header(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Trailer_sibStart(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; 
x_2 = l_Zkolang_Trailer_dirBytes(x_1);
x_3 = lean_unsigned_to_nat(9u);
x_4 = lean_nat_add(x_3, x_2);
lean_dec(x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* l_Zkolang_Trailer_sibStart___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_Zkolang_Trailer_sibStart(x_1);
lean_dec(x_1);
return x_2;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Zkolang_Trailer(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Zkolang_Trailer_rate = _init_l_Zkolang_Trailer_rate();
lean_mark_persistent(l_Zkolang_Trailer_rate);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
