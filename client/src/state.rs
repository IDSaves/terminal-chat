use std::{
  sync::{
    mpsc::{
      Sender, 
      Receiver, 
      self
    }, 
    Arc
  }, 
  io::{
    self,
    Write
  }
};

use parking_lot::Mutex;

pub struct State {
  pub username: String,
  pub chat_reload_receiver: Option<Receiver<()>>,
  pub chat_reload_sender: Sender<()>,
  pub user_input: Arc<Mutex<String>>,
  pub messages: Arc<Mutex<Vec<String>>>
}

impl State {
  pub fn new() -> io::Result<State> {
    let (sx, rx) = mpsc::channel::<()>();
    let user_input = Arc::new(Mutex::new(String::new()));
    let messages = Arc::new(Mutex::new(Vec::<String>::new()));

    let mut instance = State {
      username: String::new(),
      chat_reload_receiver: Some(rx),
      chat_reload_sender: sx,
      user_input,
      messages,
    };

    instance.read_username()?;

    Ok(instance)
  }

  fn read_username(&mut self) -> io::Result<()> {
    println!("{}", termion::clear::All);
    print!("Username: ");
    std::io::stdout().flush()?;

    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    self.username = username.trim().to_owned();
    println!("{}", termion::clear::All);

    Ok(())
  }
}