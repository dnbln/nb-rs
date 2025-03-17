# 0.21.1

Released: 2025-03-17

- Added the new Categories (`Angry` and `Run`) to the `strong-types` feature as well.


# 0.21.0

Released: 2025-03-17

- Added new Categories: `Angry` and `Run`.
- Bug fix: Make strong types feature compile again.

# 0.20.1

Released: 2023-08-19

- Bug fix: changed `File::write` to `File::write_all` when downloading images.

# 0.20.0

Released: 2023-08-19

- Made `Category` `#[non_exhaustive]` to allow for new categories to be added
  without breaking the public API.
- `download` feature: allow the downloading of images directly
  through the library.

# 0.19.0

Released: 2023-08-18

- Changed `Category::to_url_path` to less misleading `Category::to_url_name`,
  and added `Category::from_url_name` to parse a category from a name (also
  used for `FromStr` implementation).

# 0.18.0

Released: 2023-08-18

- Added new categories: `handshake`, `lurk`, `peck` and `yawn`.

# 0.17.0

Released: 2023-04-24

- Implemented a proper `Client`.
- Added rate limiting for the search endpoint.
- Removed the unused `build-dependencies` section from `Cargo.toml`,
  which was previously used for the `local` feature.

# 0.16.0

Released: 2023-04-16

- Due to the change from sequential IDs to UUIDs, the `local`
  feature no longer makes sense, and has been removed.