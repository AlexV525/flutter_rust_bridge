// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `optional_basic_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/optional_basic_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('optional_basic', () {
    addTestsIdentityFunctionCall(
        exampleOptionalBasicTypeI8TwinSyncSse, <int?>[null, 0, -128, 127]);
    addTestsIdentityFunctionCall(
        exampleOptionalBasicTypeI16TwinSyncSse, <int?>[null, 0, -32768, 32767]);
    addTestsIdentityFunctionCall(exampleOptionalBasicTypeI32TwinSyncSse,
        <int?>[null, 0, -2147483648, 2147483647]);
    addTestsIdentityFunctionCall(exampleOptionalBasicTypeI64TwinSyncSse,
        <int?>[null, 0, -9007199254740992, 9007199254740992]);
    addTestsIdentityFunctionCall(
        exampleOptionalBasicTypeU8TwinSyncSse, <int?>[null, 0, 255]);
    addTestsIdentityFunctionCall(
        exampleOptionalBasicTypeU16TwinSyncSse, <int?>[null, 0, 65535]);
    addTestsIdentityFunctionCall(
        exampleOptionalBasicTypeU32TwinSyncSse, <int?>[null, 0, 4294967295]);
    addTestsIdentityFunctionCall(exampleOptionalBasicTypeU64TwinSyncSse,
        <int?>[null, 0, 9007199254740992]);
    addTestsIdentityFunctionCall(exampleOptionalBasicTypeF32TwinSyncSse,
        <double?>[null, 0, -42.5, 123456]);
    addTestsIdentityFunctionCall(exampleOptionalBasicTypeF64TwinSyncSse,
        <double?>[null, 0, -42.5, 123456]);
    addTestsIdentityFunctionCall(
        exampleOptionalBasicTypeBoolTwinSyncSse, <bool?>[null, false, true]);
  });
}
