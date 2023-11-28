// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<U8Array5> getArrayTwinNormal({dynamic hint}) =>
    RustLib.instance.api.getArrayTwinNormal(hint: hint);

Future<PointTwinNormalArray2> getComplexArrayTwinNormal({dynamic hint}) =>
    RustLib.instance.api.getComplexArrayTwinNormal(hint: hint);

Future<MessageIdTwinNormal> newMsgidTwinNormal(
        {required U8Array32 id, dynamic hint}) =>
    RustLib.instance.api.newMsgidTwinNormal(id: id, hint: hint);

Future<U8Array32> useMsgidTwinNormal(
        {required MessageIdTwinNormal id, dynamic hint}) =>
    RustLib.instance.api.useMsgidTwinNormal(id: id, hint: hint);

Future<BlobTwinNormal> boxedBlobTwinNormal(
        {required U8Array1600 blob, dynamic hint}) =>
    RustLib.instance.api.boxedBlobTwinNormal(blob: blob, hint: hint);

Future<U8Array1600> useBoxedBlobTwinNormal(
        {required BlobTwinNormal blob, dynamic hint}) =>
    RustLib.instance.api.useBoxedBlobTwinNormal(blob: blob, hint: hint);

Future<FeedIdTwinNormal> returnBoxedFeedIdTwinNormal(
        {required U8Array8 id, dynamic hint}) =>
    RustLib.instance.api.returnBoxedFeedIdTwinNormal(id: id, hint: hint);

Future<U8Array8> returnBoxedRawFeedIdTwinNormal(
        {required FeedIdTwinNormal id, dynamic hint}) =>
    RustLib.instance.api.returnBoxedRawFeedIdTwinNormal(id: id, hint: hint);

Future<TestIdTwinNormal> funcTestIdTwinNormal(
        {required TestIdTwinNormal id, dynamic hint}) =>
    RustLib.instance.api.funcTestIdTwinNormal(id: id, hint: hint);

Future<double> lastNumberTwinNormal(
        {required F64Array16 array, dynamic hint}) =>
    RustLib.instance.api.lastNumberTwinNormal(array: array, hint: hint);

Future<TestIdTwinNormalArray2> nestedIdTwinNormal(
        {required TestIdTwinNormalArray4 id, dynamic hint}) =>
    RustLib.instance.api.nestedIdTwinNormal(id: id, hint: hint);

class BlobTwinNormal {
  final U8Array1600 field0;

  const BlobTwinNormal({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BlobTwinNormal &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class F64Array16 extends NonGrowableListView<double> {
  static const arraySize = 16;
  F64Array16(Float64List inner)
      : assert(inner.length == arraySize),
        super(inner);
  F64Array16.unchecked(Float64List inner) : super(inner);
  F64Array16.init() : super(Float64List(arraySize));
}

class FeedIdTwinNormal {
  final U8Array8 field0;

  const FeedIdTwinNormal({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeedIdTwinNormal &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class I32Array2 extends NonGrowableListView<int> {
  static const arraySize = 2;
  I32Array2(Int32List inner)
      : assert(inner.length == arraySize),
        super(inner);
  I32Array2.unchecked(Int32List inner) : super(inner);
  I32Array2.init() : super(Int32List(arraySize));
}

class MessageIdTwinNormal {
  final U8Array32 field0;

  const MessageIdTwinNormal({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MessageIdTwinNormal &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class PointTwinNormal {
  final double x;
  final double y;

  const PointTwinNormal({
    required this.x,
    required this.y,
  });

  @override
  int get hashCode => x.hashCode ^ y.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PointTwinNormal &&
          runtimeType == other.runtimeType &&
          x == other.x &&
          y == other.y;
}

class PointTwinNormalArray2 extends NonGrowableListView<PointTwinNormal> {
  static const arraySize = 2;
  PointTwinNormalArray2(List<PointTwinNormal> inner)
      : assert(inner.length == arraySize),
        super(inner);
  PointTwinNormalArray2.unchecked(List<PointTwinNormal> inner) : super(inner);
  PointTwinNormalArray2.init(PointTwinNormal fill)
      : super(List<PointTwinNormal>.filled(arraySize, fill));
}

class TestIdTwinNormal {
  final I32Array2 field0;

  const TestIdTwinNormal({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TestIdTwinNormal &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class TestIdTwinNormalArray2 extends NonGrowableListView<TestIdTwinNormal> {
  static const arraySize = 2;
  TestIdTwinNormalArray2(List<TestIdTwinNormal> inner)
      : assert(inner.length == arraySize),
        super(inner);
  TestIdTwinNormalArray2.unchecked(List<TestIdTwinNormal> inner) : super(inner);
  TestIdTwinNormalArray2.init(TestIdTwinNormal fill)
      : super(List<TestIdTwinNormal>.filled(arraySize, fill));
}

class TestIdTwinNormalArray4 extends NonGrowableListView<TestIdTwinNormal> {
  static const arraySize = 4;
  TestIdTwinNormalArray4(List<TestIdTwinNormal> inner)
      : assert(inner.length == arraySize),
        super(inner);
  TestIdTwinNormalArray4.unchecked(List<TestIdTwinNormal> inner) : super(inner);
  TestIdTwinNormalArray4.init(TestIdTwinNormal fill)
      : super(List<TestIdTwinNormal>.filled(arraySize, fill));
}

class U8Array1600 extends NonGrowableListView<int> {
  static const arraySize = 1600;
  U8Array1600(Uint8List inner)
      : assert(inner.length == arraySize),
        super(inner);
  U8Array1600.unchecked(Uint8List inner) : super(inner);
  U8Array1600.init() : super(Uint8List(arraySize));
}

class U8Array32 extends NonGrowableListView<int> {
  static const arraySize = 32;
  U8Array32(Uint8List inner)
      : assert(inner.length == arraySize),
        super(inner);
  U8Array32.unchecked(Uint8List inner) : super(inner);
  U8Array32.init() : super(Uint8List(arraySize));
}

class U8Array5 extends NonGrowableListView<int> {
  static const arraySize = 5;
  U8Array5(Uint8List inner)
      : assert(inner.length == arraySize),
        super(inner);
  U8Array5.unchecked(Uint8List inner) : super(inner);
  U8Array5.init() : super(Uint8List(arraySize));
}

class U8Array8 extends NonGrowableListView<int> {
  static const arraySize = 8;
  U8Array8(Uint8List inner)
      : assert(inner.length == arraySize),
        super(inner);
  U8Array8.unchecked(Uint8List inner) : super(inner);
  U8Array8.init() : super(Uint8List(arraySize));
}
