use super::Comment;

pub struct CommentAdded<P> {
    pub(super) poll: P,
    pub(super) comment: Comment
}
