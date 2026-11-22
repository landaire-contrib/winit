use objc2::{declare_class, mutability, ClassType, DeclaredClass};
use objc2_foundation::{ns_string, MainThreadMarker, NSObject, NSObjectProtocol, NSString};
use objc2_ui_kit::{
    UIResponder, UIScene, UISceneConnectionOptions, UISceneDelegate, UISceneSession, UIWindowScene,
    UIWindowSceneDelegate,
};

use super::app_state;

declare_class!(
    /// The delegate of each scene of the application role, set by `WinitApplicationDelegate`.
    #[derive(Debug)]
    pub(crate) struct WinitSceneDelegate;

    unsafe impl ClassType for WinitSceneDelegate {
        #[inherits(NSObject)]
        type Super = UIResponder;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "WinitSceneDelegate";
    }

    impl DeclaredClass for WinitSceneDelegate {}

    unsafe impl NSObjectProtocol for WinitSceneDelegate {}

    unsafe impl UISceneDelegate for WinitSceneDelegate {
        #[method(scene:willConnectToSession:options:)]
        fn scene_will_connect(
            &self,
            scene: &UIScene,
            _session: &UISceneSession,
            _options: &UISceneConnectionOptions,
        ) {
            let mtm = MainThreadMarker::from(self);
            if let Some(scene) = window_scene(scene) {
                app_state::scene_will_connect(mtm, scene);
            }
        }

        #[method(sceneDidDisconnect:)]
        fn scene_did_disconnect(&self, scene: &UIScene) {
            let mtm = MainThreadMarker::from(self);
            if let Some(scene) = window_scene(scene) {
                app_state::scene_did_disconnect(mtm, scene);
            }
        }

        #[method(sceneDidBecomeActive:)]
        fn scene_did_become_active(&self, _scene: &UIScene) {
            app_state::scene_did_become_active(MainThreadMarker::from(self));
        }

        #[method(sceneWillResignActive:)]
        fn scene_will_resign_active(&self, _scene: &UIScene) {
            app_state::scene_will_resign_active(MainThreadMarker::from(self));
        }

        #[method(sceneWillEnterForeground:)]
        fn scene_will_enter_foreground(&self, scene: &UIScene) {
            let mtm = MainThreadMarker::from(self);
            if let Some(scene) = window_scene(scene) {
                let windows = unsafe { scene.windows() };
                app_state::send_occluded_event_for_windows(mtm, &windows, false);
            }
        }

        #[method(sceneDidEnterBackground:)]
        fn scene_did_enter_background(&self, scene: &UIScene) {
            let mtm = MainThreadMarker::from(self);
            if let Some(scene) = window_scene(scene) {
                let windows = unsafe { scene.windows() };
                app_state::send_occluded_event_for_windows(mtm, &windows, true);
            }
        }
    }

    unsafe impl UIWindowSceneDelegate for WinitSceneDelegate {}
);

/// Whether the scene session `role` is `UIWindowSceneSessionRoleApplication`.
///
/// The constant's value is compared rather than the constant, which is absent from UIKit before
/// iOS 13 and would otherwise have to be linked strongly.
pub(crate) fn is_application_role(role: &NSString) -> bool {
    role == ns_string!("UIWindowSceneSessionRoleApplication")
}

fn window_scene(scene: &UIScene) -> Option<&UIWindowScene> {
    if scene.is_kind_of::<UIWindowScene>() {
        // SAFETY: We just checked that the scene is a `UIWindowScene`.
        Some(unsafe { &*(scene as *const UIScene).cast::<UIWindowScene>() })
    } else {
        None
    }
}
