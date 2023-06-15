# Changelog

## \[0.1.1]

- [`2387db8`](https://github.com/tauri-apps/winrt-notification/commit/2387db87b0620d7cb6341d931c22454058b7b2da)([#7](https://github.com/tauri-apps/winrt-notification/pull/7)) Fix parsing loopable sounds from string as well.

## \[0.1.0]

- Set MSRV to 1.59
  - [ebca119](https://github.com/tauri-apps/winrt-notification/commit/ebca1199adfb913dd3b701693d11b076bf063e9a) Split changefile on 2022-08-13
  - [b2b7e4c](https://github.com/tauri-apps/winrt-notification/commit/b2b7e4cd894bdc3f22057532436499204348ea1a) Reduce msrv to 1.56 on 2022-08-14
  - [b83f6f8](https://github.com/tauri-apps/winrt-notification/commit/b83f6f839e3774d4e229bee44e2f55eb0aa84bfb) Update to 0.39 on 2022-08-25
- Update windows-rs version to 0.39
  - [005a107](https://github.com/tauri-apps/winrt-notification/commit/005a107d8c747e99aaf54a73f738f732dbc71bc8) Add changefile on 2022-08-13
  - [ebca119](https://github.com/tauri-apps/winrt-notification/commit/ebca1199adfb913dd3b701693d11b076bf063e9a) Split changefile on 2022-08-13
  - [c9e8361](https://github.com/tauri-apps/winrt-notification/commit/c9e8361b5c31bde63e33ce42ee95b5d207860974) chore: update changefile on 2022-08-16
  - [b83f6f8](https://github.com/tauri-apps/winrt-notification/commit/b83f6f839e3774d4e229bee44e2f55eb0aa84bfb) Update to 0.39 on 2022-08-25

## 0.5.1

- Re-export windows::runtime::Result
- Add support for scenarios

## 0.5.0

- Updated dependencies
- Added support for gnu targets without msys

## 0.4.0

- Updated dependencies

## 0.3.1

- Allow the crate build with the gnu toolchain with msys [#4][i4]

## 0.3.0

- Switched to the windows-rs crate, dropped winapi and winrt crate.

## 0.2.4

- Made most enums Copy/Clonable.

## 0.2.3

- Fixed issue where toasts weren't appearing after a windows update.

## 0.2.2

- Fixed linking issue introduced with winapi 0.3.4 [#4][i4]

## 0.2.1

- Implemented from_str on Sound enum.

## 0.2.0

- Added Windows 8.1 support [#3][i3]
- Incremented winrt dependency

## 0.1.5

- Re-export winrt::Error [#2][i2].
- Added workaround for AppId issue [#1][i1].

## 0.1.4

- Initial release

[i1]: https://github.com/allenbenz/winrt-notification/issues/1

[i2]: https://github.com/allenbenz/winrt-notification/issues/2

[i3]: https://github.com/allenbenz/winrt-notification/issues/3

[i4]: https://github.com/allenbenz/winrt-notification/issues/4
