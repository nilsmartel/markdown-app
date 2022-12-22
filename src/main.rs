use alignment::*;
use iced::*;
use widget::*;

mod md_style {
    pub const SIZE_H1: u16 = 48;
    pub const SIZE_H2: u16 = 38;
    pub const SIZE_H3: u16 = 32;
    pub const SIZE_TEXT: u16 = 18;
    pub const SIZE_DIVIDER: u16 = 4;

    pub const PADDING: u16 = 16;
    pub const PADDING_TOP: [u16; 4] = [16, 0, 0, 0];
}

fn main() -> Result {
    let document = Markdown {
        elements: {
            use crate::MdNode::*;
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
                H3("Easy".to_string()),
                Text(
                    "it's so easy to use! I threw this together with barely any knowledge!"
                        .to_string(),
                ),
                H3("Momentum".to_string()),
                Text(
                    "With companies like system76 backing the development of iced, this is promising to become very mature soon"
                        .to_string(),
                ),
                Divider,
                Text("that's all folks!".to_string()),
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
    elements: Vec<MdNode>,
}

#[derive(Clone, Debug)]
enum MdNode {
    H1(String),
    H2(String),
    H3(String),
    Text(String),
    Divider,
}

#[derive(Debug, Clone)]
enum Message {
    Insert(MdNode, usize),
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
                MdNode::H1(s) => Some(s.clone()),
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

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let mut v = Vec::with_capacity(self.elements.len());
        for elem in self.elements.iter() {
            let elem: Element<Message> = match elem {
                MdNode::H1(value) => {
                    let elem = text(value).size(md_style::SIZE_H1);
                    widget::column![elem].padding(md_style::PADDING_TOP).into()
                }
                MdNode::H2(value) => {
                    let elem = text(value).size(md_style::SIZE_H2);

                    widget::column![
                        // headline
                        elem,
                        // ___
                        Rule::horizontal(md_style::SIZE_DIVIDER)
                    ]
                    .padding(md_style::PADDING_TOP)
                    .into()
                }
                MdNode::H3(value) => {
                    let elem = text(value)
                        .size(md_style::SIZE_H3)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center);

                    widget::column![elem].padding(md_style::PADDING_TOP).into()
                }
                MdNode::Text(value) => text(value).size(md_style::SIZE_TEXT).into(),
                MdNode::Divider => Rule::horizontal(md_style::SIZE_DIVIDER).into(),
            };

            v.push(elem);
        }

        // by setting the padding on the column, we can avoid having to pay the top padding while scrolling.
        let column = widget::Column::with_children(v).padding(md_style::PADDING);

        scrollable(column).into()
    }
}
