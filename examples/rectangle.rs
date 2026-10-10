use walzer::{Frame, Game, Rectangle};

struct RectangleDemo {
    rectangle: Rectangle,
}

impl Game for RectangleDemo {
    fn draw(&mut self, frame: &mut Frame) {
        frame.draw_rectangle(self.rectangle);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    walzer::run(RectangleDemo {
        rectangle: Rectangle {
            position: [100.0, 100.0],
            size: [160.0, 100.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
    })
}
