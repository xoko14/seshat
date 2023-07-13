use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::SqlitePool;

use crate::{
    server::errors::ApiError,
};
use seshat_schemas::{Article, ArticleCreate, Sale, Seller, SellerCreate};

type ApiResult<T> = Result<T, ApiError>;

pub async fn create_seller(
    State(db): State<SqlitePool>,
    Json(seller): Json<SellerCreate>,
) -> ApiResult<()> {
    sqlx::query!(
        "insert into seller (name, stand_number, discount, code, deleted) values (?,?,?,?,0)",
        seller.name,
        seller.stand_number,
        seller.discount,
        seller.code
    )
    .execute(&db)
    .await?;
    Ok(())
}

pub async fn get_sellers(State(db): State<SqlitePool>) -> ApiResult<Json<Vec<Seller>>> {
    let sellers: Vec<Seller> = sqlx::query_as!(
        Seller,
        "select id, name, stand_number, discount, code from seller where deleted = 0"
    )
    .fetch_all(&db)
    .await?;
    Ok(Json(sellers))
}

pub async fn delete_seller(State(db): State<SqlitePool>, Path(id): Path<i64>) -> ApiResult<()> {
    sqlx::query!("update seller set deleted = 1 where id = ?", id)
        .execute(&db)
        .await?;
    Ok(())
}

pub async fn create_article(
    State(db): State<SqlitePool>,
    Json(article): Json<ArticleCreate>,
) -> ApiResult<()> {
    sqlx::query!(
        "insert into article (title, ean, pvp, language, stock) values (?,?,?,?,?)",
        article.title,
        article.ean,
        article.pvp,
        article.language,
        article.stock
    )
    .execute(&db)
    .await?;
    Ok(())
}

pub async fn get_articles(State(db): State<SqlitePool>) -> ApiResult<Json<Vec<Article>>> {
    let articles = sqlx::query_as!(Article, "select * from article")
        .fetch_all(&db)
        .await?;
    Ok(Json(articles))
}

pub async fn get_article(
    State(db): State<SqlitePool>,
    Path(code): Path<String>,
) -> ApiResult<Json<Article>> {
    let article = sqlx::query_as!(Article, "select * from article where ean = ?", code)
        .fetch_optional(&db)
        .await?;
    match article {
        Some(a) => Ok(Json(a)),
        None => Err(ApiError::not_found()),
    }
}

pub async fn new_sale(State(db): State<SqlitePool>, Json(sale): Json<Sale>) -> ApiResult<()> {
    let sale_id = sqlx::query!(
        "insert into sale (seller_id, total) values (?,?)",
        sale.seller_id,
        sale.total,
    )
    .execute(&db)
    .await?
    .last_insert_rowid();

    for line in sale.detail {
        let article_info = match sqlx::query_as!(
            ArticleInfo,
            "select id, pvp from article where ean = ?",
            line.article_ean,
        )
        .fetch_optional(&db)
        .await?
        {
            Some(a) => a,
            None => return Err(ApiError::not_found()),
        };

        let total_line = line.quantity as f64 * article_info.pvp;

        sqlx::query!(
            "insert into sale_detail (sale_id, article_id, quantity, total_detail) values (?,?,?,?)",
            sale_id,
            article_info.id,
            line.quantity,
            total_line
        ).execute(&db)
        .await?;
    }
    Ok(())
}

struct ArticleInfo {
    pub id: i64,
    pub pvp: f64,
}
