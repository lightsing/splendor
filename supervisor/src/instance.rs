use bollard::container::UpdateContainerOptions;
use bollard::{
    container::{CreateContainerOptions, RemoveContainerOptions},
    models::*,
    network::CreateNetworkOptions,
    volume::{CreateVolumeOptions, RemoveVolumeOptions},
    Docker,
};
use futures_util::{future, TryFutureExt};
use std::future::Future;
use std::{env, mem};
use uuid::Uuid;

#[derive(Debug)]
pub struct GameInstance {
    pub id: Uuid,
    docker: Docker,
    networks: Vec<String>,
    volumes: Vec<String>,
    server: Option<String>,
    players: Vec<String>,
}

impl GameInstance {
    pub async fn new<P: AsRef<str>>(
        docker: Docker,
        server_img: &str,
        player_imgs: &[P],
        seed: Option<u64>,
        step_timeout: u64,
    ) -> Result<Self, bollard::errors::Error> {
        let id = Uuid::new_v4();
        let n_players = player_imgs.len();
        assert!(n_players == 3 || n_players == 4);

        let networks = future::try_join_all((0..n_players).map(|idx| {
            docker
                .create_network(CreateNetworkOptions {
                    name: format!("game-{id}-player{idx}"),
                    check_duplicate: true,
                    internal: true,
                    ..Default::default()
                })
                .map_ok(|resp| resp.id.unwrap())
        }))
        .await?;

        let volumes = future::try_join_all((0..n_players).map(|idx| {
            docker
                .create_volume(CreateVolumeOptions {
                    name: format!("game-{id}-player{idx}"),
                    ..Default::default()
                })
                .map_ok(|resp| resp.name)
        }))
        .await?;

        let mut server_env = vec![
            "RUST_LOG=info".to_string(),
            format!("GAME_ID={id}"),
            format!("N_PLAYERS={n_players}"),
            format!("STEP_TIMEOUT={step_timeout}"),
            "SECRETS_PATH=/app/secrets".to_string(),
            "SERVER_ADDR=0.0.0.0:8080".to_string(),
            "SUPERVISOR_SOCKET=/var/run/splendor/supervisor.sock".to_string(),
        ];
        if let Some(seed) = seed {
            server_env.push(format!("SEED={}", seed));
        }

        let mut mounts = volumes
            .iter()
            .enumerate()
            .map(|(idx, name)| Mount {
                target: Some(format!("/app/secrets/player{idx}", idx = idx)),
                source: Some(name.clone()),
                typ: Some(MountTypeEnum::VOLUME),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        mounts.push(Mount {
            target: Some("/var/run/splendor".to_string()),
            source: Some(env::var("SHARED_VOLUME").expect("SUPERVISOR_SOCKET must be set")),
            typ: Some(MountTypeEnum::VOLUME),
            ..Default::default()
        });

        // create server container
        let server = docker
            .create_container(
                Some(CreateContainerOptions {
                    name: format!("game-{id}-server"),
                    ..Default::default()
                }),
                bollard::container::Config {
                    image: Some(server_img.to_string()),
                    hostname: Some("server".to_string()),
                    host_config: Some(HostConfig {
                        mounts: Some(mounts),
                        ..Default::default()
                    }),
                    env: Some(server_env),
                    networking_config: Some(bollard::container::NetworkingConfig {
                        endpoints_config: networks
                            .iter()
                            .enumerate()
                            .map(|(idx, net_id)| {
                                (
                                    format!("game-{id}-player{idx}-net"),
                                    EndpointSettings {
                                        aliases: Some(vec!["server".to_string()]),
                                        network_id: Some(net_id.to_string()),
                                        ..Default::default()
                                    },
                                )
                            })
                            .collect(),
                    }),
                    ..Default::default()
                },
            )
            .await?
            .id;
        debug!("server container created: {server}");

        // create player containers
        let players = future::try_join_all(
            player_imgs
                .iter()
                .zip(volumes.iter().zip(networks.iter()))
                .enumerate()
                .map(|(idx, (img, (volume_id, net_id)))| {
                    docker
                        .create_container(
                            Some(CreateContainerOptions {
                                name: format!("game-{id}-player{idx}"),
                                ..Default::default()
                            }),
                            bollard::container::Config {
                                image: Some(img.as_ref().to_string()),
                                hostname: Some(format!("player{idx}")),
                                host_config: Some(HostConfig {
                                    mounts: Some(vec![Mount {
                                        target: Some("/app/secrets".to_string()),
                                        source: Some(volume_id.to_string()),
                                        typ: Some(MountTypeEnum::VOLUME),
                                        read_only: Some(true),
                                        volume_options: Some(MountVolumeOptions {
                                            no_copy: Some(true),
                                            ..Default::default()
                                        }),
                                        ..Default::default()
                                    }]),
                                    ..Default::default()
                                }),
                                env: Some(vec![
                                    "RPC_URL=ws://server:8080".to_string(),
                                    "CLIENT_SECRET=/app/secrets/secret".to_string(),
                                    format!("STEP_TIMEOUT={}", step_timeout),
                                ]),
                                networking_config: Some(bollard::container::NetworkingConfig {
                                    endpoints_config: [(
                                        format!("game-{id}-player{idx}-net", id = id),
                                        EndpointSettings {
                                            network_id: Some(net_id.to_string()),
                                            ..Default::default()
                                        },
                                    )]
                                    .into_iter()
                                    .collect(),
                                }),
                                ..Default::default()
                            },
                        )
                        .map_ok(|resp| resp.id)
                }),
        )
        .await?;
        debug!("player containers created: {players:?}");

        Ok(GameInstance {
            id,
            docker,
            networks,
            volumes,
            server: Some(server),
            players,
        })
    }

    pub async fn start(&self) -> Result<(), bollard::errors::Error> {
        self.docker
            .start_container::<String>(self.server.as_ref().unwrap(), None)
            .await?;
        future::try_join_all(
            self.players
                .iter()
                .map(|player| self.docker.start_container::<String>(player, None)),
        )
        .await?;
        Ok(())
    }

    pub async fn prepare_player_change(
        &self,
        next_player: usize,
    ) -> Result<(), bollard::errors::Error> {
        // "freeze" other players
        future::try_join_all(
            self.players
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != next_player)
                .map(|(_, player)| {
                    self.docker.update_container(
                        player,
                        UpdateContainerOptions::<String> {
                            cpu_period: Some(100000),
                            cpu_quota: Some(1000), // allow to use 1%
                            ..Default::default()
                        },
                    )
                }),
        )
        .await?;
        // unfreeze next player
        self.docker
            .update_container(
                &self.players[next_player],
                UpdateContainerOptions::<String> {
                    cpu_period: Some(100000),
                    cpu_quota: Some(950000), // allow to use 95%, reserve 5% for dispatcher
                    ..Default::default()
                },
            )
            .await
    }

    pub async fn cleanup(self) -> Result<(), bollard::errors::Error> {
        future::try_join_all(self.players.iter().map(|player| {
            debug!("removing player container {player}");
            self.docker.remove_container(
                player,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
        }))
        .await?;
        if let Some(server) = self.server {
            debug!("removing server container {server}");
            self.docker
                .remove_container(
                    &server,
                    Some(RemoveContainerOptions {
                        force: true,
                        ..Default::default()
                    }),
                )
                .await?;
        }
        future::try_join_all(self.networks.iter().map(|network| {
            debug!("removing network {network}");
            self.docker.remove_network(network)
        }))
        .await?;
        future::try_join_all(self.volumes.iter().map(|network| {
            debug!("removing volume {network}");
            self.docker
                .remove_volume(network, Some(RemoveVolumeOptions { force: true }))
        }))
        .await?;
        Ok(())
    }
}
