import 'dart:ffi';

import 'package:flutter_rust_bridge/src/dart_c_object_into_dart/_io.dart';
import 'package:flutter_rust_bridge/src/platform_types_io.dart';

/// Generates the dynamic Dart object from either an FFI struct or a JS value
///
/// {@macro flutter_rust_bridge.internal}
List<dynamic> wireSyncReturnIntoDart(WireSyncReturn syncReturn) => dartCObjectIntoDart(syncReturn.ref);

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt wire2apiI64OrU64(dynamic raw) => BigInt.from(raw as int);
