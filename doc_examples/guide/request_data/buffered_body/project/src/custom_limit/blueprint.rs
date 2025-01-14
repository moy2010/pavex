use pavex::blueprint::{constructor::Lifecycle, Blueprint};
use pavex::f;
use pavex::request::body::BodySizeLimit;
use pavex::unit::ToByteUnit;

pub fn body_size_limit() -> BodySizeLimit {
    BodySizeLimit::Enabled {
        max_size: 2.megabytes(),
    }
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.constructor(f!(self::body_size_limit), Lifecycle::Singleton);
    bp.route(
        pavex::blueprint::router::GET,
        "/custom_limit",
        f!(super::handler),
    );
    bp
}
