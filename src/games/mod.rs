pub mod siege;

use crate::Result;

pub trait Game 
{
    fn startup(&mut self) -> Result<()>;
    fn game_loop(&mut self) -> Result<()>;
}