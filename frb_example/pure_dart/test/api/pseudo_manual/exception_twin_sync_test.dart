// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/exception_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('systematic test', () {
    test('call funcReturnErrorTwinSync', () async {
      await expectLater(
          () async => funcReturnErrorTwinSync(),
          throwsA(isA<AnyhowException>()
              .having((x) => x.message, 'message', 'deliberate error')));
    });
    test('call funcTypeFalliblePanicTwinSync', () async {
      await expectLater(() async => funcTypeFalliblePanicTwinSync(),
          throwsAPanicException(messageOnNative: 'deliberate panic'));
    });
    test('call funcTypeInfalliblePanicTwinSync', () async {
      await expectLater(() async => funcTypeInfalliblePanicTwinSync(),
          throwsAPanicException(messageOnNative: 'deliberate panic'));
    });

    addTestsIdentityFunctionCall(customEnumErrorReturnOkTwinSync, [100]);
    test('call customEnumErrorPanicTwinSync', () async {
      await expectLater(() async => customEnumErrorPanicTwinSync(),
          throwsAPanicException(messageOnNative: 'deliberate panic'));
    });

    test('call funcReturnErrorTwinSync', () async {
      var matcher = isA<CustomEnumErrorTwinSync>()
          .having((x) => x.message, 'message', 'deliberate error');
      if (!kIsWeb)
        matcher = matcher.having((x) => x.backtrace, 'backtrace', isNotEmpty);
      await expectLater(
          () async => customEnumErrorReturnErrorTwinSync(), throwsA(matcher));
    });

    addTestsErrorFunctionCall(
      customNestedErrorReturnErrorTwinSync,
      [
        const CustomNestedErrorOuterTwinSync.one('hello'),
        const CustomNestedErrorOuterTwinSync.two(
            CustomNestedErrorInnerTwinSync.three('hello')),
        const CustomNestedErrorOuterTwinSync.two(
            CustomNestedErrorInnerTwinSync.four(42)),
      ],
      equals,
    );

    addTestsErrorFunctionCall(
      customStructErrorReturnErrorTwinSync,
      [const CustomStructErrorTwinSync(a: 'hello')],
      equals,
    );
  });

  group('example-based tests', () {
    // TODO rm since sync?
    // // Test if sync return is working as expected.
    // test('dart call handle_sync_return', () async {
    //   expect(handleSyncReturn(mode: 'NORMAL'), List.filled(100, 42));
    //
    //   try {
    //     handleSyncReturn(mode: 'RESULT_ERR');
    //     fail("exception not thrown");
    //   } on AnyhowException catch (e) {
    //     print('dart catch anyhow e: $e');
    //   }
    //
    //   try {
    //     handleSyncReturn(mode: 'PANIC');
    //     fail("exception not thrown");
    //   } on PanicException catch (e) {
    //     print('dart catch panic e: $e');
    //   }
    // });
    //
    // test('dart call returnErr', () async {
    //   try {
    //     await returnErr();
    //     fail("exception not thrown");
    //   } on AnyhowException catch (e) {
    //     print('dart catch e: $e');
    //   }
    // });
    //
    // test('dart call returnPanic', () async {
    //   try {
    //     await returnPanic();
    //     fail("exception not thrown");
    //   } catch (e) {
    //     print('dart catch e: $e');
    //     expect(e, isA<PanicException>());
    //   }
    // });

    group('custom errors', () {
      // The first time a backtrace is created, symbol resolution
      // takes a significant amount of time.
      test('Throw CustomError', timeout: Timeout.factor(5), () {
        expect(() async => returnErrCustomErrorTwinSync(),
            throwsA(isA<CustomErrorTwinSync>()));
      });

      test('Throw CustomStructError', () async {
        await expectLater(() async => returnCustomStructErrorTwinSync(),
            throwsA(isA<CustomStructErrorAnotherTwinSync>()));
      });

      test('Do not throw CustomStructError', () async {
        expect(await returnCustomStructOkTwinSync(), 3);
      });

      test('Throw CustomStructError non static method', () async {
        await expectLater(
            () async => CustomStructTwinSync(message: "hello")
                .nonstaticReturnCustomStructErrorTwinSync(),
            throwsA(isA<CustomStructErrorAnotherTwinSync>()));
      });

      test('Do not throw CustomStructError non static method', () async {
        expect(
            await CustomStructTwinSync(message: "hello")
                .nonstaticReturnCustomStructOkTwinSync(),
            3);
      });

      test('Throw CustomStructError static method', () async {
        await expectLater(
            () async =>
                CustomStructTwinSync.staticReturnCustomStructErrorTwinSync(),
            throwsA(isA<CustomStructErrorAnotherTwinSync>()));
      });

      test('Do not throw CustomStructError static method', () async {
        expect(
            await CustomStructTwinSync.staticReturnCustomStructOkTwinSync(), 3);
      });

      test('Throw CustomNestedError1', () async {
        await expectLater(
            () async => returnCustomNestedError1TwinSync(),
            throwsA(CustomNestedError1TwinSync.errorNested(
                CustomNestedError2TwinSync.customNested2Number(3))));
      });

      test('Throw CustomNestedError1 variant 1', () async {
        await expectLater(
            () async => returnCustomNestedError1Variant1TwinSync(),
            throwsA(CustomNestedError1TwinSync.customNested1("custom")));
      });

      test('Throw CustomNestedError2', () async {
        await expectLater(() async => returnCustomNestedError2TwinSync(),
            throwsA(CustomNestedError2TwinSync.customNested2("custom")));
      });

      test('Throw CustomError variant 0', () async {
        await expectLater(() async => returnErrorVariantTwinSync(variant: 0),
            throwsA(isA<CustomErrorTwinSync>()));
      });

      test('Throw CustomError variant 1', () async {
        await expectLater(() async => returnErrorVariantTwinSync(variant: 1),
            throwsA(isA<CustomErrorTwinSync>()));
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinSync(), 3);
      });

      test('Throw CustomError static method', () async {
        await expectLater(
            () async => SomeStructTwinSync.staticReturnErrCustomErrorTwinSync(),
            throwsA(isA<CustomErrorTwinSync>()));
      });

      test('Throw CustomError static method, verifies implements Frb',
          () async {
        await expectLater(
            () async => SomeStructTwinSync.staticReturnErrCustomErrorTwinSync(),
            throwsA(isA<FrbException>()));
      });

      test('Do not throw CustomError static method', () async {
        expect(await SomeStructTwinSync.staticReturnOkCustomErrorTwinSync(), 3);
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinSync(), 3);
      });

      test('Throw CustomError non-static method', () async {
        await expectLater(
            () async => SomeStructTwinSync(value: 7)
                .nonStaticReturnErrCustomErrorTwinSync(),
            throwsA(isA<CustomErrorTwinSync>()));
        bool didCatch = false;
        try {
          await SomeStructTwinSync(value: 7)
              .nonStaticReturnErrCustomErrorTwinSync();
        } catch (e) {
          final FrbBacktracedException ex = e as FrbBacktracedException;
          print("backtrace: ${ex.backtrace}");
          if (!kIsWeb) expect(ex.backtrace, isNotEmpty);
          didCatch = true;
        }
        expect(didCatch, true);
      });

      test('Do not throw CustomError non-static method', () async {
        expect(
            await SomeStructTwinSync(value: 6)
                .nonStaticReturnOkCustomErrorTwinSync(),
            6);
      });

      test('Throw anyhow error', () async {
        await expectLater(
            () async => throwAnyhowTwinSync(), throwsA(isA<FrbException>()));
      });

      test('Function with custom result panics', () async {
        await expectLater(() async => panicWithCustomResultTwinSync(),
            throwsA(isA<FrbException>()));
      });

      test('Stream sink throw anyhow error', () async {
        expect(
          () async {
            await for (final _ in streamSinkThrowAnyhowTwinSync()) {}
          },
          throwsA(isA<AnyhowException>().having((e) => e.toString(), 'toString',
              'AnyhowException(anyhow error)')),
        );
      });
    });
  });
}
