pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {

        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
