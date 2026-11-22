use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::{declare_class, msg_send_id, mutability, ClassType, DeclaredClass};
use objc2_foundation::{ns_string, MainThreadMarker, NSObject, NSObjectProtocol, NSSet, NSString};
use objc2_ui_kit::{
    UIOpenURLContext, UIResponder, UIScene, UISceneConnectionOptions, UISceneDelegate,
    UISceneSession, UIWindowScene, UIWindowSceneDelegate,
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
            options: &UISceneConnectionOptions,
        ) {
            let mtm = MainThreadMarker::from(self);
            if let Some(scene) = window_scene(scene) {
                app_state::scene_will_connect(mtm, scene);
            }
            // UIKit returns `nil` rather than an empty set when the scene connects without URLs,
            // despite the header's annotation.
            let contexts: Option<Retained<NSSet<UIOpenURLContext>>> =
                unsafe { msg_send_id![options, URLContexts] };
            if let Some(contexts) = contexts {
                open_url_contexts(&contexts);
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

        #[method(scene:openURLContexts:)]
        fn scene_open_url_contexts(&self, _scene: &UIScene, contexts: &NSSet<UIOpenURLContext>) {
            open_url_contexts(contexts);
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

type OpenUrlHandler = Box<dyn FnMut(&str)>;

thread_local! {
    static OPEN_URL_HANDLER: RefCell<Option<OpenUrlHandler>> = const { RefCell::new(None) };
    /// URLs opened before a handler was set.
    static QUEUED_URLS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn set_open_url_handler(handler: OpenUrlHandler) {
    let _mtm =
        MainThreadMarker::new().expect("`set_open_url_handler` must be called on the main thread");

    OPEN_URL_HANDLER.with(|cell| *cell.borrow_mut() = Some(handler));

    let queued = QUEUED_URLS.with(|cell| std::mem::take(&mut *cell.borrow_mut()));
    for url in queued {
        open_url(url);
    }
}

fn open_url_contexts(contexts: &NSSet<UIOpenURLContext>) {
    for context in contexts.iter() {
        if let Some(url) = unsafe { context.URL().absoluteString() } {
            open_url(url.to_string());
        }
    }
}

fn open_url(url: String) {
    // The handler is taken out while it runs so that it may replace itself.
    let Some(mut handler) = OPEN_URL_HANDLER.with(|cell| cell.borrow_mut().take()) else {
        QUEUED_URLS.with(|cell| cell.borrow_mut().push(url));
        return;
    };

    handler(&url);

    OPEN_URL_HANDLER.with(|cell| {
        let mut current = cell.borrow_mut();
        if current.is_none() {
            *current = Some(handler);
        }
    });
}
