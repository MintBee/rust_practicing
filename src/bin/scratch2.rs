//good one

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

        loop {
            let tmp = anchor;
            println!("{:?}", anchor);
            if let Some(ref mut node) = *tmp {
                anchor = &mut node.next;
            } else {
                anchor = tmp;
                break;
            }
        }

        anchor
    }
}

fn main() {}
