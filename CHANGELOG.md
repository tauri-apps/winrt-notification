# Changelog

## \[0.3.0]

- [`c0b9b2f`](https://github.com/tauri-apps/winrt-notification/commit/c0b9b2fc149c0b0cb5846c263f2db709218426ff)([#25](https://github.com/tauri-apps/winrt-notification/pull/25)) Add support for adding buttons using `Toast::add_button`. This also comes with a change to `Toast::on_activated` wich will now take an `Option<String>` argument, containing which button was pressed if any.

## \[0.2.1]

- [`657a812`](https://github.com/tauri-apps/winrt-notification/commit/657a812db80182a1853232fcd87e0fa8483bdc8f)([#22](https://github.com/tauri-apps/winrt-notification/pull/22)) Update `windows` crate to `0.56`

## \[0.2.0]

- [`1427bbf`](https://github.com/tauri-apps/winrt-notification/commit/1427bbfadc0152d2d42b25d6385f43ce551e3aeb)([#18](https://github.com/tauri-apps/winrt-notification/pull/18)) Update MSRV to `1.62`
- [`1427bbf`](https://github.com/tauri-apps/winrt-notification/commit/1427bbfadc0152d2d42b25d6385f43ce551e3aeb)([#18](https://github.com/tauri-apps/winrt-notification/pull/18)) Update `windows` crate to `0.54`

## \[0.1.3]

- [`7aa2785`](https://github.com/tauri-apps/winrt-notification/commit/7aa27850c28470006cf75357d9de5474a0139e50) Update `windows` crate to `0.51`

## \[0.1.2]

- [`7c19ca4`](https://github.com/tauri-apps/winrt-notification/commit/7c19ca45410e5d6575f00137dcdb49a903346b4b)([#9](https://github.com/tauri-apps/winrt-notification/pull/9)) Add `Toast::on_activated` to add a handler to run when the toast is activated.

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
