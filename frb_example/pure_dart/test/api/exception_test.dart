import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/exception.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('systematic test', () {
    test('call funcReturnErrorTwinNormal', () async {
      await expectLater(
          () async => funcReturnErrorTwinNormal(),
          throwsA(isA<AnyhowException>()
              .having((x) => x.message, 'message', 'deliberate error')));
    });
    test('call funcTypeFalliblePanicTwinNormal', () async {
      await expectRustPanic(
          () async => funcTypeFalliblePanicTwinNormal(), 'TwinNormal',
          messageOnNative: 'deliberate panic');
    });
    test('call funcTypeInfalliblePanicTwinNormal', () async {
      await expectRustPanic(
          () async => funcTypeInfalliblePanicTwinNormal(), 'TwinNormal',
          messageOnNative: 'deliberate panic');
    });

    addTestsIdentityFunctionCall(customEnumErrorReturnOkTwinNormal, [100]);
    test('call customEnumErrorPanicTwinNormal', () async {
      await expectRustPanic(
          () async => customEnumErrorPanicTwinNormal(), 'TwinNormal',
          messageOnNative: 'deliberate panic');
    });

    test('call funcReturnErrorTwinNormal', () async {
      var matcher = isA<CustomEnumErrorTwinNormal>()
          .having((x) => x.message, 'message', 'deliberate error');
      if (!kIsWeb)
        matcher = matcher.having((x) => x.backtrace, 'backtrace', isNotEmpty);
      await expectLater(
          () async => customEnumErrorReturnErrorTwinNormal(), throwsA(matcher));
    });

    addTestsErrorFunctionCall(
      customNestedErrorReturnErrorTwinNormal,
      [
        const CustomNestedErrorOuterTwinNormal.one('hello'),
        const CustomNestedErrorOuterTwinNormal.two(
            CustomNestedErrorInnerTwinNormal.three('hello')),
        const CustomNestedErrorOuterTwinNormal.two(
            CustomNestedErrorInnerTwinNormal.four(42)),
      ],
      equals,
    );

    addTestsErrorFunctionCall(
      customStructErrorReturnErrorTwinNormal,
      [const CustomStructErrorTwinNormal(a: 'hello')],
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
        expect(() async => returnErrCustomErrorTwinNormal(),
            throwsA(isA<CustomErrorTwinNormal>()));
      });

      test('Throw CustomStructError', () async {
        await expectLater(() async => returnCustomStructErrorTwinNormal(),
            throwsA(isA<CustomStructErrorAnotherTwinNormal>()));
      });

      test('Do not throw CustomStructError', () async {
        expect(await returnCustomStructOkTwinNormal(), 3);
      });

      test('Throw CustomStructError non static method', () async {
        await expectLater(
            () async => CustomStructTwinNormal(message: "hello")
                .nonstaticReturnCustomStructErrorTwinNormal(),
            throwsA(isA<CustomStructErrorAnotherTwinNormal>()));
      });

      test('Do not throw CustomStructError non static method', () async {
        expect(
            await CustomStructTwinNormal(message: "hello")
                .nonstaticReturnCustomStructOkTwinNormal(),
            3);
      });

      test('Throw CustomStructError static method', () async {
        await expectLater(
            () async => CustomStructTwinNormal
                .staticReturnCustomStructErrorTwinNormal(),
            throwsA(isA<CustomStructErrorAnotherTwinNormal>()));
      });

      test('Do not throw CustomStructError static method', () async {
        expect(
            await CustomStructTwinNormal.staticReturnCustomStructOkTwinNormal(),
            3);
      });

      test('Throw CustomNestedError1', () async {
        await expectLater(
            () async => returnCustomNestedError1TwinNormal(),
            throwsA(CustomNestedError1TwinNormal.errorNested(
                CustomNestedError2TwinNormal.customNested2Number(3))));
      });

      test('Throw CustomNestedError1 variant 1', () async {
        await expectLater(
            () async => returnCustomNestedError1Variant1TwinNormal(),
            throwsA(CustomNestedError1TwinNormal.customNested1("custom")));
      });

      test('Throw CustomNestedError2', () async {
        await expectLater(() async => returnCustomNestedError2TwinNormal(),
            throwsA(CustomNestedError2TwinNormal.customNested2("custom")));
      });

      test('Throw CustomError variant 0', () async {
        await expectLater(() async => returnErrorVariantTwinNormal(variant: 0),
            throwsA(isA<CustomErrorTwinNormal>()));
      });

      test('Throw CustomError variant 1', () async {
        await expectLater(() async => returnErrorVariantTwinNormal(variant: 1),
            throwsA(isA<CustomErrorTwinNormal>()));
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinNormal(), 3);
      });

      test('Throw CustomError static method', () async {
        await expectLater(
            () async =>
                SomeStructTwinNormal.staticReturnErrCustomErrorTwinNormal(),
            throwsA(isA<CustomErrorTwinNormal>()));
      });

      test('Throw CustomError static method, verifies implements Frb',
          () async {
        await expectLater(
            () async =>
                SomeStructTwinNormal.staticReturnErrCustomErrorTwinNormal(),
            throwsA(isA<FrbException>()));
      });

      test('Do not throw CustomError static method', () async {
        expect(await SomeStructTwinNormal.staticReturnOkCustomErrorTwinNormal(),
            3);
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinNormal(), 3);
      });

      test('Throw CustomError non-static method', () async {
        await expectLater(
            () async => SomeStructTwinNormal(value: 7)
                .nonStaticReturnErrCustomErrorTwinNormal(),
            throwsA(isA<CustomErrorTwinNormal>()));
        bool didCatch = false;
        try {
          await SomeStructTwinNormal(value: 7)
              .nonStaticReturnErrCustomErrorTwinNormal();
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
            await SomeStructTwinNormal(value: 6)
                .nonStaticReturnOkCustomErrorTwinNormal(),
            6);
      });

      test('Throw anyhow error', () async {
        await expectLater(
            () async => throwAnyhowTwinNormal(), throwsA(isA<FrbException>()));
      });

      test('Function with custom result panics', () async {
        await expectRustPanicRaw(() async => panicWithCustomResultTwinNormal(),
            'TwinNormal', throwsA(isA<FrbException>()));
      });

      test('Stream sink throw anyhow error', skip: 'wait until #11281',
          () async {
        expect(
          () async {
            await for (final _ in streamSinkThrowAnyhowTwinNormal()) {}
          },
          throwsA(isA<AnyhowException>().having((e) => e.toString(), 'toString',
              'AnyhowException(anyhow error)')),
        );
      });
    });
  });
}
