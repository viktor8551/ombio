# Engine initialization and window lifecycle

Use a builder to configure and prepare the engine: `build()` returns the prepared engine without creating or showing a window, and `run()` creates the window and processes its events. This keeps configuration separate from execution and avoids showing a window before its event loop is running.
