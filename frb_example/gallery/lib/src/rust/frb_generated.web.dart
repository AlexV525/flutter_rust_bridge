// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.16.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables

import 'api/mandelbrot.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw);

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  Point dco_decode_box_autoadd_point(dynamic raw);

  @protected
  Size dco_decode_box_autoadd_size(dynamic raw);

  @protected
  double dco_decode_f_64(dynamic raw);

  @protected
  int dco_decode_i_32(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  Point dco_decode_point(dynamic raw);

  @protected
  Size dco_decode_size(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer);

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  Point sse_decode_box_autoadd_point(SseDeserializer deserializer);

  @protected
  Size sse_decode_box_autoadd_size(SseDeserializer deserializer);

  @protected
  double sse_decode_f_64(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  Point sse_decode_point(SseDeserializer deserializer);

  @protected
  Size sse_decode_size(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  String cst_encode_AnyhowException(AnyhowException raw) {
    throw UnimplementedError();
  }

  @protected
  String cst_encode_String(String raw) {
    return raw;
  }

  @protected
  List<dynamic> cst_encode_box_autoadd_point(Point raw) {
    return cst_encode_point(raw);
  }

  @protected
  List<dynamic> cst_encode_box_autoadd_size(Size raw) {
    return cst_encode_size(raw);
  }

  @protected
  Uint8List cst_encode_list_prim_u_8_strict(Uint8List raw) {
    return raw;
  }

  @protected
  List<dynamic> cst_encode_point(Point raw) {
    return [cst_encode_f_64(raw.x), cst_encode_f_64(raw.y)];
  }

  @protected
  List<dynamic> cst_encode_size(Size raw) {
    return [cst_encode_i_32(raw.width), cst_encode_i_32(raw.height)];
  }

  @protected
  double cst_encode_f_64(double raw);

  @protected
  int cst_encode_i_32(int raw);

  @protected
  int cst_encode_u_8(int raw);

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer);

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_point(Point self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_size(Size self, SseSerializer serializer);

  @protected
  void sse_encode_f_64(double self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer);

  @protected
  void sse_encode_point(Point self, SseSerializer serializer);

  @protected
  void sse_encode_size(Size self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);
}

// Section: wire_class

class RustLibWire extends BaseWire {
  RustLibWire.fromExternalLibrary(ExternalLibrary lib);

  void dart_fn_deliver_output(int call_id, PlatformGeneralizedUint8ListPtr ptr_,
          int rust_vec_len_, int data_len_) =>
      wasmModule.dart_fn_deliver_output(
          call_id, ptr_, rust_vec_len_, data_len_);

  void wire_draw_mandelbrot(NativePortType port_, List<dynamic> image_size,
          List<dynamic> zoom_point, double scale, int num_threads) =>
      wasmModule.wire_draw_mandelbrot(
          port_, image_size, zoom_point, scale, num_threads);
}

@JS('wasm_bindgen')
external RustLibWasmModule get wasmModule;

@JS()
@anonymous
class RustLibWasmModule implements WasmModule {
  @override
  external Object /* Promise */ call([String? moduleName]);

  @override
  external RustLibWasmModule bind(dynamic thisArg, String moduleName);

  external void dart_fn_deliver_output(int call_id,
      PlatformGeneralizedUint8ListPtr ptr_, int rust_vec_len_, int data_len_);

  external void wire_draw_mandelbrot(
      NativePortType port_,
      List<dynamic> image_size,
      List<dynamic> zoom_point,
      double scale,
      int num_threads);
}
