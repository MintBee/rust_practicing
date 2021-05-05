// bad one

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    next: Option<Box<Node>>
}

struct Recursive {
    root: Option<Box<Node>>
}

impl Recursive {
    fn back(&mut self) -> &mut Option<Box<Node>> {
        let mut anchor = &mut self.root;
        while let Some(ref mut node) = *anchor {
            anchor = &mut node.next;
        }
        anchor
    }
}

fn main() {
    let mut link = Recursive {root: Some(Box::new(Node {next: None}))};
    let next = link.back();
}
