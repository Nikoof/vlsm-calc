use crate::vlsm::Vlsm;
use iced::widget::{column, container, radio, row, text, text_input, toggler};
use iced::{Alignment, Color, Element, Length, Sandbox, Theme};
use ipnetwork::Ipv4Network;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opt {
    IncludeFirstLastAddress(bool),
}

#[derive(Default)]
pub struct Opts {
    include_first_last_address: bool,
}

#[derive(Default)]
pub struct Calculator {
    theme: Theme,
    net_input: String,
    sizes_input: String,
    output_text: String,
    opts: Opts,
}

#[derive(Debug, Clone)]
pub enum Message {
    CIDRInputChanged(String),
    SizesInputChanged(String),
    Submit,
    ThemeChanged(ThemeType),
    OptsChanged(Opt),
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self {
            theme: Theme::Dark,
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("VLSM Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CIDRInputChanged(inp) => self.net_input = inp,
            Message::SizesInputChanged(inp) => self.sizes_input = inp,
            Message::ThemeChanged(theme_type) => {
                self.theme = match theme_type {
                    ThemeType::Light => Theme::Light,
                    ThemeType::Dark => Theme::Dark,
                }
            }
            Message::OptsChanged(Opt::IncludeFirstLastAddress(b)) => {
                self.opts.include_first_last_address = b;
            }
            Message::Submit => {}
        }
        if let Ok(net) = self.net_input.parse::<Ipv4Network>() {
            let sizes: Vec<u32> = self
                .sizes_input
                .split(',')
                .filter_map(|str| str.parse().ok())
                .collect();

            self.output_text = match net.vlsm(&sizes, self.opts.include_first_last_address) {
                Ok(subnets) => subnets.into_iter().map(|s| format!("{}\n", s)).collect(),
                Err(why) => format!("Error: {}", why),
            }
        } else {
            self.output_text = String::from("Error: Invalid CIDR")
        }
    }

    fn view(&self) -> Element<Message> {
        let title = text("VLSM Calculator")
            .size(70)
            .style(Color::from([0.5, 0.5, 0.5]));

        let net_input = text_input("Address space to subnet (CIDR)", &self.net_input)
            .on_input(Message::CIDRInputChanged)
            .on_submit(Message::Submit)
            .size(30)
            .padding(15);

        let sizes_input = text_input("Subnet sizes (comma-delimited)", &self.sizes_input)
            .on_input(Message::SizesInputChanged)
            .on_submit(Message::Submit)
            .size(30)
            .padding(15);

        let choose_theme = [ThemeType::Light, ThemeType::Dark].iter().fold(
            column![text("Choose theme:").size(25)].spacing(10),
            |column, theme| {
                column.push(
                    radio(
                        format!("{theme:?}"),
                        *theme,
                        Some(match self.theme {
                            Theme::Light => ThemeType::Light,
                            Theme::Dark => ThemeType::Dark,
                            Theme::Custom { .. } => ThemeType::Dark,
                        }),
                        Message::ThemeChanged,
                    )
                    .size(25),
                )
            },
        );

        let change_opts = column![
            text("Options:").size(25),
            toggler(
                String::from("Include network and broadcast addresses"),
                self.opts.include_first_last_address,
                |b| Message::OptsChanged(Opt::IncludeFirstLastAddress(b))
            )
            .size(25),
        ]
        .spacing(10);

        let output = text(&self.output_text).size(30);

        let content = column![
            title,
            net_input,
            sizes_input,
            row![choose_theme, change_opts].spacing(100),
            output
        ]
        .width(700)
        .spacing(20)
        .align_items(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
