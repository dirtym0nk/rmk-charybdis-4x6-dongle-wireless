# RMK

RMK is a feature-rich and easy-to-use keyboard firmware.

This repo is RMK firmware for a split **Charybdis 4x6 wireless** (two nRF52840
halves + an nRF52840 dongle/receiver) with a PMW3610 trackball.

## Customizations (this fork)

Stock RMK 0.8.2 has no trackball scrolling and no convenient mouse control, so this
fork adds them. The keyboard/trackball processing runs on the **dongle (central)**,
so **only `chary-dongle.uf2` needs reflashing** for any of these changes — the
left/right halves keep their firmware.

What's added:

- **Mouse buttons with hold/drag** — hold a mouse-button key and move the ball to
  drag; the trackball report keeps held buttons pressed during movement.
- **Zoom** — `Ctrl`+wheel via `WM(MouseWheelUp/Down, LCtrl)`.
- **Discrete wheel keys** — `MouseWheelUp/Down/Left/Right`.
- **Trackball scrolling on a dedicated key** — hold the **SCROLL** key and move the
  ball to scroll (smooth, with a delta accumulator so slow motion still scrolls);
  release to go back to moving the cursor. Scrolling is suppressed while a mouse
  button is held (so dragging always wins). The scroll key is `User8`; place it from
  the **User** tab in Vial (a `SCROLL` keycode is exposed in `vial.json`).

### Patched RMK

The trackball scroll behaviour is not configurable in upstream RMK, so a local copy
of rmk 0.8.2 lives in [`rmk-patched/`](rmk-patched/) and is wired in via
`[patch.crates-io]` in [`Cargo.toml`](Cargo.toml). The only edits are in
`rmk-patched/src/input_device/pmw3610.rs` (layer-/key-aware scroll + accumulator) and
`rmk-patched/src/keyboard.rs` (shared `MOUSE_BUTTONS_STATE` / `SCROLL_KEY_HELD` flags).

### Tuning

In `rmk-patched/src/input_device/pmw3610.rs`:

- `SCROLL_DIVISOR` — scroll speed (higher = slower).
- Vertical direction — the sign on `scroll_acc_v` (`-= y` vs `+= y`); horizontal is
  the sign on `scroll_acc_h`.

After editing, rebuild with `cargo make uf2` and reflash `chary-dongle.uf2`.

## uf2 support

If you’re using the Adafruit_nRF52_Bootloader (pre-installed on the nice!nano), you’re in luck! This bootloader supports the .uf2 firmware format, which eliminates the need for a debugging probe to flash your firmware. RMK uses the `cargo-make` tool to generate .uf2 firmware, with the generation process defined in the `Makefile.toml`.

Follow these steps to generate and flash the .uf2 firmware with RMK:

1. Get `cargo-make` tool:
   ```shell
   cargo install --force cargo-make
   ```
2. Compile RMK and generates .uf2 firmware:
   ```shell
   cargo make uf2 --release
   ```
3. Flash

   - Put your board into bootloader mode. A USB drive will appear on your computer.
   - Drag and drop the generated .uf2 firmware file onto the USB drive. The RMK firmware will be automatically flashed onto your microcontroller.

   For additional details on entering bootloader mode and flashing firmware, refer to the [nice!nano documentation](https://nicekeyboards.com/docs/nice-nano/getting-started#flashing-firmware-and-bootloaders)

### Tips for nRF52840

Most nice!nano compatible boards have bootloader with SoftDevice pre-flashed. Since v0.7.x, RMK will remove old SoftDevice Bluetooth stack and replace it with its own. So if you want to rollback to v0.6.x, or switch to firmwares that use SoftDevice stack(for example, zmk), you will need to [re-flash the bootloader](https://nicekeyboards.com/docs/nice-nano/troubleshooting#my-nicenano-seems-to-be-acting-up-and-i-want-to-re-flash-the-bootloader).

### Additional notes

RMK defaults to USB-priority mode if a USB cable is connected. After flashing, remember to disconnect the USB cable, or [switch to BLE-priority mode](https://rmk.rs/docs/features/wireless.html#multiple-profile-support) by pressing User11(Switch Output) key.
