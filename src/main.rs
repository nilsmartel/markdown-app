use iced::*;
use widget::*;

fn main() -> Result {
    let document = Markdown {
        elements: {
            use crate::Element::*;
            vec![
                H1("Iced GUI programming".to_string()),
                Text("an introduction".to_string()),
                H2("Why?".to_string()),
                Text("a simple question that may occur, is why iced?".to_string()),
                H3("Portability".to_string()),
                Text(
                    "iced runs on a lot of platforms. It truly is a multiplatform GUI toolkit!"
                        .to_string(),
                ),
            ]
        },
    };

    Markdown::run(Settings {
        flags: document,
        ..Settings::default()
    })
}

#[derive(Clone, Debug, Default)]
struct Markdown {
    elements: Vec<Element>,
}

#[derive(Clone, Debug)]
enum Element {
    H1(String),
    H2(String),
    H3(String),
    Text(String),
    Divider,
}

#[derive(Debug, Clone)]
enum Message {
    Insert(Element, usize),
    Delete(usize),
}

impl Application for Markdown {
    type Executor = executor::Default;
    type Flags = Self;
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (flags, Command::none())
    }

    fn title(&self) -> String {
        // Tries to set the first headline as title of the window

        self.elements
            .iter()
            .find_map(|elem| match elem {
                Element::H1(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| String::from("Markdown App"))
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Insert(element, pos) => {
                self.elements.insert(pos, element);
            }
            Message::Delete(pos) => {
                self.elements.remove(pos);
            }
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let mut v = Vec::with_capacity(self.elements.len());
        for elem in self.elements.iter() {
            let elem = match elem {
                Element::H1(value) => iced::Element::<Message>::from(text(value).size(32)),
                Element::H2(value) => {
                    let t = text(value)
                        .size(24)
                        .horizontal_alignment(alignment::Horizontal::Center);

                    let t = widget::column![t, Rule::horizontal(1)];
                    iced::Element::<Message>::from(t)
                }
                Element::H3(value) => {
                    let t = text(value).size(18);
                    iced::Element::<Message>::from(t)
                }
                Element::Text(value) => {
                    let t = text(value).size(12);

                    iced::Element::<Message>::from(t)
                }
                Element::Divider => {
                    let ruler = Rule::horizontal(2);

                    iced::Element::<Message>::from(ruler)
                }
            };

            v.push(elem);
        }

        widget::Column::with_children(v).into()
    }
}
