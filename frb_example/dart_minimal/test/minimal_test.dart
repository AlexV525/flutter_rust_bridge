import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');
  test('dart call minimalAdder', () async {
    print('Action: Call rust (before)');
    expect(await minimalAdder(a: 100, b: 200), 300);
    print('Action: Call rust (after)');
  });
  print('Action: Configure tests (end)');

  test('hi', () async {
    expect(
        await exampleBasicListTypeU64TwinNormal(
            arg: Uint64List(1)..[0] = BigInt.parse('18446744073709551615')),
        [BigInt.parse('18446744073709551615')]);
  });
}
