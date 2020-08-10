pub struct List {
    head: Link,
}

enum Link {
    Enum,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}