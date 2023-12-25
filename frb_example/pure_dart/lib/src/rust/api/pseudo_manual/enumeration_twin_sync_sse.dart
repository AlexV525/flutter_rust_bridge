// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.4.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'misc_example_twin_sync_sse.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'enumeration_twin_sync_sse.freezed.dart';

EnumSimpleTwinSyncSse funcEnumSimpleTwinSyncSse(
        {required EnumSimpleTwinSyncSse arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumSimpleTwinSyncSse(arg: arg, hint: hint);

EnumWithItemMixedTwinSyncSse funcEnumWithItemMixedTwinSyncSse(
        {required EnumWithItemMixedTwinSyncSse arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemMixedTwinSyncSse(arg: arg, hint: hint);

EnumWithItemTupleTwinSyncSse funcEnumWithItemTupleTwinSyncSse(
        {required EnumWithItemTupleTwinSyncSse arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemTupleTwinSyncSse(arg: arg, hint: hint);

EnumWithItemStructTwinSyncSse funcEnumWithItemStructTwinSyncSse(
        {required EnumWithItemStructTwinSyncSse arg, dynamic hint}) =>
    RustLib.instance.api
        .funcEnumWithItemStructTwinSyncSse(arg: arg, hint: hint);

Uint8List printNoteTwinSyncSse({required NoteTwinSyncSse note, dynamic hint}) =>
    RustLib.instance.api.printNoteTwinSyncSse(note: note, hint: hint);

WeekdaysTwinSyncSse? handleReturnEnumTwinSyncSse(
        {required String input, dynamic hint}) =>
    RustLib.instance.api.handleReturnEnumTwinSyncSse(input: input, hint: hint);

WeekdaysTwinSyncSse handleEnumParameterTwinSyncSse(
        {required WeekdaysTwinSyncSse weekday, dynamic hint}) =>
    RustLib.instance.api
        .handleEnumParameterTwinSyncSse(weekday: weekday, hint: hint);

MeasureTwinSyncSse? multiplyByTenTwinSyncSse(
        {required MeasureTwinSyncSse measure, dynamic hint}) =>
    RustLib.instance.api.multiplyByTenTwinSyncSse(measure: measure, hint: hint);

KitchenSinkTwinSyncSse handleEnumStructTwinSyncSse(
        {required KitchenSinkTwinSyncSse val, dynamic hint}) =>
    RustLib.instance.api.handleEnumStructTwinSyncSse(val: val, hint: hint);

@freezed
sealed class DistanceTwinSyncSse with _$DistanceTwinSyncSse {
  const factory DistanceTwinSyncSse.unknown() = DistanceTwinSyncSse_Unknown;
  const factory DistanceTwinSyncSse.map(
    double field0,
  ) = DistanceTwinSyncSse_Map;
}

enum EnumSimpleTwinSyncSse {
  a,
  b,
}

@freezed
sealed class EnumWithItemMixedTwinSyncSse with _$EnumWithItemMixedTwinSyncSse {
  const factory EnumWithItemMixedTwinSyncSse.a() =
      EnumWithItemMixedTwinSyncSse_A;
  const factory EnumWithItemMixedTwinSyncSse.b(
    Uint8List field0,
  ) = EnumWithItemMixedTwinSyncSse_B;
  const factory EnumWithItemMixedTwinSyncSse.c({
    required String cField,
  }) = EnumWithItemMixedTwinSyncSse_C;
}

@freezed
sealed class EnumWithItemStructTwinSyncSse
    with _$EnumWithItemStructTwinSyncSse {
  const factory EnumWithItemStructTwinSyncSse.a({
    required Uint8List aField,
  }) = EnumWithItemStructTwinSyncSse_A;
  const factory EnumWithItemStructTwinSyncSse.b({
    required Int32List bField,
  }) = EnumWithItemStructTwinSyncSse_B;
}

@freezed
sealed class EnumWithItemTupleTwinSyncSse with _$EnumWithItemTupleTwinSyncSse {
  const factory EnumWithItemTupleTwinSyncSse.a(
    Uint8List field0,
  ) = EnumWithItemTupleTwinSyncSse_A;
  const factory EnumWithItemTupleTwinSyncSse.b(
    Int32List field0,
  ) = EnumWithItemTupleTwinSyncSse_B;
}

@freezed
sealed class KitchenSinkTwinSyncSse with _$KitchenSinkTwinSyncSse {
  /// Comment on variant
  const factory KitchenSinkTwinSyncSse.empty() = KitchenSinkTwinSyncSse_Empty;
  const factory KitchenSinkTwinSyncSse.primitives({
    /// Dart field comment
    @Default(-1) int int32,
    required double float64,
    required bool boolean,
  }) = KitchenSinkTwinSyncSse_Primitives;
  const factory KitchenSinkTwinSyncSse.nested(
    int field0, [
    @Default(KitchenSinkTwinSyncSse.empty()) KitchenSinkTwinSyncSse field1,
  ]) = KitchenSinkTwinSyncSse_Nested;
  const factory KitchenSinkTwinSyncSse.optional([
    /// Comment on anonymous field
    @Default(-1) int? field0,
    int? field1,
  ]) = KitchenSinkTwinSyncSse_Optional;
  const factory KitchenSinkTwinSyncSse.buffer(
    Uint8List field0,
  ) = KitchenSinkTwinSyncSse_Buffer;
  const factory KitchenSinkTwinSyncSse.enums([
    @Default(WeekdaysTwinSyncSse.sunday) WeekdaysTwinSyncSse field0,
  ]) = KitchenSinkTwinSyncSse_Enums;
}

@freezed
sealed class MeasureTwinSyncSse with _$MeasureTwinSyncSse {
  const factory MeasureTwinSyncSse.speed(
    SpeedTwinSyncSse field0,
  ) = MeasureTwinSyncSse_Speed;
  const factory MeasureTwinSyncSse.distance(
    DistanceTwinSyncSse field0,
  ) = MeasureTwinSyncSse_Distance;
}

class NoteTwinSyncSse {
  final WeekdaysTwinSyncSse day;
  final String body;

  const NoteTwinSyncSse({
    this.day = WeekdaysTwinSyncSse.sunday,
    required this.body,
  });

  @override
  int get hashCode => day.hashCode ^ body.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NoteTwinSyncSse &&
          runtimeType == other.runtimeType &&
          day == other.day &&
          body == other.body;
}

@freezed
sealed class SpeedTwinSyncSse with _$SpeedTwinSyncSse {
  const factory SpeedTwinSyncSse.unknown() = SpeedTwinSyncSse_Unknown;
  const factory SpeedTwinSyncSse.gps(
    double field0,
  ) = SpeedTwinSyncSse_GPS;
}
