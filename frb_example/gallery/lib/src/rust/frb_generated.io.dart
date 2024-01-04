// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.11.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables

import 'api/mandelbrot.dart';
import 'api/polars.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_DataFramePtr =>
      wire._rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockDataFramePtr;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_ExprPtr =>
      wire._rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockExprPtr;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_LazyFramePtr =>
      wire._rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyFramePtr;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_LazyGroupByPtr => wire
          ._rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyGroupByPtr;

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw);

  @protected
  DataFrame dco_decode_Auto_Owned_RustOpaque_stdsyncRwLockDataFrame(
      dynamic raw);

  @protected
  Expr dco_decode_Auto_Owned_RustOpaque_stdsyncRwLockExpr(dynamic raw);

  @protected
  LazyFrame dco_decode_Auto_Owned_RustOpaque_stdsyncRwLockLazyFrame(
      dynamic raw);

  @protected
  LazyGroupBy dco_decode_Auto_Owned_RustOpaque_stdsyncRwLockLazyGroupBy(
      dynamic raw);

  @protected
  DataFrame dco_decode_Auto_Ref_RustOpaque_stdsyncRwLockDataFrame(dynamic raw);

  @protected
  DataFrame dco_decode_RustOpaque_stdsyncRwLockDataFrame(dynamic raw);

  @protected
  Expr dco_decode_RustOpaque_stdsyncRwLockExpr(dynamic raw);

  @protected
  LazyFrame dco_decode_RustOpaque_stdsyncRwLockLazyFrame(dynamic raw);

  @protected
  LazyGroupBy dco_decode_RustOpaque_stdsyncRwLockLazyGroupBy(dynamic raw);

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
  List<String> dco_decode_list_String(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  Point dco_decode_point(dynamic raw);

  @protected
  Size dco_decode_size(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  int dco_decode_usize(dynamic raw);

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer);

  @protected
  DataFrame sse_decode_Auto_Owned_RustOpaque_stdsyncRwLockDataFrame(
      SseDeserializer deserializer);

  @protected
  Expr sse_decode_Auto_Owned_RustOpaque_stdsyncRwLockExpr(
      SseDeserializer deserializer);

  @protected
  LazyFrame sse_decode_Auto_Owned_RustOpaque_stdsyncRwLockLazyFrame(
      SseDeserializer deserializer);

  @protected
  LazyGroupBy sse_decode_Auto_Owned_RustOpaque_stdsyncRwLockLazyGroupBy(
      SseDeserializer deserializer);

  @protected
  DataFrame sse_decode_Auto_Ref_RustOpaque_stdsyncRwLockDataFrame(
      SseDeserializer deserializer);

  @protected
  DataFrame sse_decode_RustOpaque_stdsyncRwLockDataFrame(
      SseDeserializer deserializer);

  @protected
  Expr sse_decode_RustOpaque_stdsyncRwLockExpr(SseDeserializer deserializer);

  @protected
  LazyFrame sse_decode_RustOpaque_stdsyncRwLockLazyFrame(
      SseDeserializer deserializer);

  @protected
  LazyGroupBy sse_decode_RustOpaque_stdsyncRwLockLazyGroupBy(
      SseDeserializer deserializer);

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
  List<String> sse_decode_list_String(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  Point sse_decode_point(SseDeserializer deserializer);

  @protected
  Size sse_decode_size(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  int sse_decode_usize(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_AnyhowException(
      AnyhowException raw) {
    throw UnimplementedError();
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_String(String raw) {
    return cst_encode_list_prim_u_8_strict(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_cst_point> cst_encode_box_autoadd_point(Point raw) {
    final ptr = wire.cst_new_box_autoadd_point();
    cst_api_fill_to_wire_point(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_size> cst_encode_box_autoadd_size(Size raw) {
    final ptr = wire.cst_new_box_autoadd_size();
    cst_api_fill_to_wire_size(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_list_String> cst_encode_list_String(List<String> raw) {
    final ans = wire.cst_new_list_String(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      ans.ref.ptr[i] = cst_encode_String(raw[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_list_prim_u_8_strict(
      Uint8List raw) {
    final ans = wire.cst_new_list_prim_u_8_strict(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_point(
      Point apiObj, ffi.Pointer<wire_cst_point> wireObj) {
    cst_api_fill_to_wire_point(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_size(
      Size apiObj, ffi.Pointer<wire_cst_size> wireObj) {
    cst_api_fill_to_wire_size(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_point(Point apiObj, wire_cst_point wireObj) {
    wireObj.x = cst_encode_f_64(apiObj.x);
    wireObj.y = cst_encode_f_64(apiObj.y);
  }

  @protected
  void cst_api_fill_to_wire_size(Size apiObj, wire_cst_size wireObj) {
    wireObj.width = cst_encode_i_32(apiObj.width);
    wireObj.height = cst_encode_i_32(apiObj.height);
  }

  @protected
  PlatformPointer cst_encode_Auto_Owned_RustOpaque_stdsyncRwLockDataFrame(
      DataFrame raw);

  @protected
  PlatformPointer cst_encode_Auto_Owned_RustOpaque_stdsyncRwLockExpr(Expr raw);

  @protected
  PlatformPointer cst_encode_Auto_Owned_RustOpaque_stdsyncRwLockLazyFrame(
      LazyFrame raw);

  @protected
  PlatformPointer cst_encode_Auto_Owned_RustOpaque_stdsyncRwLockLazyGroupBy(
      LazyGroupBy raw);

  @protected
  PlatformPointer cst_encode_Auto_Ref_RustOpaque_stdsyncRwLockDataFrame(
      DataFrame raw);

  @protected
  PlatformPointer cst_encode_RustOpaque_stdsyncRwLockDataFrame(DataFrame raw);

  @protected
  PlatformPointer cst_encode_RustOpaque_stdsyncRwLockExpr(Expr raw);

  @protected
  PlatformPointer cst_encode_RustOpaque_stdsyncRwLockLazyFrame(LazyFrame raw);

  @protected
  PlatformPointer cst_encode_RustOpaque_stdsyncRwLockLazyGroupBy(
      LazyGroupBy raw);

  @protected
  double cst_encode_f_64(double raw);

  @protected
  int cst_encode_i_32(int raw);

  @protected
  int cst_encode_u_8(int raw);

  @protected
  void cst_encode_unit(void raw);

  @protected
  int cst_encode_usize(int raw);

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer);

  @protected
  void sse_encode_Auto_Owned_RustOpaque_stdsyncRwLockDataFrame(
      DataFrame self, SseSerializer serializer);

  @protected
  void sse_encode_Auto_Owned_RustOpaque_stdsyncRwLockExpr(
      Expr self, SseSerializer serializer);

  @protected
  void sse_encode_Auto_Owned_RustOpaque_stdsyncRwLockLazyFrame(
      LazyFrame self, SseSerializer serializer);

  @protected
  void sse_encode_Auto_Owned_RustOpaque_stdsyncRwLockLazyGroupBy(
      LazyGroupBy self, SseSerializer serializer);

  @protected
  void sse_encode_Auto_Ref_RustOpaque_stdsyncRwLockDataFrame(
      DataFrame self, SseSerializer serializer);

  @protected
  void sse_encode_RustOpaque_stdsyncRwLockDataFrame(
      DataFrame self, SseSerializer serializer);

  @protected
  void sse_encode_RustOpaque_stdsyncRwLockExpr(
      Expr self, SseSerializer serializer);

  @protected
  void sse_encode_RustOpaque_stdsyncRwLockLazyFrame(
      LazyFrame self, SseSerializer serializer);

  @protected
  void sse_encode_RustOpaque_stdsyncRwLockLazyGroupBy(
      LazyGroupBy self, SseSerializer serializer);

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
  void sse_encode_list_String(List<String> self, SseSerializer serializer);

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
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_usize(int self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);
}

// Section: wire_class

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names
// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class RustLibWire implements BaseWire {
  factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) =>
      RustLibWire(lib.ffiDynamicLibrary);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  RustLibWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  RustLibWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void dart_fn_deliver_output(
    int call_id,
    ffi.Pointer<ffi.Uint8> ptr_,
    int rust_vec_len_,
    int data_len_,
  ) {
    return _dart_fn_deliver_output(
      call_id,
      ptr_,
      rust_vec_len_,
      data_len_,
    );
  }

  late final _dart_fn_deliver_outputPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int32, ffi.Pointer<ffi.Uint8>, ffi.Int32,
              ffi.Int32)>>('frbgen_frb_example_gallery_dart_fn_deliver_output');
  late final _dart_fn_deliver_output = _dart_fn_deliver_outputPtr
      .asFunction<void Function(int, ffi.Pointer<ffi.Uint8>, int, int)>();

  void wire_draw_mandelbrot(
    int port_,
    ffi.Pointer<wire_cst_size> image_size,
    ffi.Pointer<wire_cst_point> zoom_point,
    double scale,
    int num_threads,
  ) {
    return _wire_draw_mandelbrot(
      port_,
      image_size,
      zoom_point,
      scale,
      num_threads,
    );
  }

  late final _wire_draw_mandelbrotPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_cst_size>,
              ffi.Pointer<wire_cst_point>,
              ffi.Double,
              ffi.Int32)>>('frbgen_frb_example_gallery_wire_draw_mandelbrot');
  late final _wire_draw_mandelbrot = _wire_draw_mandelbrotPtr.asFunction<
      void Function(int, ffi.Pointer<wire_cst_size>,
          ffi.Pointer<wire_cst_point>, double, int)>();

  void wire_DataFrame_get_column(
    int port_,
    ffi.Pointer<ffi.Void> that,
    ffi.Pointer<wire_cst_list_prim_u_8_strict> name,
  ) {
    return _wire_DataFrame_get_column(
      port_,
      that,
      name,
    );
  }

  late final _wire_DataFrame_get_columnPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Void>,
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>)>>(
      'frbgen_frb_example_gallery_wire_DataFrame_get_column');
  late final _wire_DataFrame_get_column =
      _wire_DataFrame_get_columnPtr.asFunction<
          void Function(int, ffi.Pointer<ffi.Void>,
              ffi.Pointer<wire_cst_list_prim_u_8_strict>)>();

  WireSyncRust2DartDco wire_DataFrame_get_column_names(
    ffi.Pointer<ffi.Void> that,
  ) {
    return _wire_DataFrame_get_column_names(
      that,
    );
  }

  late final _wire_DataFrame_get_column_namesPtr = _lookup<
          ffi.NativeFunction<
              WireSyncRust2DartDco Function(ffi.Pointer<ffi.Void>)>>(
      'frbgen_frb_example_gallery_wire_DataFrame_get_column_names');
  late final _wire_DataFrame_get_column_names =
      _wire_DataFrame_get_column_namesPtr
          .asFunction<WireSyncRust2DartDco Function(ffi.Pointer<ffi.Void>)>();

  WireSyncRust2DartDco wire_DataFrame_lazy(
    ffi.Pointer<ffi.Void> that,
  ) {
    return _wire_DataFrame_lazy(
      that,
    );
  }

  late final _wire_DataFrame_lazyPtr = _lookup<
          ffi.NativeFunction<
              WireSyncRust2DartDco Function(ffi.Pointer<ffi.Void>)>>(
      'frbgen_frb_example_gallery_wire_DataFrame_lazy');
  late final _wire_DataFrame_lazy = _wire_DataFrame_lazyPtr
      .asFunction<WireSyncRust2DartDco Function(ffi.Pointer<ffi.Void>)>();

  WireSyncRust2DartDco wire_Expr_gt(
    ffi.Pointer<ffi.Void> that,
    ffi.Pointer<ffi.Void> other,
  ) {
    return _wire_Expr_gt(
      that,
      other,
    );
  }

  late final _wire_Expr_gtPtr = _lookup<
          ffi.NativeFunction<
              WireSyncRust2DartDco Function(
                  ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>)>>(
      'frbgen_frb_example_gallery_wire_Expr_gt');
  late final _wire_Expr_gt = _wire_Expr_gtPtr.asFunction<
      WireSyncRust2DartDco Function(
          ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>)>();

  WireSyncRust2DartDco wire_Expr_sum(
    ffi.Pointer<ffi.Void> that,
  ) {
    return _wire_Expr_sum(
      that,
    );
  }

  late final _wire_Expr_sumPtr = _lookup<
          ffi.NativeFunction<
              WireSyncRust2DartDco Function(ffi.Pointer<ffi.Void>)>>(
      'frbgen_frb_example_gallery_wire_Expr_sum');
  late final _wire_Expr_sum = _wire_Expr_sumPtr
      .asFunction<WireSyncRust2DartDco Function(ffi.Pointer<ffi.Void>)>();

  void wire_LazyFrame_collect(
    int port_,
    ffi.Pointer<ffi.Void> that,
  ) {
    return _wire_LazyFrame_collect(
      port_,
      that,
    );
  }

  late final _wire_LazyFrame_collectPtr = _lookup<
          ffi
          .NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Void>)>>(
      'frbgen_frb_example_gallery_wire_LazyFrame_collect');
  late final _wire_LazyFrame_collect = _wire_LazyFrame_collectPtr
      .asFunction<void Function(int, ffi.Pointer<ffi.Void>)>();

  WireSyncRust2DartDco wire_LazyFrame_filter(
    ffi.Pointer<ffi.Void> that,
    ffi.Pointer<ffi.Void> predicate,
  ) {
    return _wire_LazyFrame_filter(
      that,
      predicate,
    );
  }

  late final _wire_LazyFrame_filterPtr = _lookup<
          ffi.NativeFunction<
              WireSyncRust2DartDco Function(
                  ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>)>>(
      'frbgen_frb_example_gallery_wire_LazyFrame_filter');
  late final _wire_LazyFrame_filter = _wire_LazyFrame_filterPtr.asFunction<
      WireSyncRust2DartDco Function(
          ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>)>();

  WireSyncRust2DartDco wire_LazyFrame_group_by(
    ffi.Pointer<ffi.Void> that,
    ffi.Pointer<ffi.Void> expr,
  ) {
    return _wire_LazyFrame_group_by(
      that,
      expr,
    );
  }

  late final _wire_LazyFrame_group_byPtr = _lookup<
          ffi.NativeFunction<
              WireSyncRust2DartDco Function(
                  ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>)>>(
      'frbgen_frb_example_gallery_wire_LazyFrame_group_by');
  late final _wire_LazyFrame_group_by = _wire_LazyFrame_group_byPtr.asFunction<
      WireSyncRust2DartDco Function(
          ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>)>();

  WireSyncRust2DartDco wire_LazyGroupBy_agg(
    ffi.Pointer<ffi.Void> that,
    ffi.Pointer<ffi.Void> expr,
  ) {
    return _wire_LazyGroupBy_agg(
      that,
      expr,
    );
  }

  late final _wire_LazyGroupBy_aggPtr = _lookup<
          ffi.NativeFunction<
              WireSyncRust2DartDco Function(
                  ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>)>>(
      'frbgen_frb_example_gallery_wire_LazyGroupBy_agg');
  late final _wire_LazyGroupBy_agg = _wire_LazyGroupBy_aggPtr.asFunction<
      WireSyncRust2DartDco Function(
          ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>)>();

  WireSyncRust2DartDco wire_col(
    ffi.Pointer<wire_cst_list_prim_u_8_strict> name,
  ) {
    return _wire_col(
      name,
    );
  }

  late final _wire_colPtr = _lookup<
          ffi.NativeFunction<
              WireSyncRust2DartDco Function(
                  ffi.Pointer<wire_cst_list_prim_u_8_strict>)>>(
      'frbgen_frb_example_gallery_wire_col');
  late final _wire_col = _wire_colPtr.asFunction<
      WireSyncRust2DartDco Function(
          ffi.Pointer<wire_cst_list_prim_u_8_strict>)>();

  WireSyncRust2DartDco wire_lit(
    double t,
  ) {
    return _wire_lit(
      t,
    );
  }

  late final _wire_litPtr =
      _lookup<ffi.NativeFunction<WireSyncRust2DartDco Function(ffi.Double)>>(
          'frbgen_frb_example_gallery_wire_lit');
  late final _wire_lit =
      _wire_litPtr.asFunction<WireSyncRust2DartDco Function(double)>();

  void wire_read_sample_dataset(
    int port_,
  ) {
    return _wire_read_sample_dataset(
      port_,
    );
  }

  late final _wire_read_sample_datasetPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'frbgen_frb_example_gallery_wire_read_sample_dataset');
  late final _wire_read_sample_dataset =
      _wire_read_sample_datasetPtr.asFunction<void Function(int)>();

  void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockDataFrame(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockDataFrame(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockDataFramePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_frb_example_gallery_rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockDataFrame');
  late final _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockDataFrame =
      _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockDataFramePtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockDataFrame(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockDataFrame(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockDataFramePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_frb_example_gallery_rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockDataFrame');
  late final _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockDataFrame =
      _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockDataFramePtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockExpr(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockExpr(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockExprPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_frb_example_gallery_rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockExpr');
  late final _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockExpr =
      _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockExprPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockExpr(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockExpr(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockExprPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_frb_example_gallery_rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockExpr');
  late final _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockExpr =
      _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockExprPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyFrame(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyFrame(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyFramePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_frb_example_gallery_rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyFrame');
  late final _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyFrame =
      _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyFramePtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyFrame(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyFrame(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyFramePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_frb_example_gallery_rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyFrame');
  late final _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyFrame =
      _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyFramePtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyGroupByPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_frb_example_gallery_rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy');
  late final _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy =
      _rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyGroupByPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyGroupByPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_frb_example_gallery_rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy');
  late final _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy =
      _rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyGroupByPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<wire_cst_point> cst_new_box_autoadd_point() {
    return _cst_new_box_autoadd_point();
  }

  late final _cst_new_box_autoadd_pointPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_point> Function()>>(
          'frbgen_frb_example_gallery_cst_new_box_autoadd_point');
  late final _cst_new_box_autoadd_point = _cst_new_box_autoadd_pointPtr
      .asFunction<ffi.Pointer<wire_cst_point> Function()>();

  ffi.Pointer<wire_cst_size> cst_new_box_autoadd_size() {
    return _cst_new_box_autoadd_size();
  }

  late final _cst_new_box_autoadd_sizePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_size> Function()>>(
          'frbgen_frb_example_gallery_cst_new_box_autoadd_size');
  late final _cst_new_box_autoadd_size = _cst_new_box_autoadd_sizePtr
      .asFunction<ffi.Pointer<wire_cst_size> Function()>();

  ffi.Pointer<wire_cst_list_String> cst_new_list_String(
    int len,
  ) {
    return _cst_new_list_String(
      len,
    );
  }

  late final _cst_new_list_StringPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_cst_list_String> Function(
              ffi.Int32)>>('frbgen_frb_example_gallery_cst_new_list_String');
  late final _cst_new_list_String = _cst_new_list_StringPtr
      .asFunction<ffi.Pointer<wire_cst_list_String> Function(int)>();

  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_new_list_prim_u_8_strict(
    int len,
  ) {
    return _cst_new_list_prim_u_8_strict(
      len,
    );
  }

  late final _cst_new_list_prim_u_8_strictPtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<wire_cst_list_prim_u_8_strict> Function(ffi.Int32)>>(
      'frbgen_frb_example_gallery_cst_new_list_prim_u_8_strict');
  late final _cst_new_list_prim_u_8_strict = _cst_new_list_prim_u_8_strictPtr
      .asFunction<ffi.Pointer<wire_cst_list_prim_u_8_strict> Function(int)>();

  int dummy_method_to_enforce_bundling() {
    return _dummy_method_to_enforce_bundling();
  }

  late final _dummy_method_to_enforce_bundlingPtr =
      _lookup<ffi.NativeFunction<ffi.Int64 Function()>>(
          'dummy_method_to_enforce_bundling');
  late final _dummy_method_to_enforce_bundling =
      _dummy_method_to_enforce_bundlingPtr.asFunction<int Function()>();
}

final class wire_cst_size extends ffi.Struct {
  @ffi.Int32()
  external int width;

  @ffi.Int32()
  external int height;
}

final class wire_cst_point extends ffi.Struct {
  @ffi.Double()
  external double x;

  @ffi.Double()
  external double y;
}

final class wire_cst_list_prim_u_8_strict extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_cst_list_String extends ffi.Struct {
  external ffi.Pointer<ffi.Pointer<wire_cst_list_prim_u_8_strict>> ptr;

  @ffi.Int32()
  external int len;
}
