# Kolme-Dee

[![Tests](https://img.shields.io/github/actions/workflow/status/sh0guni/kolme-dee/test.yml?branch=master&label=tests)](https://github.com/sh0guni/kolme-dee/actions/workflows/test.yml)

A Rust library implementing the mathematical foundations of 3D game engine development, following the principles from "Foundations of Game Engine Development" by Eric Lengyel.

## Current Status

### Volume 1: Mathematics
- [x] Vector Fundamentals
  - [x] Basic Vector Operations
  - [x] Magnitude and Scalar Multiplication
- [x] Matrix Fundamentals
  - [x] Basic Matrix Operations
  - [x] Matrix Multiplication
  - [ ] Matrix Inversion
- [ ] Vector Multiplication
  - [ ] Dot Product
  - [ ] Cross Product
  - [ ] Scalar Triple Product
- [ ] Vector Projection
- [ ] Transforms
  - [ ] Coordinate Spaces
  - [ ] Rotations
  - [ ] Reflections
  - [ ] Scales
  - [ ] Skews
  - [ ] Homogeneous Coordinates
  - [ ] Quaternions
- [ ] Geometry
  - [ ] Normal Vectors
  - [ ] Lines and Rays
  - [ ] Planes
  - [ ] Plücker Coordinates

### Future Volumes
- [ ] Volume 2: Rendering
- [ ] Volume 3: Models & Materials
- [ ] Volume 4: Physics

## Usage

```rust
use kolme_dee::math::vector::Vector;

// Create a new vector
let v1 = Vector::new(1.0, 2.0, 3.0);
let v2 = Vector::new(4.0, 5.0, 6.0);

// Vector addition
let v3 = &v1 + &v2;

// Scalar multiplication
let v4 = &v1 * 2.0;

// Index access
assert_eq!(v1[0], 1.0);
assert_eq!(v1[1], 2.0);
assert_eq!(v1[2], 3.0);
```

## Operations

### Vector Operations
- Addition: `&v1 + &v2`
- Subtraction: `v1 - v2`
- Scalar multiplication: `&v1 * 2.0`
- Scalar division: `&v1 / 2.0`
- In-place multiplication: `v1 *= 2.0`
- In-place division: `v1 /= 2.0`

## Safety

- Division by zero will panic
- Index out of bounds will panic
- All operations are checked for validity

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0) - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. This project follows the mathematical foundations and implementation patterns from "Foundations of Game Engine Development" by Eric Lengyel.

## References

- [Foundations of Game Engine Development, Volume 1: Mathematics](https://foundationsofgameenginedev.com/)
