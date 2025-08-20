#![allow(dead_code)] //忽略特定代码部分中未使用（死代码）的警告
use std::collections::HashMap;

struct Request{
    method : String,
    url :String,
    headers :HashMap<String,String>,
    body:Vec<u8>,
}
struct Response{
    code :u32,
    headers:HashMap<String, String>,
    body : Vec<u8>
}
//定义了一个 类型别名 BoxedCallBack, 类型: 被Box包裹的 dyn Fn(&Request) -> Response 的闭包
// 函数指针 trait,   dyn 表示这是一个 动态分发 的 trait 对象
type BoxedCallBack = Box<dyn Fn(&Request) -> Response>; //

struct BasicRouter{
    routes : HashMap<String, BoxedCallBack>
}
impl BasicRouter{
    fn new() -> BasicRouter{
        BasicRouter{routes : HashMap::new()}
    }
//'static 是一个生命周期标注，表示数据的生命周期为整个程序运行期。它通常用于标识“静态数据”或者“长期存在的数据
    fn add_route<C>(&mut self, url : &str, callback: C)
        where C : Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request : &Request) -> Response{
        match self.routes.get(&request.url){
            None => not_found_response(),
            Some(callback) => callback(request)
        }
    }
}

fn not_found_response() -> Response{
    Response{
        code : 404, headers : HashMap::new(), body : b"<h1>Page Not Found<h1>".to_vec()
    }
}

fn get_from_response() -> Response{
    Response{
        code:200, headers: HashMap::new(), body: b"<form>".to_vec()
    }
}
fn get_gcd_response(_req: &Request) -> Response {
    Response {
        code: 500,
        headers: HashMap::new(),
        body: b"<h1>Internal server error</h1>".to_vec()
    }
}
fn req(url: &str) -> Request {
    Request {
        method: "GET".to_string(),
        url: url.to_string(),
        headers: HashMap::new(),
        body: vec![]
    }
}
#[test]
fn test_router(){
    let mut router = BasicRouter::new();
    router.add_route("/", |_| get_from_response());
    router.add_route("gcd", |req| get_gcd_response(req));
    assert_eq!(router.handle_request(&req("/piano")).code, 404);
    assert_eq!(router.handle_request(&req("/")).code, 200);
    assert_eq!(router.handle_request(&req("/gcd")).code, 500);
}
