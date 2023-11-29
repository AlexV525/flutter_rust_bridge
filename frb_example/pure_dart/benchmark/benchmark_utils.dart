import 'package:benchmark_harness/benchmark_harness.dart';

abstract class EnhancedBenchmarkBase extends BenchmarkBase {
  const EnhancedBenchmarkBase(super.name, {super.emitter});

  // To opt into the reporting the time per run() instead of per 10 run() calls.
  @override
  void exercise() => run();
}

class JsonEmitter extends ScoreEmitter {
  final String prefix;
  final items = <Map<String, Object?>>[];

  JsonEmitter({required this.prefix});

  @override
  void emit(String testName, double value) {
    const PrintEmitter().emit(testName, value);
    items.add({
      'name': '$prefix$testName',
      'unit': "Microseconds",
      'value': value,
    });
  }
}
