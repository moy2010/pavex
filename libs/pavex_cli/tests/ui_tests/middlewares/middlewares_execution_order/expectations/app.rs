//! Do NOT edit this code.
//! It was automatically generated by Pavex.
//! All manual edits will be lost next time the code is generated.
extern crate alloc;
struct ServerState {
    router: matchit::Router<u32>,
    application_state: ApplicationState,
}
pub struct ApplicationState {
    s0: app::Spy,
}
pub async fn build_application_state(v0: app::SpyState) -> crate::ApplicationState {
    let v1 = app::Spy::new(v0);
    crate::ApplicationState { s0: v1 }
}
pub fn run(
    server_builder: pavex::server::Server,
    application_state: ApplicationState,
) -> pavex::server::ServerHandle {
    let server_state = std::sync::Arc::new(ServerState {
        router: build_router(),
        application_state,
    });
    server_builder.serve(route_request, server_state)
}
fn build_router() -> matchit::Router<u32> {
    let mut router = matchit::Router::new();
    router.insert("/after_handler", 0u32).unwrap();
    router.insert("/nested", 1u32).unwrap();
    router.insert("/top_level", 2u32).unwrap();
    router
}
async fn route_request(
    request: http::Request<hyper::body::Incoming>,
    server_state: std::sync::Arc<ServerState>,
) -> pavex::response::Response {
    let (request_head, request_body) = request.into_parts();
    #[allow(unused)]
    let request_body = pavex::request::body::RawIncomingBody::from(request_body);
    let request_head: pavex::request::RequestHead = request_head.into();
    let matched_route = match server_state.router.at(&request_head.target.path()) {
        Ok(m) => m,
        Err(_) => {
            let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter(
                    vec![],
                )
                .into();
            return route_0::handler(&allowed_methods).await;
        }
    };
    let route_id = matched_route.value;
    #[allow(unused)]
    let url_params: pavex::request::path::RawPathParams<'_, '_> = matched_route
        .params
        .into();
    match route_id {
        0u32 => {
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_2::middleware_0(&server_state.application_state.s0).await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_0::handler(&allowed_methods).await
                }
            }
        }
        1u32 => {
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_1::middleware_0(&server_state.application_state.s0).await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_0::handler(&allowed_methods).await
                }
            }
        }
        2u32 => {
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_3::middleware_0(&server_state.application_state.s0).await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_0::handler(&allowed_methods).await
                }
            }
        }
        i => unreachable!("Unknown route id: {}", i),
    }
}
pub mod route_0 {
    pub async fn handler(
        v0: &pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let v1 = pavex::router::default_fallback(v0).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v1)
    }
}
pub mod route_1 {
    pub async fn middleware_0(v0: &app::Spy) -> pavex::response::Response {
        let v1 = crate::route_1::Next0 {
            s_0: v0,
            next: middleware_1,
        };
        let v2 = pavex::middleware::Next::new(v1);
        app::first(v0, v2).await
    }
    pub async fn middleware_1(v0: &app::Spy) -> pavex::response::Response {
        let v1 = crate::route_1::Next1 {
            s_0: v0,
            next: handler,
        };
        let v2 = pavex::middleware::Next::new(v1);
        app::second(v0, v2).await
    }
    pub async fn handler(v0: &app::Spy) -> pavex::response::Response {
        let v1 = app::handler(v0).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v1)
    }
    pub struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a app::Spy,
        next: fn(&'a app::Spy) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0)
        }
    }
    pub struct Next1<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a app::Spy,
        next: fn(&'a app::Spy) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next1<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0)
        }
    }
}
pub mod route_2 {
    pub async fn middleware_0(v0: &app::Spy) -> pavex::response::Response {
        let v1 = crate::route_2::Next0 {
            s_0: v0,
            next: handler,
        };
        let v2 = pavex::middleware::Next::new(v1);
        app::first(v0, v2).await
    }
    pub async fn handler(v0: &app::Spy) -> pavex::response::Response {
        let v1 = app::handler(v0).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v1)
    }
    pub struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a app::Spy,
        next: fn(&'a app::Spy) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0)
        }
    }
}
pub mod route_3 {
    pub async fn middleware_0(v0: &app::Spy) -> pavex::response::Response {
        let v1 = crate::route_3::Next0 {
            s_0: v0,
            next: middleware_1,
        };
        let v2 = pavex::middleware::Next::new(v1);
        app::first(v0, v2).await
    }
    pub async fn middleware_1(v0: &app::Spy) -> pavex::response::Response {
        let v1 = crate::route_3::Next1 {
            s_0: v0,
            next: handler,
        };
        let v2 = pavex::middleware::Next::new(v1);
        app::second(v0, v2).await
    }
    pub async fn handler(v0: &app::Spy) -> pavex::response::Response {
        let v1 = app::handler(v0).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v1)
    }
    pub struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a app::Spy,
        next: fn(&'a app::Spy) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0)
        }
    }
    pub struct Next1<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a app::Spy,
        next: fn(&'a app::Spy) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next1<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0)
        }
    }
}