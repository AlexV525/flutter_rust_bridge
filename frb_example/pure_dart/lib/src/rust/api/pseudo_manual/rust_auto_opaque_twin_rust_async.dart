// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.16.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<void> rustAutoOpaqueArgOwnTwinRustAsync(
        {required NonCloneSimpleTwinRustAsync arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgOwnTwinRustAsync(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueArgBorrowTwinRustAsync(
        {required NonCloneSimpleTwinRustAsync arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgBorrowTwinRustAsync(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueArgMutBorrowTwinRustAsync(
        {required NonCloneSimpleTwinRustAsync arg,
        required int expect,
        required int adder,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgMutBorrowTwinRustAsync(
        arg: arg, expect: expect, adder: adder, hint: hint);

Future<NonCloneSimpleTwinRustAsync> rustAutoOpaqueReturnOwnTwinRustAsync(
        {required int initial, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueReturnOwnTwinRustAsync(initial: initial, hint: hint);

Future<NonCloneSimpleTwinRustAsync>
    rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsync(
            {required NonCloneSimpleTwinRustAsync arg, dynamic hint}) =>
        RustLib.instance.api.rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsync(
            arg: arg, hint: hint);

Future<void> rustAutoOpaqueTwoArgsTwinRustAsync(
        {required NonCloneSimpleTwinRustAsync a,
        required NonCloneSimpleTwinRustAsync b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueTwoArgsTwinRustAsync(a: a, b: b, hint: hint);

Future<void> rustAutoOpaqueNormalAndOpaqueArgTwinRustAsync(
        {required NonCloneSimpleTwinRustAsync a,
        required String b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueNormalAndOpaqueArgTwinRustAsync(a: a, b: b, hint: hint);

/// "+" inside the type signature
Future<void> rustAutoOpaquePlusSignArgTwinRustAsync(
        {required BoxMyTraitTwinRustAsync arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaquePlusSignArgTwinRustAsync(arg: arg, hint: hint);

Future<BoxMyTraitTwinRustAsync> rustAutoOpaquePlusSignReturnTwinRustAsync(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaquePlusSignReturnTwinRustAsync(hint: hint);

Future<void> rustAutoOpaqueCallableArgTwinRustAsync(
        {required BoxFnStringString arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueCallableArgTwinRustAsync(arg: arg, hint: hint);

Future<BoxFnStringString> rustAutoOpaqueCallableReturnTwinRustAsync(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueCallableReturnTwinRustAsync(hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgOwnTwinRustAsync(
        {required BoxHelloTraitTwinRustAsync arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgOwnTwinRustAsync(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgBorrowTwinRustAsync(
        {required BoxHelloTraitTwinRustAsync arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgBorrowTwinRustAsync(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgMutBorrowTwinRustAsync(
        {required BoxHelloTraitTwinRustAsync arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgMutBorrowTwinRustAsync(
        arg: arg, expect: expect, hint: hint);

Future<BoxHelloTraitTwinRustAsync>
    rustAutoOpaqueTraitObjectReturnOwnOneTwinRustAsync({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueTraitObjectReturnOwnOneTwinRustAsync(hint: hint);

Future<BoxHelloTraitTwinRustAsync>
    rustAutoOpaqueTraitObjectReturnOwnTwoTwinRustAsync({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueTraitObjectReturnOwnTwoTwinRustAsync(hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinRustAsync(
        {required StructWithGoodAndOpaqueFieldTwinRustAsync arg,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinRustAsync(
            arg: arg, hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinRustAsync(
        {required StructWithGoodAndOpaqueFieldTwinRustAsync arg,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinRustAsync(
            arg: arg, hint: hint);

Future<
    void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinRustAsync(
        {required StructWithGoodAndOpaqueFieldTwinRustAsync arg,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinRustAsync(
            arg: arg, hint: hint);

Future<StructWithGoodAndOpaqueFieldTwinRustAsync>
    rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinRustAsync(
            {dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinRustAsync(
                hint: hint);

Future<OpaqueOneTwinRustAsyncOpaqueTwoTwinRustAsync>
    rustAutoOpaqueReturnOpaqueOneAndTwoTwinRustAsync({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueReturnOpaqueOneAndTwoTwinRustAsync(hint: hint);

Future<OpaqueTwoTwinRustAsync> rustAutoOpaqueReturnOpaqueTwoTwinRustAsync(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueReturnOpaqueTwoTwinRustAsync(hint: hint);

// Rust type: flutter_rust_bridge::RustOpaque<flutter_rust_bridge::for_generated::rust_async::RwLock<Box<dyn Fn (String) -> String + Send + Sync>>>
@sealed
class BoxFnStringString extends RustOpaque {
  BoxFnStringString.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  BoxFnStringString.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_BoxFnStringString,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_BoxFnStringString,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxFnStringStringPtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<flutter_rust_bridge::for_generated::rust_async::RwLock<Box<dyn HelloTraitTwinRustAsync>>>
@sealed
class BoxHelloTraitTwinRustAsync extends RustOpaque {
  BoxHelloTraitTwinRustAsync.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  BoxHelloTraitTwinRustAsync.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_BoxHelloTraitTwinRustAsync,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_BoxHelloTraitTwinRustAsync,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_BoxHelloTraitTwinRustAsyncPtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<flutter_rust_bridge::for_generated::rust_async::RwLock<Box<dyn MyTraitTwinRustAsync + Send + Sync>>>
@sealed
class BoxMyTraitTwinRustAsync extends RustOpaque {
  BoxMyTraitTwinRustAsync.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  BoxMyTraitTwinRustAsync.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_BoxMyTraitTwinRustAsync,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxMyTraitTwinRustAsync,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_BoxMyTraitTwinRustAsyncPtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<flutter_rust_bridge::for_generated::rust_async::RwLock<NonCloneSimpleTwinRustAsync>>
@sealed
class NonCloneSimpleTwinRustAsync extends RustOpaque {
  NonCloneSimpleTwinRustAsync.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  NonCloneSimpleTwinRustAsync.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_NonCloneSimpleTwinRustAsync,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleTwinRustAsync,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleTwinRustAsyncPtr,
  );

  Future<void> instanceMethodArgBorrowTwinRustAsync({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncInstanceMethodArgBorrowTwinRustAsync(
        that: this,
      );

  Future<void> instanceMethodArgMutBorrowTwinRustAsync({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncInstanceMethodArgMutBorrowTwinRustAsync(
        that: this,
      );

  Future<void> instanceMethodArgOwnTwinRustAsync({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncInstanceMethodArgOwnTwinRustAsync(
        that: this,
      );

  Future<int> get instanceMethodGetterTwinRustAsync => RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncInstanceMethodGetterTwinRustAsync(
        that: this,
      );

  Future<NonCloneSimpleTwinRustAsync> instanceMethodReturnOwnTwinRustAsync(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncInstanceMethodReturnOwnTwinRustAsync(
        that: this,
      );

  /// named constructor
  static Future<NonCloneSimpleTwinRustAsync> newCustomNameTwinRustAsync(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncNewCustomNameTwinRustAsync(hint: hint);

  /// unnamed constructor
  static Future<NonCloneSimpleTwinRustAsync> newTwinRustAsync({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncNewTwinRustAsync(hint: hint);

  /// constructor with Result
  static Future<NonCloneSimpleTwinRustAsync> newWithResultTwinRustAsync(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncNewWithResultTwinRustAsync(hint: hint);

  static Future<void> staticMethodArgBorrowTwinRustAsync(
          {required NonCloneSimpleTwinRustAsync arg, dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncStaticMethodArgBorrowTwinRustAsync(
              arg: arg, hint: hint);

  static Future<void> staticMethodArgMutBorrowTwinRustAsync(
          {required NonCloneSimpleTwinRustAsync arg, dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncStaticMethodArgMutBorrowTwinRustAsync(
              arg: arg, hint: hint);

  static Future<void> staticMethodArgOwnTwinRustAsync(
          {required NonCloneSimpleTwinRustAsync arg, dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncStaticMethodArgOwnTwinRustAsync(
              arg: arg, hint: hint);

  static Future<NonCloneSimpleTwinRustAsync> staticMethodReturnOwnTwinRustAsync(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinRustAsyncStaticMethodReturnOwnTwinRustAsync(
              hint: hint);
}

// Rust type: flutter_rust_bridge::RustOpaque<flutter_rust_bridge::for_generated::rust_async::RwLock<StructWithGoodAndOpaqueFieldTwinRustAsync>>
@sealed
class StructWithGoodAndOpaqueFieldTwinRustAsync extends RustOpaque {
  StructWithGoodAndOpaqueFieldTwinRustAsync.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  StructWithGoodAndOpaqueFieldTwinRustAsync.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_StructWithGoodAndOpaqueFieldTwinRustAsync,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_StructWithGoodAndOpaqueFieldTwinRustAsync,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_StructWithGoodAndOpaqueFieldTwinRustAsyncPtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<flutter_rust_bridge::for_generated::rust_async::RwLock<(crate::api::pseudo_manual::rust_auto_opaque_twin_rust_async::OpaqueOneTwinRustAsync,crate::api::pseudo_manual::rust_auto_opaque_twin_rust_async::OpaqueTwoTwinRustAsync,)>>
@sealed
class OpaqueOneTwinRustAsyncOpaqueTwoTwinRustAsync extends RustOpaque {
  OpaqueOneTwinRustAsyncOpaqueTwoTwinRustAsync.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  OpaqueOneTwinRustAsyncOpaqueTwoTwinRustAsync.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_OpaqueOneTwinRustAsyncOpaqueTwoTwinRustAsync,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_OpaqueOneTwinRustAsyncOpaqueTwoTwinRustAsync,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_OpaqueOneTwinRustAsyncOpaqueTwoTwinRustAsyncPtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<flutter_rust_bridge::for_generated::rust_async::RwLock<crate::api::pseudo_manual::rust_auto_opaque_twin_rust_async::OpaqueTwoTwinRustAsync>>
@sealed
class OpaqueTwoTwinRustAsync extends RustOpaque {
  OpaqueTwoTwinRustAsync.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  OpaqueTwoTwinRustAsync.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_OpaqueTwoTwinRustAsync,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_OpaqueTwoTwinRustAsync,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_OpaqueTwoTwinRustAsyncPtr,
  );
}
