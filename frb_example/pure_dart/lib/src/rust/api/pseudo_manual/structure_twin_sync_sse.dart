// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.4.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

StructWithZeroFieldTwinSyncSse funcStructWithZeroFieldTwinSyncSse(
        {required StructWithZeroFieldTwinSyncSse arg, dynamic hint}) =>
    RustLib.instance.api
        .funcStructWithZeroFieldTwinSyncSse(arg: arg, hint: hint);

StructWithOneFieldTwinSyncSse funcStructWithOneFieldTwinSyncSse(
        {required StructWithOneFieldTwinSyncSse arg, dynamic hint}) =>
    RustLib.instance.api
        .funcStructWithOneFieldTwinSyncSse(arg: arg, hint: hint);

StructWithTwoFieldTwinSyncSse funcStructWithTwoFieldTwinSyncSse(
        {required StructWithTwoFieldTwinSyncSse arg, dynamic hint}) =>
    RustLib.instance.api
        .funcStructWithTwoFieldTwinSyncSse(arg: arg, hint: hint);

TupleStructWithOneFieldTwinSyncSse funcTupleStructWithOneFieldTwinSyncSse(
        {required TupleStructWithOneFieldTwinSyncSse arg, dynamic hint}) =>
    RustLib.instance.api
        .funcTupleStructWithOneFieldTwinSyncSse(arg: arg, hint: hint);

TupleStructWithTwoFieldTwinSyncSse funcTupleStructWithTwoFieldTwinSyncSse(
        {required TupleStructWithTwoFieldTwinSyncSse arg, dynamic hint}) =>
    RustLib.instance.api
        .funcTupleStructWithTwoFieldTwinSyncSse(arg: arg, hint: hint);

class StructWithOneFieldTwinSyncSse {
  final int a;

  const StructWithOneFieldTwinSyncSse({
    required this.a,
  });

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithOneFieldTwinSyncSse &&
          runtimeType == other.runtimeType &&
          a == other.a;
}

class StructWithTwoFieldTwinSyncSse {
  final int a;
  final int b;

  const StructWithTwoFieldTwinSyncSse({
    required this.a,
    required this.b,
  });

  @override
  int get hashCode => a.hashCode ^ b.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithTwoFieldTwinSyncSse &&
          runtimeType == other.runtimeType &&
          a == other.a &&
          b == other.b;
}

class StructWithZeroFieldTwinSyncSse {
  const StructWithZeroFieldTwinSyncSse();

  @override
  int get hashCode => 0;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithZeroFieldTwinSyncSse &&
          runtimeType == other.runtimeType;
}

class TupleStructWithOneFieldTwinSyncSse {
  final int field0;

  const TupleStructWithOneFieldTwinSyncSse({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TupleStructWithOneFieldTwinSyncSse &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class TupleStructWithTwoFieldTwinSyncSse {
  final int field0;
  final int field1;

  const TupleStructWithTwoFieldTwinSyncSse({
    required this.field0,
    required this.field1,
  });

  @override
  int get hashCode => field0.hashCode ^ field1.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TupleStructWithTwoFieldTwinSyncSse &&
          runtimeType == other.runtimeType &&
          field0 == other.field0 &&
          field1 == other.field1;
}
