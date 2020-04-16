use crate::contest::comment::{BriefComment, Comment};
use crate::contest::poll::Poll;
use crate::contest::Updatable;
use crop_infra::pg::{
    comment::{CommentTable, NewComment},
    Connection,
};

pub struct CommentAdded<P> {
    pub(super) poll: P,
    pub(super) comment: BriefComment,
}

impl<P> Updatable for CommentAdded<P>
where
    P: Poll,
{
    fn save(&self, conn: &Connection) -> anyhow::Result<()> {
        let poll_id = &self.poll.id().0;
        let new_comment = NewComment {
            id: &self.comment.id().0,
            contest_id: None,
            poll_id: Some(poll_id),
            account_id: self.comment.account_id(),
            choice_name: self.comment.choice_name().map(|c| c.0.as_str()),
            created_at: self.comment.created_at(),
            content: self.comment.comment.as_str(),
        };
        CommentTable::save(conn, new_comment)
    }
}
