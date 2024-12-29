use iced::Task;

mod screenshot;

const V5_BRAIN_RAW_SCREEN_SIZE: (u32, u32) = (512, 272);
const V5_BRAIN_SCREEN_SIZE: (u32, u32) = (480, 272);

struct V5BrainStream {
    image: iced::widget::image::Handle,
}

impl Default for V5BrainStream {
    fn default() -> Self {
        let image = iced::widget::image::Handle::from_rgba(512, 272, vec![0; 512 * 272 * 4]);

        V5BrainStream { image }
    }
}

impl V5BrainStream {
    fn update(
        &mut self,
        message: iced::widget::image::Handle,
    ) -> iced::Task<iced::widget::image::Handle> {
        self.image = message;
        Task::none()
    }

    fn view(&self) -> iced::Element<'_, iced::widget::image::Handle> {
        iced::widget::image(self.image.clone()).into()
    }

    fn subscription(&self) -> iced::Subscription<iced::widget::image::Handle> {
        iced::Subscription::run(|| screenshot::screenshot_stream())
    }
}

fn main() -> iced::Result {
    iced::application(
        "V5 Brain Stream",
        V5BrainStream::update,
        V5BrainStream::view,
    )
    .subscription(V5BrainStream::subscription)
    .window_size(iced::Size::new(
        V5_BRAIN_SCREEN_SIZE.0 as f32,
        V5_BRAIN_SCREEN_SIZE.1 as f32,
    ))
    .resizable(false)
    .run()
}
