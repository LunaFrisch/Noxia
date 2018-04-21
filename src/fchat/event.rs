use serde_json;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;

use diesel::pg::PgConnection;

pub trait Event: Send + Sync + 'static {
    fn execute(&self, &serde_json::Value, &Sender<String>, &Arc<Mutex<PgConnection>>) -> Result<(), &'static str>;
    fn
}

impl Event for Arc<Event> {
    fn execute(&self, v: &serde_json::Value, s: &Sender<String>, d: &Arc<Mutex<PgConnection>>) -> Result<(), &'static str> {
        (**self).execute(v, s, d)
    }
}

macro_rules! event {
    ($name:ident($data:ident, $out:ident, $db:ident) $code:block) => {
        #[allow(non_camel_case_types)]
        pub struct $name;

         impl $crate::fchat::event::Event for $name {
            #[allow(unreachable_code, unused_mut)]
            fn execute(&self, $data: &serde_json::Value, $out: &Sender<String>, $db: &Arc<Mutex<PgConnection>>) -> Result<(), &'static str> {
                $code

                Ok(())
            }
         }
    };
}

#[allow(dead_code)]
pub enum OnEvent {
    ChannelMessage,
    ChannelInvite,
    ChannelLeave,
    ChannelJoin,
    PrivateMessage,
    ChannelAndPrivateMessage,
}