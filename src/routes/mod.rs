use actix_web::dev::HttpServiceFactory;

mod blog;
mod health;

pub fn blog() -> impl HttpServiceFactory {
    (
        blog::create_blog_route,
        blog::get_featured_blogs_route,
        blog::get_all_blogs_route,
        blog::get_blog_by_id_route,
        blog::update_blog_route,
        blog::delete_blog_route,
    )
}
pub fn health() -> impl HttpServiceFactory {
    (health::health,)
}
