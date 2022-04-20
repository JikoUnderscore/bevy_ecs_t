use bevy_ecs::prelude::{Component, Mut};
use sdl2::rect::Rect;

#[derive(Component)]
pub struct Player{
}


#[derive(Component)]
pub struct Bandit{
}





#[derive(Component, Debug)]
pub struct Movement{
    pub movetype: MoveMove,
    // pub chase: Option<Mut<'m, Movement>>,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub enum MoveMove{
    Player,
    Bandit,
}


impl Movement{
    pub fn player_movement(&mut self, mut textrect: Mut<TexRect>, mut accs: Mut<Acceleration>, fps_dt: f64) {
        self.x += accs.x * fps_dt;
        self.y += accs.y * fps_dt;

        // println!("{} {}", movem.x, movem.y);
        textrect.pos.set_x(self.x as i32);
        textrect.pos.set_y(self.y as i32);

        accs.x = 0.0;
        accs.y = 0.0;
    }
    pub fn bandit_movement(&mut self, mut textrect: Mut<TexRect>, mut accs: Mut<Acceleration>, fps_dt: f64, pla_pos: &Movement) {
        let mut dir_x = (pla_pos.x - self.x) as f64;
        let mut dir_y = (pla_pos.y - self.y) as f64;

        let hyp = (dir_x * dir_x + dir_y * dir_y).sqrt();
        dir_x /= hyp;
        dir_y /= hyp;

        accs.x = dir_x * 5.0;
        accs.y = dir_y * 5.0;

        self.x += accs.x * fps_dt;
        self.y += accs.y * fps_dt;

        // println!("{} {}", movem.x, movem.y);
        textrect.pos.set_x(self.x as i32);
        textrect.pos.set_y(self.y as i32);

        accs.x = 0.0;
        accs.y = 0.0;

    }

}









#[derive(Component)]
pub struct Acceleration {
    pub x: f64,
    pub y: f64,
}

#[derive(Component)]
pub struct TexRect {
    pub srs: Rect,
    pub pos: Rect,
}

