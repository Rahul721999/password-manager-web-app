use sqlx::migrate::Migrator;
use sqlx::{query_scalar, Pool, Postgres};
use std::fs;
use std::path::Path;
use tracing::{error, info, warn};

pub async fn apply_migration(db: &Pool<Postgres>) {
    let migrator = match Migrator::new(Path::new("./migrations")).await {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to initialize migrator; Err: {}", err);
            return;
        }
    };

    // Get the latest applied migration from the database
    let latest_db_migration: Option<i64> =
        query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
            .fetch_one(db)
            .await
            .ok();

    // Get the latest migration file version from the `./migrations/` folder
    let latest_file_migration = fs::read_dir("./migrations")
        .unwrap()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()?
                    .to_str()?
                    .split('_')
                    .next()?
                    .parse::<i64>()
                    .ok()
            })
        })
        .max(); // Get the highest version number

    if let Err(err) = migrator.run(db).await {
        error!("Failed to apply DB migration; Err: {}", err);
    } else {
        match (latest_db_migration, latest_file_migration) {
            (Some(db_version), Some(file_version)) => {
                if db_version == file_version {
                    info!("Latest migration (v{}) is already applied.", db_version);
                } else if db_version < file_version {
                    info!(
                        "New migration (v{}) was applied successfully.",
                        file_version
                    );
                } else {
                    warn!(
                        "Warning: DB version (v{}) is ahead of migration files (latest v{}).",
                        db_version, file_version
                    );
                }
            },
            _ => {
                info!("Migration applied successfully, but version check failed.");
            }
        }
    }
}
