#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_StringList {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_StringList;

typedef struct wire_SharedComplexEnumInAllBlocks_Empty {

} wire_SharedComplexEnumInAllBlocks_Empty;

typedef struct wire_SharedComplexEnumInAllBlocks_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_SharedComplexEnumInAllBlocks_Primitives;

typedef struct wire_SharedComplexEnumInAllBlocks_Nested {
  struct wire_SharedComplexEnumInAllBlocks *field0;
} wire_SharedComplexEnumInAllBlocks_Nested;

typedef struct wire_SharedComplexEnumInAllBlocks_Optional {
  int32_t *field0;
  struct wire_uint_8_list *field1;
} wire_SharedComplexEnumInAllBlocks_Optional;

typedef struct wire_float_32_list {
  float *ptr;
  int32_t len;
} wire_float_32_list;

typedef struct wire_SharedComplexEnumInAllBlocks_Buffer {
  struct wire_float_32_list *field0;
} wire_SharedComplexEnumInAllBlocks_Buffer;

typedef struct wire_SharedComplexEnumInAllBlocks_Enums {
  int32_t field0;
} wire_SharedComplexEnumInAllBlocks_Enums;

typedef struct wire_SharedComplexEnumInAllBlocks_BytesArray {
  struct wire_uint_8_list *field0;
} wire_SharedComplexEnumInAllBlocks_BytesArray;

typedef union SharedComplexEnumInAllBlocksKind {
  struct wire_SharedComplexEnumInAllBlocks_Empty *Empty;
  struct wire_SharedComplexEnumInAllBlocks_Primitives *Primitives;
  struct wire_SharedComplexEnumInAllBlocks_Nested *Nested;
  struct wire_SharedComplexEnumInAllBlocks_Optional *Optional;
  struct wire_SharedComplexEnumInAllBlocks_Buffer *Buffer;
  struct wire_SharedComplexEnumInAllBlocks_Enums *Enums;
  struct wire_SharedComplexEnumInAllBlocks_BytesArray *BytesArray;
} SharedComplexEnumInAllBlocksKind;

typedef struct wire_SharedComplexEnumInAllBlocks {
  int32_t tag;
  union SharedComplexEnumInAllBlocksKind *kind;
} wire_SharedComplexEnumInAllBlocks;

typedef struct wire_list_shared_complex_enum_in_all_blocks {
  struct wire_SharedComplexEnumInAllBlocks *ptr;
  int32_t len;
} wire_list_shared_complex_enum_in_all_blocks;

typedef struct wire_SharedStructInAllBlocks {
  int32_t id;
  double num;
  struct wire_uint_8_list *name;
  struct wire_list_shared_complex_enum_in_all_blocks *enum_list;
} wire_SharedStructInAllBlocks;

typedef struct wire_SharedStructInBlock1And2 {
  int32_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_SharedStructInBlock1And2;

typedef struct wire_SharedStructInBlock2And3 {
  int32_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_SharedStructInBlock2And3;

typedef struct wire_int_32_list {
  int32_t *ptr;
  int32_t len;
} wire_int_32_list;

typedef struct wire_list_shared_struct_in_all_blocks {
  struct wire_SharedStructInAllBlocks *ptr;
  int32_t len;
} wire_list_shared_struct_in_all_blocks;

typedef struct wire_list_shared_weekdays_enum_in_all_blocks {
  int32_t *ptr;
  int32_t len;
} wire_list_shared_weekdays_enum_in_all_blocks;

typedef struct DartCObject *WireSyncReturn;

typedef struct wire_StructOnlyForBlock1 {
  int8_t *id;
  double *num;
  struct wire_uint_8_list *name;
} wire_StructOnlyForBlock1;

typedef struct wire_StructOnlyForBlock2 {
  int16_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_StructOnlyForBlock2;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_SharedStructOnlyForSyncTest {
  struct wire_uint_8_list *name;
  double score;
} wire_SharedStructOnlyForSyncTest;

typedef struct wire_StructOnlyForBlock3 {
  int64_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_StructOnlyForBlock3;

typedef struct wire_StructDefinedInBlock3 {
  struct wire_uint_8_list *name;
} wire_StructDefinedInBlock3;

typedef struct wire_EnumDefinedInBlock3_Quit {

} wire_EnumDefinedInBlock3_Quit;

typedef struct wire_EnumDefinedInBlock3_Move {
  int32_t x;
  int32_t y;
} wire_EnumDefinedInBlock3_Move;

typedef struct wire_EnumDefinedInBlock3_Write {
  struct wire_uint_8_list *field0;
} wire_EnumDefinedInBlock3_Write;

typedef struct wire_EnumDefinedInBlock3_ChangeColor {
  int32_t field0;
  int32_t field1;
  int32_t field2;
} wire_EnumDefinedInBlock3_ChangeColor;

typedef union EnumDefinedInBlock3Kind {
  struct wire_EnumDefinedInBlock3_Quit *Quit;
  struct wire_EnumDefinedInBlock3_Move *Move;
  struct wire_EnumDefinedInBlock3_Write *Write;
  struct wire_EnumDefinedInBlock3_ChangeColor *ChangeColor;
} EnumDefinedInBlock3Kind;

typedef struct wire_EnumDefinedInBlock3 {
  int32_t tag;
  union EnumDefinedInBlock3Kind *kind;
} wire_EnumDefinedInBlock3;

typedef struct wire_list_struct_defined_in_block_3 {
  struct wire_StructDefinedInBlock3 *ptr;
  int32_t len;
} wire_list_struct_defined_in_block_3;

typedef struct wire_list_enum_defined_in_block_3 {
  struct wire_EnumDefinedInBlock3 *ptr;
  int32_t len;
} wire_list_enum_defined_in_block_3;

struct wire_StringList *new_StringList(int32_t len);

int32_t *new_box_autoadd_i32(int32_t value);

struct wire_SharedComplexEnumInAllBlocks *new_box_autoadd_shared_complex_enum_in_all_blocks(void);

struct wire_SharedStructInAllBlocks *new_box_autoadd_shared_struct_in_all_blocks(void);

struct wire_SharedStructInBlock1And2 *new_box_autoadd_shared_struct_in_block_1_and_2(void);

struct wire_SharedStructInBlock2And3 *new_box_autoadd_shared_struct_in_block_2_and_3(void);

struct wire_SharedComplexEnumInAllBlocks *new_box_shared_complex_enum_in_all_blocks(void);

struct wire_float_32_list *new_float_32_list(int32_t len);

struct wire_int_32_list *new_int_32_list(int32_t len);

struct wire_list_shared_complex_enum_in_all_blocks *new_list_shared_complex_enum_in_all_blocks(int32_t len);

struct wire_list_shared_struct_in_all_blocks *new_list_shared_struct_in_all_blocks(int32_t len);

struct wire_list_shared_weekdays_enum_in_all_blocks *new_list_shared_weekdays_enum_in_all_blocks(int32_t len);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_Primitives(void);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_Nested(void);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_Optional(void);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_Buffer(void);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_Enums(void);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_BytesArray(void);

void free_WireSyncReturn(WireSyncReturn ptr);

void wire_test_method__method__StructOnlyForBlock1(int64_t port_,
                                                   struct wire_StructOnlyForBlock1 *that,
                                                   struct wire_uint_8_list *message,
                                                   uint16_t num);

void wire_test_static_method__static_method__StructOnlyForBlock1(int64_t port_,
                                                                 struct wire_uint_8_list *message);

void wire_test_method__method__StructOnlyForBlock2(int64_t port_,
                                                   struct wire_StructOnlyForBlock2 *that,
                                                   struct wire_uint_8_list *message,
                                                   uint16_t num);

void wire_test_static_method__static_method__StructOnlyForBlock2(int64_t port_,
                                                                 struct wire_uint_8_list *message);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_test_inbuilt_type_in_block_3(int64_t port_, int32_t a, float b);

void wire_test_string_in_block_3(int64_t port_, struct wire_uint_8_list *s, uint64_t i);

void wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3(int64_t port_,
                                                                          struct wire_uint_8_list *name,
                                                                          double score);

void wire_test_shared_struct_only_for_sync_as_input_with_no_sync_return_in_block_3(int64_t port_,
                                                                                   struct wire_SharedStructOnlyForSyncTest *obj,
                                                                                   double default_score);

void wire_test_all_shared_struct_in_block_3(int64_t port_,
                                            struct wire_SharedStructInAllBlocks *custom,
                                            struct wire_uint_8_list *s,
                                            int32_t i);

void wire_test_shared_struct_in_block_3_for_2_and_3(int64_t port_,
                                                    struct wire_SharedStructInBlock2And3 *custom,
                                                    struct wire_uint_8_list *s,
                                                    int32_t i);

void wire_test_cross_shared_struct_in_block_3_for_2_and_3(int64_t port_,
                                                          struct wire_uint_8_list *name);

WireSyncReturn wire_test_cross_shared_struct_in_sync_in_block_3_for_2_and_3(struct wire_uint_8_list *name);

void wire_test_unique_struct_3(int64_t port_,
                               struct wire_StructOnlyForBlock3 *custom,
                               struct wire_uint_8_list *s,
                               int64_t i);

void wire_test_struct_defined_in_block_3(int64_t port_, struct wire_StructDefinedInBlock3 *custom);

void wire_test_enum_defined_in_block_3(int64_t port_, struct wire_EnumDefinedInBlock3 *custom);

void wire_test_list_in_block_3(int64_t port_,
                               struct wire_list_shared_struct_in_all_blocks *shared_structs,
                               struct wire_StringList *strings,
                               struct wire_int_32_list *nums,
                               struct wire_list_shared_weekdays_enum_in_all_blocks *weekdays,
                               struct wire_list_struct_defined_in_block_3 *struct_list,
                               struct wire_list_enum_defined_in_block_3 *enum_list);

void wire_test_method__method__EnumDefinedInBlock3(int64_t port_,
                                                   struct wire_EnumDefinedInBlock3 *that,
                                                   struct wire_uint_8_list *message);

void wire_test_static_method__static_method__EnumDefinedInBlock3(int64_t port_,
                                                                 struct wire_uint_8_list *message);

void wire_test_method__method__StructDefinedInBlock3(int64_t port_,
                                                     struct wire_StructDefinedInBlock3 *that,
                                                     struct wire_uint_8_list *message);

void wire_test_static_method__static_method__StructDefinedInBlock3(int64_t port_,
                                                                   struct wire_uint_8_list *message);

void wire_test_method__method__StructOnlyForBlock3(int64_t port_,
                                                   struct wire_StructOnlyForBlock3 *that,
                                                   struct wire_uint_8_list *message,
                                                   uint16_t num);

void wire_test_static_method__static_method__StructOnlyForBlock3(int64_t port_,
                                                                 struct wire_uint_8_list *message);

struct wire_EnumDefinedInBlock3 *new_box_autoadd_enum_defined_in_block_3(void);

struct wire_StructDefinedInBlock3 *new_box_autoadd_struct_defined_in_block_3(void);

struct wire_StructOnlyForBlock3 *new_box_autoadd_struct_only_for_block_3(void);

struct wire_list_enum_defined_in_block_3 *new_list_enum_defined_in_block_3(int32_t len);

struct wire_list_struct_defined_in_block_3 *new_list_struct_defined_in_block_3(int32_t len);

union EnumDefinedInBlock3Kind *inflate_EnumDefinedInBlock3_Move(void);

union EnumDefinedInBlock3Kind *inflate_EnumDefinedInBlock3_Write(void);

union EnumDefinedInBlock3Kind *inflate_EnumDefinedInBlock3_ChangeColor(void);

static int64_t dummy_method_to_enforce_bundling_ApiBlock3Class(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_test_inbuilt_type_in_block_3);
    dummy_var ^= ((int64_t) (void*) wire_test_string_in_block_3);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_only_for_sync_as_input_with_no_sync_return_in_block_3);
    dummy_var ^= ((int64_t) (void*) wire_test_all_shared_struct_in_block_3);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_in_block_3_for_2_and_3);
    dummy_var ^= ((int64_t) (void*) wire_test_cross_shared_struct_in_block_3_for_2_and_3);
    dummy_var ^= ((int64_t) (void*) wire_test_cross_shared_struct_in_sync_in_block_3_for_2_and_3);
    dummy_var ^= ((int64_t) (void*) wire_test_unique_struct_3);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_defined_in_block_3);
    dummy_var ^= ((int64_t) (void*) wire_test_enum_defined_in_block_3);
    dummy_var ^= ((int64_t) (void*) wire_test_list_in_block_3);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__EnumDefinedInBlock3);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__EnumDefinedInBlock3);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__StructDefinedInBlock3);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__StructDefinedInBlock3);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__StructOnlyForBlock3);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__StructOnlyForBlock3);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_defined_in_block_3);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_defined_in_block_3);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_only_for_block_3);
    dummy_var ^= ((int64_t) (void*) new_list_enum_defined_in_block_3);
    dummy_var ^= ((int64_t) (void*) new_list_struct_defined_in_block_3);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDefinedInBlock3_Move);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDefinedInBlock3_Write);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDefinedInBlock3_ChangeColor);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
