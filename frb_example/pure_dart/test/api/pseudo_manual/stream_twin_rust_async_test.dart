// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `stream_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/stream_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call funcStreamRealisticTwinRustAsync', () async {
    final stream = funcStreamRealisticTwinRustAsync(arg: 'hello');
    var cnt = 0;
    await for (final value in stream) {
      debugPrint("output from func_stream's stream: $value");
      cnt++;
    }
    expect(cnt, 10);
  });

  test('dart call funcStreamSinkArgPositionTwinRustAsync', () async {
    // We only care about whether the codegen can understand StreamSink
    // as non-first argument in Rust, thus we do not test the return values.
    funcStreamSinkArgPositionTwinRustAsync(a: 100, b: 200);
  });

  test('call funcStreamReturnErrorTwinRustAsync', skip: 'wait until #11281',
      () async {
    await expectLater(
      () async {
        await for (final _ in funcStreamReturnErrorTwinRustAsync()) {}
      },
      throwsA(isA<AnyhowException>()
          .having((x) => x.message, 'message', 'deliberate error')),
    );
  });

  // TODO implement in web
  test('call funcStreamReturnPanicTwinRustAsync', skip: kIsWeb, () async {
    await expectRustPanic(
      () async {
        await for (final _ in funcStreamReturnPanicTwinRustAsync()) {}
      },
      'TwinRustAsync',
      messageOnNative: 'deliberate panic',
    );
  });

  Future<void> testHandleStream(
      Stream<LogTwinRustAsync> Function(
              {dynamic hint, required int key, required int max})
          handleStreamFunction) async {
    final max = 5;
    final key = 8;
    final stream = handleStreamFunction(key: key, max: max);
    var cnt = 0;
    await for (final value in stream) {
      print("output from handle_stream_x's stream: $value");
      expect(value.key, key);
      cnt++;
    }
    expect(cnt, max);
  }

  test('dart call handle_stream_sink_at_1', () {
    testHandleStream(handleStreamSinkAt1TwinRustAsync);
  });

  test('dart call handle_stream_sink_at_2', () {
    testHandleStream(handleStreamSinkAt2TwinRustAsync);
  });

  test('dart call handle_stream_sink_at_3', () {
    testHandleStream(handleStreamSinkAt3TwinRustAsync);
  });
}
