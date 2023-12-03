import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// The Rust `std::sync::Arc` on the Dart side.
// Note: Use `extends`, instead of making the `_Droppable` a field,
// in order to ensure the `ffi.Finalizable` works well.
class RustArc<T> extends Droppable {
  /// The pointer that `std::sync::Arc::into_raw` gives.
  ///
  /// In other words, it is very similar to `std::sync::Arc.ptr`,
  /// but only with a small constant offset.
  PlatformPointer get _ptr =>
      super.dangerousReadInternalPtr() ??
      (throw const RustArcDisposedException());

  /// See comments in [RustArcStaticData] for details.
  final RustArcStaticData<T> _staticData;

  /// Mimic `std::sync::Arc::from_raw`
  RustArc.fromRaw({
    required int ptr,
    required super.externalSizeOnNative,
    required RustArcStaticData<T> staticData,
  })  : _staticData = staticData,
        super(ptr: ptrOrNullFromInt(ptr));

  /// Mimic `std::sync::Arc::clone`
  RustArc<T> clone() {
    final ptr = _ptr;

    _staticData._rustArcIncrementStrongCount(ptr);

    return RustArc.fromRaw(
      ptr: PlatformPointerUtil.ptrToInt(ptr),
      externalSizeOnNative: externalSizeOnNative,
      staticData: _staticData,
    );
  }

  /// Mimic `std::sync::Arc::into_raw`
  // Almost 1:1 implementation to `std::sync::Arc::into_raw` impl.
  PlatformPointer intoRaw() {
    final ptr = _ptr;
    forget();
    return ptr;
  }

  @override
  DroppableStaticData get staticData => _staticData;
}

/// Thrown when use after dispose
class RustArcDisposedException {
  /// Thrown when use after dispose
  const RustArcDisposedException();

  @override
  String toString() => 'RustArcDisposedException: '
      'Try to use RustArc after it has been disposed';
}

/// Should have exactly *one* instance per *type*.
///
/// For example, all `std::sync::Arc<Apple>` objects should use one
/// `RustArcTypeInfo` object, while all `std::sync::Arc<Orange>`
/// objects should use another.
///
/// The [T] is just a marker type to remind the content type and has no use.
class RustArcStaticData<T> extends DroppableStaticData {
  final RustArcIncrementStrongCountFnType _rustArcIncrementStrongCount;

  /// Constructs the data
  RustArcStaticData({
    /// Directly calls `std::sync::Arc::increment_strong_count(ptr)`
    required RustArcIncrementStrongCountFnType rustArcIncrementStrongCount,

    /// Directly calls `std::sync::Arc::decrement_strong_count(ptr)`
    required RustArcDecrementStrongCountFnType rustArcDecrementStrongCount,

    /// The function pointer to `rustArcDecrementStrongCount`
    required CrossPlatformFinalizerArg rustArcDecrementStrongCountPtr,
  })  : _rustArcIncrementStrongCount = rustArcIncrementStrongCount,
        super(
          releaseFn: rustArcDecrementStrongCount,
          releaseFnPtr: rustArcDecrementStrongCountPtr,
        );
}

/// The type of [RustArcStaticData._rustArcIncrementStrongCount]
typedef RustArcIncrementStrongCountFnType = void Function(PlatformPointer);

/// The type of [RustArcStaticData._rustArcDecrementStrongCount]
typedef RustArcDecrementStrongCountFnType = void Function(PlatformPointer);
