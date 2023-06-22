#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::prelude::AudioSource;
use bevy::prelude::{Handle, Resource};
use bevy_asset_loader::prelude::*;

// amazing_1.mp3
// amazing_2.mp3
// blackhole.mp3
// boingmachine.mp3
// bomb.mp3
// bonus.mp3
// build1.mp3
// build2.mp3
// build3.mp3
// build4.mp3
// build5.mp3
// build6.mp3
// build7.mp3
// build8.mp3
// build9.mp3
// build10.mp3
// build11.mp3
// buildsp1.mp3
// buildsp2.mp3
// buildsp3.mp3
// buildsp4.mp3
// buildsp5.mp3
// buildsp6.mp3
// comfused.mp3
// comfusedrobot.mp3
// comfusedrobot2.mp3
// comfusedrobot3.mp3
// comfusedrobot4.mp3
// comfusedrobot5.mp3
// comfusedrobot6.mp3
// congs.mp3
// dangerous.mp3
// dangerous_2.mp3
// dangerous_3.mp3
// dangerous_4.mp3
// dangerous_5.mp3
// dangerous_6.mp3
// dangerous_7.mp3
// dangerous_8.mp3
// dangerous_9.mp3
// deadmyforce.mp3
// destinyscoming.mp3
// destroyed_stones.mp3
// destroyer_1.mp3
// destroyer_2.mp3
// digitaldiamond.mp3
// digitalrolling.mp3
// digitalsaw.mp3
// digitsaw2.mp3
// energy_upload_loop.mp3
// error.mp3
// evil_robot.mp3
// evil_robot2.mp3
// exp_1.mp3
// evil_robot3.mp3
// exp_2.mp3
// factory.mp3
// failed.mp3
// fantasy.mp3
// fantasy3.mp3
// fantasy4.mp3
// fatal.mp3
// flame.mp3
// flame_2.mp3
// flame_3.mp3
// flamestart.mp3
// forcedead.mp3
// forcedead3.mp3
// frog.mp3
// further.mp3
// furtherlouder.mp3
// gameover_loud.mp3
// gmentert_loud.mp3
// gum.mp3
// hanging.mp3
// highspace.mp3
// holy_moment.mp3
// iamnotufo.mp3
// invicible.mp3
// laser.mp3
// laser2.mp3
// laser3.mp3
// laser34.mp3
// laser_interceptor.mp3
// laser_interceptor_2.mp3
// laserdot.mp3
// laserexp2.mp3
// laserexp_loud.mp3
// lowR.mp3
// lowR2.mp3
// lowR3.mp3
// lowR4.mp3
// lowbattery.mp3
// magic.mp3
// magic_2.mp3
// magic_3.mp3
// magic_4.mp3
// message2.mp3
// message3.mp3
// message_loud.mp3
// moment2.mp3
// monster_1.mp3
// monster_2.mp3
// monster_3.mp3
// monster_4.mp3
// monster_5.mp3
// monster_6.mp3
// monster_7.mp3
// monsterheart.mp3
// monsterheart2.mp3
// moreexplosion.mp3
// mystic.mp3
// mystic2.mp3
// mystic3.mp3
// nature.mp3
// nature2.mp3
// nature3.mp3
// nature4.mp3
// nature5.mp3
// nightmare.mp3
// poisongas.mp3
// proton.mp3
// proton2.mp3
// proton3.mp3
// proton4.mp3
// proton5.mp3
// psz_dead.mp3
// psz_ready.mp3
// psz_ready2.mp3
// psz_ready3.mp3
// psz_ready4.mp3
// psz_ready5.mp3
// psz_ready6.mp3
// psz_ready7.mp3
// psz_ready8.mp3
// psz_select.mp3
// radioactivity.mp3
// recall.mp3
// reload_lasergun.mp3
// roboticfactory.mp3
// roboticnoise.mp3
// roboticnoise2.mp3
// shake.mp3
// shake_electrical.mp3
// shipready.mp3
// smf.mp3
// somedead.mp3
// soul_fighter.mp3
// spaceandtime 2.mp3
// spaceandtime.mp3
// spacebug.mp3
// spacebug_1.mp3
// spacebug_2.mp3
// spacebug_3.mp3
// spacebug_4.mp3
// spacebug_5.mp3
// spacesonicnoise.mp3
// sparkles.mp3
// sparkles1.mp3
// specialinterference.mp3
// special_reload.mp3
// spike_1.mp3
// spike_10.mp3
// spike_11.mp3
// spike_12.mp3
// spike_2.mp3
// spike_3.mp3
// spike_4.mp3
// spike_5.mp3
// spike_6.mp3
// spike_7.mp3
// spike_8.mp3
// spike_9.mp3
// splats.mp3
// splats_2.mp3
// steal_1.mp3
// steal_2.mp3
// steal_3.mp3
// teleportawy.mp3
// themoment.mp3
// timeisout.mp3
// timeisright.mp3
// timesisright2.mp3
// timewarp.mp3
// timewarp_2.mp3
// tpX.mp3
// tunnel_destroying.mp3
// ufo5.mp3
// ufodead.mp3
// ufodead2.mp3
// ufodead3.mp3
// ufolouder.mp3
// ufosignal.mp3
// ufosignal2.mp3
// ufosignal3.mp3
// unitready.mp3
// unitready2.mp3
// unitready3.mp3
// whatwasthis.mp3
// whereisthepoint.mp3
// whyamiarobot.mp3
// whyamiarobot2.mp3
// whyamiarobot3.mp3
// whyamiarobot4.mp3
// whyamiarobot5.mp3
// win_loud.mp3
// wonder_1.mp3
// wonder_2.mp3
// wonder_3.mp3
// wonder_4.mp3
// wonder_5.mp3

#[derive(AssetCollection, Resource)]
pub struct SoundCollection {
    #[asset(path = "sounds/amazing_1.mp3")]
    pub amazing_1: Handle<AudioSource>,

    #[asset(path = "sounds/amazing_2.mp3")]
    pub amazing_2: Handle<AudioSource>,

    #[asset(path = "sounds/blackhole.mp3")]
    pub blackhole: Handle<AudioSource>,

    #[asset(path = "sounds/boingmachine.mp3")]
    pub boingmachine: Handle<AudioSource>,

    #[asset(path = "sounds/bomb.mp3")]
    pub bomb: Handle<AudioSource>,

    #[asset(path = "sounds/bonus.mp3")]
    pub bonus: Handle<AudioSource>,

    #[asset(path = "sounds/build1.mp3")]
    pub build1: Handle<AudioSource>,

    #[asset(path = "sounds/build2.mp3")]
    pub build2: Handle<AudioSource>,

    #[asset(path = "sounds/build3.mp3")]
    pub build3: Handle<AudioSource>,

    #[asset(path = "sounds/build4.mp3")]
    pub build4: Handle<AudioSource>,

    #[asset(path = "sounds/build5.mp3")]
    pub build5: Handle<AudioSource>,

    #[asset(path = "sounds/build6.mp3")]
    pub build6: Handle<AudioSource>,

    #[asset(path = "sounds/build7.mp3")]
    pub build7: Handle<AudioSource>,

    #[asset(path = "sounds/build8.mp3")]
    pub build8: Handle<AudioSource>,

    #[asset(path = "sounds/build9.mp3")]
    pub build9: Handle<AudioSource>,

    #[asset(path = "sounds/build10.mp3")]
    pub build10: Handle<AudioSource>,

    #[asset(path = "sounds/build11.mp3")]
    pub build11: Handle<AudioSource>,

    #[asset(path = "sounds/buildsp1.mp3")]
    pub buildsp1: Handle<AudioSource>,

    #[asset(path = "sounds/buildsp2.mp3")]
    pub buildsp2: Handle<AudioSource>,

    #[asset(path = "sounds/buildsp3.mp3")]
    pub buildsp3: Handle<AudioSource>,

    #[asset(path = "sounds/buildsp4.mp3")]
    pub buildsp4: Handle<AudioSource>,

    #[asset(path = "sounds/buildsp5.mp3")]
    pub buildsp5: Handle<AudioSource>,

    #[asset(path = "sounds/buildsp6.mp3")]
    pub buildsp6: Handle<AudioSource>,

    #[asset(path = "sounds/comfused.mp3")]
    pub comfused: Handle<AudioSource>,

    #[asset(path = "sounds/comfusedrobot.mp3")]
    pub comfusedrobot: Handle<AudioSource>,

    #[asset(path = "sounds/comfusedrobot2.mp3")]
    pub comfusedrobot2: Handle<AudioSource>,

    #[asset(path = "sounds/comfusedrobot3.mp3")]
    pub comfusedrobot3: Handle<AudioSource>,

    #[asset(path = "sounds/comfusedrobot4.mp3")]
    pub comfusedrobot4: Handle<AudioSource>,

    #[asset(path = "sounds/comfusedrobot5.mp3")]
    pub comfusedrobot5: Handle<AudioSource>,

    #[asset(path = "sounds/comfusedrobot6.mp3")]
    pub comfusedrobot6: Handle<AudioSource>,

    #[asset(path = "sounds/congs.mp3")]
    pub congs: Handle<AudioSource>,

    #[asset(path = "sounds/dangerous.mp3")]
    pub dangerous: Handle<AudioSource>,

    #[asset(path = "sounds/dangerous_2.mp3")]
    pub dangerous_2: Handle<AudioSource>,

    #[asset(path = "sounds/dangerous_3.mp3")]
    pub dangerous_3: Handle<AudioSource>,

    #[asset(path = "sounds/dangerous_4.mp3")]
    pub dangerous_4: Handle<AudioSource>,

    #[asset(path = "sounds/dangerous_5.mp3")]
    pub dangerous_5: Handle<AudioSource>,

    #[asset(path = "sounds/dangerous_6.mp3")]
    pub dangerous_6: Handle<AudioSource>,

    #[asset(path = "sounds/dangerous_7.mp3")]
    pub dangerous_7: Handle<AudioSource>,

    #[asset(path = "sounds/dangerous_8.mp3")]
    pub dangerous_8: Handle<AudioSource>,

    #[asset(path = "sounds/dangerous_9.mp3")]
    pub dangerous_9: Handle<AudioSource>,

    #[asset(path = "sounds/deadmyforce.mp3")]
    pub deadmyforce: Handle<AudioSource>,

    #[asset(path = "sounds/destinyscoming.mp3")]
    pub destinyscoming: Handle<AudioSource>,

    #[asset(path = "sounds/destroyed_stones.mp3")]
    pub destroyed_stones: Handle<AudioSource>,

    #[asset(path = "sounds/destroyer_1.mp3")]
    pub destroyer_1: Handle<AudioSource>,

    #[asset(path = "sounds/destroyer_2.mp3")]
    pub destroyer_2: Handle<AudioSource>,

    #[asset(path = "sounds/digitaldiamond.mp3")]
    pub digitaldiamond: Handle<AudioSource>,

    #[asset(path = "sounds/digitalrolling.mp3")]
    pub digitalrolling: Handle<AudioSource>,

    #[asset(path = "sounds/digitalsaw.mp3")]
    pub digitalsaw: Handle<AudioSource>,

    #[asset(path = "sounds/digitsaw2.mp3")]
    pub digitsaw2: Handle<AudioSource>,

    #[asset(path = "sounds/energy_upload_loop.mp3")]
    pub energy_upload_loop: Handle<AudioSource>,

    #[asset(path = "sounds/error.mp3")]
    pub error: Handle<AudioSource>,

    #[asset(path = "sounds/evil_robot.mp3")]
    pub evil_robot: Handle<AudioSource>,

    #[asset(path = "sounds/evil_robot2.mp3")]
    pub evil_robot2: Handle<AudioSource>,

    #[asset(path = "sounds/exp_1.mp3")]
    pub exp_1: Handle<AudioSource>,

    #[asset(path = "sounds/exp_2.mp3")]
    pub exp_2: Handle<AudioSource>,

    #[asset(path = "sounds/evil_robot3.mp3")]
    pub evil_robot3: Handle<AudioSource>,

    #[asset(path = "sounds/factory.mp3")]
    pub factory: Handle<AudioSource>,

    #[asset(path = "sounds/failed.mp3")]
    pub failed: Handle<AudioSource>,

    #[asset(path = "sounds/fantasy.mp3")]
    pub fantasy: Handle<AudioSource>,

    #[asset(path = "sounds/fantasy3.mp3")]
    pub fantasy3: Handle<AudioSource>,

    #[asset(path = "sounds/fantasy4.mp3")]
    pub fantasy4: Handle<AudioSource>,

    #[asset(path = "sounds/fatal.mp3")]
    pub fatal: Handle<AudioSource>,

    #[asset(path = "sounds/flame.mp3")]
    pub flame: Handle<AudioSource>,

    #[asset(path = "sounds/flame_2.mp3")]
    pub flame_2: Handle<AudioSource>,

    #[asset(path = "sounds/flame_3.mp3")]
    pub flame_3: Handle<AudioSource>,

    #[asset(path = "sounds/flamestart.mp3")]
    pub flamestart: Handle<AudioSource>,

    #[asset(path = "sounds/forcedead.mp3")]
    pub forcedead: Handle<AudioSource>,

    #[asset(path = "sounds/forcedead3.mp3")]
    pub forcedead3: Handle<AudioSource>,

    #[asset(path = "sounds/frog.mp3")]
    pub frog: Handle<AudioSource>,

    #[asset(path = "sounds/further.mp3")]
    pub further: Handle<AudioSource>,

    #[asset(path = "sounds/furtherlouder.mp3")]
    pub furtherlouder: Handle<AudioSource>,

    #[asset(path = "sounds/gameover_loud.mp3")]
    pub gameover_loud: Handle<AudioSource>,

    #[asset(path = "sounds/gmentert_loud.mp3")]
    pub gmentert_loud: Handle<AudioSource>,

    #[asset(path = "sounds/gum.mp3")]
    pub gum: Handle<AudioSource>,

    #[asset(path = "sounds/hanging.mp3")]
    pub hanging: Handle<AudioSource>,

    #[asset(path = "sounds/highspace.mp3")]
    pub highspace: Handle<AudioSource>,

    #[asset(path = "sounds/holy_moment.mp3")]
    pub holy_moment: Handle<AudioSource>,

    #[asset(path = "sounds/iamnotufo.mp3")]
    pub iamnotufo: Handle<AudioSource>,

    #[asset(path = "sounds/invicible.mp3")]
    pub invicible: Handle<AudioSource>,

    #[asset(path = "sounds/laser.mp3")]
    pub laser: Handle<AudioSource>,

    #[asset(path = "sounds/laser2.mp3")]
    pub laser2: Handle<AudioSource>,

    #[asset(path = "sounds/laser3.mp3")]
    pub laser3: Handle<AudioSource>,

    #[asset(path = "sounds/laser34.mp3")]
    pub laser34: Handle<AudioSource>,
}
