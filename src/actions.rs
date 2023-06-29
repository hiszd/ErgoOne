use core::marker::ConstParamTy;

#[derive(ConstParamTy, PartialEq, Eq)]
pub enum ActionType {
    Enqueue,
    Dequeue,
    LedColor,
    LedAnim,
}

impl From<&str> for ActionType {
    fn from(s: &str) -> Self {
        match s {
            "enqueue" => ActionType::Enqueue,
            "dequeue" => ActionType::Dequeue,
            "ledcolor" => ActionType::LedColor,
            "ledanim" => ActionType::LedAnim,
        }
    }
}

impl From<ActionType> for &str {
    fn from(a: ActionType) -> Self {
        match a {
            ActionType::Enqueue => "enqueue",
            ActionType::Dequeue => "dequeue",
            ActionType::LedColor => "ledcolor",
            ActionType::LedAnim => "ledanim",
        }
    }
}

pub struct Action {
    typ: ActionType,
}

// what this struct needs:
// 1. execute specific actions based on the type
// 2. look for specific parameters based on the action
impl Action {
    pub fn new_enqueue(typ: ActionType) -> Self {
        Action {
            typ: ActionType::Enqueue,
        }
    }
    pub fn new_dequeue(typ: ActionType) -> Self {
        Action {
            typ: ActionType::Dequeue,
        }
    }
    pub fn new_ledcolor(typ: ActionType) -> Self {
        Action {
            typ: ActionType::LedColor,
        }
    }
    pub fn new_ledanim(typ: ActionType) -> Self {
        Action {
            typ: ActionType::LedAnim,
        }
    }

    pub fn execute(&mut self) {
        match self.typ {
            ActionType::Enqueue => {}
            ActionType::Dequeue => {}
            ActionType::LedColor => {}
            ActionType::LedAnim => {}
        }
    }
}


// What I want to be able to do:
// from the key action functions(e.g. tap, hold, off, idle) I want to be able to execute an action
// within the scope of the main function by passing a action caller through the constructor of the
// matrix that ends up being called from those action functions.
// I want the function call to look something like this:
// action("actiontype", {parameters});
// or
// action("actiontype", (parameters));
