use clap::Parser;
use loco_rs::{
    app::Hooks,
    boot::create_context,
    environment::{resolve_from_env, Environment, DEFAULT_ENVIRONMENT},
    hash,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ExprTrait,
    IntoActiveModel, QueryFilter,
};
use stat_api_rs::app::App;
use stat_api_rs::models::{_entities::user_roles::Column, user_roles, users};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct AdmCreation {
    #[arg(short, long, help = "Логин (email или username) администратора")]
    username: String,

    #[arg(short, long, help = "Пароль в открытом виде")]
    password: String,

    /// Specify the environment
    #[arg(short, long, global = true, help = &format!("Specify the environment [default: {}]", DEFAULT_ENVIRONMENT))]
    environment: Option<String>,
}

async fn assing_admin_role(db: &DatabaseConnection, user_id: i64) -> loco_rs::Result<()> {
    if user_roles::Entity::find()
        .filter(Column::UserId.eq(user_id).and(Column::RoleId.eq(1)))
        .one(db)
        .await?
        .is_some()
    {
        println!("Роль уже назначена");
        return Ok(());
    }
    user_roles::ActiveModel::builder()
        .set_user_id(user_id)
        .set_role_id(1)
        .insert(db)
        .await?;
    println!("Админская роль успешно привязана");
    Ok(())
}

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let cli = AdmCreation::parse();
    let env: Environment = cli.environment.unwrap_or_else(resolve_from_env).into();
    let config = App::load_config(&env).await?;
    let ctx = create_context::<App>(&env, config).await?;

    let user = users::Model::find_by_login(&ctx.db, &cli.username).await;

    if let Ok(user) = user {
        let user_id = user.id;
        let mut active_model = user.into_active_model();
        active_model.password = ActiveValue::Set(hash::hash_password(&cli.password)?);
        active_model.update(&ctx.db).await?;
        println!("Пароль пользователя \"{}\" изменен", cli.username);
        assing_admin_role(&ctx.db, user_id).await?;
        return Ok(());
    }

    let result = users::ActiveModel::builder()
        .set_email("admin@mpunav.ru".to_string())
        .set_login(cli.username.clone())
        .set_name("admin".to_string())
        .set_password(hash::hash_password(&cli.password)?)
        .set_is_active(true)
        .insert(&ctx.db)
        .await;

    match result {
        Ok(user) => {
            println!("Пользователь успешно создан. Его id {}", user.id);
            assing_admin_role(&ctx.db, user.id).await?;
        }
        Err(e) => {
            println!("Ошибка при создании пользователя. Подробнее: {e}");
        }
    }

    Ok(())
}
