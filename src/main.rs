use crossterm::{
	cursor, 
	terminal::{self, ClearType,enable_raw_mode, disable_raw_mode},
	event::{self, Event, KeyCode},
	ExecutableCommand,
	};


use std::io::{stdout, Write,Stdout};
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;



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
		Self{position: Position {x : 10, y: 20},
		emoji:'🦀',


		} }
}





fn display_framerate(out : &mut Stdout, start_time : &mut Instant){


	
	let mut time_now = Instant::now();

	let elapsed_time = *start_time - time_now;


	*start_time = time_now;

	let fps = 1.0 / elapsed_time.as_secs_f64();


	out.execute(cursor::MoveTo(0,0)).unwrap();

	write!(out,"        ");


	out.execute(cursor::MoveTo(0,0)).unwrap();


	write!(out,"FPS:{}",fps);




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



		sleep(Duration::from_millis(50));





	let mut crab : Crab = Crab::default();

	stdout.execute(cursor::MoveTo(crab.position.x.try_into().unwrap(),crab.position.y.try_into().unwrap())).unwrap();
	write!(stdout,"{}",crab.emoji).unwrap();
	stdout.flush().unwrap();

	loop{
	// this is the main game loop


	if event::poll(Duration::from_millis(500)).unwrap_or(false) {
		if let Ok(Event::Key(key)) = event::read() {
			match key.code{
				KeyCode::Char('q') => break,
				KeyCode::Left => {
					crab.position.x -= 1;
					stdout.execute(cursor::MoveTo(crab.position.x.try_into().unwrap(),crab.position.y.try_into().unwrap())).unwrap();

					write!(stdout,"{}",crab.emoji).unwrap();
					stdout.flush().unwrap();
					},
				KeyCode::Right => {
					crab.position.x +=1;
					stdout.execute(cursor::MoveTo(crab.position.x.try_into().unwrap(),crab.position.y.try_into().unwrap())).unwrap();
					write!(stdout,"{}",crab.emoji).unwrap();
					stdout.flush().unwrap();


					},
				_ => {},

			}	

		}

	}

		//I will attempt to have some logic to display the frame rate here


}




	stdout.execute(cursor::Show).unwrap();
	stdout.execute(terminal::LeaveAlternateScreen).unwrap();
	terminal::disable_raw_mode().unwrap();



	let _ = disable_raw_mode();

}
