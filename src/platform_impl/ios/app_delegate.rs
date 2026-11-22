use objc2::rc::Retained;
use objc2::{declare_class, mutability, ClassType, DeclaredClass};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol};
use objc2_ui_kit::{
    UIApplication, UIApplicationDelegate, UISceneConfiguration, UISceneConnectionOptions,
    UISceneSession,
};

use super::scene_delegate::{is_application_role, WinitSceneDelegate};

declare_class!(
    /// The application delegate that `UIApplicationMain` instantiates.
    ///
    /// Only used with the scene-based life cycle. Implementing
    /// `application:configurationForConnectingSceneSession:options:` is what opts the application
    /// into it without a `UIApplicationSceneManifest` entry in `Info.plist`.
    #[derive(Debug)]
    pub(crate) struct WinitApplicationDelegate;

    unsafe impl ClassType for WinitApplicationDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "WinitApplicationDelegate";
    }

    impl DeclaredClass for WinitApplicationDelegate {}

    unsafe impl NSObjectProtocol for WinitApplicationDelegate {}

    unsafe impl UIApplicationDelegate for WinitApplicationDelegate {
        #[method_id(application:configurationForConnectingSceneSession:options:)]
        fn configuration_for_connecting_scene_session(
            &self,
            _application: &UIApplication,
            session: &UISceneSession,
            _options: &UISceneConnectionOptions,
        ) -> Retained<UISceneConfiguration> {
            let mtm = MainThreadMarker::from(self);
            let role = unsafe { session.role() };
            let configuration =
                unsafe { UISceneConfiguration::configurationWithName_sessionRole(None, &role, mtm) };

            // Scenes of other roles, such as a non-interactive external display, are left without
            // a delegate so that UIKit keeps its default behavior for them.
            if is_application_role(&role) {
                unsafe { configuration.setDelegateClass(Some(WinitSceneDelegate::class())) };
            }

            configuration
        }
    }
);
