// use iced::{
//     alignment::{Horizontal, Vertical},
//     font::Family,
//     widget::{button,Button,column, text, Text},
//     Font, Length, Sandbox, Settings,
// };


// fn main() -> iced::Result {
//     MyApp::run(Settings::default())
// }

// //tipo de mensaje personalizado
// #[derive(Debug, Clone)]
// enum MyAppMessage {
//     DoSomething,
// }

// struct MyApp;

// impl Sandbox for MyApp {
//     //type -> es un tipo asociado que se usa para definir el tipo de mensaje
//     // que se puede enviar a la aplicacion
//     type Message = MyAppMessage;

//     fn new() -> Self {
//         Self
//     }

//     fn title(&self) -> String {
//         String::from("Ejemplo basico con Iced")
//     }

//     fn update(&mut self, _message: Self::Message) {}

//     fn view(&self) -> iced::Element<Self::Message> {
//         column![
//             Button::new("Doom Emacs"),
//             //Text::new("Holaa"),
//             text("Agregar fuentes").font(Font {
//                 family: Family::Fantasy,
//                 ..Font::DEFAULT
//             }),
//             text("Centrar")
//                 .width(Length::Fill)
//                 .horizontal_alignment(Horizontal::Center).size(24),
//             text("Centrar verticalmente")
//                 .height(Length::Fill)
//                 .vertical_alignment(Vertical::Center),
//             button("Doom Emacs").on_press(MyAppMessage::DoSomething)
//                                 .padding(10),
//         ]
//         .into()
//     }
// }
//NOTE: .into() es necesario para convertir el widget en un Elemen
// Un Element es un tipo de widget que puede ser renderizado en la pantalla


//NOTE: Ejemplo de Login usando textinputs,button y progress bar :

use iced::{
    widget::{Button, Column, ProgressBar, Text, TextInput},
    Sandbox, Settings,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    UpdateUsername(String),
    UpdatePassword(String),
    Login,
}

#[derive(Default)]
struct MyApp {
    username: String,
    password: String,
    progress: f32, 
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Trutraveler App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MyAppMessage::UpdateUsername(username) => self.username = username,
            MyAppMessage::UpdatePassword(password) => self.password = password,
            MyAppMessage::Login => {
                println!("Intento de login con usuario: {} y contraseña: {}", self.username, self.password);
                //NOTE: Simular el proceso de la barra
                self.progress = 50.0; 
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        Column::new()
            .spacing(20)
            .padding(40)
            .push(Text::new("Trutraveler").size(40))
            .push(Text::new("Hola viajero biemvenido a Trujillo!!"))
            .push(TextInput::new("Username", &self.username)
                .on_input(MyAppMessage::UpdateUsername)
                .padding(10)
                .size(20))
            .push(TextInput::new("Password", &self.password)
                .secure(true)
                .on_input(MyAppMessage::UpdatePassword)
                .padding(10)
                .size(20))
            .push(Button::new("Login")
                .on_press(MyAppMessage::Login)
                .padding(10))
            .push(ProgressBar::new(0.0..=100.0, self.progress)) 
            .into()
    }
}
