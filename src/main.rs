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



const SCREEN_MEASURES: (i32,i32) = (60,30);

#[derive(Clone)]
enum ScreenPixel {


	Crab,
	Fish,
	Wall,
	Empty,

}

struct Position{

	x:f64,
	y:f64,

}



struct Crab {
	position : Position,
	speed : f64,
	emoji : char,

}

impl Default for Crab{


	fn default() -> Self{
		Self{position: Position {x : 10.0, y: 20.0},
		speed : 0.25,
		emoji:'🦀',
	} }
}





fn display_framerate(out : &mut Stdout, start_time : &mut Instant){


	
	let mut time_now = Instant::now();

	let elapsed_time = time_now - *start_time;


	*start_time = time_now;

	let fps = 1.0 / elapsed_time.as_secs_f64();


	out.execute(cursor::MoveTo(0,0)).unwrap();


	write!(out,"FPS:{}",fps).unwrap();

	}


fn display_speed(out : &mut Stdout,crab :&Crab){


	out.execute(cursor::MoveTo(SCREEN_MEASURES.0.try_into().unwrap(),25));


	write!(out,"SPEED:{}",crab.speed).unwrap();
}

fn make_walls(out : &mut Stdout){


	out.execute(cursor::MoveTo(0,1)).unwrap();
	for _ in 0..SCREEN_MEASURES.0{
		write!(out,"#").unwrap();
	}

	out.execute(cursor::MoveTo(0,SCREEN_MEASURES.1.try_into().unwrap())).unwrap();
	for _ in 0..SCREEN_MEASURES.0{
		write!(out,"#").unwrap();
	}

	for left_rail in 1..=SCREEN_MEASURES.1{	
	
		out.execute(cursor::MoveTo(0,left_rail.try_into().unwrap())).unwrap();	
		write!(out,"#").unwrap();
		out.execute(cursor::MoveTo(SCREEN_MEASURES.0.try_into().unwrap(),left_rail.try_into().unwrap())).unwrap();
		write!(out,"#").unwrap();
	
	}
		
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

	stdout.execute(cursor::MoveTo(crab.position.x as u16,crab.position.y as u16)).unwrap();
	write!(stdout,"{}",crab.emoji).unwrap();
	stdout.flush().unwrap();


	make_walls(&mut stdout);

	loop{
	// this is the main game loop

	if event::poll(Duration::from_millis(0)).unwrap_or(false) {
		if let Ok(Event::Key(key)) = event::read() {
			match key.code{
				KeyCode::Char('q') => break,
				KeyCode::Left => {
					crab.position.x -= crab.speed;
					stdout.execute(cursor::MoveTo(crab.position.x as u16,crab.position.y as u16)).unwrap();

					write!(stdout,"{}",crab.emoji).unwrap();
					stdout.flush().unwrap();
					},
				KeyCode::Right => {
					crab.position.x += crab.speed;
					stdout.execute(cursor::MoveTo(crab.position.x as u16,crab.position.y as u16)).unwrap();
					write!(stdout,"{}",crab.emoji).unwrap();
					stdout.flush().unwrap();


					},
				KeyCode::Up => {},
				KeyCode::Down => {},
				_ => {},

			}	

		}

	}

		//I will attempt to have some logic to display the frame rate here
		
		let mut start_time = Instant::now();

		display_framerate(&mut stdout,&mut start_time);
		

}




//	stdout.execute(cursor::Show).unwrap();
	stdout.execute(terminal::LeaveAlternateScreen).unwrap();
	terminal::disable_raw_mode().unwrap();



	let _ = disable_raw_mode();

}
