use serde::{Deserialize, Serialize};
use factorio_mod_api_codegen::Visitable;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ActivateEquipmentCapsuleAction {
    pub equipment: EquipmentID,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ActivateImpactTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_category: Option<String>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ActivatePasteTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct ActiveTriggerID(pub String);
impl std::fmt::Display for ActiveTriggerID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for ActiveTriggerID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ActivityBarStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_size_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ActivityMatchingModifiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AdvancedMapGenSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asteroids: Option<MapGenPresetAsteroidSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty_settings: Option<MapGenPresetDifficultySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_evolution: Option<MapGenPresetEnemyEvolutionSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_expansion: Option<MapGenPresetEnemyExpansionSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pollution: Option<MapGenPresetPollutionSettings>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AdvancedVolumeControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attenuation: Option<Fade>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darkness_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fades: Option<Fades>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AggregationSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_already_playing: Option<bool>,
    pub max_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_threshold: Option<f64>,
    pub remove: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_reduction_rate: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AgriculturalCraneProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_arm_extent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_grappler_extent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_angle: Option<f64>,
    pub origin: Vector3D,
    pub parts: Vec<CranePart>,
    pub shadow_direction: Vector3D,
    pub speed: AgriculturalCraneSpeed,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telescope_default_extention: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AgriculturalCraneSpeed {
    pub arm: AgriculturalCraneSpeedArm,
    pub grappler: AgriculturalCraneSpeedGrappler,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AgriculturalCraneSpeedArm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_rate: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AgriculturalCraneSpeedGrappler {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_transpolar_movement: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_turn_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_turn_rate: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct AirbornePollutantID(pub String);
impl std::fmt::Display for AirbornePollutantID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for AirbornePollutantID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum Alignment {
    #[serde(rename = "top-left")]
    LiteralTopLeft,
    #[serde(rename = "middle-left")]
    LiteralMiddleLeft,
    #[serde(rename = "left")]
    LiteralLeft,
    #[serde(rename = "bottom-left")]
    LiteralBottomLeft,
    #[serde(rename = "top-center")]
    LiteralTopCenter,
    #[serde(rename = "middle-center")]
    LiteralMiddleCenter,
    #[serde(rename = "center")]
    LiteralCenter,
    #[serde(rename = "bottom-center")]
    LiteralBottomCenter,
    #[serde(rename = "top-right")]
    LiteralTopRight,
    #[serde(rename = "middle-right")]
    LiteralMiddleRight,
    #[serde(rename = "right")]
    LiteralRight,
    #[serde(rename = "bottom-right")]
    LiteralBottomRight,
}
impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Alignment::LiteralTopLeft => "top-left",
                Alignment::LiteralMiddleLeft => "middle-left",
                Alignment::LiteralLeft => "left",
                Alignment::LiteralBottomLeft => "bottom-left",
                Alignment::LiteralTopCenter => "top-center",
                Alignment::LiteralMiddleCenter => "middle-center",
                Alignment::LiteralCenter => "center",
                Alignment::LiteralBottomCenter => "bottom-center",
                Alignment::LiteralTopRight => "top-right",
                Alignment::LiteralMiddleRight => "middle-right",
                Alignment::LiteralRight => "right",
                Alignment::LiteralBottomRight => "bottom-right",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AlternativeBuildTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum AmbientSoundType {
    #[serde(rename = "menu-track")]
    LiteralMenuTrack,
    #[serde(rename = "main-track")]
    LiteralMainTrack,
    #[serde(rename = "hero-track")]
    LiteralHeroTrack,
    #[serde(rename = "interlude")]
    LiteralInterlude,
    #[serde(rename = "script-track")]
    LiteralScriptTrack,
}
impl std::fmt::Display for AmbientSoundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                AmbientSoundType::LiteralMenuTrack => "menu-track",
                AmbientSoundType::LiteralMainTrack => "main-track",
                AmbientSoundType::LiteralHeroTrack => "hero-track",
                AmbientSoundType::LiteralInterlude => "interlude",
                AmbientSoundType::LiteralScriptTrack => "script-track",
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct AmmoCategoryID(pub String);
impl std::fmt::Display for AmmoCategoryID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for AmmoCategoryID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AmmoDamageModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    pub ammo_category: AmmoCategoryID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub modifier: f64,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum AmmoSourceType {
    #[serde(rename = "default")]
    LiteralDefault,
    #[serde(rename = "player")]
    LiteralPlayer,
    #[serde(rename = "turret")]
    LiteralTurret,
    #[serde(rename = "vehicle")]
    LiteralVehicle,
}
impl std::fmt::Display for AmmoSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                AmmoSourceType::LiteralDefault => "default",
                AmmoSourceType::LiteralPlayer => "player",
                AmmoSourceType::LiteralTurret => "turret",
                AmmoSourceType::LiteralVehicle => "vehicle",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AmmoType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamp_position: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_consumption: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_clamp_to_max_range: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<AmmoSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_filter: Option<Vec<EntityID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AndTipTrigger {
    pub triggers: Vec<TipTrigger>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AnimatedVector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_shift: Option<DirectionShift>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    pub rotations: Vec<VectorRotation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Animation {
    #[serde(flatten)]
    pub parent: AnimationParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<FileName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Animation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_per_file: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slice: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripes: Option<Vec<Stripe>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum Animation4Way {
    Struct0,
    Animation(Box<Animation>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AnimationElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_draw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_tint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_draw_order: Option<i8>,
}
pub type AnimationFrameSequence = Vec<u16>;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AnimationParameters {
    #[serde(flatten)]
    pub parent: SpriteParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reducing_frames: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_x: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_y: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_sequence: Option<AnimationFrameSequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_sdf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_advance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mipmap_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_mode: Option<AnimationRunMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum AnimationRunMode {
    #[serde(rename = "forward")]
    LiteralForward,
    #[serde(rename = "backward")]
    LiteralBackward,
    #[serde(rename = "forward-then-backward")]
    LiteralForwardThenBackward,
}
impl std::fmt::Display for AnimationRunMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                AnimationRunMode::LiteralForward => "forward",
                AnimationRunMode::LiteralBackward => "backward",
                AnimationRunMode::LiteralForwardThenBackward => "forward-then-backward",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AnimationSheet {
    #[serde(flatten)]
    pub parent: AnimationParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<FileName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_per_file: Option<u32>,
    pub variation_count: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum AnimationVariations {
    Struct0,
    Animation(Box<Animation>),
    Array2(Vec<Animation>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum AnyBasic {
    String(Box<String>),
    Boolean(Box<bool>),
    Number(Box<f64>),
    Table(Box<serde_json::Value>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum AnyPrototype {
    #[serde(rename = "accumulator")]
    AccumulatorPrototype(AccumulatorPrototype),
    #[serde(rename = "achievement")]
    AchievementPrototype(AchievementPrototype),
    #[serde(rename = "active-defense-equipment")]
    ActiveDefenseEquipmentPrototype(ActiveDefenseEquipmentPrototype),
    #[serde(rename = "agricultural-tower")]
    AgriculturalTowerPrototype(AgriculturalTowerPrototype),
    #[serde(rename = "airborne-pollutant")]
    AirbornePollutantPrototype(AirbornePollutantPrototype),
    #[serde(rename = "ambient-sound")]
    AmbientSound(AmbientSound),
    #[serde(rename = "ammo-category")]
    AmmoCategory(AmmoCategory),
    #[serde(rename = "ammo")]
    AmmoItemPrototype(AmmoItemPrototype),
    #[serde(rename = "ammo-turret")]
    AmmoTurretPrototype(AmmoTurretPrototype),
    #[serde(rename = "animation")]
    AnimationPrototype(AnimationPrototype),
    #[serde(rename = "arithmetic-combinator")]
    ArithmeticCombinatorPrototype(ArithmeticCombinatorPrototype),
    #[serde(rename = "armor")]
    ArmorPrototype(ArmorPrototype),
    #[serde(rename = "arrow")]
    ArrowPrototype(ArrowPrototype),
    #[serde(rename = "artillery-flare")]
    ArtilleryFlarePrototype(ArtilleryFlarePrototype),
    #[serde(rename = "artillery-projectile")]
    ArtilleryProjectilePrototype(ArtilleryProjectilePrototype),
    #[serde(rename = "artillery-turret")]
    ArtilleryTurretPrototype(ArtilleryTurretPrototype),
    #[serde(rename = "artillery-wagon")]
    ArtilleryWagonPrototype(ArtilleryWagonPrototype),
    #[serde(rename = "assembling-machine")]
    AssemblingMachinePrototype(AssemblingMachinePrototype),
    #[serde(rename = "asteroid-chunk")]
    AsteroidChunkPrototype(AsteroidChunkPrototype),
    #[serde(rename = "asteroid-collector")]
    AsteroidCollectorPrototype(AsteroidCollectorPrototype),
    #[serde(rename = "asteroid")]
    AsteroidPrototype(AsteroidPrototype),
    #[serde(rename = "autoplace-control")]
    AutoplaceControl(AutoplaceControl),
    #[serde(rename = "battery-equipment")]
    BatteryEquipmentPrototype(BatteryEquipmentPrototype),
    #[serde(rename = "beacon")]
    BeaconPrototype(BeaconPrototype),
    #[serde(rename = "beam")]
    BeamPrototype(BeamPrototype),
    #[serde(rename = "belt-immunity-equipment")]
    BeltImmunityEquipmentPrototype(BeltImmunityEquipmentPrototype),
    #[serde(rename = "blueprint-book")]
    BlueprintBookPrototype(BlueprintBookPrototype),
    #[serde(rename = "blueprint")]
    BlueprintItemPrototype(BlueprintItemPrototype),
    #[serde(rename = "boiler")]
    BoilerPrototype(BoilerPrototype),
    #[serde(rename = "build-entity-achievement")]
    BuildEntityAchievementPrototype(BuildEntityAchievementPrototype),
    #[serde(rename = "burner-generator")]
    BurnerGeneratorPrototype(BurnerGeneratorPrototype),
    #[serde(rename = "burner-usage")]
    BurnerUsagePrototype(BurnerUsagePrototype),
    #[serde(rename = "capsule")]
    CapsulePrototype(CapsulePrototype),
    #[serde(rename = "capture-robot")]
    CaptureRobotPrototype(CaptureRobotPrototype),
    #[serde(rename = "car")]
    CarPrototype(CarPrototype),
    #[serde(rename = "cargo-bay")]
    CargoBayPrototype(CargoBayPrototype),
    #[serde(rename = "cargo-landing-pad")]
    CargoLandingPadPrototype(CargoLandingPadPrototype),
    #[serde(rename = "cargo-pod")]
    CargoPodPrototype(CargoPodPrototype),
    #[serde(rename = "cargo-wagon")]
    CargoWagonPrototype(CargoWagonPrototype),
    #[serde(rename = "chain-active-trigger")]
    ChainActiveTriggerPrototype(ChainActiveTriggerPrototype),
    #[serde(rename = "change-surface-achievement")]
    ChangedSurfaceAchievementPrototype(ChangedSurfaceAchievementPrototype),
    #[serde(rename = "character-corpse")]
    CharacterCorpsePrototype(CharacterCorpsePrototype),
    #[serde(rename = "character")]
    CharacterPrototype(CharacterPrototype),
    #[serde(rename = "cliff")]
    CliffPrototype(CliffPrototype),
    #[serde(rename = "collision-layer")]
    CollisionLayerPrototype(CollisionLayerPrototype),
    #[serde(rename = "combat-robot-count-achievement")]
    CombatRobotCountAchievementPrototype(CombatRobotCountAchievementPrototype),
    #[serde(rename = "combat-robot")]
    CombatRobotPrototype(CombatRobotPrototype),
    #[serde(rename = "complete-objective-achievement")]
    CompleteObjectiveAchievementPrototype(CompleteObjectiveAchievementPrototype),
    #[serde(rename = "constant-combinator")]
    ConstantCombinatorPrototype(ConstantCombinatorPrototype),
    #[serde(rename = "construct-with-robots-achievement")]
    ConstructWithRobotsAchievementPrototype(ConstructWithRobotsAchievementPrototype),
    #[serde(rename = "construction-robot")]
    ConstructionRobotPrototype(ConstructionRobotPrototype),
    #[serde(rename = "container")]
    ContainerPrototype(ContainerPrototype),
    #[serde(rename = "copy-paste-tool")]
    CopyPasteToolPrototype(CopyPasteToolPrototype),
    #[serde(rename = "corpse")]
    CorpsePrototype(CorpsePrototype),
    #[serde(rename = "create-platform-achievement")]
    CreatePlatformAchievementPrototype(CreatePlatformAchievementPrototype),
    #[serde(rename = "curved-rail-a")]
    CurvedRailAPrototype(CurvedRailAPrototype),
    #[serde(rename = "curved-rail-b")]
    CurvedRailBPrototype(CurvedRailBPrototype),
    #[serde(rename = "custom-event")]
    CustomEventPrototype(CustomEventPrototype),
    #[serde(rename = "custom-input")]
    CustomInputPrototype(CustomInputPrototype),
    #[serde(rename = "damage-type")]
    DamageType(DamageType),
    #[serde(rename = "decider-combinator")]
    DeciderCombinatorPrototype(DeciderCombinatorPrototype),
    #[serde(rename = "deconstruct-with-robots-achievement")]
    DeconstructWithRobotsAchievementPrototype(DeconstructWithRobotsAchievementPrototype),
    #[serde(rename = "deconstructible-tile-proxy")]
    DeconstructibleTileProxyPrototype(DeconstructibleTileProxyPrototype),
    #[serde(rename = "deconstruction-item")]
    DeconstructionItemPrototype(DeconstructionItemPrototype),
    #[serde(rename = "optimized-decorative")]
    DecorativePrototype(DecorativePrototype),
    #[serde(rename = "delayed-active-trigger")]
    DelayedActiveTriggerPrototype(DelayedActiveTriggerPrototype),
    #[serde(rename = "deliver-by-robots-achievement")]
    DeliverByRobotsAchievementPrototype(DeliverByRobotsAchievementPrototype),
    #[serde(rename = "deliver-category")]
    DeliverCategory(DeliverCategory),
    #[serde(rename = "deliver-impact-combination")]
    DeliverImpactCombination(DeliverImpactCombination),
    #[serde(rename = "deplete-resource-achievement")]
    DepleteResourceAchievementPrototype(DepleteResourceAchievementPrototype),
    #[serde(rename = "destroy-cliff-achievement")]
    DestroyCliffAchievementPrototype(DestroyCliffAchievementPrototype),
    #[serde(rename = "display-panel")]
    DisplayPanelPrototype(DisplayPanelPrototype),
    #[serde(rename = "dont-build-entity-achievement")]
    DontBuildEntityAchievementPrototype(DontBuildEntityAchievementPrototype),
    #[serde(rename = "dont-craft-manually-achievement")]
    DontCraftManuallyAchievementPrototype(DontCraftManuallyAchievementPrototype),
    #[serde(rename = "dont-kill-manually-achievement")]
    DontKillManuallyAchievementPrototype(DontKillManuallyAchievementPrototype),
    #[serde(rename = "dont-research-before-researching-achievement")]
    DontResearchBeforeResearchingAchievementPrototype(
        DontResearchBeforeResearchingAchievementPrototype,
    ),
    #[serde(rename = "dont-use-entity-in-energy-production-achievement")]
    DontUseEntityInEnergyProductionAchievementPrototype(
        DontUseEntityInEnergyProductionAchievementPrototype,
    ),
    #[serde(rename = "editor-controller")]
    EditorControllerPrototype(EditorControllerPrototype),
    #[serde(rename = "electric-energy-interface-equipment")]
    ElectricEnergyInterfaceEquipmentPrototype(ElectricEnergyInterfaceEquipmentPrototype),
    #[serde(rename = "electric-energy-interface")]
    ElectricEnergyInterfacePrototype(ElectricEnergyInterfacePrototype),
    #[serde(rename = "electric-pole")]
    ElectricPolePrototype(ElectricPolePrototype),
    #[serde(rename = "electric-turret")]
    ElectricTurretPrototype(ElectricTurretPrototype),
    #[serde(rename = "elevated-curved-rail-a")]
    ElevatedCurvedRailAPrototype(ElevatedCurvedRailAPrototype),
    #[serde(rename = "elevated-curved-rail-b")]
    ElevatedCurvedRailBPrototype(ElevatedCurvedRailBPrototype),
    #[serde(rename = "elevated-half-diagonal-rail")]
    ElevatedHalfDiagonalRailPrototype(ElevatedHalfDiagonalRailPrototype),
    #[serde(rename = "elevated-straight-rail")]
    ElevatedStraightRailPrototype(ElevatedStraightRailPrototype),
    #[serde(rename = "unit-spawner")]
    EnemySpawnerPrototype(EnemySpawnerPrototype),
    #[serde(rename = "energy-shield-equipment")]
    EnergyShieldEquipmentPrototype(EnergyShieldEquipmentPrototype),
    #[serde(rename = "entity-ghost")]
    EntityGhostPrototype(EntityGhostPrototype),
    #[serde(rename = "equip-armor-achievement")]
    EquipArmorAchievementPrototype(EquipArmorAchievementPrototype),
    #[serde(rename = "equipment-category")]
    EquipmentCategory(EquipmentCategory),
    #[serde(rename = "equipment-ghost")]
    EquipmentGhostPrototype(EquipmentGhostPrototype),
    #[serde(rename = "equipment-grid")]
    EquipmentGridPrototype(EquipmentGridPrototype),
    #[serde(rename = "explosion")]
    ExplosionPrototype(ExplosionPrototype),
    #[serde(rename = "fire")]
    FireFlamePrototype(FireFlamePrototype),
    #[serde(rename = "fish")]
    FishPrototype(FishPrototype),
    #[serde(rename = "fluid")]
    FluidPrototype(FluidPrototype),
    #[serde(rename = "stream")]
    FluidStreamPrototype(FluidStreamPrototype),
    #[serde(rename = "fluid-turret")]
    FluidTurretPrototype(FluidTurretPrototype),
    #[serde(rename = "fluid-wagon")]
    FluidWagonPrototype(FluidWagonPrototype),
    #[serde(rename = "font")]
    FontPrototype(FontPrototype),
    #[serde(rename = "fuel-category")]
    FuelCategory(FuelCategory),
    #[serde(rename = "furnace")]
    FurnacePrototype(FurnacePrototype),
    #[serde(rename = "fusion-generator")]
    FusionGeneratorPrototype(FusionGeneratorPrototype),
    #[serde(rename = "fusion-reactor")]
    FusionReactorPrototype(FusionReactorPrototype),
    #[serde(rename = "gate")]
    GatePrototype(GatePrototype),
    #[serde(rename = "generator-equipment")]
    GeneratorEquipmentPrototype(GeneratorEquipmentPrototype),
    #[serde(rename = "generator")]
    GeneratorPrototype(GeneratorPrototype),
    #[serde(rename = "god-controller")]
    GodControllerPrototype(GodControllerPrototype),
    #[serde(rename = "group-attack-achievement")]
    GroupAttackAchievementPrototype(GroupAttackAchievementPrototype),
    #[serde(rename = "gui-style")]
    GuiStyle(GuiStyle),
    #[serde(rename = "gun")]
    GunPrototype(GunPrototype),
    #[serde(rename = "half-diagonal-rail")]
    HalfDiagonalRailPrototype(HalfDiagonalRailPrototype),
    #[serde(rename = "heat-interface")]
    HeatInterfacePrototype(HeatInterfacePrototype),
    #[serde(rename = "heat-pipe")]
    HeatPipePrototype(HeatPipePrototype),
    #[serde(rename = "highlight-box")]
    HighlightBoxEntityPrototype(HighlightBoxEntityPrototype),
    #[serde(rename = "impact-category")]
    ImpactCategory(ImpactCategory),
    #[serde(rename = "infinity-cargo-wagon")]
    InfinityCargoWagonPrototype(InfinityCargoWagonPrototype),
    #[serde(rename = "infinity-container")]
    InfinityContainerPrototype(InfinityContainerPrototype),
    #[serde(rename = "infinity-pipe")]
    InfinityPipePrototype(InfinityPipePrototype),
    #[serde(rename = "inserter")]
    InserterPrototype(InserterPrototype),
    #[serde(rename = "inventory-bonus-equipment")]
    InventoryBonusEquipmentPrototype(InventoryBonusEquipmentPrototype),
    #[serde(rename = "item-entity")]
    ItemEntityPrototype(ItemEntityPrototype),
    #[serde(rename = "item-group")]
    ItemGroup(ItemGroup),
    #[serde(rename = "item")]
    ItemPrototype(ItemPrototype),
    #[serde(rename = "item-request-proxy")]
    ItemRequestProxyPrototype(ItemRequestProxyPrototype),
    #[serde(rename = "item-subgroup")]
    ItemSubGroup(ItemSubGroup),
    #[serde(rename = "item-with-entity-data")]
    ItemWithEntityDataPrototype(ItemWithEntityDataPrototype),
    #[serde(rename = "item-with-inventory")]
    ItemWithInventoryPrototype(ItemWithInventoryPrototype),
    #[serde(rename = "item-with-label")]
    ItemWithLabelPrototype(ItemWithLabelPrototype),
    #[serde(rename = "item-with-tags")]
    ItemWithTagsPrototype(ItemWithTagsPrototype),
    #[serde(rename = "kill-achievement")]
    KillAchievementPrototype(KillAchievementPrototype),
    #[serde(rename = "lab")]
    LabPrototype(LabPrototype),
    #[serde(rename = "lamp")]
    LampPrototype(LampPrototype),
    #[serde(rename = "land-mine")]
    LandMinePrototype(LandMinePrototype),
    #[serde(rename = "lane-splitter")]
    LaneSplitterPrototype(LaneSplitterPrototype),
    #[serde(rename = "legacy-curved-rail")]
    LegacyCurvedRailPrototype(LegacyCurvedRailPrototype),
    #[serde(rename = "legacy-straight-rail")]
    LegacyStraightRailPrototype(LegacyStraightRailPrototype),
    #[serde(rename = "lightning-attractor")]
    LightningAttractorPrototype(LightningAttractorPrototype),
    #[serde(rename = "lightning")]
    LightningPrototype(LightningPrototype),
    #[serde(rename = "linked-belt")]
    LinkedBeltPrototype(LinkedBeltPrototype),
    #[serde(rename = "linked-container")]
    LinkedContainerPrototype(LinkedContainerPrototype),
    #[serde(rename = "loader-1x1")]
    Loader1x1Prototype(Loader1x1Prototype),
    #[serde(rename = "loader")]
    Loader1x2Prototype(Loader1x2Prototype),
    #[serde(rename = "locomotive")]
    LocomotivePrototype(LocomotivePrototype),
    #[serde(rename = "logistic-container")]
    LogisticContainerPrototype(LogisticContainerPrototype),
    #[serde(rename = "logistic-robot")]
    LogisticRobotPrototype(LogisticRobotPrototype),
    #[serde(rename = "map-gen-presets")]
    MapGenPresets(MapGenPresets),
    #[serde(rename = "map-settings")]
    MapSettings(MapSettings),
    #[serde(rename = "market")]
    MarketPrototype(MarketPrototype),
    #[serde(rename = "mining-drill")]
    MiningDrillPrototype(MiningDrillPrototype),
    #[serde(rename = "mod-data")]
    ModData(ModData),
    #[serde(rename = "module-category")]
    ModuleCategory(ModuleCategory),
    #[serde(rename = "module")]
    ModulePrototype(ModulePrototype),
    #[serde(rename = "module-transfer-achievement")]
    ModuleTransferAchievementPrototype(ModuleTransferAchievementPrototype),
    #[serde(rename = "mouse-cursor")]
    MouseCursor(MouseCursor),
    #[serde(rename = "movement-bonus-equipment")]
    MovementBonusEquipmentPrototype(MovementBonusEquipmentPrototype),
    #[serde(rename = "noise-expression")]
    NamedNoiseExpression(NamedNoiseExpression),
    #[serde(rename = "noise-function")]
    NamedNoiseFunction(NamedNoiseFunction),
    #[serde(rename = "night-vision-equipment")]
    NightVisionEquipmentPrototype(NightVisionEquipmentPrototype),
    #[serde(rename = "offshore-pump")]
    OffshorePumpPrototype(OffshorePumpPrototype),
    #[serde(rename = "optimized-particle")]
    ParticlePrototype(ParticlePrototype),
    #[serde(rename = "particle-source")]
    ParticleSourcePrototype(ParticleSourcePrototype),
    #[serde(rename = "pipe")]
    PipePrototype(PipePrototype),
    #[serde(rename = "pipe-to-ground")]
    PipeToGroundPrototype(PipeToGroundPrototype),
    #[serde(rename = "place-equipment-achievement")]
    PlaceEquipmentAchievementPrototype(PlaceEquipmentAchievementPrototype),
    #[serde(rename = "planet")]
    PlanetPrototype(PlanetPrototype),
    #[serde(rename = "plant")]
    PlantPrototype(PlantPrototype),
    #[serde(rename = "player-damaged-achievement")]
    PlayerDamagedAchievementPrototype(PlayerDamagedAchievementPrototype),
    #[serde(rename = "player-port")]
    PlayerPortPrototype(PlayerPortPrototype),
    #[serde(rename = "power-switch")]
    PowerSwitchPrototype(PowerSwitchPrototype),
    #[serde(rename = "procession-layer-inheritance-group")]
    ProcessionLayerInheritanceGroup(ProcessionLayerInheritanceGroup),
    #[serde(rename = "procession")]
    ProcessionPrototype(ProcessionPrototype),
    #[serde(rename = "produce-achievement")]
    ProduceAchievementPrototype(ProduceAchievementPrototype),
    #[serde(rename = "produce-per-hour-achievement")]
    ProducePerHourAchievementPrototype(ProducePerHourAchievementPrototype),
    #[serde(rename = "programmable-speaker")]
    ProgrammableSpeakerPrototype(ProgrammableSpeakerPrototype),
    #[serde(rename = "projectile")]
    ProjectilePrototype(ProjectilePrototype),
    #[serde(rename = "proxy-container")]
    ProxyContainerPrototype(ProxyContainerPrototype),
    #[serde(rename = "pump")]
    PumpPrototype(PumpPrototype),
    #[serde(rename = "quality")]
    QualityPrototype(QualityPrototype),
    #[serde(rename = "radar")]
    RadarPrototype(RadarPrototype),
    #[serde(rename = "rail-chain-signal")]
    RailChainSignalPrototype(RailChainSignalPrototype),
    #[serde(rename = "rail-planner")]
    RailPlannerPrototype(RailPlannerPrototype),
    #[serde(rename = "rail-ramp")]
    RailRampPrototype(RailRampPrototype),
    #[serde(rename = "rail-remnants")]
    RailRemnantsPrototype(RailRemnantsPrototype),
    #[serde(rename = "rail-signal")]
    RailSignalPrototype(RailSignalPrototype),
    #[serde(rename = "rail-support")]
    RailSupportPrototype(RailSupportPrototype),
    #[serde(rename = "reactor")]
    ReactorPrototype(ReactorPrototype),
    #[serde(rename = "recipe-category")]
    RecipeCategory(RecipeCategory),
    #[serde(rename = "recipe")]
    RecipePrototype(RecipePrototype),
    #[serde(rename = "remote-controller")]
    RemoteControllerPrototype(RemoteControllerPrototype),
    #[serde(rename = "repair-tool")]
    RepairToolPrototype(RepairToolPrototype),
    #[serde(rename = "research-achievement")]
    ResearchAchievementPrototype(ResearchAchievementPrototype),
    #[serde(rename = "research-with-science-pack-achievement")]
    ResearchWithSciencePackAchievementPrototype(
        ResearchWithSciencePackAchievementPrototype,
    ),
    #[serde(rename = "resource-category")]
    ResourceCategory(ResourceCategory),
    #[serde(rename = "resource")]
    ResourceEntityPrototype(ResourceEntityPrototype),
    #[serde(rename = "roboport-equipment")]
    RoboportEquipmentPrototype(RoboportEquipmentPrototype),
    #[serde(rename = "roboport")]
    RoboportPrototype(RoboportPrototype),
    #[serde(rename = "rocket-silo")]
    RocketSiloPrototype(RocketSiloPrototype),
    #[serde(rename = "rocket-silo-rocket")]
    RocketSiloRocketPrototype(RocketSiloRocketPrototype),
    #[serde(rename = "rocket-silo-rocket-shadow")]
    RocketSiloRocketShadowPrototype(RocketSiloRocketShadowPrototype),
    #[serde(rename = "segment")]
    SegmentPrototype(SegmentPrototype),
    #[serde(rename = "segmented-unit")]
    SegmentedUnitPrototype(SegmentedUnitPrototype),
    #[serde(rename = "selection-tool")]
    SelectionToolPrototype(SelectionToolPrototype),
    #[serde(rename = "selector-combinator")]
    SelectorCombinatorPrototype(SelectorCombinatorPrototype),
    #[serde(rename = "shoot-achievement")]
    ShootAchievementPrototype(ShootAchievementPrototype),
    #[serde(rename = "shortcut")]
    ShortcutPrototype(ShortcutPrototype),
    #[serde(rename = "simple-entity")]
    SimpleEntityPrototype(SimpleEntityPrototype),
    #[serde(rename = "simple-entity-with-force")]
    SimpleEntityWithForcePrototype(SimpleEntityWithForcePrototype),
    #[serde(rename = "simple-entity-with-owner")]
    SimpleEntityWithOwnerPrototype(SimpleEntityWithOwnerPrototype),
    #[serde(rename = "smoke-with-trigger")]
    SmokeWithTriggerPrototype(SmokeWithTriggerPrototype),
    #[serde(rename = "solar-panel-equipment")]
    SolarPanelEquipmentPrototype(SolarPanelEquipmentPrototype),
    #[serde(rename = "solar-panel")]
    SolarPanelPrototype(SolarPanelPrototype),
    #[serde(rename = "sound")]
    SoundPrototype(SoundPrototype),
    #[serde(rename = "space-connection-distance-traveled-achievement")]
    SpaceConnectionDistanceTraveledAchievementPrototype(
        SpaceConnectionDistanceTraveledAchievementPrototype,
    ),
    #[serde(rename = "space-connection")]
    SpaceConnectionPrototype(SpaceConnectionPrototype),
    #[serde(rename = "space-location")]
    SpaceLocationPrototype(SpaceLocationPrototype),
    #[serde(rename = "space-platform-hub")]
    SpacePlatformHubPrototype(SpacePlatformHubPrototype),
    #[serde(rename = "space-platform-starter-pack")]
    SpacePlatformStarterPackPrototype(SpacePlatformStarterPackPrototype),
    #[serde(rename = "spectator-controller")]
    SpectatorControllerPrototype(SpectatorControllerPrototype),
    #[serde(rename = "speech-bubble")]
    SpeechBubblePrototype(SpeechBubblePrototype),
    #[serde(rename = "spider-leg")]
    SpiderLegPrototype(SpiderLegPrototype),
    #[serde(rename = "spider-unit")]
    SpiderUnitPrototype(SpiderUnitPrototype),
    #[serde(rename = "spider-vehicle")]
    SpiderVehiclePrototype(SpiderVehiclePrototype),
    #[serde(rename = "spidertron-remote")]
    SpidertronRemotePrototype(SpidertronRemotePrototype),
    #[serde(rename = "splitter")]
    SplitterPrototype(SplitterPrototype),
    #[serde(rename = "sprite")]
    SpritePrototype(SpritePrototype),
    #[serde(rename = "sticker")]
    StickerPrototype(StickerPrototype),
    #[serde(rename = "storage-tank")]
    StorageTankPrototype(StorageTankPrototype),
    #[serde(rename = "straight-rail")]
    StraightRailPrototype(StraightRailPrototype),
    #[serde(rename = "surface-property")]
    SurfacePropertyPrototype(SurfacePropertyPrototype),
    #[serde(rename = "surface")]
    SurfacePrototype(SurfacePrototype),
    #[serde(rename = "technology")]
    TechnologyPrototype(TechnologyPrototype),
    #[serde(rename = "temporary-container")]
    TemporaryContainerPrototype(TemporaryContainerPrototype),
    #[serde(rename = "thruster")]
    ThrusterPrototype(ThrusterPrototype),
    #[serde(rename = "tile-effect")]
    TileEffectDefinition(TileEffectDefinition),
    #[serde(rename = "tile-ghost")]
    TileGhostPrototype(TileGhostPrototype),
    #[serde(rename = "tile")]
    TilePrototype(TilePrototype),
    #[serde(rename = "tips-and-tricks-item")]
    TipsAndTricksItem(TipsAndTricksItem),
    #[serde(rename = "tips-and-tricks-item-category")]
    TipsAndTricksItemCategory(TipsAndTricksItemCategory),
    #[serde(rename = "tool")]
    ToolPrototype(ToolPrototype),
    #[serde(rename = "train-path-achievement")]
    TrainPathAchievementPrototype(TrainPathAchievementPrototype),
    #[serde(rename = "train-stop")]
    TrainStopPrototype(TrainStopPrototype),
    #[serde(rename = "transport-belt")]
    TransportBeltPrototype(TransportBeltPrototype),
    #[serde(rename = "tree")]
    TreePrototype(TreePrototype),
    #[serde(rename = "trigger-target-type")]
    TriggerTargetType(TriggerTargetType),
    #[serde(rename = "trivial-smoke")]
    TrivialSmokePrototype(TrivialSmokePrototype),
    #[serde(rename = "turret")]
    TurretPrototype(TurretPrototype),
    #[serde(rename = "tutorial")]
    TutorialDefinition(TutorialDefinition),
    #[serde(rename = "underground-belt")]
    UndergroundBeltPrototype(UndergroundBeltPrototype),
    #[serde(rename = "unit")]
    UnitPrototype(UnitPrototype),
    #[serde(rename = "upgrade-item")]
    UpgradeItemPrototype(UpgradeItemPrototype),
    #[serde(rename = "use-entity-in-energy-production-achievement")]
    UseEntityInEnergyProductionAchievementPrototype(
        UseEntityInEnergyProductionAchievementPrototype,
    ),
    #[serde(rename = "use-item-achievement")]
    UseItemAchievementPrototype(UseItemAchievementPrototype),
    #[serde(rename = "utility-constants")]
    UtilityConstants(UtilityConstants),
    #[serde(rename = "utility-sounds")]
    UtilitySounds(UtilitySounds),
    #[serde(rename = "utility-sprites")]
    UtilitySprites(UtilitySprites),
    #[serde(rename = "valve")]
    ValvePrototype(ValvePrototype),
    #[serde(rename = "virtual-signal")]
    VirtualSignalPrototype(VirtualSignalPrototype),
    #[serde(rename = "wall")]
    WallPrototype(WallPrototype),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ApplyStarterPackTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ApplyTileTint {
    #[serde(rename = "primary")]
    LiteralPrimary,
    #[serde(rename = "secondary")]
    LiteralSecondary,
}
impl std::fmt::Display for ApplyTileTint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ApplyTileTint::LiteralPrimary => "primary",
                ApplyTileTint::LiteralSecondary => "secondary",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AreaTriggerItem {
    #[serde(flatten)]
    pub parent: TriggerItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_mode: Option<serde_json::Value>,
    pub radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_origin_is_valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_tooltip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_enemies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_entities: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_from_target: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ArtilleryRangeModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ArtilleryRemoteCapsuleAction {
    pub flare: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_sound_on_failure: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ArtilleryTriggerDelivery {
    #[serde(flatten)]
    pub parent: TriggerDeliveryItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_deviation: Option<f64>,
    pub projectile: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_deviation: Option<f64>,
    pub starting_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_speed_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_fired_artillery: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct AsteroidChunkID(pub String);
impl std::fmt::Display for AsteroidChunkID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for AsteroidChunkID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AsteroidCollectorGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_head_animation: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_head_top_animation: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_link: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub below_arm_pictures: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub below_ground_pictures: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_lamp_picture_full: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_lamp_picture_off: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_lamp_picture_on: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AsteroidGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_light: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lights: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_strength: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_purity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_strength: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sss_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sss_contrast: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AsteroidSettings {
    pub max_ray_portals_expanded_per_tick: u32,
    pub spawning_rate: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AsteroidSpawnPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle_when_stopped: Option<f64>,
    pub probability: f64,
    pub speed: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AsteroidVariation {
    pub color_texture: Sprite,
    pub normal_map: Sprite,
    pub roughness_map: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_shift: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum AttackParameters {
    #[serde(rename = "projectile")]
    ProjectileAttackParameters(ProjectileAttackParameters),
    #[serde(rename = "beam")]
    BeamAttackParameters(BeamAttackParameters),
    #[serde(rename = "stream")]
    StreamAttackParameters(StreamAttackParameters),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AttackReactionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_type: Option<DamageTypeID>,
    pub range: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_modifier: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Attenuation {
    pub curve_type: AttenuationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuning_parameter: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum AttenuationType {
    #[serde(rename = "none")]
    LiteralNone,
    #[serde(rename = "linear")]
    LiteralLinear,
    #[serde(rename = "logarithmic")]
    LiteralLogarithmic,
    #[serde(rename = "exponential")]
    LiteralExponential,
    #[serde(rename = "cosine")]
    LiteralCosine,
    #[serde(rename = "S-curve")]
    LiteralSCurve,
}
impl std::fmt::Display for AttenuationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                AttenuationType::LiteralNone => "none",
                AttenuationType::LiteralLinear => "linear",
                AttenuationType::LiteralLogarithmic => "logarithmic",
                AttenuationType::LiteralExponential => "exponential",
                AttenuationType::LiteralCosine => "cosine",
                AttenuationType::LiteralSCurve => "S-curve",
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct AutoplaceControlID(pub String);
impl std::fmt::Display for AutoplaceControlID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for AutoplaceControlID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AutoplaceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<
        std::collections::HashMap<serde_json::Value, FrequencySizeRichness>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_as_default: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AutoplaceSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<AutoplaceControlID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_expressions: Option<std::collections::HashMap<String, NoiseExpression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_functions: Option<std::collections::HashMap<String, NoiseFunction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_density: Option<u32>,
    pub probability_expression: NoiseExpression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub richness_expression: Option<NoiseExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_restriction: Option<Vec<TileIDRestriction>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BaseAttackParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ammo_categories: Option<Vec<AmmoCategoryID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ammo_category: Option<AmmoCategoryID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ammo_consumption_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ammo_type: Option<AmmoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<RotatedAnimation>,
    pub cooldown: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyclic_sound: Option<CyclicSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_target_for_projectile_delay: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_target_for_projectile_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_attack_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_range: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_slow_down_cooldown: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_slow_down_factor: Option<f64>,
    pub range: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_mode: Option<RangeMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<LayeredSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threatening_asteroid_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_collinear_ejection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_range: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_shooter_direction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warmup: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BaseEnergySource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissions_per_minute: Option<
        std::collections::HashMap<AirbornePollutantID, f64>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_no_network_icon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_no_power_icon: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BaseModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BasePumpWagonConnectionAnimations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<BasePumpWagonConnectionAnimations4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<BasePumpWagonConnectionAnimations4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BasePumpWagonConnectionAnimations4Way {
    pub east: Animation,
    pub north: Animation,
    pub south: Animation,
    pub west: Animation,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BaseStyleSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_margin: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_padding: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<HorizontalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontally_squashable: Option<StretchRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontally_stretchable: Option<StretchRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_by_search: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_margin: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_padding: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximal_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximal_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural_size: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_hide_by_search: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_margin: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_padding: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_margin: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_padding: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertically_squashable: Option<StretchRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertically_stretchable: Option<StretchRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeaconDistributionModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeaconGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_list: Option<Vec<AnimationElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_module_tint: Option<ModuleTint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_animation_when_idle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_light_when_idle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_icons_suppressed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_tint_mode: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_visualisations: Option<Vec<BeaconModuleVisualizations>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_modules_tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_animation_offset: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_animation_when_frozen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeaconModuleVisualization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_module_tint: Option<ModuleTint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_empty_slot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_draw_order: Option<i8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeaconModuleVisualizations {
    pub art_style: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<Vec<BeaconModuleVisualization>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_for_empty_slots: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeaconVisualizationTints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quaternary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tertiary: Option<Color>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeamAnimationSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail: Option<Animation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeamAttackParameters {
    #[serde(flatten)]
    pub parent: BaseAttackParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_direction_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_offset: Option<Vector>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeamGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beam: Option<BeamAnimationSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_segment_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground: Option<BeamAnimationSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_end_animation_rotation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomize_animation_per_segment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent_start_end_animations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeamTriggerDelivery {
    #[serde(flatten)]
    pub parent: TriggerDeliveryItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_to_shooter: Option<bool>,
    pub beam: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_with_source_or_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_offset: Option<Vector>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeltReaderLayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    pub sprites: RotatedAnimation,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeltStackSizeBonusModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeltTraverseTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum BlendMode {
    #[serde(rename = "normal")]
    LiteralNormal,
    #[serde(rename = "additive")]
    LiteralAdditive,
    #[serde(rename = "additive-soft")]
    LiteralAdditiveSoft,
    #[serde(rename = "multiplicative")]
    LiteralMultiplicative,
    #[serde(rename = "multiplicative-with-alpha")]
    LiteralMultiplicativeWithAlpha,
    #[serde(rename = "overwrite")]
    LiteralOverwrite,
}
impl std::fmt::Display for BlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                BlendMode::LiteralNormal => "normal",
                BlendMode::LiteralAdditive => "additive",
                BlendMode::LiteralAdditiveSoft => "additive-soft",
                BlendMode::LiteralMultiplicative => "multiplicative",
                BlendMode::LiteralMultiplicativeWithAlpha => "multiplicative-with-alpha",
                BlendMode::LiteralOverwrite => "overwrite",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BoilerPictureSet {
    pub east: BoilerPictures,
    pub north: BoilerPictures,
    pub south: BoilerPictures,
    pub west: BoilerPictures,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BoilerPictures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_glow: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Sprite>,
    pub structure: Animation,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BonusUtilityConstants {
    pub artillery_range: String,
    pub beacon_distribution: String,
    pub bulk_inserter: String,
    pub character: String,
    pub follower_robots: String,
    pub inserter: String,
    pub mining_productivity: String,
    pub research_speed: String,
    pub stack_inserter: String,
    pub train_braking_force: String,
    pub turret_attack: String,
    pub worker_robots: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BoolModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    pub modifier: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BorderImageSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_end: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_left_corner: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_right_corner: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_t: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_line: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_end: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_t: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_end: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_t: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_end: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_left_coner: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_right_corner: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_t: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_line: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum BoundingBox {
    Struct0,
    Variant1((MapPosition, MapPosition)),
    Variant2((MapPosition, MapPosition, f64)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BoxSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_whole_box: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_side_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_length: Option<f64>,
    pub sprite: Sprite,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BuildEntityByRobotTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BuildEntityTechnologyTrigger {
    pub entity: EntityIDFilter,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BuildEntityTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_by_dragging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_in_line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear_power_pole_line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityID>,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum BuildMode {
    #[serde(rename = "normal")]
    LiteralNormal,
    #[serde(rename = "forced")]
    LiteralForced,
    #[serde(rename = "superforced")]
    LiteralSuperforced,
}
impl std::fmt::Display for BuildMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                BuildMode::LiteralNormal => "normal",
                BuildMode::LiteralForced => "forced",
                BuildMode::LiteralSuperforced => "superforced",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BulkInserterCapacityBonusModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BurnerEnergySource {
    #[serde(flatten)]
    pub parent: BaseEnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refuel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burner_usage: Option<BurnerUsageID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burnt_inventory_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effectivity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_categories: Option<Vec<FuelCategoryID>>,
    pub fuel_inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_fuel: Option<ItemID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_fuel_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_flicker: Option<LightFlickeringDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke: Option<Vec<SmokeSource>>,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct BurnerUsageID(pub String);
impl std::fmt::Display for BurnerUsageID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for BurnerUsageID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ButtonStyleSpecification {
    #[serde(flatten)]
    pub parent: StyleWithClickableGraphicalSetSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked_vertical_offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_grayscale_picture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_shadow_under_picture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_horizontal_align: Option<HorizontalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_colors_of_picture_when_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_colors_of_picture_when_hovered_or_toggled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_progress_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_clicked_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_hovered_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough_color: Option<Color>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CameraEffectTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<u8>,
    pub duration: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ease_in_duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ease_out_duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_strength_max_distance: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CameraStyleSpecification {
    #[serde(flatten)]
    pub parent: EmptyWidgetStyleSpecification,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum CapsuleAction {
    #[serde(rename = "throw")]
    ThrowCapsuleAction(ThrowCapsuleAction),
    #[serde(rename = "equipment-remote")]
    ActivateEquipmentCapsuleAction(ActivateEquipmentCapsuleAction),
    #[serde(rename = "use-on-self")]
    UseOnSelfCapsuleAction(UseOnSelfCapsuleAction),
    #[serde(rename = "destroy-cliffs")]
    DestroyCliffsCapsuleAction(DestroyCliffsCapsuleAction),
    #[serde(rename = "artillery-remote")]
    ArtilleryRemoteCapsuleAction(ArtilleryRemoteCapsuleAction),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CaptureSpawnerTechnologyTrigger {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<EntityID>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CargoBayConnectableGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<CargoBayConnections>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<LayeredSprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CargoBayConnections {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_crossing: Option<LayeredSpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_horizontal_narrow: Option<LayeredSpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_horizontal_wide: Option<LayeredSpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_vertical_narrow: Option<LayeredSpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_vertical_wide: Option<LayeredSpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tileset: Option<Vec<Vec<LayeredSpriteVariations>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tileset_mapping: Option<std::collections::HashMap<u8, serde_json::Value>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CargoHatchDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy_timeout_ticks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_unit_entity_to_spawn: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closing_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entering_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatch_graphics: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatch_opening_ticks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatch_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub illumination_graphic_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_shadow_offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiving_cargo_units: Option<Vec<EntityID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sky_slice_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slice_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_height: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CargoLandingPadLimitModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CargoStationParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giga_hatch_definitions: Option<Vec<GigaCargoHatchDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatch_definitions: Option<Vec<CargoHatchDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_input_station: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_output_station: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_packed_cargo_units: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ChainTriggerDelivery {
    #[serde(flatten)]
    pub parent: TriggerDeliveryItem,
    pub chain: ActiveTriggerID,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ChangeRecipeProductivityModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    pub change: f64,
    pub recipe: RecipeID,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ChangeSurfaceTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub surface: String,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterArmorAnimation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armors: Option<Vec<ItemID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_smoke_cycles_per_tile: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flipped_shadow_running_with_gun: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flying: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flying_with_gun: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_in_air: Option<RotatedAnimation>,
    pub idle_with_gun: RotatedAnimation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_with_gun_in_air: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing: Option<RotatedAnimation>,
    pub mining_with_tool: RotatedAnimation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_with_tool_particles_animation_positions: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<RotatedAnimation>,
    pub running_with_gun: RotatedAnimation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke_cycles_per_tick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke_in_air: Option<Vec<SmokeSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_off: Option<RotatedAnimation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterBuildDistanceModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterCraftingSpeedModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterHealthBonusModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterInventorySlotsBonusModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterItemDropDistanceModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterItemPickupDistanceModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterLogisticRequestsModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterLogisticTrashSlotsModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterLootPickupDistanceModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterMiningSpeedModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterReachDistanceModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterResourceReachDistanceModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterRunningSpeedModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ChargableGraphics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_animation_is_looped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_cooldown: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_cooldown: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ChartUtilityConstants {
    pub artillery_range_color: Color,
    pub blue_signal_color: Color,
    pub chart_construction_robot_color: Color,
    pub chart_deconstruct_active_color: Color,
    pub chart_deconstruct_tint: Color,
    pub chart_delivery_to_me_logistic_robot_color: Color,
    pub chart_logistic_robot_color: Color,
    pub chart_mobile_construction_robot_color: Color,
    pub chart_personal_construction_robot_color: Color,
    pub chart_player_circle_size: f64,
    pub chart_train_stop_disabled_text_color: Color,
    pub chart_train_stop_full_text_color: Color,
    pub chart_train_stop_text_color: Color,
    pub circuit_network_member_color: Color,
    pub copper_wire_color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tag_max_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tag_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tag_selected_overlay_tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_color_by_type: Option<std::collections::HashMap<String, Color>>,
    pub default_enemy_color: Color,
    pub default_enemy_territory_color: Color,
    pub default_friendly_color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_friendly_color_by_type: Option<std::collections::HashMap<String, Color>>,
    pub disabled_switch_color: Color,
    pub electric_line_minimum_absolute_width: f64,
    pub electric_line_width: f64,
    pub electric_power_pole_color: Color,
    pub elevated_rail_color: Color,
    pub enabled_switch_color: Color,
    pub entity_ghost_color: Color,
    pub explosion_visualization_duration: u64,
    pub green_signal_color: Color,
    pub green_wire_color: Color,
    pub rail_color: Color,
    pub rail_ramp_color: Color,
    pub recipe_icon_scale: f64,
    pub red_signal_color: Color,
    pub red_wire_color: Color,
    pub resource_outline_selection_color: Color,
    pub tile_ghost_color: Color,
    pub train_current_path_outline_color: Color,
    pub train_path_color: Color,
    pub train_preview_path_outline_color: Color,
    pub turret_range_color: Color,
    pub vehicle_inner_color: Color,
    pub vehicle_outer_color: Color,
    pub vehicle_outer_color_selected: Color,
    pub vehicle_wagon_connection_color: Color,
    pub yellow_signal_color: Color,
    pub zoom_threshold_to_draw_spider_path: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CheckBoxStyleSpecification {
    #[serde(flatten)]
    pub parent: StyleWithClickableGraphicalSetSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkmark: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_checkmark: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediate_mark: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_padding: Option<u32>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CircuitConditionConnector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<ComparatorString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CircuitConnectorDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<WireConnectionPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprites: Option<CircuitConnectorSprites>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CircuitConnectorLayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<RenderLayer>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CircuitConnectorSecondaryDrawOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<i8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CircuitConnectorSprites {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_led_light_offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_main: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_shadow: Option<Sprite>,
    pub led_blue: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub led_blue_off: Option<Sprite>,
    pub led_green: Sprite,
    pub led_light: LightDefinition,
    pub led_red: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_green_led_light_offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_pins: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_pins_shadow: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CircuitNetworkModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CircularParticleCreationSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_distance_orientation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_deviation: Option<f64>,
    pub name: ParticleID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_deviation: Option<f64>,
    pub starting_frame_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_frame_speed_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_source_position: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_speed_deviation: Option<f64>,
}
pub type CircularProjectileCreationSpecification = Vec<(f64, Vector)>;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ClearCursorTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CliffDeconstructionEnabledModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CliffPlacementSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliff_elevation_0: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliff_elevation_interval: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliff_smoothing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<AutoplaceControlID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub richness: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum CloudEffectStyle {
    #[serde(rename = "none")]
    LiteralNone,
    #[serde(rename = "euclidean")]
    LiteralEuclidean,
    #[serde(rename = "manhattan")]
    LiteralManhattan,
    #[serde(rename = "euclidean-outside")]
    LiteralEuclideanOutside,
    #[serde(rename = "manhattan-outside")]
    LiteralManhattanOutside,
    #[serde(rename = "horizontal-stripe")]
    LiteralHorizontalStripe,
    #[serde(rename = "texture")]
    LiteralTexture,
    #[serde(rename = "texture-outside")]
    LiteralTextureOutside,
}
impl std::fmt::Display for CloudEffectStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                CloudEffectStyle::LiteralNone => "none",
                CloudEffectStyle::LiteralEuclidean => "euclidean",
                CloudEffectStyle::LiteralManhattan => "manhattan",
                CloudEffectStyle::LiteralEuclideanOutside => "euclidean-outside",
                CloudEffectStyle::LiteralManhattanOutside => "manhattan-outside",
                CloudEffectStyle::LiteralHorizontalStripe => "horizontal-stripe",
                CloudEffectStyle::LiteralTexture => "texture",
                CloudEffectStyle::LiteralTextureOutside => "texture-outside",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CloudsEffectProperties {
    pub additional_density_sample: CloudsTextureCoordinateTransformation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density_at_night: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_exponent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_factor_at_night: Option<f64>,
    pub detail_noise_texture: EffectTexture,
    pub detail_sample_1: CloudsTextureCoordinateTransformation,
    pub detail_sample_2: CloudsTextureCoordinateTransformation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_sample_morph_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_speed_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity_at_night: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape_factor: Option<f64>,
    pub shape_noise_texture: EffectTexture,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape_warp_strength: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape_warp_weight: Option<f64>,
    pub warp_sample_1: CloudsTextureCoordinateTransformation,
    pub warp_sample_2: CloudsTextureCoordinateTransformation,
    pub warped_shape_sample: CloudsTextureCoordinateTransformation,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CloudsTextureCoordinateTransformation {
    pub scale: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed_factor: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ClusterTriggerItem {
    #[serde(flatten)]
    pub parent: TriggerItem,
    pub cluster_count: u32,
    pub distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_deviation: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct CollisionLayerID(pub String);
impl std::fmt::Display for CollisionLayerID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for CollisionLayerID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CollisionMaskConnector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colliding_with_tiles_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consider_tile_transitions: Option<bool>,
    pub layers: std::collections::HashMap<CollisionLayerID, bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_colliding_with_itself: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum Color {
    Struct0,
    Variant1((f64, f64, f64)),
    Variant2((f64, f64, f64, f64)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ColorFilterData {
    pub localised_name: String,
    pub matrix: Vec<Vec<f64>>,
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ColorHintSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Color>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum ColorLookupTable {
    FileName(Box<FileName>),
    LiteralIdentity,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ColumnAlignment {
    pub alignment: Alignment,
    pub column: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ColumnWidth {
    #[serde(flatten)]
    pub parent: ColumnWidthItem,
    pub column: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ColumnWidthItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximal_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ComparatorString {
    #[serde(rename = "=")]
    LiteralEq,
    #[serde(rename = ">")]
    LiteralGt,
    #[serde(rename = "<")]
    LiteralLt,
    #[serde(rename = "≥")]
    LiteralGreaterEqual,
    #[serde(rename = ">=")]
    LiteralGe,
    #[serde(rename = "≤")]
    LiteralLessEqual,
    #[serde(rename = "<=")]
    LiteralLe,
    #[serde(rename = "≠")]
    LiteralNotEqual,
    #[serde(rename = "!=")]
    LiteralNe,
}
impl std::fmt::Display for ComparatorString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ComparatorString::LiteralEq => "=",
                ComparatorString::LiteralGt => ">",
                ComparatorString::LiteralLt => "<",
                ComparatorString::LiteralGreaterEqual => "≥",
                ComparatorString::LiteralGe => ">=",
                ComparatorString::LiteralLessEqual => "≤",
                ComparatorString::LiteralLe => "<=",
                ComparatorString::LiteralNotEqual => "≠",
                ComparatorString::LiteralNe => "!=",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ConnectableEntityGraphics {
    pub corner_left_down: SpriteVariations,
    pub corner_left_up: SpriteVariations,
    pub corner_right_down: SpriteVariations,
    pub corner_right_up: SpriteVariations,
    pub cross: SpriteVariations,
    pub ending_down: SpriteVariations,
    pub ending_left: SpriteVariations,
    pub ending_right: SpriteVariations,
    pub ending_up: SpriteVariations,
    pub single: SpriteVariations,
    pub straight_horizontal: SpriteVariations,
    pub straight_vertical: SpriteVariations,
    pub t_down: SpriteVariations,
    pub t_left: SpriteVariations,
    pub t_right: SpriteVariations,
    pub t_up: SpriteVariations,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ConsumingType {
    #[serde(rename = "none")]
    LiteralNone,
    #[serde(rename = "game-only")]
    LiteralGameOnly,
}
impl std::fmt::Display for ConsumingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ConsumingType::LiteralNone => "none",
                ConsumingType::LiteralGameOnly => "game-only",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum ControlPoint {
    Struct0,
    Variant1((f64, f64)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CountBasedTipTrigger {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CoverGraphicEffectData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_traveled_strength: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_movement_strength: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_to: Option<EffectRelativeTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CloudEffectStyle>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CoverGraphicProcessionLayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_effect: Option<CoverGraphicEffectData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_traveled_strength: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<CoverGraphicEffectData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_graphic: Option<ProcessionGraphic>,
    pub frames: Vec<CoverGraphicProcessionLayerBezierControlPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphic: Option<ProcessionGraphic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_from: Option<ProcessionLayerInheritanceGroupID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cloud_effect_advanced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_quad_texture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_graphic: Option<ProcessionGraphic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_movement_strength: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_group: Option<ProcessionLayerInheritanceGroupID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_with_pod: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_draw_order: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_relative_to: Option<EffectRelativeTo>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_size: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CoverGraphicProcessionLayerBezierControlPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_effect_scale_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_effect_scale_max_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_effect_scale_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_effect_scale_min_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_effect_shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_effect_shift_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_effect_shift_rate_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_effect_shift_t: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_scale_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_scale_max_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_scale_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_scale_min_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_shift_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_shift_rate_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_shift_t: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_rate_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_t: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CraftFluidTechnologyTrigger {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    pub fluid: FluidID,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CraftItemTechnologyTrigger {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    pub item: ItemIDFilter,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CraftItemTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive: Option<bool>,
    pub event_type: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<ItemID>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CraftingMachineGraphicsSet {
    #[serde(flatten)]
    pub parent: WorkingVisualisations,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector_layer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector_secondary_draw_order: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_animation_when_frozen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CranePart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sprite_rotation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_effect: Option<CranePartDyingEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extendable_length: Option<Vector3D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extendable_length_grappler: Option<Vector3D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contractible_by_cropping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_shift: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_position: Option<Vector3D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_position_grappler: Option<Vector3D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotated_sprite: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotated_sprite_reflection: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotated_sprite_shadow: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_to_fit_model: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_scale_for_perspective: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_end: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_end_arm_extent_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_start: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite_reflection: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite_shadow: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_length: Option<Vector3D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_length_grappler: Option<Vector3D>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CranePartDyingEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosion: Option<ExplosionDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosion_linear_distance_step: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_effect_linear_distance_step: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_effects: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CraterPlacementDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_segments_to_place: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_probability: Option<f64>,
    pub segments: Vec<CraterSegment>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CraterSegment {
    pub offset: Vector,
    pub orientation: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateAsteroidChunkEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    pub asteroid_name: AsteroidChunkID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_deviation: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsets: Option<Vec<Vector>>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateDecorativesTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_projection: Option<bool>,
    pub decorative: DecorativeID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_curve: Option<f64>,
    pub spawn_max: u16,
    pub spawn_max_radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_min: Option<u16>,
    pub spawn_min_radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread_evenly: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateEntityTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_if_over_space: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_enemy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_buildability: Option<bool>,
    pub entity_name: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_non_colliding_position: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_no_enemies_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_colliding_fail_result: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_colliding_search_precision: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_colliding_search_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_deviation: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsets: Option<Vec<Vector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_when_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_ghosts_and_corpses: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_details_in_tooltip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_tooltip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_collision_mask: Option<TileCollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_created_entity: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateExplosionTriggerEffectItem {
    #[serde(flatten)]
    pub parent: CreateEntityTriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_while_moving: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_movement_distance_from_projectile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_movement_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_movement_distance_deviation: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateFireTriggerEffectItem {
    #[serde(flatten)]
    pub parent: CreateEntityTriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_ground_flame_count: Option<u8>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateGhostOnEntityDeathModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateParticleTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_tile_tint: Option<ApplyTileTint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_speed_deviation: Option<f64>,
    pub initial_height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_height_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_vertical_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_vertical_speed_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_deviation: Option<SimpleBoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsets: Option<Vec<Vector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_when_visible: Option<bool>,
    pub particle_name: ParticleID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_offsets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_tooltip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_from_center: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_from_center_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_length_deviation: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<Color>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreatePollutionTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<EntityID>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_entity_from_trigger: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateSmokeTriggerEffectItem {
    #[serde(flatten)]
    pub parent: CreateEntityTriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_from_center: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_from_center_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_multiplier_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_frame: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_frame_deviation: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateSpacePlatformTechnologyTrigger {
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateStickerTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_tooltip: Option<bool>,
    pub sticker: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_created_entity: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreateTrivialSmokeEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_deviation: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsets: Option<Vec<Vector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_when_visible: Option<bool>,
    pub smoke_name: TrivialSmokeID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_from_center: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_from_center_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_multiplier_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_frame: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_frame_deviation: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CursorBoxSpecification {
    pub blueprint_snap_rectangle: Vec<BoxSpecification>,
    pub copy: Vec<BoxSpecification>,
    pub electricity: Vec<BoxSpecification>,
    pub logistics: Vec<BoxSpecification>,
    pub multiplayer_selection: Vec<BoxSpecification>,
    pub not_allowed: Vec<BoxSpecification>,
    pub pair: Vec<BoxSpecification>,
    pub regular: Vec<BoxSpecification>,
    pub spidertron_remote_selected: Vec<BoxSpecification>,
    pub spidertron_remote_to_be_selected: Vec<BoxSpecification>,
    pub train_visualization: Vec<BoxSpecification>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum CursorBoxType {
    #[serde(rename = "entity")]
    LiteralEntity,
    #[serde(rename = "multiplayer-entity")]
    LiteralMultiplayerEntity,
    #[serde(rename = "electricity")]
    LiteralElectricity,
    #[serde(rename = "copy")]
    LiteralCopy,
    #[serde(rename = "not-allowed")]
    LiteralNotAllowed,
    #[serde(rename = "pair")]
    LiteralPair,
    #[serde(rename = "logistics")]
    LiteralLogistics,
    #[serde(rename = "train-visualization")]
    LiteralTrainVisualization,
    #[serde(rename = "blueprint-snap-rectangle")]
    LiteralBlueprintSnapRectangle,
    #[serde(rename = "spidertron-remote-selected")]
    LiteralSpidertronRemoteSelected,
    #[serde(rename = "spidertron-remote-to-be-selected")]
    LiteralSpidertronRemoteToBeSelected,
}
impl std::fmt::Display for CursorBoxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                CursorBoxType::LiteralEntity => "entity",
                CursorBoxType::LiteralMultiplayerEntity => "multiplayer-entity",
                CursorBoxType::LiteralElectricity => "electricity",
                CursorBoxType::LiteralCopy => "copy",
                CursorBoxType::LiteralNotAllowed => "not-allowed",
                CursorBoxType::LiteralPair => "pair",
                CursorBoxType::LiteralLogistics => "logistics",
                CursorBoxType::LiteralTrainVisualization => "train-visualization",
                CursorBoxType::LiteralBlueprintSnapRectangle => {
                    "blueprint-snap-rectangle"
                }
                CursorBoxType::LiteralSpidertronRemoteSelected => {
                    "spidertron-remote-selected"
                }
                CursorBoxType::LiteralSpidertronRemoteToBeSelected => {
                    "spidertron-remote-to-be-selected"
                }
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct CustomEventID(pub String);
impl std::fmt::Display for CustomEventID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for CustomEventID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CustomTooltipField {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_values: Option<std::collections::HashMap<QualityID, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_factoriopedia: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_tooltip: Option<bool>,
    pub value: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CyclicSound {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DamageEntityTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_damage_to_trees: Option<bool>,
    pub damage: DamageParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_damage_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_distance_threshold: Option<u16>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_damage_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_distance_threshold: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_substitute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vaporize: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DamageParameters {
    pub amount: f64,
    pub r#type: DamageTypeID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DamageTileTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    pub damage: DamageParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum DamageTypeFilters {
    Struct0,
    DamageTypeID(Box<DamageTypeID>),
    Array2(Vec<DamageTypeID>),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct DamageTypeID(pub String);
impl std::fmt::Display for DamageTypeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for DamageTypeID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Data {
    pub extend: serde_json::Value,
    pub is_demo: bool,
    pub raw: std::collections::HashMap<
        String,
        std::collections::HashMap<String, AnyPrototype>,
    >,
}
pub type DaytimeColorLookupTable = Vec<(f64, ColorLookupTable)>;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DeconstructionTimeToLiveModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct DecorativeID(pub String);
impl std::fmt::Display for DecorativeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for DecorativeID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DelayedTriggerDelivery {
    #[serde(flatten)]
    pub parent: TriggerDeliveryItem,
    pub delayed_trigger: ActiveTriggerID,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DependenciesMetTipTrigger {
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DestroyCliffsCapsuleAction {
    pub attack_parameters: AttackParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_sound_on_failure: Option<bool>,
    pub radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_stack: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DestroyCliffsTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosion_at_cliff: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosion_at_trigger: Option<EntityID>,
    pub radius: f64,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DestroyDecorativesTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoratives_with_trigger_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_decals: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_soft_decoratives: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_decorative_trigger: Option<bool>,
    pub radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_render_layer: Option<RenderLayer>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DifficultySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoil_time_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology_price_multiplier: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DirectTriggerItem {
    #[serde(flatten)]
    pub parent: TriggerItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_enabled: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DirectionShift {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<Vector>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum DirectionString {
    #[serde(rename = "north")]
    LiteralNorth,
    #[serde(rename = "north_north_east")]
    LiteralNorthNorthEast,
    #[serde(rename = "north_east")]
    LiteralNorthEast,
    #[serde(rename = "east_north_east")]
    LiteralEastNorthEast,
    #[serde(rename = "east")]
    LiteralEast,
    #[serde(rename = "east_south_east")]
    LiteralEastSouthEast,
    #[serde(rename = "south_east")]
    LiteralSouthEast,
    #[serde(rename = "south_south_east")]
    LiteralSouthSouthEast,
    #[serde(rename = "south")]
    LiteralSouth,
    #[serde(rename = "south_south_west")]
    LiteralSouthSouthWest,
    #[serde(rename = "south_west")]
    LiteralSouthWest,
    #[serde(rename = "west_south_west")]
    LiteralWestSouthWest,
    #[serde(rename = "west")]
    LiteralWest,
    #[serde(rename = "west_north_west")]
    LiteralWestNorthWest,
    #[serde(rename = "north_west")]
    LiteralNorthWest,
    #[serde(rename = "north_north_west")]
    LiteralNorthNorthWest,
}
impl std::fmt::Display for DirectionString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                DirectionString::LiteralNorth => "north",
                DirectionString::LiteralNorthNorthEast => "north_north_east",
                DirectionString::LiteralNorthEast => "north_east",
                DirectionString::LiteralEastNorthEast => "east_north_east",
                DirectionString::LiteralEast => "east",
                DirectionString::LiteralEastSouthEast => "east_south_east",
                DirectionString::LiteralSouthEast => "south_east",
                DirectionString::LiteralSouthSouthEast => "south_south_east",
                DirectionString::LiteralSouth => "south",
                DirectionString::LiteralSouthSouthWest => "south_south_west",
                DirectionString::LiteralSouthWest => "south_west",
                DirectionString::LiteralWestSouthWest => "west_south_west",
                DirectionString::LiteralWest => "west",
                DirectionString::LiteralWestNorthWest => "west_north_west",
                DirectionString::LiteralNorthWest => "north_west",
                DirectionString::LiteralNorthNorthWest => "north_north_west",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DoubleSliderStyleSpecification {
    #[serde(flatten)]
    pub parent: SliderStyleSpecification,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DropDownStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_style: Option<ButtonStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_box_style: Option<ListBoxStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_and_title_spacing: Option<i16>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DropItemTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_into_entity: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EditorUtilityConstants {
    pub cliff_editor_remove_cliffs_color: Color,
    pub clone_editor_brush_cursor_preview_tint: Color,
    pub clone_editor_brush_destination_color: Color,
    pub clone_editor_brush_source_color: Color,
    pub clone_editor_brush_world_preview_tint: Color,
    pub clone_editor_copy_destination_allowed_color: Color,
    pub clone_editor_copy_destination_not_allowed_color: Color,
    pub clone_editor_copy_source_color: Color,
    pub decorative_editor_selection_preview_radius: u8,
    pub decorative_editor_selection_preview_tint: Color,
    pub force_editor_select_area_color: Color,
    pub script_editor_drag_area_color: Color,
    pub script_editor_select_area_color: Color,
    pub tile_editor_area_selection_color: Color,
    pub tile_editor_selection_preview_radius: u8,
    pub tile_editor_selection_preview_tint: Color,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Effect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pollution: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub productivity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EffectReceiver {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_effect: Option<Effect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_limits: Option<EffectValueRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pollution_limits: Option<EffectValueRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub productivity_limits: Option<EffectValueRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_limits: Option<EffectValueRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_limits: Option<EffectValueRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_beacon_effects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_module_effects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_surface_effects: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum EffectRelativeTo {
    #[serde(rename = "ground-origin")]
    LiteralGroundOrigin,
    #[serde(rename = "pod")]
    LiteralPod,
    #[serde(rename = "spawn-origin")]
    LiteralSpawnOrigin,
}
impl std::fmt::Display for EffectRelativeTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                EffectRelativeTo::LiteralGroundOrigin => "ground-origin",
                EffectRelativeTo::LiteralPod => "pod",
                EffectRelativeTo::LiteralSpawnOrigin => "spawn-origin",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EffectTexture {
    #[serde(flatten)]
    pub parent: SpriteSource,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum EffectTypeLimitation {
    Variant0(serde_json::Value),
    Array1(Vec<serde_json::Value>),
}
pub type EffectValue = f64;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EffectValueRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum EffectVariation {
    #[serde(rename = "lava")]
    LiteralLava,
    #[serde(rename = "wetland-water")]
    LiteralWetlandWater,
    #[serde(rename = "oil")]
    LiteralOil,
    #[serde(rename = "water")]
    LiteralWater,
}
impl std::fmt::Display for EffectVariation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                EffectVariation::LiteralLava => "lava",
                EffectVariation::LiteralWetlandWater => "wetland-water",
                EffectVariation::LiteralOil => "oil",
                EffectVariation::LiteralWater => "water",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ElectricEnergySource {
    #[serde(flatten)]
    pub parent: BaseEnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_capacity: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drain: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_flow_limit: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_flow_limit: Option<Energy>,
    pub r#type: String,
    pub usage_priority: ElectricUsagePriority,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ElectricUsagePriority {
    #[serde(rename = "primary-input")]
    LiteralPrimaryInput,
    #[serde(rename = "primary-output")]
    LiteralPrimaryOutput,
    #[serde(rename = "secondary-input")]
    LiteralSecondaryInput,
    #[serde(rename = "secondary-output")]
    LiteralSecondaryOutput,
    #[serde(rename = "tertiary")]
    LiteralTertiary,
    #[serde(rename = "solar")]
    LiteralSolar,
    #[serde(rename = "lamp")]
    LiteralLamp,
    #[serde(rename = "dynamic")]
    LiteralDynamic,
}
impl std::fmt::Display for ElectricUsagePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ElectricUsagePriority::LiteralPrimaryInput => "primary-input",
                ElectricUsagePriority::LiteralPrimaryOutput => "primary-output",
                ElectricUsagePriority::LiteralSecondaryInput => "secondary-input",
                ElectricUsagePriority::LiteralSecondaryOutput => "secondary-output",
                ElectricUsagePriority::LiteralTertiary => "tertiary",
                ElectricUsagePriority::LiteralSolar => "solar",
                ElectricUsagePriority::LiteralLamp => "lamp",
                ElectricUsagePriority::LiteralDynamic => "dynamic",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum ElementImageSet {
    Struct0,
    ElementImageSetLayer(Box<ElementImageSetLayer>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum ElementImageSetLayer {
    Struct0,
    Sprite(Box<Sprite>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EmptyWidgetStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphical_set: Option<ElementImageSet>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EnemyEvolutionSettings {
    pub destroy_factor: f64,
    pub enabled: bool,
    pub pollution_factor: f64,
    pub time_factor: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EnemyExpansionSettings {
    pub building_coefficient: f64,
    pub enabled: bool,
    pub enemy_building_influence_radius: u32,
    pub evolution_group_size_factor: f64,
    pub friendly_base_influence_radius: u32,
    pub max_colliding_tiles_coefficient: f64,
    pub max_expansion_cooldown: u32,
    pub max_expansion_distance: u32,
    pub min_expansion_cooldown: u32,
    pub min_expansion_distance: u32,
    pub neighbouring_base_chunk_coefficient: f64,
    pub neighbouring_chunk_coefficient: f64,
    pub other_base_coefficient: f64,
    pub settler_group_max_size: u32,
    pub settler_group_min_size: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EnemySpawnerAbsorption {
    pub absolute: f64,
    pub proportional: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EnemySpawnerGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_animation_offset: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_animations: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_layer_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_effect_map_animations: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct Energy(pub String);
impl std::fmt::Display for Energy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for Energy {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum EnergySource {
    #[serde(rename = "electric")]
    ElectricEnergySource(ElectricEnergySource),
    #[serde(rename = "burner")]
    BurnerEnergySource(BurnerEnergySource),
    #[serde(rename = "heat")]
    HeatEnergySource(HeatEnergySource),
    #[serde(rename = "fluid")]
    FluidEnergySource(FluidEnergySource),
    #[serde(rename = "void")]
    VoidEnergySource(VoidEnergySource),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EnterVehicleTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type_only: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<EntityID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EntityBuildAnimationPiece {
    pub body: Animation,
    pub top: Animation,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EntityBuildAnimations {
    pub back_left: EntityBuildAnimationPiece,
    pub back_right: EntityBuildAnimationPiece,
    pub front_left: EntityBuildAnimationPiece,
    pub front_right: EntityBuildAnimationPiece,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct EntityID(pub String);
impl std::fmt::Display for EntityID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for EntityID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum EntityIDFilter {
    Struct0,
    EntityID(Box<EntityID>),
}
pub type EntityPrototypeFlags = Vec<serde_json::Value>;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EntityRendererSearchBoxLimits {
    pub bottom: u8,
    pub left: u8,
    pub right: u8,
    pub top: u8,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum EntityStatus {
    #[serde(rename = "working")]
    LiteralWorking,
    #[serde(rename = "normal")]
    LiteralNormal,
    #[serde(rename = "ghost")]
    LiteralGhost,
    #[serde(rename = "not-plugged-in-electric-network")]
    LiteralNotPluggedInElectricNetwork,
    #[serde(rename = "networks-connected")]
    LiteralNetworksConnected,
    #[serde(rename = "networks-disconnected")]
    LiteralNetworksDisconnected,
    #[serde(rename = "no-ammo")]
    LiteralNoAmmo,
    #[serde(rename = "waiting-for-target-to-be-built")]
    LiteralWaitingForTargetToBeBuilt,
    #[serde(rename = "waiting-for-train")]
    LiteralWaitingForTrain,
    #[serde(rename = "no-power")]
    LiteralNoPower,
    #[serde(rename = "low-temperature")]
    LiteralLowTemperature,
    #[serde(rename = "charging")]
    LiteralCharging,
    #[serde(rename = "discharging")]
    LiteralDischarging,
    #[serde(rename = "fully-charged")]
    LiteralFullyCharged,
    #[serde(rename = "no-fuel")]
    LiteralNoFuel,
    #[serde(rename = "no-food")]
    LiteralNoFood,
    #[serde(rename = "out-of-logistic-network")]
    LiteralOutOfLogisticNetwork,
    #[serde(rename = "no-recipe")]
    LiteralNoRecipe,
    #[serde(rename = "no-ingredients")]
    LiteralNoIngredients,
    #[serde(rename = "no-input-fluid")]
    LiteralNoInputFluid,
    #[serde(rename = "no-research-in-progress")]
    LiteralNoResearchInProgress,
    #[serde(rename = "no-minable-resources")]
    LiteralNoMinableResources,
    #[serde(rename = "low-input-fluid")]
    LiteralLowInputFluid,
    #[serde(rename = "low-power")]
    LiteralLowPower,
    #[serde(rename = "not-connected-to-rail")]
    LiteralNotConnectedToRail,
    #[serde(rename = "cant-divide-segments")]
    LiteralCantDivideSegments,
    #[serde(rename = "recharging-after-power-outage")]
    LiteralRechargingAfterPowerOutage,
    #[serde(rename = "no-modules-to-transmit")]
    LiteralNoModulesToTransmit,
    #[serde(rename = "disabled-by-control-behavior")]
    LiteralDisabledByControlBehavior,
    #[serde(rename = "opened-by-circuit-network")]
    LiteralOpenedByCircuitNetwork,
    #[serde(rename = "closed-by-circuit-network")]
    LiteralClosedByCircuitNetwork,
    #[serde(rename = "disabled-by-script")]
    LiteralDisabledByScript,
    #[serde(rename = "disabled")]
    LiteralDisabled,
    #[serde(rename = "turned-off-during-daytime")]
    LiteralTurnedOffDuringDaytime,
    #[serde(rename = "fluid-ingredient-shortage")]
    LiteralFluidIngredientShortage,
    #[serde(rename = "item-ingredient-shortage")]
    LiteralItemIngredientShortage,
    #[serde(rename = "full-output")]
    LiteralFullOutput,
    #[serde(rename = "not-enough-space-in-output")]
    LiteralNotEnoughSpaceInOutput,
    #[serde(rename = "full-burnt-result-output")]
    LiteralFullBurntResultOutput,
    #[serde(rename = "marked-for-deconstruction")]
    LiteralMarkedForDeconstruction,
    #[serde(rename = "missing-required-fluid")]
    LiteralMissingRequiredFluid,
    #[serde(rename = "missing-science-packs")]
    LiteralMissingSciencePacks,
    #[serde(rename = "waiting-for-source-items")]
    LiteralWaitingForSourceItems,
    #[serde(rename = "waiting-for-space-in-destination")]
    LiteralWaitingForSpaceInDestination,
    #[serde(rename = "preparing-rocket-for-launch")]
    LiteralPreparingRocketForLaunch,
    #[serde(rename = "waiting-to-launch-rocket")]
    LiteralWaitingToLaunchRocket,
    #[serde(rename = "waiting-for-space-in-platform-hub")]
    LiteralWaitingForSpaceInPlatformHub,
    #[serde(rename = "launching-rocket")]
    LiteralLaunchingRocket,
    #[serde(rename = "thrust-not-required")]
    LiteralThrustNotRequired,
    #[serde(rename = "not-enough-thrust")]
    LiteralNotEnoughThrust,
    #[serde(rename = "on-the-way")]
    LiteralOnTheWay,
    #[serde(rename = "waiting-in-orbit")]
    LiteralWaitingInOrbit,
    #[serde(rename = "waiting-for-rocket-to-arrive")]
    LiteralWaitingForRocketToArrive,
    #[serde(rename = "waiting-to-clear-drop-slots")]
    LiteralWaitingToClearDropSlots,
    #[serde(rename = "no-path")]
    LiteralNoPath,
    #[serde(rename = "broken")]
    LiteralBroken,
    #[serde(rename = "none")]
    LiteralNone,
    #[serde(rename = "frozen")]
    LiteralFrozen,
    #[serde(rename = "paused")]
    LiteralPaused,
    #[serde(rename = "not-connected-to-hub-or-pad")]
    LiteralNotConnectedToHubOrPad,
    #[serde(rename = "too-far-from-pad-to-unload")]
    LiteralTooFarFromPadToUnload,
    #[serde(rename = "computing-navigation")]
    LiteralComputingNavigation,
    #[serde(rename = "no-filter")]
    LiteralNoFilter,
    #[serde(rename = "waiting-at-stop")]
    LiteralWaitingAtStop,
    #[serde(rename = "waiting-for-upgrade")]
    LiteralWaitingForUpgrade,
    #[serde(rename = "destination-stop-full")]
    LiteralDestinationStopFull,
    #[serde(rename = "pipeline-overextended")]
    LiteralPipelineOverextended,
    #[serde(rename = "no-spot-seedable-by-inputs")]
    LiteralNoSpotSeedableByInputs,
    #[serde(rename = "waiting-for-plants-to-grow")]
    LiteralWaitingForPlantsToGrow,
    #[serde(rename = "recipe-not-researched")]
    LiteralRecipeNotResearched,
    #[serde(rename = "armed")]
    LiteralArmed,
}
impl std::fmt::Display for EntityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                EntityStatus::LiteralWorking => "working",
                EntityStatus::LiteralNormal => "normal",
                EntityStatus::LiteralGhost => "ghost",
                EntityStatus::LiteralNotPluggedInElectricNetwork => {
                    "not-plugged-in-electric-network"
                }
                EntityStatus::LiteralNetworksConnected => "networks-connected",
                EntityStatus::LiteralNetworksDisconnected => "networks-disconnected",
                EntityStatus::LiteralNoAmmo => "no-ammo",
                EntityStatus::LiteralWaitingForTargetToBeBuilt => {
                    "waiting-for-target-to-be-built"
                }
                EntityStatus::LiteralWaitingForTrain => "waiting-for-train",
                EntityStatus::LiteralNoPower => "no-power",
                EntityStatus::LiteralLowTemperature => "low-temperature",
                EntityStatus::LiteralCharging => "charging",
                EntityStatus::LiteralDischarging => "discharging",
                EntityStatus::LiteralFullyCharged => "fully-charged",
                EntityStatus::LiteralNoFuel => "no-fuel",
                EntityStatus::LiteralNoFood => "no-food",
                EntityStatus::LiteralOutOfLogisticNetwork => "out-of-logistic-network",
                EntityStatus::LiteralNoRecipe => "no-recipe",
                EntityStatus::LiteralNoIngredients => "no-ingredients",
                EntityStatus::LiteralNoInputFluid => "no-input-fluid",
                EntityStatus::LiteralNoResearchInProgress => "no-research-in-progress",
                EntityStatus::LiteralNoMinableResources => "no-minable-resources",
                EntityStatus::LiteralLowInputFluid => "low-input-fluid",
                EntityStatus::LiteralLowPower => "low-power",
                EntityStatus::LiteralNotConnectedToRail => "not-connected-to-rail",
                EntityStatus::LiteralCantDivideSegments => "cant-divide-segments",
                EntityStatus::LiteralRechargingAfterPowerOutage => {
                    "recharging-after-power-outage"
                }
                EntityStatus::LiteralNoModulesToTransmit => "no-modules-to-transmit",
                EntityStatus::LiteralDisabledByControlBehavior => {
                    "disabled-by-control-behavior"
                }
                EntityStatus::LiteralOpenedByCircuitNetwork => {
                    "opened-by-circuit-network"
                }
                EntityStatus::LiteralClosedByCircuitNetwork => {
                    "closed-by-circuit-network"
                }
                EntityStatus::LiteralDisabledByScript => "disabled-by-script",
                EntityStatus::LiteralDisabled => "disabled",
                EntityStatus::LiteralTurnedOffDuringDaytime => {
                    "turned-off-during-daytime"
                }
                EntityStatus::LiteralFluidIngredientShortage => {
                    "fluid-ingredient-shortage"
                }
                EntityStatus::LiteralItemIngredientShortage => "item-ingredient-shortage",
                EntityStatus::LiteralFullOutput => "full-output",
                EntityStatus::LiteralNotEnoughSpaceInOutput => {
                    "not-enough-space-in-output"
                }
                EntityStatus::LiteralFullBurntResultOutput => "full-burnt-result-output",
                EntityStatus::LiteralMarkedForDeconstruction => {
                    "marked-for-deconstruction"
                }
                EntityStatus::LiteralMissingRequiredFluid => "missing-required-fluid",
                EntityStatus::LiteralMissingSciencePacks => "missing-science-packs",
                EntityStatus::LiteralWaitingForSourceItems => "waiting-for-source-items",
                EntityStatus::LiteralWaitingForSpaceInDestination => {
                    "waiting-for-space-in-destination"
                }
                EntityStatus::LiteralPreparingRocketForLaunch => {
                    "preparing-rocket-for-launch"
                }
                EntityStatus::LiteralWaitingToLaunchRocket => "waiting-to-launch-rocket",
                EntityStatus::LiteralWaitingForSpaceInPlatformHub => {
                    "waiting-for-space-in-platform-hub"
                }
                EntityStatus::LiteralLaunchingRocket => "launching-rocket",
                EntityStatus::LiteralThrustNotRequired => "thrust-not-required",
                EntityStatus::LiteralNotEnoughThrust => "not-enough-thrust",
                EntityStatus::LiteralOnTheWay => "on-the-way",
                EntityStatus::LiteralWaitingInOrbit => "waiting-in-orbit",
                EntityStatus::LiteralWaitingForRocketToArrive => {
                    "waiting-for-rocket-to-arrive"
                }
                EntityStatus::LiteralWaitingToClearDropSlots => {
                    "waiting-to-clear-drop-slots"
                }
                EntityStatus::LiteralNoPath => "no-path",
                EntityStatus::LiteralBroken => "broken",
                EntityStatus::LiteralNone => "none",
                EntityStatus::LiteralFrozen => "frozen",
                EntityStatus::LiteralPaused => "paused",
                EntityStatus::LiteralNotConnectedToHubOrPad => {
                    "not-connected-to-hub-or-pad"
                }
                EntityStatus::LiteralTooFarFromPadToUnload => {
                    "too-far-from-pad-to-unload"
                }
                EntityStatus::LiteralComputingNavigation => "computing-navigation",
                EntityStatus::LiteralNoFilter => "no-filter",
                EntityStatus::LiteralWaitingAtStop => "waiting-at-stop",
                EntityStatus::LiteralWaitingForUpgrade => "waiting-for-upgrade",
                EntityStatus::LiteralDestinationStopFull => "destination-stop-full",
                EntityStatus::LiteralPipelineOverextended => "pipeline-overextended",
                EntityStatus::LiteralNoSpotSeedableByInputs => {
                    "no-spot-seedable-by-inputs"
                }
                EntityStatus::LiteralWaitingForPlantsToGrow => {
                    "waiting-for-plants-to-grow"
                }
                EntityStatus::LiteralRecipeNotResearched => "recipe-not-researched",
                EntityStatus::LiteralArmed => "armed",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EntityTransferTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<serde_json::Value>,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct EquipmentCategoryID(pub String);
impl std::fmt::Display for EquipmentCategoryID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for EquipmentCategoryID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct EquipmentGridID(pub String);
impl std::fmt::Display for EquipmentGridID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for EquipmentGridID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct EquipmentID(pub String);
impl std::fmt::Display for EquipmentID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for EquipmentID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EquipmentShape {
    pub height: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<Vec<u32>>>,
    pub r#type: serde_json::Value,
    pub width: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum ExplosionDefinition {
    EntityID(Box<EntityID>),
    Struct1,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum Fade {
    Struct0,
    AttenuationType(Box<AttenuationType>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Fades {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in: Option<Fade>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out: Option<Fade>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FastBeltBendTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FastReplaceTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<EntityID>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FeatureFlags {
    pub expansion: bool,
    pub expansion_shaders: bool,
    pub freezing: bool,
    pub quality: bool,
    pub rail_bridges: bool,
    pub segmented_units: bool,
    pub space_travel: bool,
    pub spoiling: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct FileName(pub String);
impl std::fmt::Display for FileName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for FileName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FlipEntityTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FlowStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_spacing: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_on_row: Option<i32>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_spacing: Option<i32>,
}
pub type FluidAmount = f64;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FluidBox {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_draw_covers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_only_when_connected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_working_visualisations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FluidID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pipeline_extent: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirrored_pipe_picture: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirrored_pipe_picture_frozen: Option<Sprite4Way>,
    pub pipe_connections: Vec<PipeConnectionDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe_covers: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe_covers_frozen: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe_picture: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe_picture_frozen: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_type: Option<ProductionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_draw_order: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_draw_orders: Option<FluidBoxSecondaryDrawOrders>,
    pub volume: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_reservation_fraction: Option<f64>,
}
pub type FluidBoxLinkedConnectionID = u32;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FluidBoxSecondaryDrawOrders {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<i8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FluidEnergySource {
    #[serde(flatten)]
    pub parent: BaseEnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burns_fluid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_non_fuel_fluid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effectivity: Option<f64>,
    pub fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_usage_per_tick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_flicker: Option<LightFlickeringDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_fluid_box: Option<FluidBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_fluid_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke: Option<Vec<SmokeSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_fluid: Option<SpentFluidSpecification>,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum FluidFlowDirection {
    #[serde(rename = "input-output")]
    LiteralInputOutput,
    #[serde(rename = "input")]
    LiteralInput,
    #[serde(rename = "output")]
    LiteralOutput,
}
impl std::fmt::Display for FluidFlowDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                FluidFlowDirection::LiteralInputOutput => "input-output",
                FluidFlowDirection::LiteralInput => "input",
                FluidFlowDirection::LiteralOutput => "output",
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct FluidID(pub String);
impl std::fmt::Display for FluidID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for FluidID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FluidIngredientPrototype {
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluidbox_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluidbox_multiplier: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_by_stats: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_temperature: Option<f64>,
    pub name: FluidID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_fluidbox_indexes: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FluidProductPrototype {
    #[serde(flatten)]
    pub parent: ProductPrototypeBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluidbox_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluidbox_multiplier: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_by_productivity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_by_stats: Option<f64>,
    pub name: FluidID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_fluidbox_indexes: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FogEffectProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color1: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color2: Option<Color>,
    pub detail_noise_texture: EffectTexture,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog_type: Option<serde_json::Value>,
    pub shape_noise_texture: EffectTexture,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_factor: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FogMaskShapeDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub falloff: Option<f64>,
    pub rect: SimpleBoundingBox,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FollowerRobotLifetimeModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FootprintParticle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_name: Option<ParticleID>,
    pub tiles: Vec<TileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_as_default: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FootstepTriggerEffectItem {
    #[serde(flatten)]
    pub parent: CreateParticleTriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<CreateParticleTriggerEffectItem>>,
    pub tiles: Vec<TileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_as_default: Option<bool>,
}
pub type FootstepTriggerEffectList = Vec<FootstepTriggerEffectItem>;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ForceCondition {
    #[serde(rename = "all")]
    LiteralAll,
    #[serde(rename = "enemy")]
    LiteralEnemy,
    #[serde(rename = "ally")]
    LiteralAlly,
    #[serde(rename = "friend")]
    LiteralFriend,
    #[serde(rename = "not-friend")]
    LiteralNotFriend,
    #[serde(rename = "same")]
    LiteralSame,
    #[serde(rename = "not-same")]
    LiteralNotSame,
}
impl std::fmt::Display for ForceCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ForceCondition::LiteralAll => "all",
                ForceCondition::LiteralEnemy => "enemy",
                ForceCondition::LiteralAlly => "ally",
                ForceCondition::LiteralFriend => "friend",
                ForceCondition::LiteralNotFriend => "not-friend",
                ForceCondition::LiteralSame => "same",
                ForceCondition::LiteralNotSame => "not-same",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FrameStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<BorderImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_by_title: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_filler_style: Option<EmptyWidgetStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_flow_style: Option<HorizontalFlowStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_flow_style: Option<HorizontalFlowStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_style: Option<LabelStyleSpecification>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_header_filler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_flow_style: Option<VerticalFlowStyleSpecification>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FrequencySizeRichness {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<MapGenSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub richness: Option<MapGenSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<MapGenSize>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct FuelCategoryID(pub String);
impl std::fmt::Display for FuelCategoryID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for FuelCategoryID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FusionGeneratorDirectionGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_input_graphics: Option<Vec<FusionGeneratorFluidInputGraphics>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fusion_effect_uv_map: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_light: Option<Animation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FusionGeneratorFluidInputGraphics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fusion_effect_uv_map: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_light: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FusionGeneratorGraphicsSet {
    pub east_graphics_set: FusionGeneratorDirectionGraphicsSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    pub north_graphics_set: FusionGeneratorDirectionGraphicsSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    pub south_graphics_set: FusionGeneratorDirectionGraphicsSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
    pub west_graphics_set: FusionGeneratorDirectionGraphicsSet,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FusionReactorConnectionGraphics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fusion_effect_uv_map: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_light_pictures: Option<Animation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FusionReactorGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections_graphics: Option<Vec<FusionReactorConnectionGraphics>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_fuel_glow_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_to_connections_graphics: Option<
        std::collections::HashMap<DirectionString, Vec<u8>>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fusion_effect_uv_map: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    pub plasma_category: NeighbourConnectableConnectionCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_fuel_glow_color: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_light_pictures: Option<Sprite4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GameControllerVibrationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_frequency_vibration_intensity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_frequency_vibration_intensity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_for: Option<PlayFor>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GameViewSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_show_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_tall_entities: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert_gui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_controller_gui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_crafting_queue: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_entity_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_entity_tooltip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_hotkey_suggestions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_map_view_options: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_minimap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_pins_gui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_quickbar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_rail_block_visualisation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_research_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_shortcut_bar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_side_menu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_surface_list: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_tool_bar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_entity_selection: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GateOverRailBuildTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GeneratingPowerTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GeneratorPictureSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<GeneratorPictures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north: Option<GeneratorPictures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<GeneratorPictures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<GeneratorPictures>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GeneratorPictures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GhostShimmerConfig {
    pub blend_mode: i32,
    pub distortion: f64,
    pub distortion_layers: Vec<GhostShimmerDistortionData>,
    pub overlay_layers: Vec<GhostShimmerOverlayData>,
    pub proportional_distortion: bool,
    pub tint: Color,
    pub visualize_borders: bool,
    pub world_uv_modulo: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GhostShimmerDistortionData {
    pub intensity: f64,
    pub shape: i32,
    pub x: f64,
    pub y: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GhostShimmerOverlayData {
    pub blend_mode: i32,
    pub cutoff: f64,
    pub shape: i32,
    pub tint: Color,
    pub x: f64,
    pub y: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GhostTintSet {
    pub ghost_delivery_tint: Color,
    pub ghost_tint: Color,
    pub tile_ghost_delivery_tint: Color,
    pub tile_ghost_tint: Color,
    pub wire_tint: Color,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GigaCargoHatchDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closing_sound: Option<InterruptibleSound>,
    pub covered_hatches: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatch_graphics_back: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatch_graphics_front: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatch_render_layer_back: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatch_render_layer_front: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_sound: Option<InterruptibleSound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GiveItemModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    pub item: ItemID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityID>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GlobalRecipeTints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quaternary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tertiary: Option<Color>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GlobalTintEffectProperties {
    pub global_intensity: f64,
    pub global_scale: f64,
    pub intensity: Vector4f,
    pub noise_texture: EffectTexture,
    pub offset: Vector4f,
    pub scale_u: Vector4f,
    pub scale_v: Vector4f,
    pub zoom_factor: f64,
    pub zoom_intensity: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GlowStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_set: Option<ElementImageSet>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GraphStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_line_highlight_distance: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_right_margin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_top_margin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_lines_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide_lines_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_label_style: Option<LabelStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_labels_margin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_colors: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_horizontal_label_spacing: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_vertical_label_spacing: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_dot_radius: Option<u32>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_label_style: Option<LabelStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_labels_margin: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GroupAttackTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GunShift4Way {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<Vector>,
    pub north: Vector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GunSpeedModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    pub ammo_category: AmmoCategoryID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub modifier: f64,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct HeatBuffer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<HeatConnectionDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_glow: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_picture: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_pipe_covers: Option<Sprite4Way>,
    pub max_temperature: f64,
    pub max_transfer: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_temperature_gradient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_working_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_glow_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe_covers: Option<Sprite4Way>,
    pub specific_heat: Energy,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct HeatConnectionDefinition {
    pub direction: u32,
    pub position: MapPosition,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct HeatEnergySource {
    #[serde(flatten)]
    pub parent: BaseEnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<HeatConnectionDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissions_per_minute: Option<
        std::collections::HashMap<AirbornePollutantID, f64>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_glow: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_picture: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_pipe_covers: Option<Sprite4Way>,
    pub max_temperature: f64,
    pub max_transfer: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_temperature_gradient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_working_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_glow_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe_covers: Option<Sprite4Way>,
    pub specific_heat: Energy,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum HorizontalAlign {
    #[serde(rename = "left")]
    LiteralLeft,
    #[serde(rename = "center")]
    LiteralCenter,
    #[serde(rename = "right")]
    LiteralRight,
}
impl std::fmt::Display for HorizontalAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                HorizontalAlign::LiteralLeft => "left",
                HorizontalAlign::LiteralCenter => "center",
                HorizontalAlign::LiteralRight => "right",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct HorizontalFlowStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_spacing: Option<i32>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct HorizontalScrollBarStyleSpecification {
    #[serde(flatten)]
    pub parent: ScrollBarStyleSpecification,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct IconData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floating: Option<bool>,
    pub icon: FileName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<Color>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct IconDrawSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_for_many: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct IconSequencePositioning {
    pub inventory_index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_icon_rows: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_icons_per_row: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_row_initial_height_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separation_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ImageStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_colors_of_picture_when_hovered_or_toggled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stretch_image_to_widget_size: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum IngredientPrototype {
    #[serde(rename = "item")]
    ItemIngredientPrototype(ItemIngredientPrototype),
    #[serde(rename = "fluid")]
    FluidIngredientPrototype(FluidIngredientPrototype),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InsertItemTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    pub item: ItemID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityID>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InserterStackSizeBonusModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InstantTriggerDelivery {
    #[serde(flatten)]
    pub parent: TriggerDeliveryItem,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InterruptibleSound {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_ticks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_change_per_tick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_sound_duration_for_stopped_sound: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InventoryWithCustomStackSizeSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size_max: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size_min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size_override: Option<std::collections::HashMap<ItemID, u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_bar: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InvokeTileEffectTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_collision_mask: Option<TileCollisionMaskConnector>,
    pub r#type: String,
}
pub type ItemCountType = u32;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct ItemGroupID(pub String);
impl std::fmt::Display for ItemGroupID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for ItemGroupID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemHealthColorData {
    pub color: Color,
    pub threshold: f64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct ItemID(pub String);
impl std::fmt::Display for ItemID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for ItemID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum ItemIDFilter {
    Struct0,
    ItemID(Box<ItemID>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemIngredientPrototype {
    pub amount: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_by_stats: Option<u16>,
    pub name: ItemID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_change: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_max: Option<QualityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_min: Option<QualityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoil_weight: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemProductPrototype {
    #[serde(flatten)]
    pub parent: ProductPrototypeBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_by_quality: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_fresh: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_max: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_min: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_count_fraction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_by_productivity: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_by_stats: Option<u16>,
    pub name: ItemID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_spoiled: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_change: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_max: Option<QualityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_min: Option<QualityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_freshness_on_craft: Option<bool>,
    pub r#type: String,
}
pub type ItemPrototypeFlags = Vec<serde_json::Value>;
pub type ItemStackIndex = u16;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct ItemSubGroupID(pub String);
impl std::fmt::Display for ItemSubGroupID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for ItemSubGroupID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemToPlace {
    pub count: u32,
    pub item: ItemID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct KillTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_type: Option<DamageTypeID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type_only: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LabelStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_controller_hovered_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_hovered_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text_highlight_error_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text_highlight_ok_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text_highlight_warning_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text_setting: Option<RichTextSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_line: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LaboratoryProductivityModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LaboratorySpeedModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum LayeredSound {
    Struct0,
    Sound(Box<Sound>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum LayeredSprite {
    Struct0,
    Array1(Vec<LayeredSprite>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum LayeredSprite4Way {
    Struct0,
    LayeredSprite(Box<LayeredSprite>),
}
pub type LayeredSpriteVariations = Vec<LayeredSprite>;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum LightDefinition {
    Struct0,
    Array1(Vec<serde_json::Value>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LightFlickeringDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_fix_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivation_change_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivation_change_frequency: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_intensity_to_size_coefficient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_intensity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_intensity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_light_size: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LightProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Vector3D>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LightningGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attractor_hit_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bolt_detail_level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bolt_half_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bolt_midpoint_variance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_background: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_detail_level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_fork_orientation_variance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_forks: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosion: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork_intensity_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork_orientation_variance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_streamer_variance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_streamers: Option<Vec<Animation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bolt_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fork_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ground_streamer_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_relative_fork_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ground_streamer_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_relative_fork_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_cloud_fork_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shader_configuration: Option<Vec<LightningShaderConfiguration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LightningPriorityRule {
    #[serde(flatten)]
    pub parent: LightningRuleBase,
    pub priority_bonus: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LightningProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_rules: Option<Vec<LightningRuleBase>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightning_multiplier_at_day: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightning_multiplier_at_night: Option<f64>,
    pub lightning_types: Vec<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightning_warning_icon: Option<Sprite>,
    pub lightnings_per_chunk_per_tick: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier_surface_property: Option<SurfacePropertyID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_rules: Option<Vec<LightningPriorityRule>>,
    pub search_radius: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LightningRuleBase {
    pub string: String,
    pub r#type: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LightningShaderConfiguration {
    pub color: Color,
    pub distortion: f64,
    pub power: f64,
    pub thickness: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LimitChestTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LineStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<BorderImageSet>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LineTriggerItem {
    #[serde(flatten)]
    pub parent: TriggerItem,
    pub range: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_effects: Option<TriggerEffect>,
    pub r#type: String,
    pub width: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LinkedBeltStructure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_in: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_in_side_loading: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_out: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_out_side_loading: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_patch: Option<Sprite4Way>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum LinkedGameControl {
    #[serde(rename = "move-up")]
    LiteralMoveUp,
    #[serde(rename = "move-down")]
    LiteralMoveDown,
    #[serde(rename = "move-left")]
    LiteralMoveLeft,
    #[serde(rename = "move-right")]
    LiteralMoveRight,
    #[serde(rename = "alternative-gui-move-up")]
    LiteralAlternativeGuiMoveUp,
    #[serde(rename = "alternative-gui-move-down")]
    LiteralAlternativeGuiMoveDown,
    #[serde(rename = "alternative-gui-move-left")]
    LiteralAlternativeGuiMoveLeft,
    #[serde(rename = "alternative-gui-move-right")]
    LiteralAlternativeGuiMoveRight,
    #[serde(rename = "open-character-gui")]
    LiteralOpenCharacterGui,
    #[serde(rename = "open-gui")]
    LiteralOpenGui,
    #[serde(rename = "confirm-gui")]
    LiteralConfirmGui,
    #[serde(rename = "toggle-free-cursor")]
    LiteralToggleFreeCursor,
    #[serde(rename = "mine")]
    LiteralMine,
    #[serde(rename = "build")]
    LiteralBuild,
    #[serde(rename = "build-ghost")]
    LiteralBuildGhost,
    #[serde(rename = "super-forced-build")]
    LiteralSuperForcedBuild,
    #[serde(rename = "clear-cursor")]
    LiteralClearCursor,
    #[serde(rename = "pipette")]
    LiteralPipette,
    #[serde(rename = "rotate")]
    LiteralRotate,
    #[serde(rename = "reverse-rotate")]
    LiteralReverseRotate,
    #[serde(rename = "flip-horizontal")]
    LiteralFlipHorizontal,
    #[serde(rename = "flip-vertical")]
    LiteralFlipVertical,
    #[serde(rename = "pick-items")]
    LiteralPickItems,
    #[serde(rename = "drop-cursor")]
    LiteralDropCursor,
    #[serde(rename = "show-info")]
    LiteralShowInfo,
    #[serde(rename = "hide-tall-entities")]
    LiteralHideTallEntities,
    #[serde(rename = "shoot-enemy")]
    LiteralShootEnemy,
    #[serde(rename = "shoot-selected")]
    LiteralShootSelected,
    #[serde(rename = "next-weapon")]
    LiteralNextWeapon,
    #[serde(rename = "toggle-driving")]
    LiteralToggleDriving,
    #[serde(rename = "zoom-in")]
    LiteralZoomIn,
    #[serde(rename = "zoom-out")]
    LiteralZoomOut,
    #[serde(rename = "use-item")]
    LiteralUseItem,
    #[serde(rename = "alternative-use-item")]
    LiteralAlternativeUseItem,
    #[serde(rename = "toggle-console")]
    LiteralToggleConsole,
    #[serde(rename = "copy-entity-settings")]
    LiteralCopyEntitySettings,
    #[serde(rename = "paste-entity-settings")]
    LiteralPasteEntitySettings,
    #[serde(rename = "controller-gui-logistics-tab")]
    LiteralControllerGuiLogisticsTab,
    #[serde(rename = "controller-gui-character-tab")]
    LiteralControllerGuiCharacterTab,
    #[serde(rename = "controller-gui-crafting-tab")]
    LiteralControllerGuiCraftingTab,
    #[serde(rename = "toggle-rail-layer")]
    LiteralToggleRailLayer,
    #[serde(rename = "select-for-blueprint")]
    LiteralSelectForBlueprint,
    #[serde(rename = "select-for-cancel-deconstruct")]
    LiteralSelectForCancelDeconstruct,
    #[serde(rename = "select-for-super-forced-deconstruct")]
    LiteralSelectForSuperForcedDeconstruct,
    #[serde(rename = "reverse-select")]
    LiteralReverseSelect,
    #[serde(rename = "alt-reverse-select")]
    LiteralAltReverseSelect,
    #[serde(rename = "deselect")]
    LiteralDeselect,
    #[serde(rename = "cycle-blueprint-forwards")]
    LiteralCycleBlueprintForwards,
    #[serde(rename = "cycle-blueprint-backwards")]
    LiteralCycleBlueprintBackwards,
    #[serde(rename = "focus-search")]
    LiteralFocusSearch,
    #[serde(rename = "larger-terrain-building-area")]
    LiteralLargerTerrainBuildingArea,
    #[serde(rename = "smaller-terrain-building-area")]
    LiteralSmallerTerrainBuildingArea,
    #[serde(rename = "remove-pole-cables")]
    LiteralRemovePoleCables,
    #[serde(rename = "build-with-obstacle-avoidance")]
    LiteralBuildWithObstacleAvoidance,
    #[serde(rename = "add-station")]
    LiteralAddStation,
    #[serde(rename = "add-temporary-station")]
    LiteralAddTemporaryStation,
    #[serde(rename = "rename-all")]
    LiteralRenameAll,
    #[serde(rename = "fast-wait-condition")]
    LiteralFastWaitCondition,
    #[serde(rename = "drag-map")]
    LiteralDragMap,
    #[serde(rename = "move-tag")]
    LiteralMoveTag,
    #[serde(rename = "place-in-chat")]
    LiteralPlaceInChat,
    #[serde(rename = "place-ping")]
    LiteralPlacePing,
    #[serde(rename = "pin")]
    LiteralPin,
    #[serde(rename = "activate-tooltip")]
    LiteralActivateTooltip,
    #[serde(rename = "next-surface")]
    LiteralNextSurface,
    #[serde(rename = "previous-surface")]
    LiteralPreviousSurface,
    #[serde(rename = "cycle-quality-up")]
    LiteralCycleQualityUp,
    #[serde(rename = "cycle-quality-down")]
    LiteralCycleQualityDown,
    #[serde(rename = "scroll-tooltip-up")]
    LiteralScrollTooltipUp,
    #[serde(rename = "scroll-tooltip-down")]
    LiteralScrollTooltipDown,
    #[serde(rename = "visualize-entity-fluid-segments")]
    LiteralVisualizeEntityFluidSegments,
    #[serde(rename = "craft")]
    LiteralCraft,
    #[serde(rename = "craft-5")]
    LiteralCraft5,
    #[serde(rename = "craft-all")]
    LiteralCraftAll,
    #[serde(rename = "cancel-craft")]
    LiteralCancelCraft,
    #[serde(rename = "cancel-craft-5")]
    LiteralCancelCraft5,
    #[serde(rename = "cancel-craft-all")]
    LiteralCancelCraftAll,
    #[serde(rename = "pick-item")]
    LiteralPickItem,
    #[serde(rename = "stack-transfer")]
    LiteralStackTransfer,
    #[serde(rename = "inventory-transfer")]
    LiteralInventoryTransfer,
    #[serde(rename = "fast-entity-transfer")]
    LiteralFastEntityTransfer,
    #[serde(rename = "cursor-split")]
    LiteralCursorSplit,
    #[serde(rename = "stack-split")]
    LiteralStackSplit,
    #[serde(rename = "inventory-split")]
    LiteralInventorySplit,
    #[serde(rename = "fast-entity-split")]
    LiteralFastEntitySplit,
    #[serde(rename = "toggle-filter")]
    LiteralToggleFilter,
    #[serde(rename = "open-item")]
    LiteralOpenItem,
    #[serde(rename = "copy-inventory-filter")]
    LiteralCopyInventoryFilter,
    #[serde(rename = "paste-inventory-filter")]
    LiteralPasteInventoryFilter,
    #[serde(rename = "show-quick-panel")]
    LiteralShowQuickPanel,
    #[serde(rename = "next-quick-panel-page")]
    LiteralNextQuickPanelPage,
    #[serde(rename = "previous-quick-panel-page")]
    LiteralPreviousQuickPanelPage,
    #[serde(rename = "next-quick-panel-tab")]
    LiteralNextQuickPanelTab,
    #[serde(rename = "previous-quick-panel-tab")]
    LiteralPreviousQuickPanelTab,
    #[serde(rename = "rotate-active-quick-bars")]
    LiteralRotateActiveQuickBars,
    #[serde(rename = "next-active-quick-bar")]
    LiteralNextActiveQuickBar,
    #[serde(rename = "previous-active-quick-bar")]
    LiteralPreviousActiveQuickBar,
    #[serde(rename = "quick-bar-button-1")]
    LiteralQuickBarButton1,
    #[serde(rename = "quick-bar-button-2")]
    LiteralQuickBarButton2,
    #[serde(rename = "quick-bar-button-3")]
    LiteralQuickBarButton3,
    #[serde(rename = "quick-bar-button-4")]
    LiteralQuickBarButton4,
    #[serde(rename = "quick-bar-button-5")]
    LiteralQuickBarButton5,
    #[serde(rename = "quick-bar-button-6")]
    LiteralQuickBarButton6,
    #[serde(rename = "quick-bar-button-7")]
    LiteralQuickBarButton7,
    #[serde(rename = "quick-bar-button-8")]
    LiteralQuickBarButton8,
    #[serde(rename = "quick-bar-button-9")]
    LiteralQuickBarButton9,
    #[serde(rename = "quick-bar-button-10")]
    LiteralQuickBarButton10,
    #[serde(rename = "quick-bar-button-1-secondary")]
    LiteralQuickBarButton1Secondary,
    #[serde(rename = "quick-bar-button-2-secondary")]
    LiteralQuickBarButton2Secondary,
    #[serde(rename = "quick-bar-button-3-secondary")]
    LiteralQuickBarButton3Secondary,
    #[serde(rename = "quick-bar-button-4-secondary")]
    LiteralQuickBarButton4Secondary,
    #[serde(rename = "quick-bar-button-5-secondary")]
    LiteralQuickBarButton5Secondary,
    #[serde(rename = "quick-bar-button-6-secondary")]
    LiteralQuickBarButton6Secondary,
    #[serde(rename = "quick-bar-button-7-secondary")]
    LiteralQuickBarButton7Secondary,
    #[serde(rename = "quick-bar-button-8-secondary")]
    LiteralQuickBarButton8Secondary,
    #[serde(rename = "quick-bar-button-9-secondary")]
    LiteralQuickBarButton9Secondary,
    #[serde(rename = "quick-bar-button-10-secondary")]
    LiteralQuickBarButton10Secondary,
    #[serde(rename = "action-bar-select-page-1")]
    LiteralActionBarSelectPage1,
    #[serde(rename = "action-bar-select-page-2")]
    LiteralActionBarSelectPage2,
    #[serde(rename = "action-bar-select-page-3")]
    LiteralActionBarSelectPage3,
    #[serde(rename = "action-bar-select-page-4")]
    LiteralActionBarSelectPage4,
    #[serde(rename = "action-bar-select-page-5")]
    LiteralActionBarSelectPage5,
    #[serde(rename = "action-bar-select-page-6")]
    LiteralActionBarSelectPage6,
    #[serde(rename = "action-bar-select-page-7")]
    LiteralActionBarSelectPage7,
    #[serde(rename = "action-bar-select-page-8")]
    LiteralActionBarSelectPage8,
    #[serde(rename = "action-bar-select-page-9")]
    LiteralActionBarSelectPage9,
    #[serde(rename = "action-bar-select-page-10")]
    LiteralActionBarSelectPage10,
    #[serde(rename = "copy")]
    LiteralCopy,
    #[serde(rename = "cut")]
    LiteralCut,
    #[serde(rename = "paste")]
    LiteralPaste,
    #[serde(rename = "cycle-clipboard-forwards")]
    LiteralCycleClipboardForwards,
    #[serde(rename = "cycle-clipboard-backwards")]
    LiteralCycleClipboardBackwards,
    #[serde(rename = "undo")]
    LiteralUndo,
    #[serde(rename = "redo")]
    LiteralRedo,
    #[serde(rename = "toggle-menu")]
    LiteralToggleMenu,
    #[serde(rename = "toggle-map")]
    LiteralToggleMap,
    #[serde(rename = "close-menu")]
    LiteralCloseMenu,
    #[serde(rename = "open-technology-gui")]
    LiteralOpenTechnologyGui,
    #[serde(rename = "production-statistics")]
    LiteralProductionStatistics,
    #[serde(rename = "logistic-networks")]
    LiteralLogisticNetworks,
    #[serde(rename = "toggle-blueprint-library")]
    LiteralToggleBlueprintLibrary,
    #[serde(rename = "open-trains-gui")]
    LiteralOpenTrainsGui,
    #[serde(rename = "open-factoriopedia")]
    LiteralOpenFactoriopedia,
    #[serde(rename = "back")]
    LiteralBack,
    #[serde(rename = "forward")]
    LiteralForward,
    #[serde(rename = "pause-game")]
    LiteralPauseGame,
    #[serde(rename = "confirm-message")]
    LiteralConfirmMessage,
    #[serde(rename = "previous-mod")]
    LiteralPreviousMod,
    #[serde(rename = "connect-train")]
    LiteralConnectTrain,
    #[serde(rename = "disconnect-train")]
    LiteralDisconnectTrain,
    #[serde(rename = "submit-feedback")]
    LiteralSubmitFeedback,
    #[serde(rename = "editor-next-variation")]
    LiteralEditorNextVariation,
    #[serde(rename = "editor-previous-variation")]
    LiteralEditorPreviousVariation,
    #[serde(rename = "editor-clone-item")]
    LiteralEditorCloneItem,
    #[serde(rename = "editor-delete-item")]
    LiteralEditorDeleteItem,
    #[serde(rename = "editor-toggle-pause")]
    LiteralEditorTogglePause,
    #[serde(rename = "editor-tick-once")]
    LiteralEditorTickOnce,
    #[serde(rename = "editor-speed-up")]
    LiteralEditorSpeedUp,
    #[serde(rename = "editor-speed-down")]
    LiteralEditorSpeedDown,
    #[serde(rename = "editor-reset-speed")]
    LiteralEditorResetSpeed,
    #[serde(rename = "editor-set-clone-brush-source")]
    LiteralEditorSetCloneBrushSource,
    #[serde(rename = "editor-set-clone-brush-destination")]
    LiteralEditorSetCloneBrushDestination,
    #[serde(rename = "editor-switch-to-surface")]
    LiteralEditorSwitchToSurface,
    #[serde(rename = "editor-remove-scripting-object")]
    LiteralEditorRemoveScriptingObject,
    #[serde(rename = "debug-toggle-atlas-gui")]
    LiteralDebugToggleAtlasGui,
    #[serde(rename = "debug-toggle-gui-visibility")]
    LiteralDebugToggleGuiVisibility,
    #[serde(rename = "debug-toggle-debug-settings")]
    LiteralDebugToggleDebugSettings,
    #[serde(rename = "debug-toggle-basic")]
    LiteralDebugToggleBasic,
    #[serde(rename = "debug-reset-zoom")]
    LiteralDebugResetZoom,
    #[serde(rename = "debug-reset-zoom-2x")]
    LiteralDebugResetZoom2x,
    #[serde(rename = "toggle-gui-debug")]
    LiteralToggleGuiDebug,
    #[serde(rename = "toggle-gui-style-view")]
    LiteralToggleGuiStyleView,
    #[serde(rename = "toggle-gui-shadows")]
    LiteralToggleGuiShadows,
    #[serde(rename = "toggle-gui-glows")]
    LiteralToggleGuiGlows,
    #[serde(rename = "open-prototypes-gui")]
    LiteralOpenPrototypesGui,
    #[serde(rename = "open-prototype-explorer-gui")]
    LiteralOpenPrototypeExplorerGui,
    #[serde(rename = "increase-ui-scale")]
    LiteralIncreaseUiScale,
    #[serde(rename = "decrease-ui-scale")]
    LiteralDecreaseUiScale,
    #[serde(rename = "reset-ui-scale")]
    LiteralResetUiScale,
    #[serde(rename = "slash-editor")]
    LiteralSlashEditor,
    #[serde(rename = "toggle-entity")]
    LiteralToggleEntity,
    #[serde(rename = "next-player-in-replay")]
    LiteralNextPlayerInReplay,
    #[serde(rename = "move-blueprint-absolute-grid-up")]
    LiteralMoveBlueprintAbsoluteGridUp,
    #[serde(rename = "move-blueprint-absolute-grid-down")]
    LiteralMoveBlueprintAbsoluteGridDown,
    #[serde(rename = "move-blueprint-absolute-grid-left")]
    LiteralMoveBlueprintAbsoluteGridLeft,
    #[serde(rename = "move-blueprint-absolute-grid-right")]
    LiteralMoveBlueprintAbsoluteGridRight,
    #[serde(rename = "move-blueprint-entities-up")]
    LiteralMoveBlueprintEntitiesUp,
    #[serde(rename = "move-blueprint-entities-down")]
    LiteralMoveBlueprintEntitiesDown,
    #[serde(rename = "move-blueprint-entities-left")]
    LiteralMoveBlueprintEntitiesLeft,
    #[serde(rename = "move-blueprint-entities-right")]
    LiteralMoveBlueprintEntitiesRight,
    #[serde(rename = "toggle-blueprint-snap-to-grid")]
    LiteralToggleBlueprintSnapToGrid,
    #[serde(rename = "play-next-track")]
    LiteralPlayNextTrack,
    #[serde(rename = "play-previous-track")]
    LiteralPlayPreviousTrack,
    #[serde(rename = "pause-resume-music")]
    LiteralPauseResumeMusic,
    #[serde(rename = "")]
    LiteralEmpty,
}
impl std::fmt::Display for LinkedGameControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                LinkedGameControl::LiteralMoveUp => "move-up",
                LinkedGameControl::LiteralMoveDown => "move-down",
                LinkedGameControl::LiteralMoveLeft => "move-left",
                LinkedGameControl::LiteralMoveRight => "move-right",
                LinkedGameControl::LiteralAlternativeGuiMoveUp => {
                    "alternative-gui-move-up"
                }
                LinkedGameControl::LiteralAlternativeGuiMoveDown => {
                    "alternative-gui-move-down"
                }
                LinkedGameControl::LiteralAlternativeGuiMoveLeft => {
                    "alternative-gui-move-left"
                }
                LinkedGameControl::LiteralAlternativeGuiMoveRight => {
                    "alternative-gui-move-right"
                }
                LinkedGameControl::LiteralOpenCharacterGui => "open-character-gui",
                LinkedGameControl::LiteralOpenGui => "open-gui",
                LinkedGameControl::LiteralConfirmGui => "confirm-gui",
                LinkedGameControl::LiteralToggleFreeCursor => "toggle-free-cursor",
                LinkedGameControl::LiteralMine => "mine",
                LinkedGameControl::LiteralBuild => "build",
                LinkedGameControl::LiteralBuildGhost => "build-ghost",
                LinkedGameControl::LiteralSuperForcedBuild => "super-forced-build",
                LinkedGameControl::LiteralClearCursor => "clear-cursor",
                LinkedGameControl::LiteralPipette => "pipette",
                LinkedGameControl::LiteralRotate => "rotate",
                LinkedGameControl::LiteralReverseRotate => "reverse-rotate",
                LinkedGameControl::LiteralFlipHorizontal => "flip-horizontal",
                LinkedGameControl::LiteralFlipVertical => "flip-vertical",
                LinkedGameControl::LiteralPickItems => "pick-items",
                LinkedGameControl::LiteralDropCursor => "drop-cursor",
                LinkedGameControl::LiteralShowInfo => "show-info",
                LinkedGameControl::LiteralHideTallEntities => "hide-tall-entities",
                LinkedGameControl::LiteralShootEnemy => "shoot-enemy",
                LinkedGameControl::LiteralShootSelected => "shoot-selected",
                LinkedGameControl::LiteralNextWeapon => "next-weapon",
                LinkedGameControl::LiteralToggleDriving => "toggle-driving",
                LinkedGameControl::LiteralZoomIn => "zoom-in",
                LinkedGameControl::LiteralZoomOut => "zoom-out",
                LinkedGameControl::LiteralUseItem => "use-item",
                LinkedGameControl::LiteralAlternativeUseItem => "alternative-use-item",
                LinkedGameControl::LiteralToggleConsole => "toggle-console",
                LinkedGameControl::LiteralCopyEntitySettings => "copy-entity-settings",
                LinkedGameControl::LiteralPasteEntitySettings => "paste-entity-settings",
                LinkedGameControl::LiteralControllerGuiLogisticsTab => {
                    "controller-gui-logistics-tab"
                }
                LinkedGameControl::LiteralControllerGuiCharacterTab => {
                    "controller-gui-character-tab"
                }
                LinkedGameControl::LiteralControllerGuiCraftingTab => {
                    "controller-gui-crafting-tab"
                }
                LinkedGameControl::LiteralToggleRailLayer => "toggle-rail-layer",
                LinkedGameControl::LiteralSelectForBlueprint => "select-for-blueprint",
                LinkedGameControl::LiteralSelectForCancelDeconstruct => {
                    "select-for-cancel-deconstruct"
                }
                LinkedGameControl::LiteralSelectForSuperForcedDeconstruct => {
                    "select-for-super-forced-deconstruct"
                }
                LinkedGameControl::LiteralReverseSelect => "reverse-select",
                LinkedGameControl::LiteralAltReverseSelect => "alt-reverse-select",
                LinkedGameControl::LiteralDeselect => "deselect",
                LinkedGameControl::LiteralCycleBlueprintForwards => {
                    "cycle-blueprint-forwards"
                }
                LinkedGameControl::LiteralCycleBlueprintBackwards => {
                    "cycle-blueprint-backwards"
                }
                LinkedGameControl::LiteralFocusSearch => "focus-search",
                LinkedGameControl::LiteralLargerTerrainBuildingArea => {
                    "larger-terrain-building-area"
                }
                LinkedGameControl::LiteralSmallerTerrainBuildingArea => {
                    "smaller-terrain-building-area"
                }
                LinkedGameControl::LiteralRemovePoleCables => "remove-pole-cables",
                LinkedGameControl::LiteralBuildWithObstacleAvoidance => {
                    "build-with-obstacle-avoidance"
                }
                LinkedGameControl::LiteralAddStation => "add-station",
                LinkedGameControl::LiteralAddTemporaryStation => "add-temporary-station",
                LinkedGameControl::LiteralRenameAll => "rename-all",
                LinkedGameControl::LiteralFastWaitCondition => "fast-wait-condition",
                LinkedGameControl::LiteralDragMap => "drag-map",
                LinkedGameControl::LiteralMoveTag => "move-tag",
                LinkedGameControl::LiteralPlaceInChat => "place-in-chat",
                LinkedGameControl::LiteralPlacePing => "place-ping",
                LinkedGameControl::LiteralPin => "pin",
                LinkedGameControl::LiteralActivateTooltip => "activate-tooltip",
                LinkedGameControl::LiteralNextSurface => "next-surface",
                LinkedGameControl::LiteralPreviousSurface => "previous-surface",
                LinkedGameControl::LiteralCycleQualityUp => "cycle-quality-up",
                LinkedGameControl::LiteralCycleQualityDown => "cycle-quality-down",
                LinkedGameControl::LiteralScrollTooltipUp => "scroll-tooltip-up",
                LinkedGameControl::LiteralScrollTooltipDown => "scroll-tooltip-down",
                LinkedGameControl::LiteralVisualizeEntityFluidSegments => {
                    "visualize-entity-fluid-segments"
                }
                LinkedGameControl::LiteralCraft => "craft",
                LinkedGameControl::LiteralCraft5 => "craft-5",
                LinkedGameControl::LiteralCraftAll => "craft-all",
                LinkedGameControl::LiteralCancelCraft => "cancel-craft",
                LinkedGameControl::LiteralCancelCraft5 => "cancel-craft-5",
                LinkedGameControl::LiteralCancelCraftAll => "cancel-craft-all",
                LinkedGameControl::LiteralPickItem => "pick-item",
                LinkedGameControl::LiteralStackTransfer => "stack-transfer",
                LinkedGameControl::LiteralInventoryTransfer => "inventory-transfer",
                LinkedGameControl::LiteralFastEntityTransfer => "fast-entity-transfer",
                LinkedGameControl::LiteralCursorSplit => "cursor-split",
                LinkedGameControl::LiteralStackSplit => "stack-split",
                LinkedGameControl::LiteralInventorySplit => "inventory-split",
                LinkedGameControl::LiteralFastEntitySplit => "fast-entity-split",
                LinkedGameControl::LiteralToggleFilter => "toggle-filter",
                LinkedGameControl::LiteralOpenItem => "open-item",
                LinkedGameControl::LiteralCopyInventoryFilter => "copy-inventory-filter",
                LinkedGameControl::LiteralPasteInventoryFilter => {
                    "paste-inventory-filter"
                }
                LinkedGameControl::LiteralShowQuickPanel => "show-quick-panel",
                LinkedGameControl::LiteralNextQuickPanelPage => "next-quick-panel-page",
                LinkedGameControl::LiteralPreviousQuickPanelPage => {
                    "previous-quick-panel-page"
                }
                LinkedGameControl::LiteralNextQuickPanelTab => "next-quick-panel-tab",
                LinkedGameControl::LiteralPreviousQuickPanelTab => {
                    "previous-quick-panel-tab"
                }
                LinkedGameControl::LiteralRotateActiveQuickBars => {
                    "rotate-active-quick-bars"
                }
                LinkedGameControl::LiteralNextActiveQuickBar => "next-active-quick-bar",
                LinkedGameControl::LiteralPreviousActiveQuickBar => {
                    "previous-active-quick-bar"
                }
                LinkedGameControl::LiteralQuickBarButton1 => "quick-bar-button-1",
                LinkedGameControl::LiteralQuickBarButton2 => "quick-bar-button-2",
                LinkedGameControl::LiteralQuickBarButton3 => "quick-bar-button-3",
                LinkedGameControl::LiteralQuickBarButton4 => "quick-bar-button-4",
                LinkedGameControl::LiteralQuickBarButton5 => "quick-bar-button-5",
                LinkedGameControl::LiteralQuickBarButton6 => "quick-bar-button-6",
                LinkedGameControl::LiteralQuickBarButton7 => "quick-bar-button-7",
                LinkedGameControl::LiteralQuickBarButton8 => "quick-bar-button-8",
                LinkedGameControl::LiteralQuickBarButton9 => "quick-bar-button-9",
                LinkedGameControl::LiteralQuickBarButton10 => "quick-bar-button-10",
                LinkedGameControl::LiteralQuickBarButton1Secondary => {
                    "quick-bar-button-1-secondary"
                }
                LinkedGameControl::LiteralQuickBarButton2Secondary => {
                    "quick-bar-button-2-secondary"
                }
                LinkedGameControl::LiteralQuickBarButton3Secondary => {
                    "quick-bar-button-3-secondary"
                }
                LinkedGameControl::LiteralQuickBarButton4Secondary => {
                    "quick-bar-button-4-secondary"
                }
                LinkedGameControl::LiteralQuickBarButton5Secondary => {
                    "quick-bar-button-5-secondary"
                }
                LinkedGameControl::LiteralQuickBarButton6Secondary => {
                    "quick-bar-button-6-secondary"
                }
                LinkedGameControl::LiteralQuickBarButton7Secondary => {
                    "quick-bar-button-7-secondary"
                }
                LinkedGameControl::LiteralQuickBarButton8Secondary => {
                    "quick-bar-button-8-secondary"
                }
                LinkedGameControl::LiteralQuickBarButton9Secondary => {
                    "quick-bar-button-9-secondary"
                }
                LinkedGameControl::LiteralQuickBarButton10Secondary => {
                    "quick-bar-button-10-secondary"
                }
                LinkedGameControl::LiteralActionBarSelectPage1 => {
                    "action-bar-select-page-1"
                }
                LinkedGameControl::LiteralActionBarSelectPage2 => {
                    "action-bar-select-page-2"
                }
                LinkedGameControl::LiteralActionBarSelectPage3 => {
                    "action-bar-select-page-3"
                }
                LinkedGameControl::LiteralActionBarSelectPage4 => {
                    "action-bar-select-page-4"
                }
                LinkedGameControl::LiteralActionBarSelectPage5 => {
                    "action-bar-select-page-5"
                }
                LinkedGameControl::LiteralActionBarSelectPage6 => {
                    "action-bar-select-page-6"
                }
                LinkedGameControl::LiteralActionBarSelectPage7 => {
                    "action-bar-select-page-7"
                }
                LinkedGameControl::LiteralActionBarSelectPage8 => {
                    "action-bar-select-page-8"
                }
                LinkedGameControl::LiteralActionBarSelectPage9 => {
                    "action-bar-select-page-9"
                }
                LinkedGameControl::LiteralActionBarSelectPage10 => {
                    "action-bar-select-page-10"
                }
                LinkedGameControl::LiteralCopy => "copy",
                LinkedGameControl::LiteralCut => "cut",
                LinkedGameControl::LiteralPaste => "paste",
                LinkedGameControl::LiteralCycleClipboardForwards => {
                    "cycle-clipboard-forwards"
                }
                LinkedGameControl::LiteralCycleClipboardBackwards => {
                    "cycle-clipboard-backwards"
                }
                LinkedGameControl::LiteralUndo => "undo",
                LinkedGameControl::LiteralRedo => "redo",
                LinkedGameControl::LiteralToggleMenu => "toggle-menu",
                LinkedGameControl::LiteralToggleMap => "toggle-map",
                LinkedGameControl::LiteralCloseMenu => "close-menu",
                LinkedGameControl::LiteralOpenTechnologyGui => "open-technology-gui",
                LinkedGameControl::LiteralProductionStatistics => "production-statistics",
                LinkedGameControl::LiteralLogisticNetworks => "logistic-networks",
                LinkedGameControl::LiteralToggleBlueprintLibrary => {
                    "toggle-blueprint-library"
                }
                LinkedGameControl::LiteralOpenTrainsGui => "open-trains-gui",
                LinkedGameControl::LiteralOpenFactoriopedia => "open-factoriopedia",
                LinkedGameControl::LiteralBack => "back",
                LinkedGameControl::LiteralForward => "forward",
                LinkedGameControl::LiteralPauseGame => "pause-game",
                LinkedGameControl::LiteralConfirmMessage => "confirm-message",
                LinkedGameControl::LiteralPreviousMod => "previous-mod",
                LinkedGameControl::LiteralConnectTrain => "connect-train",
                LinkedGameControl::LiteralDisconnectTrain => "disconnect-train",
                LinkedGameControl::LiteralSubmitFeedback => "submit-feedback",
                LinkedGameControl::LiteralEditorNextVariation => "editor-next-variation",
                LinkedGameControl::LiteralEditorPreviousVariation => {
                    "editor-previous-variation"
                }
                LinkedGameControl::LiteralEditorCloneItem => "editor-clone-item",
                LinkedGameControl::LiteralEditorDeleteItem => "editor-delete-item",
                LinkedGameControl::LiteralEditorTogglePause => "editor-toggle-pause",
                LinkedGameControl::LiteralEditorTickOnce => "editor-tick-once",
                LinkedGameControl::LiteralEditorSpeedUp => "editor-speed-up",
                LinkedGameControl::LiteralEditorSpeedDown => "editor-speed-down",
                LinkedGameControl::LiteralEditorResetSpeed => "editor-reset-speed",
                LinkedGameControl::LiteralEditorSetCloneBrushSource => {
                    "editor-set-clone-brush-source"
                }
                LinkedGameControl::LiteralEditorSetCloneBrushDestination => {
                    "editor-set-clone-brush-destination"
                }
                LinkedGameControl::LiteralEditorSwitchToSurface => {
                    "editor-switch-to-surface"
                }
                LinkedGameControl::LiteralEditorRemoveScriptingObject => {
                    "editor-remove-scripting-object"
                }
                LinkedGameControl::LiteralDebugToggleAtlasGui => "debug-toggle-atlas-gui",
                LinkedGameControl::LiteralDebugToggleGuiVisibility => {
                    "debug-toggle-gui-visibility"
                }
                LinkedGameControl::LiteralDebugToggleDebugSettings => {
                    "debug-toggle-debug-settings"
                }
                LinkedGameControl::LiteralDebugToggleBasic => "debug-toggle-basic",
                LinkedGameControl::LiteralDebugResetZoom => "debug-reset-zoom",
                LinkedGameControl::LiteralDebugResetZoom2x => "debug-reset-zoom-2x",
                LinkedGameControl::LiteralToggleGuiDebug => "toggle-gui-debug",
                LinkedGameControl::LiteralToggleGuiStyleView => "toggle-gui-style-view",
                LinkedGameControl::LiteralToggleGuiShadows => "toggle-gui-shadows",
                LinkedGameControl::LiteralToggleGuiGlows => "toggle-gui-glows",
                LinkedGameControl::LiteralOpenPrototypesGui => "open-prototypes-gui",
                LinkedGameControl::LiteralOpenPrototypeExplorerGui => {
                    "open-prototype-explorer-gui"
                }
                LinkedGameControl::LiteralIncreaseUiScale => "increase-ui-scale",
                LinkedGameControl::LiteralDecreaseUiScale => "decrease-ui-scale",
                LinkedGameControl::LiteralResetUiScale => "reset-ui-scale",
                LinkedGameControl::LiteralSlashEditor => "slash-editor",
                LinkedGameControl::LiteralToggleEntity => "toggle-entity",
                LinkedGameControl::LiteralNextPlayerInReplay => "next-player-in-replay",
                LinkedGameControl::LiteralMoveBlueprintAbsoluteGridUp => {
                    "move-blueprint-absolute-grid-up"
                }
                LinkedGameControl::LiteralMoveBlueprintAbsoluteGridDown => {
                    "move-blueprint-absolute-grid-down"
                }
                LinkedGameControl::LiteralMoveBlueprintAbsoluteGridLeft => {
                    "move-blueprint-absolute-grid-left"
                }
                LinkedGameControl::LiteralMoveBlueprintAbsoluteGridRight => {
                    "move-blueprint-absolute-grid-right"
                }
                LinkedGameControl::LiteralMoveBlueprintEntitiesUp => {
                    "move-blueprint-entities-up"
                }
                LinkedGameControl::LiteralMoveBlueprintEntitiesDown => {
                    "move-blueprint-entities-down"
                }
                LinkedGameControl::LiteralMoveBlueprintEntitiesLeft => {
                    "move-blueprint-entities-left"
                }
                LinkedGameControl::LiteralMoveBlueprintEntitiesRight => {
                    "move-blueprint-entities-right"
                }
                LinkedGameControl::LiteralToggleBlueprintSnapToGrid => {
                    "toggle-blueprint-snap-to-grid"
                }
                LinkedGameControl::LiteralPlayNextTrack => "play-next-track",
                LinkedGameControl::LiteralPlayPreviousTrack => "play-previous-track",
                LinkedGameControl::LiteralPauseResumeMusic => "pause-resume-music",
                LinkedGameControl::LiteralEmpty => "",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ListBoxStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_style: Option<ButtonStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_pane_style: Option<ScrollPaneStyleSpecification>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LoaderStructure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_in: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_out: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch_in: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch_out: Option<Sprite4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum LocalisedString {
    String(Box<String>),
    Array1(Vec<String>),
}
pub type LogisticFilterIndex = u16;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum LogisticMode {
    #[serde(rename = "active-provider")]
    LiteralActiveProvider,
    #[serde(rename = "passive-provider")]
    LiteralPassiveProvider,
    #[serde(rename = "requester")]
    LiteralRequester,
    #[serde(rename = "storage")]
    LiteralStorage,
    #[serde(rename = "buffer")]
    LiteralBuffer,
}
impl std::fmt::Display for LogisticMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                LogisticMode::LiteralActiveProvider => "active-provider",
                LogisticMode::LiteralPassiveProvider => "passive-provider",
                LogisticMode::LiteralRequester => "requester",
                LogisticMode::LiteralStorage => "storage",
                LogisticMode::LiteralBuffer => "buffer",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LowPowerTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MainSound {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_to_speed_modifiers: Option<ActivityMatchingModifiers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_to_volume_modifiers: Option<ActivityMatchingModifiers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in_ticks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out_ticks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_progress_to_activity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_speed_to_activity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_volume_to_activity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_for_directions: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_for_working_visualisations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_smoothing_window_size: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ManualTransferTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ManualWireDragTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<EntityID>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_type: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MapGenPreset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_settings: Option<AdvancedMapGenSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_settings: Option<MapGenSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    pub order: Order,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MapGenPresetAsteroidSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ray_portals_expanded_per_tick: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawning_rate: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MapGenPresetDifficultySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology_price_multiplier: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MapGenPresetEnemyEvolutionSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pollution_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_factor: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MapGenPresetEnemyExpansionSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evolution_group_size_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_expansion_cooldown: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_expansion_distance: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_expansion_cooldown: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_expansion_distance: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settler_group_max_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settler_group_min_size: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MapGenPresetPollutionSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ageing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffusion_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_attack_pollution_consumption_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pollution_to_damage_trees: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pollution_restored_per_tree_damage: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MapGenSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoplace_controls: Option<
        std::collections::HashMap<AutoplaceControlID, FrequencySizeRichness>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoplace_settings: Option<
        std::collections::HashMap<serde_json::Value, AutoplaceSettings>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliff_settings: Option<CliffPlacementSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_enable_all_autoplace_controls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_enemies_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peaceful_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_expression_names: Option<
        std::collections::HashMap<String, serde_json::Value>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_area: Option<MapGenSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_points: Option<Vec<MapPosition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub territory_settings: Option<TerritorySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum MapGenSize {
    Wrapped0(f64),
    LiteralNone,
    LiteralVeryLow,
    LiteralVerySmall,
    LiteralVeryPoor,
    LiteralLow,
    LiteralSmall,
    LiteralPoor,
    LiteralNormal,
    LiteralMedium,
    LiteralRegular,
    LiteralHigh,
    LiteralBig,
    LiteralGood,
    LiteralVeryHigh,
    LiteralVeryBig,
    LiteralVeryGood,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MapLocation {
    pub direction: u32,
    pub position: MapPosition,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum MapPosition {
    Struct0,
    Variant1((f64, f64)),
}
pub type MapTick = u64;
pub type MaterialAmountType = f64;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MaterialTextureParameters {
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_length: Option<u32>,
    pub picture: FileName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<u16>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct MathExpression(pub String);
impl std::fmt::Display for MathExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for MathExpression {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MaxCargoBayUnloadingDistanceModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MaxFailedAttemptsPerTickPerConstructionQueueModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MaxSuccessfulAttemptsPerTickPerConstructionQueueModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MaximumFollowingRobotsCountModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MinableProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_show_counts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_particle: Option<ParticleID>,
    pub mining_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_fluid: Option<FluidID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ItemID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ProductPrototype>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_entity_health_to_products: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MineEntityTechnologyTrigger {
    pub entities: Vec<EntityID>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MineItemByRobotTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MinimapStyleSpecification {
    #[serde(flatten)]
    pub parent: EmptyWidgetStyleSpecification,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MiningDrillGraphicsSet {
    #[serde(flatten)]
    pub parent: WorkingVisualisations,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector_layer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector_secondary_draw_order: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drilling_vertical_movement_duration: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_animation_when_frozen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MiningDrillProductivityBonusModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MiningWithFluidModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ModSetting {
    pub value: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum Modifier {
    #[serde(rename = "inserter-stack-size-bonus")]
    InserterStackSizeBonusModifier(InserterStackSizeBonusModifier),
    #[serde(rename = "bulk-inserter-capacity-bonus")]
    BulkInserterCapacityBonusModifier(BulkInserterCapacityBonusModifier),
    #[serde(rename = "laboratory-speed")]
    LaboratorySpeedModifier(LaboratorySpeedModifier),
    #[serde(rename = "character-logistic-trash-slots")]
    CharacterLogisticTrashSlotsModifier(CharacterLogisticTrashSlotsModifier),
    #[serde(rename = "maximum-following-robots-count")]
    MaximumFollowingRobotsCountModifier(MaximumFollowingRobotsCountModifier),
    #[serde(rename = "worker-robot-speed")]
    WorkerRobotSpeedModifier(WorkerRobotSpeedModifier),
    #[serde(rename = "worker-robot-storage")]
    WorkerRobotStorageModifier(WorkerRobotStorageModifier),
    #[serde(rename = "turret-attack")]
    TurretAttackModifier(TurretAttackModifier),
    #[serde(rename = "ammo-damage")]
    AmmoDamageModifier(AmmoDamageModifier),
    #[serde(rename = "give-item")]
    GiveItemModifier(GiveItemModifier),
    #[serde(rename = "gun-speed")]
    GunSpeedModifier(GunSpeedModifier),
    #[serde(rename = "unlock-recipe")]
    UnlockRecipeModifier(UnlockRecipeModifier),
    #[serde(rename = "character-crafting-speed")]
    CharacterCraftingSpeedModifier(CharacterCraftingSpeedModifier),
    #[serde(rename = "character-mining-speed")]
    CharacterMiningSpeedModifier(CharacterMiningSpeedModifier),
    #[serde(rename = "character-running-speed")]
    CharacterRunningSpeedModifier(CharacterRunningSpeedModifier),
    #[serde(rename = "character-build-distance")]
    CharacterBuildDistanceModifier(CharacterBuildDistanceModifier),
    #[serde(rename = "character-item-drop-distance")]
    CharacterItemDropDistanceModifier(CharacterItemDropDistanceModifier),
    #[serde(rename = "character-reach-distance")]
    CharacterReachDistanceModifier(CharacterReachDistanceModifier),
    #[serde(rename = "character-resource-reach-distance")]
    CharacterResourceReachDistanceModifier(CharacterResourceReachDistanceModifier),
    #[serde(rename = "character-item-pickup-distance")]
    CharacterItemPickupDistanceModifier(CharacterItemPickupDistanceModifier),
    #[serde(rename = "character-loot-pickup-distance")]
    CharacterLootPickupDistanceModifier(CharacterLootPickupDistanceModifier),
    #[serde(rename = "character-inventory-slots-bonus")]
    CharacterInventorySlotsBonusModifier(CharacterInventorySlotsBonusModifier),
    #[serde(rename = "deconstruction-time-to-live")]
    DeconstructionTimeToLiveModifier(DeconstructionTimeToLiveModifier),
    #[serde(rename = "max-failed-attempts-per-tick-per-construction-queue")]
    MaxFailedAttemptsPerTickPerConstructionQueueModifier(
        MaxFailedAttemptsPerTickPerConstructionQueueModifier,
    ),
    #[serde(rename = "max-successful-attempts-per-tick-per-construction-queue")]
    MaxSuccessfulAttemptsPerTickPerConstructionQueueModifier(
        MaxSuccessfulAttemptsPerTickPerConstructionQueueModifier,
    ),
    #[serde(rename = "character-health-bonus")]
    CharacterHealthBonusModifier(CharacterHealthBonusModifier),
    #[serde(rename = "mining-drill-productivity-bonus")]
    MiningDrillProductivityBonusModifier(MiningDrillProductivityBonusModifier),
    #[serde(rename = "train-braking-force-bonus")]
    TrainBrakingForceBonusModifier(TrainBrakingForceBonusModifier),
    #[serde(rename = "worker-robot-battery")]
    WorkerRobotBatteryModifier(WorkerRobotBatteryModifier),
    #[serde(rename = "laboratory-productivity")]
    LaboratoryProductivityModifier(LaboratoryProductivityModifier),
    #[serde(rename = "follower-robot-lifetime")]
    FollowerRobotLifetimeModifier(FollowerRobotLifetimeModifier),
    #[serde(rename = "artillery-range")]
    ArtilleryRangeModifier(ArtilleryRangeModifier),
    #[serde(rename = "nothing")]
    NothingModifier(NothingModifier),
    #[serde(rename = "character-logistic-requests")]
    CharacterLogisticRequestsModifier(CharacterLogisticRequestsModifier),
    #[serde(rename = "vehicle-logistics")]
    VehicleLogisticsModifier(VehicleLogisticsModifier),
    #[serde(rename = "unlock-space-location")]
    UnlockSpaceLocationModifier(UnlockSpaceLocationModifier),
    #[serde(rename = "unlock-quality")]
    UnlockQualityModifier(UnlockQualityModifier),
    #[serde(rename = "unlock-space-platforms")]
    SpacePlatformsModifier(SpacePlatformsModifier),
    #[serde(rename = "unlock-travel-to-space-platforms")]
    TravelToSpacePlatformsModifier(TravelToSpacePlatformsModifier),
    #[serde(rename = "unlock-circuit-network")]
    CircuitNetworkModifier(CircuitNetworkModifier),
    #[serde(rename = "cargo-landing-pad-count")]
    CargoLandingPadLimitModifier(CargoLandingPadLimitModifier),
    #[serde(rename = "max-cargo-bay-unloading-distance")]
    MaxCargoBayUnloadingDistanceModifier(MaxCargoBayUnloadingDistanceModifier),
    #[serde(rename = "change-recipe-productivity")]
    ChangeRecipeProductivityModifier(ChangeRecipeProductivityModifier),
    #[serde(rename = "cliff-deconstruction-enabled")]
    CliffDeconstructionEnabledModifier(CliffDeconstructionEnabledModifier),
    #[serde(rename = "mining-with-fluid")]
    MiningWithFluidModifier(MiningWithFluidModifier),
    #[serde(rename = "rail-support-on-deep-oil-ocean")]
    RailSupportOnDeepOilOceanModifier(RailSupportOnDeepOilOceanModifier),
    #[serde(rename = "rail-planner-allow-elevated-rails")]
    RailPlannerAllowElevatedRailsModifier(RailPlannerAllowElevatedRailsModifier),
    #[serde(rename = "beacon-distribution")]
    BeaconDistributionModifier(BeaconDistributionModifier),
    #[serde(rename = "create-ghost-on-entity-death")]
    CreateGhostOnEntityDeathModifier(CreateGhostOnEntityDeathModifier),
    #[serde(rename = "belt-stack-size-bonus")]
    BeltStackSizeBonusModifier(BeltStackSizeBonusModifier),
    #[serde(rename = "unlock-logistic-network")]
    UnlockLogisticNetworkModifier(UnlockLogisticNetworkModifier),
}
pub type Mods = std::collections::HashMap<String, String>;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct ModuleCategoryID(pub String);
impl std::fmt::Display for ModuleCategoryID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for ModuleCategoryID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ModuleTint {
    #[serde(rename = "primary")]
    LiteralPrimary,
    #[serde(rename = "secondary")]
    LiteralSecondary,
    #[serde(rename = "tertiary")]
    LiteralTertiary,
    #[serde(rename = "quaternary")]
    LiteralQuaternary,
    #[serde(rename = "none")]
    LiteralNone,
}
impl std::fmt::Display for ModuleTint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ModuleTint::LiteralPrimary => "primary",
                ModuleTint::LiteralSecondary => "secondary",
                ModuleTint::LiteralTertiary => "tertiary",
                ModuleTint::LiteralQuaternary => "quaternary",
                ModuleTint::LiteralNone => "none",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ModuleTransferTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub module: ItemID,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct MouseCursorID(pub String);
impl std::fmt::Display for MouseCursorID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for MouseCursorID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct NeighbourConnectable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_by_direction: Option<bool>,
    pub connections: Vec<NeighbourConnectableConnectionDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbour_search_distance: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct NeighbourConnectableConnectionCategory(pub String);
impl std::fmt::Display for NeighbourConnectableConnectionCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for NeighbourConnectableConnectionCategory {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct NeighbourConnectableConnectionDefinition {
    pub category: NeighbourConnectableConnectionCategory,
    pub location: MapLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbour_category: Option<Vec<NeighbourConnectableConnectionCategory>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct NestedTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    pub action: Trigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum NoiseExpression {
    String(Box<String>),
    Boolean(Box<bool>),
    Double(Box<f64>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct NoiseFunction {
    pub expression: NoiseExpression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_expressions: Option<std::collections::HashMap<String, NoiseExpression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_functions: Option<std::collections::HashMap<String, NoiseFunction>>,
    pub parameters: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct NothingModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_description: Option<String>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct OffshorePumpGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_pictures: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_animation: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glass_pictures: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_layer_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_pictures: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct OrTipTrigger {
    pub triggers: Vec<TipTrigger>,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct Order(pub String);
impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for Order {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct OrientedCliffPrototype {
    pub collision_bounding_box: BoundingBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures_lower: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct OrientedCliffPrototypeSet {
    pub east_to_none: OrientedCliffPrototype,
    pub east_to_north: OrientedCliffPrototype,
    pub east_to_south: OrientedCliffPrototype,
    pub east_to_west: OrientedCliffPrototype,
    pub none_to_east: OrientedCliffPrototype,
    pub none_to_north: OrientedCliffPrototype,
    pub none_to_south: OrientedCliffPrototype,
    pub none_to_west: OrientedCliffPrototype,
    pub north_to_east: OrientedCliffPrototype,
    pub north_to_none: OrientedCliffPrototype,
    pub north_to_south: OrientedCliffPrototype,
    pub north_to_west: OrientedCliffPrototype,
    pub south_to_east: OrientedCliffPrototype,
    pub south_to_none: OrientedCliffPrototype,
    pub south_to_north: OrientedCliffPrototype,
    pub south_to_west: OrientedCliffPrototype,
    pub west_to_east: OrientedCliffPrototype,
    pub west_to_none: OrientedCliffPrototype,
    pub west_to_north: OrientedCliffPrototype,
    pub west_to_south: OrientedCliffPrototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct OtherColors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    pub less_than: f64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct ParticleID(pub String);
impl std::fmt::Display for ParticleID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for ParticleID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PasteEntitySettingsTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<EntityID>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PathFinderSettings {
    pub cache_accept_path_end_distance_ratio: f64,
    pub cache_accept_path_start_distance_ratio: f64,
    pub cache_max_connect_to_cache_steps_multiplier: u32,
    pub cache_path_end_distance_rating_multiplier: f64,
    pub cache_path_start_distance_rating_multiplier: f64,
    pub direct_distance_to_consider_short_request: u32,
    pub enemy_with_different_destination_collision_penalty: f64,
    pub extended_collision_penalty: f64,
    pub fwd2bwd_ratio: u32,
    pub general_entity_collision_penalty: f64,
    pub general_entity_subsequent_collision_penalty: f64,
    pub goal_pressure_ratio: f64,
    pub ignore_moving_enemy_collision_distance: f64,
    pub long_cache_min_cacheable_distance: f64,
    pub long_cache_size: u32,
    pub max_clients_to_accept_any_new_request: u32,
    pub max_clients_to_accept_short_new_request: u32,
    pub max_steps_worked_per_tick: f64,
    pub max_work_done_per_tick: u32,
    pub min_steps_to_check_path_find_termination: u32,
    pub negative_cache_accept_path_end_distance_ratio: f64,
    pub negative_cache_accept_path_start_distance_ratio: f64,
    pub negative_path_cache_delay_interval: u32,
    pub overload_levels: Vec<u32>,
    pub overload_multipliers: Vec<f64>,
    pub short_cache_min_algo_steps_to_cache: u32,
    pub short_cache_min_cacheable_distance: f64,
    pub short_cache_size: u32,
    pub short_request_max_steps: u32,
    pub short_request_ratio: f64,
    pub stale_enemy_with_same_destination_collision_penalty: f64,
    pub start_to_goal_cost_multiplier_to_terminate_path_find: f64,
    pub use_path_cache: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PerceivedPerformance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_to_activity_rate: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum PersistentWorldAmbientSoundDefinition {
    Struct0,
    Sound(Box<Sound>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PersistentWorldAmbientSoundsDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_ambience: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossfade: Option<PersistentWorldAmbientSoundsDefinitionCrossfade>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semi_persistent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PersistentWorldAmbientSoundsDefinitionCrossfade {
    #[serde(flatten)]
    pub parent: Fade,
    pub order: (serde_json::Value, serde_json::Value),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PipeConnectionDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_direction: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_position: Option<MapPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_category: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<PipeConnectionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_working_visualisations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_direction: Option<FluidFlowDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_connection_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_connection_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance_tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_underground_distance: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<MapPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<(MapPosition, MapPosition, MapPosition, MapPosition)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underground_collision_mask: Option<CollisionMaskConnector>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum PipeConnectionType {
    #[serde(rename = "normal")]
    LiteralNormal,
    #[serde(rename = "underground")]
    LiteralUnderground,
    #[serde(rename = "linked")]
    LiteralLinked,
}
impl std::fmt::Display for PipeConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                PipeConnectionType::LiteralNormal => "normal",
                PipeConnectionType::LiteralUnderground => "underground",
                PipeConnectionType::LiteralLinked => "linked",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PipePictures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_down_left: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_down_left_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_down_left_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_down_left_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_down_right: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_down_right_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_down_right_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_down_right_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_up_left: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_up_left_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_up_left_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_up_left_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_up_right: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_up_right_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_up_right_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_up_right_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_down: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_down_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_down_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_down_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_left: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_left_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_left_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_left_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_right: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_right_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_right_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_right_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_up: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_up_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_up_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_up_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_background: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_flow: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_temperature_flow: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_window_background: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_temperature_flow: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_temperature_flow: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_horizontal: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_horizontal_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_horizontal_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_horizontal_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_horizontal_window: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_horizontal_window_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_horizontal_window_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_horizontal_window_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_single: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_single_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_single_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_single_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_window: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_window_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_window_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical_window_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_down: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_down_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_down_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_down_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_left: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_left_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_left_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_left_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_right: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_right_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_right_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_right_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_up: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_up_disabled_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_up_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_up_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_window_background: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlaceAsTile {
    pub condition: CollisionMaskConnector,
    pub condition_size: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    pub result: TileID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_condition: Option<Vec<TileID>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlaceEquipmentTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment: Option<EquipmentID>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlanTrainPathTipTrigger {
    pub distance: f64,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlanetPrototypeMapGenSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoplace_controls: Option<
        std::collections::HashMap<AutoplaceControlID, FrequencySizeRichness>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoplace_settings: Option<
        std::collections::HashMap<serde_json::Value, AutoplaceSettings>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aux_climate_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliff_settings: Option<CliffPlacementSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moisture_climate_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_expression_names: Option<
        std::collections::HashMap<String, serde_json::Value>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub territory_settings: Option<TerritorySettings>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlatformBackdrop {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atmosphere_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atmosphere_ray_light_color_1: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atmosphere_ray_light_color_2: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atmosphere_thickness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_flow_intensity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_flow_seconds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_normal_intensity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_panning_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_vertical_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudiness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_scalar: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_scales_with_shadow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_approach_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cloud: Option<EffectTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cloud_flow: Option<EffectTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cloud_normal: Option<EffectTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_cloud_texture_1: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_cloud_texture_2: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_cloud_texture_3: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_clouds: Option<Vec<PlatformBackdropHeroCloud>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_clouds_are_emissive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_direction: Option<Vector3D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_intensity_contrast: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallax_strength: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_axis: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_axis_deviation_amplitude: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_axis_deviation_seconds: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_emission: Option<EffectTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_normal: Option<EffectTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_reflectivity: Option<EffectTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_surface: Option<EffectTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_seconds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_intensity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_normal_intensity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_vertical_offset: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlatformBackdropHeroCloud {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_deviation: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Vector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_style: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_with_planet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_frame_offset: Option<u16>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum PlayFor {
    #[serde(rename = "character_actions")]
    LiteralCharacterActions,
    #[serde(rename = "everything")]
    LiteralEverything,
}
impl std::fmt::Display for PlayFor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                PlayFor::LiteralCharacterActions => "character_actions",
                PlayFor::LiteralEverything => "everything",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlaySoundTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_on_target_position: Option<bool>,
    pub sound: Sound,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlayerColorData {
    pub chat_color: Color,
    pub name: String,
    pub player_color: Color,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum PlayerInputMethodFilter {
    #[serde(rename = "all")]
    LiteralAll,
    #[serde(rename = "keyboard_and_mouse")]
    LiteralKeyboardAndMouse,
    #[serde(rename = "game_controller")]
    LiteralGameController,
}
impl std::fmt::Display for PlayerInputMethodFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                PlayerInputMethodFilter::LiteralAll => "all",
                PlayerInputMethodFilter::LiteralKeyboardAndMouse => "keyboard_and_mouse",
                PlayerInputMethodFilter::LiteralGameController => "game_controller",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlumeEffect {
    #[serde(flatten)]
    pub parent: StatelessVisualisation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_discrimination: Option<i8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlumesSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_y_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_y_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_visualisations: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PodAnimationProcessionBezierControlPoint {
    pub frame: f64,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PodAnimationProcessionLayer {
    pub frames: Vec<PodAnimationProcessionBezierControlPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphic: Option<ProcessionGraphic>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PodDistanceTraveledProcessionBezierControlPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PodDistanceTraveledProcessionLayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribute_to_distance_traveled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_traveled_contribution: Option<f64>,
    pub frames: Vec<PodDistanceTraveledProcessionBezierControlPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_group: Option<ProcessionLayerInheritanceGroupID>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PodMovementProcessionBezierControlPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_rate_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_t: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tilt: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tilt_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PodMovementProcessionLayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribute_to_distance_traveled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_traveled_contribution: Option<f64>,
    pub frames: Vec<PodMovementProcessionBezierControlPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_from: Option<ProcessionLayerInheritanceGroupID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_group: Option<ProcessionLayerInheritanceGroupID>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PodOpacityProcessionBezierControlPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutscene_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutscene_opacity_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lut_blend: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lut_blend_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outside_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outside_opacity_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PodOpacityProcessionLayer {
    pub frames: Vec<PodOpacityProcessionBezierControlPoint>,
    pub lut: ColorLookupTable,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PollutionSettings {
    pub ageing: f64,
    pub diffusion_ratio: f64,
    pub enabled: bool,
    pub enemy_attack_pollution_consumption_modifier: f64,
    pub expected_max_per_chunk: f64,
    pub max_pollution_to_restore_trees: f64,
    pub min_pollution_to_damage_trees: f64,
    pub min_to_diffuse: f64,
    pub min_to_show_per_chunk: f64,
    pub pollution_per_tree_damage: f64,
    pub pollution_restored_per_tree_damage: f64,
    pub pollution_with_max_forest_damage: f64,
}
pub type ProbabilityTable = Vec<ProbabilityTableItem>;
pub type ProbabilityTableItem = (u8, u8);
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProcessionAudio {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogue_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub looped_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,
    pub r#type: ProcessionAudioType,
}
pub type ProcessionAudioCatalogue = Vec<ProcessionAudioCatalogueItem>;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProcessionAudioCatalogueItem {
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub looped_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProcessionAudioEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<ProcessionAudio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_id: Option<u32>,
    pub r#type: ProcessionAudioEventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ProcessionAudioUsage>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessionAudioEventType {
    #[serde(rename = "play-sound")]
    LiteralPlaySound,
    #[serde(rename = "start-looped-sound")]
    LiteralStartLoopedSound,
    #[serde(rename = "stop-looped-sound")]
    LiteralStopLoopedSound,
}
impl std::fmt::Display for ProcessionAudioEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ProcessionAudioEventType::LiteralPlaySound => "play-sound",
                ProcessionAudioEventType::LiteralStartLoopedSound => "start-looped-sound",
                ProcessionAudioEventType::LiteralStopLoopedSound => "stop-looped-sound",
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessionAudioType {
    #[serde(rename = "none")]
    LiteralNone,
    #[serde(rename = "sound")]
    LiteralSound,
    #[serde(rename = "looped-sound")]
    LiteralLoopedSound,
    #[serde(rename = "pod-catalogue")]
    LiteralPodCatalogue,
    #[serde(rename = "location-catalogue")]
    LiteralLocationCatalogue,
}
impl std::fmt::Display for ProcessionAudioType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ProcessionAudioType::LiteralNone => "none",
                ProcessionAudioType::LiteralSound => "sound",
                ProcessionAudioType::LiteralLoopedSound => "looped-sound",
                ProcessionAudioType::LiteralPodCatalogue => "pod-catalogue",
                ProcessionAudioType::LiteralLocationCatalogue => "location-catalogue",
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessionAudioUsage {
    #[serde(rename = "both")]
    LiteralBoth,
    #[serde(rename = "passenger")]
    LiteralPassenger,
    #[serde(rename = "outside")]
    LiteralOutside,
}
impl std::fmt::Display for ProcessionAudioUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ProcessionAudioUsage::LiteralBoth => "both",
                ProcessionAudioUsage::LiteralPassenger => "passenger",
                ProcessionAudioUsage::LiteralOutside => "outside",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProcessionGraphic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogue_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<Sprite>,
    pub r#type: ProcessionGraphicType,
}
pub type ProcessionGraphicCatalogue = Vec<ProcessionGraphicCatalogueItem>;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProcessionGraphicCatalogueItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<Sprite>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessionGraphicType {
    #[serde(rename = "none")]
    LiteralNone,
    #[serde(rename = "sprite")]
    LiteralSprite,
    #[serde(rename = "animation")]
    LiteralAnimation,
    #[serde(rename = "pod-catalogue")]
    LiteralPodCatalogue,
    #[serde(rename = "location-catalogue")]
    LiteralLocationCatalogue,
    #[serde(rename = "hatch-location-catalogue-index")]
    LiteralHatchLocationCatalogueIndex,
}
impl std::fmt::Display for ProcessionGraphicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ProcessionGraphicType::LiteralNone => "none",
                ProcessionGraphicType::LiteralSprite => "sprite",
                ProcessionGraphicType::LiteralAnimation => "animation",
                ProcessionGraphicType::LiteralPodCatalogue => "pod-catalogue",
                ProcessionGraphicType::LiteralLocationCatalogue => "location-catalogue",
                ProcessionGraphicType::LiteralHatchLocationCatalogueIndex => {
                    "hatch-location-catalogue-index"
                }
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct ProcessionID(pub String);
impl std::fmt::Display for ProcessionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for ProcessionID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum ProcessionLayer {
    #[serde(rename = "pod-distance-traveled")]
    PodDistanceTraveledProcessionLayer(PodDistanceTraveledProcessionLayer),
    #[serde(rename = "pod-movement")]
    PodMovementProcessionLayer(PodMovementProcessionLayer),
    #[serde(rename = "pod-opacity")]
    PodOpacityProcessionLayer(PodOpacityProcessionLayer),
    #[serde(rename = "single-graphic")]
    SingleGraphicProcessionLayer(SingleGraphicProcessionLayer),
    #[serde(rename = "cover-graphic")]
    CoverGraphicProcessionLayer(CoverGraphicProcessionLayer),
    #[serde(rename = "tint")]
    TintProcessionLayer(TintProcessionLayer),
    #[serde(rename = "pod-animation")]
    PodAnimationProcessionLayer(PodAnimationProcessionLayer),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct ProcessionLayerInheritanceGroupID(pub String);
impl std::fmt::Display for ProcessionLayerInheritanceGroupID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for ProcessionLayerInheritanceGroupID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProcessionSet {
    pub arrival: Vec<ProcessionID>,
    pub departure: Vec<ProcessionID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProcessionTimeline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_events: Option<Vec<ProcessionAudioEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_switch_tick: Option<u64>,
    pub duration: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermezzo_max_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermezzo_min_duration: Option<u64>,
    pub layers: Vec<ProcessionLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_action_tick: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum ProductPrototype {
    #[serde(rename = "item")]
    ItemProductPrototype(ItemProductPrototype),
    #[serde(rename = "fluid")]
    FluidProductPrototype(FluidProductPrototype),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProductPrototypeBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub independent_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_probability: Option<SharedProbabilityDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_details_in_recipe_tooltip: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProductionHealthEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_type: Option<DamageTypeID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_producing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producing: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ProductionType {
    #[serde(rename = "none")]
    LiteralNone,
    #[serde(rename = "input")]
    LiteralInput,
    #[serde(rename = "input-output")]
    LiteralInputOutput,
    #[serde(rename = "output")]
    LiteralOutput,
}
impl std::fmt::Display for ProductionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ProductionType::LiteralNone => "none",
                ProductionType::LiteralInput => "input",
                ProductionType::LiteralInputOutput => "input-output",
                ProductionType::LiteralOutput => "output",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProgrammableSpeakerInstrument {
    pub name: String,
    pub notes: Vec<ProgrammableSpeakerNote>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProgrammableSpeakerNote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyclic_sound: Option<CyclicSound>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProgressBarStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_text_in_bar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_colors: Option<Vec<OtherColors>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_text_padding: Option<i16>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProjectileAttackParameters {
    #[serde(flatten)]
    pub parent: BaseAttackParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_projection_to_projectile_creation_position: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectile_center: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectile_creation_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectile_creation_offsets: Option<Vec<Vector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectile_creation_parameters: Option<CircularProjectileCreationSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectile_orientation_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_particle: Option<CircularParticleCreationSpecification>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProjectileTriggerDelivery {
    #[serde(flatten)]
    pub parent: TriggerDeliveryItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_speed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_range: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_range: Option<f64>,
    pub projectile: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_deviation: Option<f64>,
    pub starting_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_speed_deviation: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PrototypeStrafeSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clockwise_chance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_distance_importance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_distance_importance_variance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_distance_tolerance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_distance_variance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PuddleTileEffectParameters {
    pub puddle_noise_texture: EffectTexture,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_effect: Option<TileEffectDefinitionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_effect_parameters: Option<WaterTileEffectParameters>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PumpWagonConnectionGraphics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<BasePumpWagonConnectionAnimations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_animation_finished_at_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamp_animation_starts_at_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamp_y_shift: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_diff_to_wagon: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part1_to_2_shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part2_crop_adjustment: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part2_shadow_crop_adjustment: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_1: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_1_shadow: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_2: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_2_shadow: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resting_position_shift: Option<PumpWagonConnectionShift4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suction_clamp: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suction_clamp_shadow: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_pivot_shift: Option<PumpWagonConnectionShift4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PumpWagonConnectionShift4Way {
    pub east: Vector,
    pub north: Vector,
    pub south: Vector,
    pub west: Vector,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PushBackTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    pub distance: f64,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct QualityID(pub String);
impl std::fmt::Display for QualityID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for QualityID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RadioButtonStyleSpecification {
    #[serde(flatten)]
    pub parent: StyleWithClickableGraphicalSetSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_padding: Option<u32>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RadiusVisualisationSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_quality_multiplier: Option<std::collections::HashMap<QualityID, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_in_cursor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_on_selection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailFenceDirectionSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub northeast: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub northwest: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub southeast: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub southwest: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<SpriteVariations>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailFenceGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_fence_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_fence_render_layer_secondary: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_fence_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_fence_render_layer_secondary: Option<RenderLayer>,
    pub segment_count: u8,
    pub side_A: RailFencePictureSet,
    pub side_B: RailFencePictureSet,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailFencePictureSet {
    pub ends: (
        RailFenceDirectionSet,
        RailFenceDirectionSet,
        RailFenceDirectionSet,
        RailFenceDirectionSet,
    ),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_upper: Option<
        (
            RailFenceDirectionSet,
            RailFenceDirectionSet,
            RailFenceDirectionSet,
            RailFenceDirectionSet,
        ),
    >,
    pub fence: RailFenceDirectionSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fence_upper: Option<RailFenceDirectionSet>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailPictureSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_rail_endings: Option<Sprite16Way>,
    pub east: RailPieceLayers,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog_mask: Option<RailsFogMaskDefinitions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_rail_endings: Option<Sprite16Way>,
    pub north: RailPieceLayers,
    pub northeast: RailPieceLayers,
    pub northwest: RailPieceLayers,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rail_endings: Option<Sprite16Way>,
    pub render_layers: RailRenderLayers,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_render_layers: Option<RailRenderLayers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_visualisation_endings: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slice_origin: Option<RailsSliceOffsets>,
    pub south: RailPieceLayers,
    pub southeast: RailPieceLayers,
    pub southwest: RailPieceLayers,
    pub west: RailPieceLayers,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailPieceLayers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backplates: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metals: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_visualisation_middle: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_mask: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_subtract_mask: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stone_path: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stone_path_background: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ties: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_structure: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailPlannerAllowElevatedRailsModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailRenderLayers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_end: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_end: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metal: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screw: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stone_path: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stone_path_lower: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tie: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_layer_offset: Option<i8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailSignalColorToFrameIndex {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yellow: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailSignalLightDefinition {
    pub light: LightDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailSignalLights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue: Option<RailSignalLightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green: Option<RailSignalLightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red: Option<RailSignalLightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yellow: Option<RailSignalLightDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailSignalPictureSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<Vec<CircuitConnectorDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector_render_layer: Option<RenderLayer>,
    pub lights: RailSignalLights,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rail_piece: Option<RailSignalStaticSpriteLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_box_shift: Option<Vec<Vector>>,
    pub signal_color_to_structure_frame_index: RailSignalColorToFrameIndex,
    pub structure: RotatedAnimation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_align_to_animation_index: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_rail_piece: Option<RailSignalStaticSpriteLayer>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailSignalStaticSpriteLayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_to_frame_index: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_if_not_connected_to_rails: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_if_simulation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shifts: Option<Vec<MapPosition>>,
    pub sprites: Animation,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailSupportGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    pub structure: RotatedSprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_layer_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_structure: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailSupportOnDeepOilOceanModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailsFogMaskDefinitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<FogMaskShapeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north: Option<FogMaskShapeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<FogMaskShapeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<FogMaskShapeDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailsSliceOffsets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum RandomRange {
    Wrapped0((u8, u8)),
    Wrapped1(u8),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum RangeMode {
    #[serde(rename = "center-to-center")]
    LiteralCenterToCenter,
    #[serde(rename = "bounding-box-to-bounding-box")]
    LiteralBoundingBoxToBoundingBox,
    #[serde(rename = "center-to-bounding-box")]
    LiteralCenterToBoundingBox,
}
impl std::fmt::Display for RangeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                RangeMode::LiteralCenterToCenter => "center-to-center",
                RangeMode::LiteralBoundingBoxToBoundingBox => {
                    "bounding-box-to-bounding-box"
                }
                RangeMode::LiteralCenterToBoundingBox => "center-to-bounding-box",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum RangedValue {
    Variant0((f64, f64)),
    Float(Box<f64>),
}
pub type RealOrientation = f64;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct RecipeCategoryID(pub String);
impl std::fmt::Display for RecipeCategoryID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for RecipeCategoryID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct RecipeID(pub String);
impl std::fmt::Display for RecipeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for RecipeID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RecipeTints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quaternary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tertiary: Option<Color>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum RenderLayer {
    #[serde(rename = "zero")]
    LiteralZero,
    #[serde(rename = "background-transitions")]
    LiteralBackgroundTransitions,
    #[serde(rename = "under-tiles")]
    LiteralUnderTiles,
    #[serde(rename = "decals")]
    LiteralDecals,
    #[serde(rename = "above-tiles")]
    LiteralAboveTiles,
    #[serde(rename = "ground-layer-1")]
    LiteralGroundLayer1,
    #[serde(rename = "ground-layer-2")]
    LiteralGroundLayer2,
    #[serde(rename = "ground-layer-3")]
    LiteralGroundLayer3,
    #[serde(rename = "ground-layer-4")]
    LiteralGroundLayer4,
    #[serde(rename = "ground-layer-5")]
    LiteralGroundLayer5,
    #[serde(rename = "lower-radius-visualization")]
    LiteralLowerRadiusVisualization,
    #[serde(rename = "radius-visualization")]
    LiteralRadiusVisualization,
    #[serde(rename = "transport-belt-integration")]
    LiteralTransportBeltIntegration,
    #[serde(rename = "resource")]
    LiteralResource,
    #[serde(rename = "building-smoke")]
    LiteralBuildingSmoke,
    #[serde(rename = "rail-stone-path-lower")]
    LiteralRailStonePathLower,
    #[serde(rename = "rail-stone-path")]
    LiteralRailStonePath,
    #[serde(rename = "rail-tie")]
    LiteralRailTie,
    #[serde(rename = "decorative")]
    LiteralDecorative,
    #[serde(rename = "ground-patch")]
    LiteralGroundPatch,
    #[serde(rename = "ground-patch-higher")]
    LiteralGroundPatchHigher,
    #[serde(rename = "ground-patch-higher2")]
    LiteralGroundPatchHigher2,
    #[serde(rename = "rail-chain-signal-metal")]
    LiteralRailChainSignalMetal,
    #[serde(rename = "rail-screw")]
    LiteralRailScrew,
    #[serde(rename = "rail-metal")]
    LiteralRailMetal,
    #[serde(rename = "remnants")]
    LiteralRemnants,
    #[serde(rename = "floor")]
    LiteralFloor,
    #[serde(rename = "transport-belt")]
    LiteralTransportBelt,
    #[serde(rename = "transport-belt-endings")]
    LiteralTransportBeltEndings,
    #[serde(rename = "floor-mechanics-under-corpse")]
    LiteralFloorMechanicsUnderCorpse,
    #[serde(rename = "corpse")]
    LiteralCorpse,
    #[serde(rename = "floor-mechanics")]
    LiteralFloorMechanics,
    #[serde(rename = "item")]
    LiteralItem,
    #[serde(rename = "transport-belt-reader")]
    LiteralTransportBeltReader,
    #[serde(rename = "lower-object")]
    LiteralLowerObject,
    #[serde(rename = "transport-belt-circuit-connector")]
    LiteralTransportBeltCircuitConnector,
    #[serde(rename = "lower-object-above-shadow")]
    LiteralLowerObjectAboveShadow,
    #[serde(rename = "lower-object-overlay")]
    LiteralLowerObjectOverlay,
    #[serde(rename = "object-under")]
    LiteralObjectUnder,
    #[serde(rename = "object")]
    LiteralObject,
    #[serde(rename = "cargo-hatch")]
    LiteralCargoHatch,
    #[serde(rename = "higher-object-under")]
    LiteralHigherObjectUnder,
    #[serde(rename = "higher-object-above")]
    LiteralHigherObjectAbove,
    #[serde(rename = "train-stop-top")]
    LiteralTrainStopTop,
    #[serde(rename = "item-in-inserter-hand")]
    LiteralItemInInserterHand,
    #[serde(rename = "above-inserters")]
    LiteralAboveInserters,
    #[serde(rename = "wires")]
    LiteralWires,
    #[serde(rename = "under-elevated")]
    LiteralUnderElevated,
    #[serde(rename = "elevated-rail-stone-path-lower")]
    LiteralElevatedRailStonePathLower,
    #[serde(rename = "elevated-rail-stone-path")]
    LiteralElevatedRailStonePath,
    #[serde(rename = "elevated-rail-tie")]
    LiteralElevatedRailTie,
    #[serde(rename = "elevated-rail-screw")]
    LiteralElevatedRailScrew,
    #[serde(rename = "elevated-rail-metal")]
    LiteralElevatedRailMetal,
    #[serde(rename = "elevated-lower-object")]
    LiteralElevatedLowerObject,
    #[serde(rename = "elevated-object")]
    LiteralElevatedObject,
    #[serde(rename = "elevated-higher-object")]
    LiteralElevatedHigherObject,
    #[serde(rename = "fluid-visualization")]
    LiteralFluidVisualization,
    #[serde(rename = "wires-above")]
    LiteralWiresAbove,
    #[serde(rename = "entity-info-icon")]
    LiteralEntityInfoIcon,
    #[serde(rename = "entity-info-icon-above")]
    LiteralEntityInfoIconAbove,
    #[serde(rename = "explosion")]
    LiteralExplosion,
    #[serde(rename = "projectile")]
    LiteralProjectile,
    #[serde(rename = "smoke")]
    LiteralSmoke,
    #[serde(rename = "air-object")]
    LiteralAirObject,
    #[serde(rename = "air-entity-info-icon")]
    LiteralAirEntityInfoIcon,
    #[serde(rename = "light-effect")]
    LiteralLightEffect,
    #[serde(rename = "selection-box")]
    LiteralSelectionBox,
    #[serde(rename = "higher-selection-box")]
    LiteralHigherSelectionBox,
    #[serde(rename = "collision-selection-box")]
    LiteralCollisionSelectionBox,
    #[serde(rename = "arrow")]
    LiteralArrow,
    #[serde(rename = "cursor")]
    LiteralCursor,
}
impl std::fmt::Display for RenderLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                RenderLayer::LiteralZero => "zero",
                RenderLayer::LiteralBackgroundTransitions => "background-transitions",
                RenderLayer::LiteralUnderTiles => "under-tiles",
                RenderLayer::LiteralDecals => "decals",
                RenderLayer::LiteralAboveTiles => "above-tiles",
                RenderLayer::LiteralGroundLayer1 => "ground-layer-1",
                RenderLayer::LiteralGroundLayer2 => "ground-layer-2",
                RenderLayer::LiteralGroundLayer3 => "ground-layer-3",
                RenderLayer::LiteralGroundLayer4 => "ground-layer-4",
                RenderLayer::LiteralGroundLayer5 => "ground-layer-5",
                RenderLayer::LiteralLowerRadiusVisualization => {
                    "lower-radius-visualization"
                }
                RenderLayer::LiteralRadiusVisualization => "radius-visualization",
                RenderLayer::LiteralTransportBeltIntegration => {
                    "transport-belt-integration"
                }
                RenderLayer::LiteralResource => "resource",
                RenderLayer::LiteralBuildingSmoke => "building-smoke",
                RenderLayer::LiteralRailStonePathLower => "rail-stone-path-lower",
                RenderLayer::LiteralRailStonePath => "rail-stone-path",
                RenderLayer::LiteralRailTie => "rail-tie",
                RenderLayer::LiteralDecorative => "decorative",
                RenderLayer::LiteralGroundPatch => "ground-patch",
                RenderLayer::LiteralGroundPatchHigher => "ground-patch-higher",
                RenderLayer::LiteralGroundPatchHigher2 => "ground-patch-higher2",
                RenderLayer::LiteralRailChainSignalMetal => "rail-chain-signal-metal",
                RenderLayer::LiteralRailScrew => "rail-screw",
                RenderLayer::LiteralRailMetal => "rail-metal",
                RenderLayer::LiteralRemnants => "remnants",
                RenderLayer::LiteralFloor => "floor",
                RenderLayer::LiteralTransportBelt => "transport-belt",
                RenderLayer::LiteralTransportBeltEndings => "transport-belt-endings",
                RenderLayer::LiteralFloorMechanicsUnderCorpse => {
                    "floor-mechanics-under-corpse"
                }
                RenderLayer::LiteralCorpse => "corpse",
                RenderLayer::LiteralFloorMechanics => "floor-mechanics",
                RenderLayer::LiteralItem => "item",
                RenderLayer::LiteralTransportBeltReader => "transport-belt-reader",
                RenderLayer::LiteralLowerObject => "lower-object",
                RenderLayer::LiteralTransportBeltCircuitConnector => {
                    "transport-belt-circuit-connector"
                }
                RenderLayer::LiteralLowerObjectAboveShadow => "lower-object-above-shadow",
                RenderLayer::LiteralLowerObjectOverlay => "lower-object-overlay",
                RenderLayer::LiteralObjectUnder => "object-under",
                RenderLayer::LiteralObject => "object",
                RenderLayer::LiteralCargoHatch => "cargo-hatch",
                RenderLayer::LiteralHigherObjectUnder => "higher-object-under",
                RenderLayer::LiteralHigherObjectAbove => "higher-object-above",
                RenderLayer::LiteralTrainStopTop => "train-stop-top",
                RenderLayer::LiteralItemInInserterHand => "item-in-inserter-hand",
                RenderLayer::LiteralAboveInserters => "above-inserters",
                RenderLayer::LiteralWires => "wires",
                RenderLayer::LiteralUnderElevated => "under-elevated",
                RenderLayer::LiteralElevatedRailStonePathLower => {
                    "elevated-rail-stone-path-lower"
                }
                RenderLayer::LiteralElevatedRailStonePath => "elevated-rail-stone-path",
                RenderLayer::LiteralElevatedRailTie => "elevated-rail-tie",
                RenderLayer::LiteralElevatedRailScrew => "elevated-rail-screw",
                RenderLayer::LiteralElevatedRailMetal => "elevated-rail-metal",
                RenderLayer::LiteralElevatedLowerObject => "elevated-lower-object",
                RenderLayer::LiteralElevatedObject => "elevated-object",
                RenderLayer::LiteralElevatedHigherObject => "elevated-higher-object",
                RenderLayer::LiteralFluidVisualization => "fluid-visualization",
                RenderLayer::LiteralWiresAbove => "wires-above",
                RenderLayer::LiteralEntityInfoIcon => "entity-info-icon",
                RenderLayer::LiteralEntityInfoIconAbove => "entity-info-icon-above",
                RenderLayer::LiteralExplosion => "explosion",
                RenderLayer::LiteralProjectile => "projectile",
                RenderLayer::LiteralSmoke => "smoke",
                RenderLayer::LiteralAirObject => "air-object",
                RenderLayer::LiteralAirEntityInfoIcon => "air-entity-info-icon",
                RenderLayer::LiteralLightEffect => "light-effect",
                RenderLayer::LiteralSelectionBox => "selection-box",
                RenderLayer::LiteralHigherSelectionBox => "higher-selection-box",
                RenderLayer::LiteralCollisionSelectionBox => "collision-selection-box",
                RenderLayer::LiteralArrow => "arrow",
                RenderLayer::LiteralCursor => "cursor",
            },
        )
    }
}
pub type ResearchIngredient = (ItemID, u16);
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ResearchTechnologyTipTrigger {
    pub technology: TechnologyID,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ResearchWithSciencePackTipTrigger {
    pub science_pack: ItemID,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Resistance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decrease: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<f64>,
    pub r#type: DamageTypeID,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct ResourceCategoryID(pub String);
impl std::fmt::Display for ResourceCategoryID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for ResourceCategoryID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum RichTextSetting {
    #[serde(rename = "enabled")]
    LiteralEnabled,
    #[serde(rename = "disabled")]
    LiteralDisabled,
    #[serde(rename = "highlight")]
    LiteralHighlight,
}
impl std::fmt::Display for RichTextSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                RichTextSetting::LiteralEnabled => "enabled",
                RichTextSetting::LiteralDisabled => "disabled",
                RichTextSetting::LiteralHighlight => "highlight",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RobotDoorSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_duration: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RollingStockRotatedSlopedGraphics {
    pub rotated: RotatedSprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slope_angle_between_frames: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slope_back_equals_front: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sloped: Option<RotatedSprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RotateEntityTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RotatedAnimation {
    #[serde(flatten)]
    pub parent: AnimationParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_projection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterclockwise: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<FileName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<RotatedAnimation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_per_file: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_orientation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_range: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slice: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub still_frame: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripes: Option<Vec<Stripe>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum RotatedAnimation8Way {
    Struct0,
    RotatedAnimation(Box<RotatedAnimation>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum RotatedAnimationVariations {
    RotatedAnimation(Box<RotatedAnimation>),
    Array1(Vec<RotatedAnimation>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RotatedSprite {
    #[serde(flatten)]
    pub parent: SpriteParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_low_quality_rotation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_projection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_equals_front: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterclockwise: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<FileName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<Vec<RotatedSpriteFrame>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_sdf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<RotatedSprite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_per_file: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RotatedSpriteFrame {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ScriptTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_event: Option<CustomEventID>,
    pub effect_id: String,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ScriptedTechnologyTrigger {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_description: Option<String>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ScrollBarStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_button_style: Option<ButtonStyleSpecification>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ScrollPaneStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_draw_borders: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_force_clipping_rect_for_contents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_bottom_margin_when_activated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_bottom_padding_when_activated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_left_margin_when_activated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_left_padding_when_activated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_margin_when_activated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_padding_when_activated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_right_margin_when_activated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_right_padding_when_activated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_top_margin_when_activated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_top_padding_when_activated: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_scrollbar_style: Option<HorizontalScrollBarStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollbars_go_outside: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_flow_style: Option<VerticalFlowStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_scrollbar_style: Option<VerticalScrollBarStyleSpecification>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SegmentEngineSpecification {
    pub segments: Vec<SegmentSpecification>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SegmentSpecification {
    pub segment: EntityID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SelectionModeData {
    pub border_color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_button_color: Option<Color>,
    pub cursor_box_type: CursorBoxType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_filter_mode: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_filters: Option<Vec<EntityID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type_filters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_cannot_select_entities: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_cannot_select_tiles: Option<bool>,
    pub mode: SelectionModeFlags,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_ended_sound_when_nothing_selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_filter_mode: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_filters: Option<Vec<TileID>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum SelectionModeFlags {
    Variant0(serde_json::Value),
    Array1(Vec<serde_json::Value>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum SemiPersistentWorldAmbientSoundDefinition {
    Struct0,
    Sound(Box<Sound>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SendItemToOrbitTechnologyTrigger {
    pub item: ItemIDFilter,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SendSpidertronTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum SendToOrbitMode {
    #[serde(rename = "not-sendable")]
    LiteralNotSendable,
    #[serde(rename = "manual")]
    LiteralManual,
    #[serde(rename = "automated")]
    LiteralAutomated,
}
impl std::fmt::Display for SendToOrbitMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                SendToOrbitMode::LiteralNotSendable => "not-sendable",
                SendToOrbitMode::LiteralManual => "manual",
                SendToOrbitMode::LiteralAutomated => "automated",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SequenceTipTrigger {
    pub triggers: Vec<TipTrigger>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SetFilterTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type_only: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SetLogisticRequestTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logistic_chest_only: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SetRecipeTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_quality: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<RecipeID>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_fluid: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SetTileTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_on_space_platform: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_projection: Option<bool>,
    pub radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_collision_mask: Option<TileCollisionMaskConnector>,
    pub tile_name: TileID,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Settings {
    pub startup: std::collections::HashMap<String, ModSetting>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SharedProbabilityDefinition {
    pub max: f64,
    pub min: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ShiftAnimationWaypoints {
    pub east: Vec<Vector>,
    pub north: Vec<Vector>,
    pub south: Vec<Vector>,
    pub west: Vec<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ShootTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<serde_json::Value>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ShowExplosionOnChartTriggerEffectItem {
    #[serde(flatten)]
    pub parent: TriggerEffectItem,
    pub scale: f64,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SignalColorMapping {
    #[serde(flatten)]
    pub parent: SignalIDConnector,
    pub color: Color,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SignalIDConnector {
    pub name: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityID>,
    pub r#type: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum SimpleBoundingBox {
    Struct0,
    Variant1((MapPosition, MapPosition)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SimpleModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    pub modifier: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SimulationDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_view_settings: Option<GameViewSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_map: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_factoriopedia_gradient: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_health_bars: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_file: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_update_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_alert_sounds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_technology_finished_sound: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_wind_sounds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_volume: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet: Option<SpaceLocationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_file: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_modifier: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SingleGraphicLayerProcessionBezierControlPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_rate_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_t: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_t: Option<Color>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SingleGraphicProcessionLayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_driven_by_curve: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_with_hatches: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compensated_pivot: Option<bool>,
    pub frames: Vec<SingleGraphicLayerProcessionBezierControlPoint>,
    pub graphic: ProcessionGraphic,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_passenger_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_to: Option<EffectRelativeTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotates_with_pod: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_draw_order: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_rotates_with_pod: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SliderStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<ButtonStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_notches: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_bar: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_bar_disabled: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_bar: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_bar_disabled: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_button: Option<ButtonStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notch: Option<ElementImageSet>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SmokeSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deviation: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_position: Option<Vector>,
    pub frequency: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_8_directions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_deviation: Option<f64>,
    pub name: TrivialSmokeID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_east_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_west_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_east_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_west_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_frame: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_frame_deviation: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_vertical_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_vertical_speed_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_speed_slowdown: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_position: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum Sound {
    Struct0,
    SoundDefinition(Box<SoundDefinition>),
    Array2(Vec<SoundDefinition>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SoundAccent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_for_directions: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_for_working_visualisation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum SoundDefinition {
    Struct0,
    FileName(Box<FileName>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SoundModifier {
    pub r#type: SoundModifierType,
    pub volume_multiplier: f64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum SoundModifierType {
    #[serde(rename = "game")]
    LiteralGame,
    #[serde(rename = "main-menu")]
    LiteralMainMenu,
    #[serde(rename = "tips-and-tricks")]
    LiteralTipsAndTricks,
    #[serde(rename = "driving")]
    LiteralDriving,
    #[serde(rename = "elevation")]
    LiteralElevation,
    #[serde(rename = "space-platform")]
    LiteralSpacePlatform,
    #[serde(rename = "tall-entities-hidden")]
    LiteralTallEntitiesHidden,
}
impl std::fmt::Display for SoundModifierType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                SoundModifierType::LiteralGame => "game",
                SoundModifierType::LiteralMainMenu => "main-menu",
                SoundModifierType::LiteralTipsAndTricks => "tips-and-tricks",
                SoundModifierType::LiteralDriving => "driving",
                SoundModifierType::LiteralElevation => "elevation",
                SoundModifierType::LiteralSpacePlatform => "space-platform",
                SoundModifierType::LiteralTallEntitiesHidden => "tall-entities-hidden",
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum SoundType {
    #[serde(rename = "game-effect")]
    LiteralGameEffect,
    #[serde(rename = "gui-effect")]
    LiteralGuiEffect,
    #[serde(rename = "ambient")]
    LiteralAmbient,
    #[serde(rename = "environment")]
    LiteralEnvironment,
    #[serde(rename = "walking")]
    LiteralWalking,
    #[serde(rename = "alert")]
    LiteralAlert,
    #[serde(rename = "wind")]
    LiteralWind,
    #[serde(rename = "world-ambient")]
    LiteralWorldAmbient,
    #[serde(rename = "weapon")]
    LiteralWeapon,
    #[serde(rename = "explosion")]
    LiteralExplosion,
    #[serde(rename = "enemy")]
    LiteralEnemy,
}
impl std::fmt::Display for SoundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                SoundType::LiteralGameEffect => "game-effect",
                SoundType::LiteralGuiEffect => "gui-effect",
                SoundType::LiteralAmbient => "ambient",
                SoundType::LiteralEnvironment => "environment",
                SoundType::LiteralWalking => "walking",
                SoundType::LiteralAlert => "alert",
                SoundType::LiteralWind => "wind",
                SoundType::LiteralWorldAmbient => "world-ambient",
                SoundType::LiteralWeapon => "weapon",
                SoundType::LiteralExplosion => "explosion",
                SoundType::LiteralEnemy => "enemy",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum SpaceConnectionAsteroidSpawnDefinition {
    Struct0,
    Variant1((EntityID, Vec<SpaceConnectionAsteroidSpawnPoint>)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpaceConnectionAsteroidSpawnPoint {
    #[serde(flatten)]
    pub parent: AsteroidSpawnPoint,
    pub distance: f64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct SpaceConnectionID(pub String);
impl std::fmt::Display for SpaceConnectionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for SpaceConnectionID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpaceDustEffectProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_speed: Option<f64>,
    pub asteroid_normal_texture: EffectTexture,
    pub asteroid_texture: EffectTexture,
    pub noise_texture: EffectTexture,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpaceLocationAsteroidSpawnDefinition {
    #[serde(flatten)]
    pub parent: AsteroidSpawnPoint,
    pub asteroid: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct SpaceLocationID(pub String);
impl std::fmt::Display for SpaceLocationID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for SpaceLocationID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpacePlatformTileDefinition {
    pub position: TilePosition,
    pub tile: TileID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpacePlatformsModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpaceTileEffectParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nebula_brightness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nebula_saturation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nebula_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_brightness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_density: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_parallax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_saturations: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_shape: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom_base_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom_base_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom_exponent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom_offset: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpacingItem {
    pub index: u32,
    pub spacing: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum SpawnPoint {
    Struct0,
    Variant1((f64, f64)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpeechBubbleStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrow_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrow_indent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_style: Option<FrameStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_style: Option<LabelStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_through_mouse: Option<bool>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpentFluidSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    pub name: FluidID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpiderEngineSpecification {
    pub legs: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walking_group_overlap: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpiderLegGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foot: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foot_shadow: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joint: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joint_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joint_shadow: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joint_turn_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_part: Option<SpiderLegPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_part_shadow: Option<SpiderLegPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_part_water_reflection: Option<SpiderLegPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_part: Option<SpiderLegPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_part_shadow: Option<SpiderLegPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_part_water_reflection: Option<SpiderLegPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpiderLegPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_end: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_end_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_end_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_offset_from_bottom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_offset_from_top: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_end: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_end_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_end_offset: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpiderLegSpecification {
    pub ground_position: Vector,
    pub leg: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leg_hit_the_ground_trigger: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leg_hit_the_ground_when_attacking_trigger: Option<TriggerEffect>,
    pub mount_position: Vector,
    pub walking_group: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpiderLegTriggerEffect {
    pub effect: TriggerEffect,
    pub position: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpiderTorsoGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_animation: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_animation: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_base_animation: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpiderVehicleGraphicsSet {
    #[serde(flatten)]
    pub parent: SpiderTorsoGraphicsSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autopilot_destination_on_map_visualisation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autopilot_destination_queue_on_map_visualisation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autopilot_destination_queue_visualisation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autopilot_destination_visualisation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autopilot_destination_visualisation_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autopilot_path_visualisation_line_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autopilot_path_visualisation_on_map_line_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_color: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eye_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_positions: Option<Vec<Vec<Vector>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpoilToTriggerResult {
    pub items_per_trigger: u32,
    pub trigger: Trigger,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Sprite {
    #[serde(flatten)]
    pub parent: SpriteParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Sprite>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Sprite16Way {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_north_east: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_south_east: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_east: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_north_east: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_north_west: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_west: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<SpriteNWaySheet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<SpriteNWaySheet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_east: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_south_east: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_south_west: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_west: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_north_west: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_south_west: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum Sprite4Way {
    Struct0,
    Sprite(Box<Sprite>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Sprite8Way {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_east: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_west: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<SpriteNWaySheet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<SpriteNWaySheet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_east: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_west: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west: Option<Sprite>,
}
pub type SpriteFlags = Vec<serde_json::Value>;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpriteNWaySheet {
    #[serde(flatten)]
    pub parent: SpriteParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_repeat: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_sdf: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpriteParameters {
    #[serde(flatten)]
    pub parent: SpriteSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_runtime_tint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_special_effect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_as_glow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_as_light: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_as_shadow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<SpriteFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_sdf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_colors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mipmap_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occludes_light: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<SpritePriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_shift: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<SpriteUsageSurfaceHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_as_overlay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<SpriteUsageHint>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum SpritePriority {
    #[serde(rename = "extra-high-no-scale")]
    LiteralExtraHighNoScale,
    #[serde(rename = "extra-high")]
    LiteralExtraHigh,
    #[serde(rename = "high")]
    LiteralHigh,
    #[serde(rename = "medium")]
    LiteralMedium,
    #[serde(rename = "low")]
    LiteralLow,
    #[serde(rename = "very-low")]
    LiteralVeryLow,
    #[serde(rename = "no-atlas")]
    LiteralNoAtlas,
}
impl std::fmt::Display for SpritePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                SpritePriority::LiteralExtraHighNoScale => "extra-high-no-scale",
                SpritePriority::LiteralExtraHigh => "extra-high",
                SpritePriority::LiteralHigh => "high",
                SpritePriority::LiteralMedium => "medium",
                SpritePriority::LiteralLow => "low",
                SpritePriority::LiteralVeryLow => "very-low",
                SpritePriority::LiteralNoAtlas => "no-atlas",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpriteSheet {
    #[serde(flatten)]
    pub parent: SpriteParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<FileName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<SpriteSheet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_per_file: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variation_count: Option<u32>,
}
pub type SpriteSizeType = i16;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpriteSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_forced_downscale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_channels: Option<serde_json::Value>,
    pub filename: FileName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_in_minimal_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<(u16, u16)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premul_alpha: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<u16>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum SpriteUsageHint {
    #[serde(rename = "any")]
    LiteralAny,
    #[serde(rename = "mining")]
    LiteralMining,
    #[serde(rename = "tile-artifical")]
    LiteralTileArtifical,
    #[serde(rename = "corpse-decay")]
    LiteralCorpseDecay,
    #[serde(rename = "enemy")]
    LiteralEnemy,
    #[serde(rename = "player")]
    LiteralPlayer,
    #[serde(rename = "train")]
    LiteralTrain,
    #[serde(rename = "vehicle")]
    LiteralVehicle,
    #[serde(rename = "explosion")]
    LiteralExplosion,
    #[serde(rename = "rail")]
    LiteralRail,
    #[serde(rename = "elevated-rail")]
    LiteralElevatedRail,
    #[serde(rename = "air")]
    LiteralAir,
    #[serde(rename = "remnant")]
    LiteralRemnant,
    #[serde(rename = "decorative")]
    LiteralDecorative,
}
impl std::fmt::Display for SpriteUsageHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                SpriteUsageHint::LiteralAny => "any",
                SpriteUsageHint::LiteralMining => "mining",
                SpriteUsageHint::LiteralTileArtifical => "tile-artifical",
                SpriteUsageHint::LiteralCorpseDecay => "corpse-decay",
                SpriteUsageHint::LiteralEnemy => "enemy",
                SpriteUsageHint::LiteralPlayer => "player",
                SpriteUsageHint::LiteralTrain => "train",
                SpriteUsageHint::LiteralVehicle => "vehicle",
                SpriteUsageHint::LiteralExplosion => "explosion",
                SpriteUsageHint::LiteralRail => "rail",
                SpriteUsageHint::LiteralElevatedRail => "elevated-rail",
                SpriteUsageHint::LiteralAir => "air",
                SpriteUsageHint::LiteralRemnant => "remnant",
                SpriteUsageHint::LiteralDecorative => "decorative",
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum SpriteUsageSurfaceHint {
    #[serde(rename = "any")]
    LiteralAny,
    #[serde(rename = "nauvis")]
    LiteralNauvis,
    #[serde(rename = "vulcanus")]
    LiteralVulcanus,
    #[serde(rename = "gleba")]
    LiteralGleba,
    #[serde(rename = "fulgora")]
    LiteralFulgora,
    #[serde(rename = "aquilo")]
    LiteralAquilo,
    #[serde(rename = "space")]
    LiteralSpace,
}
impl std::fmt::Display for SpriteUsageSurfaceHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                SpriteUsageSurfaceHint::LiteralAny => "any",
                SpriteUsageSurfaceHint::LiteralNauvis => "nauvis",
                SpriteUsageSurfaceHint::LiteralVulcanus => "vulcanus",
                SpriteUsageSurfaceHint::LiteralGleba => "gleba",
                SpriteUsageSurfaceHint::LiteralFulgora => "fulgora",
                SpriteUsageSurfaceHint::LiteralAquilo => "aquilo",
                SpriteUsageSurfaceHint::LiteralSpace => "space",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum SpriteVariations {
    Struct0,
    SpriteSheet(Box<SpriteSheet>),
    Array2(Vec<Sprite>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StackTransferTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<serde_json::Value>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StateSteeringSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separation_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separation_force: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StatelessVisualisation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_z: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_animation_speed_by_base_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_by_wind: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_lay_on_the_ground: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in_progress_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out_progress_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_slowdown_factor_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_slowdown_factor_y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_slowdown_factor_z: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_visualisations: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<RangedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<RangedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_z: Option<RangedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_tick_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Vector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<RangedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_x: Option<RangedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_y: Option<RangedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_z: Option<RangedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread_progress_duration: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StatusColors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_output: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insufficient_input: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_power: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_minable_resources: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_power: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working: Option<Color>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SteeringSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_unit_fuzzy_goto_behavior: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#move: Option<StateSteeringSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stay: Option<StateSteeringSettings>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StorageTankPictures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_background: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_flow: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_background: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StreamAttackParameters {
    #[serde(flatten)]
    pub parent: BaseAttackParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_consumption: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluids: Option<Vec<StreamFluidProperties>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gun_barrel_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gun_center_shift: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectile_creation_parameters: Option<CircularProjectileCreationSpecification>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StreamFluidProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_modifier: Option<f64>,
    pub r#type: FluidID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StreamTriggerDelivery {
    #[serde(flatten)]
    pub parent: TriggerDeliveryItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_offset: Option<Vector>,
    pub stream: EntityID,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum StretchRule {
    #[serde(rename = "on")]
    LiteralOn,
    #[serde(rename = "off")]
    LiteralOff,
    #[serde(rename = "auto")]
    LiteralAuto,
    #[serde(rename = "stretch_and_expand")]
    LiteralStretchAndExpand,
}
impl std::fmt::Display for StretchRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                StretchRule::LiteralOn => "on",
                StretchRule::LiteralOff => "off",
                StretchRule::LiteralAuto => "auto",
                StretchRule::LiteralStretchAndExpand => "stretch_and_expand",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Stripe {
    pub filename: FileName,
    pub height_in_frames: u32,
    pub width_in_frames: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum StyleSpecification {
    #[serde(rename = "activity_bar_style")]
    ActivityBarStyleSpecification(ActivityBarStyleSpecification),
    #[serde(rename = "button_style")]
    ButtonStyleSpecification(ButtonStyleSpecification),
    #[serde(rename = "camera_style")]
    CameraStyleSpecification(CameraStyleSpecification),
    #[serde(rename = "checkbox_style")]
    CheckBoxStyleSpecification(CheckBoxStyleSpecification),
    #[serde(rename = "dropdown_style")]
    DropDownStyleSpecification(DropDownStyleSpecification),
    #[serde(rename = "flow_style")]
    FlowStyleSpecification(FlowStyleSpecification),
    #[serde(rename = "frame_style")]
    FrameStyleSpecification(FrameStyleSpecification),
    #[serde(rename = "graph_style")]
    GraphStyleSpecification(GraphStyleSpecification),
    #[serde(rename = "horizontal_flow_style")]
    HorizontalFlowStyleSpecification(HorizontalFlowStyleSpecification),
    #[serde(rename = "line_style")]
    LineStyleSpecification(LineStyleSpecification),
    #[serde(rename = "image_style")]
    ImageStyleSpecification(ImageStyleSpecification),
    #[serde(rename = "label_style")]
    LabelStyleSpecification(LabelStyleSpecification),
    #[serde(rename = "list_box_style")]
    ListBoxStyleSpecification(ListBoxStyleSpecification),
    #[serde(rename = "progressbar_style")]
    ProgressBarStyleSpecification(ProgressBarStyleSpecification),
    #[serde(rename = "radiobutton_style")]
    RadioButtonStyleSpecification(RadioButtonStyleSpecification),
    #[serde(rename = "horizontal_scrollbar_style")]
    HorizontalScrollBarStyleSpecification(HorizontalScrollBarStyleSpecification),
    #[serde(rename = "vertical_scrollbar_style")]
    VerticalScrollBarStyleSpecification(VerticalScrollBarStyleSpecification),
    #[serde(rename = "scroll_pane_style")]
    ScrollPaneStyleSpecification(ScrollPaneStyleSpecification),
    #[serde(rename = "slider_style")]
    SliderStyleSpecification(SliderStyleSpecification),
    #[serde(rename = "switch_style")]
    SwitchStyleSpecification(SwitchStyleSpecification),
    #[serde(rename = "table_style")]
    TableStyleSpecification(TableStyleSpecification),
    #[serde(rename = "tab_style")]
    TabStyleSpecification(TabStyleSpecification),
    #[serde(rename = "textbox_style")]
    TextBoxStyleSpecification(TextBoxStyleSpecification),
    #[serde(rename = "vertical_flow_style")]
    VerticalFlowStyleSpecification(VerticalFlowStyleSpecification),
    #[serde(rename = "tabbed_pane_style")]
    TabbedPaneStyleSpecification(TabbedPaneStyleSpecification),
    #[serde(rename = "empty_widget_style")]
    EmptyWidgetStyleSpecification(EmptyWidgetStyleSpecification),
    #[serde(rename = "minimap_style")]
    MinimapStyleSpecification(MinimapStyleSpecification),
    #[serde(rename = "technology_slot_style")]
    TechnologySlotStyleSpecification(TechnologySlotStyleSpecification),
    #[serde(rename = "glow_style")]
    GlowStyleSpecification(GlowStyleSpecification),
    #[serde(rename = "speech_bubble_style")]
    SpeechBubbleStyleSpecification(SpeechBubbleStyleSpecification),
    #[serde(rename = "double_slider_style")]
    DoubleSliderStyleSpecification(DoubleSliderStyleSpecification),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StyleWithClickableGraphicalSetSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_controller_selected_hovered_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_click_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_clicked_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_hovered_graphical_set: Option<ElementImageSet>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SurfaceCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    pub property: SurfacePropertyID,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct SurfaceID(pub String);
impl std::fmt::Display for SurfaceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for SurfaceID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct SurfacePropertyID(pub String);
impl std::fmt::Display for SurfacePropertyID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for SurfacePropertyID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SurfaceRenderParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clouds: Option<CloudsEffectProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_night_cycle_color_lookup: Option<DaytimeColorLookupTable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_sprite_clouds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog: Option<FogEffectProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_backdrop: Option<PlatformBackdrop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_dust_background: Option<SpaceDustEffectProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_dust_foreground: Option<SpaceDustEffectProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terrain_tint_effect: Option<GlobalTintEffectProperties>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SwitchStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_label: Option<LabelStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<ButtonStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_background: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_background: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_background: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_label: Option<LabelStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_button_position: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_button_position: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_button_position: Option<u32>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TabStyleSpecification {
    #[serde(flatten)]
    pub parent: StyleWithClickableGraphicalSetSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_horizontal_spacing: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_badge_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_badge_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_badge_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_badge_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_grayscale_picture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_badge_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub increase_height_when_selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_edge_selected_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_graphics_on_edges: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_badge_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_edge_selected_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_badge_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_badge_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_font_color: Option<Color>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TabbedPaneStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_container: Option<TableStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_content_frame: Option<FrameStyleSpecification>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_spacing: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TableStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_row_graphical_set_per_column: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<BorderImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_cell_padding: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_padding: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_alignments: Option<Vec<ColumnAlignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_ordering_ascending_button_style: Option<ButtonStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_ordering_descending_button_style: Option<ButtonStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_widths: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_row_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_row_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_line_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_spacing: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_row_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_column_ordering_ascending_button_style: Option<
        ButtonStyleSpecification,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_column_ordering_descending_button_style: Option<
        ButtonStyleSpecification,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_cell_padding: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odd_row_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_cell_padding: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_clicked_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_hovered_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_row_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_cell_padding: Option<i16>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_line_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_spacing: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wide_as_column_count: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct TechnologyID(pub String);
impl std::fmt::Display for TechnologyID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for TechnologyID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TechnologySlotStyleSpecification {
    #[serde(flatten)]
    pub parent: ButtonStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked_ingredients_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked_overlay: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_background_shadow: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ingredients_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_ingredients_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_handle_style: Option<EmptyWidgetStyleSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted_graphical_set: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted_ingredients_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_ingredients_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_level_band: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_level_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_level_range_band: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_level_range_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredient_icon_overlap: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredient_icon_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredients_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredients_padding: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_band: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_band_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_band_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_offset_x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_offset_y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_range_band: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_range_font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_range_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_range_offset_x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_range_offset_y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_bar: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_bar_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_bar_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_bar_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_bar_shadow: Option<ElementImageSet>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum TechnologyTrigger {
    #[serde(rename = "mine-entity")]
    MineEntityTechnologyTrigger(MineEntityTechnologyTrigger),
    #[serde(rename = "craft-item")]
    CraftItemTechnologyTrigger(CraftItemTechnologyTrigger),
    #[serde(rename = "craft-fluid")]
    CraftFluidTechnologyTrigger(CraftFluidTechnologyTrigger),
    #[serde(rename = "send-item-to-orbit")]
    SendItemToOrbitTechnologyTrigger(SendItemToOrbitTechnologyTrigger),
    #[serde(rename = "capture-spawner")]
    CaptureSpawnerTechnologyTrigger(CaptureSpawnerTechnologyTrigger),
    #[serde(rename = "build-entity")]
    BuildEntityTechnologyTrigger(BuildEntityTechnologyTrigger),
    #[serde(rename = "create-space-platform")]
    CreateSpacePlatformTechnologyTrigger(CreateSpacePlatformTechnologyTrigger),
    #[serde(rename = "scripted")]
    ScriptedTechnologyTrigger(ScriptedTechnologyTrigger),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TechnologyUnit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_formula: Option<MathExpression>,
    pub ingredients: Vec<ResearchIngredient>,
    pub time: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TerritorySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_territory_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub territory_index_expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub territory_variation_expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<EntityID>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TextBoxStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_controller_hovered_background: Option<ElementImageSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text_highlight_error_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text_highlight_ok_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text_highlight_warning_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text_setting: Option<RichTextSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_rich_text_highlight_error_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_rich_text_highlight_ok_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_rich_text_highlight_warning_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_background_color: Option<Color>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ThrowCapsuleAction {
    pub attack_parameters: AttackParameters,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_stack: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ThrusterGraphicsSet {
    #[serde(flatten)]
    pub parent: WorkingVisualisations,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_random_start_frame: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flame: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flame_effect: Option<EffectTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flame_effect_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flame_effect_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flame_effect_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flame_half_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flame_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum ThrusterPerformancePoint {
    Struct0,
    Variant1((f64, f64, f64)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileAndAlpha {
    pub alpha: f64,
    pub tile: TileID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileBasedParticleTints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Color>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileBuildSound {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileBuildabilityRule {
    pub area: SimpleBoundingBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colliding_tiles: Option<TileCollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_on_collision: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_tiles: Option<TileCollisionMaskConnector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileCollisionMaskConnector {
    pub layers: std::collections::HashMap<CollisionLayerID, bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct TileEffectDefinitionID(pub String);
impl std::fmt::Display for TileEffectDefinitionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for TileEffectDefinitionID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct TileID(pub String);
impl std::fmt::Display for TileID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for TileID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum TileIDRestriction {
    TileID(Box<TileID>),
    Variant1((TileID, TileID)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileLightPictures {
    #[serde(flatten)]
    pub parent: TileSpriteLayout,
    pub size: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileMainPictures {
    #[serde(flatten)]
    pub parent: TileSpriteLayout,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
    pub size: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f64>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum TilePosition {
    Struct0,
    Variant1((i32, i32)),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum TileRenderLayer {
    #[serde(rename = "zero")]
    LiteralZero,
    #[serde(rename = "water")]
    LiteralWater,
    #[serde(rename = "water-overlay")]
    LiteralWaterOverlay,
    #[serde(rename = "ground-natural")]
    LiteralGroundNatural,
    #[serde(rename = "ground-artificial")]
    LiteralGroundArtificial,
    #[serde(rename = "top")]
    LiteralTop,
}
impl std::fmt::Display for TileRenderLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                TileRenderLayer::LiteralZero => "zero",
                TileRenderLayer::LiteralWater => "water",
                TileRenderLayer::LiteralWaterOverlay => "water-overlay",
                TileRenderLayer::LiteralGroundNatural => "ground-natural",
                TileRenderLayer::LiteralGroundArtificial => "ground-artificial",
                TileRenderLayer::LiteralTop => "top",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileSpriteLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_length: Option<u8>,
    pub picture: FileName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileSpriteLayoutVariant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spritesheet: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileTransitionSpritesheetLayout {
    #[serde(flatten)]
    pub parent: TileSpriteLayoutVariant,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_effect_mask: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_mask: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_map: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightmap: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_y: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileTransitionVariantLayout {
    #[serde(flatten)]
    pub parent: TileSpriteLayoutVariant,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side: Option<TileSpriteLayoutVariant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner: Option<TileSpriteLayoutVariant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition: Option<TileSpriteLayoutVariant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_transition_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner: Option<TileSpriteLayoutVariant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<TileSpriteLayoutVariant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition: Option<TileSpriteLayoutVariant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_line_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_tile_height: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_offset: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_offset: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileTransitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_effect_color_to_overlay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_waving_effect_on_background_mask: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_waving_effect_on_masks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_effect_mask_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_effect_mask_layout: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_effect_mask_spritesheet: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_layer_group: Option<TileRenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_layer_occludes_light: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_layer_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_layout: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_mask_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_mask_layout: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_mask_spritesheet: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_spritesheet: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_variations_in_group: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_side_weights: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_background_layer_under_tiles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_simple_outer_corner_over_diagonal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_map_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_map_layout: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_map_spritesheet: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_corner_weights: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<TileTransitionSpritesheetLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightmap_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightmap_layout: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightmap_spritesheet: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_layout: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_spritesheet: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_background_layer_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_overlay_layer_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_background_layer_by_tile_layer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_corner_weights: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_layer_group: Option<TileRenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_layer_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_layout: Option<TileTransitionVariantLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_variations_in_group: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_weights: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spritesheet: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_transition_weights: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_patch: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waving_effect_time_scale: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileTransitionsBetweenTransitions {
    #[serde(flatten)]
    pub parent: TileTransitions,
    pub transition_group1: u8,
    pub transition_group2: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileTransitionsToTiles {
    #[serde(flatten)]
    pub parent: TileTransitions,
    pub to_tiles: Vec<TileID>,
    pub transition_group: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileTransitionsVariants {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_transitions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<Vec<TileLightPictures>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<Vec<TileMainPictures>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_background: Option<MaterialTextureParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_light: Option<MaterialTextureParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_texture_height_in_tiles: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_texture_width_in_tiles: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<TileTransitions>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TimeElapsedTipTrigger {
    pub ticks: u32,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TimeSinceLastTipActivationTipTrigger {
    pub ticks: u64,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TintProcessionBezierControlPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity_t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_lower: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_lower_t: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_upper: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_upper_t: Option<Color>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TintProcessionLayer {
    pub frames: Vec<TintProcessionBezierControlPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum TipStatus {
    #[serde(rename = "locked")]
    LiteralLocked,
    #[serde(rename = "optional")]
    LiteralOptional,
    #[serde(rename = "dependencies-not-met")]
    LiteralDependenciesNotMet,
    #[serde(rename = "unlocked")]
    LiteralUnlocked,
    #[serde(rename = "suggested")]
    LiteralSuggested,
    #[serde(rename = "not-to-be-suggested")]
    LiteralNotToBeSuggested,
    #[serde(rename = "completed-without-tutorial")]
    LiteralCompletedWithoutTutorial,
    #[serde(rename = "completed")]
    LiteralCompleted,
}
impl std::fmt::Display for TipStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                TipStatus::LiteralLocked => "locked",
                TipStatus::LiteralOptional => "optional",
                TipStatus::LiteralDependenciesNotMet => "dependencies-not-met",
                TipStatus::LiteralUnlocked => "unlocked",
                TipStatus::LiteralSuggested => "suggested",
                TipStatus::LiteralNotToBeSuggested => "not-to-be-suggested",
                TipStatus::LiteralCompletedWithoutTutorial => {
                    "completed-without-tutorial"
                }
                TipStatus::LiteralCompleted => "completed",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum TipTrigger {
    #[serde(rename = "or")]
    OrTipTrigger(OrTipTrigger),
    #[serde(rename = "and")]
    AndTipTrigger(AndTipTrigger),
    #[serde(rename = "sequence")]
    SequenceTipTrigger(SequenceTipTrigger),
    #[serde(rename = "dependencies-met")]
    DependenciesMetTipTrigger(DependenciesMetTipTrigger),
    #[serde(rename = "time-elapsed")]
    TimeElapsedTipTrigger(TimeElapsedTipTrigger),
    #[serde(rename = "time-since-last-tip-activation")]
    TimeSinceLastTipActivationTipTrigger(TimeSinceLastTipActivationTipTrigger),
    #[serde(rename = "research")]
    ResearchTechnologyTipTrigger(ResearchTechnologyTipTrigger),
    #[serde(rename = "research-with-science-pack")]
    ResearchWithSciencePackTipTrigger(ResearchWithSciencePackTipTrigger),
    #[serde(rename = "unlock-recipe")]
    UnlockRecipeTipTrigger(UnlockRecipeTipTrigger),
    #[serde(rename = "craft-item")]
    CraftItemTipTrigger(CraftItemTipTrigger),
    #[serde(rename = "build-entity")]
    BuildEntityTipTrigger(BuildEntityTipTrigger),
    #[serde(rename = "manual-transfer")]
    ManualTransferTipTrigger(ManualTransferTipTrigger),
    #[serde(rename = "module-transfer")]
    ModuleTransferTipTrigger(ModuleTransferTipTrigger),
    #[serde(rename = "stack-transfer")]
    StackTransferTipTrigger(StackTransferTipTrigger),
    #[serde(rename = "entity-transfer")]
    EntityTransferTipTrigger(EntityTransferTipTrigger),
    #[serde(rename = "drop-item")]
    DropItemTipTrigger(DropItemTipTrigger),
    #[serde(rename = "set-recipe")]
    SetRecipeTipTrigger(SetRecipeTipTrigger),
    #[serde(rename = "set-filter")]
    SetFilterTipTrigger(SetFilterTipTrigger),
    #[serde(rename = "limit-chest")]
    LimitChestTipTrigger(LimitChestTipTrigger),
    #[serde(rename = "use-pipette")]
    UsePipetteTipTrigger(UsePipetteTipTrigger),
    #[serde(rename = "set-logistic-request")]
    SetLogisticRequestTipTrigger(SetLogisticRequestTipTrigger),
    #[serde(rename = "use-confirm")]
    UseConfirmTipTrigger(UseConfirmTipTrigger),
    #[serde(rename = "toggle-show-entity-info")]
    ToggleShowEntityInfoTipTrigger(ToggleShowEntityInfoTipTrigger),
    #[serde(rename = "generating-power")]
    GeneratingPowerTipTrigger(GeneratingPowerTipTrigger),
    #[serde(rename = "low-power")]
    LowPowerTipTrigger(LowPowerTipTrigger),
    #[serde(rename = "paste-entity-settings")]
    PasteEntitySettingsTipTrigger(PasteEntitySettingsTipTrigger),
    #[serde(rename = "fast-replace")]
    FastReplaceTipTrigger(FastReplaceTipTrigger),
    #[serde(rename = "group-attack")]
    GroupAttackTipTrigger(GroupAttackTipTrigger),
    #[serde(rename = "fast-belt-bend")]
    FastBeltBendTipTrigger(FastBeltBendTipTrigger),
    #[serde(rename = "belt-traverse")]
    BeltTraverseTipTrigger(BeltTraverseTipTrigger),
    #[serde(rename = "place-equipment")]
    PlaceEquipmentTipTrigger(PlaceEquipmentTipTrigger),
    #[serde(rename = "clear-cursor")]
    ClearCursorTipTrigger(ClearCursorTipTrigger),
    #[serde(rename = "rotate-entity")]
    RotateEntityTipTrigger(RotateEntityTipTrigger),
    #[serde(rename = "flip-entity")]
    FlipEntityTipTrigger(FlipEntityTipTrigger),
    #[serde(rename = "alternative-build")]
    AlternativeBuildTipTrigger(AlternativeBuildTipTrigger),
    #[serde(rename = "gate-over-rail-build")]
    GateOverRailBuildTipTrigger(GateOverRailBuildTipTrigger),
    #[serde(rename = "manual-wire-drag")]
    ManualWireDragTipTrigger(ManualWireDragTipTrigger),
    #[serde(rename = "shoot")]
    ShootTipTrigger(ShootTipTrigger),
    #[serde(rename = "change-surface")]
    ChangeSurfaceTipTrigger(ChangeSurfaceTipTrigger),
    #[serde(rename = "apply-starter-pack")]
    ApplyStarterPackTipTrigger(ApplyStarterPackTipTrigger),
    #[serde(rename = "mine-item-by-robot")]
    MineItemByRobotTipTrigger(MineItemByRobotTipTrigger),
    #[serde(rename = "build-entity-by-robot")]
    BuildEntityByRobotTipTrigger(BuildEntityByRobotTipTrigger),
    #[serde(rename = "plan-train-path")]
    PlanTrainPathTipTrigger(PlanTrainPathTipTrigger),
    #[serde(rename = "use-rail-planner")]
    UseRailPlannerTipTrigger(UseRailPlannerTipTrigger),
    #[serde(rename = "toggle-rail-layer")]
    ToggleRailLayerTipTrigger(ToggleRailLayerTipTrigger),
    #[serde(rename = "enter-vehicle")]
    EnterVehicleTipTrigger(EnterVehicleTipTrigger),
    #[serde(rename = "send-spidertron")]
    SendSpidertronTipTrigger(SendSpidertronTipTrigger),
    #[serde(rename = "activate-paste")]
    ActivatePasteTipTrigger(ActivatePasteTipTrigger),
    #[serde(rename = "kill")]
    KillTipTrigger(KillTipTrigger),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ToggleRailLayerTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ToggleShowEntityInfoTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TrainBrakingForceBonusModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TrainPathFinderConstants {
    pub signal_reserved_by_circuit_network_penalty: u32,
    pub stopped_manually_controlled_train_penalty: u32,
    pub stopped_manually_controlled_train_without_passenger_penalty: u32,
    pub train_arriving_to_signal_penalty: u32,
    pub train_arriving_to_station_penalty: u32,
    pub train_auto_without_schedule_penalty: u32,
    pub train_in_station_penalty: u32,
    pub train_in_station_with_no_other_valid_stops_in_schedule: u32,
    pub train_stop_penalty: u32,
    pub train_waiting_at_signal_penalty: u32,
    pub train_waiting_at_signal_tick_multiplier_penalty: f64,
    pub train_with_no_path_penalty: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TrainStopDrawingBoxes {
    pub east: BoundingBox,
    pub north: BoundingBox,
    pub south: BoundingBox,
    pub west: BoundingBox,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TrainStopLight {
    pub light: LightDefinition,
    pub picture: Sprite4Way,
    pub red_picture: Sprite4Way,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TrainVisualizationConstants {
    pub box_length: f64,
    pub box_width: f64,
    pub connection_distance: f64,
    pub final_margin: f64,
    pub joint_distance: f64,
    pub last_box_color: Color,
    pub last_reverse_box_color: Color,
    pub not_last_box_color: Color,
    pub reverse_box_color: Color,
    pub stock_number_scale: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TransitionApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_offset: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TransportBeltAnimationSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate: Option<bool>,
    pub animation_set: RotatedAnimation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belt_reader: Option<Vec<BeltReaderLayer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_east_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_east_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_north_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_north_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_south_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_south_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_west_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_west_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_east_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_east_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_north_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_north_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_south_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_south_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_west_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_west_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_index_frozen: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TransportBeltAnimationSetWithCorners {
    #[serde(flatten)]
    pub parent: TransportBeltAnimationSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_to_north_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_to_north_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_to_south_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_to_south_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_to_east_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_to_east_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_to_west_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_to_west_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_to_east_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_to_east_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_to_west_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_to_west_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_to_north_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_to_north_index_frozen: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_to_south_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_to_south_index_frozen: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TransportBeltConnectorFrame {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_back_patch: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_front_patch: Option<SpriteVariations>,
    pub frame_main: AnimationVariations,
    pub frame_main_scanner: Animation,
    pub frame_main_scanner_cross_horizontal_end_shift: Vector,
    pub frame_main_scanner_cross_horizontal_rotation: f64,
    pub frame_main_scanner_cross_horizontal_start_shift: Vector,
    pub frame_main_scanner_cross_horizontal_y_scale: f64,
    pub frame_main_scanner_cross_vertical_end_shift: Vector,
    pub frame_main_scanner_cross_vertical_rotation: f64,
    pub frame_main_scanner_cross_vertical_start_shift: Vector,
    pub frame_main_scanner_cross_vertical_y_scale: f64,
    pub frame_main_scanner_horizontal_end_shift: Vector,
    pub frame_main_scanner_horizontal_rotation: f64,
    pub frame_main_scanner_horizontal_start_shift: Vector,
    pub frame_main_scanner_horizontal_y_scale: f64,
    pub frame_main_scanner_movement_speed: f64,
    pub frame_main_scanner_nw_ne: Animation,
    pub frame_main_scanner_sw_se: Animation,
    pub frame_main_scanner_vertical_end_shift: Vector,
    pub frame_main_scanner_vertical_rotation: f64,
    pub frame_main_scanner_vertical_start_shift: Vector,
    pub frame_main_scanner_vertical_y_scale: f64,
    pub frame_shadow: AnimationVariations,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TravelToSpacePlatformsModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TreeGrowth {
    pub growth_warp: Sprite,
    pub harvest_alpha: Sprite,
    pub harvest_warp: Sprite,
    pub progress_exponent: f64,
    pub shadow_alpha: Sprite,
    pub shadow_warp: Sprite,
    pub trunk_alpha: Sprite,
    pub trunk_warp: Sprite,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TreeVariation {
    pub branch_generation: CreateParticleTriggerEffectItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_when_damaged: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_when_destroyed: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_when_mined_automatically: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_when_mined_manually: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_shadow_distortion_beginning_at_frame: Option<u32>,
    pub leaf_generation: CreateParticleTriggerEffectItem,
    pub leaves: Animation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves_when_damaged: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves_when_destroyed: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves_when_mined_automatically: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves_when_mined_manually: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<Animation>,
    pub trunk: Animation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_layer_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum Trigger {
    Variant0(serde_json::Value),
    Array1(Vec<serde_json::Value>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(tag = "type")]
pub enum TriggerDelivery {
    #[serde(rename = "instant")]
    InstantTriggerDelivery(InstantTriggerDelivery),
    #[serde(rename = "projectile")]
    ProjectileTriggerDelivery(ProjectileTriggerDelivery),
    #[serde(rename = "beam")]
    BeamTriggerDelivery(BeamTriggerDelivery),
    #[serde(rename = "stream")]
    StreamTriggerDelivery(StreamTriggerDelivery),
    #[serde(rename = "artillery")]
    ArtilleryTriggerDelivery(ArtilleryTriggerDelivery),
    #[serde(rename = "chain")]
    ChainTriggerDelivery(ChainTriggerDelivery),
    #[serde(rename = "delayed")]
    DelayedTriggerDelivery(DelayedTriggerDelivery),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TriggerDeliveryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_effects: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_effects: Option<TriggerEffect>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum TriggerEffect {
    Variant0(serde_json::Value),
    Array1(Vec<serde_json::Value>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TriggerEffectItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affects_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_type_filters: Option<DamageTypeFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count_deviation: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_tooltip: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TriggerEffectWithCooldown {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_cooldown: Option<f64>,
    pub effect: TriggerEffect,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_distance_cooldown: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_time_cooldown: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_cooldown: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TriggerItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_delivery: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_flags: Option<EntityPrototypeFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<ForceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_collision_condition: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_target_mask: Option<TriggerTargetMask>,
}
pub type TriggerTargetMask = Vec<String>;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct TrivialSmokeID(pub String);
impl std::fmt::Display for TrivialSmokeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for TrivialSmokeID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TurretAttackModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub modifier: f64,
    pub turret_id: EntityID,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TurretBaseVisualisation {
    pub animation: Animation4Way,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_when_frozen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_when_has_ammo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_when_has_energy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_when_no_ammo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_when_no_energy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_when_not_frozen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_states: Option<Vec<TurretState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_draw_order: Option<i8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TurretGraphicsSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_visualisation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TurretSpecialEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attacking_falloff: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attacking_max_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attacking_min_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<TurretSpecialEffectCenter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub falloff: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_radius: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum TurretSpecialEffectCenter {
    Struct0,
    Vector(Box<Vector>),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum TurretState {
    #[serde(rename = "folded")]
    LiteralFolded,
    #[serde(rename = "preparing")]
    LiteralPreparing,
    #[serde(rename = "prepared")]
    LiteralPrepared,
    #[serde(rename = "starting-attack")]
    LiteralStartingAttack,
    #[serde(rename = "attacking")]
    LiteralAttacking,
    #[serde(rename = "ending-attack")]
    LiteralEndingAttack,
    #[serde(rename = "rotate-for-folding")]
    LiteralRotateForFolding,
    #[serde(rename = "folding")]
    LiteralFolding,
}
impl std::fmt::Display for TurretState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                TurretState::LiteralFolded => "folded",
                TurretState::LiteralPreparing => "preparing",
                TurretState::LiteralPrepared => "prepared",
                TurretState::LiteralStartingAttack => "starting-attack",
                TurretState::LiteralAttacking => "attacking",
                TurretState::LiteralEndingAttack => "ending-attack",
                TurretState::LiteralRotateForFolding => "rotate-for-folding",
                TurretState::LiteralFolding => "folding",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UndergroundBeltStructure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_in: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_in_side_loading: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_out: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_out_side_loading: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch_in: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch_out: Option<Sprite4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UnitAISettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_try_return_to_spawner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_when_commands_fail: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_separation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_attacks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_resolution_modifier: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_group: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strafe_settings: Option<PrototypeStrafeSettings>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UnitAlternativeFrameSequence {
    pub attacking_animation_speed: f64,
    pub attacking_frame_sequence: Vec<u16>,
    pub back_to_walk_animation_speed: f64,
    pub back_to_walk_frame_sequence: Vec<u16>,
    pub cooldown_animation_speed: f64,
    pub cooldown_frame_sequence: Vec<u16>,
    pub prepared_animation_speed: f64,
    pub prepared_frame_sequence: Vec<u16>,
    pub warmup2_frame_sequence: Vec<u16>,
    pub warmup_animation_speed: f64,
    pub warmup_frame_sequence: Vec<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UnitGroupSettings {
    pub max_gathering_unit_groups: u32,
    pub max_group_gathering_time: u32,
    pub max_group_member_fallback_factor: f64,
    pub max_group_radius: f64,
    pub max_group_slowdown_factor: f64,
    pub max_member_slowdown_when_ahead: f64,
    pub max_member_speedup_when_behind: f64,
    pub max_unit_group_size: u32,
    pub max_wait_time_for_late_members: u32,
    pub member_disown_distance: f64,
    pub min_group_gathering_time: u32,
    pub min_group_radius: f64,
    pub tick_tolerance_when_member_arrives: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum UnitSpawnDefinition {
    Struct0,
    Variant1((EntityID, Vec<SpawnPoint>)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UnlockLogisticNetworkModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UnlockQualityModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    pub quality: QualityID,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UnlockRecipeModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    pub recipe: RecipeID,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UnlockRecipeTipTrigger {
    pub recipe: RecipeID,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UnlockSpaceLocationModifier {
    #[serde(flatten)]
    pub parent: BaseModifier,
    pub space_location: SpaceLocationID,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UseConfirmTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UseOnSelfCapsuleAction {
    pub attack_parameters: AttackParameters,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_stack: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UsePipetteTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UseRailPlannerTipTrigger {
    #[serde(flatten)]
    pub parent: CountBasedTipTrigger,
    pub build_mode: BuildMode,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum ValveMode {
    #[serde(rename = "one-way")]
    LiteralOneWay,
    #[serde(rename = "overflow")]
    LiteralOverflow,
    #[serde(rename = "top-up")]
    LiteralTopUp,
}
impl std::fmt::Display for ValveMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                ValveMode::LiteralOneWay => "one-way",
                ValveMode::LiteralOverflow => "overflow",
                ValveMode::LiteralTopUp => "top-up",
            },
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum VariableAmbientSoundCompositionMode {
    #[serde(rename = "randomized")]
    LiteralRandomized,
    #[serde(rename = "semi-randomized")]
    LiteralSemiRandomized,
    #[serde(rename = "shuffled")]
    LiteralShuffled,
    #[serde(rename = "layer-controlled")]
    LiteralLayerControlled,
}
impl std::fmt::Display for VariableAmbientSoundCompositionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                VariableAmbientSoundCompositionMode::LiteralRandomized => "randomized",
                VariableAmbientSoundCompositionMode::LiteralSemiRandomized => {
                    "semi-randomized"
                }
                VariableAmbientSoundCompositionMode::LiteralShuffled => "shuffled",
                VariableAmbientSoundCompositionMode::LiteralLayerControlled => {
                    "layer-controlled"
                }
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VariableAmbientSoundLayer {
    pub composition_mode: VariableAmbientSoundCompositionMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_layer_sample_mapping: Option<Vec<Vec<u8>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_end_sample: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_start_sample: Option<bool>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_sublayers: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_length: Option<RandomRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sublayer_offset: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sublayer_starting_offset: Option<serde_json::Value>,
    pub variants: Vec<Sound>,
}
pub type VariableAmbientSoundLayerSample = (String, u32);
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VariableAmbientSoundLayerStateProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_pause: Option<RandomRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_repetitions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_between_repetitions: Option<RandomRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_between_samples: Option<RandomRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_length: Option<RandomRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silence_instead_of_sample_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_pause: Option<RandomRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VariableAmbientSoundNextStateConditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_sample: Option<VariableAmbientSoundLayerSample>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<String>,
    pub weight: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VariableAmbientSoundNextStateItem {
    pub conditions: VariableAmbientSoundNextStateConditions,
    pub state: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum VariableAmbientSoundNextStateTrigger {
    #[serde(rename = "layers-finished")]
    LiteralLayersFinished,
    #[serde(rename = "duration")]
    LiteralDuration,
}
impl std::fmt::Display for VariableAmbientSoundNextStateTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                VariableAmbientSoundNextStateTrigger::LiteralLayersFinished => {
                    "layers-finished"
                }
                VariableAmbientSoundNextStateTrigger::LiteralDuration => "duration",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VariableAmbientSoundState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_pause: Option<RandomRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers_properties: Option<Vec<VariableAmbientSoundLayerStateProperties>>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_state_layers_finished_layers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_state_trigger: Option<VariableAmbientSoundNextStateTrigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_states: Option<Vec<VariableAmbientSoundNextStateItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_enabled_layers: Option<RandomRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_pause: Option<RandomRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_duration_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<VariableAmbientSoundStateType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum VariableAmbientSoundStateType {
    #[serde(rename = "regular")]
    LiteralRegular,
    #[serde(rename = "intermezzo")]
    LiteralIntermezzo,
    #[serde(rename = "final")]
    LiteralFinal,
    #[serde(rename = "stop")]
    LiteralStop,
}
impl std::fmt::Display for VariableAmbientSoundStateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                VariableAmbientSoundStateType::LiteralRegular => "regular",
                VariableAmbientSoundStateType::LiteralIntermezzo => "intermezzo",
                VariableAmbientSoundStateType::LiteralFinal => "final",
                VariableAmbientSoundStateType::LiteralStop => "stop",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VariableAmbientSoundVariableSound {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_samples: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermezzo: Option<Sound>,
    pub layers: Vec<VariableAmbientSoundLayer>,
    pub length_seconds: u32,
    pub states: Vec<VariableAmbientSoundState>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum Vector {
    Struct0,
    Variant1((f64, f64)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum Vector3D {
    Struct0,
    Variant1((f64, f64, f64)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum Vector4f {
    Struct0,
    Variant1((f64, f64, f64, f64)),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VectorRotation {
    pub frames: Vec<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VehicleLogisticsModifier {
    #[serde(flatten)]
    pub parent: BoolModifier,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
#[serde(rename_all = "kebab-case")]
pub enum VerticalAlign {
    #[serde(rename = "top")]
    LiteralTop,
    #[serde(rename = "center")]
    LiteralCenter,
    #[serde(rename = "bottom")]
    LiteralBottom,
}
impl std::fmt::Display for VerticalAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                VerticalAlign::LiteralTop => "top",
                VerticalAlign::LiteralCenter => "center",
                VerticalAlign::LiteralBottom => "bottom",
            },
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VerticalFlowStyleSpecification {
    #[serde(flatten)]
    pub parent: BaseStyleSpecification,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_spacing: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VerticalScrollBarStyleSpecification {
    #[serde(flatten)]
    pub parent: ScrollBarStyleSpecification,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Visitable)]
pub struct VirtualSignalID(pub String);
impl std::fmt::Display for VirtualSignalID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for VirtualSignalID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VisualState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    pub duration: u8,
    pub name: String,
    pub next_active: String,
    pub next_inactive: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VoidEnergySource {
    #[serde(flatten)]
    pub parent: BaseEnergySource,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WallPictures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_left_down: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_right_down: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_left: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_right: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filling: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gate_connection_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_horizontal: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straight_vertical: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_up: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_connection_patch: Option<Sprite4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WaterReflectionDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_to_variation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WaterTileEffectParameters {
    pub animation_scale: serde_json::Value,
    pub animation_speed: f64,
    pub dark_threshold: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub far_zoom: Option<f64>,
    pub foam_color: Color,
    pub foam_color_multiplier: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightmap_alpha: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub near_zoom: Option<f64>,
    pub reflection_threshold: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_texture_variations_columns: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_texture_variations_rows: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shader_variation: Option<EffectVariation>,
    pub specular_lightness: Color,
    pub specular_threshold: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_variations_columns: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_variations_rows: Option<u8>,
    pub textures: Vec<EffectTexture>,
    pub tick_scale: f64,
}
pub type Weight = f64;
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WireConnectionPoint {
    pub shadow: WirePosition,
    pub wire: WirePosition,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WirePosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copper: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WorkerRobotBatteryModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WorkerRobotSpeedModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WorkerRobotStorageModifier {
    #[serde(flatten)]
    pub parent: SimpleModifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_icon: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_icon_overlay_constant: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum WorkingSound {
    Struct0,
    Sound(Box<Sound>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WorkingVisualisation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_to_waypoint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_draw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated_shift: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_recipe_tint: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_tint: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_speed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_in_states: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_when_state_filter_matches: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_fog_mask: Option<FogMaskShapeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_secondary_draw_order: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_in_animated_shift_during_transition: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_in_animated_shift_during_waypoint_stop: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fadeout: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog_mask: Option<FogMaskShapeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_based_on_shift_animation_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_drill_scorch_mark: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_fog_mask: Option<FogMaskShapeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub north_secondary_draw_order: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scorch_mark_fade_in_frames: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scorch_mark_fade_out_duration: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scorch_mark_lifetime: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_draw_order: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_fog_mask: Option<FogMaskShapeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_secondary_draw_order: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_fadeout: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_fog_mask: Option<FogMaskShapeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_position: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub west_secondary_draw_order: Option<i8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WorkingVisualisations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_draw_idle_animation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_recipe_tint: Option<GlobalRecipeTints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_animation: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_not_set_tint: Option<GlobalRecipeTints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_animation_transition_duration: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_animation_waypoint_stop_duration: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_animation_waypoints: Option<ShiftAnimationWaypoints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<VisualState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_colors: Option<StatusColors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_visualisations: Option<Vec<WorkingVisualisation>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
#[serde(untagged)]
pub enum WorldAmbientSoundDefinition {
    Struct0,
    Sound(Box<Sound>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AccumulatorPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargable_graphics: Option<ChargableGraphics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub energy_source: ElectricEnergySource,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AchievementPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_without_fight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steam_stats_name: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AchievementPrototypeWithCondition {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective_condition: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ActiveDefenseEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    pub attack_parameters: AttackParameters,
    pub automatic: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ActiveTriggerPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AgriculturalTowerPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_seeds: Option<Vec<ItemID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_effects: Option<EffectTypeLimitation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_module_categories: Option<Vec<ModuleCategoryID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_extending_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_extending_sound_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub central_orienting_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub central_orienting_sound_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    pub crane: AgriculturalCraneProperties,
    pub crane_energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_receiver: Option<EffectReceiver>,
    pub energy_source: EnergySource,
    pub energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<CraftingMachineGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grappler_extending_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grappler_extending_sound_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grappler_orienting_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grappler_orienting_sound_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_area_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_grid_tile_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvesting_procedure_points: Option<Vec<Vector3D>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvesting_sound: Option<Sound>,
    pub input_inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_slots: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_inventory_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planting_procedure_points: Option<Vec<Vector3D>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planting_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_module_slots: Option<bool>,
    pub radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_visualisation_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_growth_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomize_planting_tile: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AirbornePollutantPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    pub affects_evolution: bool,
    pub affects_water_tint: bool,
    pub chart_color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damages_trees: Option<bool>,
    pub icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localised_name_with_amount: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AmbientSound {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_planets: Option<Vec<SpaceLocationID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_surface_names: Option<Vec<String>>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planets: Option<Vec<SpaceLocationID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_on_all_surfaces: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub track_type: AmbientSoundType,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_sound: Option<VariableAmbientSoundVariableSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AmmoCategory {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_gui_order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AmmoItemPrototype {
    #[serde(flatten)]
    pub parent: ItemPrototype,
    pub ammo_category: AmmoCategoryID,
    pub ammo_type: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magazine_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reload_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shoot_protected: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AmmoTurretPrototype {
    #[serde(flatten)]
    pub parent: TurretPrototype,
    pub automated_ammo_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_per_shot: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_source: Option<ElectricEnergySource>,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_with_no_ammo: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AnimationPrototype {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_forced_downscale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_runtime_tint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_special_effect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_x: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_y: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_as_glow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_as_light: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_as_shadow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<FileName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<SpriteFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_sequence: Option<AnimationFrameSequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_sdf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_colors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Animation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_per_file: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_in_minimal_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_advance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mipmap_count: Option<u8>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<(u16, u16)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premul_alpha: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<SpritePriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_shift: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_mode: Option<AnimationRunMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slice: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripes: Option<Vec<Stripe>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<SpriteUsageSurfaceHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_as_overlay: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<SpriteUsageHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ArithmeticCombinatorPrototype {
    #[serde(flatten)]
    pub parent: CombinatorPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divide_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_shift_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minus_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modulo_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiply_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plus_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_shift_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xor_symbol_sprites: Option<Sprite4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ArmorPrototype {
    #[serde(flatten)]
    pub parent: ToolPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawing_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment_grid: Option<EquipmentGridID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_size_bonus: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moving_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provides_flight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resistances: Option<Vec<Resistance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub takeoff_sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ArrowPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    pub arrow_picture: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blinking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ArtilleryFlarePrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_death_ticks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_in_water_trigger_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_frame_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_speed: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_vertical_speed: Option<f64>,
    pub life_time: u16,
    pub map_color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_modifier_when_on_ground: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_trigger_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_trigger_effect_frequency: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer_when_on_ground: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadows: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_category: Option<AmmoCategoryID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shots_per_flare: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ArtilleryProjectilePrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_from_ground: Option<f64>,
    pub map_color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite>,
    pub reveal_map: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotatable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ArtilleryTurretPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_when_attacking: Option<bool>,
    pub ammo_stack_limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_ammo_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_picture: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_picture_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_picture_secondary_draw_order: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_barrel_light_direction: Option<Vector3D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_barrel_pictures: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_barrel_recoil_shiftings: Option<Vec<Vector3D>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_barrel_recoil_shiftings_load_correction_matrix: Option<Vec<Vector3D>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_base_pictures: Option<RotatedSprite>,
    pub cannon_base_shift: Vector3D,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_parking_frame_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_parking_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_automatic_firing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub gun: ItemID,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_military_target: Option<bool>,
    pub manual_range_modifier: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotating_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_after_shooting_cooldown: Option<u16>,
    pub turret_rotation_speed: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ArtilleryWagonPrototype {
    #[serde(flatten)]
    pub parent: RollingStockPrototype,
    pub ammo_stack_limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_ammo_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_barrel_light_direction: Option<Vector3D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_barrel_pictures: Option<RollingStockRotatedSlopedGraphics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_barrel_recoil_shiftings: Option<Vec<Vector3D>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_barrel_recoil_shiftings_load_correction_matrix: Option<Vec<Vector3D>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_base_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_base_pictures: Option<RollingStockRotatedSlopedGraphics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_base_shift_when_horizontal: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_base_shift_when_vertical: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_parking_frame_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon_parking_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_automatic_firing: Option<bool>,
    pub gun: ItemID,
    pub inventory_size: u16,
    pub manual_range_modifier: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotating_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_after_shooting_cooldown: Option<u16>,
    pub turret_rotation_speed: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AssemblingMachinePrototype {
    #[serde(flatten)]
    pub parent: CraftingMachinePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector_flipped: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_recipe_finished_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_working_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_when_recipe_not_researched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_quality: Option<QualityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_recipe: Option<RecipeID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_boxes_off_when_no_fluid_recipe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui_title_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredient_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_item_product_count: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AsteroidChunkPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_trigger_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<AsteroidGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_from_signal_gui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minable: Option<MinableProperties>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AsteroidCollectorPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_angular_speed_cap_base: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_angular_speed_cap_quality_scaling: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_color_gradient: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_count_base: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_count_quality_scaling: Option<u32>,
    pub arm_energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_extend_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_inventory_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_inventory_size_quality_increase: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_retract_sound: Option<Sound>,
    pub arm_slow_energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_speed_base: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_speed_quality_scaling: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_box_offset: Option<f64>,
    pub collection_radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub energy_source: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_usage_quality_scaling: Option<f64>,
    pub graphics_set: AsteroidCollectorGraphicsSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_collection_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_items_display_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_items_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_items_spread: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_size_quality_increase: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_arm_swing_segment_retraction: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub munch_sound: Option<Sound>,
    pub passive_energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_visualisation_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tether_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpowered_arm_speed_scale: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AsteroidPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_per_hp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissions_per_second: Option<
        std::collections::HashMap<AirbornePollutantID, f64>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<AsteroidGraphicsSet>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct AutoplaceControl {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_disabled: Option<bool>,
    pub category: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_to_fight_achievements: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub richness: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BatteryEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeaconPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_effects: Option<EffectTypeLimitation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_module_categories: Option<Vec<ModuleCategoryID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_picture: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beacon_counter: Option<serde_json::Value>,
    pub distribution_effectivity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_effectivity_bonus_per_quality_level: Option<f64>,
    pub energy_source: serde_json::Value,
    pub energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<BeaconGraphicsSet>,
    pub module_slots: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perceived_performance: Option<PerceivedPerformance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_module_slots: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_supply_area_distance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_visualisation_picture: Option<Sprite>,
    pub supply_area_distance: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeamPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_triggered_automatically: Option<bool>,
    pub damage_interval: u32,
    pub graphics_set: BeamGraphicsSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_target_offset: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_offset: Option<Vector>,
    pub width: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BeltImmunityEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    pub energy_consumption: Energy,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BlueprintBookPrototype {
    #[serde(flatten)]
    pub parent: ItemWithInventoryPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_label_for_cursor_render: Option<bool>,
    pub inventory_size: serde_json::Value,
    pub stack_size: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BlueprintItemPrototype {
    #[serde(flatten)]
    pub parent: SelectionToolPrototype,
    pub alt_select: SelectionModeData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_include_tiles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_label_for_cursor_render: Option<bool>,
    pub select: SelectionModeData,
    pub stack_size: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BoilerPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub burning_cooldown: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub energy_consumption: Energy,
    pub energy_source: EnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_flicker_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_glow_flicker_enabled: Option<bool>,
    pub fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<serde_json::Value>,
    pub output_fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<BoilerPictureSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_temperature: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BuildEntityAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_to_one_game: Option<bool>,
    pub to_build: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub within: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BurnerGeneratorPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_draw_idle_animation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation4Way>,
    pub burner: BurnerEnergySource,
    pub energy_source: ElectricEnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_animation: Option<Animation4Way>,
    pub max_power_output: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perceived_performance: Option<PerceivedPerformance>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct BurnerUsagePrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    pub accepted_fuel_key: String,
    pub burned_in_key: String,
    pub empty_slot_caption: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_slot_description: Option<String>,
    pub empty_slot_sprite: Sprite,
    pub icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_fuel_status: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CapsulePrototype {
    #[serde(flatten)]
    pub parent: ItemPrototype,
    pub capsule_action: CapsuleAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_color: Option<Color>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CaptureRobotPrototype {
    #[serde(flatten)]
    pub parent: FlyingRobotPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_radius: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CarPrototype {
    #[serde(flatten)]
    pub parent: VehiclePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sort_inventory: Option<bool>,
    pub consumption: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darkness_to_render_light_animation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driving_sound_volume_modifier: Option<f64>,
    pub effectivity: f64,
    pub energy_source: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guns: Option<Vec<ItemID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_belt_immunity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immune_to_all_impacts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immune_to_cliff_impacts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immune_to_rock_impacts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immune_to_tree_impacts: Option<bool>,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_animation: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    pub rotation_snap_angle: f64,
    pub rotation_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_no_fuel: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tank_driving: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_particle_triggers: Option<FootstepTriggerEffectList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trash_inventory_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turret_animation: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turret_return_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turret_rotation_speed: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CargoBayPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unloading: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_grid_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<CargoBayConnectableGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_direction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatch_definitions: Option<Vec<CargoHatchDefinition>>,
    pub inventory_size_bonus: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_graphics_set: Option<CargoBayConnectableGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_unloading_distance_limit: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CargoLandingPadPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub cargo_station_parameters: CargoStationParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<CargoBayConnectableGraphicsSet>,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_range: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_visualisation_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_visualisation_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_door: Option<RobotDoorSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trash_inventory_size: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CargoPodPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_graphic: Option<ProcessionGraphic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shadow_graphic: Option<ProcessionGraphic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_trigger: Option<Trigger>,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procession_audio_catalogue: Option<ProcessionAudioCatalogue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procession_graphic_catalogue: Option<ProcessionGraphicCatalogue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_slave_entity: Option<EntityID>,
    pub spawned_container: EntityID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CargoWagonPrototype {
    #[serde(flatten)]
    pub parent: RollingStockPrototype,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_inventory_size: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ChainActiveTriggerPrototype {
    #[serde(flatten)]
    pub parent: ActiveTriggerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork_chance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork_chance_increase_per_quality_level: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_delay_ticks: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_forks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_forks_per_jump: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_jumps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_range: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_range_per_jump: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ChangedSurfaceAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterCorpsePrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_picture_mapping: Option<std::collections::HashMap<ItemID, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    pub time_to_live: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CharacterPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub animations: Vec<CharacterArmorAnimation>,
    pub build_distance: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_corpse: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crafting_categories: Option<Vec<RecipeCategoryID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crafting_speed: Option<f64>,
    pub damage_hit_tint: Color,
    pub distance_per_frame: f64,
    pub drop_item_distance: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_vehicle_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flying_bob_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flying_collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footprint_particles: Option<Vec<FootprintParticle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footstep_particle_triggers: Option<FootstepTriggerEffectList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounded_landing_search_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guns_inventory_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_belt_immunity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat: Option<Sound>,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_military_target: Option<bool>,
    pub item_pickup_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_footprint_frames: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_footprint_offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    pub loot_pickup_distance: f64,
    pub maximum_corner_sliding_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_categories: Option<Vec<ResourceCategoryID>>,
    pub mining_speed: f64,
    pub mining_with_tool_particles_animation_positions: Vec<f64>,
    pub moving_sound_animation_positions: Vec<f64>,
    pub reach_distance: u32,
    pub reach_resource_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respawn_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_footprint_frames: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_footprint_offset: Option<Vector>,
    pub running_sound_animation_positions: Vec<f64>,
    pub running_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_footstep_particle_triggers: Option<FootstepTriggerEffectList>,
    pub ticks_to_keep_aiming_direction: u32,
    pub ticks_to_keep_gun: u32,
    pub ticks_to_stay_in_combat: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_attack_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_attack_result: Option<Trigger>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CliffPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliff_explosive: Option<ItemID>,
    pub grid_offset: Vector,
    pub grid_size: Vector,
    pub orientations: OrientedCliffPrototypeSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_as_crater: Option<CraterPlacementDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CollisionLayerPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CombatRobotCountAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CombatRobotPrototype {
    #[serde(flatten)]
    pub parent: FlyingRobotPrototype,
    pub attack_parameters: AttackParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follows_player: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_motion: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_separation_force: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_from_player: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separation_force_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separation_range: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_idle: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_in_motion: Option<RotatedAnimation>,
    pub time_to_live: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CombinatorPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub active_energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_led_hold_time: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_led_light: Option<LightDefinition>,
    pub activity_led_light_offsets: (Vector, Vector, Vector, Vector),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_led_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissions_per_second: Option<
        std::collections::HashMap<AirbornePollutantID, f64>,
    >,
    pub energy_source: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite4Way>,
    pub input_connection_bounding_box: BoundingBox,
    pub input_connection_points: (
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    ),
    pub output_connection_bounding_box: BoundingBox,
    pub output_connection_points: (
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    ),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_light: Option<LightDefinition>,
    pub screen_light_offsets: (Vector, Vector, Vector, Vector),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprites: Option<Sprite4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CompleteObjectiveAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototypeWithCondition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub within: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ConstantCombinatorPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_led_light: Option<LightDefinition>,
    pub activity_led_light_offsets: (Vector, Vector, Vector, Vector),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_led_sprites: Option<Sprite4Way>,
    pub circuit_wire_connection_points: (
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    ),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprites: Option<Sprite4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ConstructWithRobotsAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    pub limited_to_one_game: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more_than_manually: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ConstructionRobotPrototype {
    #[serde(flatten)]
    pub parent: RobotWithLogisticInterfacePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    pub construction_vector: Vector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mined_sound_volume_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repairing_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_working: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparks: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_light: Option<LightDefinition>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ContainerPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<Vec<CircuitConnectorDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_status: Option<EntityStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_properties: Option<InventoryWithCustomStackSizeSpecification>,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_weight_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_inventory_size: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CopyPasteToolPrototype {
    #[serde(flatten)]
    pub parent: SelectionToolPrototype,
    pub alt_select: SelectionModeData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_include_tiles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cuts: Option<bool>,
    pub select: SelectionModeData,
    pub stack_size: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CorpsePrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<RotatedAnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_overlay: Option<RotatedAnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_overlay_final_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_overlay_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_setup_collision_box: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decay_animation: Option<RotatedAnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decay_frame_transition_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_shuffle: Option<Vec<Vec<u16>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_patch: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_patch_decay: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_patch_fade_in_delay: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_patch_fade_in_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_patch_fade_out_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_patch_fade_out_start: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_patch_higher: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_patch_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_from_tile_building: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_on_entity_placement: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_on_tile_placement: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shuffle_directions_at_frame: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_before_removed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_before_shading_off: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_layer_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater_patch: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_decay_layer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_tile_color_for_ground_patch_tint: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CraftingMachinePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_effects: Option<EffectTypeLimitation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_module_categories: Option<Vec<ModuleCategoryID>>,
    pub crafting_categories: Vec<RecipeCategoryID>,
    pub crafting_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crafting_speed_quality_multiplier: Option<
        std::collections::HashMap<QualityID, f64>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_entity_info_icon_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_receiver: Option<EffectReceiver>,
    pub energy_source: EnergySource,
    pub energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_usage_quality_multiplier: Option<
        std::collections::HashMap<QualityID, f64>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_transfer_modules_into_module_slots_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_boxes: Option<Vec<FluidBox>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<CraftingMachineGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set_flipped: Option<CraftingMachineGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_output_full: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_animation_speed_to_activity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_slots: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_slots_quality_bonus: Option<std::collections::HashMap<QualityID, u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perceived_performance: Option<PerceivedPerformance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_health_effect: Option<ProductionHealthEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_energy_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_module_slots: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_ingredients_on_change: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_recipe_icon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_recipe_icon_on_map: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trash_inventory_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_mirroring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_to_place_result: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CreatePlatformAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CurvedRailAPrototype {
    #[serde(flatten)]
    pub parent: RailPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CurvedRailBPrototype {
    #[serde(flatten)]
    pub parent: RailPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CustomEventPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct CustomInputPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_key_sequence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_modifiers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consuming: Option<ConsumingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_alternative_key_sequence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_key_sequence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_while_in_cutscene: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_while_spectating: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_selected_prototype: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_to_spawn: Option<ItemID>,
    pub key_sequence: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_game_control: Option<LinkedGameControl>,
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DamageType {
    #[serde(flatten)]
    pub parent: Prototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DeciderCombinatorPrototype {
    #[serde(flatten)]
    pub parent: CombinatorPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equal_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_or_equal_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_or_equal_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equal_symbol_sprites: Option<Sprite4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DeconstructWithRobotsAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    pub amount: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DeconstructibleTileProxyPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DeconstructionItemPrototype {
    #[serde(flatten)]
    pub parent: SelectionToolPrototype,
    pub alt_select: SelectionModeData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_include_tiles: Option<bool>,
    pub select: SelectionModeData,
    pub stack_size: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DecorativePrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoplace: Option<AutoplaceSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_overdraw_priority: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grows_through_rail_path: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_separation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity_over_water: Option<f64>,
    pub pictures: SpriteVariations,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placed_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_visualisation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_visualisation_variations: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_layer: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walking_sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DelayedActiveTriggerPrototype {
    #[serde(flatten)]
    pub parent: ActiveTriggerPrototype,
    pub action: Trigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_when_source_is_destroyed: Option<bool>,
    pub delay: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_delay: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DeliverByRobotsAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    pub amount: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DeliverCategory {
    pub name: String,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DeliverImpactCombination {
    pub deliver_category: String,
    pub impact_category: String,
    pub name: String,
    pub trigger_effect_item: TriggerEffect,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DepleteResourceAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_to_one_game: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DestroyCliffAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_to_one_game: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DisplayPanelPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_text_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_text_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_shift: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DontBuildEntityAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototypeWithCondition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    pub dont_build: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub research_with: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DontCraftManuallyAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototypeWithCondition,
    pub amount: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DontKillManuallyAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototypeWithCondition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_kill: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_not_to_kill: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DontResearchBeforeResearchingAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototypeWithCondition,
    pub dont_research: serde_json::Value,
    pub research_with: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct DontUseEntityInEnergyProductionAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototypeWithCondition,
    pub excluded: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_hour_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_energy_produced: Option<Energy>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EditorControllerPrototype {
    pub adjust_speed_based_off_zoom: bool,
    pub enable_flash_light: bool,
    pub fill_built_entity_energy_buffers: bool,
    pub generate_neighbor_chunks: bool,
    pub gun_inventory_size: u16,
    pub ignore_tile_conditions: bool,
    pub instant_blueprint_building: bool,
    pub instant_deconstruction: bool,
    pub instant_rail_planner: bool,
    pub instant_upgrading: bool,
    pub inventory_size: u16,
    pub item_pickup_distance: f64,
    pub loot_pickup_distance: f64,
    pub mining_speed: f64,
    pub movement_speed: f64,
    pub name: String,
    pub placed_corpses_never_expire: bool,
    pub render_as_day: bool,
    pub show_additional_entity_info_gui: bool,
    pub show_character_tab_in_controller_gui: bool,
    pub show_entity_health_bars: bool,
    pub show_entity_tags: bool,
    pub show_hidden_entities: bool,
    pub show_infinity_filters_in_controller_gui: bool,
    pub show_status_icons: bool,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ElectricEnergyInterfaceEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_production: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_usage: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui_mode: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_power_production_in_tooltip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_power_usage_in_tooltip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_stored_energy_in_tooltip: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ElectricEnergyInterfacePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_copy_paste: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_animation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_production: Option<Energy>,
    pub energy_source: ElectricEnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_usage: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui_mode: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ElectricPolePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_connect_up_to_n_wires: Option<u8>,
    pub connection_points: Vec<WireConnectionPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_wire_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_visualisation_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewire_neighbours_when_destroying: Option<bool>,
    pub supply_area_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_coverage_during_drag_building: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ElectricTurretPrototype {
    #[serde(flatten)]
    pub parent: TurretPrototype,
    pub energy_source: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ElevatedCurvedRailAPrototype {
    #[serde(flatten)]
    pub parent: CurvedRailAPrototype,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tall: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ElevatedCurvedRailBPrototype {
    #[serde(flatten)]
    pub parent: CurvedRailBPrototype,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tall: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ElevatedHalfDiagonalRailPrototype {
    #[serde(flatten)]
    pub parent: HalfDiagonalRailPrototype,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tall: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ElevatedStraightRailPrototype {
    #[serde(flatten)]
    pub parent: StraightRailPrototype,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tall: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EnemySpawnerPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absorptions_per_second: Option<
        std::collections::HashMap<AirbornePollutantID, EnemySpawnerAbsorption>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_run_time_change_of_is_military_target: Option<bool>,
    pub call_for_help_radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captured_spawner_entity: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_sound: Option<Sound>,
    pub graphics_set: EnemySpawnerGraphicsSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_military_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count_of_owned_defensive_units: Option<u32>,
    pub max_count_of_owned_units: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_darkness_to_spawn: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_defensive_friends_around_to_spawn: Option<u32>,
    pub max_friends_around_to_spawn: u32,
    pub max_richness_for_spawn_shift: f64,
    pub max_spawn_shift: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_darkness_to_spawn: Option<f64>,
    pub result_units: Vec<UnitSpawnDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_blocked_trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_decoration: Option<Vec<CreateDecorativesTriggerEffectItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_decorations_on_expansion: Option<bool>,
    pub spawning_cooldown: (f64, f64),
    pub spawning_radius: f64,
    pub spawning_spacing: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_capture: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EnergyShieldEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    pub energy_per_shield: Energy,
    pub max_shield_value: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EntityGhostPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub huge_build_animated_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub huge_build_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_build_animated_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_build_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_build_animated_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_build_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_build_animated_sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EntityPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_pastable_entities: Option<Vec<EntityID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_icon_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_icon_shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_copy_paste: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_sounds: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_sounds_group: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoplace: Option<AutoplaceSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_grid_size: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_effect: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_smoke: Option<CreateTrivialSmokeEffectItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deconstruction_alternative: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagonal_tile_grid_size: Option<TilePosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_stateless_visualisations_in_ghost: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawing_box_vertical_extension: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissions_per_second: Option<
        std::collections::HashMap<AirbornePollutantID, f64>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_map_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_replaceable_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<EntityPrototypeFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_map_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heating_energy: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_visualization_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_draw_specification: Option<IconDrawSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons_positioning: Option<Vec<IconSequencePositioning>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_generator_bounding_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minable: Option<MinableProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mined_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_upgrade: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeable_by: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeable_position_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_from_tile_building: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_visualisation_specification: Option<RadiusVisualisationSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remains_when_mined: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_decoratives: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotated_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_in_game: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shooting_cursor_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_fluid_visualization_when_in_cursor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_visualisation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_conditions: Option<Vec<SurfaceCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tall: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_buildability_rules: Option<Vec<TileBuildabilityRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_target_mask: Option<TriggerTargetMask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_reflection: Option<WaterReflectionDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_sound: Option<WorkingSound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EntityWithHealthPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_when_damaged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_reaction: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corpse: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_ghost_on_death: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damaged_trigger_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_explosion: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_trigger_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healing_per_tick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_resistances: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_patch_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot: Option<Vec<ItemProductPrototype>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_health: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overkill_fraction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_corpse_variation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair_speed_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resistances: Option<Vec<Resistance>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EntityWithOwnerPrototype {
    #[serde(flatten)]
    pub parent: EntityWithHealthPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_run_time_change_of_is_military_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_military_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_indicator_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_indicator_shift: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EquipArmorAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    pub alternative_armor: ItemID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    pub armor: ItemID,
    pub limit_quality: QualityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_to_one_game: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EquipmentCategory {
    #[serde(flatten)]
    pub parent: Prototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EquipmentGhostPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<EquipmentCategoryID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_source: Option<ElectricEnergySource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<EquipmentShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_result: Option<ItemID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EquipmentGridPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    pub equipment_categories: Vec<EquipmentCategoryID>,
    pub height: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    pub width: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct EquipmentPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_border_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,
    pub categories: Vec<EquipmentCategoryID>,
    pub energy_source: ElectricEnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabbed_background_color: Option<Color>,
    pub shape: EquipmentShape,
    pub sprite: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_result: Option<ItemID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ExplosionPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    pub animations: AnimationVariations,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beam: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_rotation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_deviation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosion_effect: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in_duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out_duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_intensity_factor_final: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_intensity_factor_initial: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_intensity_peak_end_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_intensity_peak_start_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_size_factor_final: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_size_factor_initial: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_size_peak_end_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_size_peak_start_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_animation_speed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_end: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_increment_per_tick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_initial: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_initial_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_out_duration: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke: Option<TrivialSmokeID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke_slow_down_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FireFlamePrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_fuel_cooldown: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burnt_patch_alpha_default: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burnt_patch_alpha_variations: Option<Vec<TileAndAlpha>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burnt_patch_lifetime: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burnt_patch_pictures: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_multiplier_decrease_per_tick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_multiplier_increase_per_added_fuel: Option<f64>,
    pub damage_per_tick: DamageParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_between_initial_flames: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flame_alpha: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flame_alpha_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_flame_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_lifetime: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime_increase_by: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime_increase_cooldown: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_size_modifier_maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_size_modifier_per_flame: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_overlapping_particles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_damage_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_lifetime: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_spread_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_damage_tick_effect: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_fuel_added_action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_alpha: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_alpha_blend_duration: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_alpha_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_picture_fade_out_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_picture_fade_out_start: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_pictures: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_tree_fire_pictures: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke: Option<Vec<SmokeSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke_fade_in_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke_fade_out_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke_source_pictures: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_entity: Option<EntityID>,
    pub spread_delay: u32,
    pub spread_delay_deviation: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_dying_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_alternative_behavior: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FishPrototype {
    #[serde(flatten)]
    pub parent: EntityWithHealthPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<SpriteVariations>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FluidPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_barrel: Option<bool>,
    pub base_color: Color,
    pub default_temperature: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_as_glow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissions_multiplier: Option<f64>,
    pub flow_color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_value: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_capacity: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_fluid: Option<SpentFluidSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visualization_color: Option<Color>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FluidStreamPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oriented_particle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_alpha_per_part: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_buffer_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_end_alpha: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_fade_out_duration: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_fade_out_threshold: Option<f64>,
    pub particle_horizontal_speed: f64,
    pub particle_horizontal_speed_deviation: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_loop_exit_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_loop_frame_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_scale_per_part: Option<f64>,
    pub particle_spawn_interval: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_spawn_timeout: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_start_alpha: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_start_scale: Option<f64>,
    pub particle_vertical_acceleration: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_to_create_smoke: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_scale_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke_sources: Option<Vec<SmokeSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_neutral_target_damage: Option<DamageParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spine_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_initial_position_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_position_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FluidTurretPrototype {
    #[serde(flatten)]
    pub parent: TurretPrototype,
    pub activation_buffer_ratio: f64,
    pub attack_parameters: StreamAttackParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attacking_muzzle_animation_shift: Option<AnimatedVector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_attack_muzzle_animation_shift: Option<AnimatedVector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enough_fuel_indicator_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enough_fuel_indicator_picture: Option<Sprite8Way>,
    pub fluid_box: FluidBox,
    pub fluid_buffer_input_flow: f64,
    pub fluid_buffer_size: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded_muzzle_animation_shift: Option<AnimatedVector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folding_muzzle_animation_shift: Option<AnimatedVector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muzzle_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muzzle_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_enough_fuel_indicator_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_enough_fuel_indicator_picture: Option<Sprite8Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_ammo_alert_icon: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_muzzle_animation_shift: Option<AnimatedVector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparing_muzzle_animation_shift: Option<AnimatedVector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_attack_muzzle_animation_shift: Option<AnimatedVector>,
    pub turret_base_has_direction: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FluidWagonPrototype {
    #[serde(flatten)]
    pub parent: RollingStockPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_valve_xy_offset_when_horizontal: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_valve_xy_offset_when_vertical: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_valve_z_offset_projected_when_horizontal: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_valve_z_offset_projected_when_vertical: Option<f64>,
    pub capacity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_category: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_capacity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tank_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tank_spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valve_to_valve_offset_when_horizontal: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valve_to_valve_offset_when_vertical: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FlyingRobotPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_per_move: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_per_tick: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_military_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_energy: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_to_charge: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_to_charge: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    pub speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_multiplier_when_out_of_energy: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FontPrototype {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered: Option<bool>,
    pub from: String,
    pub name: String,
    pub size: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<f64>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FuelCategory {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_value_type: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FurnacePrototype {
    #[serde(flatten)]
    pub parent: CraftingMachinePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cant_insert_at_source_message_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector_flipped: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_input_slot_tooltip_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_recipe_finished_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_working_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub result_inventory_size: u16,
    pub source_inventory_size: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FusionGeneratorPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burns_fluid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effectivity: Option<f64>,
    pub energy_source: ElectricEnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<FusionGeneratorGraphicsSet>,
    pub input_fluid_box: FluidBox,
    pub max_fluid_usage: f64,
    pub output_fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perceived_performance: Option<PerceivedPerformance>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct FusionReactorPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub burner: BurnerEnergySource,
    pub energy_source: ElectricEnergySource,
    pub graphics_set: FusionReactorGraphicsSet,
    pub input_fluid_box: FluidBox,
    pub max_fluid_usage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbour_bonus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbour_connectable: Option<NeighbourConnectable>,
    pub output_fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perceived_performance: Option<PerceivedPerformance>,
    pub power_input: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_direction_only: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GatePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub activation_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closing_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fadeout_interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_rail_animation_left: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_rail_animation_right: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_rail_base: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_sound: Option<Sound>,
    pub opening_speed: f64,
    pub timeout_to_close: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_rail_animation_left: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_rail_animation_right: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_rail_base: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_patch: Option<Animation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GeneratorEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burner: Option<BurnerEnergySource>,
    pub power: Energy,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GeneratorPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burns_fluid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_non_fuel_fluid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effectivity: Option<f64>,
    pub energy_source: ElectricEnergySource,
    pub fluid_box: FluidBox,
    pub fluid_usage_per_tick: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_power_output: Option<Energy>,
    pub maximum_temperature: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_fluid_box: Option<FluidBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perceived_performance: Option<PerceivedPerformance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<GeneratorPictureSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_fluid_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke: Option<Vec<SmokeSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_fluid: Option<SpentFluidSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_direction_only: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GodControllerPrototype {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crafting_categories: Option<Vec<RecipeCategoryID>>,
    pub inventory_size: u16,
    pub item_pickup_distance: f64,
    pub loot_pickup_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_categories: Option<Vec<ResourceCategoryID>>,
    pub mining_speed: f64,
    pub movement_speed: f64,
    pub name: String,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GroupAttackAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<EntityID>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GuiStyle {
    #[serde(flatten)]
    pub parent: PrototypeBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sprite_priority: Option<SpritePriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sprite_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tileset: Option<FileName>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct GunPrototype {
    #[serde(flatten)]
    pub parent: ItemPrototype,
    pub attack_parameters: AttackParameters,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct HalfDiagonalRailPrototype {
    #[serde(flatten)]
    pub parent: RailPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct HeatInterfacePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui_mode: Option<serde_json::Value>,
    pub heat_buffer: HeatBuffer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heating_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct HeatPipePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<Vec<CircuitConnectorDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_sprites: Option<ConnectableEntityGraphics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_temperature_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub heat_buffer: HeatBuffer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_glow_sprites: Option<ConnectableEntityGraphics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heating_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct HighlightBoxEntityPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ImpactCategory {
    pub name: String,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InfinityCargoWagonPrototype {
    #[serde(flatten)]
    pub parent: CargoWagonPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erase_contents_when_mined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui_mode: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_contents_when_created: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InfinityContainerPrototype {
    #[serde(flatten)]
    pub parent: LogisticContainerPrototype,
    pub erase_contents_when_mined: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui_mode: Option<serde_json::Value>,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logistic_mode: Option<LogisticMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_contents_when_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_not_in_network_icon: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InfinityPipePrototype {
    #[serde(flatten)]
    pub parent: PipePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui_mode: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InserterPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_burner_leech: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_custom_vectors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chases_belt_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector_flipped: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_stack_control_input_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_held_item: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_inserter_arrow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_per_movement: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_per_rotation: Option<Energy>,
    pub energy_source: EnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_drop_mode_if_held_stack_spoiled: Option<bool>,
    pub extension_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grab_less_to_match_belt_stack: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_base_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_base_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_base_shadow: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_closed_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_closed_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_closed_shadow: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_open_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_open_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_open_shadow: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_size: Option<f64>,
    pub insert_position: Vector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_belt_stack_size: Option<u8>,
    pub pickup_position: Vector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_frozen: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_frozen_flipped: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_picture: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_picture_flipped: Option<Sprite4Way>,
    pub rotation_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size_bonus: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_easter_egg: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_mirroring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_inserter_stack_size_bonus: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_full_hand: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct InventoryBonusEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_source: Option<ElectricEnergySource>,
    pub inventory_size_bonus: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemEntityPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemGroup {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_in_recipe: Option<Order>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_recycle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burnt_result: Option<ItemID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_hint: Option<ColorHintSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_background_icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_background_icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_background_icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_import_location: Option<SpaceLocationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroyed_by_dropping_trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ItemPrototypeFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_acceleration_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_acceleration_multiplier_quality_bonus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_category: Option<FuelCategoryID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_emissions_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_glow_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_top_speed_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_top_speed_multiplier_quality_bonus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_value: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_random_tint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredient_to_weight_coefficient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_move_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lab_ignores_spoil_percent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moved_to_hub_when_building: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pick_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_as_equipment_result: Option<EquipmentID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_as_tile: Option<PlaceAsTile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_result: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plant_result: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_tint_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_launch_products: Option<Vec<ItemProductPrototype>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_to_orbit_mode: Option<SendToOrbitMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_platform_request_priority: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoil_level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoil_quality_change: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoil_quality_max: Option<QualityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoil_quality_min: Option<QualityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoil_result: Option<ItemID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoil_ticks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoil_to_trigger_result: Option<SpoilToTriggerResult>,
    pub stack_size: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemRequestProxyPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_target_entity_alert_icon_shift: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemSubGroup {
    #[serde(flatten)]
    pub parent: Prototype,
    pub group: ItemGroupID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemWithEntityDataPrototype {
    #[serde(flatten)]
    pub parent: ItemPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_tintable: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_tintable_mask: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_tintable_mask_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_tintable_masks: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_tintable_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_tintables: Option<Vec<IconData>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemWithInventoryPrototype {
    #[serde(flatten)]
    pub parent: ItemWithLabelPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_message_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_mode: Option<serde_json::Value>,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_filters: Option<Vec<ItemID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_group_filters: Option<Vec<ItemGroupID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_subgroup_filters: Option<Vec<ItemSubGroupID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_inventory_size: Option<bool>,
    pub stack_size: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemWithLabelPrototype {
    #[serde(flatten)]
    pub parent: ItemPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_label_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_label_for_cursor_render: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ItemWithTagsPrototype {
    #[serde(flatten)]
    pub parent: ItemWithLabelPrototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct KillAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_dealer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_type: Option<DamageTypeID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_vehicle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personally: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_kill: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_to_kill: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LabPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_effects: Option<EffectTypeLimitation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_module_categories: Option<Vec<ModuleCategoryID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_technology_level_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_receiver: Option<EffectReceiver>,
    pub energy_source: EnergySource,
    pub energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite>,
    pub inputs: Vec<ItemID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_slots: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_module_slots: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub researching_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub science_pack_drain_rate_percent: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trash_inventory_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_quality_drain_modifier: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LampPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darkness_for_all_lamps_off: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darkness_for_all_lamps_on: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_blue_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_green_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_red_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_rgb_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub energy_source: serde_json::Value,
    pub energy_usage_per_tick: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_color_intensity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_render_mode: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_when_colored: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_off: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_on: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_to_color_mapping: Option<Vec<SignalColorMapping>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LandMinePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ammo_category: Option<AmmoCategoryID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_die_on_attack: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_military_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_safe: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_set: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_set_enemy: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_force: Option<ForceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_interval: Option<u32>,
    pub trigger_radius: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LaneSplitterPrototype {
    #[serde(flatten)]
    pub parent: TransportBeltConnectablePrototype,
    pub structure: Animation4Way,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_animation_movement_cooldown: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_animation_speed_coefficient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_patch: Option<Animation4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LegacyCurvedRailPrototype {
    #[serde(flatten)]
    pub parent: RailPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LegacyStraightRailPrototype {
    #[serde(flatten)]
    pub parent: RailPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LightningAttractorPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargable_graphics: Option<ChargableGraphics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efficiency: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_source: Option<ElectricEnergySource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightning_strike_offset: Option<MapPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_elongation: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LightningPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attracted_volume_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attractor_hit_effect: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage: Option<DamageParameters>,
    pub effect_duration: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<LightningGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_variance: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_effect: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_damage: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LinkedBeltPrototype {
    #[serde(flatten)]
    pub parent: TransportBeltConnectablePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_blueprint_connection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_clone_connection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_side_loading: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<LinkedBeltStructure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_render_layer: Option<RenderLayer>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LinkedContainerPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui_mode: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_properties: Option<InventoryWithCustomStackSizeSpecification>,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_weight_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Loader1x1Prototype {
    #[serde(flatten)]
    pub parent: LoaderPrototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Loader1x2Prototype {
    #[serde(flatten)]
    pub parent: LoaderPrototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LoaderPrototype {
    #[serde(flatten)]
    pub parent: TransportBeltConnectablePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_belt_stack_size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_container_interaction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rail_interaction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belt_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<Vec<CircuitConnectorDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_per_item: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_source: Option<serde_json::Value>,
    pub filter_count: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_belt_stack_size: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_lane_filters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respect_insert_limits: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<LoaderStructure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_full_stack: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LocomotivePrototype {
    #[serde(flatten)]
    pub parent: RollingStockPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darkness_to_render_light_animation: Option<f64>,
    pub energy_source: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_light_pictures: Option<RollingStockRotatedSlopedGraphics>,
    pub max_power: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_snap_to_train_stop_distance: Option<f64>,
    pub reversing_power_modifier: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LogisticContainerPrototype {
    #[serde(flatten)]
    pub parent: ContainerPrototype,
    pub logistic_mode: LogisticMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_logistic_slots: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_not_in_network_icon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_door: Option<RobotDoorSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trash_inventory_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_exact_mode: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct LogisticRobotPrototype {
    #[serde(flatten)]
    pub parent: RobotWithLogisticInterfacePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_with_cargo: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_motion_with_cargo: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_idle_with_cargo: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_in_motion_with_cargo: Option<RotatedAnimation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MapGenPresets {
    pub name: String,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MapSettings {
    pub asteroids: AsteroidSettings,
    pub difficulty_settings: DifficultySettings,
    pub enemy_evolution: EnemyEvolutionSettings,
    pub enemy_expansion: EnemyExpansionSettings,
    pub max_failed_behavior_count: u32,
    pub name: String,
    pub path_finder: PathFinderSettings,
    pub pollution: PollutionSettings,
    pub r#type: String,
    pub unit_group: UnitGroupSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MarketPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_access_to_all_forces: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MiningDrillPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_effects: Option<EffectTypeLimitation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_module_categories: Option<Vec<ModuleCategoryID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector_flipped: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drilling_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drilling_sound_animation_end_frame: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drilling_sound_animation_start_frame: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drops_full_belt_stacks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_receiver: Option<EffectReceiver>,
    pub energy_source: EnergySource,
    pub energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<MiningDrillGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set_flipped: Option<MiningDrillGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_fluid_box: Option<FluidBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_horizontal_mirroring: Option<bool>,
    pub mining_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_slots: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_visualization_tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moving_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_fluid_box: Option<FluidBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perceived_performance: Option<PerceivedPerformance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_mining_radius: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_module_slots: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_visualisation_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_resources_to_place: Option<bool>,
    pub resource_categories: Vec<ResourceCategoryID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_drain_rate_percent: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_searching_offset: Option<Vector>,
    pub resource_searching_radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shuffle_resources_to_mine: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_mirroring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_force_mining_productivity_bonus: Option<bool>,
    pub vector_to_place_result: Vector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wet_mining_graphics_set: Option<MiningDrillGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wet_mining_graphics_set_flipped: Option<MiningDrillGraphicsSet>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ModData {
    #[serde(flatten)]
    pub parent: Prototype,
    pub data: std::collections::HashMap<String, AnyBasic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ModuleCategory {
    #[serde(flatten)]
    pub parent: Prototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ModulePrototype {
    #[serde(flatten)]
    pub parent: ItemPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub art_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beacon_tint: Option<BeaconVisualizationTints>,
    pub category: ModuleCategoryID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_quality_multiplier: Option<f64>,
    pub effect: Effect,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pollution_quality_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub productivity_quality_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_quality_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_beacon_alt_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_quality_multiplier: Option<f64>,
    pub tier: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ModuleTransferAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_to_one_game: Option<bool>,
    pub module: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MouseCursor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_pixel_x: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_pixel_y: Option<i16>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_cursor: Option<serde_json::Value>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct MovementBonusEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    pub energy_consumption: Energy,
    pub movement_bonus: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct NamedNoiseExpression {
    #[serde(flatten)]
    pub parent: Prototype,
    pub expression: NoiseExpression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intended_property: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_expressions: Option<std::collections::HashMap<String, NoiseExpression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_functions: Option<std::collections::HashMap<String, NoiseFunction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct NamedNoiseFunction {
    #[serde(flatten)]
    pub parent: Prototype,
    pub expression: NoiseExpression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_expressions: Option<std::collections::HashMap<String, NoiseExpression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_functions: Option<std::collections::HashMap<String, NoiseFunction>>,
    pub parameters: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct NightVisionEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate_sound: Option<Sound>,
    pub color_lookup: DaytimeColorLookupTable,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darkness_to_turn_on: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivate_sound: Option<Sound>,
    pub energy_input: Energy,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct OffshorePumpPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_draw_fluid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub energy_source: EnergySource,
    pub energy_usage: Energy,
    pub fluid_box: FluidBox,
    pub fluid_source_offset: Vector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<OffshorePumpGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perceived_performance: Option<PerceivedPerformance>,
    pub pumping_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_on_tile_collision: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_fluid_visualization_when_in_cursor: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ParticlePrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_shadow_when_on_ground: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_in_water_trigger_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_on_ground_trigger_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_away_duration: Option<u16>,
    pub life_time: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_particle_frame_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_modifier_when_on_ground: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_trigger_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_trigger_effect_frequency: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer_when_on_ground: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadows: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_acceleration: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ParticleSourcePrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    pub height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_deviation: Option<f64>,
    pub horizontal_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_speed_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle: Option<ParticleID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke: Option<Vec<SmokeSource>>,
    pub time_before_start: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_before_start_deviation: Option<f64>,
    pub time_to_live: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_deviation: Option<f64>,
    pub vertical_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_speed_deviation: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PipePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_fluid_temperature_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub fluid_box: FluidBox,
    pub horizontal_window_bounding_box: BoundingBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<PipePictures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_fluid_visualization_when_in_cursor: Option<bool>,
    pub vertical_window_bounding_box: BoundingBox,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PipeToGroundPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_fluid_temperature_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_visualization: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_fluid_icon_override: Option<bool>,
    pub fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_fluid_visualization_when_in_cursor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visualization: Option<Sprite4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlaceEquipmentAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    pub armor: ItemID,
    pub limit_equip_quality: QualityID,
    pub limit_quality: QualityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_to_one_game: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlanetPrototype {
    #[serde(flatten)]
    pub parent: SpaceLocationPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities_require_heating: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightning_properties: Option<LightningProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_gen_settings: Option<PlanetPrototypeMapGenSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_seed_offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_ambient_sounds: Option<PersistentWorldAmbientSoundsDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_effects: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pollutant_type: Option<AirbornePollutantID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_properties: Option<std::collections::HashMap<SurfacePropertyID, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_render_parameters: Option<SurfaceRenderParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks_between_player_effects: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlantPrototype {
    #[serde(flatten)]
    pub parent: TreePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agricultural_tower_tint: Option<RecipeTints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_mounds: Option<Vec<Sprite>>,
    pub growth_ticks: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_variations: Option<Vec<TreeGrowth>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvest_emissions: Option<std::collections::HashMap<AirbornePollutantID, f64>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlayerDamagedAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    pub minimum_damage: f64,
    pub should_survive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_of_dealer: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PlayerPortPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PowerSwitchPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub circuit_wire_connection_point: WireConnectionPoint,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub led_off: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub led_on: Option<Sprite>,
    pub left_wire_connection_point: WireConnectionPoint,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_loop: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay_start: Option<Animation>,
    pub overlay_start_delay: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_on_animation: Option<Animation>,
    pub right_wire_connection_point: WireConnectionPoint,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_max_distance: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProcessionLayerInheritanceGroup {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_application: Option<TransitionApplication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermezzo_application: Option<TransitionApplication>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProcessionPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_timeline: Option<ProcessionTimeline>,
    pub procession_style: serde_json::Value,
    pub timeline: ProcessionTimeline,
    pub usage: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProduceAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_product: Option<FluidID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_product: Option<ItemIDFilter>,
    pub limited_to_one_game: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProducePerHourAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_product: Option<FluidID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_product: Option<ItemIDFilter>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProgrammableSpeakerPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audible_distance_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub energy_source: serde_json::Value,
    pub energy_usage_per_tick: Energy,
    pub instruments: Vec<ProgrammableSpeakerInstrument>,
    pub maximum_polyphony: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProjectilePrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    pub acceleration: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<RotatedAnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_drawing_with_mask: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_condition: Option<ForceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_at_collision_position: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub piercing_damage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotatable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<RotatedAnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoke: Option<Vec<SmokeSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_modifier: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turning_speed_increases_exponentially_with_projectile_speed: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct Prototype {
    #[serde(flatten)]
    pub parent: PrototypeBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tooltip_fields: Option<Vec<CustomTooltipField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factoriopedia_alternative: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PrototypeBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factoriopedia_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factoriopedia_simulation: Option<SimulationDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_in_factoriopedia: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localised_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localised_name: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgroup: Option<ItemSubGroupID>,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ProxyContainerPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_inventory_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct PumpPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_orienting_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_lifting_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamp_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub energy_source: EnergySource,
    pub energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_scaling: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_animation: Option<Animation4Way>,
    pub fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_wagon_connector_frame_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_wagon_connector_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_wagon_tank_valve_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glass_pictures: Option<Sprite4Way>,
    pub pumping_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_fluid_visualization_when_in_cursor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wagon_connection_graphics: Option<PumpWagonConnectionGraphics>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct QualityPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulator_capacity_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asteroid_collector_collection_radius_bonus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beacon_module_slots_bonus: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beacon_power_usage_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beacon_supply_area_distance_bonus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_wagon_inventory_size_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_probability: Option<f64>,
    pub color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crafting_machine_energy_usage_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crafting_machine_module_slots_bonus: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crafting_machine_speed_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_sprite_by_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electric_pole_supply_area_distance_bonus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electric_pole_wire_reach_bonus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment_grid_height_bonus: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment_grid_width_bonus: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid_wagon_capacity_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flying_robot_max_energy_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inserter_speed_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_size_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lab_module_slots_bonus: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lab_research_speed_multiplier: Option<f64>,
    pub level: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locomotive_power_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logistic_cell_charging_energy_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logistic_cell_charging_station_count_bonus: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_drill_mining_radius_bonus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_drill_module_slots_bonus: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_drill_resource_drain_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_consumption_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_pollution_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_productivity_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_quality_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_speed_multiplier: Option<f64>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<QualityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_chain_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_stock_max_speed_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub science_pack_drain_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoil_ticks_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_durability_multiplier: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RadarPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connects_to_other_radars: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_universe_channel: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_fraction_to_connect: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_fraction_to_disconnect: Option<f64>,
    pub energy_per_nearby_scan: Energy,
    pub energy_per_sector: Energy,
    pub energy_source: EnergySource,
    pub energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_military_target: Option<bool>,
    pub max_distance_of_nearby_sector_revealed: u32,
    pub max_distance_of_sector_revealed: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<RotatedSprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_minimap_visualisation_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_orientation_when_frozen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_speed: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailChainSignalPrototype {
    #[serde(flatten)]
    pub parent: RailSignalBasePrototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailPlannerPrototype {
    #[serde(flatten)]
    pub parent: ItemPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_length_limit: Option<f64>,
    pub rails: Vec<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<EntityID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_grid_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deconstruction_marker_positions: Option<Vec<Vector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_shifts: Option<Vec<Vector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_planner_goal_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_planner_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fence_pictures: Option<RailFenceGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forced_fence_segment_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<RailPictureSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removes_soft_decoratives: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walking_sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailRampPrototype {
    #[serde(flatten)]
    pub parent: RailPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_mask_allow_on_deep_oil_ocean: Option<CollisionMaskConnector>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_range: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tall: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailRemnantsPrototype {
    #[serde(flatten)]
    pub parent: CorpsePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_grid_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<RailPictureSet>,
    pub related_rail: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_collision_box: Option<BoundingBox>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailSignalBasePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_blue_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_green_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_orange_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_red_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevated_collision_mask: Option<CollisionMaskConnector>,
    pub elevated_picture_set: RailSignalPictureSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevated_selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<EntityPrototypeFlags>,
    pub ground_picture_set: RailSignalPictureSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailSignalPrototype {
    #[serde(flatten)]
    pub parent: RailSignalBasePrototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RailSupportPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_grid_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_mask_allow_on_deep_oil_ocean: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevated_selection_boxes: Option<Vec<BoundingBox>>,
    pub graphics_set: RailSupportGraphicsSet,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_buildable_if_no_rails: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_to_spots_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_range: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ReactorPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_patches_connected: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_patches_disconnected: Option<SpriteVariations>,
    pub consumption: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_fuel_glow_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_temperature_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub energy_source: EnergySource,
    pub heat_buffer: HeatBuffer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_connection_patches_connected: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_connection_patches_disconnected: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_lower_layer_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heating_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_layer_picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meltdown_action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbour_bonus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbour_connectable: Option<NeighbourConnectable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_energy_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_to_suppress_energy_icons: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_fuel_glow_color: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_light_picture: Option<Animation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RecipeCategory {
    #[serde(flatten)]
    pub parent: Prototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RecipePrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_as_intermediate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_consumption: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_consumption_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_decomposition: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_inserter_overload: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_intermediates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_pollution: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_pollution_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_productivity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_productivity_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_quality: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_quality_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_speed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_speed_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_module_categories: Option<Vec<ModuleCategoryID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_unlock_methods: Option<Vec<TechnologyID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_show_made_in: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_recycle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_quality: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<RecipeCategoryID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crafting_machine_tint: Option<RecipeTints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissions_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_required: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_from_bonus_gui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_from_player_crafting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_from_signal_gui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_from_stats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredients: Option<Vec<IngredientPrototype>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_productivity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overload_multiplier: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_products_in_machine_output: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raise_on_crafted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_paste_multiplier: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_ingredients_to_unlock_results: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ProductPrototype>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_item_ingredients: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_conditions: Option<Vec<SurfaceCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_results: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RemoteControllerPrototype {
    pub movement_speed: f64,
    pub name: String,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RepairToolPrototype {
    #[serde(flatten)]
    pub parent: ToolPrototype,
    pub speed: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ResearchAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub research_all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology: Option<TechnologyID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ResearchWithSciencePackAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    pub science_pack: ItemID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ResourceCategory {
    #[serde(flatten)]
    pub parent: Prototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ResourceEntityPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<ResourceCategoryID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliff_removal_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_stateless_visualisation_under_building: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driving_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_animation_period: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_animation_period_deviation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_darkness_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite_depletion_amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_grid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_effect_alpha: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_effect_alpha: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_visualisation_tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomize_visual_position: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_patch_search_radius: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    pub stage_counts: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages_effect: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_removal_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_removal_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walking_sound: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RoboportEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burner: Option<BurnerEnergySource>,
    pub charge_approach_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_distance: Option<f64>,
    pub charging_energy: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_offsets: Option<Vec<Vector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_station_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_station_count_affected_by_quality: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_station_shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_threshold_distance: Option<f64>,
    pub construction_radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_construction_radius_visualization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_logistic_radius_visualization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recharging_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recharging_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_vertical_acceleration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots_shrink_when_entering_and_exiting: Option<bool>,
    pub spawn_and_station_height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_and_station_shadow_height_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_minimum: Option<Energy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stationing_offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stationing_render_layer_swap_height: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RoboportPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_patch: Option<Sprite>,
    pub charge_approach_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_distance: Option<f64>,
    pub charging_energy: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_offsets: Option<Vec<Vector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_station_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_station_count_affected_by_quality: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_station_shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_threshold_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_door_trigger_effect: Option<TriggerEffect>,
    pub construction_radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_available_construction_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_available_logistic_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_roboport_count_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_total_construction_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_total_logistic_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_animation_down: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_animation_up: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_construction_radius_visualization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_logistic_radius_visualization: Option<bool>,
    pub energy_source: serde_json::Value,
    pub energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logistics_connection_distance: Option<f64>,
    pub logistics_radius: f64,
    pub material_slots_count: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_logistic_slots: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_door_trigger_effect: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_range: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_visualisation_color: Option<Color>,
    pub recharge_minimum: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recharging_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recharging_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_recharge_icon: Option<bool>,
    pub request_to_open_door_timeout: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_limit: Option<u32>,
    pub robot_slots_count: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_vertical_acceleration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots_shrink_when_entering_and_exiting: Option<bool>,
    pub spawn_and_station_height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_and_station_shadow_height_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stationing_offset: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stationing_render_layer_swap_height: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RobotWithLogisticInterfacePrototype {
    #[serde(flatten)]
    pub parent: FlyingRobotPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_cargo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_motion: Option<RotatedAnimation>,
    pub max_payload_size: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payload_size_after_bonus: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_charge_to_mine: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_idle: Option<RotatedAnimation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_in_motion: Option<RotatedAnimation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RocketSiloPrototype {
    #[serde(flatten)]
    pub parent: AssemblingMachinePrototype,
    pub active_energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_trigger: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_01_back_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_02_right_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arm_03_front_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arms_speed_modifier_per_quality_level: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_day_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_engine_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_front_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_front_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_night_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_launch_without_landing_pads: Option<bool>,
    pub cargo_station_parameters: CargoStationParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamps_off_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamps_off_trigger: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamps_on_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamps_on_trigger: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_back_frozen: Option<Sprite>,
    pub door_back_open_offset: Vector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_back_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_front_frozen: Option<Sprite>,
    pub door_front_open_offset: Vector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_front_sprite: Option<Sprite>,
    pub door_opening_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doors_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doors_trigger: Option<TriggerEffect>,
    pub hole_clipping_box: BoundingBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hole_frozen: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hole_light_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hole_sprite: Option<Sprite>,
    pub lamp_energy_usage: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_to_space_platforms: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_wait_time: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lift_weight: Option<f64>,
    pub light_blinking_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logistic_trash_inventory_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_alarm_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raise_rocket_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raise_rocket_trigger: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_lights_back_sprites: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_lights_front_sprites: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_not_in_network_icon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_door: Option<RobotDoorSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_engine_starting_speed_modifier_per_quality_level: Option<f64>,
    pub rocket_entity: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_glow_overlay_sprite: Option<Sprite>,
    pub rocket_parts_required: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_parts_storage_cap: Option<u32>,
    pub rocket_quick_relaunch_start_offset: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_rising_delay: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_rising_speed_modifier_per_quality_level: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_shadow_overlay_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_shadow_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_sprite: Option<Sprite>,
    pub silo_fade_out_end_distance: f64,
    pub silo_fade_out_start_distance: f64,
    pub times_to_blink: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_be_inserted_to_rocket_inventory_size: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RocketSiloRocketPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_attachment_offset: Option<Vector>,
    pub cargo_pod_entity: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_explosion: Option<EntityID>,
    pub effects_fade_in_end_distance: f64,
    pub effects_fade_in_start_distance: f64,
    pub engine_starting_speed: f64,
    pub flying_acceleration: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flying_sound: Option<Sound>,
    pub flying_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flying_trigger: Option<TriggerEffect>,
    pub full_render_layer_switch_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_light: Option<LightDefinition>,
    pub inventory_size: u16,
    pub rising_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_above_wires_slice_offset_from_center: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_air_object_slice_offset_from_center: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_flame_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_flame_left_animation: Option<Animation>,
    pub rocket_flame_left_rotation: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_flame_right_animation: Option<Animation>,
    pub rocket_flame_right_rotation: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_fog_mask: Option<FogMaskShapeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_glare_overlay_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_initial_offset: Option<Vector>,
    pub rocket_launch_offset: Vector,
    pub rocket_render_layer_switch_distance: f64,
    pub rocket_rise_offset: Vector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_shadow_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_smoke_bottom1_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_smoke_bottom2_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_smoke_top1_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_smoke_top2_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_smoke_top3_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_sprite: Option<Sprite>,
    pub rocket_visible_distance_from_center: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    pub shadow_fade_out_end_ratio: f64,
    pub shadow_fade_out_start_ratio: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_slave_entity: Option<EntityID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RocketSiloRocketShadowPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct RollingStockPrototype {
    #[serde(flatten)]
    pub parent: VehiclePrototype,
    pub air_resistance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_manual_color: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_robot_dispatch_in_automatic_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    pub connection_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_copy_color_from_train_stop: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_closing_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_opening_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_over_elevated_tie_trigger: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_over_tie_trigger: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_over_tie_trigger_minimal_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevated_collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevated_rail_sound: Option<MainSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevated_selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_doors: Option<Animation>,
    pub joint_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<RollingStockRotatedSlopedGraphics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_affects_max_speed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_by_light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tie_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_doors: Option<Animation>,
    pub vertical_selection_shift: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheels: Option<RollingStockRotatedSlopedGraphics>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SegmentPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub animation: RotatedAnimation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward_overlap: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward_padding: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_sound_volume_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_overlap: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_padding: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_effects: Option<Vec<TriggerEffectWithCooldown>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_effects_while_enraged: Option<Vec<TriggerEffectWithCooldown>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SegmentedUnitPrototype {
    #[serde(flatten)]
    pub parent: SegmentPrototype,
    pub acceleration_rate: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_parameters: Option<AttackParameters>,
    pub attacking_speed: f64,
    pub enraged_duration: u64,
    pub enraged_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hurt_roar: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hurt_thresholds: Option<Vec<f64>>,
    pub investigating_speed: f64,
    pub patrolling_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patrolling_turn_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenge_attack_parameters: Option<AttackParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roar: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roar_probability: Option<f64>,
    pub segment_engine: SegmentEngineSpecification,
    pub territory_radius: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks_per_scan: Option<u32>,
    pub turn_radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_smoothing: Option<f64>,
    pub vision_distance: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SelectionToolPrototype {
    #[serde(flatten)]
    pub parent: ItemWithLabelPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_reverse_select: Option<SelectionModeData>,
    pub alt_select: SelectionModeData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_include_tiles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse_cursor: Option<MouseCursorID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_select: Option<SelectionModeData>,
    pub select: SelectionModeData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_fog_of_war: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_forced_select: Option<SelectionModeData>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SelectorCombinatorPrototype {
    #[serde(flatten)]
    pub parent: CombinatorPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_day_length_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_day_tick_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_game_tick_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_symbol_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_capacity_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size_sprites: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_symbol_sprites: Option<Sprite4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ShootAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ammo_type: Option<ItemID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ShortcutPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    pub action: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_control_input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_to_spawn: Option<ItemID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology_to_unlock: Option<TechnologyID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggleable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_until_unlocked: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SimpleEntityPrototype {
    #[serde(flatten)]
    pub parent: EntityWithHealthPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_as_rock_for_filtered_deconstruction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_pictures: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_animation_offset: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_variation_on_create: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_draw_order: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shuffled_variation_on_chunk_generated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_visualisation_variations: Option<Vec<serde_json::Value>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SimpleEntityWithForcePrototype {
    #[serde(flatten)]
    pub parent: SimpleEntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_military_target: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SimpleEntityWithOwnerPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<AnimationVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_visibility: Option<ForceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_pictures: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_animation_offset: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_variation_on_create: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_draw_order: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_visualisation_variations: Option<Vec<serde_json::Value>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SmokePrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_by_wind: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyclic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_away_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_fade_away_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_slow_down_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_when_smoke_off: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_scale: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SmokeWithTriggerPrototype {
    #[serde(flatten)]
    pub parent: SmokePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_cooldown: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_when_attachment_is_destroyed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_distance_scale_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_duration_variation: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_scale_factor: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_spread: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread_duration_variation: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wave_distance: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wave_speed: Option<Vector>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SolarPanelEquipmentPrototype {
    #[serde(flatten)]
    pub parent: EquipmentPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_at_day: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_at_night: Option<f64>,
    pub power: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solar_coefficient_property: Option<SurfacePropertyID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SolarPanelPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub energy_source: ElectricEnergySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_at_day: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_at_night: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<SpriteVariations>,
    pub production: Energy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solar_coefficient_property: Option<SurfacePropertyID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SoundPrototype {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_volume_control: Option<AdvancedVolumeControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<AggregationSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_random_repeat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audible_distance_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<SoundType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_controller_vibration_data: Option<GameControllerVibrationData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<serde_json::Value>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_smoothing_window_size: Option<u32>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpaceConnectionDistanceTraveledAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    pub distance: u32,
    pub reversed: bool,
    pub tracked_connection: SpaceConnectionID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpaceConnectionPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asteroid_spawn_definitions: Option<Vec<SpaceConnectionAsteroidSpawnDefinition>>,
    pub from: SpaceLocationID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    pub to: SpaceLocationID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpaceLocationPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asteroid_spawn_definitions: Option<Vec<SpaceLocationAsteroidSpawnDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asteroid_spawn_influence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_save_on_first_trip: Option<bool>,
    pub distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_orbit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fly_condition: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravity_pull: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_orientation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnitude: Option<f64>,
    pub orientation: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parked_platforms_orientation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_procession_set: Option<ProcessionSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_procession_set: Option<ProcessionSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_surface_render_parameters: Option<SurfaceRenderParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procession_audio_catalogue: Option<ProcessionAudioCatalogue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procession_graphic_catalogue: Option<ProcessionGraphicCatalogue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solar_power_in_space: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starmap_icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starmap_icon_orientation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starmap_icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starmap_icons: Option<Vec<IconData>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpacePlatformHubPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_grid_size: Option<f64>,
    pub cargo_station_parameters: CargoStationParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_damage_taken_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_speed_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<CargoBayConnectableGraphicsSet>,
    pub inventory_size: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_ambient_sounds: Option<PersistentWorldAmbientSoundsDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_repair_speed_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpacePlatformStarterPackPrototype {
    #[serde(flatten)]
    pub parent: ItemPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_electric_network: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_items: Option<Vec<ItemProductPrototype>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<SurfaceID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<Vec<SpacePlatformTileDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpectatorControllerPrototype {
    pub movement_speed: f64,
    pub name: String,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpeechBubblePrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in_out_ticks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    pub style: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapper_flow_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_offset: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpiderLegPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ankle_height: Option<f64>,
    pub base_position_selection_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<SpiderLegGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hip_flexibility: Option<f64>,
    pub initial_movement_speed: f64,
    pub knee_distance_factor: f64,
    pub knee_height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_leg_dying_trigger_effects: Option<Vec<SpiderLegTriggerEffect>>,
    pub minimal_step_size: f64,
    pub movement_acceleration: f64,
    pub movement_based_position_selection_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stretch_force_scalar: Option<f64>,
    pub target_position_randomisation_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_leg_dying_trigger_effects: Option<Vec<SpiderLegTriggerEffect>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walking_sound_speed_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walking_sound_volume_modifier: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpiderUnitPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absorptions_to_join_attack: Option<
        std::collections::HashMap<AirbornePollutantID, f64>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_settings: Option<UnitAISettings>,
    pub attack_parameters: AttackParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildable_entities: Option<Vec<EntityID>>,
    pub distraction_cooldown: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<SpiderTorsoGraphicsSet>,
    pub height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pursue_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pursue_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_range: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawning_time_modifier: Option<f64>,
    pub spider_engine: SpiderEngineSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steering: Option<SteeringSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torso_bob_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torso_rotation_speed: Option<f64>,
    pub vision_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warcry: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpiderVehiclePrototype {
    #[serde(flatten)]
    pub parent: VehiclePrototype,
    pub automatic_weapon_cycling: bool,
    pub chain_shooting_cooldown_modifier: f64,
    pub energy_source: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<SpiderVehicleGraphicsSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guns: Option<Vec<ItemID>>,
    pub height: f64,
    pub inventory_size: u16,
    pub movement_energy_consumption: Energy,
    pub spider_engine: SpiderEngineSpecification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torso_bob_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torso_rotation_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trash_inventory_size: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpidertronRemotePrototype {
    #[serde(flatten)]
    pub parent: SelectionToolPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color_indicator_mask: Option<FileName>,
    pub stack_size: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SplitterPrototype {
    #[serde(flatten)]
    pub parent: TransportBeltConnectablePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_input_left_condition: Option<CircuitConditionConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_input_right_condition: Option<CircuitConditionConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_output_left_condition: Option<CircuitConditionConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_output_right_condition: Option<CircuitConditionConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_transport_belt: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_animation_movement_cooldown: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_animation_speed_coefficient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_patch: Option<Animation4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SpritePrototype {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_forced_downscale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_runtime_tint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_special_effect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice_y: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_as_glow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_as_light: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_as_shadow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<SpriteFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_sdf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_colors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Sprite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_in_minimal_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mipmap_count: Option<u8>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<(u16, u16)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premul_alpha: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<SpritePriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_shift: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Vector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<SpriteUsageSurfaceHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_as_overlay: Option<bool>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<SpriteUsageHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StickerPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_per_tick: Option<DamageParameters>,
    pub duration_in_ticks: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_spread_cooldown: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_spread_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_visibility: Option<ForceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_target: Option<bool>,
    pub hidden: bool,
    pub hidden_in_factoriopedia: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_box_type: Option<CursorBoxType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_particle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread_fire_entity: Option<EntityID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers_per_square_meter: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_movement_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_movement_max_from: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_movement_max_to: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_movement_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_movement_modifier_from: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_movement_modifier_to: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_effects: Option<Vec<TriggerEffectWithCooldown>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_damage_substitute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_friction_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_friction_modifier_from: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_friction_modifier_to: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_speed_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_speed_max_from: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_speed_max_to: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_speed_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_speed_modifier_from: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_speed_modifier_to: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StorageTankPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_fluid_temperature_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    pub flow_length_in_ticks: u32,
    pub fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<StorageTankPictures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_fluid_icon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_fluid_visualization_when_in_cursor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_direction_only: Option<bool>,
    pub window_bounding_box: BoundingBox,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct StraightRailPrototype {
    #[serde(flatten)]
    pub parent: RailPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SurfacePropertyPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    pub default_value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_time: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localised_unit_key: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct SurfacePrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_properties: Option<std::collections::HashMap<SurfacePropertyID, f64>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TechnologyPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_productivity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<Vec<Modifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_tech_cost_multiplier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level: Option<serde_json::Value>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisites: Option<Vec<TechnologyID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub research_trigger: Option<TechnologyTrigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_levels_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<TechnologyUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_when_disabled: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TemporaryContainerPrototype {
    #[serde(flatten)]
    pub parent: ContainerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_after_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_on_empty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ThrusterPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub fuel_fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics_set: Option<ThrusterGraphicsSet>,
    pub max_performance: ThrusterPerformancePoint,
    pub min_performance: ThrusterPerformancePoint,
    pub oxidizer_fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plumes: Option<PlumesSpecification>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileEffectDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub puddle: Option<PuddleTileEffectParameters>,
    pub shader: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<SpaceTileEffectParameters>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water: Option<WaterTileEffectParameters>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TileGhostPrototype {
    #[serde(flatten)]
    pub parent: EntityPrototype,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TilePrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absorptions_per_second: Option<
        std::collections::HashMap<AirbornePollutantID, f64>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_neighbors: Option<Vec<TileID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_being_covered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_sounds: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_sounds_group: Option<TileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoplace: Option<AutoplaceSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound_decoratives: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_animations: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_animations_background: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_sound: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_animation_frame: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_part_of_blueprint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_collision_with_entities: Option<bool>,
    pub collision_mask: TileCollisionMaskConnector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decorative_removal_probability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cover_tile: Option<TileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_destroyed_dropped_item_trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroys_dropped_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driving_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_explosion: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<TileEffectDefinitionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_color_secondary: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_is_opaque: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluid: Option<FluidID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_variant: Option<TileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_foundation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_steps_sound: Option<Sound>,
    pub layer: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_group: Option<TileRenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowland_fog: Option<bool>,
    pub map_color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_health: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minable: Option<MinableProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mined_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_correction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_direction: Option<TileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_tints: Option<TileBasedParticleTints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeable_by: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scorch_mark_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite_usage_surface: Option<SpriteUsageSurfaceHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thawed_variant: Option<TileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_merges_with_tile: Option<TileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_overlay_layer_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<TileTransitionsToTiles>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions_between_transitions: Option<Vec<TileTransitionsBetweenTransitions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_effect: Option<TriggerEffect>,
    pub variants: TileTransitionsVariants,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_friction_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walking_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walking_speed_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TipsAndTricksItem {
    #[serde(flatten)]
    pub parent: PrototypeBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_title: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_input_method_filter: Option<PlayerInputMethodFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation: Option<SimulationDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_trigger: Option<TipTrigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_status: Option<TipStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<TipTrigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tutorial: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TipsAndTricksItemCategory {
    pub name: String,
    pub order: Order,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ToolPrototype {
    #[serde(flatten)]
    pub parent: ItemPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability_description_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability_description_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TrainPathAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    pub minimum_distance: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TrainStopPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    pub animation_ticks_per_frame: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_grid_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<
        (
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
            CircuitConnectorDefinition,
        ),
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_priority_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_train_stopped_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_trains_count_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_trains_limit_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawing_boxes: Option<TrainStopDrawingBoxes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light1: Option<TrainStopLight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light2: Option<TrainStopLight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rail_overlay_animations: Option<Animation4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_animations: Option<Animation4Way>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TransportBeltConnectablePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_speed_coefficient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belt_animation_set: Option<TransportBeltAnimationSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<EntityPrototypeFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_priority: Option<u8>,
    pub speed: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TransportBeltPrototype {
    #[serde(flatten)]
    pub parent: TransportBeltConnectablePrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belt_animation_set: Option<TransportBeltAnimationSetWithCorners>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<Vec<CircuitConnectorDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_frame_sprites: Option<TransportBeltConnectorFrame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_underground_belt: Option<EntityID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TreePrototype {
    #[serde(flatten)]
    pub parent: EntityWithHealthPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darkness_of_burnt_tree: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healing_per_tick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<SpriteVariations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_visualisation_variations: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variation_weights: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<Vec<TreeVariation>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TriggerTargetType {
    pub name: String,
    pub r#type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TrivialSmokePrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_by_wind: Option<bool>,
    pub animation: Animation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyclic: Option<bool>,
    pub duration: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_away_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_fade_away_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_slow_down_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_when_smoke_off: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_scale: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TurretPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_when_attacking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_turning_when_starting_attack: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_from_start_frame: Option<bool>,
    pub attack_parameters: AttackParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_target_mask: Option<TriggerTargetMask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attacking_animation: Option<RotatedAnimation8Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attacking_speed: Option<f64>,
    pub call_for_help_radius: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_retarget_while_starting_attack: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<Vec<CircuitConnectorDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_speed_secondary: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_speed_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_starting_progress_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_attack_animation: Option<RotatedAnimation8Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_attack_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_attack_speed_secondary: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_attack_speed_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_attack_starting_progress_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_glow_animation: Option<RotatedAnimation8Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_glow_animation_flicker_strength: Option<f64>,
    pub folded_animation: RotatedAnimation8Way,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded_animation_is_stateless: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded_speed_secondary: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded_speed_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded_starting_progress_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded_state_corpse: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folding_animation: Option<RotatedAnimation8Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folding_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folding_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folding_speed_secondary: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folding_speed_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folding_starting_progress_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_light_intensity: Option<f64>,
    pub graphics_set: TurretGraphicsSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gun_animation_render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gun_animation_secondary_draw_order: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_target_mask: Option<TriggerTargetMask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_military_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_attacking_if_shoot_fails: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_range: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_alternative_animation: Option<RotatedAnimation8Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_alternative_chance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_alternative_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_alternative_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_alternative_speed_secondary: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_alternative_speed_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_alternative_starting_progress_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_animation: Option<RotatedAnimation8Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_speed_secondary: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_speed_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_starting_progress_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparing_animation: Option<RotatedAnimation8Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparing_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparing_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparing_speed_secondary: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparing_speed_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparing_starting_progress_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_animation_offset: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_indicator_animation: Option<RotatedAnimation8Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotating_sound: Option<InterruptibleSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_speed_secondary: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_speed_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_starting_progress_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shoot_in_prepare_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_decoration: Option<Vec<CreateDecorativesTriggerEffectItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_decorations_on_expansion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_effect: Option<TurretSpecialEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_attacking_only_when_can_shoot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_attack_animation: Option<RotatedAnimation8Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_attack_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_attack_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_attack_speed_secondary: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_attack_speed_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_attack_starting_progress_when_killed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turret_base_has_direction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfolds_before_dying: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct TutorialDefinition {
    #[serde(flatten)]
    pub parent: PrototypeBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    pub scenario: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UndergroundBeltPrototype {
    #[serde(flatten)]
    pub parent: TransportBeltConnectablePrototype,
    pub max_distance: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance_tint: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance_underground_remove_belts_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<UndergroundBeltStructure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underground_collision_mask: Option<CollisionMaskConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underground_remove_belts_sprite: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underground_sprite: Option<Sprite>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UnitPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absorptions_to_join_attack: Option<
        std::collections::HashMap<AirbornePollutantID, f64>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_by_tiles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_settings: Option<UnitAISettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_run_time_change_of_is_military_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_attacking_frame_sequence: Option<UnitAlternativeFrameSequence>,
    pub attack_parameters: AttackParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildable_entities: Option<Vec<EntityID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_open_gates: Option<bool>,
    pub distance_per_frame: f64,
    pub distraction_cooldown: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dying_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_belt_immunity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_military_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pursue_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pursue_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_while_shooting: Option<bool>,
    pub movement_speed: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_range: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_layer: Option<RenderLayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_speed: Option<f64>,
    pub run_animation: RotatedAnimation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_sound_animation_positions: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawning_time_modifier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steering: Option<SteeringSettings>,
    pub vision_distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walking_sound: Option<Sound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warcry: Option<Sound>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UpgradeItemPrototype {
    #[serde(flatten)]
    pub parent: SelectionToolPrototype,
    pub alt_select: SelectionModeData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_include_tiles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_label_for_cursor_render: Option<bool>,
    pub select: SelectionModeData,
    pub stack_size: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UseEntityInEnergyProductionAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_condition: Option<ItemIDFilter>,
    pub entity: EntityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produced_condition: Option<ItemIDFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_to_build: Option<EntityID>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UseItemAchievementPrototype {
    #[serde(flatten)]
    pub parent: AchievementPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    pub limit_quality: QualityID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_to_one_game: Option<bool>,
    pub to_use: ItemID,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UtilityConstants {
    #[serde(flatten)]
    pub parent: PrototypeBase,
    pub agricultural_range_visualization_color: Color,
    pub artillery_range_visualization_color: Color,
    pub asteroid_collector_blockage_update_tile_distance: u32,
    pub asteroid_collector_max_nurbs_control_point_separation: f64,
    pub asteroid_collector_navmesh_refresh_tick_interval: u32,
    pub asteroid_collector_static_head_swing_segment_count: u32,
    pub asteroid_collector_static_head_swing_strength_scale: f64,
    pub asteroid_fading_range: f64,
    pub asteroid_position_offset_to_speed_coefficient: f64,
    pub asteroid_spawning_offset: SimpleBoundingBox,
    pub asteroid_spawning_with_random_orientation_max_speed: f64,
    pub blueprint_big_slots_per_row: u8,
    pub blueprint_small_slots_per_row: u8,
    pub bonus_gui_ordering: BonusUtilityConstants,
    pub building_buildable_tint: Color,
    pub building_buildable_too_far_tint: Color,
    pub building_collision_mask: CollisionMaskConnector,
    pub building_ignorable_tint: Color,
    pub building_no_tint: Color,
    pub building_not_buildable_tint: Color,
    pub capsule_range_visualization_color: Color,
    pub capture_water_mask_at_layer: u8,
    pub chart: ChartUtilityConstants,
    pub chart_search_highlight: Color,
    pub checkerboard_black: Color,
    pub checkerboard_white: Color,
    pub clear_cursor_volume_modifier: f64,
    pub clipboard_history_size: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_filters: Option<Vec<ColorFilterData>>,
    pub construction_robots_use_busy_robots_queue: bool,
    pub count_button_size: i32,
    pub crafting_queue_slots_per_row: u8,
    pub daytime_color_lookup: DaytimeColorLookupTable,
    pub deconstruct_mark_tint: Color,
    pub default_alert_icon_scale: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_alert_icon_scale_by_type: Option<std::collections::HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_alert_icon_shift_by_type: Option<
        std::collections::HashMap<String, Vector>,
    >,
    pub default_collision_masks: std::collections::HashMap<
        String,
        CollisionMaskConnector,
    >,
    pub default_enemy_force_color: Color,
    pub default_item_weight: f64,
    pub default_other_force_color: Color,
    pub default_pipeline_extent: f64,
    pub default_planet_procession_set: ProcessionSet,
    pub default_platform_procession_set: ProcessionSet,
    pub default_platform_surface_render_parameters: SurfaceRenderParameters,
    pub default_player_force_color: Color,
    pub default_rocket_lift_weight: f64,
    pub default_scorch_mark_color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_trigger_target_mask_by_type: Option<
        std::collections::HashMap<String, TriggerTargetMask>,
    >,
    pub disabled_recipe_slot_background_tint: Color,
    pub disabled_recipe_slot_tint: Color,
    pub drop_item_radius: f64,
    pub dynamic_recipe_overload_factor: f64,
    pub ejected_item_direction_variation: f64,
    pub ejected_item_friction: f64,
    pub ejected_item_lifetime: u64,
    pub ejected_item_speed: f64,
    pub enabled_recipe_slot_tint: Color,
    pub enemies_in_simulation_volume_modifier: f64,
    pub entity_button_background_color: Color,
    pub entity_renderer_search_box_limits: EntityRendererSearchBoxLimits,
    pub environment_sounds_transition_fade_in_ticks: u32,
    pub equipment_default_background_border_color: Color,
    pub equipment_default_background_color: Color,
    pub equipment_default_grabbed_background_color: Color,
    pub equipment_disabled_background_tint: Color,
    pub equipment_disabled_tint: Color,
    pub explosions_in_simulation_volume_modifier: f64,
    pub factoriopedia_recycling_recipe_categories: Vec<RecipeCategoryID>,
    pub far_away_chunk_generation_radius: u8,
    pub feedback_screenshot_file_name: String,
    pub feedback_screenshot_subfolder_name: String,
    pub filter_outline_color: Color,
    pub flying_text_ttl: i32,
    pub forced_enabled_recipe_slot_background_tint: Color,
    pub freezing_temperature: f64,
    pub frozen_color_lookup: ColorLookupTable,
    pub ghost_layer: CollisionLayerID,
    pub ghost_product_count_tint: Color,
    pub ghost_shader_tint: GhostTintSet,
    pub ghost_shaderless_tint: GhostTintSet,
    pub ghost_shimmer_settings: GhostShimmerConfig,
    pub gui_remark_color: Color,
    pub gui_search_match_background_color: Color,
    pub gui_search_match_foreground_color: Color,
    pub huge_area_size: f64,
    pub huge_platform_animation_sound_area: f64,
    pub icon_shadow_color: Color,
    pub icon_shadow_inset: f64,
    pub icon_shadow_radius: f64,
    pub icon_shadow_sharpness: f64,
    pub inserter_hand_stack_items_per_sprite: u32,
    pub inserter_hand_stack_max_sprites: u32,
    pub inventory_width: u8,
    pub item_ammo_magazine_left_bar_color: Color,
    pub item_default_random_tint_strength: Color,
    pub item_health_bar_colors: Vec<ItemHealthColorData>,
    pub item_outline_color: Color,
    pub item_outline_inset: f64,
    pub item_outline_radius: f64,
    pub item_outline_sharpness: f64,
    pub item_tool_durability_bar_color: Color,
    pub landing_area_clear_zone_radius: f64,
    pub landing_area_max_radius: f64,
    pub landing_squash_immunity: u64,
    pub large_area_size: f64,
    pub large_blueprint_area_size: f64,
    pub light_renderer_search_distance_limit: u8,
    pub lightning_attractor_collection_range_color: Color,
    pub lightning_attractor_protection_range_color: Color,
    pub logistic_gui_selected_network_highlight_tint: Color,
    pub logistic_gui_unselected_network_highlight_tint: Color,
    pub logistic_robots_use_busy_robots_queue: bool,
    pub logistic_slots_per_row: u8,
    pub low_energy_robot_estimate_multiplier: f64,
    pub main_menu_background_image_location: FileName,
    pub main_menu_background_vignette_intensity: f64,
    pub main_menu_background_vignette_sharpness: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_menu_simulations: Option<
        std::collections::HashMap<String, SimulationDefinition>,
    >,
    pub manual_rail_building_reach_modifier: f64,
    pub map_editor: EditorUtilityConstants,
    pub max_belt_stack_size: u8,
    pub max_logistic_filter_count: u16,
    pub max_terrain_building_size: u8,
    pub maximum_quality_jump: u8,
    pub maximum_recipe_overload_multiplier: u32,
    pub medium_area_size: f64,
    pub medium_blueprint_area_size: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_bonus_gui_production_bonuses: Option<bool>,
    pub minimum_recipe_overload_multiplier: u32,
    pub missing_preview_sprite_location: FileName,
    pub module_inventory_width: u8,
    pub moving_sound_count_reduction_rate: f64,
    pub player_colors: Vec<PlayerColorData>,
    pub probability_product_count_tint: Color,
    pub quality_selector_dropdown_threshold: u8,
    pub rail_planner_count_button_color: Color,
    pub rail_segment_colors: Vec<Color>,
    pub recipe_step_limit: u32,
    pub remote_view_LPF_max_cutoff_frequency: f64,
    pub remote_view_LPF_min_cutoff_frequency: f64,
    pub script_command_console_chat_color: Color,
    pub select_group_row_count: u8,
    pub select_slot_row_count: u8,
    pub selected_chart_search_highlight: Color,
    pub server_command_console_chat_color: Color,
    pub show_chunk_components_collision_mask: CollisionMaskConnector,
    pub small_area_size: f64,
    pub small_blueprint_area_size: f64,
    pub sound_fade_ticks: u32,
    pub space_LPF_max_cutoff_frequency: f64,
    pub space_LPF_min_cutoff_frequency: f64,
    pub space_platform_acceleration_expression: MathExpression,
    pub space_platform_asteroid_chunk_trajectory_updates_per_tick: u32,
    pub space_platform_dump_cooldown: u32,
    pub space_platform_manual_dump_cooldown: u32,
    pub space_platform_max_relative_speed_deviation_for_asteroid_chunks_update: f64,
    pub space_platform_max_size: SimpleBoundingBox,
    pub space_platform_relative_speed_factor: f64,
    pub space_platform_starfield_movement_vector: Vector,
    pub spawner_evolution_factor_health_modifier: f64,
    pub starmap_orbit_clicked_color: Color,
    pub starmap_orbit_default_color: Color,
    pub starmap_orbit_disabled_color: Color,
    pub starmap_orbit_hovered_color: Color,
    pub tall_entity_smoke_tint: Color,
    pub tall_entity_tint: Color,
    pub time_to_show_full_health_bar: u64,
    pub tooltip_monitor_edge_border: i32,
    pub train_inactivity_wait_condition_default: u32,
    pub train_on_elevated_rail_shadow_shift_multiplier: Vector,
    pub train_path_finding: TrainPathFinderConstants,
    pub train_pushed_by_player_ignores_friction: bool,
    pub train_pushed_by_player_max_acceleration: f64,
    pub train_pushed_by_player_max_speed: f64,
    pub train_temporary_stop_wait_time: u32,
    pub train_time_wait_condition_default: u32,
    pub train_visualization: TrainVisualizationConstants,
    pub trash_inventory_width: u8,
    pub tree_leaf_distortion_distortion_far: Vector,
    pub tree_leaf_distortion_distortion_near: Vector,
    pub tree_leaf_distortion_speed_far: Vector,
    pub tree_leaf_distortion_speed_near: Vector,
    pub tree_leaf_distortion_strength_far: Vector,
    pub tree_leaf_distortion_strength_near: Vector,
    pub tree_shadow_roughness: f64,
    pub tree_shadow_speed: f64,
    pub turret_range_visualization_color: Color,
    pub underground_belt_max_distance_tint: Color,
    pub underground_pipe_max_distance_tint: Color,
    pub unit_group_max_pursue_distance: f64,
    pub unit_group_pathfind_resolution: i8,
    pub walking_sound_count_reduction_rate: f64,
    pub water_collision_mask: TileCollisionMaskConnector,
    pub weapons_in_simulation_volume_modifier: f64,
    pub zero_count_value_tint: Color,
    pub zoom_to_world_can_use_nightvision: bool,
    pub zoom_to_world_daytime_color_lookup: DaytimeColorLookupTable,
    pub zoom_to_world_effect_strength: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UtilitySounds {
    #[serde(flatten)]
    pub parent: PrototypeBase,
    pub achievement_unlocked: Sound,
    pub adjust_blueprint_snapping: Sound,
    pub alert_destroyed: Sound,
    pub armor_insert: Sound,
    pub armor_remove: Sound,
    pub axe_fighting: Sound,
    pub axe_mining_ore: Sound,
    pub axe_mining_stone: Sound,
    pub blueprint_preview_build: Sound,
    pub blueprint_preview_mine: Sound,
    pub build_animated_huge: Sound,
    pub build_animated_large: Sound,
    pub build_animated_medium: Sound,
    pub build_animated_small: Sound,
    pub build_behemoth: Sound,
    pub build_blueprint_huge: Sound,
    pub build_blueprint_large: Sound,
    pub build_blueprint_medium: Sound,
    pub build_blueprint_small: Sound,
    pub build_ghost_upgrade: Sound,
    pub build_ghost_upgrade_cancel: Sound,
    pub build_huge: Sound,
    pub build_large: Sound,
    pub build_medium: Sound,
    pub build_small: Sound,
    pub cannot_build: Sound,
    pub change_quality: Sound,
    pub clear_cursor: Sound,
    pub confirm: Sound,
    pub console_message: Sound,
    pub console_platform_created: Sound,
    pub console_platform_destroyed: Sound,
    pub console_player_changed_logistic_group: Sound,
    pub console_player_died: Sound,
    pub console_player_joined: Sound,
    pub console_player_left: Sound,
    pub console_player_paused_game: Sound,
    pub console_player_research: Sound,
    pub console_player_respawned: Sound,
    pub console_player_resumed_game: Sound,
    pub crafting_finished: Sound,
    pub cycle_blueprint_book: Sound,
    pub deconstruct_behemoth: Sound,
    pub deconstruct_huge: Sound,
    pub deconstruct_large: Sound,
    pub deconstruct_medium: Sound,
    pub deconstruct_robot: Sound,
    pub deconstruct_small: Sound,
    pub default_driving_sound: InterruptibleSound,
    pub default_landing_steps: Sound,
    pub default_manual_repair: Sound,
    pub drop_item: Sound,
    pub entity_settings_copied: Sound,
    pub entity_settings_pasted: Sound,
    pub game_lost: Sound,
    pub game_won: Sound,
    pub gui_click: Sound,
    pub gui_switch: Sound,
    pub gui_tab: Sound,
    pub gui_toggle: Sound,
    pub heat_pipe_walking_sound: Sound,
    pub inventory_click: Sound,
    pub inventory_move: Sound,
    pub item_deleted: Sound,
    pub item_spawned: Sound,
    pub list_box_click: Sound,
    pub metal_walking_sound: Sound,
    pub mining_wood: Sound,
    pub new_objective: Sound,
    pub paste_activated: Sound,
    pub picked_up_item: Sound,
    pub rail_plan_start: Sound,
    pub research_completed: Sound,
    pub rotated_huge: Sound,
    pub rotated_large: Sound,
    pub rotated_medium: Sound,
    pub rotated_small: Sound,
    pub scenario_message: Sound,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_dying_sound: Option<Sound>,
    pub smart_pipette: Sound,
    pub switch_gun: Sound,
    pub toggle_show_entity_info: Sound,
    pub tutorial_notice: Sound,
    pub undo: Sound,
    pub wire_connect_pole: Sound,
    pub wire_disconnect: Sound,
    pub wire_pickup: Sound,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct UtilitySprites {
    #[serde(flatten)]
    pub parent: PrototypeBase,
    pub achievement_label: Sprite,
    pub achievement_label_completed: Sprite,
    pub achievement_label_failed: Sprite,
    pub achievement_warning: Sprite,
    pub add: Sprite,
    pub add_white: Sprite,
    pub alert_arrow: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ammo_damage_modifier_constant: Option<Sprite>,
    pub ammo_damage_modifier_icon: Sprite,
    pub ammo_icon: Sprite,
    pub and_or: Sprite,
    pub any_quality: Sprite,
    pub area_icon: Sprite,
    pub arrow_button: Animation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artillery_range_modifier_constant: Option<Sprite>,
    pub artillery_range_modifier_icon: Sprite,
    pub asteroid_chunk_editor_icon: Sprite,
    pub asteroid_collector_path_blocked_icon: Sprite,
    pub backward_arrow: Sprite,
    pub backward_arrow_black: Sprite,
    pub bar_gray_pip: Sprite,
    pub battery: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beacon_distribution_modifier_constant: Option<Sprite>,
    pub beacon_distribution_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belt_stack_size_bonus_modifier_constant: Option<Sprite>,
    pub belt_stack_size_bonus_modifier_icon: Sprite,
    pub bookmark: Sprite,
    pub brush_circle_shape: Sprite,
    pub brush_icon: Sprite,
    pub brush_square_shape: Sprite,
    pub buildability_collision: Sprite,
    pub buildability_collision_elevated: Sprite,
    pub buildability_elevated_collision_bottom: Sprite,
    pub buildability_elevated_collision_line: Sprite,
    pub buildability_elevated_collision_top: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_inserter_capacity_bonus_modifier_constant: Option<Sprite>,
    pub bulk_inserter_capacity_bonus_modifier_icon: Sprite,
    pub cable_editor_icon: Sprite,
    pub cargo_bay_not_connected_icon: Sprite,
    pub cargo_bay_too_far_from_source_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_landing_pad_count_modifier_constant: Option<Sprite>,
    pub cargo_landing_pad_count_modifier_icon: Sprite,
    pub center: Sprite,
    pub change_recipe: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_recipe_productivity_modifier_constant: Option<Sprite>,
    pub change_recipe_productivity_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_additional_mining_categories_modifier_constant: Option<Sprite>,
    pub character_additional_mining_categories_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_build_distance_modifier_constant: Option<Sprite>,
    pub character_build_distance_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_crafting_speed_modifier_constant: Option<Sprite>,
    pub character_crafting_speed_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_health_bonus_modifier_constant: Option<Sprite>,
    pub character_health_bonus_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_inventory_slots_bonus_modifier_constant: Option<Sprite>,
    pub character_inventory_slots_bonus_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_item_drop_distance_modifier_constant: Option<Sprite>,
    pub character_item_drop_distance_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_item_pickup_distance_modifier_constant: Option<Sprite>,
    pub character_item_pickup_distance_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_logistic_requests_modifier_constant: Option<Sprite>,
    pub character_logistic_requests_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_logistic_trash_slots_modifier_constant: Option<Sprite>,
    pub character_logistic_trash_slots_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_loot_pickup_distance_modifier_constant: Option<Sprite>,
    pub character_loot_pickup_distance_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_mining_speed_modifier_constant: Option<Sprite>,
    pub character_mining_speed_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_reach_distance_modifier_constant: Option<Sprite>,
    pub character_reach_distance_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_resource_reach_distance_modifier_constant: Option<Sprite>,
    pub character_resource_reach_distance_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_running_speed_modifier_constant: Option<Sprite>,
    pub character_running_speed_modifier_icon: Sprite,
    pub check_mark: Sprite,
    pub check_mark_dark_green: Sprite,
    pub check_mark_green: Sprite,
    pub check_mark_white: Sprite,
    pub circuit_network_panel: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliff_deconstruction_enabled_modifier_constant: Option<Sprite>,
    pub cliff_deconstruction_enabled_modifier_icon: Sprite,
    pub cliff_editor_icon: Sprite,
    pub clock: Sprite,
    pub clone: Sprite,
    pub clone_editor_icon: Sprite,
    pub close: Sprite,
    pub close_black: Sprite,
    pub close_fat: Sprite,
    pub close_map_preview: Sprite,
    pub clouds: Animation,
    pub collapse: Sprite,
    pub collapse_dark: Sprite,
    pub color_effect: Sprite,
    pub color_picker: Sprite,
    pub confirm_slot: Sprite,
    pub construction_radius_visualization: Sprite,
    pub controller_joycon_a: Sprite,
    pub controller_joycon_b: Sprite,
    pub controller_joycon_back: Sprite,
    pub controller_joycon_black_a: Sprite,
    pub controller_joycon_black_b: Sprite,
    pub controller_joycon_black_back: Sprite,
    pub controller_joycon_black_dpdown: Sprite,
    pub controller_joycon_black_dpleft: Sprite,
    pub controller_joycon_black_dpright: Sprite,
    pub controller_joycon_black_dpup: Sprite,
    pub controller_joycon_black_left_stick: Sprite,
    pub controller_joycon_black_leftshoulder: Sprite,
    pub controller_joycon_black_leftstick: Sprite,
    pub controller_joycon_black_lefttrigger: Sprite,
    pub controller_joycon_black_paddle1: Sprite,
    pub controller_joycon_black_paddle2: Sprite,
    pub controller_joycon_black_paddle3: Sprite,
    pub controller_joycon_black_paddle4: Sprite,
    pub controller_joycon_black_right_stick: Sprite,
    pub controller_joycon_black_rightshoulder: Sprite,
    pub controller_joycon_black_rightstick: Sprite,
    pub controller_joycon_black_righttrigger: Sprite,
    pub controller_joycon_black_start: Sprite,
    pub controller_joycon_black_x: Sprite,
    pub controller_joycon_black_y: Sprite,
    pub controller_joycon_dpdown: Sprite,
    pub controller_joycon_dpleft: Sprite,
    pub controller_joycon_dpright: Sprite,
    pub controller_joycon_dpup: Sprite,
    pub controller_joycon_left_stick: Sprite,
    pub controller_joycon_leftshoulder: Sprite,
    pub controller_joycon_leftstick: Sprite,
    pub controller_joycon_lefttrigger: Sprite,
    pub controller_joycon_paddle1: Sprite,
    pub controller_joycon_paddle2: Sprite,
    pub controller_joycon_paddle3: Sprite,
    pub controller_joycon_paddle4: Sprite,
    pub controller_joycon_right_stick: Sprite,
    pub controller_joycon_rightshoulder: Sprite,
    pub controller_joycon_rightstick: Sprite,
    pub controller_joycon_righttrigger: Sprite,
    pub controller_joycon_start: Sprite,
    pub controller_joycon_x: Sprite,
    pub controller_joycon_y: Sprite,
    pub controller_ps_a: Sprite,
    pub controller_ps_b: Sprite,
    pub controller_ps_back: Sprite,
    pub controller_ps_black_a: Sprite,
    pub controller_ps_black_b: Sprite,
    pub controller_ps_black_back: Sprite,
    pub controller_ps_black_dpdown: Sprite,
    pub controller_ps_black_dpleft: Sprite,
    pub controller_ps_black_dpright: Sprite,
    pub controller_ps_black_dpup: Sprite,
    pub controller_ps_black_left_stick: Sprite,
    pub controller_ps_black_leftshoulder: Sprite,
    pub controller_ps_black_leftstick: Sprite,
    pub controller_ps_black_lefttrigger: Sprite,
    pub controller_ps_black_right_stick: Sprite,
    pub controller_ps_black_rightshoulder: Sprite,
    pub controller_ps_black_rightstick: Sprite,
    pub controller_ps_black_righttrigger: Sprite,
    pub controller_ps_black_start: Sprite,
    pub controller_ps_black_x: Sprite,
    pub controller_ps_black_y: Sprite,
    pub controller_ps_dpdown: Sprite,
    pub controller_ps_dpleft: Sprite,
    pub controller_ps_dpright: Sprite,
    pub controller_ps_dpup: Sprite,
    pub controller_ps_left_stick: Sprite,
    pub controller_ps_leftshoulder: Sprite,
    pub controller_ps_leftstick: Sprite,
    pub controller_ps_lefttrigger: Sprite,
    pub controller_ps_right_stick: Sprite,
    pub controller_ps_rightshoulder: Sprite,
    pub controller_ps_rightstick: Sprite,
    pub controller_ps_righttrigger: Sprite,
    pub controller_ps_start: Sprite,
    pub controller_ps_x: Sprite,
    pub controller_ps_y: Sprite,
    pub controller_steamdeck_a: Sprite,
    pub controller_steamdeck_b: Sprite,
    pub controller_steamdeck_back: Sprite,
    pub controller_steamdeck_black_a: Sprite,
    pub controller_steamdeck_black_b: Sprite,
    pub controller_steamdeck_black_back: Sprite,
    pub controller_steamdeck_black_dpdown: Sprite,
    pub controller_steamdeck_black_dpleft: Sprite,
    pub controller_steamdeck_black_dpright: Sprite,
    pub controller_steamdeck_black_dpup: Sprite,
    pub controller_steamdeck_black_left_stick: Sprite,
    pub controller_steamdeck_black_leftshoulder: Sprite,
    pub controller_steamdeck_black_leftstick: Sprite,
    pub controller_steamdeck_black_lefttrigger: Sprite,
    pub controller_steamdeck_black_paddle1: Sprite,
    pub controller_steamdeck_black_paddle2: Sprite,
    pub controller_steamdeck_black_paddle3: Sprite,
    pub controller_steamdeck_black_paddle4: Sprite,
    pub controller_steamdeck_black_right_stick: Sprite,
    pub controller_steamdeck_black_rightshoulder: Sprite,
    pub controller_steamdeck_black_rightstick: Sprite,
    pub controller_steamdeck_black_righttrigger: Sprite,
    pub controller_steamdeck_black_start: Sprite,
    pub controller_steamdeck_black_x: Sprite,
    pub controller_steamdeck_black_y: Sprite,
    pub controller_steamdeck_dpdown: Sprite,
    pub controller_steamdeck_dpleft: Sprite,
    pub controller_steamdeck_dpright: Sprite,
    pub controller_steamdeck_dpup: Sprite,
    pub controller_steamdeck_left_stick: Sprite,
    pub controller_steamdeck_leftshoulder: Sprite,
    pub controller_steamdeck_leftstick: Sprite,
    pub controller_steamdeck_lefttrigger: Sprite,
    pub controller_steamdeck_paddle1: Sprite,
    pub controller_steamdeck_paddle2: Sprite,
    pub controller_steamdeck_paddle3: Sprite,
    pub controller_steamdeck_paddle4: Sprite,
    pub controller_steamdeck_right_stick: Sprite,
    pub controller_steamdeck_rightshoulder: Sprite,
    pub controller_steamdeck_rightstick: Sprite,
    pub controller_steamdeck_righttrigger: Sprite,
    pub controller_steamdeck_start: Sprite,
    pub controller_steamdeck_x: Sprite,
    pub controller_steamdeck_y: Sprite,
    pub controller_xbox_a: Sprite,
    pub controller_xbox_b: Sprite,
    pub controller_xbox_back: Sprite,
    pub controller_xbox_black_a: Sprite,
    pub controller_xbox_black_b: Sprite,
    pub controller_xbox_black_back: Sprite,
    pub controller_xbox_black_dpdown: Sprite,
    pub controller_xbox_black_dpleft: Sprite,
    pub controller_xbox_black_dpright: Sprite,
    pub controller_xbox_black_dpup: Sprite,
    pub controller_xbox_black_left_stick: Sprite,
    pub controller_xbox_black_leftshoulder: Sprite,
    pub controller_xbox_black_leftstick: Sprite,
    pub controller_xbox_black_lefttrigger: Sprite,
    pub controller_xbox_black_right_stick: Sprite,
    pub controller_xbox_black_rightshoulder: Sprite,
    pub controller_xbox_black_rightstick: Sprite,
    pub controller_xbox_black_righttrigger: Sprite,
    pub controller_xbox_black_start: Sprite,
    pub controller_xbox_black_x: Sprite,
    pub controller_xbox_black_y: Sprite,
    pub controller_xbox_dpdown: Sprite,
    pub controller_xbox_dpleft: Sprite,
    pub controller_xbox_dpright: Sprite,
    pub controller_xbox_dpup: Sprite,
    pub controller_xbox_left_stick: Sprite,
    pub controller_xbox_leftshoulder: Sprite,
    pub controller_xbox_leftstick: Sprite,
    pub controller_xbox_lefttrigger: Sprite,
    pub controller_xbox_right_stick: Sprite,
    pub controller_xbox_rightshoulder: Sprite,
    pub controller_xbox_rightstick: Sprite,
    pub controller_xbox_righttrigger: Sprite,
    pub controller_xbox_start: Sprite,
    pub controller_xbox_x: Sprite,
    pub controller_xbox_y: Sprite,
    pub copper_wire: Sprite,
    pub copper_wire_highlight: Sprite,
    pub copy: Sprite,
    pub covered_chunk: Sprite,
    pub crafting_machine_recipe_not_unlocked: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_ghost_on_entity_death_modifier_constant: Option<Sprite>,
    pub create_ghost_on_entity_death_modifier_icon: Sprite,
    pub cross_select: Sprite,
    pub crosshair: Sprite,
    pub cursor_box: CursorBoxSpecification,
    pub cursor_icon: Sprite,
    pub custom_tag_icon: Sprite,
    pub custom_tag_in_map_view: Sprite,
    pub danger_icon: Sprite,
    pub deconstruction_mark: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deconstruction_time_to_live_modifier_constant: Option<Sprite>,
    pub deconstruction_time_to_live_modifier_icon: Sprite,
    pub decorative_editor_icon: Sprite,
    pub default_ammo_damage_modifier_icon: Sprite,
    pub default_gun_speed_modifier_icon: Sprite,
    pub default_turret_attack_modifier_icon: Sprite,
    pub destination_full_icon: Sprite,
    pub destroyed_icon: Sprite,
    pub down_arrow: Sprite,
    pub downloaded: Sprite,
    pub downloading: Sprite,
    pub dropdown: Sprite,
    pub editor_pause: Sprite,
    pub editor_play: Sprite,
    pub editor_selection: Sprite,
    pub editor_speed_down: Sprite,
    pub editor_speed_up: Sprite,
    pub electricity_icon: Sprite,
    pub electricity_icon_unplugged: Sprite,
    pub empty_ammo_slot: Sprite,
    pub empty_armor_slot: Sprite,
    pub empty_drop_cargo_slot: Sprite,
    pub empty_gun_slot: Sprite,
    pub empty_inserter_hand_slot: Sprite,
    pub empty_module_slot: Sprite,
    pub empty_robot_material_slot: Sprite,
    pub empty_robot_slot: Sprite,
    pub empty_trash_slot: Sprite,
    pub enemy_force_icon: Sprite,
    pub enter: Sprite,
    pub entity_editor_icon: Sprite,
    pub entity_info_dark_background: Sprite,
    pub equipment_collision: Sprite,
    pub equipment_grid: Sprite,
    pub equipment_grid_small: Sprite,
    pub equipment_slot: Sprite,
    pub expand: Sprite,
    pub expand_dots: Sprite,
    pub explosion_chart_visualization: Animation,
    pub export: Sprite,
    pub export_slot: Sprite,
    pub feedback: Sprite,
    pub filter_blacklist: Sprite,
    pub fluid_icon: Sprite,
    pub fluid_indication_arrow: Sprite,
    pub fluid_indication_arrow_both_ways: Sprite,
    pub fluid_mixing_icon: Sprite,
    pub fluid_visualization_connection: Sprite,
    pub fluid_visualization_connection_both_ways: Sprite,
    pub fluid_visualization_connection_underground: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_robot_lifetime_modifier_constant: Option<Sprite>,
    pub follower_robot_lifetime_modifier_icon: Sprite,
    pub force_editor_icon: Sprite,
    pub force_ghost_cursor: Sprite,
    pub force_tile_ghost_cursor: Sprite,
    pub forward_arrow: Sprite,
    pub forward_arrow_black: Sprite,
    pub frozen_icon: Sprite,
    pub fuel_icon: Sprite,
    pub game_stopped_visualization: Sprite,
    pub ghost_bar_pip: Sprite,
    pub ghost_cursor: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub give_item_modifier_constant: Option<Sprite>,
    pub give_item_modifier_icon: Sprite,
    pub go_to_arrow: Sprite,
    pub gps_map_icon: Sprite,
    pub gradient: Sprite,
    pub green_circle: Sprite,
    pub green_dot: Sprite,
    pub green_wire: Sprite,
    pub green_wire_highlight: Sprite,
    pub grey_placement_indicator_leg: Sprite,
    pub grey_rail_signal_placement_indicator: Sprite,
    pub grid_view: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gun_speed_modifier_constant: Option<Sprite>,
    pub gun_speed_modifier_icon: Sprite,
    pub hand: Sprite,
    pub hand_black: Sprite,
    pub health_bar_green_pip: Sprite,
    pub health_bar_red_pip: Sprite,
    pub health_bar_yellow_pip: Sprite,
    pub heat_exchange_indication: Sprite,
    pub hint_arrow_down: Sprite,
    pub hint_arrow_left: Sprite,
    pub hint_arrow_right: Sprite,
    pub hint_arrow_up: Sprite,
    pub import: Sprite,
    pub import_slot: Sprite,
    pub indication_arrow: Sprite,
    pub indication_line: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inserter_stack_size_bonus_modifier_constant: Option<Sprite>,
    pub inserter_stack_size_bonus_modifier_icon: Sprite,
    pub item_editor_icon: Sprite,
    pub item_to_be_delivered_symbol: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub laboratory_productivity_modifier_constant: Option<Sprite>,
    pub laboratory_productivity_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub laboratory_speed_modifier_constant: Option<Sprite>,
    pub laboratory_speed_modifier_icon: Sprite,
    pub left_arrow: Sprite,
    pub light_cone: Sprite,
    pub light_medium: Sprite,
    pub light_small: Sprite,
    pub lightning_warning_icon: Sprite,
    pub line_icon: Sprite,
    pub list_view: Sprite,
    pub logistic_network_panel_black: Sprite,
    pub logistic_network_panel_white: Sprite,
    pub logistic_radius_visualization: Sprite,
    pub lua_snippet_tool_icon: Sprite,
    pub map: Sprite,
    pub map_exchange_string: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cargo_bay_unloading_distance_modifier_constant: Option<Sprite>,
    pub max_cargo_bay_unloading_distance_modifier_icon: Sprite,
    pub max_distance_underground_remove_belts: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failed_attempts_per_tick_per_construction_queue_modifier_constant: Option<
        Sprite,
    >,
    pub max_failed_attempts_per_tick_per_construction_queue_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_successful_attempts_per_tick_per_construction_queue_modifier_constant: Option<
        Sprite,
    >,
    pub max_successful_attempts_per_tick_per_construction_queue_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_following_robots_count_modifier_constant: Option<Sprite>,
    pub maximum_following_robots_count_modifier_icon: Sprite,
    pub medium_gui_arrow: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_drill_productivity_bonus_modifier_constant: Option<Sprite>,
    pub mining_drill_productivity_bonus_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mining_with_fluid_modifier_constant: Option<Sprite>,
    pub mining_with_fluid_modifier_icon: Sprite,
    pub missing_icon: Sprite,
    pub missing_mod_icon: Sprite,
    pub mod_category: Sprite,
    pub mod_dependency_arrow: Sprite,
    pub mod_downloads_count: Sprite,
    pub mod_last_updated: Sprite,
    pub mouse_cursor: Sprite,
    pub mouse_cursor_macos: Sprite,
    pub move_tag: Sprite,
    pub multiplayer_waiting_icon: Sprite,
    pub nature_icon: Sprite,
    pub navmesh_pending_icon: Animation,
    pub neutral_force_icon: Sprite,
    pub no_building_material_icon: Sprite,
    pub no_nature_icon: Sprite,
    pub no_path_icon: Sprite,
    pub no_platform_storage_space_icon: Sprite,
    pub no_roboport_storage_space_icon: Sprite,
    pub no_storage_space_icon: Sprite,
    pub none_editor_icon: Sprite,
    pub not_available: Sprite,
    pub not_available_black: Sprite,
    pub not_enough_construction_robots_icon: Sprite,
    pub not_enough_repair_packs_icon: Sprite,
    pub not_played_yet_dark_green: Sprite,
    pub not_played_yet_green: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nothing_modifier_constant: Option<Sprite>,
    pub nothing_modifier_icon: Sprite,
    pub notification: Sprite,
    pub output_console_gradient: Sprite,
    pub paint_bucket_icon: Sprite,
    pub parametrise: Sprite,
    pub pause: Sprite,
    pub pin_arrow: Sprite,
    pub pin_center: Sprite,
    pub pipeline_disabled_icon: Sprite,
    pub placement_indicator_leg: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_entity_build_animations: Option<EntityBuildAnimations>,
    pub play: Sprite,
    pub played_dark_green: Sprite,
    pub played_green: Sprite,
    pub player_force_icon: Sprite,
    pub preset: Sprite,
    pub pump_cannot_connect_icon: Sprite,
    pub questionmark: Sprite,
    pub rail_path_not_possible: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rail_planner_allow_elevated_rails_modifier_constant: Option<Sprite>,
    pub rail_planner_allow_elevated_rails_modifier_icon: Sprite,
    pub rail_planner_indication_arrow: Sprite,
    pub rail_planner_indication_arrow_anchored: Sprite,
    pub rail_planner_indication_arrow_too_far: Sprite,
    pub rail_signal_placement_indicator: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rail_support_on_deep_oil_ocean_modifier_constant: Option<Sprite>,
    pub rail_support_on_deep_oil_ocean_modifier_icon: Sprite,
    pub rail_support_placement_indicator: Sprite,
    pub reassign: Sprite,
    pub rebuild_mark: Sprite,
    pub recharge_icon: Sprite,
    pub recipe_arrow: Sprite,
    pub recipe_ghost_arrow: Sprite,
    pub recipe_potential_arrow: Sprite,
    pub red_wire: Sprite,
    pub red_wire_highlight: Sprite,
    pub reference_point: Sprite,
    pub refresh: Sprite,
    pub refresh_white: Animation,
    pub rename_icon: Sprite,
    pub reset: Sprite,
    pub reset_white: Sprite,
    pub resource_editor_icon: Sprite,
    pub resources_depleted_icon: Sprite,
    pub right_arrow: Sprite,
    pub robot_slot: Sprite,
    pub scripting_editor_icon: Sprite,
    pub search: Sprite,
    pub search_icon: Sprite,
    pub select_icon_black: Sprite,
    pub select_icon_white: Sprite,
    pub set_bar_slot: Sprite,
    pub shield_bar_pip: Sprite,
    pub shoot_cursor_green: Sprite,
    pub shoot_cursor_red: Sprite,
    pub short_indication_line: Sprite,
    pub short_indication_line_green: Sprite,
    pub show_electric_network_in_map_view: Sprite,
    pub show_logistics_network_in_map_view: Sprite,
    pub show_pipelines_in_map_view: Sprite,
    pub show_player_names_in_map_view: Sprite,
    pub show_rail_signal_states_in_map_view: Sprite,
    pub show_recipe_icons_in_map_view: Sprite,
    pub show_tags_in_map_view: Sprite,
    pub show_train_station_names_in_map_view: Sprite,
    pub show_turret_range_in_map_view: Sprite,
    pub show_worker_robots_in_map_view: Sprite,
    pub shuffle: Sprite,
    pub side_menu_achievements_icon: Sprite,
    pub side_menu_blueprint_library_icon: Sprite,
    pub side_menu_bonus_icon: Sprite,
    pub side_menu_factoriopedia_icon: Sprite,
    pub side_menu_logistic_networks_icon: Sprite,
    pub side_menu_map_icon: Sprite,
    pub side_menu_menu_icon: Sprite,
    pub side_menu_players_icon: Sprite,
    pub side_menu_production_icon: Sprite,
    pub side_menu_space_platforms_icon: Sprite,
    pub side_menu_technology_icon: Sprite,
    pub side_menu_train_icon: Sprite,
    pub side_menu_tutorials_icon: Sprite,
    pub slot: Sprite,
    pub slots_view: Sprite,
    pub small_gui_arrow: Sprite,
    pub sort_by_name: Sprite,
    pub sort_by_time: Sprite,
    pub space_age_icon: Sprite,
    pub spawn_flag: Sprite,
    pub speed_down: Sprite,
    pub speed_up: Sprite,
    pub spray_icon: Sprite,
    pub starmap_platform_moving: Sprite,
    pub starmap_platform_moving_clicked: Sprite,
    pub starmap_platform_moving_hovered: Sprite,
    pub starmap_platform_stacked: Sprite,
    pub starmap_platform_stacked_clicked: Sprite,
    pub starmap_platform_stacked_hovered: Sprite,
    pub starmap_platform_stopped: Sprite,
    pub starmap_platform_stopped_clicked: Sprite,
    pub starmap_platform_stopped_hovered: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starmap_star: Option<Sprite>,
    pub station_name: Sprite,
    pub status_blue: Sprite,
    pub status_inactive: Sprite,
    pub status_not_working: Sprite,
    pub status_working: Sprite,
    pub status_yellow: Sprite,
    pub stop: Sprite,
    pub surface_editor_icon: Sprite,
    pub sync_mods: Sprite,
    pub technology_white: Sprite,
    pub tick_custom: Sprite,
    pub tick_once: Sprite,
    pub tick_sixty: Sprite,
    pub tile_editor_icon: Sprite,
    pub tile_ghost_cursor: Sprite,
    pub time_editor_icon: Sprite,
    pub tip_icon: Sprite,
    pub too_far: Sprite,
    pub too_far_from_roboport_icon: Sprite,
    pub tooltip_category_spoilable: Sprite,
    pub track_button: Sprite,
    pub track_button_white: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub train_braking_force_bonus_modifier_constant: Option<Sprite>,
    pub train_braking_force_bonus_modifier_icon: Sprite,
    pub train_stop_disabled_in_map_view: Sprite,
    pub train_stop_full_in_map_view: Sprite,
    pub train_stop_in_map_view: Sprite,
    pub train_stop_placement_indicator: Sprite,
    pub trash: Sprite,
    pub trash_white: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turret_attack_modifier_constant: Option<Sprite>,
    pub turret_attack_modifier_icon: Sprite,
    pub unclaimed_cargo_icon: Sprite,
    pub underground_pipe_connection: Sprite,
    pub underground_remove_belts: Sprite,
    pub underground_remove_pipes: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_circuit_network_modifier_constant: Option<Sprite>,
    pub unlock_circuit_network_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_logistic_network_modifier_constant: Option<Sprite>,
    pub unlock_logistic_network_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_quality_modifier_constant: Option<Sprite>,
    pub unlock_quality_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_recipe_modifier_constant: Option<Sprite>,
    pub unlock_recipe_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_space_location_modifier_constant: Option<Sprite>,
    pub unlock_space_location_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_space_platforms_modifier_constant: Option<Sprite>,
    pub unlock_space_platforms_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_travel_to_space_platforms_modifier_constant: Option<Sprite>,
    pub unlock_travel_to_space_platforms_modifier_icon: Sprite,
    pub upgrade_blueprint: Sprite,
    pub upgrade_mark: Sprite,
    pub variations_tool_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_logistics_modifier_constant: Option<Sprite>,
    pub vehicle_logistics_modifier_icon: Sprite,
    pub warning: Sprite,
    pub warning_icon: Sprite,
    pub warning_white: Sprite,
    pub white_mask: Sprite,
    pub white_square: Sprite,
    pub white_square_icon: Sprite,
    pub wire_shadow: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_robot_battery_modifier_constant: Option<Sprite>,
    pub worker_robot_battery_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_robot_speed_modifier_constant: Option<Sprite>,
    pub worker_robot_speed_modifier_icon: Sprite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_robot_storage_modifier_constant: Option<Sprite>,
    pub worker_robot_storage_modifier_icon: Sprite,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct ValvePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Animation4Way>,
    pub flow_rate: f64,
    pub fluid_box: FluidBox,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_patch: Option<Sprite4Way>,
    pub mode: ValveMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VehiclePrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_passengers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_driving: Option<bool>,
    pub braking_force: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_exploration_radius: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crash_trigger: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_category: Option<String>,
    pub energy_per_hit_point: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment_grid: Option<EquipmentGridID>,
    pub friction_force: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_speed_to_volume_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimap_representation: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_minimap_representation: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_trigger: Option<TriggerEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_trigger_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terrain_friction_modifier: Option<f64>,
    pub weight: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct VirtualSignalPrototype {
    #[serde(flatten)]
    pub parent: Prototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Visitable)]
pub struct WallPrototype {
    #[serde(flatten)]
    pub parent: EntityWithOwnerPrototype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_connector: Option<CircuitConnectorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_wire_max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_gate_visualization: Option<Sprite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_output_signal: Option<SignalIDConnector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_circuit_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_copper_wires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pictures: Option<WallPictures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_merge_group: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_diode_green: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_diode_green_light_bottom: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_diode_green_light_left: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_diode_green_light_right: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_diode_green_light_top: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_diode_red: Option<Sprite4Way>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_diode_red_light_bottom: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_diode_red_light_left: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_diode_red_light_right: Option<LightDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_diode_red_light_top: Option<LightDefinition>,
}
