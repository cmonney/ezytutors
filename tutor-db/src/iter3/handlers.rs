use super::db_access::*;
use super::models::{Course, NewCourse};
use super::state::AppState;
use std::convert::TryFrom;

use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
    _app_state: web::Data<AppState>,
    _params: web::Path<i32>,
) -> HttpResponse {
    let tutor_id = _params.into_inner();
    let courses = get_courses_for_tutor_db(&_app_state.db, tutor_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_details(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = _params.into_inner();
    let course = get_course_details_db(&_app_state.db, tutor_id, course_id).await;
    HttpResponse::Ok().json(course)
}

pub async fn post_new_course(
    _new_course: web::Json<NewCourse>,
    _app_state: web::Data<AppState>,
) -> HttpResponse {
    let course = post_new_course_db(&_app_state.db, _new_course.into()).await;
    HttpResponse::Ok().json(course)
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;
    use sqlx::Pool;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool
        });

        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool
        });
        let new_course_msg = NewCourse {
            tutor_id: 1,
            course_name: "This is the next course".into(),
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(course_param, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}