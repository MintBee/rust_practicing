// bad one

type Link = Option<Box<Node>>;

struct Node {
    next: Link
}

struct Recursive {
    root: Link
}

impl Recursive {
    fn back(&mut self) -> &mut Link {
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
