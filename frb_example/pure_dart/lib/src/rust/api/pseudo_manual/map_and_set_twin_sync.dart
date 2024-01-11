// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.16.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../auxiliary/sample_types.dart';
import '../../frb_generated.dart';
import 'enumeration_twin_sync.dart';
import 'misc_example_twin_sync.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Map<int, int> funcHashMapI32I32TwinSync(
        {required Map<int, int> arg, dynamic hint}) =>
    RustLib.instance.api.funcHashMapI32I32TwinSync(arg: arg, hint: hint);

Set<int> funcHashSetI32TwinSync({required Set<int> arg, dynamic hint}) =>
    RustLib.instance.api.funcHashSetI32TwinSync(arg: arg, hint: hint);

Map<String, String> funcHashMapStringStringTwinSync(
        {required Map<String, String> arg, dynamic hint}) =>
    RustLib.instance.api.funcHashMapStringStringTwinSync(arg: arg, hint: hint);

Set<String> funcHashSetStringTwinSync(
        {required Set<String> arg, dynamic hint}) =>
    RustLib.instance.api.funcHashSetStringTwinSync(arg: arg, hint: hint);

Map<String, Uint8List> funcHashMapStringBytesTwinSync(
        {required Map<String, Uint8List> arg, dynamic hint}) =>
    RustLib.instance.api.funcHashMapStringBytesTwinSync(arg: arg, hint: hint);

Map<String, MySize> funcHashMapStringStructTwinSync(
        {required Map<String, MySize> arg, dynamic hint}) =>
    RustLib.instance.api.funcHashMapStringStructTwinSync(arg: arg, hint: hint);

Map<String, EnumSimpleTwinSync> funcHashMapStringSimpleEnumTwinSync(
        {required Map<String, EnumSimpleTwinSync> arg, dynamic hint}) =>
    RustLib.instance.api
        .funcHashMapStringSimpleEnumTwinSync(arg: arg, hint: hint);

Map<String, KitchenSinkTwinSync> funcHashMapStringComplexEnumTwinSync(
        {required Map<String, KitchenSinkTwinSync> arg, dynamic hint}) =>
    RustLib.instance.api
        .funcHashMapStringComplexEnumTwinSync(arg: arg, hint: hint);
