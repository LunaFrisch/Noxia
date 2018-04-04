use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use tungstenite::{Message, connect};
use url::Url;
use serde_json;
use threadpool;
use regex;

pub struct ThreadedClient {
    evnts_on_ch_jn: Arc<HashMap<String, Arc<::fchat::event::Event>>>,
    evnts_on_ch_lv: Arc<HashMap<String, Arc<::fchat::event::Event>>>,
    evnts_on_ch_inv: Arc<HashMap<String, Arc<::fchat::event::Event>>>,
    evnts_on_ch_msg: Arc<HashMap<String, Arc<::fchat::event::Event>>>,
    evnts_on_pm_msg: Arc<HashMap<String, Arc<::fchat::event::Event>>>,
    evnts_on_pm_and_ch_msg: Arc<HashMap<String, Arc<::fchat::event::Event>>>,
    cmds_on_ch_msg: Arc<HashMap<String, Arc<::fchat::command::Command>>>,
    cmds_on_pm_msg: Arc<HashMap<String, Arc<::fchat::command::Command>>>,
    cmds_on_pm_and_ch_msg: Arc<HashMap<String, Arc<::fchat::command::Command>>>,
    prefix: String,
    account: String,
    password: String,
    character: String,
    server: String,
}

impl ThreadedClient {

    pub fn new() -> Self {
        ThreadedClient {
            evnts_on_ch_jn: Arc::new(HashMap::new()),
            evnts_on_ch_lv: Arc::new(HashMap::new()),
            evnts_on_ch_inv: Arc::new(HashMap::new()),
            evnts_on_ch_msg: Arc::new(HashMap::new()),
            evnts_on_pm_msg: Arc::new(HashMap::new()),
            evnts_on_pm_and_ch_msg: Arc::new(HashMap::new()),
            cmds_on_ch_msg: Arc::new(HashMap::new()),
            cmds_on_pm_msg: Arc::new(HashMap::new()),
            cmds_on_pm_and_ch_msg: Arc::new(HashMap::new()),
            prefix: "".to_string(),
            account: "".to_string(),
            password: "".to_string(),
            character: "".to_string(),
            server: "".to_string(),
        }
    }

    pub fn prefix<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.prefix = s.to_string();
        self
    }

    pub fn account<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.account = s.to_string();
        self
    }

    pub fn password<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.password = s.to_string();
        self
    }

    pub fn character<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.character = s.to_string();
        self
    }

    pub fn server<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.server = s.to_string();
        self
    }

    pub fn register_event<'a, E: ::fchat::event::Event + 'static>(&'a mut self, s: E, e: ::fchat::event::OnEvent) -> &'a mut Self {
        match e {
            ::fchat::event::OnEvent::ChannelJoin => { Arc::get_mut(&mut self.evnts_on_ch_jn).unwrap().insert(::utility::return_type_of(&s), Arc::new(s)); },
            ::fchat::event::OnEvent::ChannelLeave => { Arc::get_mut(&mut self.evnts_on_ch_lv).unwrap().insert(::utility::return_type_of(&s), Arc::new(s)); },
            ::fchat::event::OnEvent::ChannelInvite => { Arc::get_mut(&mut self.evnts_on_ch_inv).unwrap().insert(::utility::return_type_of(&s), Arc::new(s)); },
            ::fchat::event::OnEvent::ChannelMessage => { Arc::get_mut(&mut self.evnts_on_ch_msg).unwrap().insert(::utility::return_type_of(&s), Arc::new(s)); },
            ::fchat::event::OnEvent::PrivateMessage => { Arc::get_mut(&mut self.evnts_on_pm_msg).unwrap().insert(::utility::return_type_of(&s), Arc::new(s)); },
            ::fchat::event::OnEvent::ChannelAndPrivateMessage => { Arc::get_mut(&mut self.evnts_on_pm_and_ch_msg).unwrap().insert(::utility::return_type_of(&s), Arc::new(s)); },
        }
        self
    }

    pub fn register_command<'a, C: ::fchat::command::Command + 'static>(&'a mut self, s: C, e: ::fchat::command::OnEvent, name: &str) -> &'a mut Self {
        println!("{}", ::utility::return_type_of(&s));
        match e {
            ::fchat::command::OnEvent::ChannelAndPrivateMessage => { Arc::get_mut(&mut self.cmds_on_pm_and_ch_msg).unwrap().insert(name.to_string(), Arc::new(s)); },
            ::fchat::command::OnEvent::ChannelMessage => { Arc::get_mut(&mut self.cmds_on_ch_msg).unwrap().insert(name.to_string(), Arc::new(s)); },
            ::fchat::command::OnEvent::PrivateMessage => { Arc::get_mut(&mut self.cmds_on_pm_msg).unwrap().insert(name.to_string(), Arc::new(s)); },
        }
        self
    }

    pub fn run<'a>(&'a mut self) -> Result<(), &'static str> {
        let login_ticket = ::flist::api::ApiTicket::new()
            .account(self.account.as_str())
            .password(self.password.as_str())
            .acquire();

        let login_info = ::fchat::message::LoginBuilder::new()
            .method("ticket")
            .account(self.account.as_str())
            .ticket(login_ticket.as_str())
            .character(self.character.as_str())
            .cname("Noxia")
            .cversion("0.3.0")
            .build();

        let (mut socket, _response) = connect(Url::parse(self.server.as_str()).unwrap())
            .expect("Cannot connect to server.");
        socket.write_message(Message::text(login_info)).unwrap();

        let pool = threadpool::Builder::new()
            .num_threads(4)
            .build();

        let mut last_msg_tick = Instant::now();

        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

        loop {
            let msg = socket.read_message()
                .expect("Error reading message from socket.");

            let tx = sender.clone();
            let evnts_ch_inv = self.evnts_on_ch_inv.clone();
            let evnts_ch_jn = self.evnts_on_ch_jn.clone();
            let evnts_ch_lv = self.evnts_on_ch_lv.clone();
            let evnts_ch_msg = self.evnts_on_ch_msg.clone();
            let evnts_pm_msg = self.evnts_on_pm_msg.clone();
            let evnts_ch_pm_msg = self.evnts_on_pm_and_ch_msg.clone();
            let cmds_ch_msg = self.cmds_on_ch_msg.clone();
            let cmds_pm_msg = self.cmds_on_pm_msg.clone();
            let cmds_pm_and_ch_msg = self.cmds_on_pm_and_ch_msg.clone();

            pool.execute(move || {
                let (opcode, mut json) = msg.to_text().unwrap().split_at(3);
                if json.len() > 1 {
                    json = &json[1..];
                }

                let mut json_data: serde_json::Value = serde_json::from_str(json).unwrap_or(json! {null});

                match opcode.parse::<::fchat::message::ServerOpcodes>().unwrap() {
                    ::fchat::message::ServerOpcodes::ADL => {},
                    ::fchat::message::ServerOpcodes::AOP => {},
                    ::fchat::message::ServerOpcodes::BRO => {},
                    ::fchat::message::ServerOpcodes::CDS => {},
                    ::fchat::message::ServerOpcodes::CHA => {},
                    ::fchat::message::ServerOpcodes::CIU => { for (_k, v) in evnts_ch_inv.iter() { let _ = v.execute(&json_data.clone(), &tx.clone()); } },
                    ::fchat::message::ServerOpcodes::CBU => {},
                    ::fchat::message::ServerOpcodes::CKU => {},
                    ::fchat::message::ServerOpcodes::COA => {},
                    ::fchat::message::ServerOpcodes::COL => {},
                    ::fchat::message::ServerOpcodes::CON => {},
                    ::fchat::message::ServerOpcodes::COR => {},
                    ::fchat::message::ServerOpcodes::CSO => {},
                    ::fchat::message::ServerOpcodes::CTU => {},
                    ::fchat::message::ServerOpcodes::DOP => {},
                    ::fchat::message::ServerOpcodes::ERR => {},
                    ::fchat::message::ServerOpcodes::FKS => {},
                    ::fchat::message::ServerOpcodes::FLN => {},
                    ::fchat::message::ServerOpcodes::HLO => {},
                    ::fchat::message::ServerOpcodes::ICH => {},
                    ::fchat::message::ServerOpcodes::IDN => {},
                    ::fchat::message::ServerOpcodes::JCH => { for(_k, v) in evnts_ch_jn.iter() { let _ = v.execute(&json_data.clone(), &tx.clone()); } },
                    ::fchat::message::ServerOpcodes::KID => {},
                    ::fchat::message::ServerOpcodes::LCH => { for(_k, v) in evnts_ch_lv.iter() { let _ = v.execute(&json_data.clone(), &tx.clone()); } },
                    ::fchat::message::ServerOpcodes::LIS => {},
                    ::fchat::message::ServerOpcodes::NLN => {},
                    ::fchat::message::ServerOpcodes::IGN => {},
                    ::fchat::message::ServerOpcodes::FRL => {},
                    ::fchat::message::ServerOpcodes::ORS => {},
                    ::fchat::message::ServerOpcodes::PIN => { /*ping*/ tx.send("PIN".into()).expect(""); },
                    ::fchat::message::ServerOpcodes::PRD => {},
                    ::fchat::message::ServerOpcodes::PRI => {
                        let cmd_regex = regex::Regex::new(r#"^(~\w+)"#).unwrap();

                        for (_k, v) in evnts_pm_msg.iter() {
                            let _ = v.execute(&json_data.clone(), &tx.clone());
                        }

                        let msg_value = json_data["message"].take();
                        let msg = msg_value.as_str().unwrap().to_string();

                        for cap in cmd_regex.captures_iter(msg.as_str()) {
                            let trimmed = msg.trim_left_matches(&cap[0]).to_string();
                            let trimmed = trimmed.trim_left().to_string();
                            let cmd = &cap[0][1..];

                            *json_data.get_mut("message").unwrap() = json!(trimmed);

                            for (k, v) in cmds_pm_and_ch_msg.iter() {
                                if cmd == k.as_str() {
                                    let _ = v.execute(&json_data.clone(), &tx.clone());
                                }
                            }

                            for (k, v) in cmds_pm_msg.iter() {
                                if cmd == k.as_str() {
                                    let _ = v.execute(&json_data.clone(), &tx.clone());
                                }
                            }
                        }
                    },
                    ::fchat::message::ServerOpcodes::MSG => {
                        let cmd_regex = regex::Regex::new(r#"^(~\w+)"#).unwrap();

                        for (_k, v) in evnts_ch_msg.iter() {
                            let _ = v.execute(&json_data.clone(), &tx.clone());
                        }

                        let msg_value = json_data["message"].take();
                        let msg = msg_value.as_str().unwrap().to_string();

                        for cap in cmd_regex.captures_iter(msg.as_str()) {
                            let trimmed = msg.trim_left_matches(&cap[0]).to_string();
                            let trimmed = trimmed.trim_left().to_string();
                            let cmd = &cap[0][1..];

                            *json_data.get_mut("message").unwrap() = json!(trimmed);

                            for (k, v) in cmds_pm_and_ch_msg.iter() {
                                if cmd == k.as_str() {
                                    let _ = v.execute(&json_data.clone(), &tx.clone());
                                }
                            }

                            for (k, v) in cmds_ch_msg.iter() {
                                if cmd == k.as_str() {
                                    let _ = v.execute(&json_data.clone(), &tx.clone());
                                }
                            }
                        }
                    },
                    ::fchat::message::ServerOpcodes::LRP => {},
                    ::fchat::message::ServerOpcodes::RLL => {},
                    ::fchat::message::ServerOpcodes::RMO => {},
                    ::fchat::message::ServerOpcodes::RTB => {},
                    ::fchat::message::ServerOpcodes::SFC => {},
                    ::fchat::message::ServerOpcodes::STA => {},
                    ::fchat::message::ServerOpcodes::SYS => {},
                    ::fchat::message::ServerOpcodes::TPN => {},
                    ::fchat::message::ServerOpcodes::UPT => {},
                    ::fchat::message::ServerOpcodes::VAR => {},
                }
            });

            if let Ok(data) = receiver.try_recv() {
                if last_msg_tick.elapsed() >= Duration::from_secs(1) {
                    socket.write_message(Message::Text(data)).unwrap();

                    last_msg_tick = Instant::now();
                } else {
                    println!("Failed to send message, requeuing message.");
                    sender.send(data).unwrap();
                }
            }
        }
    }
}