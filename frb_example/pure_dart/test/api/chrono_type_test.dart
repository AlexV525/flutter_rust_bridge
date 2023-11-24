import 'package:frb_example_pure_dart/src/rust/api/chrono_type.dart';
import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('DateTime<Utc>', () async {
    final date = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
    final resp = await datetimeUtc(d: date);
    expect(resp.year, date.year);
    expect(resp.month, date.month);
    expect(resp.day, date.day);
    expect(resp.hour, date.hour);
    expect(resp.minute, date.minute);
    expect(resp.second, date.second);
    expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
    expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
  });

  test('DateTime<Local>', () async {
    final date = DateTime(2022, 09, 10, 20, 48, 53, 123, 456);
    final resp = await datetimeLocal(d: date);
    expect(resp.year, date.year);
    expect(resp.month, date.month);
    expect(resp.day, date.day);
    expect(resp.hour, date.hour);
    expect(resp.minute, date.minute);
    expect(resp.second, date.second);
    expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
    expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
  });

  test('NaiveDateTime', () async {
    final date = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
    final resp = await naivedatetime(d: date);
    expect(resp.year, date.year);
    expect(resp.month, date.month);
    expect(resp.day, date.day);
    expect(resp.hour, date.hour);
    expect(resp.minute, date.minute);
    expect(resp.second, date.second);
    expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
    expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
  });
  test('Empty DateTime', () async {
    final resp = await optionalEmptyDatetimeUtc(d: null);
    expect(resp, null);
  });

  test('Duration', () async {
    final duration = Duration(hours: 4);
    final resp = await duration(d: duration);
    expect(resp.inHours, duration.inHours);
  });

  test('List<Duration>', () {
    final expected = [
      Duration(days: 1),
      Duration(days: 10),
      Duration(days: 100),
      Duration(milliseconds: 333),
      if (!kIsWeb) Duration(microseconds: 333)
    ];
    final now = DateTime.now();
    final durations = handleTimestamps(
      timestamps: expected.map(now.subtract).toList(),
      epoch: now,
    );
    expect(durations, completion(expected));
  });

  test('List<DateTime>', () {
    final expected = [
      Duration(days: 3),
      Duration(hours: 2),
      Duration(seconds: 1),
      Duration(milliseconds: 500),
      if (!kIsWeb) Duration(microseconds: 400)
    ];
    final now = DateTime.now();
    final result = handleDurations(durations: expected, since: now);
    expect(result, completion(expected.map(now.subtract)));
  });

  test('Combined Chrono types', () async {
    final test = await testChrono();
    expect(castInt(test.dt!.millisecondsSinceEpoch), castInt(1631297333000));
    expect(castInt(test.dt2!.millisecondsSinceEpoch), castInt(1631297333000));
    expect(test.du, Duration(hours: 4));
  });

  test('combined chrono types precise', () async {
    final datetime_1 = DateTime.utc(2002, 02, 23, 12, 13, 55);
    final datetime_2 = DateTime.utc(1800, 01, 23, 12, 56, 25);
    final duration = Duration(hours: 4);

    final result = await testPreciseChrono();

    expect(result.dt!.millisecondsSinceEpoch, datetime_1.millisecondsSinceEpoch);
    expect(result.dt2!.millisecondsSinceEpoch, datetime_2.millisecondsSinceEpoch);
    expect(result.du!.inHours, duration.inHours);
  });

  test('nested chrono types', () async {
    const duration = Duration(hours: 4);
    final naive = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
    final local = DateTime.now();
    final utc = DateTime.now().toUtc();
    final difference =
        await howLongDoesItTake(mine: FeatureChrono(utc: utc, local: local, duration: duration, naive: naive));
    log('$difference');
  });
}
