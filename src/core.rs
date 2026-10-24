use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamId(pub u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameTick(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 { pub x: f32, pub y: f32 }
impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub fn distance(self, other: Self) -> f32 { ((self.x-other.x).powi(2)+(self.y-other.y).powi(2)).sqrt() }
    pub fn length(self) -> f32 { self.distance(Self::ZERO) }
    pub fn normalized(self) -> Self { let l=self.length(); if l==0.0 { Self::ZERO } else { Self{x:self.x/l,y:self.y/l} } }
}
impl Add for Vec2 { type Output=Self; fn add(self,rhs:Self)->Self{Self{x:self.x+rhs.x,y:self.y+rhs.y}} }
impl AddAssign for Vec2 { fn add_assign(&mut self,rhs:Self){self.x+=rhs.x;self.y+=rhs.y;} }
impl Sub for Vec2 { type Output=Self; fn sub(self,rhs:Self)->Self{Self{x:self.x-rhs.x,y:self.y-rhs.y}} }
impl Mul<f32> for Vec2 { type Output=Self; fn mul(self,rhs:f32)->Self{Self{x:self.x*rhs,y:self.y*rhs}} }
