use std::{collections::HashMap, error::Error, path::Path};

use bollard::{
    container::{
        Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
        WaitContainerOptions,
    },
    image::CreateImageOptions,
    secret::HostConfig,
    Docker,
};
use dotenvy::dotenv;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let docker = Docker::connect_with_local_defaults()?;

    let working_dir = std::env::var("QMK_FIRMWARE_PATH").expect("Missing firmware working dir");

    let container_name = "qmk_build";
    let options = CreateContainerOptions {
        name: container_name,
        platform: None,
    };

    let config = Config {
        image: Some("ghcr.io/qmk/qmk_cli"),
        working_dir: Some("/qmk_firmware"),
        env: Some(vec!["ALT_GET_KEYBOARDS=true"]),
        host_config: Some(HostConfig {
            binds: Some(vec![format!("{}:/qmk_firmware", working_dir)]),
            auto_remove: Some(true),
            ..Default::default()
        }),
        cmd: Some(vec!["make", "moonlander:custom"]),
        ..Config::default()
    };

    // docker
    //     .remove_container(container_name, None::<RemoveContainerOptions>)
    //     .await?;

    docker.create_container(Some(options), config).await?;

    // Start the container
    docker
        .start_container(container_name, None::<StartContainerOptions<String>>)
        .await?;

    let a = docker
        .wait_container(container_name, None::<WaitContainerOptions<String>>)
        .collect::<Vec<_>>()
        .await;
    println!("Hello, world!");

    Ok(())
}
