import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/benchmark.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/builder.dart';

Map<String, String> generateRustDirectSources() {
  return {
    'pseudo_manual/basic.rs': _generateBasic(),
    'pseudo_manual/optional_basic.rs': _generateOptionalBasic(),
    'pseudo_manual/basic_list.rs': _generateBasicList(),
    'pseudo_manual/benchmark_api.rs': generateBenchmark(),
  };
}

String _generateBasic() {
  final builder = RustFileBuilder();
  for (final ty in kBasicTypes) {
    builder.addIdentityFunction(ty.name, 'example_basic_type_${ty.name}');
  }
  return builder.toString();
}

String _generateBasicList() {
  final builder = RustFileBuilder();
  for (final ty in kBasicTypes) {
    builder.addIdentityFunction(
        'Vec<${ty.name}>', 'example_basic_list_type_${ty.name}');
  }
  return builder.toString();
}

String _generateOptionalBasic() {
  final builder = RustFileBuilder();
  for (final ty in kBasicTypes) {
    builder.addIdentityFunction(
        'Option<${ty.name}>', 'example_optional_basic_type_${ty.name}');
  }
  return builder.toString();
}

class BasicTypeInfo {
  final String name;
  final String dartTypeName;
  final String primitiveListName;
  final List<String> interestRawValues;
  final String Function(BasicTypeInfo, String) primitiveWrapper;
  final String Function(BasicTypeInfo, String) primitiveListWrapper;

  const BasicTypeInfo({
    required this.name,
    required this.dartTypeName,
    required this.primitiveListName,
    required this.interestRawValues,
    this.primitiveWrapper = _defaultPrimitiveWrapper,
    this.primitiveListWrapper = _defaultPrimitiveListWrapper,
  });

  static String _defaultPrimitiveWrapper(BasicTypeInfo info, String value) =>
      value;

  static String _defaultPrimitiveListWrapper(
          BasicTypeInfo info, String value) =>
      '${info.primitiveListName}.fromList([$value])';
}

final kBasicTypes = [
  const BasicTypeInfo(
    name: 'i8',
    dartTypeName: 'int',
    primitiveListName: 'Int8List',
    interestRawValues: ['0', '-128', '127'],
  ),
  const BasicTypeInfo(
    name: 'i16',
    dartTypeName: 'int',
    primitiveListName: 'Int16List',
    interestRawValues: ['0', '-32768', '32767'],
  ),
  const BasicTypeInfo(
    name: 'i32',
    dartTypeName: 'int',
    primitiveListName: 'Int32List',
    interestRawValues: ['0', '-2147483648', '2147483647'],
  ),
  const BasicTypeInfo(
    name: 'i64',
    // dartTypeName: 'BigInt',
    dartTypeName: 'int',
    primitiveListName: 'Int64List',
    interestRawValues: [
      '0',
      '-9007199254740992',
      '9007199254740992',
      // TODO handle >53bit values, in dart web compiler it will error
      // '-9223372036854775808',
      // '9223372036854775807',
    ],
    // primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  const BasicTypeInfo(
    name: 'u8',
    dartTypeName: 'int',
    primitiveListName: 'Uint8List',
    interestRawValues: ['0', '255'],
  ),
  const BasicTypeInfo(
    name: 'u16',
    dartTypeName: 'int',
    primitiveListName: 'Uint16List',
    interestRawValues: ['0', '65535'],
  ),
  const BasicTypeInfo(
    name: 'u32',
    dartTypeName: 'int',
    primitiveListName: 'Uint32List',
    interestRawValues: ['0', '4294967295'],
  ),
  const BasicTypeInfo(
    name: 'u64',
    // dartTypeName: 'BigInt',
    dartTypeName: 'int',
    primitiveListName: 'Uint64List',
    // '18446744073709551615', // not support numbers bigger than max i64 yet (but implementable)
    interestRawValues: [
      '0',
      '9007199254740992',
      // TODO handle >53bit values, in dart web compiler it will error
      // '9223372036854775807',
    ],
    // primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  const BasicTypeInfo(
    name: 'f32',
    dartTypeName: 'double',
    primitiveListName: 'Float32List',
    interestRawValues: ['0', '-42.5', '123456'],
  ),
  const BasicTypeInfo(
    name: 'f64',
    dartTypeName: 'double',
    primitiveListName: 'Float64List',
    interestRawValues: ['0', '-42.5', '123456'],
  ),
  BasicTypeInfo(
    name: 'bool',
    dartTypeName: 'bool',
    primitiveListName: 'List<bool>',
    interestRawValues: ['false', 'true'],
    primitiveListWrapper: (info, x) => '<bool>[$x]',
  ),
];
