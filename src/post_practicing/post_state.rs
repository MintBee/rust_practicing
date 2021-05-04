mod post_practicing {
    trait PostState {
        fn request_approval(self: Box<Self>) -> Box<dyn PostState>;
        fn approve(self: Box<Self>) -> Box<dyn PostState>;
    }

    struct Draft{}

    impl PostState for Draft {

    }
}