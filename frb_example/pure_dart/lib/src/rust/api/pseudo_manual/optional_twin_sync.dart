// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.4.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'misc_example_twin_sync.dart';
import 'newtype_pattern_twin_sync.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

double? handleOptionalReturnTwinSync(
        {required double left, required double right, dynamic hint}) =>
    RustLib.instance.api
        .handleOptionalReturnTwinSync(left: left, right: right, hint: hint);

ElementTwinSync? handleOptionalStructTwinSync(
        {String? document, dynamic hint}) =>
    RustLib.instance.api
        .handleOptionalStructTwinSync(document: document, hint: hint);

ExoticOptionalsTwinSync? handleOptionalIncrementTwinSync(
        {ExoticOptionalsTwinSync? opt, dynamic hint}) =>
    RustLib.instance.api.handleOptionalIncrementTwinSync(opt: opt, hint: hint);

double handleIncrementBoxedOptionalTwinSync({double? opt, dynamic hint}) =>
    RustLib.instance.api
        .handleIncrementBoxedOptionalTwinSync(opt: opt, hint: hint);

OptVecsTwinSync handleVecOfOptsTwinSync(
        {required OptVecsTwinSync opt, dynamic hint}) =>
    RustLib.instance.api.handleVecOfOptsTwinSync(opt: opt, hint: hint);

String handleOptionBoxArgumentsTwinSync(
        {int? i8Box,
        int? u8Box,
        int? i32Box,
        int? i64Box,
        double? f64Box,
        bool? boolbox,
        ExoticOptionalsTwinSync? structbox,
        dynamic hint}) =>
    RustLib.instance.api.handleOptionBoxArgumentsTwinSync(
        i8Box: i8Box,
        u8Box: u8Box,
        i32Box: i32Box,
        i64Box: i64Box,
        f64Box: f64Box,
        boolbox: boolbox,
        structbox: structbox,
        hint: hint);

class AttributeTwinSync {
  final String key;
  final String value;

  const AttributeTwinSync({
    required this.key,
    required this.value,
  });

  @override
  int get hashCode => key.hashCode ^ value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AttributeTwinSync &&
          runtimeType == other.runtimeType &&
          key == other.key &&
          value == other.value;
}

class ElementTwinSync {
  final String? tag;
  final String? text;
  final List<AttributeTwinSync>? attributes;
  final List<ElementTwinSync>? children;

  const ElementTwinSync({
    this.tag,
    this.text,
    this.attributes,
    this.children,
  });

  @override
  int get hashCode =>
      tag.hashCode ^ text.hashCode ^ attributes.hashCode ^ children.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ElementTwinSync &&
          runtimeType == other.runtimeType &&
          tag == other.tag &&
          text == other.text &&
          attributes == other.attributes &&
          children == other.children;
}

class ExoticOptionalsTwinSync {
  final int? int32;
  final int? int64;
  final double? float64;
  final bool? boolean;
  final Uint8List? zerocopy;
  final Int8List? int8List;
  final Uint8List? uint8List;
  final Int32List? int32List;
  final Float32List? float32List;
  final Float64List? float64List;
  final List<AttributeTwinSync>? attributes;
  final List<AttributeTwinSync?> attributesNullable;
  final List<AttributeTwinSync?>? nullableAttributes;
  final NewTypeIntTwinSync? newtypeint;

  const ExoticOptionalsTwinSync({
    this.int32,
    this.int64,
    this.float64,
    this.boolean,
    this.zerocopy,
    this.int8List,
    this.uint8List,
    this.int32List,
    this.float32List,
    this.float64List,
    this.attributes,
    required this.attributesNullable,
    this.nullableAttributes,
    this.newtypeint,
  });

  @override
  int get hashCode =>
      int32.hashCode ^
      int64.hashCode ^
      float64.hashCode ^
      boolean.hashCode ^
      zerocopy.hashCode ^
      int8List.hashCode ^
      uint8List.hashCode ^
      int32List.hashCode ^
      float32List.hashCode ^
      float64List.hashCode ^
      attributes.hashCode ^
      attributesNullable.hashCode ^
      nullableAttributes.hashCode ^
      newtypeint.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ExoticOptionalsTwinSync &&
          runtimeType == other.runtimeType &&
          int32 == other.int32 &&
          int64 == other.int64 &&
          float64 == other.float64 &&
          boolean == other.boolean &&
          zerocopy == other.zerocopy &&
          int8List == other.int8List &&
          uint8List == other.uint8List &&
          int32List == other.int32List &&
          float32List == other.float32List &&
          float64List == other.float64List &&
          attributes == other.attributes &&
          attributesNullable == other.attributesNullable &&
          nullableAttributes == other.nullableAttributes &&
          newtypeint == other.newtypeint;
}

class OptVecsTwinSync {
  final List<int?> i32;
  final List<WeekdaysTwinSync?> enums;
  final List<String?> strings;
  final List<Int32List?> buffers;

  const OptVecsTwinSync({
    required this.i32,
    required this.enums,
    required this.strings,
    required this.buffers,
  });

  @override
  int get hashCode =>
      i32.hashCode ^ enums.hashCode ^ strings.hashCode ^ buffers.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is OptVecsTwinSync &&
          runtimeType == other.runtimeType &&
          i32 == other.i32 &&
          enums == other.enums &&
          strings == other.strings &&
          buffers == other.buffers;
}
