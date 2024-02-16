use sqlx::{Error, Pool, Postgres};

use crate::data_types::structs::Blog;

pub async fn get_all_featured_blogs(pool: &Pool<Postgres>) -> Result<Vec<Blog>, Error> {
    sqlx::query_as!(
        Blog,
        r#"
            SELECT
                id,
                title,
                slug,
                content,
                image_link,
                thumbnail_link,
                featured,
                (
                    trim(to_char(created, 'DD')) || ' ' ||
                    trim(to_char(created, 'Month')) || ' ' ||
                    trim(to_char(created, 'YYYY HH12:MI AM'))
                ) as created,
                (
                    trim(to_char(edited, 'DD')) || ' ' ||
                    trim(to_char(edited, 'Month')) || ' ' ||
                    trim(to_char(edited, 'YYYY HH12:MI AM'))
                ) as edited
            FROM blog
            WHERE featured = TRUE
            LIMIT 2;
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_blog_by_id(id: &i32, pool: &Pool<Postgres>) -> Result<Blog, Error> {
    sqlx::query_as!(
        Blog,
        r#"
            SELECT
                id,
                title,
                slug,
                content,
                image_link,
                thumbnail_link,
                featured,
                (
                    trim(to_char(created, 'DD')) || ' ' ||
                    trim(to_char(created, 'Month')) || ' ' ||
                    trim(to_char(created, 'YYYY HH12:MI AM'))
                ) as created,
                (
                    trim(to_char(edited, 'DD')) || ' ' ||
                    trim(to_char(edited, 'Month')) || ' ' ||
                    trim(to_char(edited, 'YYYY HH12:MI AM'))
                ) as edited
            FROM blog
            WHERE id = $1
            LIMIT 1;
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_all_blogs(pool: &Pool<Postgres>) -> Result<Vec<Blog>, Error> {
    sqlx::query_as!(
        Blog,
        r#"
            SELECT
                id,
                title,
                slug,
                content,
                image_link,
                thumbnail_link,
                featured,
                (
                    trim(to_char(created, 'DD')) || ' ' ||
                    trim(to_char(created, 'Month')) || ' ' ||
                    trim(to_char(created, 'YYYY HH12:MI AM'))
                ) as created,
                (
                    trim(to_char(edited, 'DD')) || ' ' ||
                    trim(to_char(edited, 'Month')) || ' ' ||
                    trim(to_char(edited, 'YYYY HH12:MI AM'))
                ) as edited
            FROM blog;
        "#,
    )
    .fetch_all(pool)
    .await
}
