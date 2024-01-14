// ignore_for_file: non_constant_identifier_names

import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:js/js.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class GeneralizedFrbRustBinding {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  GeneralizedFrbRustBinding(ExternalLibrary externalLibrary);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void initFrbDartApiDl() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void pdeFfiDispatcher() {
    throw UnimplementedError("TODO js");
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  PlatformPointer dartOpaqueDart2RustEncode(
          Object object, NativePortType dartHandlerPort) =>
      _dart_opaque_dart2rust_encode(object, dartHandlerPort);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object dartOpaqueRust2DartDecode(int ptr) =>
      _dart_opaque_rust2dart_decode(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dartOpaqueDropThreadBoxPersistentHandle(int ptr) =>
      _dart_opaque_drop_thread_box_persistent_handle(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncRust2DartDco(WireSyncRust2DartDco raw) {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncRust2DartSse(WireSyncRust2DartDco raw) {}
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.dart_opaque_dart2rust_encode")
external int _dart_opaque_dart2rust_encode(
    Object object, NativePortType dartHandlerPort);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.dart_opaque_rust2dart_decode")
external Object _dart_opaque_rust2dart_decode(int ptr);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.dart_opaque_drop_thread_box_persistent_handle")
external void _dart_opaque_drop_thread_box_persistent_handle(int ptr);
