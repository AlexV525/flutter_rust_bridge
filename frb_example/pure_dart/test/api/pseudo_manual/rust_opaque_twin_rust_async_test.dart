// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create and dispose', () async {
    var futureData = createOpaqueTwinRustAsync();
    var data = await createOpaqueTwinRustAsync();
    data.dispose();
    (await futureData).dispose();
  });

  test('simple call', () async {
    var opaque = await createOpaqueTwinRustAsync();
    var hideData = await runOpaqueTwinRustAsync(opaque: opaque);

    expect(
        hideData,
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    opaque.dispose();
  });

  test('double Call', () async {
    var data = await createOpaqueTwinRustAsync();
    expect(
        await runOpaqueTwinRustAsync(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinRustAsync(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
  });

  test('call after dispose', () async {
    var data = await createOpaqueTwinRustAsync();
    expect(
        await runOpaqueTwinRustAsync(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    await expectThrowsPanic(
        () => runOpaqueTwinRustAsync(opaque: data), 'TwinRustAsync');
  });

  test('dispose before complete', () async {
    var data = await createOpaqueTwinRustAsync();
    var task = runOpaqueWithDelayTwinRustAsync(opaque: data);
    data.dispose();
    expect(
        await task,
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    await expectThrowsPanic(
        () => runOpaqueTwinRustAsync(opaque: data), 'TwinRustAsync');
  });

  test('create array of opaque type', () async {
    var data = await opaqueArrayTwinRustAsync();
    for (var v in data) {
      expect(
          await runOpaqueTwinRustAsync(opaque: v),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      v.dispose();
      await expectThrowsPanic(
          () => runOpaqueTwinRustAsync(opaque: v), 'TwinRustAsync');
    }
  });

  test('create enums of opaque type', () async {
    var data = await createArrayOpaqueEnumTwinRustAsync();

    expect(
        await runEnumOpaqueTwinRustAsync(opaque: data[0]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    (data[0] as EnumOpaqueTwinRustAsync_Struct).field0.dispose();

    expect(await runEnumOpaqueTwinRustAsync(opaque: data[1]), "42");
    (data[1] as EnumOpaqueTwinRustAsync_Primitive).field0.dispose();

    expect(await runEnumOpaqueTwinRustAsync(opaque: data[2]), "\"String\"");
    (data[2] as EnumOpaqueTwinRustAsync_TraitObj).field0.dispose();

    expect(
        await runEnumOpaqueTwinRustAsync(opaque: data[3]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[3] as EnumOpaqueTwinRustAsync_Mutex).field0.dispose();

    expect(
        await runEnumOpaqueTwinRustAsync(opaque: data[4]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[4] as EnumOpaqueTwinRustAsync_RwLock).field0.dispose();
    await expectThrowsPanic(
        () => runEnumOpaqueTwinRustAsync(opaque: data[4]), 'TwinRustAsync');
  });

  test('opaque field', () async {
    var data = await createNestedOpaqueTwinRustAsync();
    await futurizeVoidTwinRustAsync(runNestedOpaqueTwinRustAsync(opaque: data));

    expect(
        await runOpaqueTwinRustAsync(opaque: data.first),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinRustAsync(opaque: data.second),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.first.dispose();
    await expectThrowsPanic(
        () => runOpaqueTwinRustAsync(opaque: data.first), 'TwinRustAsync');
    await expectThrowsPanic(
        () => runNestedOpaqueTwinRustAsync(opaque: data), 'TwinRustAsync');
    expect(
        await runOpaqueTwinRustAsync(opaque: data.second),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.second.dispose();
  });

  test('array', () async {
    var data = await opaqueArrayTwinRustAsync();
    await futurizeVoidTwinRustAsync(opaqueArrayRunTwinRustAsync(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinRustAsync(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectThrowsPanic(
        () => opaqueArrayRunTwinRustAsync(data: data), 'TwinRustAsync');
    data[1].dispose();
  });

  test('vec', () async {
    var data = await opaqueVecTwinRustAsync();
    await futurizeVoidTwinRustAsync(opaqueVecRunTwinRustAsync(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinRustAsync(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectThrowsPanic(
        () => opaqueVecRunTwinRustAsync(data: data), 'TwinRustAsync');
    data[1].dispose();
  });

  test('unwrap', () async {
    var data = await createOpaqueTwinRustAsync();
    data.move = true;
    expect(
        await unwrapRustOpaqueTwinRustAsync(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(data.isStale(), isTrue);

    var data2 = await createOpaqueTwinRustAsync();
    await expectLater(() => unwrapRustOpaqueTwinRustAsync(opaque: data2),
        throwsA(isA<AnyhowException>()));
    expect(data2.isStale(), isFalse);
  });
}
