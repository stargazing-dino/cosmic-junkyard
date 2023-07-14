#![allow(dead_code)]
use bevy::asset::AssetServer;
use bevy::prelude::AudioSource;
use bevy::prelude::{Handle, Resource};
use bevy_asset_loader::prelude::*;

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

    #[asset(path = "sounds/laser_interceptor.mp3")]
    pub laser_interceptor: Handle<AudioSource>,

    #[asset(path = "sounds/laser_interceptor_2.mp3")]
    pub laser_interceptor_2: Handle<AudioSource>,

    #[asset(path = "sounds/laserdot.mp3")]
    pub laserdot: Handle<AudioSource>,

    #[asset(path = "sounds/laserexp2.mp3")]
    pub laserexp2: Handle<AudioSource>,

    #[asset(path = "sounds/laserexp_loud.mp3")]
    pub laserexp_loud: Handle<AudioSource>,

    #[asset(path = "sounds/lowR.mp3")]
    pub low_r: Handle<AudioSource>,

    #[asset(path = "sounds/lowR2.mp3")]
    pub low_r2: Handle<AudioSource>,

    #[asset(path = "sounds/lowR3.mp3")]
    pub low_r3: Handle<AudioSource>,

    #[asset(path = "sounds/lowR4.mp3")]
    pub low_r4: Handle<AudioSource>,

    #[asset(path = "sounds/lowbattery.mp3")]
    pub lowbattery: Handle<AudioSource>,

    #[asset(path = "sounds/magic.mp3")]
    pub magic: Handle<AudioSource>,

    #[asset(path = "sounds/magic_2.mp3")]
    pub magic_2: Handle<AudioSource>,

    #[asset(path = "sounds/magic_3.mp3")]
    pub magic_3: Handle<AudioSource>,

    #[asset(path = "sounds/magic_4.mp3")]
    pub magic_4: Handle<AudioSource>,

    #[asset(path = "sounds/message2.mp3")]
    pub message2: Handle<AudioSource>,

    #[asset(path = "sounds/message3.mp3")]
    pub message3: Handle<AudioSource>,

    #[asset(path = "sounds/message_loud.mp3")]
    pub message_loud: Handle<AudioSource>,

    #[asset(path = "sounds/moment2.mp3")]
    pub moment2: Handle<AudioSource>,

    #[asset(path = "sounds/monster_1.mp3")]
    pub monster_1: Handle<AudioSource>,

    #[asset(path = "sounds/monster_2.mp3")]
    pub monster_2: Handle<AudioSource>,

    #[asset(path = "sounds/monster_3.mp3")]
    pub monster_3: Handle<AudioSource>,

    #[asset(path = "sounds/monster_4.mp3")]
    pub monster_4: Handle<AudioSource>,

    #[asset(path = "sounds/monster_5.mp3")]
    pub monster_5: Handle<AudioSource>,

    #[asset(path = "sounds/monster_6.mp3")]
    pub monster_6: Handle<AudioSource>,

    #[asset(path = "sounds/monster_7.mp3")]
    pub monster_7: Handle<AudioSource>,

    #[asset(path = "sounds/monsterheart.mp3")]
    pub monsterheart: Handle<AudioSource>,

    #[asset(path = "sounds/monsterheart2.mp3")]
    pub monsterheart2: Handle<AudioSource>,

    #[asset(path = "sounds/moreexplosion.mp3")]
    pub moreexplosion: Handle<AudioSource>,

    #[asset(path = "sounds/mystic.mp3")]
    pub mystic: Handle<AudioSource>,

    #[asset(path = "sounds/mystic2.mp3")]
    pub mystic2: Handle<AudioSource>,

    #[asset(path = "sounds/mystic3.mp3")]
    pub mystic3: Handle<AudioSource>,

    #[asset(path = "sounds/nature.mp3")]
    pub nature: Handle<AudioSource>,

    #[asset(path = "sounds/nature2.mp3")]
    pub nature2: Handle<AudioSource>,

    #[asset(path = "sounds/nature3.mp3")]
    pub nature3: Handle<AudioSource>,

    #[asset(path = "sounds/nature4.mp3")]
    pub nature4: Handle<AudioSource>,

    #[asset(path = "sounds/nature5.mp3")]
    pub nature5: Handle<AudioSource>,

    #[asset(path = "sounds/nightmare.mp3")]
    pub nightmare: Handle<AudioSource>,

    #[asset(path = "sounds/poisongas.mp3")]
    pub poisongas: Handle<AudioSource>,

    #[asset(path = "sounds/proton.mp3")]
    pub proton: Handle<AudioSource>,

    #[asset(path = "sounds/proton2.mp3")]
    pub proton2: Handle<AudioSource>,

    #[asset(path = "sounds/proton3.mp3")]
    pub proton3: Handle<AudioSource>,

    #[asset(path = "sounds/proton4.mp3")]
    pub proton4: Handle<AudioSource>,

    #[asset(path = "sounds/proton5.mp3")]
    pub proton5: Handle<AudioSource>,

    #[asset(path = "sounds/psz_dead.mp3")]
    pub psz_dead: Handle<AudioSource>,

    #[asset(path = "sounds/psz_ready.mp3")]
    pub psz_ready: Handle<AudioSource>,

    #[asset(path = "sounds/psz_ready2.mp3")]
    pub psz_ready2: Handle<AudioSource>,

    #[asset(path = "sounds/psz_ready3.mp3")]
    pub psz_ready3: Handle<AudioSource>,

    #[asset(path = "sounds/psz_ready4.mp3")]
    pub psz_ready4: Handle<AudioSource>,

    #[asset(path = "sounds/psz_ready5.mp3")]
    pub psz_ready5: Handle<AudioSource>,

    #[asset(path = "sounds/psz_ready6.mp3")]
    pub psz_ready6: Handle<AudioSource>,

    #[asset(path = "sounds/psz_ready7.mp3")]
    pub psz_ready7: Handle<AudioSource>,

    #[asset(path = "sounds/psz_ready8.mp3")]
    pub psz_ready8: Handle<AudioSource>,

    #[asset(path = "sounds/psz_select.mp3")]
    pub psz_select: Handle<AudioSource>,

    #[asset(path = "sounds/radioactivity.mp3")]
    pub radioactivity: Handle<AudioSource>,

    #[asset(path = "sounds/recall.mp3")]
    pub recall: Handle<AudioSource>,

    #[asset(path = "sounds/reload_lasergun.mp3")]
    pub reload_lasergun: Handle<AudioSource>,

    #[asset(path = "sounds/roboticfactory.mp3")]
    pub roboticfactory: Handle<AudioSource>,

    #[asset(path = "sounds/roboticnoise.mp3")]
    pub roboticnoise: Handle<AudioSource>,

    #[asset(path = "sounds/roboticnoise2.mp3")]
    pub roboticnoise2: Handle<AudioSource>,

    #[asset(path = "sounds/shake.mp3")]
    pub shake: Handle<AudioSource>,

    #[asset(path = "sounds/shake_electrical.mp3")]
    pub shake_electrical: Handle<AudioSource>,

    #[asset(path = "sounds/shipready.mp3")]
    pub shipready: Handle<AudioSource>,

    #[asset(path = "sounds/smf.mp3")]
    pub smf: Handle<AudioSource>,

    #[asset(path = "sounds/somedead.mp3")]
    pub somedead: Handle<AudioSource>,

    #[asset(path = "sounds/soul_fighter.mp3")]
    pub soul_fighter: Handle<AudioSource>,

    #[asset(path = "sounds/spaceandtime 2.mp3")]
    pub spaceandtime_2: Handle<AudioSource>,

    #[asset(path = "sounds/spaceandtime.mp3")]
    pub spaceandtime: Handle<AudioSource>,

    #[asset(path = "sounds/spacebug.mp3")]
    pub spacebug: Handle<AudioSource>,

    #[asset(path = "sounds/spacebug_1.mp3")]
    pub spacebug_1: Handle<AudioSource>,

    #[asset(path = "sounds/spacebug_2.mp3")]
    pub spacebug_2: Handle<AudioSource>,

    #[asset(path = "sounds/spacebug_3.mp3")]
    pub spacebug_3: Handle<AudioSource>,

    #[asset(path = "sounds/spacebug_4.mp3")]
    pub spacebug_4: Handle<AudioSource>,

    #[asset(path = "sounds/spacebug_5.mp3")]
    pub spacebug_5: Handle<AudioSource>,

    #[asset(path = "sounds/spacesonicnoise.mp3")]
    pub spacesonicnoise: Handle<AudioSource>,

    #[asset(path = "sounds/sparkles.mp3")]
    pub sparkles: Handle<AudioSource>,

    #[asset(path = "sounds/sparkles1.mp3")]
    pub sparkles1: Handle<AudioSource>,

    #[asset(path = "sounds/specialinterference.mp3")]
    pub specialinterference: Handle<AudioSource>,

    #[asset(path = "sounds/special_reload.mp3")]
    pub special_reload: Handle<AudioSource>,

    #[asset(path = "sounds/spike_1.mp3")]
    pub spike_1: Handle<AudioSource>,

    #[asset(path = "sounds/spike_10.mp3")]
    pub spike_10: Handle<AudioSource>,

    #[asset(path = "sounds/spike_11.mp3")]
    pub spike_11: Handle<AudioSource>,

    #[asset(path = "sounds/spike_12.mp3")]
    pub spike_12: Handle<AudioSource>,

    #[asset(path = "sounds/spike_2.mp3")]
    pub spike_2: Handle<AudioSource>,

    #[asset(path = "sounds/spike_3.mp3")]
    pub spike_3: Handle<AudioSource>,

    #[asset(path = "sounds/spike_4.mp3")]
    pub spike_4: Handle<AudioSource>,

    #[asset(path = "sounds/spike_5.mp3")]
    pub spike_5: Handle<AudioSource>,

    #[asset(path = "sounds/spike_6.mp3")]
    pub spike_6: Handle<AudioSource>,

    #[asset(path = "sounds/spike_7.mp3")]
    pub spike_7: Handle<AudioSource>,

    #[asset(path = "sounds/spike_8.mp3")]
    pub spike_8: Handle<AudioSource>,

    #[asset(path = "sounds/spike_9.mp3")]
    pub spike_9: Handle<AudioSource>,

    #[asset(path = "sounds/splats.mp3")]
    pub splats: Handle<AudioSource>,

    #[asset(path = "sounds/splats_2.mp3")]
    pub splats_2: Handle<AudioSource>,

    #[asset(path = "sounds/steal_1.mp3")]
    pub steal_1: Handle<AudioSource>,

    #[asset(path = "sounds/steal_2.mp3")]
    pub steal_2: Handle<AudioSource>,

    #[asset(path = "sounds/steal_3.mp3")]
    pub steal_3: Handle<AudioSource>,

    #[asset(path = "sounds/teleportawy.mp3")]
    pub teleportawy: Handle<AudioSource>,

    #[asset(path = "sounds/themoment.mp3")]
    pub themoment: Handle<AudioSource>,

    #[asset(path = "sounds/timeisout.mp3")]
    pub timeisout: Handle<AudioSource>,

    #[asset(path = "sounds/timeisright.mp3")]
    pub timeisright: Handle<AudioSource>,

    #[asset(path = "sounds/timesisright2.mp3")]
    pub timesisright2: Handle<AudioSource>,

    #[asset(path = "sounds/timewarp.mp3")]
    pub timewarp: Handle<AudioSource>,

    #[asset(path = "sounds/timewarp_2.mp3")]
    pub timewarp_2: Handle<AudioSource>,

    #[asset(path = "sounds/tpX.mp3")]
    pub tp_x: Handle<AudioSource>,

    #[asset(path = "sounds/tunnel_destroying.mp3")]
    pub tunnel_destroying: Handle<AudioSource>,

    #[asset(path = "sounds/ufo5.mp3")]
    pub ufo5: Handle<AudioSource>,

    #[asset(path = "sounds/ufodead.mp3")]
    pub ufodead: Handle<AudioSource>,

    #[asset(path = "sounds/ufodead2.mp3")]
    pub ufodead2: Handle<AudioSource>,

    #[asset(path = "sounds/ufodead3.mp3")]
    pub ufodead3: Handle<AudioSource>,

    #[asset(path = "sounds/ufolouder.mp3")]
    pub ufolouder: Handle<AudioSource>,

    #[asset(path = "sounds/ufosignal.mp3")]
    pub ufosignal: Handle<AudioSource>,

    #[asset(path = "sounds/ufosignal2.mp3")]
    pub ufosignal2: Handle<AudioSource>,

    #[asset(path = "sounds/ufosignal3.mp3")]
    pub ufosignal3: Handle<AudioSource>,

    #[asset(path = "sounds/unitready.mp3")]
    pub unitready: Handle<AudioSource>,

    #[asset(path = "sounds/unitready2.mp3")]
    pub unitready2: Handle<AudioSource>,

    #[asset(path = "sounds/unitready3.mp3")]
    pub unitready3: Handle<AudioSource>,

    #[asset(path = "sounds/whatwasthis.mp3")]
    pub whatwasthis: Handle<AudioSource>,

    #[asset(path = "sounds/whereisthepoint.mp3")]
    pub whereisthepoint: Handle<AudioSource>,

    #[asset(path = "sounds/whyamiarobot.mp3")]
    pub whyamiarobot: Handle<AudioSource>,

    #[asset(path = "sounds/whyamiarobot2.mp3")]
    pub whyamiarobot2: Handle<AudioSource>,

    #[asset(path = "sounds/whyamiarobot3.mp3")]
    pub whyamiarobot3: Handle<AudioSource>,

    #[asset(path = "sounds/whyamiarobot4.mp3")]
    pub whyamiarobot4: Handle<AudioSource>,

    #[asset(path = "sounds/whyamiarobot5.mp3")]
    pub whyamiarobot5: Handle<AudioSource>,

    #[asset(path = "sounds/win_loud.mp3")]
    pub win_loud: Handle<AudioSource>,

    #[asset(path = "sounds/wonder_1.mp3")]
    pub wonder_1: Handle<AudioSource>,

    #[asset(path = "sounds/wonder_2.mp3")]
    pub wonder_2: Handle<AudioSource>,

    #[asset(path = "sounds/wonder_3.mp3")]
    pub wonder_3: Handle<AudioSource>,

    #[asset(path = "sounds/wonder_4.mp3")]
    pub wonder_4: Handle<AudioSource>,

    #[asset(path = "sounds/wonder_5.mp3")]
    pub wonder_5: Handle<AudioSource>,
}
