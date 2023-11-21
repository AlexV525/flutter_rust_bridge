import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/ffigen_generated/multi_package.dart';
import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class GeneralizedFrbRustBinding {
  final MultiPackageCBinding _binding;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  GeneralizedFrbRustBinding(ExternalLibrary externalLibrary) : _binding = MultiPackageCBinding(externalLibrary);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject() {
    try {
      _binding.store_dart_post_cobject(ffi.NativeApi.postCObject.cast());
    } on ArgumentError catch (e, s) {
      _userFriendlyDynamicLibraryErrorReporting(e, s);
      rethrow;
    }
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void initFrbDartApiDl() => _binding.init_frb_dart_api_dl(ffi.NativeApi.initializeApiDLData);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object getDartObject(int ptr) => _binding.get_dart_object(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dropDartObject(int ptr) => _binding.drop_dart_object(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  int newDartOpaque(Object obj) => _binding.new_dart_opaque(obj);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncReturn(WireSyncReturn val) => _binding.free_wire_sync_return(val);
}

void _userFriendlyDynamicLibraryErrorReporting(ArgumentError e, StackTrace s) {
  final message = e.message;
  if (message is String && message.contains('Failed to lookup symbol')) {
    throw ArgumentError(
      '$e\n'
      'This is often because the Rust library is not loaded correctly.\n'
      'Original stack trace: $s',
    );
  }
}
