#[macro_export]
macro_rules! anim_man {
    // Matches a single sprite
    ({
        path: $path:expr,
        size: ($w:expr, $h:expr),
        $(length:$length:expr,)?
        $(fps:$fps:expr,)?
        $(next:$next:expr,)?
        $(color:$color:expr,)?
    }) => {{
        AnimationManager::from_nodes(vec![("core", AnimationNode {
            sprite: SpriteInfo::new($path, $w, $h)$(.with_color($color))?,
            $(length: $length,)*
            $(fps: $fps,)*
            $(next: Some($next.to_string()),)*
            ..default()
        })])
    }};
    // Matches a map of sprites
    ({
        $($name:ident: {
            path: $path:expr,
            size: ($w:expr, $h:expr),
            $(length:$length:expr,)?
            $(fps:$fps:expr,)?
            $(next:$next:expr,)?
            $(color:$color:expr,)?
        }$(,)?)+
    }) => {{
        AnimationManager::from_nodes(vec![$(
            (&stringify!($name), AnimationNode {
                sprite: SpriteInfo::new($path, $w, $h)$(.with_color($color))?,
                $(length: $length,)*
                $(fps: $fps,)*
                $(next: Some($next.to_string()),)*
                ..default()
            }),
        )+])
    }};
}
pub use crate::anim_man;

#[macro_export]
macro_rules! multi {
    // Matches many animations
    ([$(($name:expr, $anim:expr $(,)?)$(,)?)+]) => {{
        MultiAnimationManager::from_pairs(vec![$(
            ($name, $anim),
        )+])
    }};
    // Matches a single animation
    ($single:expr) => {{
        MultiAnimationManager::from_single($single)
    }};
}
pub use crate::multi;
