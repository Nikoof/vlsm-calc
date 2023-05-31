use crate::vlsm::Vlsm;
use iced::widget::{column, container, text, text_input};
use iced::{Alignment, Color, Element, Length, Sandbox};
use ipnetwork::Ipv4Network;

#[derive(Default)]
pub struct Calculator {
    net_input: String,
    sizes_input: String,
    output_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    CIDRInputChanged(String),
    SizesInputChanged(String),
    Submit,
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("VLSM Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CIDRInputChanged(inp) => self.net_input = inp,
            Message::SizesInputChanged(inp) => self.sizes_input = inp,
            Message::Submit => {
                if let Ok(net) = self.net_input.parse::<Ipv4Network>() {
                    let sizes: Vec<u32> = self
                        .sizes_input
                        .split(',')
                        .filter_map(|str| str.parse().ok())
                        .collect();

                    self.output_text = match net.vlsm(&sizes) {
                        Ok(subnets) => subnets.into_iter().map(|s| format!("{}\n", s)).collect(),
                        Err(why) => format!("Error: {}", why),
                    }
                } else {
                    self.output_text = String::from("Error: Invalid CIDR")
                }
            }
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

        let output = text(&self.output_text).size(30);

        let content = column![title, net_input, sizes_input, output]
            .width(700)
            .spacing(20)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
