//good one

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

        loop {
            let tmp = anchor;
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
