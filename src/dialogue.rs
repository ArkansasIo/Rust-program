use serde::{Deserialize,Serialize};
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct DialogueChoice {pub id:String,pub text:String,pub next_node:Option<String>}
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct DialogueNode {pub id:String,pub speaker:String,pub text:String,pub choices:Vec<DialogueChoice>}
#[derive(Debug,Default,Clone,Serialize,Deserialize)] pub struct DialogueTree {pub id:String,pub start_node:String,pub nodes:Vec<DialogueNode>}
impl DialogueTree {pub fn node(&self,id:&str)->Option<&DialogueNode>{self.nodes.iter().find(|n|n.id==id)}}
