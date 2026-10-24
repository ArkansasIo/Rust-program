use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum InstanceState { Forming, Active, Complete, Resetting }
#[derive(Debug, Clone)] pub struct Instance { pub id: u64, pub template_id: String, pub owner_group: u64, pub players: Vec<u64>, pub state: InstanceState, pub elapsed_ticks: u64 }
#[derive(Debug, Default)] pub struct InstanceManager { pub instances: HashMap<u64, Instance>, next_id: u64 }
impl InstanceManager {
    pub fn create(&mut self, template: impl Into<String>, group: u64, players: Vec<u64>) -> u64 { self.next_id+=1; let id=self.next_id; self.instances.insert(id,Instance{id,template_id:template.into(),owner_group:group,players,state:InstanceState::Forming,elapsed_ticks:0}); id }
    pub fn tick(&mut self) { for i in self.instances.values_mut() { if i.state==InstanceState::Active { i.elapsed_ticks+=1; } } }
    pub fn complete(&mut self,id:u64){if let Some(i)=self.instances.get_mut(&id){i.state=InstanceState::Complete;}}
    pub fn remove(&mut self,id:u64)->Option<Instance>{self.instances.remove(&id)}
}
