// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

void benchmarkVoidTwinSync({dynamic hint}) =>
    RustLib.instance.api.benchmarkVoidTwinSync(hint: hint);

int benchmarkInputBytesTwinSync({required Uint8List bytes, dynamic hint}) =>
    RustLib.instance.api.benchmarkInputBytesTwinSync(bytes: bytes, hint: hint);

Uint8List benchmarkOutputBytesTwinSync({required int size, dynamic hint}) =>
    RustLib.instance.api.benchmarkOutputBytesTwinSync(size: size, hint: hint);
