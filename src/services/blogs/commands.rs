use sqlx::{postgres::PgQueryResult, Error, Pool, Postgres};

use crate::data_types::structs::Blog;

pub async fn insert_blog(blog: &Blog, pool: &Pool<Postgres>) -> Result<Blog, Error> {
    sqlx::query_as!(
        Blog,
        r#"
            WITH new_row AS (
                INSERT INTO blog
                    (
                        title,
                        slug,
                        content,
                        image_link,
                        thumbnail_link,
                        featured
                    )
                VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6
                )
                RETURNING
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
            )
            SELECT
                new_row.id,
                new_row.title,
                new_row.slug,
                new_row.content,
                new_row.image_link,
                new_row.thumbnail_link,
                new_row.featured,
                new_row.created,
                new_row.edited
            FROM new_row
        "#,
        blog.title,
        blog.slug,
        blog.content,
        blog.image_link,
        blog.thumbnail_link,
        blog.featured
    )
    .fetch_one(pool)
    .await
}

pub async fn update_blog(id: &i32, blog: &Blog, pool: &Pool<Postgres>) -> Result<Blog, Error> {
    sqlx::query_as!(
        Blog,
        r#"
        WITH new_row AS (
            INSERT INTO blog (id, title, slug, content, image_link, thumbnail_link, featured)
            VALUES (
                $1, $2, $3, $4, $5, $6, $7
            )
            ON CONFLICT (id)
            DO UPDATE SET
                id = EXCLUDED.id,
                title = EXCLUDED.title,
                slug = EXCLUDED.slug,
                content = EXCLUDED.content,
                image_link = EXCLUDED.image_link,
                thumbnail_link = EXCLUDED.thumbnail_link,
                featured = EXCLUDED.featured,
                edited = NOW()
            RETURNING
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
        )
        SELECT
            new_row.id,
            new_row.title,
            new_row.slug,
            new_row.content,
            new_row.image_link,
            new_row.thumbnail_link,
            new_row.featured,
            new_row.created,
            new_row.edited
        FROM new_row
    "#,
        id,
        blog.title,
        blog.slug,
        blog.content,
        blog.image_link,
        blog.thumbnail_link,
        blog.featured,
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_blog(id: &i32, pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
    sqlx::query_as!(Blog, "DELETE FROM blog WHERE id = $1;", id)
        .execute(pool)
        .await
}
