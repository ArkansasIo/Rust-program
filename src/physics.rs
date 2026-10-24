use crate::core::Vec2;

pub fn move_toward(position:Vec2,target:Vec2,speed:f32,delta_seconds:f32)->Vec2{let delta=target-position;let distance=delta.length();if distance==0.0{return position}let step=(speed.max(0.0)*delta_seconds).min(distance);position+delta.normalized()*step}
pub fn within_range(a:Vec2,b:Vec2,radius:f32)->bool{a.distance(b)<=radius.max(0.0)}
