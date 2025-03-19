use cosmic::iced::{Color, Rectangle, Size};
use cosmic::widget;
use cosmic::widget::canvas;
use cosmic_comp::hooks::{Decorations, Hooks};
use cosmic_comp::shell::element::stack::DefaultDecorations as DefaultStackDecorations;
use cosmic_comp::shell::element::window::DefaultDecorations as DefaultWindowDecorations;
use std::sync::Arc;

#[derive(Debug)]
struct AddIndicator<Lower> {
    lower: Lower,
}

struct Circle {
    radius: f32,
    color: Color,
}

// Then, we implement the `Program` trait
impl<Message, Theme, Renderer: cosmic::iced_renderer::geometry::Renderer>
    canvas::Program<Message, Theme, Renderer> for Circle
{
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: cosmic::iced::mouse::Cursor,
    ) -> Vec<Renderer::Geometry> {
        let bounds = bounds.size();
        let min = bounds.height.min(bounds.width);
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, Size::new(min, min));

        // We create a `Path` representing a simple circle
        let circle = canvas::Path::circle(frame.center(), self.radius);

        // And fill it with some color
        frame.fill(&circle, self.color);

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

impl<Internal, Message: std::clone::Clone + 'static, Lower: Decorations<Internal, Message>>
    Decorations<Internal, Message> for AddIndicator<Lower>
{
    fn height(&self, window: &Internal) -> i32 {
        self.lower.height(window)
    }
    fn view(&self, window: &Internal) -> cosmic::Element<'_, Message> {
        let orig = self.lower.view(window);
        let height = self.lower.height(window);
        widget::row()
            .push(
                widget::column()
                    .push(canvas(Circle {
                        radius: (height as f32 / 2.) * 0.8,
                        color: Color::new(1.0, 0.0, 0.0, 1.0),
                    }))
                    .width(height as f32),
            )
            .push(orig)
            .into()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cosmic_comp::run(Hooks {
        window_decorations: Some(Arc::new(AddIndicator {
            lower: DefaultWindowDecorations,
        })),
        stack_decorations: Some(Arc::new(AddIndicator {
            lower: DefaultStackDecorations,
        })),
    })
}
