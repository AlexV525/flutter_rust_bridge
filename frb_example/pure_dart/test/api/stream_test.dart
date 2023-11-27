// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/stream.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call funcStreamRealisticTwinNormal', () async {
    final stream = funcStreamRealisticTwinNormal(arg: 'hello');
    var cnt = 0;
    await for (final value in stream) {
      debugPrint("output from func_stream's stream: $value");
      cnt++;
    }
    expect(cnt, 10);
  });

  test('dart call funcStreamSinkArgPositionTwinNormal', () async {
    // We only care about whether the codegen can understand StreamSink
    // as non-first argument in Rust, thus we do not test the return values.
    funcStreamSinkArgPositionTwinNormal(a: 100, b: 200);
  });

  test('call funcStreamReturnErrorTwinNormal', () async {
    await expectLater(
      () async {
        await for (final _ in funcStreamReturnErrorTwinNormal()) {}
      },
      throwsA(isA<AnyhowException>()
          .having((x) => x.message, 'message', 'deliberate error')),
    );
  });

  // TODO implement in web
  test('call funcStreamReturnPanicTwinNormal', skip: kIsWeb, () async {
    await expectLater(
      () async {
        await for (final _ in funcStreamReturnPanicTwinNormal()) {}
      },
      throwsAPanicException(messageOnNative: 'deliberate panic'),
    );
  });

  Future<void> testHandleStream(
      Stream<Log> Function({dynamic hint, required int key, required int max})
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
    testHandleStream(handleStreamSinkAt1);
  });

  test('dart call handle_stream_sink_at_2', () {
    testHandleStream(handleStreamSinkAt2);
  });

  test('dart call handle_stream_sink_at_3', () {
    testHandleStream(handleStreamSinkAt3);
  });
}
