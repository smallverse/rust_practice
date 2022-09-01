#[derive(Clone, Debug, PartialEq)]
pub struct ResultVO<T> {
    /** 状态码，比如1000代表响应成功 */
    pub code: i32,
    /** 响应信息，用来说明响应情况 */
    pub msg: String,
    /** 响应的具体数据 */
    pub data: T,
}

