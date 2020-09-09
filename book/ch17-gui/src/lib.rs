pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // @Note: `Box<dyn Draw>` is a trait object, which acts as a stand-in
    // for any type inside a `Box` that implements the `Draw` trait.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code to actually draw a button...
    }
}
