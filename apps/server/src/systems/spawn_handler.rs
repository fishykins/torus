/// Spawns entities on behalf of clients.
pub fn spawn_handler(
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut spawner: ResMut<Spawner>,
    mut mats: ResMut<Assets<PhysicsMaterial>>,
) {
    while let Some(id) = spawner.pop() {
        let material = mats.add(PhysicsMaterial::new(0.8));
        let position = Point::new(0.0f32, 2.0f32);

        let player_bundle = PlayerBundle::new(format!("Player {}", id), position, material.clone());
        let player_uuid = spawner.get_uuid();

        let player_entity = commands
            .spawn_bundle(player_bundle)
            .insert(Drag::default())
            .insert(Agent::new(player_uuid).with_owner(id))
            .id();
        lobby.assign(id, player_entity);
    }
}
