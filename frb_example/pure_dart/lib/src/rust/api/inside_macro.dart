// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.4.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<MacroStruct> funcMacroStructTwinNormal(
        {required MacroStruct arg, dynamic hint}) =>
    RustLib.instance.api.funcMacroStructTwinNormal(arg: arg, hint: hint);

Future<AnotherMacroStructTwinNormal> anotherMacroStructTwinNormal(
        {dynamic hint}) =>
    RustLib.instance.api.anotherMacroStructTwinNormal(hint: hint);

class AnotherMacroStructTwinNormal {
  final int data;
  int nonFinalData;

  AnotherMacroStructTwinNormal({
    required this.data,
    required this.nonFinalData,
  });

  @override
  int get hashCode => data.hashCode ^ nonFinalData.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AnotherMacroStructTwinNormal &&
          runtimeType == other.runtimeType &&
          data == other.data &&
          nonFinalData == other.nonFinalData;
}

class MacroStruct {
  final int data;

  const MacroStruct({
    required this.data,
  });

  @override
  int get hashCode => data.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MacroStruct &&
          runtimeType == other.runtimeType &&
          data == other.data;
}
