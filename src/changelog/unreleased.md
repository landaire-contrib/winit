The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

The sections should follow the order `Added`, `Changed`, `Deprecated`,
`Removed`, and `Fixed`.

Platform specific changed should be added to the end of the section and grouped
by platform name. Common API additions should have `, implemented` at the end
for platforms where the API was initially implemented. See the following example
on how to add them:

```md
### Added

- Add `Window::turbo()`, implemented on X11, Wayland, and Web.
- On X11, add `Window::some_rare_api`.
- On X11, add `Window::even_more_rare_api`.
- On Wayland, add `Window::common_api`.
- On Windows, add `Window::some_rare_api`.
```

When the change requires non-trivial amount of work for users to comply
with it, the migration guide should be added below the entry, like:

```md
- Deprecate `Window` creation outside of `EventLoop::run`

  This was done to simply migration in the future. Consider the
  following code:

  // Code snippet.

  To migrate it we should do X, Y, and then Z, for example:

  // Code snippet.

```

The migration guide could reference other migration examples in the current
changelog entry.

## Unreleased

### Added

- On iOS, add `EventLoopBuilderExtIOS::with_scene_life_cycle` to adopt the UIKit scene-based life cycle, which apps built with the iOS 27 SDK require.
- On iOS, add `platform::ios::set_open_url_handler` to receive the URLs the system opens in an application that uses the scene-based life cycle.
- On iOS, implement `Window::theme`, `ActiveEventLoop::system_theme` and `WindowEvent::ThemeChanged`.
- On iOS, report the hover of a trackpad or mouse pointer as `WindowEvent::CursorEntered`, `WindowEvent::CursorMoved` and `WindowEvent::CursorLeft`.

