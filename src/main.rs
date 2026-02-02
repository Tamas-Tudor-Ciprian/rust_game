use crossterm::{
	cursor, 
	terminal::{self, ClearType,enable_raw_mode, disable_raw_mode},
	event::{self, Event, KeyCode},
	ExecutableCommand,
	};


use std::io::{stdout, Write,Stdout};
use std::thread::sleep;
use std::time::{Instant,Duration};
use std::sync::atomic::{AtomicU64, Ordering};

use rand::Rng;


static DELTA_TIME_NS: AtomicU64 = AtomicU64::new(0);


static NR_FISH : i32 = 4;


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
		Self{position: Position {x : 10.0, y: (SCREEN_MEASURES.1 as f64 - 1.0)},
		speed : 1.0,
		emoji:'🦀',
	} }
}


struct Fish {
	position : Position,
	speed : f64,
	emoji : char,
}

impl Default for Fish{

	fn default() -> Self{
		let mut rng = rand::thread_rng();
		Self{position: Position {x : rng.gen::<f64>() * 29.0 + 1.0, y :2.0},
		speed : 2.0,
		emoji : '🐟',
		}

	}
}

impl Fish{
	fn move_down(&mut self){ 

		self.position.y += self.speed * get_delta_time().as_secs_f64();
	
		}
}


fn display_framerate(out : &mut Stdout, start_time : &mut Instant){

	let fps = 1.0 / get_delta_time().as_secs_f64();


	out.execute(cursor::MoveTo(0,0)).unwrap();


	write!(out,"FPS:{}",fps).unwrap();

	}


fn display_speed(out : &mut Stdout,crab :&Crab){


	out.execute(cursor::MoveTo(SCREEN_MEASURES.0.try_into().unwrap(),25));


	write!(out,"SPEED:{}",crab.speed).unwrap();
}


fn display_score(out : &mut Stdout, score :&i64){



	out.execute(cursor::MoveTo(SCREEN_MEASURES.0 as u16 + 3,25));

	write!(out,"SCORE:{}",score).unwrap();
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


fn shoal_manager(shoal : &mut Vec<Fish>, score : &mut i64, crab : &Crab, out : &mut Stdout){





	for fish in shoal.iter(){


			erase_fish(out,fish);


	}




	shoal.retain(|fish| { fish.position.y <= (SCREEN_MEASURES.1 - 2).try_into().unwrap()});

	for fish in shoal.iter_mut(){
		fish.move_down();
		}

	if shoal.len() <= NR_FISH as usize{
		shoal.push(Fish::default());
	}
	

}

fn display_shoal(out : &mut Stdout, shoal : &mut Vec<Fish>){
	for fish in shoal{
		out.execute(cursor::MoveTo(fish.position.x as u16, fish.position.y as u16)).unwrap();
		write!(out,"{}",fish.emoji).unwrap();
	}
}


fn erase_fish(out : &mut Stdout, fish : &Fish){
	

	out.execute(cursor::MoveTo(fish.position.x as u16, fish.position.y as u16)).unwrap();
	write!(out," ").unwrap();

	}


fn get_delta_time() -> Duration {

	Duration::from_nanos(DELTA_TIME_NS.load(Ordering::Relaxed))

}



fn main(){


	let mut last = Instant::now();

	let _ = enable_raw_mode();


	let mut shoal : Vec<Fish> = Vec::new();

	shoal.push(Fish::default());


	let mut score = 0;




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

	let now = Instant::now();
	let dt = now.duration_since(last);
	last = now;


	DELTA_TIME_NS.store(dt.as_nanos() as u64, Ordering::Relaxed);



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

		
		shoal_manager(&mut shoal, &mut score, &crab, &mut stdout);
		display_shoal(&mut stdout, &mut shoal);
		


		//this is where the display function calls go
		let mut start_time = Instant::now();
		display_framerate(&mut stdout,&mut start_time);
		display_score(&mut stdout, &score);
		

}




//	stdout.execute(cursor::Show).unwrap();
	stdout.execute(terminal::LeaveAlternateScreen).unwrap();
	terminal::disable_raw_mode().unwrap();



	let _ = disable_raw_mode();

}
