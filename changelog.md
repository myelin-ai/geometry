# Changelog

## 1.0.0
- Initial release

## 2.0.0
- Sanitize Aabbs by replacing `fn new(...) -> Self` with `fn try_new(...) -> Result<Self>`

## 2.1.0
- Add support for intersection tests between `Polygon`s.
- Unify the `intersects` methods of `Polygon` and `Aabb` under the `Intersects` trait
- Add some useful functions to `Vector`.
    - Support calculating the `normal` (i.e. perpendicular) vector
    - Support calculating the `magnitude`
    - Support calculating the `unit` vector

## 2.2.0
- Add `Vector::rotate`

## 2.3.0
- Add `Vector::rotate_clockwise`

## 2.4.0
- Add `Vector::negative`
- Add `Radians::try_from_degrees`

## 2.4.1
- Correct small typos in the documentation.
- Remove an unnecessary check.
- Remove direct dependency on `serde_derive`.

## Unreleased
- Explicitly disallow NaN, +∞ and -∞ in polygons instead of looping forever (#36).
