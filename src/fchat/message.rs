use std::str::FromStr;

pub struct LoginBuilder {
    method: String,
    account: String,
    ticket: String,
    character: String,
    cname: String,
    cversion: String,
}

impl LoginBuilder {
    pub fn new() -> Self {
        LoginBuilder {
            method: "".to_string(),
            account: "".to_string(),
            ticket: "".to_string(),
            character: "".to_string(),
            cname: "".to_string(),
            cversion: "".to_string(),
        }
    }

    pub fn method<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.method = s.to_string();
        self
    }

    pub fn account<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.account = s.to_string();
        self
    }

    pub fn ticket<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.ticket = s.to_string();
        self
    }

    pub fn character<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.character = s.to_string();
        self
    }

    pub fn cname<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.cname = s.to_string();
        self
    }

    pub fn cversion<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.cversion = s.to_string();
        self
    }

    pub fn build(&self) -> String {
        let mut result = String::from(r#"IDN {"method": ""#);
        result.push_str(self.method.as_str());
        result.push_str(r#"", "account": ""#);
        result.push_str(self.account.as_str());
        result.push_str(r#"", "ticket": ""#);
        result.push_str(self.ticket.as_str());
        result.push_str(r#"", "character": ""#);
        result.push_str(self.character.as_str());
        result.push_str(r#"", "cname": ""#);
        result.push_str(self.cname.as_str());
        result.push_str(r#"", "cversion": ""#);
        result.push_str(self.cversion.as_str());
        result.push_str(r#""}"#);

        result
    }
}

pub struct InviteResponseBuilder {
    channel: String,
    name: String,
}

impl InviteResponseBuilder {
    pub fn new() -> Self {
        InviteResponseBuilder {
            channel: "".to_string(),
            name: "".to_string(),
        }
    }

    pub fn channel<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.channel = s.to_string();
        self
    }

    pub fn name<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.name = s.to_string();
        self
    }

    pub fn build(&self) -> String {
        let mut result = String::from(r#"JCH {"channel": ""#);
        result.push_str(self.channel.as_str());
        result.push_str(r#""}"#);

        result
    }
}

pub struct MessageBuilder {
    channel: String,
    data: Vec<String>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder {
            channel: "".to_string(),
            data: Vec::new(),
        }
    }

    pub fn channel<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.channel = s.to_string();
        self
    }

    pub fn push<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.data.push(s.to_string());
        self
    }

    pub fn build(&mut self) -> String {
        let mut result = String::from(r#"MSG {"channel": ""#);
        result.push_str(self.channel.as_str());
        result.push_str(r#"", "message": ""#);

        while let Some(s) = self.data.pop() {
            result.push_str(s.as_str());
        }

        result.push_str(r#""}"#);

        result
    }
}

pub struct PrivateMessageBuilder {
    recipient: String,
    data: Vec<String>,
}

impl PrivateMessageBuilder {
    pub fn new() -> Self {
        PrivateMessageBuilder {
            recipient: "".to_string(),
            data: Vec::new(),
        }
    }

    pub fn recipient<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.recipient = s.to_string();
        self
    }

    pub fn push<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.data.push(s.to_string());
        self
    }

    pub fn build(&mut self) -> String {
        let mut result = String::from(r#"PRI {"recipient": ""#);
        result.push_str(self.recipient.as_str());
        result.push_str(r#"", "message": ""#);

        for x in self.data.drain(..) {
            result.push_str(x.as_str());
        }

        result.push_str(r#""}"#);

        println!("[VIXEN] {}", result.as_str());

        result
    }
}

pub struct ChannelCreateBuilder {
    data: String,
}

impl ChannelCreateBuilder {
    pub fn new() -> Self {
        ChannelCreateBuilder {
            data: "".to_string(),
        }
    }

    pub fn name<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.data = s.to_string();
        self
    }

    pub fn build(&mut self) -> String {
        let mut result = String::from(r#"CCR {"channel": ""#);
        result.push_str(self.data.as_str());
        result.push_str(r#"" }"#);
        //let result = format!(r#"CCR {"channel": "{}"}"#, self.data.as_str());
        result
    }
}

pub struct ChannelDescriptionBuilder {
    channel: String,
    data: String,
}

impl ChannelDescriptionBuilder {
    pub fn new() -> Self {
        ChannelDescriptionBuilder {
            channel: "".to_string(),
            data: "".to_string(),
        }
    }

    pub fn desc<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.data = s.to_string();
        self
    }

    pub fn channel<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.channel = s.to_string();
        self
    }

    pub fn build(&mut self) -> String {
        let mut result = String::from(r#"CDS {"channel": ""#);
        result.push_str(self.channel.as_str());
        result.push_str(r#"", "description": ""#);
        result.push_str(self.data.as_str());
        result.push_str(r#"" }"#);
        result
    }
}

pub struct ChannelInviteBuilder {
    channel: String,
    character: String,
}

impl ChannelInviteBuilder {
    pub fn new() -> Self {
        ChannelInviteBuilder {
            channel: "".to_string(),
            character: "".to_string(),
        }
    }

    pub fn channel<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.channel = s.to_string();
        self
    }

    pub fn character<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.character = s.to_string();
        self
    }

    pub fn build(&mut self) -> String {
        let mut result = String::from(r#"CIU {"channel": ""#);
        result.push_str(self.channel.as_str());
        result.push_str(r#"", "character": ""#);
        result.push_str(self.character.as_str());
        result.push_str(r#""}"#);
        result
    }
}

pub struct ChannelLeaveBuilder {
    channel: String,
}

impl ChannelLeaveBuilder {
    pub fn new() -> Self {
        ChannelLeaveBuilder {
            channel: "".to_string(),
        }
    }

    pub fn channel<'a>(&'a mut self, s: &str) -> &'a mut Self {
        self.channel = s.to_string();
        self
    }

    pub fn build(&mut self) -> String {
        let mut result = String::from(r#"LCH {"channek": ""#);
        result.push_str(self.channel.as_str());
        result.push_str(r#""}"#);
        result
    }
}

pub enum ServerOpcodes {
    ADL,
    AOP,
    BRO,
    CDS,
    CHA,
    CIU,
    CBU,
    CKU,
    COA,
    COL,
    CON,
    COR,
    CSO,
    CTU,
    DOP,
    ERR,
    FKS,
    FLN,
    HLO,
    ICH,
    IDN,
    JCH,
    KID,
    LCH,
    LIS,
    NLN,
    IGN,
    FRL,
    ORS,
    PIN,
    PRD,
    PRI,
    MSG,
    LRP,
    RLL,
    RMO,
    RTB,
    SFC,
    STA,
    SYS,
    TPN,
    UPT,
    VAR,
}

impl FromStr for ServerOpcodes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADL" => Ok(ServerOpcodes::ADL),
            "AOP" => Ok(ServerOpcodes::AOP),
            "BRO" => Ok(ServerOpcodes::BRO),
            "CDS" => Ok(ServerOpcodes::CDS),
            "CHA" => Ok(ServerOpcodes::CHA),
            "CIU" => Ok(ServerOpcodes::CIU),
            "CBU" => Ok(ServerOpcodes::CBU),
            "CKU" => Ok(ServerOpcodes::CKU),
            "COA" => Ok(ServerOpcodes::COA),
            "COL" => Ok(ServerOpcodes::COL),
            "CON" => Ok(ServerOpcodes::CON),
            "COR" => Ok(ServerOpcodes::COR),
            "CSO" => Ok(ServerOpcodes::CSO),
            "CTU" => Ok(ServerOpcodes::CTU),
            "DOP" => Ok(ServerOpcodes::DOP),
            "ERR" => Ok(ServerOpcodes::ERR),
            "FKS" => Ok(ServerOpcodes::FKS),
            "FLN" => Ok(ServerOpcodes::FLN),
            "HLO" => Ok(ServerOpcodes::HLO),
            "ICH" => Ok(ServerOpcodes::ICH),
            "IDN" => Ok(ServerOpcodes::IDN),
            "JCH" => Ok(ServerOpcodes::JCH),
            "KID" => Ok(ServerOpcodes::KID),
            "LCH" => Ok(ServerOpcodes::LCH),
            "LIS" => Ok(ServerOpcodes::LIS),
            "NLN" => Ok(ServerOpcodes::NLN),
            "IGN" => Ok(ServerOpcodes::IGN),
            "FRL" => Ok(ServerOpcodes::FRL),
            "ORS" => Ok(ServerOpcodes::ORS),
            "PIN" => Ok(ServerOpcodes::PIN),
            "PRD" => Ok(ServerOpcodes::PRD),
            "PRI" => Ok(ServerOpcodes::PRI),
            "MSG" => Ok(ServerOpcodes::MSG),
            "LRP" => Ok(ServerOpcodes::LRP),
            "RLL" => Ok(ServerOpcodes::RLL),
            "RMO" => Ok(ServerOpcodes::RMO),
            "RTB" => Ok(ServerOpcodes::RTB),
            "SFC" => Ok(ServerOpcodes::SFC),
            "STA" => Ok(ServerOpcodes::STA),
            "SYS" => Ok(ServerOpcodes::SYS),
            "TPN" => Ok(ServerOpcodes::TPN),
            "UPT" => Ok(ServerOpcodes::UPT),
            "VAR" => Ok(ServerOpcodes::VAR),
            _ => Err(()),
        }
    }
}