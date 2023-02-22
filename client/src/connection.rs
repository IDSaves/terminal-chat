use std::{
  net::TcpStream, 
  io::{
    self, 
    Write,
    Error, 
    ErrorKind, BufRead
  },
};

use crate::types::{
  SygnalType, 
  SygnalHeader, 
  SygnalData,
  AuthStatus
};

pub struct Connection {
  pub stream: TcpStream,
  reader: io::BufReader<TcpStream>
}

impl Connection {
  pub fn new(address: &str, connection_type: SygnalType, username: &str) -> io::Result<Connection> {
    let signal = SygnalData::new(
      vec![
        SygnalHeader::SygnalType(connection_type),
        SygnalHeader::Username(username.to_owned())
      ],
      None
    );
    let mut connection = TcpStream::connect(address)?;
    connection.write_all(signal.to_string().as_bytes())?;
    let reader = io::BufReader::new(connection.try_clone()?);

    let mut instance = Connection {
      stream: connection,
      reader
    };

    if let SygnalType::ConnectionProducer = connection_type {
      let data_from_socket = instance.read_signal(None)?;
      if data_from_socket.contains(&AuthStatus::DENIED.to_string()) {
        return Err(Error::new(ErrorKind::ConnectionAborted, "Access denied"));
      }
    }
  
    return Ok(instance)
  }

  pub fn read_signal(&mut self, max_read_try: Option<u8>) -> io::Result<String> {
    let mut res_line = String::new();
    let mut headers_read = false;
    let mut fail_reads_count: u8 = 0;
    loop {
      let mut buf_line = String::new();
      match self.reader.read_line(&mut buf_line) {
        Err(e) => {
          match e.kind() {
            io::ErrorKind::WouldBlock => {
              if let Some(max_fails) = max_read_try {
                fail_reads_count += 1;
                if fail_reads_count == max_fails {
                  return Err(Error::new(ErrorKind::ConnectionAborted, "Connection aborted"))
                }
              }
              continue;
            },
            _ => panic!("Got an error: {}", e),
          }
        },
        Ok(m) => {
          if m == 0 {
            return Err(Error::new(ErrorKind::BrokenPipe, "Connection closed"))
          }
          m
        },
      };
      res_line.push_str(&buf_line);
  
      if res_line.ends_with("\r\n\r\n"){
        if !res_line.contains(&SygnalHeader::WithMessage.to_string()) || headers_read {
          break;
        }
        headers_read = true;
      }
    }
  
    Ok(res_line)
  }
}