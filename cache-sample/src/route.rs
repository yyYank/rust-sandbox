use axum::Router;
use axum::routing::get;
use axum::extract;
use cached::{Cached, UnboundCache};
use cached::proc_macro::cached;
use tokio::sync::MutexGuard;

/*
cached! {
    TWICE_CACHE: SizedCache<String, String> = SizedCache::with_size(5000);
    fn cached_twice(id:String) -> String = {
        println!("{} not cached", id);
        return (id.parse::<i32>().unwrap() * 2).to_string();
    }
}
 */

#[cached (name="TWICE_CACHE")]
async fn cached_twice(id:String) -> String {
    println!("{} not cached", id);
    return format!("{} * 2 = {}", id, id.parse::<i32>().unwrap() * 2);
}

pub fn routes() -> Router{
    Router::new()
        .route("/cache/:id", get(get_cache))
        .route("/clear-cache/:id", get(clear_cache))
}

async fn get_cache(
    extract::Path(id) : extract::Path<String>
) -> String {
    cached_twice(id).await
}

async fn clear_cache(
    extract::Path(id) : extract::Path<String>
) -> String {
    {
        let mut cache : MutexGuard<UnboundCache<String, String>>= TWICE_CACHE.lock().await;
        cache.cache_remove(&id);
    }
    return format!("{} cache clear has done", id);
}


