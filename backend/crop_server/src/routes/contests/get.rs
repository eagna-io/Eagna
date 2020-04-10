pub fn route(ctx: Context) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contests")
        .and(warp::filters::method::get())
        .and_then(move || {
            let ctx = ctx.clone();
            async move {
                ctx.pg.with_conn(|conn| kj)
            }
        })
}

async fn inner(ctx: Context) -> Result<(StatusCode, ResBody), (StatusCode, &'static str)> {
    ctx.pg.with_conn(|conn| {
    }).await.:q

}
