// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.77.1.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated_api_block_3.dart';
export 'bridge_generated_api_block_3.dart';
import 'bridge_generated_shares.web.dart';

class ApiBlock3ClassPlatform extends FlutterRustBridgeBase<ApiBlock3ClassWire> with FlutterRustBridgeSetupMixin {
  final BridgeGeneratedSharesPlatform _sharedPlatform;
  ApiBlock3ClassPlatform(FutureOr<WasmModule> dylib)
      : _sharedPlatform = BridgeGeneratedSharesPlatform(dylib),
        super(ApiBlock3ClassWire(dylib)) {
    setupMixinConstructor();
  }
  Future<void> setup() => inner.init;

// Section: api2wire

  @protected
  List<dynamic> api2wire_box_autoadd_struct_defined_in_block_3(StructDefinedInBlock3 raw) {
    return api2wire_struct_defined_in_block_3(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_struct_only_for_block_3(StructOnlyForBlock3 raw) {
    return api2wire_struct_only_for_block_3(raw);
  }

  @protected
  Object api2wire_i64(int raw) {
    return castNativeBigInt(raw);
  }

  @protected
  List<dynamic>? api2wire_opt_box_autoadd_shared_struct_in_all_blocks(SharedStructInAllBlocks? raw) {
    return raw == null ? null : _sharedPlatform.api2wire_box_autoadd_shared_struct_in_all_blocks(raw);
  }

  @protected
  List<dynamic> api2wire_struct_defined_in_block_3(StructDefinedInBlock3 raw) {
    return [_sharedPlatform.api2wire_String(raw.name)];
  }

  @protected
  List<dynamic> api2wire_struct_only_for_block_3(StructOnlyForBlock3 raw) {
    return [api2wire_i64(raw.id), api2wire_f64(raw.num), _sharedPlatform.api2wire_String(raw.name)];
  }
// Section: finalizer
}

// Section: WASM wire module

@JS('wasm_bindgen')
external ApiBlock3ClassWasmModule get wasmModule;

@JS()
@anonymous
class ApiBlock3ClassWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);
  external ApiBlock3ClassWasmModule bind(dynamic thisArg, String moduleName);
  external dynamic /* void */ wire_test_inbuilt_type_in_block_3(NativePortType port_, int a, double b);

  external dynamic /* void */ wire_test_string_in_block_3(NativePortType port_, String s, Object i);

  external dynamic /* void */ wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3(
      NativePortType port_, String name, double score);

  external dynamic /* void */ wire_test_shared_struct_only_for_sync_as_input_with_no_sync_return_in_block_3(
      NativePortType port_, List<dynamic> obj, double default_score);

  external dynamic /* void */ wire_test_all_shared_struct_in_block_3(
      NativePortType port_, List<dynamic>? custom, String s, int i);

  external dynamic /* void */ wire_test_shared_struct_in_block_3_for_2_and_3(
      NativePortType port_, List<dynamic> custom, String s, int i);

  external dynamic /* void */ wire_test_cross_shared_struct_in_block_3_for_2_and_3(NativePortType port_, String name);

  external dynamic /* List<dynamic> */ wire_test_cross_shared_struct_in_sync_in_block_3_for_2_and_3(String name);

  external dynamic /* void */ wire_test_unique_struct_3(NativePortType port_, List<dynamic> custom, String s, Object i);

  external dynamic /* void */ wire_test_struct_defined_in_block_3(NativePortType port_, List<dynamic> custom);

  external dynamic /* void */ wire_test_method__method__StructDefinedInBlock3(
      NativePortType port_, List<dynamic> that, String message);

  external dynamic /* void */ wire_test_static_method__static_method__StructDefinedInBlock3(
      NativePortType port_, String message);

  external dynamic /* void */ wire_test_method__method__StructOnlyForBlock3(
      NativePortType port_, List<dynamic> that, String message, int num);

  external dynamic /* void */ wire_test_static_method__static_method__StructOnlyForBlock3(
      NativePortType port_, String message);

  external dynamic /* void */ wire_test_enum_method__method__Weekdays(NativePortType port_, int that, String message);

  external dynamic /* void */ wire_test_static_enum_method__static_method__Weekdays(
      NativePortType port_, String message);

  external dynamic /* void */ wire_test_method__method__SharedStructInBlock2And3(
      NativePortType port_, List<dynamic> that, String message);

  external dynamic /* void */ wire_test_static_method__static_method__SharedStructInBlock2And3(
      NativePortType port_, String message);

  external dynamic /* void */ wire_test_method__method__SharedStructInAllBlocks(
      NativePortType port_, List<dynamic> that, String message, int num);

  external dynamic /* void */ wire_test_static_method__static_method__SharedStructInAllBlocks(
      NativePortType port_, String message);

  external dynamic /* void */ wire_test_method__method__SharedStructOnlyForSyncTest(
      NativePortType port_, List<dynamic> that, String message);

  external dynamic /* void */ wire_test_static_method__static_method__SharedStructOnlyForSyncTest(
      NativePortType port_, String message);

  external dynamic /* void */ wire_test_method__method__CrossSharedStructInBlock2And3(
      NativePortType port_, List<dynamic> that, String message);

  external dynamic /* void */ wire_test_static_method__static_method__CrossSharedStructInBlock2And3(
      NativePortType port_, String message);
}

// Section: WASM wire connector

class ApiBlock3ClassWire extends FlutterRustBridgeWasmWireBase<ApiBlock3ClassWasmModule> {
  ApiBlock3ClassWire(FutureOr<WasmModule> module) : super(WasmModule.cast<ApiBlock3ClassWasmModule>(module));

  void wire_test_inbuilt_type_in_block_3(NativePortType port_, int a, double b) =>
      wasmModule.wire_test_inbuilt_type_in_block_3(port_, a, b);

  void wire_test_string_in_block_3(NativePortType port_, String s, Object i) =>
      wasmModule.wire_test_string_in_block_3(port_, s, i);

  void wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3(
          NativePortType port_, String name, double score) =>
      wasmModule.wire_test_shared_struct_only_for_sync_with_no_sync_return_in_block_3(port_, name, score);

  void wire_test_shared_struct_only_for_sync_as_input_with_no_sync_return_in_block_3(
          NativePortType port_, List<dynamic> obj, double default_score) =>
      wasmModule.wire_test_shared_struct_only_for_sync_as_input_with_no_sync_return_in_block_3(
          port_, obj, default_score);

  void wire_test_all_shared_struct_in_block_3(NativePortType port_, List<dynamic>? custom, String s, int i) =>
      wasmModule.wire_test_all_shared_struct_in_block_3(port_, custom, s, i);

  void wire_test_shared_struct_in_block_3_for_2_and_3(NativePortType port_, List<dynamic> custom, String s, int i) =>
      wasmModule.wire_test_shared_struct_in_block_3_for_2_and_3(port_, custom, s, i);

  void wire_test_cross_shared_struct_in_block_3_for_2_and_3(NativePortType port_, String name) =>
      wasmModule.wire_test_cross_shared_struct_in_block_3_for_2_and_3(port_, name);

  dynamic /* List<dynamic> */ wire_test_cross_shared_struct_in_sync_in_block_3_for_2_and_3(String name) =>
      wasmModule.wire_test_cross_shared_struct_in_sync_in_block_3_for_2_and_3(name);

  void wire_test_unique_struct_3(NativePortType port_, List<dynamic> custom, String s, Object i) =>
      wasmModule.wire_test_unique_struct_3(port_, custom, s, i);

  void wire_test_struct_defined_in_block_3(NativePortType port_, List<dynamic> custom) =>
      wasmModule.wire_test_struct_defined_in_block_3(port_, custom);

  void wire_test_method__method__StructDefinedInBlock3(NativePortType port_, List<dynamic> that, String message) =>
      wasmModule.wire_test_method__method__StructDefinedInBlock3(port_, that, message);

  void wire_test_static_method__static_method__StructDefinedInBlock3(NativePortType port_, String message) =>
      wasmModule.wire_test_static_method__static_method__StructDefinedInBlock3(port_, message);

  void wire_test_method__method__StructOnlyForBlock3(
          NativePortType port_, List<dynamic> that, String message, int num) =>
      wasmModule.wire_test_method__method__StructOnlyForBlock3(port_, that, message, num);

  void wire_test_static_method__static_method__StructOnlyForBlock3(NativePortType port_, String message) =>
      wasmModule.wire_test_static_method__static_method__StructOnlyForBlock3(port_, message);

  void wire_test_enum_method__method__Weekdays(NativePortType port_, int that, String message) =>
      wasmModule.wire_test_enum_method__method__Weekdays(port_, that, message);

  void wire_test_static_enum_method__static_method__Weekdays(NativePortType port_, String message) =>
      wasmModule.wire_test_static_enum_method__static_method__Weekdays(port_, message);

  void wire_test_method__method__SharedStructInBlock2And3(NativePortType port_, List<dynamic> that, String message) =>
      wasmModule.wire_test_method__method__SharedStructInBlock2And3(port_, that, message);

  void wire_test_static_method__static_method__SharedStructInBlock2And3(NativePortType port_, String message) =>
      wasmModule.wire_test_static_method__static_method__SharedStructInBlock2And3(port_, message);

  void wire_test_method__method__SharedStructInAllBlocks(
          NativePortType port_, List<dynamic> that, String message, int num) =>
      wasmModule.wire_test_method__method__SharedStructInAllBlocks(port_, that, message, num);

  void wire_test_static_method__static_method__SharedStructInAllBlocks(NativePortType port_, String message) =>
      wasmModule.wire_test_static_method__static_method__SharedStructInAllBlocks(port_, message);

  void wire_test_method__method__SharedStructOnlyForSyncTest(
          NativePortType port_, List<dynamic> that, String message) =>
      wasmModule.wire_test_method__method__SharedStructOnlyForSyncTest(port_, that, message);

  void wire_test_static_method__static_method__SharedStructOnlyForSyncTest(NativePortType port_, String message) =>
      wasmModule.wire_test_static_method__static_method__SharedStructOnlyForSyncTest(port_, message);

  void wire_test_method__method__CrossSharedStructInBlock2And3(
          NativePortType port_, List<dynamic> that, String message) =>
      wasmModule.wire_test_method__method__CrossSharedStructInBlock2And3(port_, that, message);

  void wire_test_static_method__static_method__CrossSharedStructInBlock2And3(NativePortType port_, String message) =>
      wasmModule.wire_test_static_method__static_method__CrossSharedStructInBlock2And3(port_, message);
}
