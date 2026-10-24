use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progression { pub level:u32, pub xp:u64, pub skill_points:u32 }
impl Default for Progression { fn default()->Self{Self{level:1,xp:0,skill_points:0}} }
impl Progression { pub fn required_xp(level:u32)->u64{(level as u64).saturating_pow(2)*500} pub fn add_xp(&mut self,amount:u64){self.xp=self.xp.saturating_add(amount);while self.xp>=Self::required_xp(self.level){self.xp-=Self::required_xp(self.level);self.level+=1;self.skill_points+=1;}} }
