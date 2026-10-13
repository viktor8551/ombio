# OMBIO

OMBIO is a 2D game engine. Applications describe the pictures displayed in its window.

## Language

**Game**:
The application running in OMBIO, including the state that determines what it draws.

**Frame**:
One complete picture drawn by the application for display in the window.

**Drawable**:
An object that describes a picture within a frame, including its placement. It may combine other drawables into one picture.

**Image**:
A reusable 2D picture that may be shared by multiple sprites, without a position in a frame.

**Sprite**:
A positioned use of an image within a frame. Transparent parts of the image allow pictures beneath it to remain visible.
