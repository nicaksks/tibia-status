use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use serde::Deserialize;

use crate::model::server::{self, Tsqp};

#[derive(Debug, Deserialize)]
pub struct ProtocolGame {
    pub ip: String,
    pub port: String,
}

impl ProtocolGame {
    const INFO: [u8; 8] = [0x06, 0x00, 0xFF, 0xFF, 0x69, 0x6E, 0x66, 0x6F];

    #[allow(unused_must_use)]
    pub fn server_status(&self) -> Result<server::Server, (i16, &str)> {
        let addr = &format!("{}:{}", self.ip, self.port);
        let conn = TcpStream::connect(addr);

        match conn {
            Ok(mut server) => {
                server.set_write_timeout(Some(Duration::from_secs(10)));
                server.set_read_timeout(Some(Duration::from_secs(10))); 
                server.write_all(&Self::INFO);

                let mut result = Vec::new();
                server.read_to_end(&mut result);
                
                return Ok(self.xml_to_string(&self.vec_to_string(result)))?;
            }
            Err(_) => Err((404, "server.not.found")),
        }
    }

    fn vec_to_string(&self, vec: Vec<u8>) -> String {
        println!("Vec -> {:?}", vec);
        String::from_utf8(vec).unwrap()
    }

    fn xml_to_string(&self, xml: &str) -> Result<server::Server, (i16, &str)> {
        println!("Xml -> {:?}", xml);
        let json = xmltojson::to_json(xml).unwrap();
        match serde_json::from_value::<Tsqp>(json) {
            Ok(Tsqp { mut tsqp }) => {
                if let Some(unique) = tsqp.players.unique_players {
                    tsqp.players.bots = Some(tsqp.players.online - unique);
                }
                Ok(tsqp)
            }
            Err(_) => Err((500, "internal.server.error"))
        }
    }
}
