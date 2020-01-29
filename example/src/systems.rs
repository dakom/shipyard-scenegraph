use shipyard::prelude::*;
use rand::prelude::*;
use crate::components::*;
use crate::geometry::*;
use crate::config::*;

pub const TICK:&'static str = "TICK";

pub fn register_workloads(world:&World) {
    world.add_workload::<(Start, HandleController, Update, Commit, Render, End), _>(TICK); 
}

#[system(Start)]
pub fn run (mut fps_counter: Unique<&mut FpsCounter>) {
    fps_counter.begin();
}

#[system(HandleController)]
pub fn run (
    mut entities:EntitiesMut, 
    controller: Unique<&Controller>, 
    mut positions: &mut Position, 
    mut speeds:&mut Speed, 
    mut gravities:&mut Gravity, 
    stage_area:Unique<&StageArea>, 
    img_area:Unique<&ImageArea>,
    mut instance_positions:Unique<&mut InstancePositions>
) {
    if *controller == Controller::Adding {
        let count = positions.len();
        let len = count + N_BUNNIES_PER_TICK;
        let stage_size = &stage_area.0;
        let img_size = &img_area.0;


        for count in 0..N_BUNNIES_PER_TICK {
            //alternate between corners
            let pos_x = match count % 2 {
                0 => 0.0f64,
                _ => (stage_size.width - img_size.width) as f64
            };

            let pos_y = (stage_size.height - img_size.height) as f64;
            let position = Point { x: pos_x, y: pos_y };

            let mut speed = Point::new_random();

            speed.x *= 10.0;
            speed.y = (speed.y * 10.0) - 5.0;

            entities.add_entity((&mut positions, &mut speeds, &mut gravities), (Position(position), Speed(speed), Gravity(START_GRAVITY)));
        }

        instance_positions.0.resize(len * 2, 0.0);
    }
}

#[system(Update)]
pub fn run (
    mut positions: &mut Position, 
    mut speeds:&mut Speed, 
    mut gravities:&mut Gravity, 
    stage_area:Unique<&StageArea>, 
    img_area:Unique<&ImageArea>,
) {

    let stage_size = &stage_area.0;
    let img_size = &img_area.0;

    (&mut positions, &mut speeds, &mut gravities)
        .iter()
        .for_each(|(pos, speed, gravity)| {
            let mut pos = &mut pos.0;
            let mut speed = &mut speed.0;
            let gravity = &gravity.0;

            //movement is made to match https://github.com/pixijs/bunny-mark/blob/master/src/Bunny.js
            pos.x += speed.x;
            pos.y -= speed.y;
        
            speed.y += *gravity;

            let bounds_right = (stage_size.width - img_size.width) as f64;
            if pos.x > bounds_right {
                speed.x *= -1.0;
                pos.x = bounds_right;
            } else if pos.x < 0.0 {
                speed.x *= -1.0;
                pos.x = 0.0 
            }

            let bounds_top = (stage_size.height - img_size.height) as f64;
        
            if pos.y < 0.0 {
                speed.y *= -0.85;
                pos.y = 0.0;
                let rand_bool:bool = thread_rng().gen();
                if rand_bool  {
                    let rand_float:f64 = thread_rng().gen();
                    speed.y  -= rand_float * 6.0;
                }
            } else if pos.y > bounds_top {
                speed.y = 0.0;
                pos.y = bounds_top;
            }


    });
}


#[system(Commit)]
pub fn run (
    positions: &Position, 
    mut instance_positions:Unique<&mut InstancePositions>
) {

    let instance_positions = &mut instance_positions.0;

    (&positions)
        .iter()
        .enumerate()
        .for_each(|(index, pos)| {
            //Set the instance data from bunny positions
            let instance_idx = index * 2;
            instance_positions[instance_idx] = pos.0.x as f32;
            instance_positions[instance_idx+1] = pos.0.y as f32;
    });
}


#[system(Render)]
pub fn run (
    mut renderer: Unique<NonSendSync<&mut SceneRenderer>>,
    positions: &Position, 
    stage_area:Unique<&StageArea>, 
    img_area:Unique<&ImageArea>,
    instance_positions:Unique<&InstancePositions>
) {
    renderer.render(positions.len(), &img_area.0, &stage_area.0, &instance_positions.0).unwrap();
}

#[system(End)]
pub fn run (mut fps_counter: Unique<&mut FpsCounter>, hud: Unique<NonSendSync<&mut Hud>>, positions:&Position) {
    fps_counter.end();
    let fps = fps_counter.current.ceil() as u32;
    let len = positions.len();
    hud.update(len, fps);
}