use iced::{Settings, Task};
use screenshot::{MouseEvent, ScreenshotStreamEvent};

mod screenshot;

const V5_BRAIN_RAW_SCREEN_SIZE: (u32, u32) = (512, 272);
const V5_BRAIN_SCREEN_SIZE: (u32, u32) = (480, 272);

#[derive(Debug, Clone)]
enum MouseUpdate {
    Press,
    Release,
}

#[derive(Debug, Clone)]
enum Message {
    Screenshot(iced::widget::image::Handle),
    Mouse(MouseUpdate),
    MouseMove(u16, u16),
    MouseEventSender(tokio::sync::mpsc::Sender<MouseEvent>),
}

struct V5BrainStream {
    image: iced::widget::image::Handle,
    mouse_event_sender: Option<tokio::sync::mpsc::Sender<MouseEvent>>,
    mouse_position: (u16, u16),
    mouse_pressed: bool,
}

impl Default for V5BrainStream {
    fn default() -> Self {
        let image = iced::widget::image::Handle::from_rgba(512, 272, vec![0; 512 * 272 * 4]);

        V5BrainStream {
            image,
            mouse_event_sender: None,
            mouse_position: (0, 0),
            mouse_pressed: false,
        }
    }
}

impl V5BrainStream {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Screenshot(image) => {
                self.image = image;
            }
            Message::MouseMove(x, y) => {
                self.mouse_position = (x, y);
                if self.mouse_pressed {
                    if let Some(sender) = &self.mouse_event_sender {
                        sender
                            .try_send(MouseEvent {
                                x,
                                y,
                                pressing: true,
                            })
                            .unwrap_or_else(|_| {
                                eprintln!("Failed to send mouse event: {:?}", message);
                            });
                    } else {
                        eprintln!("Mouse event sender not initialized");
                    }
                }
            }
            Message::Mouse(event) => {
                if let Some(sender) = &self.mouse_event_sender {
                    sender
                        .try_send(match event {
                            MouseUpdate::Press => {
                                self.mouse_pressed = true;
                                MouseEvent {
                                    x: self.mouse_position.0,
                                    y: self.mouse_position.1,
                                    pressing: true,
                                }
                            }
                            MouseUpdate::Release => {
                                self.mouse_pressed = false;
                                MouseEvent {
                                    x: self.mouse_position.0,
                                    y: self.mouse_position.1,
                                    pressing: false,
                                }
                            }
                        })
                        .unwrap_or_else(|_| {
                            eprintln!("Failed to send mouse event: {:?}", event);
                        });
                } else {
                    eprintln!("Mouse event sender not initialized");
                }
            }

            Message::MouseEventSender(sender) => {
                self.mouse_event_sender = Some(sender);
            }
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        iced::widget::mouse_area(iced::widget::image(self.image.clone()))
            .on_press(Message::Mouse(MouseUpdate::Press))
            .on_release(Message::Mouse(MouseUpdate::Release))
            .on_move(|pos| Message::MouseMove(pos.x as u16, pos.y as u16))
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::run(screenshot::screenshot_stream).map(|event| match event {
            ScreenshotStreamEvent::Image(image) => Message::Screenshot(image),
            ScreenshotStreamEvent::Sender(sender) => Message::MouseEventSender(sender),
        })
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
