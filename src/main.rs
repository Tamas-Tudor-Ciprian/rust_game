use crossterm::{
	cursor, 
	terminal::{self, ClearType,enable_raw_mode, disable_raw_mode},
	event::{self, Event, KeyCode},
	ExecutableCommand,
	style::*,
	};


use std::io::{stdout, Write,Stdout};
use std::thread::sleep;
use std::time::{Instant,Duration};
use std::sync::atomic::{AtomicU64, Ordering};

use rand::Rng;


static DELTA_TIME_NS: AtomicU64 = AtomicU64::new(0);


static NR_FISH : i32 = 4;

static FISH_SPAWN_DELAY : f64 = 2.0;


const SCREEN_MEASURES: (i32,i32) = (60,30);

#[derive(Copy, Clone)]
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
	last_position: Position,
	speed : f64,
	emoji : char,
	timestamp :Instant,
}

impl Default for Fish{

	fn default() -> Self{
		let mut rng = rand::thread_rng();
		let position = Position { x : rng.gen::<f64>() * (SCREEN_MEASURES.0 as f64 - 3.0) + 2.0 , y : 2.0};
		Self{
		position,
		last_position : position,
		speed : 2.0,
		emoji : '🐟',
		timestamp : Instant::now(),
		}

	}
}

impl Fish{
	fn move_down(&mut self) -> bool{ 

		self.position.y += self.speed * get_delta_time().as_secs_f64();

		if self.last_position.y as u16 != self.position.y as u16 {
			true

		}else{
			false

		}
	
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

	out.execute(cursor::MoveTo(SCREEN_MEASURES.0 as u16 + 9,25));
	write!(out,"   ").unwrap();
	out.execute(cursor::MoveTo(SCREEN_MEASURES.0 as u16 + 9, 25));
	write!(out,"{}",score).unwrap();
}

fn make_walls(out : &mut Stdout){
	out.execute(cursor::MoveTo(0,1)).unwrap();
	for _ in 0..SCREEN_MEASURES.0/2{
		write!(out,"🧱").unwrap();
	}

	out.execute(cursor::MoveTo(0,SCREEN_MEASURES.1.try_into().unwrap())).unwrap();
	for _ in 0..SCREEN_MEASURES.0/2{
		write!(out,"🧱").unwrap();
	}

	for left_rail in 1..=SCREEN_MEASURES.1{	
	
		out.execute(cursor::MoveTo(0,left_rail.try_into().unwrap())).unwrap();
		write!(out,"🧱").unwrap();
		out.execute(cursor::MoveTo(SCREEN_MEASURES.0.try_into().unwrap(),left_rail.try_into().unwrap())).unwrap();
		write!(out,"🧱").unwrap();
	
	}
		
}

fn you_won(out: &mut Stdout){


	let win_screen = vec![
				"🪸🪸🪸🪸🪸🪸🪸🪸",
				"🪸   YOU WON! 🪸",
				"🪸  CRAB RAVE 🪸",
				"🪸🪸🪸🪸🪸🪸🪸🪸",
				


				];

	let colors = [
		Color::Red,
		Color::Yellow,
		Color::Green,
		Color::Cyan,
		Color::Blue,
		Color::Magenta,
		];
	let mut i = 0;	

		let mut rng = rand::thread_rng();
		let color = colors[rng.gen::<usize>() % colors.len()];
		out.execute(SetForegroundColor(color)).unwrap();
	for line in win_screen{
		out.execute(cursor::MoveTo((SCREEN_MEASURES.0/2 - 6) as u16, (SCREEN_MEASURES.1/2 + i) as u16)).unwrap();
		write!(out,"{}",line);
		out.flush().unwrap();
		i += 1;
	}


	//this resets the color just in case
	out.execute(SetForegroundColor(Color::White)).unwrap();

	}





fn shoal_manager(shoal : &mut Vec<Fish>, score : &mut i64, crab : &Crab, out : &mut Stdout){



	for fish in shoal.iter_mut(){


			if fish.move_down(){
				out.execute(cursor::MoveTo(fish.last_position.x as u16, fish.last_position.y as u16)).unwrap();
				write!(out," ",).unwrap();
				fish.last_position = fish.position;
				out.execute(cursor::MoveTo(fish.position.x as u16, fish.position.y as u16)).unwrap();
				write!(out,"\x08{}",fish.emoji).unwrap();

				}
				


				}



	let first_fish = shoal.first().unwrap();
	if shoal.first().unwrap().position.y >(SCREEN_MEASURES.1 - 2).try_into().unwrap(){
		let x_diff = (first_fish.position.x - crab.position.x).abs();
		let y_diff = (first_fish.position.y - crab.position.y).abs();
		if x_diff < 2.0 && y_diff <2.0{
		
			*score += 1;

		}else{
			*score -=1;
		}
		out.execute(cursor::MoveTo(first_fish.position.x as u16,first_fish.position.y as u16)).unwrap();
		write!(out," ");
		shoal.remove(0);
	}


	out.execute(cursor::MoveTo((SCREEN_MEASURES.0+1) as u16,(SCREEN_MEASURES.1+1) as u16)).unwrap();




	let time_since_last_fish = (Instant::now() -shoal.last().unwrap().timestamp).as_secs_f64();
	if shoal.len() <= NR_FISH as usize && time_since_last_fish > FISH_SPAWN_DELAY{
		shoal.push(Fish::default());
	}
	

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
	let mut old_score = 1;


	let mut stdout = stdout();

	terminal::enable_raw_mode().unwrap();
	stdout.execute(terminal::EnterAlternateScreen).unwrap();
	stdout.execute(terminal::Clear(ClearType::All)).unwrap();

		sleep(Duration::from_millis(50));

	let mut crab : Crab = Crab::default();

	stdout.execute(cursor::MoveTo(crab.position.x as u16,crab.position.y as u16)).unwrap();
	write!(stdout,"{}",crab.emoji).unwrap();
	stdout.flush().unwrap();


	//this will put the score display, so that it does not update the whole word every time:

	stdout.execute(cursor::MoveTo(SCREEN_MEASURES.0 as u16 + 3,25));
	write!(stdout,"SCORE:");


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
					if (crab.position.x - crab.speed) > 0 as f64{
						crab.position.x -= crab.speed;
						stdout.execute(cursor::MoveTo(crab.position.x as u16,crab.position.y as u16)).unwrap();
						write!(stdout,"{}",crab.emoji).unwrap();
						stdout.flush().unwrap();
					}
					},
				KeyCode::Right => {
					if (crab.position.x + crab.speed) < SCREEN_MEASURES.0 as f64 - 1.0{
						crab.position.x += crab.speed;
						stdout.execute(cursor::MoveTo(crab.position.x as u16,crab.position.y as u16)).unwrap();
						write!(stdout,"{}",crab.emoji).unwrap();
						stdout.flush().unwrap();
					}

					},
				KeyCode::Up => {},
				KeyCode::Down => {},
				_ => {},

			}	

		}

	}

		
		shoal_manager(&mut shoal, &mut score, &crab, &mut stdout);


		//this is where the display function calls go
		let mut start_time = Instant::now();
		//display_framerate(&mut stdout,&mut start_time);
		if old_score != score {
		display_score(&mut stdout, &score);
		old_score = score;

		if score == 10{
			
			for _ in 0..500{	
			you_won(&mut stdout);
			sleep(Duration::from_millis(10));
			}
	
			break;
		}

		}
		
}

	
	//stdout.execute(cursor::Show).unwrap();
	stdout.execute(terminal::LeaveAlternateScreen).unwrap();
	terminal::disable_raw_mode().unwrap();


	let _ = disable_raw_mode();

}
