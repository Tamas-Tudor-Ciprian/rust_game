use crossterm::{
	cursor, 
	terminal::{self, ClearType,enable_raw_mode, disable_raw_mode},
	event::{self, Event, KeyCode},
	ExecutableCommand,
	};


use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;




#[derive(Clone)]
enum ScreenPixel {


	Crab,
	Fish,
	Wall,
	Empty,



}



struct Position{

	x:i32,
	y:i32,

}



struct Crab {
	position : Position,
	emoji : char,

}

impl Default for Crab{


	fn default() -> Self{
		Self{position: Position {x : 10, y: 10},
		emoji:'🦀',


		} }
}

fn main(){

	let _ = enable_raw_mode();

	let rows = 30;
	let collumns = 120;



	let mut screen_buffer  = vec![vec![ScreenPixel::Empty;collumns];rows];



	let mut stdout = stdout();

	terminal::enable_raw_mode().unwrap();
	stdout.execute(terminal::EnterAlternateScreen).unwrap();
	stdout.execute(terminal::Clear(ClearType::All)).unwrap();


	for i in 0..50 {
		//this is just a test
		stdout.execute(cursor::MoveTo(0,0)).unwrap();
		stdout.execute(terminal::Clear(ClearType::All)).unwrap();


		write!(stdout, "Frame{}",i).unwrap();
		stdout.flush().unwrap();

		sleep(Duration::from_millis(50));


	}
	

	loop{
	// this is the main game loop


	if event::poll(Duration::from_millis(0)).unwrap_or(false) {
		if let Ok(Event::Key(key)) = event::read() {
			match key.code{
				KeyCode::Char('q') => break,
				KeyCode::Left => {},
				KeyCode::Right => {},
				KeyCode::Up => {},
				KeyCode::Down => {},
				_ => {},

			}	

		}

	}

}




	stdout.execute(cursor::Show).unwrap();
	stdout.execute(terminal::LeaveAlternateScreen).unwrap();
	terminal::disable_raw_mode().unwrap();



	let _ = disable_raw_mode();

}
