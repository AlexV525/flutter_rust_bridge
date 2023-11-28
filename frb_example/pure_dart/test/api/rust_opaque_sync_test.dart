import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/rust_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/api/rust_opaque_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create', () {
    var data = syncCreateOpaqueTwinNormal();
    data.dispose();
  });

  test('option', () async {
    var x = syncOptionRustOpaqueTwinNormal();
    expect(x, isNotNull);
    x!.dispose();
  });

  test('nonclone', () async {
    var data = syncCreateNonCloneTwinNormal();
    var data2 = await runNonCloneTwinNormal(clone: data);
    expect(data2, "content");
    data.dispose();
  });

  test('double call', () {
    var data = syncCreateSyncOpaqueTwinNormal();
    expect(
        syncRunOpaqueTwinNormal(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        syncRunOpaqueTwinNormal(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
  });

  test('call after drop', () {
    var data = syncCreateSyncOpaqueTwinNormal();
    expect(
        syncRunOpaqueTwinNormal(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    expect(() => syncRunOpaqueTwinNormal(opaque: data),
        throwsA(isA<PanicException>()));
  });

  test('check generator', () {
    expect(frbSyncGeneratorTestTwinNormal().runtimeType == FrbOpaqueSyncReturn,
        isTrue);
  });
}
