// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/rust_auto_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
            arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
            arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
            arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);

        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100));

        await futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
            arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await Future.wait([
          futurizeVoidTwinNormal(
              rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100)),
          futurizeVoidTwinNormal(
              rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinNormal(
      //           rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100)),
      //       futurizeVoidTwinNormal(
      //           rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinNormal(
      //           rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100)),
      //       futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinNormal', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinNormal(arg: a);

      await futurizeVoidTwinNormal(
          rustAutoOpaqueArgOwnTwinNormal(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinNormal', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 20);

      await futurizeVoidTwinNormal(rustAutoOpaqueTwoArgsTwinNormal(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinNormal', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 42);

      await futurizeVoidTwinNormal(
          rustAutoOpaqueNormalAndOpaqueArgTwinNormal(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinNormal();
      await futurizeVoidTwinNormal(
          rustAutoOpaquePlusSignArgTwinNormal(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinNormal();
      await futurizeVoidTwinNormal(
          rustAutoOpaqueCallableArgTwinNormal(arg: obj));
    });
  });

  group('trait object', () {
    Future<void> _body(BoxHelloTraitTwinNormal obj, String expect) async {
      await futurizeVoidTwinNormal(rustAutoOpaqueTraitObjectArgBorrowTwinNormal(
          arg: obj, expect: expect));
      await futurizeVoidTwinNormal(
          rustAutoOpaqueTraitObjectArgMutBorrowTwinNormal(
              arg: obj, expect: expect));
      await futurizeVoidTwinNormal(
          rustAutoOpaqueTraitObjectArgOwnTwinNormal(arg: obj, expect: expect));
    }

    test(
        'case one',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnOneTwinNormal(), 'hello'));
    test(
        'case two',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnTwoTwinNormal(), 'B'));
  });

  test('static method', () async {
    final obj =
        await NonCloneSimpleTwinNormal.staticMethodReturnOwnTwinNormal();
    await futurizeVoidTwinNormal(
        NonCloneSimpleTwinNormal.staticMethodArgBorrowTwinNormal(arg: obj));
    await futurizeVoidTwinNormal(
        NonCloneSimpleTwinNormal.staticMethodArgMutBorrowTwinNormal(arg: obj));
    await futurizeVoidTwinNormal(
        NonCloneSimpleTwinNormal.staticMethodArgOwnTwinNormal(arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinNormal.newTwinNormal();
    await futurizeVoidTwinNormal(obj.instanceMethodArgBorrowTwinNormal());
    await futurizeVoidTwinNormal(obj.instanceMethodArgMutBorrowTwinNormal());
    await futurizeVoidTwinNormal(obj.instanceMethodReturnOwnTwinNormal());
    await futurizeVoidTwinNormal(obj.instanceMethodArgOwnTwinNormal());
  });
  test('instance newWithResult', () async {
    final obj = await NonCloneSimpleTwinNormal.newWithResultTwinNormal();
    await futurizeVoidTwinNormal(obj.instanceMethodArgBorrowTwinNormal());
  });

  test('getter', () async {
    final obj = await NonCloneSimpleTwinNormal.newTwinNormal();
    expect(await obj.instanceMethodGetterTwinNormal, 42);
  });

  test('types with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinNormal();
    await futurizeVoidTwinNormal(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinNormal(
            arg: obj));
    await futurizeVoidTwinNormal(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinNormal(
            arg: obj));
    await futurizeVoidTwinNormal(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinNormal(arg: obj));
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      await expectRustPanic(
        () async =>
            rustAutoOpaqueBorrowAndMutBorrow(borrow: obj, mutBorrow: obj),
        'TwinNormal',
        messageOnNative: 'TODO',
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrow(borrow: a, mutBorrow: b), 300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      expect(await rustAutoOpaqueBorrowAndBorrow(a: obj, b: obj), 200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 200);
      expect(await rustAutoOpaqueBorrowAndBorrow(a: a, b: b), 300);
    });
  });
}
