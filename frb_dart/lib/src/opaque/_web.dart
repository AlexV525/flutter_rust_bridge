import 'package:flutter_rust_bridge/src/platform_types/_web.dart';

/// {@macro flutter_rust_bridge.internal}
typedef OpaqueTypeFinalizer = Finalizer<PlatformPointer>;

/// {@macro flutter_rust_bridge.internal}
class FrbOpaqueBase {
  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer initPtr(int ptr) => ptr;

  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer nullPtr() => 0;

  /// {@macro flutter_rust_bridge.internal}
  static bool isStalePtr(PlatformPointer ptr) => ptr == 0;

  /// {@macro flutter_rust_bridge.internal}
  static void finalizerAttach(FrbOpaqueBase opaque, PlatformPointer ptr, int _,
          OpaqueTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque);
}
