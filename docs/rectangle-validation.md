# Rectangle rendering validation

Issue: [#11](https://github.com/viktor8551/walzer/issues/11), part of
[#9](https://github.com/viktor8551/walzer/issues/9).

Checked on Windows on 2026-10-12 using the public `Game`/`Frame` example in
separate processes. The available display reported 96 DPI (scale factor 1.0).
Client-area captures were inspected visually; a local, temporary Windows harness
also checked color regions with tolerance, their bounds, window existence, and
normal closure. These are observations on this machine, not a portable,
pixel-perfect screenshot test suite.

| Initial configuration | Observed output |
| --- | --- |
| `640 × 480`, rectangle `(50, 50)`, size `200 × 100` | Amber rectangle over green, occupying x=50–249 and y=50–149. Fully visible. |
| `180 × 120`, same rectangle | Visible portion occupies x=50–179 and y=50–119. Clips at the right and bottom content boundaries. |
| `640 × 480`, rectangle `(80, 30)`, size `120 × 160`, color `rgb(64, 112, 224)` | Blue rectangle occupies x=80–199 and y=30–189. All changed application values affect the picture. |
| `640 × 480`, rectangle `(-30, -20)`, size `200 × 100` | Visible portion occupies x=0–169 and y=0–79. Clips at the left and top content boundaries. |
| `640 × 480`, background clear only | Green background, with no rectangle. Prior background rendering still works. |
| Prepare, wait three seconds, then run | No visible application window during preparation. Running opens the configured window and draws the rectangle. |

Each process returned exit code 0 after `WM_CLOSE`; the inspected application
window no longer existed, and stderr contained no rendering errors.

The renderer converts logical coordinates using the window's scale factor.
Higher initial DPI scaling could not be checked on the available display.
Post-startup size and DPI changes remain the next slice's validation scope.

Build checks: `cargo check --all-targets`, `cargo build --example window`,
`cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, and
`cargo test --all-targets` pass. There are currently no automated test cases in
the repository; the test command checks that all test targets build and run.
