#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateTag {
    Draft,
    Published,
}

trait State {
    fn publish(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn tag(&self) -> StateTag;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Draft {}

#[derive(Clone, PartialEq, Debug)]
pub struct Published {}

impl State for Draft {
    fn publish(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn tag(&self) -> StateTag {
        StateTag::Draft
    }
}

impl State for Published {
    fn publish(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn tag(&self) -> StateTag {
        StateTag::Published
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: "".to_string(),
        }
    }
    pub fn add_content(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn publish(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.publish());
        }
    }
    pub fn reject(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject());
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish() {
        let mut post = Post::new();
        post.publish();
        assert_eq!(
            post.state.as_ref().map(|s| s.tag()).unwrap(),
            StateTag::Published
        );
    }

    #[test]
    fn test_reject() {
        let mut post = Post::new();
        post.reject();
        assert_eq!(
            post.state.as_ref().map(|s| s.tag()).unwrap(),
            StateTag::Draft
        );
    }

    #[test]
    fn test_add_content() {
        let mut post = Post::new();
        post.add_content("content");
        assert_eq!(post.content, "content".to_string());
    }
}
