// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `basic_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('basic', () {
    addTestsIdentityFunctionCall(
        exampleBasicTypeI8TwinSse, <int>[0, -128, 127]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeI16TwinSse, <int>[0, -32768, 32767]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeI32TwinSse, <int>[0, -2147483648, 2147483647]);
    addTestsIdentityFunctionCall(exampleBasicTypeI64TwinSse,
        <int>[0, -9007199254740992, 9007199254740992]);
    addTestsIdentityFunctionCall(exampleBasicTypeU8TwinSse, <int>[0, 255]);
    addTestsIdentityFunctionCall(exampleBasicTypeU16TwinSse, <int>[0, 65535]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeU32TwinSse, <int>[0, 4294967295]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeU64TwinSse, <int>[0, 9007199254740992]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeF32TwinSse, <double>[0, -42.5, 123456]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeF64TwinSse, <double>[0, -42.5, 123456]);
    addTestsIdentityFunctionCall(
        exampleBasicTypeBoolTwinSse, <bool>[false, true]);
  });
}
