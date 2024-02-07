// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.23.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<DroppableTwinSyncSse>>
@sealed
class DroppableTwinSyncSse extends RustOpaque {
  DroppableTwinSyncSse.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  DroppableTwinSyncSse.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_DroppableTwinSyncSse,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_DroppableTwinSyncSse,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_DroppableTwinSyncSsePtr,
  );

  Stream<int> createStreamTwinSyncSse({dynamic hint}) =>
      RustLib.instance.api.droppableTwinSyncSseCreateStreamTwinSyncSse(
        that: this,
      );

  static int getDropCountTwinSyncSse({dynamic hint}) => RustLib.instance.api
      .droppableTwinSyncSseGetDropCountTwinSyncSse(hint: hint);

  static DroppableTwinSyncSse newTwinSyncSse({dynamic hint}) =>
      RustLib.instance.api.droppableTwinSyncSseNewTwinSyncSse(hint: hint);

  void simpleMethodTwinSyncSse({dynamic hint}) =>
      RustLib.instance.api.droppableTwinSyncSseSimpleMethodTwinSyncSse(
        that: this,
      );
}
