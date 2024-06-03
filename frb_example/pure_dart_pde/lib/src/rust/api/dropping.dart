// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.37.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DroppableTwinNormal>>
@sealed
class DroppableTwinNormal extends RustOpaque {
  // Not to be used by end users
  DroppableTwinNormal.frbInternalDcoDecode(List<dynamic> wire)
      : super.frbInternalDcoDecode(wire, _kStaticData);

  // Not to be used by end users
  DroppableTwinNormal.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative)
      : super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_DroppableTwinNormal,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_DroppableTwinNormal,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_DroppableTwinNormalPtr,
  );

  Stream<int> createStream() =>
      RustLib.instance.api.crateApiDroppingDroppableTwinNormalCreateStream(
        that: this,
      );

  Future<void> drop() =>
      RustLib.instance.api.crateApiDroppingDroppableTwinNormalDrop(
        that: this,
      );

  static Future<int> getDropCountTwinNormal() => RustLib.instance.api
      .crateApiDroppingDroppableTwinNormalGetDropCountTwinNormal();

  static Future<DroppableTwinNormal> newTwinNormal() =>
      RustLib.instance.api.crateApiDroppingDroppableTwinNormalNewTwinNormal();

  Future<void> simpleMethodTwinNormal() => RustLib.instance.api
          .crateApiDroppingDroppableTwinNormalSimpleMethodTwinNormal(
        that: this,
      );
}
